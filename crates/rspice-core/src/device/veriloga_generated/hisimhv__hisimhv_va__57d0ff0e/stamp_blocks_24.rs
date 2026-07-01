#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_384(
        p: &Parameters,
        var_arg: f64,
        var_arg_dn0: f64,
        var_arg_dn10: f64,
        var_arg_dn11: f64,
        var_arg_dn14: f64,
        var_arg_dn2: f64,
        var_arg_dn4: f64,
        var_arg_dn5: f64,
        var_arg_dn6: f64,
        var_arg_dn7: f64,
        var_arg_dn8: f64,
        var_arg_dn9: f64,
        var_guard2340: f64,
        var_guard2360: f64,
        var_guard2375: f64,
        var_guard2376: f64,
        var_guard2377: f64,
        var_kdep: f64,
        var_noverd: f64,
        var_rd_ps0ld: f64,
        var_rd_ps0ld_dn0: f64,
        var_rd_ps0ld_dn10: f64,
        var_rd_ps0ld_dn11: f64,
        var_rd_ps0ld_dn14: f64,
        var_rd_ps0ld_dn2: f64,
        var_rd_ps0ld_dn4: f64,
        var_rd_ps0ld_dn5: f64,
        var_rd_ps0ld_dn6: f64,
        var_rd_ps0ld_dn7: f64,
        var_rd_ps0ld_dn8: f64,
        var_rd_ps0ld_dn9: f64,
        var_tmf1: f64,
        var_tmf1_dn0: f64,
        var_tmf1_dn10: f64,
        var_tmf1_dn11: f64,
        var_tmf1_dn14: f64,
        var_tmf1_dn2: f64,
        var_tmf1_dn4: f64,
        var_tmf1_dn5: f64,
        var_tmf1_dn6: f64,
        var_tmf1_dn7: f64,
        var_tmf1_dn8: f64,
        var_tmf1_dn9: f64,
        var_vbs__blk2357: f64,
        var_vbs__blk2357_dn8: f64,
        var_vbs__blk2357_dn9: f64,
        var_vds__blk2356: f64,
        var_vds__blk2356_dn6: f64,
        var_vds__blk2356_dn8: f64,
        var_xmp: f64,
        var_xmp_dn0: f64,
        var_xmp_dn10: f64,
        var_xmp_dn11: f64,
        var_xmp_dn14: f64,
        var_xmp_dn2: f64,
        var_xmp_dn4: f64,
        var_xmp_dn5: f64,
        var_xmp_dn6: f64,
        var_xmp_dn7: f64,
        var_xmp_dn8: f64,
        var_xmp_dn9: f64,
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
        var_dnm_slot: &mut f64,
        var_dnm_dn0_slot: &mut f64,
        var_dnm_dn10_slot: &mut f64,
        var_dnm_dn11_slot: &mut f64,
        var_dnm_dn14_slot: &mut f64,
        var_dnm_dn2_slot: &mut f64,
        var_dnm_dn4_slot: &mut f64,
        var_dnm_dn5_slot: &mut f64,
        var_dnm_dn6_slot: &mut f64,
        var_dnm_dn7_slot: &mut f64,
        var_dnm_dn8_slot: &mut f64,
        var_dnm_dn9_slot: &mut f64,
        var_guard2382_slot: &mut f64,
        var_guard2383_slot: &mut f64,
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
        var_t9_slot: &mut f64,
        var_t9_dn0_slot: &mut f64,
        var_t9_dn10_slot: &mut f64,
        var_t9_dn11_slot: &mut f64,
        var_t9_dn14_slot: &mut f64,
        var_t9_dn2_slot: &mut f64,
        var_t9_dn4_slot: &mut f64,
        var_t9_dn5_slot: &mut f64,
        var_t9_dn6_slot: &mut f64,
        var_t9_dn7_slot: &mut f64,
        var_t9_dn8_slot: &mut f64,
        var_t9_dn9_slot: &mut f64,
        var_tmf0_slot: &mut f64,
        var_tmf0_dn0_slot: &mut f64,
        var_tmf0_dn10_slot: &mut f64,
        var_tmf0_dn11_slot: &mut f64,
        var_tmf0_dn14_slot: &mut f64,
        var_tmf0_dn2_slot: &mut f64,
        var_tmf0_dn4_slot: &mut f64,
        var_tmf0_dn5_slot: &mut f64,
        var_tmf0_dn6_slot: &mut f64,
        var_tmf0_dn7_slot: &mut f64,
        var_tmf0_dn8_slot: &mut f64,
        var_tmf0_dn9_slot: &mut f64,
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
        var_wdepl_slot: &mut f64,
        var_wdepl_dn0_slot: &mut f64,
        var_wdepl_dn10_slot: &mut f64,
        var_wdepl_dn11_slot: &mut f64,
        var_wdepl_dn14_slot: &mut f64,
        var_wdepl_dn2_slot: &mut f64,
        var_wdepl_dn4_slot: &mut f64,
        var_wdepl_dn5_slot: &mut f64,
        var_wdepl_dn6_slot: &mut f64,
        var_wdepl_dn7_slot: &mut f64,
        var_wdepl_dn8_slot: &mut f64,
        var_wdepl_dn9_slot: &mut f64,
    ) {
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
        let mut var_dnm: f64 = *var_dnm_slot;
        let mut var_dnm_dn0: f64 = *var_dnm_dn0_slot;
        let mut var_dnm_dn10: f64 = *var_dnm_dn10_slot;
        let mut var_dnm_dn11: f64 = *var_dnm_dn11_slot;
        let mut var_dnm_dn14: f64 = *var_dnm_dn14_slot;
        let mut var_dnm_dn2: f64 = *var_dnm_dn2_slot;
        let mut var_dnm_dn4: f64 = *var_dnm_dn4_slot;
        let mut var_dnm_dn5: f64 = *var_dnm_dn5_slot;
        let mut var_dnm_dn6: f64 = *var_dnm_dn6_slot;
        let mut var_dnm_dn7: f64 = *var_dnm_dn7_slot;
        let mut var_dnm_dn8: f64 = *var_dnm_dn8_slot;
        let mut var_dnm_dn9: f64 = *var_dnm_dn9_slot;
        let mut var_guard2382: f64 = *var_guard2382_slot;
        let mut var_guard2383: f64 = *var_guard2383_slot;
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
        let mut var_t9: f64 = *var_t9_slot;
        let mut var_t9_dn0: f64 = *var_t9_dn0_slot;
        let mut var_t9_dn10: f64 = *var_t9_dn10_slot;
        let mut var_t9_dn11: f64 = *var_t9_dn11_slot;
        let mut var_t9_dn14: f64 = *var_t9_dn14_slot;
        let mut var_t9_dn2: f64 = *var_t9_dn2_slot;
        let mut var_t9_dn4: f64 = *var_t9_dn4_slot;
        let mut var_t9_dn5: f64 = *var_t9_dn5_slot;
        let mut var_t9_dn6: f64 = *var_t9_dn6_slot;
        let mut var_t9_dn7: f64 = *var_t9_dn7_slot;
        let mut var_t9_dn8: f64 = *var_t9_dn8_slot;
        let mut var_t9_dn9: f64 = *var_t9_dn9_slot;
        let mut var_tmf0: f64 = *var_tmf0_slot;
        let mut var_tmf0_dn0: f64 = *var_tmf0_dn0_slot;
        let mut var_tmf0_dn10: f64 = *var_tmf0_dn10_slot;
        let mut var_tmf0_dn11: f64 = *var_tmf0_dn11_slot;
        let mut var_tmf0_dn14: f64 = *var_tmf0_dn14_slot;
        let mut var_tmf0_dn2: f64 = *var_tmf0_dn2_slot;
        let mut var_tmf0_dn4: f64 = *var_tmf0_dn4_slot;
        let mut var_tmf0_dn5: f64 = *var_tmf0_dn5_slot;
        let mut var_tmf0_dn6: f64 = *var_tmf0_dn6_slot;
        let mut var_tmf0_dn7: f64 = *var_tmf0_dn7_slot;
        let mut var_tmf0_dn8: f64 = *var_tmf0_dn8_slot;
        let mut var_tmf0_dn9: f64 = *var_tmf0_dn9_slot;
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
        let mut var_wdepl: f64 = *var_wdepl_slot;
        let mut var_wdepl_dn0: f64 = *var_wdepl_dn0_slot;
        let mut var_wdepl_dn10: f64 = *var_wdepl_dn10_slot;
        let mut var_wdepl_dn11: f64 = *var_wdepl_dn11_slot;
        let mut var_wdepl_dn14: f64 = *var_wdepl_dn14_slot;
        let mut var_wdepl_dn2: f64 = *var_wdepl_dn2_slot;
        let mut var_wdepl_dn4: f64 = *var_wdepl_dn4_slot;
        let mut var_wdepl_dn5: f64 = *var_wdepl_dn5_slot;
        let mut var_wdepl_dn6: f64 = *var_wdepl_dn6_slot;
        let mut var_wdepl_dn7: f64 = *var_wdepl_dn7_slot;
        let mut var_wdepl_dn8: f64 = *var_wdepl_dn8_slot;
        let mut var_wdepl_dn9: f64 = *var_wdepl_dn9_slot;

        let (assign104570_e156854, assign104570_e156854_d_n0, assign104570_e156854_d_n2, assign104570_e156854_d_n4, assign104570_e156854_d_n5, assign104570_e156854_d_n6, assign104570_e156854_d_n7, assign104570_e156854_d_n8, assign104570_e156854_d_n9, assign104570_e156854_d_n10, assign104570_e156854_d_n11, assign104570_e156854_d_n14,) = {
    if (((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2375 != 0.0)) && (var_guard2376 != 0.0)) && (var_guard2377 == 0.0)) {
        let (assign104570_e156852, assign104570_e156852_d_n0, assign104570_e156852_d_n2, assign104570_e156852_d_n4, assign104570_e156852_d_n5, assign104570_e156852_d_n6, assign104570_e156852_d_n7, assign104570_e156852_d_n8, assign104570_e156852_d_n9, assign104570_e156852_d_n10, assign104570_e156852_d_n11, assign104570_e156852_d_n14,) = {
            if (var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign104570_e156849: f64 = (2.0 * p.p442);
                let assign104570_e156850: f64 = (1.0 / assign104570_e156849);
                let assign104570_e156851: f64 = (var_dnm).powf(assign104570_e156850);
                (assign104570_e156851, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((var_dnm).powf(assign104570_e156850 - 1.0) * var_dnm_dn0)) } } else { (assign104570_e156851 * (assign104570_e156850 * (var_dnm_dn0 / var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((var_dnm).powf(assign104570_e156850 - 1.0) * var_dnm_dn2)) } } else { (assign104570_e156851 * (assign104570_e156850 * (var_dnm_dn2 / var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((var_dnm).powf(assign104570_e156850 - 1.0) * var_dnm_dn4)) } } else { (assign104570_e156851 * (assign104570_e156850 * (var_dnm_dn4 / var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((var_dnm).powf(assign104570_e156850 - 1.0) * var_dnm_dn5)) } } else { (assign104570_e156851 * (assign104570_e156850 * (var_dnm_dn5 / var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((var_dnm).powf(assign104570_e156850 - 1.0) * var_dnm_dn6)) } } else { (assign104570_e156851 * (assign104570_e156850 * (var_dnm_dn6 / var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((var_dnm).powf(assign104570_e156850 - 1.0) * var_dnm_dn7)) } } else { (assign104570_e156851 * (assign104570_e156850 * (var_dnm_dn7 / var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((var_dnm).powf(assign104570_e156850 - 1.0) * var_dnm_dn8)) } } else { (assign104570_e156851 * (assign104570_e156850 * (var_dnm_dn8 / var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((var_dnm).powf(assign104570_e156850 - 1.0) * var_dnm_dn9)) } } else { (assign104570_e156851 * (assign104570_e156850 * (var_dnm_dn9 / var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((var_dnm).powf(assign104570_e156850 - 1.0) * var_dnm_dn10)) } } else { (assign104570_e156851 * (assign104570_e156850 * (var_dnm_dn10 / var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((var_dnm).powf(assign104570_e156850 - 1.0) * var_dnm_dn11)) } } else { (assign104570_e156851 * (assign104570_e156850 * (var_dnm_dn11 / var_dnm))) }, if 0.0 == 0.0 && ((assign104570_e156850) as f64).is_finite() && ((assign104570_e156850) as f64).fract() == 0.0 { if assign104570_e156850 == 0.0 { 0.0 } else { (assign104570_e156850 * ((var_dnm).powf(assign104570_e156850 - 1.0) * var_dnm_dn14)) } } else { (assign104570_e156851 * (assign104570_e156850 * (var_dnm_dn14 / var_dnm))) },)
            }
        };
        (assign104570_e156852, assign104570_e156852_d_n0, assign104570_e156852_d_n2, assign104570_e156852_d_n4, assign104570_e156852_d_n5, assign104570_e156852_d_n6, assign104570_e156852_d_n7, assign104570_e156852_d_n8, assign104570_e156852_d_n9, assign104570_e156852_d_n10, assign104570_e156852_d_n11, assign104570_e156852_d_n14,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn11, var_dnm_dn14,)
    }
};
        var_dnm = assign104570_e156854;
        var_dnm_dn0 = assign104570_e156854_d_n0;
        var_dnm_dn2 = assign104570_e156854_d_n2;
        var_dnm_dn4 = assign104570_e156854_d_n4;
        var_dnm_dn5 = assign104570_e156854_d_n5;
        var_dnm_dn6 = assign104570_e156854_d_n6;
        var_dnm_dn7 = assign104570_e156854_d_n7;
        var_dnm_dn8 = assign104570_e156854_d_n8;
        var_dnm_dn9 = assign104570_e156854_d_n9;
        var_dnm_dn10 = assign104570_e156854_d_n10;
        var_dnm_dn11 = assign104570_e156854_d_n11;
        var_dnm_dn14 = assign104570_e156854_d_n14;

        let (assign104580_e156867, assign104580_e156867_d_n0, assign104580_e156867_d_n2, assign104580_e156867_d_n4, assign104580_e156867_d_n5, assign104580_e156867_d_n6, assign104580_e156867_d_n7, assign104580_e156867_d_n8, assign104580_e156867_d_n9, assign104580_e156867_d_n10, assign104580_e156867_d_n11, assign104580_e156867_d_n14,) = {
    if ((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2375 != 0.0)) && (var_guard2376 != 0.0)) {
        let assign104580_e156865: f64 = (1.0 / var_dnm);
        (assign104580_e156865, (-(var_dnm_dn0 / (var_dnm * var_dnm))), (-(var_dnm_dn2 / (var_dnm * var_dnm))), (-(var_dnm_dn4 / (var_dnm * var_dnm))), (-(var_dnm_dn5 / (var_dnm * var_dnm))), (-(var_dnm_dn6 / (var_dnm * var_dnm))), (-(var_dnm_dn7 / (var_dnm * var_dnm))), (-(var_dnm_dn8 / (var_dnm * var_dnm))), (-(var_dnm_dn9 / (var_dnm * var_dnm))), (-(var_dnm_dn10 / (var_dnm * var_dnm))), (-(var_dnm_dn11 / (var_dnm * var_dnm))), (-(var_dnm_dn14 / (var_dnm * var_dnm))),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn11, var_dnm_dn14,)
    }
};
        var_dnm = assign104580_e156867;
        var_dnm_dn0 = assign104580_e156867_d_n0;
        var_dnm_dn2 = assign104580_e156867_d_n2;
        var_dnm_dn4 = assign104580_e156867_d_n4;
        var_dnm_dn5 = assign104580_e156867_d_n5;
        var_dnm_dn6 = assign104580_e156867_d_n6;
        var_dnm_dn7 = assign104580_e156867_d_n7;
        var_dnm_dn8 = assign104580_e156867_d_n8;
        var_dnm_dn9 = assign104580_e156867_d_n9;
        var_dnm_dn10 = assign104580_e156867_d_n10;
        var_dnm_dn11 = assign104580_e156867_d_n11;
        var_dnm_dn14 = assign104580_e156867_d_n14;

        let (assign104590_e156884, assign104590_e156884_d_n0, assign104590_e156884_d_n2, assign104590_e156884_d_n4, assign104590_e156884_d_n5, assign104590_e156884_d_n6, assign104590_e156884_d_n7, assign104590_e156884_d_n8, assign104590_e156884_d_n9, assign104590_e156884_d_n10, assign104590_e156884_d_n11, assign104590_e156884_d_n14,) = {
    if ((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2375 != 0.0)) && (var_guard2376 != 0.0)) {
        let assign104590_e156879: f64 = (var_noverd * p.p441);
        let assign104590_e156880: f64 = (var_tmf1 * assign104590_e156879);
        let assign104590_e156882: f64 = (assign104590_e156880 * var_dnm);
        (assign104590_e156882, (((var_tmf1_dn0 * assign104590_e156879) * var_dnm) + (assign104590_e156880 * var_dnm_dn0)), (((var_tmf1_dn2 * assign104590_e156879) * var_dnm) + (assign104590_e156880 * var_dnm_dn2)), (((var_tmf1_dn4 * assign104590_e156879) * var_dnm) + (assign104590_e156880 * var_dnm_dn4)), (((var_tmf1_dn5 * assign104590_e156879) * var_dnm) + (assign104590_e156880 * var_dnm_dn5)), (((var_tmf1_dn6 * assign104590_e156879) * var_dnm) + (assign104590_e156880 * var_dnm_dn6)), (((var_tmf1_dn7 * assign104590_e156879) * var_dnm) + (assign104590_e156880 * var_dnm_dn7)), (((var_tmf1_dn8 * assign104590_e156879) * var_dnm) + (assign104590_e156880 * var_dnm_dn8)), (((var_tmf1_dn9 * assign104590_e156879) * var_dnm) + (assign104590_e156880 * var_dnm_dn9)), (((var_tmf1_dn10 * assign104590_e156879) * var_dnm) + (assign104590_e156880 * var_dnm_dn10)), (((var_tmf1_dn11 * assign104590_e156879) * var_dnm) + (assign104590_e156880 * var_dnm_dn11)), (((var_tmf1_dn14 * assign104590_e156879) * var_dnm) + (assign104590_e156880 * var_dnm_dn14)),)
    } else {
        (var_tmf0, var_tmf0_dn0, var_tmf0_dn2, var_tmf0_dn4, var_tmf0_dn5, var_tmf0_dn6, var_tmf0_dn7, var_tmf0_dn8, var_tmf0_dn9, var_tmf0_dn10, var_tmf0_dn11, var_tmf0_dn14,)
    }
};
        var_tmf0 = assign104590_e156884;
        var_tmf0_dn0 = assign104590_e156884_d_n0;
        var_tmf0_dn2 = assign104590_e156884_d_n2;
        var_tmf0_dn4 = assign104590_e156884_d_n4;
        var_tmf0_dn5 = assign104590_e156884_d_n5;
        var_tmf0_dn6 = assign104590_e156884_d_n6;
        var_tmf0_dn7 = assign104590_e156884_d_n7;
        var_tmf0_dn8 = assign104590_e156884_d_n8;
        var_tmf0_dn9 = assign104590_e156884_d_n9;
        var_tmf0_dn10 = assign104590_e156884_d_n10;
        var_tmf0_dn11 = assign104590_e156884_d_n11;
        var_tmf0_dn14 = assign104590_e156884_d_n14;

        let (assign104600_e156903, assign104600_e156903_d_n0, assign104600_e156903_d_n2, assign104600_e156903_d_n4, assign104600_e156903_d_n5, assign104600_e156903_d_n6, assign104600_e156903_d_n7, assign104600_e156903_d_n8, assign104600_e156903_d_n9, assign104600_e156903_d_n10, assign104600_e156903_d_n11, assign104600_e156903_d_n14,) = {
    if ((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2375 != 0.0)) && (var_guard2376 != 0.0)) {
        let assign104600_e156895: f64 = (var_noverd * p.p441);
        let assign104600_e156897: f64 = (assign104600_e156895 * var_xmp);
        let assign104600_e156899: f64 = (assign104600_e156897 * var_dnm);
        let assign104600_e156901: f64 = (assign104600_e156899 / var_arg);
        (assign104600_e156901, ((((((assign104600_e156895 * var_xmp_dn0) * var_dnm) + (assign104600_e156897 * var_dnm_dn0)) * var_arg) - (assign104600_e156899 * var_arg_dn0)) / (var_arg * var_arg)), ((((((assign104600_e156895 * var_xmp_dn2) * var_dnm) + (assign104600_e156897 * var_dnm_dn2)) * var_arg) - (assign104600_e156899 * var_arg_dn2)) / (var_arg * var_arg)), ((((((assign104600_e156895 * var_xmp_dn4) * var_dnm) + (assign104600_e156897 * var_dnm_dn4)) * var_arg) - (assign104600_e156899 * var_arg_dn4)) / (var_arg * var_arg)), ((((((assign104600_e156895 * var_xmp_dn5) * var_dnm) + (assign104600_e156897 * var_dnm_dn5)) * var_arg) - (assign104600_e156899 * var_arg_dn5)) / (var_arg * var_arg)), ((((((assign104600_e156895 * var_xmp_dn6) * var_dnm) + (assign104600_e156897 * var_dnm_dn6)) * var_arg) - (assign104600_e156899 * var_arg_dn6)) / (var_arg * var_arg)), ((((((assign104600_e156895 * var_xmp_dn7) * var_dnm) + (assign104600_e156897 * var_dnm_dn7)) * var_arg) - (assign104600_e156899 * var_arg_dn7)) / (var_arg * var_arg)), ((((((assign104600_e156895 * var_xmp_dn8) * var_dnm) + (assign104600_e156897 * var_dnm_dn8)) * var_arg) - (assign104600_e156899 * var_arg_dn8)) / (var_arg * var_arg)), ((((((assign104600_e156895 * var_xmp_dn9) * var_dnm) + (assign104600_e156897 * var_dnm_dn9)) * var_arg) - (assign104600_e156899 * var_arg_dn9)) / (var_arg * var_arg)), ((((((assign104600_e156895 * var_xmp_dn10) * var_dnm) + (assign104600_e156897 * var_dnm_dn10)) * var_arg) - (assign104600_e156899 * var_arg_dn10)) / (var_arg * var_arg)), ((((((assign104600_e156895 * var_xmp_dn11) * var_dnm) + (assign104600_e156897 * var_dnm_dn11)) * var_arg) - (assign104600_e156899 * var_arg_dn11)) / (var_arg * var_arg)), ((((((assign104600_e156895 * var_xmp_dn14) * var_dnm) + (assign104600_e156897 * var_dnm_dn14)) * var_arg) - (assign104600_e156899 * var_arg_dn14)) / (var_arg * var_arg)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign104600_e156903;
        var_t0_dn0 = assign104600_e156903_d_n0;
        var_t0_dn2 = assign104600_e156903_d_n2;
        var_t0_dn4 = assign104600_e156903_d_n4;
        var_t0_dn5 = assign104600_e156903_d_n5;
        var_t0_dn6 = assign104600_e156903_d_n6;
        var_t0_dn7 = assign104600_e156903_d_n7;
        var_t0_dn8 = assign104600_e156903_d_n8;
        var_t0_dn9 = assign104600_e156903_d_n9;
        var_t0_dn10 = assign104600_e156903_d_n10;
        var_t0_dn11 = assign104600_e156903_d_n11;
        var_t0_dn14 = assign104600_e156903_d_n14;

        let (assign104610_e156922, assign104610_e156922_d_n0, assign104610_e156922_d_n2, assign104610_e156922_d_n4, assign104610_e156922_d_n5, assign104610_e156922_d_n6, assign104610_e156922_d_n7, assign104610_e156922_d_n8, assign104610_e156922_d_n9, assign104610_e156922_d_n10, assign104610_e156922_d_n11, assign104610_e156922_d_n14,) = {
    if ((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2375 != 0.0)) && (var_guard2376 != 0.0)) {
        let assign104610_e156914: f64 = (var_noverd * p.p440);
        let assign104610_e156917: f64 = (var_noverd * p.p441);
        let assign104610_e156918: f64 = (assign104610_e156914 - assign104610_e156917);
        let assign104610_e156920: f64 = (assign104610_e156918 + var_tmf0);
        (assign104610_e156920, var_tmf0_dn0, var_tmf0_dn2, var_tmf0_dn4, var_tmf0_dn5, var_tmf0_dn6, var_tmf0_dn7, var_tmf0_dn8, var_tmf0_dn9, var_tmf0_dn10, var_tmf0_dn11, var_tmf0_dn14,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign104610_e156922;
        var_t2_dn0 = assign104610_e156922_d_n0;
        var_t2_dn2 = assign104610_e156922_d_n2;
        var_t2_dn4 = assign104610_e156922_d_n4;
        var_t2_dn5 = assign104610_e156922_d_n5;
        var_t2_dn6 = assign104610_e156922_d_n6;
        var_t2_dn7 = assign104610_e156922_d_n7;
        var_t2_dn8 = assign104610_e156922_d_n8;
        var_t2_dn9 = assign104610_e156922_d_n9;
        var_t2_dn10 = assign104610_e156922_d_n10;
        var_t2_dn11 = assign104610_e156922_d_n11;
        var_t2_dn14 = assign104610_e156922_d_n14;

        let (assign104620_e156933, assign104620_e156933_d_n0, assign104620_e156933_d_n2, assign104620_e156933_d_n4, assign104620_e156933_d_n5, assign104620_e156933_d_n6, assign104620_e156933_d_n7, assign104620_e156933_d_n8, assign104620_e156933_d_n9, assign104620_e156933_d_n10, assign104620_e156933_d_n11, assign104620_e156933_d_n14,) = {
    if ((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2375 != 0.0)) && (var_guard2376 != 0.0)) {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign104620_e156933;
        var_t0_dn0 = assign104620_e156933_d_n0;
        var_t0_dn2 = assign104620_e156933_d_n2;
        var_t0_dn4 = assign104620_e156933_d_n4;
        var_t0_dn5 = assign104620_e156933_d_n5;
        var_t0_dn6 = assign104620_e156933_d_n6;
        var_t0_dn7 = assign104620_e156933_d_n7;
        var_t0_dn8 = assign104620_e156933_d_n8;
        var_t0_dn9 = assign104620_e156933_d_n9;
        var_t0_dn10 = assign104620_e156933_d_n10;
        var_t0_dn11 = assign104620_e156933_d_n11;
        var_t0_dn14 = assign104620_e156933_d_n14;

        let (assign104630_e156945, assign104630_e156945_d_n0, assign104630_e156945_d_n2, assign104630_e156945_d_n4, assign104630_e156945_d_n5, assign104630_e156945_d_n6, assign104630_e156945_d_n7, assign104630_e156945_d_n8, assign104630_e156945_d_n9, assign104630_e156945_d_n10, assign104630_e156945_d_n11, assign104630_e156945_d_n14,) = {
    if ((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2375 != 0.0)) && (var_guard2376 == 0.0)) {
        (var_carr, var_carr_dn0, var_carr_dn2, var_carr_dn4, var_carr_dn5, var_carr_dn6, var_carr_dn7, var_carr_dn8, var_carr_dn9, var_carr_dn10, var_carr_dn11, var_carr_dn14,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign104630_e156945;
        var_t2_dn0 = assign104630_e156945_d_n0;
        var_t2_dn2 = assign104630_e156945_d_n2;
        var_t2_dn4 = assign104630_e156945_d_n4;
        var_t2_dn5 = assign104630_e156945_d_n5;
        var_t2_dn6 = assign104630_e156945_d_n6;
        var_t2_dn7 = assign104630_e156945_d_n7;
        var_t2_dn8 = assign104630_e156945_d_n8;
        var_t2_dn9 = assign104630_e156945_d_n9;
        var_t2_dn10 = assign104630_e156945_d_n10;
        var_t2_dn11 = assign104630_e156945_d_n11;
        var_t2_dn14 = assign104630_e156945_d_n14;

        let (assign104640_e156957, assign104640_e156957_d_n0, assign104640_e156957_d_n2, assign104640_e156957_d_n4, assign104640_e156957_d_n5, assign104640_e156957_d_n6, assign104640_e156957_d_n7, assign104640_e156957_d_n8, assign104640_e156957_d_n9, assign104640_e156957_d_n10, assign104640_e156957_d_n11, assign104640_e156957_d_n14,) = {
    if ((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2375 != 0.0)) && (var_guard2376 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign104640_e156957;
        var_t0_dn0 = assign104640_e156957_d_n0;
        var_t0_dn2 = assign104640_e156957_d_n2;
        var_t0_dn4 = assign104640_e156957_d_n4;
        var_t0_dn5 = assign104640_e156957_d_n5;
        var_t0_dn6 = assign104640_e156957_d_n6;
        var_t0_dn7 = assign104640_e156957_d_n7;
        var_t0_dn8 = assign104640_e156957_d_n8;
        var_t0_dn9 = assign104640_e156957_d_n9;
        var_t0_dn10 = assign104640_e156957_d_n10;
        var_t0_dn11 = assign104640_e156957_d_n11;
        var_t0_dn14 = assign104640_e156957_d_n14;

        let (assign104650_e156966, assign104650_e156966_d_n0, assign104650_e156966_d_n2, assign104650_e156966_d_n4, assign104650_e156966_d_n5, assign104650_e156966_d_n6, assign104650_e156966_d_n7, assign104650_e156966_d_n8, assign104650_e156966_d_n9, assign104650_e156966_d_n10, assign104650_e156966_d_n11, assign104650_e156966_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2375 != 0.0)) {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    } else {
        (var_carr, var_carr_dn0, var_carr_dn2, var_carr_dn4, var_carr_dn5, var_carr_dn6, var_carr_dn7, var_carr_dn8, var_carr_dn9, var_carr_dn10, var_carr_dn11, var_carr_dn14,)
    }
};
        var_carr = assign104650_e156966;
        var_carr_dn0 = assign104650_e156966_d_n0;
        var_carr_dn2 = assign104650_e156966_d_n2;
        var_carr_dn4 = assign104650_e156966_d_n4;
        var_carr_dn5 = assign104650_e156966_d_n5;
        var_carr_dn6 = assign104650_e156966_d_n6;
        var_carr_dn7 = assign104650_e156966_d_n7;
        var_carr_dn8 = assign104650_e156966_d_n8;
        var_carr_dn9 = assign104650_e156966_d_n9;
        var_carr_dn10 = assign104650_e156966_d_n10;
        var_carr_dn11 = assign104650_e156966_d_n11;
        var_carr_dn14 = assign104650_e156966_d_n14;

        let (assign104660_e156974, assign104660_e156974_d_n0, assign104660_e156974_d_n2, assign104660_e156974_d_n4, assign104660_e156974_d_n5, assign104660_e156974_d_n6, assign104660_e156974_d_n7, assign104660_e156974_d_n8, assign104660_e156974_d_n9, assign104660_e156974_d_n10, assign104660_e156974_d_n11, assign104660_e156974_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104660_e156972: f64 = (-var_rd_ps0ld);
        (assign104660_e156972, (-var_rd_ps0ld_dn0), (-var_rd_ps0ld_dn2), (-var_rd_ps0ld_dn4), (-var_rd_ps0ld_dn5), (-var_rd_ps0ld_dn6), (-var_rd_ps0ld_dn7), (-var_rd_ps0ld_dn8), (-var_rd_ps0ld_dn9), (-var_rd_ps0ld_dn10), (-var_rd_ps0ld_dn11), (-var_rd_ps0ld_dn14),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign104660_e156974;
        var_t0_dn0 = assign104660_e156974_d_n0;
        var_t0_dn2 = assign104660_e156974_d_n2;
        var_t0_dn4 = assign104660_e156974_d_n4;
        var_t0_dn5 = assign104660_e156974_d_n5;
        var_t0_dn6 = assign104660_e156974_d_n6;
        var_t0_dn7 = assign104660_e156974_d_n7;
        var_t0_dn8 = assign104660_e156974_d_n8;
        var_t0_dn9 = assign104660_e156974_d_n9;
        var_t0_dn10 = assign104660_e156974_d_n10;
        var_t0_dn11 = assign104660_e156974_d_n11;
        var_t0_dn14 = assign104660_e156974_d_n14;

        let (assign104670_e156990, assign104670_e156990_d_n0, assign104670_e156990_d_n2, assign104670_e156990_d_n4, assign104670_e156990_d_n5, assign104670_e156990_d_n6, assign104670_e156990_d_n7, assign104670_e156990_d_n8, assign104670_e156990_d_n9, assign104670_e156990_d_n10, assign104670_e156990_d_n11, assign104670_e156990_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104670_e156981: f64 = (var_t0 * var_t0);
        let assign104670_e156984: f64 = (4.0 * 0.01);
        let assign104670_e156986: f64 = (assign104670_e156984 * 0.01);
        let assign104670_e156987: f64 = (assign104670_e156981 + assign104670_e156986);
        let assign104670_e156988: f64 = (assign104670_e156987).sqrt();
        (assign104670_e156988, (((var_t0_dn0 * var_t0) + (var_t0 * var_t0_dn0)) / (2.0 * assign104670_e156988)), (((var_t0_dn2 * var_t0) + (var_t0 * var_t0_dn2)) / (2.0 * assign104670_e156988)), (((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4)) / (2.0 * assign104670_e156988)), (((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5)) / (2.0 * assign104670_e156988)), (((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6)) / (2.0 * assign104670_e156988)), (((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7)) / (2.0 * assign104670_e156988)), (((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8)) / (2.0 * assign104670_e156988)), (((var_t0_dn9 * var_t0) + (var_t0 * var_t0_dn9)) / (2.0 * assign104670_e156988)), (((var_t0_dn10 * var_t0) + (var_t0 * var_t0_dn10)) / (2.0 * assign104670_e156988)), (((var_t0_dn11 * var_t0) + (var_t0 * var_t0_dn11)) / (2.0 * assign104670_e156988)), (((var_t0_dn14 * var_t0) + (var_t0 * var_t0_dn14)) / (2.0 * assign104670_e156988)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign104670_e156990;
        var_tmf2_dn0 = assign104670_e156990_d_n0;
        var_tmf2_dn2 = assign104670_e156990_d_n2;
        var_tmf2_dn4 = assign104670_e156990_d_n4;
        var_tmf2_dn5 = assign104670_e156990_d_n5;
        var_tmf2_dn6 = assign104670_e156990_d_n6;
        var_tmf2_dn7 = assign104670_e156990_d_n7;
        var_tmf2_dn8 = assign104670_e156990_d_n8;
        var_tmf2_dn9 = assign104670_e156990_d_n9;
        var_tmf2_dn10 = assign104670_e156990_d_n10;
        var_tmf2_dn11 = assign104670_e156990_d_n11;
        var_tmf2_dn14 = assign104670_e156990_d_n14;

        let (assign104680_e157003, assign104680_e157003_d_n0, assign104680_e157003_d_n2, assign104680_e157003_d_n4, assign104680_e157003_d_n5, assign104680_e157003_d_n6, assign104680_e157003_d_n7, assign104680_e157003_d_n8, assign104680_e157003_d_n9, assign104680_e157003_d_n10, assign104680_e157003_d_n11, assign104680_e157003_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104680_e156999: f64 = (var_t0 / var_tmf2);
        let assign104680_e157000: f64 = (1.0 + assign104680_e156999);
        let assign104680_e157001: f64 = (0.5 * assign104680_e157000);
        (assign104680_e157001, (0.5 * (((var_t0_dn0 * var_tmf2) - (var_t0 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t0_dn2 * var_tmf2) - (var_t0 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t0_dn4 * var_tmf2) - (var_t0 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t0_dn5 * var_tmf2) - (var_t0 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t0_dn6 * var_tmf2) - (var_t0 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t0_dn7 * var_tmf2) - (var_t0 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t0_dn8 * var_tmf2) - (var_t0 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t0_dn9 * var_tmf2) - (var_t0 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t0_dn10 * var_tmf2) - (var_t0 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t0_dn11 * var_tmf2) - (var_t0 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t0_dn14 * var_tmf2) - (var_t0 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_dn9, var_t9_dn10, var_t9_dn11, var_t9_dn14,)
    }
};
        var_t9 = assign104680_e157003;
        var_t9_dn0 = assign104680_e157003_d_n0;
        var_t9_dn2 = assign104680_e157003_d_n2;
        var_t9_dn4 = assign104680_e157003_d_n4;
        var_t9_dn5 = assign104680_e157003_d_n5;
        var_t9_dn6 = assign104680_e157003_d_n6;
        var_t9_dn7 = assign104680_e157003_d_n7;
        var_t9_dn8 = assign104680_e157003_d_n8;
        var_t9_dn9 = assign104680_e157003_d_n9;
        var_t9_dn10 = assign104680_e157003_d_n10;
        var_t9_dn11 = assign104680_e157003_d_n11;
        var_t9_dn14 = assign104680_e157003_d_n14;

        let (assign104690_e157014, assign104690_e157014_d_n0, assign104690_e157014_d_n2, assign104690_e157014_d_n4, assign104690_e157014_d_n5, assign104690_e157014_d_n6, assign104690_e157014_d_n7, assign104690_e157014_d_n8, assign104690_e157014_d_n9, assign104690_e157014_d_n10, assign104690_e157014_d_n11, assign104690_e157014_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104690_e157011: f64 = (var_t0 + var_tmf2);
        let assign104690_e157012: f64 = (0.5 * assign104690_e157011);
        (assign104690_e157012, (0.5 * (var_t0_dn0 + var_tmf2_dn0)), (0.5 * (var_t0_dn2 + var_tmf2_dn2)), (0.5 * (var_t0_dn4 + var_tmf2_dn4)), (0.5 * (var_t0_dn5 + var_tmf2_dn5)), (0.5 * (var_t0_dn6 + var_tmf2_dn6)), (0.5 * (var_t0_dn7 + var_tmf2_dn7)), (0.5 * (var_t0_dn8 + var_tmf2_dn8)), (0.5 * (var_t0_dn9 + var_tmf2_dn9)), (0.5 * (var_t0_dn10 + var_tmf2_dn10)), (0.5 * (var_t0_dn11 + var_tmf2_dn11)), (0.5 * (var_t0_dn14 + var_tmf2_dn14)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign104690_e157014;
        var_t0_dn0 = assign104690_e157014_d_n0;
        var_t0_dn2 = assign104690_e157014_d_n2;
        var_t0_dn4 = assign104690_e157014_d_n4;
        var_t0_dn5 = assign104690_e157014_d_n5;
        var_t0_dn6 = assign104690_e157014_d_n6;
        var_t0_dn7 = assign104690_e157014_d_n7;
        var_t0_dn8 = assign104690_e157014_d_n8;
        var_t0_dn9 = assign104690_e157014_d_n9;
        var_t0_dn10 = assign104690_e157014_d_n10;
        var_t0_dn11 = assign104690_e157014_d_n11;
        var_t0_dn14 = assign104690_e157014_d_n14;

        let assign104700_e157017: f64 = if var_t0 < 0.0 { 1.0 } else { 0.0 };
        var_guard2382 = assign104700_e157017;

        let (assign104710_e157026, assign104710_e157026_d_n0, assign104710_e157026_d_n2, assign104710_e157026_d_n4, assign104710_e157026_d_n5, assign104710_e157026_d_n6, assign104710_e157026_d_n7, assign104710_e157026_d_n8, assign104710_e157026_d_n9, assign104710_e157026_d_n10, assign104710_e157026_d_n11, assign104710_e157026_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2382 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign104710_e157026;
        var_t0_dn0 = assign104710_e157026_d_n0;
        var_t0_dn2 = assign104710_e157026_d_n2;
        var_t0_dn4 = assign104710_e157026_d_n4;
        var_t0_dn5 = assign104710_e157026_d_n5;
        var_t0_dn6 = assign104710_e157026_d_n6;
        var_t0_dn7 = assign104710_e157026_d_n7;
        var_t0_dn8 = assign104710_e157026_d_n8;
        var_t0_dn9 = assign104710_e157026_d_n9;
        var_t0_dn10 = assign104710_e157026_d_n10;
        var_t0_dn11 = assign104710_e157026_d_n11;
        var_t0_dn14 = assign104710_e157026_d_n14;

        let (assign104720_e157035, assign104720_e157035_d_n0, assign104720_e157035_d_n2, assign104720_e157035_d_n4, assign104720_e157035_d_n5, assign104720_e157035_d_n6, assign104720_e157035_d_n7, assign104720_e157035_d_n8, assign104720_e157035_d_n9, assign104720_e157035_d_n10, assign104720_e157035_d_n11, assign104720_e157035_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2382 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_dn9, var_t9_dn10, var_t9_dn11, var_t9_dn14,)
    }
};
        var_t9 = assign104720_e157035;
        var_t9_dn0 = assign104720_e157035_d_n0;
        var_t9_dn2 = assign104720_e157035_d_n2;
        var_t9_dn4 = assign104720_e157035_d_n4;
        var_t9_dn5 = assign104720_e157035_d_n5;
        var_t9_dn6 = assign104720_e157035_d_n6;
        var_t9_dn7 = assign104720_e157035_d_n7;
        var_t9_dn8 = assign104720_e157035_d_n8;
        var_t9_dn9 = assign104720_e157035_d_n9;
        var_t9_dn10 = assign104720_e157035_d_n10;
        var_t9_dn11 = assign104720_e157035_d_n11;
        var_t9_dn14 = assign104720_e157035_d_n14;

        let (assign104730_e157046, assign104730_e157046_d_n0, assign104730_e157046_d_n2, assign104730_e157046_d_n4, assign104730_e157046_d_n5, assign104730_e157046_d_n6, assign104730_e157046_d_n7, assign104730_e157046_d_n8, assign104730_e157046_d_n9, assign104730_e157046_d_n10, assign104730_e157046_d_n11, assign104730_e157046_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104730_e157043: f64 = (10.0 * 2.220446049250313e-16);
        let assign104730_e157044: f64 = (var_t0 + assign104730_e157043);
        (assign104730_e157044, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign104730_e157046;
        var_t0_dn0 = assign104730_e157046_d_n0;
        var_t0_dn2 = assign104730_e157046_d_n2;
        var_t0_dn4 = assign104730_e157046_d_n4;
        var_t0_dn5 = assign104730_e157046_d_n5;
        var_t0_dn6 = assign104730_e157046_d_n6;
        var_t0_dn7 = assign104730_e157046_d_n7;
        var_t0_dn8 = assign104730_e157046_d_n8;
        var_t0_dn9 = assign104730_e157046_d_n9;
        var_t0_dn10 = assign104730_e157046_d_n10;
        var_t0_dn11 = assign104730_e157046_d_n11;
        var_t0_dn14 = assign104730_e157046_d_n14;

        let (assign104740_e157056, assign104740_e157056_d_n0, assign104740_e157056_d_n2, assign104740_e157056_d_n4, assign104740_e157056_d_n5, assign104740_e157056_d_n6, assign104740_e157056_d_n7, assign104740_e157056_d_n8, assign104740_e157056_d_n9, assign104740_e157056_d_n10, assign104740_e157056_d_n11, assign104740_e157056_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104740_e157053: f64 = (var_kdep * var_t0);
        let assign104740_e157054: f64 = (assign104740_e157053).sqrt();
        (assign104740_e157054, ((var_kdep * var_t0_dn0) / (2.0 * assign104740_e157054)), ((var_kdep * var_t0_dn2) / (2.0 * assign104740_e157054)), ((var_kdep * var_t0_dn4) / (2.0 * assign104740_e157054)), ((var_kdep * var_t0_dn5) / (2.0 * assign104740_e157054)), ((var_kdep * var_t0_dn6) / (2.0 * assign104740_e157054)), ((var_kdep * var_t0_dn7) / (2.0 * assign104740_e157054)), ((var_kdep * var_t0_dn8) / (2.0 * assign104740_e157054)), ((var_kdep * var_t0_dn9) / (2.0 * assign104740_e157054)), ((var_kdep * var_t0_dn10) / (2.0 * assign104740_e157054)), ((var_kdep * var_t0_dn11) / (2.0 * assign104740_e157054)), ((var_kdep * var_t0_dn14) / (2.0 * assign104740_e157054)),)
    } else {
        (var_wdepl, var_wdepl_dn0, var_wdepl_dn2, var_wdepl_dn4, var_wdepl_dn5, var_wdepl_dn6, var_wdepl_dn7, var_wdepl_dn8, var_wdepl_dn9, var_wdepl_dn10, var_wdepl_dn11, var_wdepl_dn14,)
    }
};
        var_wdepl = assign104740_e157056;
        var_wdepl_dn0 = assign104740_e157056_d_n0;
        var_wdepl_dn2 = assign104740_e157056_d_n2;
        var_wdepl_dn4 = assign104740_e157056_d_n4;
        var_wdepl_dn5 = assign104740_e157056_d_n5;
        var_wdepl_dn6 = assign104740_e157056_d_n6;
        var_wdepl_dn7 = assign104740_e157056_d_n7;
        var_wdepl_dn8 = assign104740_e157056_d_n8;
        var_wdepl_dn9 = assign104740_e157056_d_n9;
        var_wdepl_dn10 = assign104740_e157056_d_n10;
        var_wdepl_dn11 = assign104740_e157056_d_n11;
        var_wdepl_dn14 = assign104740_e157056_d_n14;

        let (assign104750_e157067, assign104750_e157067_d_n0, assign104750_e157067_d_n2, assign104750_e157067_d_n4, assign104750_e157067_d_n5, assign104750_e157067_d_n6, assign104750_e157067_d_n7, assign104750_e157067_d_n8, assign104750_e157067_d_n9, assign104750_e157067_d_n10, assign104750_e157067_d_n11, assign104750_e157067_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104750_e157063: f64 = (var_vds__blk2356 - var_vbs__blk2357);
        let assign104750_e157065: f64 = (assign104750_e157063 + p.p137);
        (assign104750_e157065, 0.0, 0.0, 0.0, 0.0, var_vds__blk2356_dn6, 0.0, (var_vds__blk2356_dn8 - var_vbs__blk2357_dn8), (-var_vbs__blk2357_dn9), 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign104750_e157067;
        var_t2_dn0 = assign104750_e157067_d_n0;
        var_t2_dn2 = assign104750_e157067_d_n2;
        var_t2_dn4 = assign104750_e157067_d_n4;
        var_t2_dn5 = assign104750_e157067_d_n5;
        var_t2_dn6 = assign104750_e157067_d_n6;
        var_t2_dn7 = assign104750_e157067_d_n7;
        var_t2_dn8 = assign104750_e157067_d_n8;
        var_t2_dn9 = assign104750_e157067_d_n9;
        var_t2_dn10 = assign104750_e157067_d_n10;
        var_t2_dn11 = assign104750_e157067_d_n11;
        var_t2_dn14 = assign104750_e157067_d_n14;

        let (assign104760_e157083, assign104760_e157083_d_n0, assign104760_e157083_d_n2, assign104760_e157083_d_n4, assign104760_e157083_d_n5, assign104760_e157083_d_n6, assign104760_e157083_d_n7, assign104760_e157083_d_n8, assign104760_e157083_d_n9, assign104760_e157083_d_n10, assign104760_e157083_d_n11, assign104760_e157083_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104760_e157074: f64 = (var_t2 * var_t2);
        let assign104760_e157077: f64 = (4.0 * 0.01);
        let assign104760_e157079: f64 = (assign104760_e157077 * 0.01);
        let assign104760_e157080: f64 = (assign104760_e157074 + assign104760_e157079);
        let assign104760_e157081: f64 = (assign104760_e157080).sqrt();
        (assign104760_e157081, (((var_t2_dn0 * var_t2) + (var_t2 * var_t2_dn0)) / (2.0 * assign104760_e157081)), (((var_t2_dn2 * var_t2) + (var_t2 * var_t2_dn2)) / (2.0 * assign104760_e157081)), (((var_t2_dn4 * var_t2) + (var_t2 * var_t2_dn4)) / (2.0 * assign104760_e157081)), (((var_t2_dn5 * var_t2) + (var_t2 * var_t2_dn5)) / (2.0 * assign104760_e157081)), (((var_t2_dn6 * var_t2) + (var_t2 * var_t2_dn6)) / (2.0 * assign104760_e157081)), (((var_t2_dn7 * var_t2) + (var_t2 * var_t2_dn7)) / (2.0 * assign104760_e157081)), (((var_t2_dn8 * var_t2) + (var_t2 * var_t2_dn8)) / (2.0 * assign104760_e157081)), (((var_t2_dn9 * var_t2) + (var_t2 * var_t2_dn9)) / (2.0 * assign104760_e157081)), (((var_t2_dn10 * var_t2) + (var_t2 * var_t2_dn10)) / (2.0 * assign104760_e157081)), (((var_t2_dn11 * var_t2) + (var_t2 * var_t2_dn11)) / (2.0 * assign104760_e157081)), (((var_t2_dn14 * var_t2) + (var_t2 * var_t2_dn14)) / (2.0 * assign104760_e157081)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign104760_e157083;
        var_tmf2_dn0 = assign104760_e157083_d_n0;
        var_tmf2_dn2 = assign104760_e157083_d_n2;
        var_tmf2_dn4 = assign104760_e157083_d_n4;
        var_tmf2_dn5 = assign104760_e157083_d_n5;
        var_tmf2_dn6 = assign104760_e157083_d_n6;
        var_tmf2_dn7 = assign104760_e157083_d_n7;
        var_tmf2_dn8 = assign104760_e157083_d_n8;
        var_tmf2_dn9 = assign104760_e157083_d_n9;
        var_tmf2_dn10 = assign104760_e157083_d_n10;
        var_tmf2_dn11 = assign104760_e157083_d_n11;
        var_tmf2_dn14 = assign104760_e157083_d_n14;

        let (assign104770_e157096, assign104770_e157096_d_n0, assign104770_e157096_d_n2, assign104770_e157096_d_n4, assign104770_e157096_d_n5, assign104770_e157096_d_n6, assign104770_e157096_d_n7, assign104770_e157096_d_n8, assign104770_e157096_d_n9, assign104770_e157096_d_n10, assign104770_e157096_d_n11, assign104770_e157096_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104770_e157092: f64 = (var_t2 / var_tmf2);
        let assign104770_e157093: f64 = (1.0 + assign104770_e157092);
        let assign104770_e157094: f64 = (0.5 * assign104770_e157093);
        (assign104770_e157094, (0.5 * (((var_t2_dn0 * var_tmf2) - (var_t2 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t2_dn2 * var_tmf2) - (var_t2 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t2_dn4 * var_tmf2) - (var_t2 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t2_dn5 * var_tmf2) - (var_t2 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t2_dn6 * var_tmf2) - (var_t2 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t2_dn7 * var_tmf2) - (var_t2 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t2_dn8 * var_tmf2) - (var_t2 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t2_dn9 * var_tmf2) - (var_t2 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t2_dn10 * var_tmf2) - (var_t2 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t2_dn11 * var_tmf2) - (var_t2 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t2_dn14 * var_tmf2) - (var_t2 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_dn9, var_t9_dn10, var_t9_dn11, var_t9_dn14,)
    }
};
        var_t9 = assign104770_e157096;
        var_t9_dn0 = assign104770_e157096_d_n0;
        var_t9_dn2 = assign104770_e157096_d_n2;
        var_t9_dn4 = assign104770_e157096_d_n4;
        var_t9_dn5 = assign104770_e157096_d_n5;
        var_t9_dn6 = assign104770_e157096_d_n6;
        var_t9_dn7 = assign104770_e157096_d_n7;
        var_t9_dn8 = assign104770_e157096_d_n8;
        var_t9_dn9 = assign104770_e157096_d_n9;
        var_t9_dn10 = assign104770_e157096_d_n10;
        var_t9_dn11 = assign104770_e157096_d_n11;
        var_t9_dn14 = assign104770_e157096_d_n14;

        let (assign104780_e157107, assign104780_e157107_d_n0, assign104780_e157107_d_n2, assign104780_e157107_d_n4, assign104780_e157107_d_n5, assign104780_e157107_d_n6, assign104780_e157107_d_n7, assign104780_e157107_d_n8, assign104780_e157107_d_n9, assign104780_e157107_d_n10, assign104780_e157107_d_n11, assign104780_e157107_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104780_e157104: f64 = (var_t2 + var_tmf2);
        let assign104780_e157105: f64 = (0.5 * assign104780_e157104);
        (assign104780_e157105, (0.5 * (var_t2_dn0 + var_tmf2_dn0)), (0.5 * (var_t2_dn2 + var_tmf2_dn2)), (0.5 * (var_t2_dn4 + var_tmf2_dn4)), (0.5 * (var_t2_dn5 + var_tmf2_dn5)), (0.5 * (var_t2_dn6 + var_tmf2_dn6)), (0.5 * (var_t2_dn7 + var_tmf2_dn7)), (0.5 * (var_t2_dn8 + var_tmf2_dn8)), (0.5 * (var_t2_dn9 + var_tmf2_dn9)), (0.5 * (var_t2_dn10 + var_tmf2_dn10)), (0.5 * (var_t2_dn11 + var_tmf2_dn11)), (0.5 * (var_t2_dn14 + var_tmf2_dn14)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign104780_e157107;
        var_t2_dn0 = assign104780_e157107_d_n0;
        var_t2_dn2 = assign104780_e157107_d_n2;
        var_t2_dn4 = assign104780_e157107_d_n4;
        var_t2_dn5 = assign104780_e157107_d_n5;
        var_t2_dn6 = assign104780_e157107_d_n6;
        var_t2_dn7 = assign104780_e157107_d_n7;
        var_t2_dn8 = assign104780_e157107_d_n8;
        var_t2_dn9 = assign104780_e157107_d_n9;
        var_t2_dn10 = assign104780_e157107_d_n10;
        var_t2_dn11 = assign104780_e157107_d_n11;
        var_t2_dn14 = assign104780_e157107_d_n14;

        let assign104790_e157110: f64 = if var_t2 < 0.0 { 1.0 } else { 0.0 };
        var_guard2383 = assign104790_e157110;

        let (assign104800_e157119, assign104800_e157119_d_n0, assign104800_e157119_d_n2, assign104800_e157119_d_n4, assign104800_e157119_d_n5, assign104800_e157119_d_n6, assign104800_e157119_d_n7, assign104800_e157119_d_n8, assign104800_e157119_d_n9, assign104800_e157119_d_n10, assign104800_e157119_d_n11, assign104800_e157119_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2383 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign104800_e157119;
        var_t2_dn0 = assign104800_e157119_d_n0;
        var_t2_dn2 = assign104800_e157119_d_n2;
        var_t2_dn4 = assign104800_e157119_d_n4;
        var_t2_dn5 = assign104800_e157119_d_n5;
        var_t2_dn6 = assign104800_e157119_d_n6;
        var_t2_dn7 = assign104800_e157119_d_n7;
        var_t2_dn8 = assign104800_e157119_d_n8;
        var_t2_dn9 = assign104800_e157119_d_n9;
        var_t2_dn10 = assign104800_e157119_d_n10;
        var_t2_dn11 = assign104800_e157119_d_n11;
        var_t2_dn14 = assign104800_e157119_d_n14;

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
        *var_dnm_slot = var_dnm;
        *var_dnm_dn0_slot = var_dnm_dn0;
        *var_dnm_dn10_slot = var_dnm_dn10;
        *var_dnm_dn11_slot = var_dnm_dn11;
        *var_dnm_dn14_slot = var_dnm_dn14;
        *var_dnm_dn2_slot = var_dnm_dn2;
        *var_dnm_dn4_slot = var_dnm_dn4;
        *var_dnm_dn5_slot = var_dnm_dn5;
        *var_dnm_dn6_slot = var_dnm_dn6;
        *var_dnm_dn7_slot = var_dnm_dn7;
        *var_dnm_dn8_slot = var_dnm_dn8;
        *var_dnm_dn9_slot = var_dnm_dn9;
        *var_guard2382_slot = var_guard2382;
        *var_guard2383_slot = var_guard2383;
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
        *var_t9_slot = var_t9;
        *var_t9_dn0_slot = var_t9_dn0;
        *var_t9_dn10_slot = var_t9_dn10;
        *var_t9_dn11_slot = var_t9_dn11;
        *var_t9_dn14_slot = var_t9_dn14;
        *var_t9_dn2_slot = var_t9_dn2;
        *var_t9_dn4_slot = var_t9_dn4;
        *var_t9_dn5_slot = var_t9_dn5;
        *var_t9_dn6_slot = var_t9_dn6;
        *var_t9_dn7_slot = var_t9_dn7;
        *var_t9_dn8_slot = var_t9_dn8;
        *var_t9_dn9_slot = var_t9_dn9;
        *var_tmf0_slot = var_tmf0;
        *var_tmf0_dn0_slot = var_tmf0_dn0;
        *var_tmf0_dn10_slot = var_tmf0_dn10;
        *var_tmf0_dn11_slot = var_tmf0_dn11;
        *var_tmf0_dn14_slot = var_tmf0_dn14;
        *var_tmf0_dn2_slot = var_tmf0_dn2;
        *var_tmf0_dn4_slot = var_tmf0_dn4;
        *var_tmf0_dn5_slot = var_tmf0_dn5;
        *var_tmf0_dn6_slot = var_tmf0_dn6;
        *var_tmf0_dn7_slot = var_tmf0_dn7;
        *var_tmf0_dn8_slot = var_tmf0_dn8;
        *var_tmf0_dn9_slot = var_tmf0_dn9;
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
        *var_wdepl_slot = var_wdepl;
        *var_wdepl_dn0_slot = var_wdepl_dn0;
        *var_wdepl_dn10_slot = var_wdepl_dn10;
        *var_wdepl_dn11_slot = var_wdepl_dn11;
        *var_wdepl_dn14_slot = var_wdepl_dn14;
        *var_wdepl_dn2_slot = var_wdepl_dn2;
        *var_wdepl_dn4_slot = var_wdepl_dn4;
        *var_wdepl_dn5_slot = var_wdepl_dn5;
        *var_wdepl_dn6_slot = var_wdepl_dn6;
        *var_wdepl_dn7_slot = var_wdepl_dn7;
        *var_wdepl_dn8_slot = var_wdepl_dn8;
        *var_wdepl_dn9_slot = var_wdepl_dn9;
    }

    pub(super) fn stamp_transient_block_385(
        p: &Parameters,
        var_carr: f64,
        var_carr_dn0: f64,
        var_carr_dn10: f64,
        var_carr_dn11: f64,
        var_carr_dn14: f64,
        var_carr_dn2: f64,
        var_carr_dn4: f64,
        var_carr_dn5: f64,
        var_carr_dn6: f64,
        var_carr_dn7: f64,
        var_carr_dn8: f64,
        var_carr_dn9: f64,
        var_cx: f64,
        var_guard2340: f64,
        var_guard2360: f64,
        var_guard2383: f64,
        var_kjunc: f64,
        var_kjunc_dn0: f64,
        var_kjunc_dn10: f64,
        var_kjunc_dn11: f64,
        var_kjunc_dn14: f64,
        var_kjunc_dn2: f64,
        var_kjunc_dn4: f64,
        var_kjunc_dn5: f64,
        var_kjunc_dn6: f64,
        var_kjunc_dn7: f64,
        var_kjunc_dn8: f64,
        var_kjunc_dn9: f64,
        var_ldrifte: f64,
        var_mu__blk2358: f64,
        var_mu__blk2358_dn0: f64,
        var_mu__blk2358_dn10: f64,
        var_mu__blk2358_dn11: f64,
        var_mu__blk2358_dn14: f64,
        var_mu__blk2358_dn2: f64,
        var_mu__blk2358_dn4: f64,
        var_mu__blk2358_dn5: f64,
        var_mu__blk2358_dn6: f64,
        var_mu__blk2358_dn7: f64,
        var_mu__blk2358_dn8: f64,
        var_mu__blk2358_dn9: f64,
        var_rd_xldld: f64,
        var_uc_rdrcx: f64,
        var_wdepl: f64,
        var_wdepl_dn0: f64,
        var_wdepl_dn10: f64,
        var_wdepl_dn11: f64,
        var_wdepl_dn14: f64,
        var_wdepl_dn2: f64,
        var_wdepl_dn4: f64,
        var_wdepl_dn5: f64,
        var_wdepl_dn6: f64,
        var_wdepl_dn7: f64,
        var_wdepl_dn8: f64,
        var_wdepl_dn9: f64,
        var_xmax: f64,
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
        var_guard2384_slot: &mut f64,
        var_guard2385_slot: &mut f64,
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
        var_t9_slot: &mut f64,
        var_t9_dn0_slot: &mut f64,
        var_t9_dn10_slot: &mut f64,
        var_t9_dn11_slot: &mut f64,
        var_t9_dn14_slot: &mut f64,
        var_t9_dn2_slot: &mut f64,
        var_t9_dn4_slot: &mut f64,
        var_t9_dn5_slot: &mut f64,
        var_t9_dn6_slot: &mut f64,
        var_t9_dn7_slot: &mut f64,
        var_t9_dn8_slot: &mut f64,
        var_t9_dn9_slot: &mut f64,
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
        var_wjunc_slot: &mut f64,
        var_wjunc0_slot: &mut f64,
        var_wjunc0_dn0_slot: &mut f64,
        var_wjunc0_dn10_slot: &mut f64,
        var_wjunc0_dn11_slot: &mut f64,
        var_wjunc0_dn14_slot: &mut f64,
        var_wjunc0_dn2_slot: &mut f64,
        var_wjunc0_dn4_slot: &mut f64,
        var_wjunc0_dn5_slot: &mut f64,
        var_wjunc0_dn6_slot: &mut f64,
        var_wjunc0_dn7_slot: &mut f64,
        var_wjunc0_dn8_slot: &mut f64,
        var_wjunc0_dn9_slot: &mut f64,
        var_wjunc_dn0_slot: &mut f64,
        var_wjunc_dn10_slot: &mut f64,
        var_wjunc_dn11_slot: &mut f64,
        var_wjunc_dn14_slot: &mut f64,
        var_wjunc_dn2_slot: &mut f64,
        var_wjunc_dn4_slot: &mut f64,
        var_wjunc_dn5_slot: &mut f64,
        var_wjunc_dn6_slot: &mut f64,
        var_wjunc_dn7_slot: &mut f64,
        var_wjunc_dn8_slot: &mut f64,
        var_wjunc_dn9_slot: &mut f64,
        var_wrdrdjunc_slot: &mut f64,
        var_x2_slot: &mut f64,
        var_x2_dn0_slot: &mut f64,
        var_x2_dn10_slot: &mut f64,
        var_x2_dn11_slot: &mut f64,
        var_x2_dn14_slot: &mut f64,
        var_x2_dn2_slot: &mut f64,
        var_x2_dn4_slot: &mut f64,
        var_x2_dn5_slot: &mut f64,
        var_x2_dn6_slot: &mut f64,
        var_x2_dn7_slot: &mut f64,
        var_x2_dn8_slot: &mut f64,
        var_x2_dn9_slot: &mut f64,
        var_xmax2_slot: &mut f64,
        var_xmax2_dn0_slot: &mut f64,
        var_xmax2_dn10_slot: &mut f64,
        var_xmax2_dn11_slot: &mut f64,
        var_xmax2_dn14_slot: &mut f64,
        var_xmax2_dn2_slot: &mut f64,
        var_xmax2_dn4_slot: &mut f64,
        var_xmax2_dn5_slot: &mut f64,
        var_xmax2_dn6_slot: &mut f64,
        var_xmax2_dn7_slot: &mut f64,
        var_xmax2_dn8_slot: &mut f64,
        var_xmax2_dn9_slot: &mut f64,
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
        var_xp_slot: &mut f64,
        var_xp_dn0_slot: &mut f64,
        var_xp_dn10_slot: &mut f64,
        var_xp_dn11_slot: &mut f64,
        var_xp_dn14_slot: &mut f64,
        var_xp_dn2_slot: &mut f64,
        var_xp_dn4_slot: &mut f64,
        var_xp_dn5_slot: &mut f64,
        var_xp_dn6_slot: &mut f64,
        var_xp_dn7_slot: &mut f64,
        var_xp_dn8_slot: &mut f64,
        var_xp_dn9_slot: &mut f64,
    ) {
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
        let mut var_guard2384: f64 = *var_guard2384_slot;
        let mut var_guard2385: f64 = *var_guard2385_slot;
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
        let mut var_t9: f64 = *var_t9_slot;
        let mut var_t9_dn0: f64 = *var_t9_dn0_slot;
        let mut var_t9_dn10: f64 = *var_t9_dn10_slot;
        let mut var_t9_dn11: f64 = *var_t9_dn11_slot;
        let mut var_t9_dn14: f64 = *var_t9_dn14_slot;
        let mut var_t9_dn2: f64 = *var_t9_dn2_slot;
        let mut var_t9_dn4: f64 = *var_t9_dn4_slot;
        let mut var_t9_dn5: f64 = *var_t9_dn5_slot;
        let mut var_t9_dn6: f64 = *var_t9_dn6_slot;
        let mut var_t9_dn7: f64 = *var_t9_dn7_slot;
        let mut var_t9_dn8: f64 = *var_t9_dn8_slot;
        let mut var_t9_dn9: f64 = *var_t9_dn9_slot;
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
        let mut var_wjunc: f64 = *var_wjunc_slot;
        let mut var_wjunc0: f64 = *var_wjunc0_slot;
        let mut var_wjunc0_dn0: f64 = *var_wjunc0_dn0_slot;
        let mut var_wjunc0_dn10: f64 = *var_wjunc0_dn10_slot;
        let mut var_wjunc0_dn11: f64 = *var_wjunc0_dn11_slot;
        let mut var_wjunc0_dn14: f64 = *var_wjunc0_dn14_slot;
        let mut var_wjunc0_dn2: f64 = *var_wjunc0_dn2_slot;
        let mut var_wjunc0_dn4: f64 = *var_wjunc0_dn4_slot;
        let mut var_wjunc0_dn5: f64 = *var_wjunc0_dn5_slot;
        let mut var_wjunc0_dn6: f64 = *var_wjunc0_dn6_slot;
        let mut var_wjunc0_dn7: f64 = *var_wjunc0_dn7_slot;
        let mut var_wjunc0_dn8: f64 = *var_wjunc0_dn8_slot;
        let mut var_wjunc0_dn9: f64 = *var_wjunc0_dn9_slot;
        let mut var_wjunc_dn0: f64 = *var_wjunc_dn0_slot;
        let mut var_wjunc_dn10: f64 = *var_wjunc_dn10_slot;
        let mut var_wjunc_dn11: f64 = *var_wjunc_dn11_slot;
        let mut var_wjunc_dn14: f64 = *var_wjunc_dn14_slot;
        let mut var_wjunc_dn2: f64 = *var_wjunc_dn2_slot;
        let mut var_wjunc_dn4: f64 = *var_wjunc_dn4_slot;
        let mut var_wjunc_dn5: f64 = *var_wjunc_dn5_slot;
        let mut var_wjunc_dn6: f64 = *var_wjunc_dn6_slot;
        let mut var_wjunc_dn7: f64 = *var_wjunc_dn7_slot;
        let mut var_wjunc_dn8: f64 = *var_wjunc_dn8_slot;
        let mut var_wjunc_dn9: f64 = *var_wjunc_dn9_slot;
        let mut var_wrdrdjunc: f64 = *var_wrdrdjunc_slot;
        let mut var_x2: f64 = *var_x2_slot;
        let mut var_x2_dn0: f64 = *var_x2_dn0_slot;
        let mut var_x2_dn10: f64 = *var_x2_dn10_slot;
        let mut var_x2_dn11: f64 = *var_x2_dn11_slot;
        let mut var_x2_dn14: f64 = *var_x2_dn14_slot;
        let mut var_x2_dn2: f64 = *var_x2_dn2_slot;
        let mut var_x2_dn4: f64 = *var_x2_dn4_slot;
        let mut var_x2_dn5: f64 = *var_x2_dn5_slot;
        let mut var_x2_dn6: f64 = *var_x2_dn6_slot;
        let mut var_x2_dn7: f64 = *var_x2_dn7_slot;
        let mut var_x2_dn8: f64 = *var_x2_dn8_slot;
        let mut var_x2_dn9: f64 = *var_x2_dn9_slot;
        let mut var_xmax2: f64 = *var_xmax2_slot;
        let mut var_xmax2_dn0: f64 = *var_xmax2_dn0_slot;
        let mut var_xmax2_dn10: f64 = *var_xmax2_dn10_slot;
        let mut var_xmax2_dn11: f64 = *var_xmax2_dn11_slot;
        let mut var_xmax2_dn14: f64 = *var_xmax2_dn14_slot;
        let mut var_xmax2_dn2: f64 = *var_xmax2_dn2_slot;
        let mut var_xmax2_dn4: f64 = *var_xmax2_dn4_slot;
        let mut var_xmax2_dn5: f64 = *var_xmax2_dn5_slot;
        let mut var_xmax2_dn6: f64 = *var_xmax2_dn6_slot;
        let mut var_xmax2_dn7: f64 = *var_xmax2_dn7_slot;
        let mut var_xmax2_dn8: f64 = *var_xmax2_dn8_slot;
        let mut var_xmax2_dn9: f64 = *var_xmax2_dn9_slot;
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
        let mut var_xp: f64 = *var_xp_slot;
        let mut var_xp_dn0: f64 = *var_xp_dn0_slot;
        let mut var_xp_dn10: f64 = *var_xp_dn10_slot;
        let mut var_xp_dn11: f64 = *var_xp_dn11_slot;
        let mut var_xp_dn14: f64 = *var_xp_dn14_slot;
        let mut var_xp_dn2: f64 = *var_xp_dn2_slot;
        let mut var_xp_dn4: f64 = *var_xp_dn4_slot;
        let mut var_xp_dn5: f64 = *var_xp_dn5_slot;
        let mut var_xp_dn6: f64 = *var_xp_dn6_slot;
        let mut var_xp_dn7: f64 = *var_xp_dn7_slot;
        let mut var_xp_dn8: f64 = *var_xp_dn8_slot;
        let mut var_xp_dn9: f64 = *var_xp_dn9_slot;

        let (assign104810_e157128, assign104810_e157128_d_n0, assign104810_e157128_d_n2, assign104810_e157128_d_n4, assign104810_e157128_d_n5, assign104810_e157128_d_n6, assign104810_e157128_d_n7, assign104810_e157128_d_n8, assign104810_e157128_d_n9, assign104810_e157128_d_n10, assign104810_e157128_d_n11, assign104810_e157128_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2383 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_dn9, var_t9_dn10, var_t9_dn11, var_t9_dn14,)
    }
};
        var_t9 = assign104810_e157128;
        var_t9_dn0 = assign104810_e157128_d_n0;
        var_t9_dn2 = assign104810_e157128_d_n2;
        var_t9_dn4 = assign104810_e157128_d_n4;
        var_t9_dn5 = assign104810_e157128_d_n5;
        var_t9_dn6 = assign104810_e157128_d_n6;
        var_t9_dn7 = assign104810_e157128_d_n7;
        var_t9_dn8 = assign104810_e157128_d_n8;
        var_t9_dn9 = assign104810_e157128_d_n9;
        var_t9_dn10 = assign104810_e157128_d_n10;
        var_t9_dn11 = assign104810_e157128_d_n11;
        var_t9_dn14 = assign104810_e157128_d_n14;

        let (assign104820_e157139, assign104820_e157139_d_n0, assign104820_e157139_d_n2, assign104820_e157139_d_n4, assign104820_e157139_d_n5, assign104820_e157139_d_n6, assign104820_e157139_d_n7, assign104820_e157139_d_n8, assign104820_e157139_d_n9, assign104820_e157139_d_n10, assign104820_e157139_d_n11, assign104820_e157139_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104820_e157136: f64 = (10.0 * 2.220446049250313e-16);
        let assign104820_e157137: f64 = (var_t2 + assign104820_e157136);
        (assign104820_e157137, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign104820_e157139;
        var_t2_dn0 = assign104820_e157139_d_n0;
        var_t2_dn2 = assign104820_e157139_d_n2;
        var_t2_dn4 = assign104820_e157139_d_n4;
        var_t2_dn5 = assign104820_e157139_d_n5;
        var_t2_dn6 = assign104820_e157139_d_n6;
        var_t2_dn7 = assign104820_e157139_d_n7;
        var_t2_dn8 = assign104820_e157139_d_n8;
        var_t2_dn9 = assign104820_e157139_d_n9;
        var_t2_dn10 = assign104820_e157139_d_n10;
        var_t2_dn11 = assign104820_e157139_d_n11;
        var_t2_dn14 = assign104820_e157139_d_n14;

        let (assign104830_e157149, assign104830_e157149_d_n0, assign104830_e157149_d_n2, assign104830_e157149_d_n4, assign104830_e157149_d_n5, assign104830_e157149_d_n6, assign104830_e157149_d_n7, assign104830_e157149_d_n8, assign104830_e157149_d_n9, assign104830_e157149_d_n10, assign104830_e157149_d_n11, assign104830_e157149_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104830_e157146: f64 = (var_kjunc * var_t2);
        let assign104830_e157147: f64 = (assign104830_e157146).sqrt();
        (assign104830_e157147, (((var_kjunc_dn0 * var_t2) + (var_kjunc * var_t2_dn0)) / (2.0 * assign104830_e157147)), (((var_kjunc_dn2 * var_t2) + (var_kjunc * var_t2_dn2)) / (2.0 * assign104830_e157147)), (((var_kjunc_dn4 * var_t2) + (var_kjunc * var_t2_dn4)) / (2.0 * assign104830_e157147)), (((var_kjunc_dn5 * var_t2) + (var_kjunc * var_t2_dn5)) / (2.0 * assign104830_e157147)), (((var_kjunc_dn6 * var_t2) + (var_kjunc * var_t2_dn6)) / (2.0 * assign104830_e157147)), (((var_kjunc_dn7 * var_t2) + (var_kjunc * var_t2_dn7)) / (2.0 * assign104830_e157147)), (((var_kjunc_dn8 * var_t2) + (var_kjunc * var_t2_dn8)) / (2.0 * assign104830_e157147)), (((var_kjunc_dn9 * var_t2) + (var_kjunc * var_t2_dn9)) / (2.0 * assign104830_e157147)), (((var_kjunc_dn10 * var_t2) + (var_kjunc * var_t2_dn10)) / (2.0 * assign104830_e157147)), (((var_kjunc_dn11 * var_t2) + (var_kjunc * var_t2_dn11)) / (2.0 * assign104830_e157147)), (((var_kjunc_dn14 * var_t2) + (var_kjunc * var_t2_dn14)) / (2.0 * assign104830_e157147)),)
    } else {
        (var_wjunc0, var_wjunc0_dn0, var_wjunc0_dn2, var_wjunc0_dn4, var_wjunc0_dn5, var_wjunc0_dn6, var_wjunc0_dn7, var_wjunc0_dn8, var_wjunc0_dn9, var_wjunc0_dn10, var_wjunc0_dn11, var_wjunc0_dn14,)
    }
};
        var_wjunc0 = assign104830_e157149;
        var_wjunc0_dn0 = assign104830_e157149_d_n0;
        var_wjunc0_dn2 = assign104830_e157149_d_n2;
        var_wjunc0_dn4 = assign104830_e157149_d_n4;
        var_wjunc0_dn5 = assign104830_e157149_d_n5;
        var_wjunc0_dn6 = assign104830_e157149_d_n6;
        var_wjunc0_dn7 = assign104830_e157149_d_n7;
        var_wjunc0_dn8 = assign104830_e157149_d_n8;
        var_wjunc0_dn9 = assign104830_e157149_d_n9;
        var_wjunc0_dn10 = assign104830_e157149_d_n10;
        var_wjunc0_dn11 = assign104830_e157149_d_n11;
        var_wjunc0_dn14 = assign104830_e157149_d_n14;

        let (assign104840_e157162, assign104840_e157162_d_n0, assign104840_e157162_d_n2, assign104840_e157162_d_n4, assign104840_e157162_d_n5, assign104840_e157162_d_n6, assign104840_e157162_d_n7, assign104840_e157162_d_n8, assign104840_e157162_d_n9, assign104840_e157162_d_n10, assign104840_e157162_d_n11, assign104840_e157162_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104840_e157156: f64 = (var_rd_xldld - var_wjunc0);
        let assign104840_e157159: f64 = (0.01 * var_rd_xldld);
        let assign104840_e157160: f64 = (assign104840_e157156 - assign104840_e157159);
        (assign104840_e157160, (-var_wjunc0_dn0), (-var_wjunc0_dn2), (-var_wjunc0_dn4), (-var_wjunc0_dn5), (-var_wjunc0_dn6), (-var_wjunc0_dn7), (-var_wjunc0_dn8), (-var_wjunc0_dn9), (-var_wjunc0_dn10), (-var_wjunc0_dn11), (-var_wjunc0_dn14),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign104840_e157162;
        var_tmf1_dn0 = assign104840_e157162_d_n0;
        var_tmf1_dn2 = assign104840_e157162_d_n2;
        var_tmf1_dn4 = assign104840_e157162_d_n4;
        var_tmf1_dn5 = assign104840_e157162_d_n5;
        var_tmf1_dn6 = assign104840_e157162_d_n6;
        var_tmf1_dn7 = assign104840_e157162_d_n7;
        var_tmf1_dn8 = assign104840_e157162_d_n8;
        var_tmf1_dn9 = assign104840_e157162_d_n9;
        var_tmf1_dn10 = assign104840_e157162_d_n10;
        var_tmf1_dn11 = assign104840_e157162_d_n11;
        var_tmf1_dn14 = assign104840_e157162_d_n14;

        let (assign104850_e157175, assign104850_e157175_d_n0, assign104850_e157175_d_n2, assign104850_e157175_d_n4, assign104850_e157175_d_n5, assign104850_e157175_d_n6, assign104850_e157175_d_n7, assign104850_e157175_d_n8, assign104850_e157175_d_n9, assign104850_e157175_d_n10, assign104850_e157175_d_n11, assign104850_e157175_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104850_e157169: f64 = (4.0 * var_rd_xldld);
        let assign104850_e157172: f64 = (0.01 * var_rd_xldld);
        let assign104850_e157173: f64 = (assign104850_e157169 * assign104850_e157172);
        (assign104850_e157173, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign104850_e157175;
        var_tmf2_dn0 = assign104850_e157175_d_n0;
        var_tmf2_dn2 = assign104850_e157175_d_n2;
        var_tmf2_dn4 = assign104850_e157175_d_n4;
        var_tmf2_dn5 = assign104850_e157175_d_n5;
        var_tmf2_dn6 = assign104850_e157175_d_n6;
        var_tmf2_dn7 = assign104850_e157175_d_n7;
        var_tmf2_dn8 = assign104850_e157175_d_n8;
        var_tmf2_dn9 = assign104850_e157175_d_n9;
        var_tmf2_dn10 = assign104850_e157175_d_n10;
        var_tmf2_dn11 = assign104850_e157175_d_n11;
        var_tmf2_dn14 = assign104850_e157175_d_n14;

        let (assign104860_e157188, assign104860_e157188_d_n0, assign104860_e157188_d_n2, assign104860_e157188_d_n4, assign104860_e157188_d_n5, assign104860_e157188_d_n6, assign104860_e157188_d_n7, assign104860_e157188_d_n8, assign104860_e157188_d_n9, assign104860_e157188_d_n10, assign104860_e157188_d_n11, assign104860_e157188_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let (assign104860_e157186, assign104860_e157186_d_n0, assign104860_e157186_d_n2, assign104860_e157186_d_n4, assign104860_e157186_d_n5, assign104860_e157186_d_n6, assign104860_e157186_d_n7, assign104860_e157186_d_n8, assign104860_e157186_d_n9, assign104860_e157186_d_n10, assign104860_e157186_d_n11, assign104860_e157186_d_n14,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
            } else {
                let assign104860_e157185: f64 = (-var_tmf2);
                (assign104860_e157185, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
            }
        };
        (assign104860_e157186, assign104860_e157186_d_n0, assign104860_e157186_d_n2, assign104860_e157186_d_n4, assign104860_e157186_d_n5, assign104860_e157186_d_n6, assign104860_e157186_d_n7, assign104860_e157186_d_n8, assign104860_e157186_d_n9, assign104860_e157186_d_n10, assign104860_e157186_d_n11, assign104860_e157186_d_n14,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign104860_e157188;
        var_tmf2_dn0 = assign104860_e157188_d_n0;
        var_tmf2_dn2 = assign104860_e157188_d_n2;
        var_tmf2_dn4 = assign104860_e157188_d_n4;
        var_tmf2_dn5 = assign104860_e157188_d_n5;
        var_tmf2_dn6 = assign104860_e157188_d_n6;
        var_tmf2_dn7 = assign104860_e157188_d_n7;
        var_tmf2_dn8 = assign104860_e157188_d_n8;
        var_tmf2_dn9 = assign104860_e157188_d_n9;
        var_tmf2_dn10 = assign104860_e157188_d_n10;
        var_tmf2_dn11 = assign104860_e157188_d_n11;
        var_tmf2_dn14 = assign104860_e157188_d_n14;

        let (assign104870_e157200, assign104870_e157200_d_n0, assign104870_e157200_d_n2, assign104870_e157200_d_n4, assign104870_e157200_d_n5, assign104870_e157200_d_n6, assign104870_e157200_d_n7, assign104870_e157200_d_n8, assign104870_e157200_d_n9, assign104870_e157200_d_n10, assign104870_e157200_d_n11, assign104870_e157200_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104870_e157195: f64 = (var_tmf1 * var_tmf1);
        let assign104870_e157197: f64 = (assign104870_e157195 + var_tmf2);
        let assign104870_e157198: f64 = (assign104870_e157197).sqrt();
        (assign104870_e157198, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign104870_e157198)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign104870_e157198)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign104870_e157198)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign104870_e157198)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign104870_e157198)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign104870_e157198)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign104870_e157198)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign104870_e157198)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign104870_e157198)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign104870_e157198)), ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign104870_e157198)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign104870_e157200;
        var_tmf2_dn0 = assign104870_e157200_d_n0;
        var_tmf2_dn2 = assign104870_e157200_d_n2;
        var_tmf2_dn4 = assign104870_e157200_d_n4;
        var_tmf2_dn5 = assign104870_e157200_d_n5;
        var_tmf2_dn6 = assign104870_e157200_d_n6;
        var_tmf2_dn7 = assign104870_e157200_d_n7;
        var_tmf2_dn8 = assign104870_e157200_d_n8;
        var_tmf2_dn9 = assign104870_e157200_d_n9;
        var_tmf2_dn10 = assign104870_e157200_d_n10;
        var_tmf2_dn11 = assign104870_e157200_d_n11;
        var_tmf2_dn14 = assign104870_e157200_d_n14;

        let (assign104880_e157213, assign104880_e157213_d_n0, assign104880_e157213_d_n2, assign104880_e157213_d_n4, assign104880_e157213_d_n5, assign104880_e157213_d_n6, assign104880_e157213_d_n7, assign104880_e157213_d_n8, assign104880_e157213_d_n9, assign104880_e157213_d_n10, assign104880_e157213_d_n11, assign104880_e157213_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104880_e157209: f64 = (var_tmf1 / var_tmf2);
        let assign104880_e157210: f64 = (1.0 + assign104880_e157209);
        let assign104880_e157211: f64 = (0.5 * assign104880_e157210);
        (assign104880_e157211, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign104880_e157213;
        var_t0_dn0 = assign104880_e157213_d_n0;
        var_t0_dn2 = assign104880_e157213_d_n2;
        var_t0_dn4 = assign104880_e157213_d_n4;
        var_t0_dn5 = assign104880_e157213_d_n5;
        var_t0_dn6 = assign104880_e157213_d_n6;
        var_t0_dn7 = assign104880_e157213_d_n7;
        var_t0_dn8 = assign104880_e157213_d_n8;
        var_t0_dn9 = assign104880_e157213_d_n9;
        var_t0_dn10 = assign104880_e157213_d_n10;
        var_t0_dn11 = assign104880_e157213_d_n11;
        var_t0_dn14 = assign104880_e157213_d_n14;

        let (assign104890_e157226, assign104890_e157226_d_n0, assign104890_e157226_d_n2, assign104890_e157226_d_n4, assign104890_e157226_d_n5, assign104890_e157226_d_n6, assign104890_e157226_d_n7, assign104890_e157226_d_n8, assign104890_e157226_d_n9, assign104890_e157226_d_n10, assign104890_e157226_d_n11, assign104890_e157226_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104890_e157222: f64 = (var_tmf1 + var_tmf2);
        let assign104890_e157223: f64 = (0.5 * assign104890_e157222);
        let assign104890_e157224: f64 = (var_rd_xldld - assign104890_e157223);
        (assign104890_e157224, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (-(0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (-(0.5 * (var_tmf1_dn8 + var_tmf2_dn8))), (-(0.5 * (var_tmf1_dn9 + var_tmf2_dn9))), (-(0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (-(0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (-(0.5 * (var_tmf1_dn14 + var_tmf2_dn14))),)
    } else {
        (var_wjunc, var_wjunc_dn0, var_wjunc_dn2, var_wjunc_dn4, var_wjunc_dn5, var_wjunc_dn6, var_wjunc_dn7, var_wjunc_dn8, var_wjunc_dn9, var_wjunc_dn10, var_wjunc_dn11, var_wjunc_dn14,)
    }
};
        var_wjunc = assign104890_e157226;
        var_wjunc_dn0 = assign104890_e157226_d_n0;
        var_wjunc_dn2 = assign104890_e157226_d_n2;
        var_wjunc_dn4 = assign104890_e157226_d_n4;
        var_wjunc_dn5 = assign104890_e157226_d_n5;
        var_wjunc_dn6 = assign104890_e157226_d_n6;
        var_wjunc_dn7 = assign104890_e157226_d_n7;
        var_wjunc_dn8 = assign104890_e157226_d_n8;
        var_wjunc_dn9 = assign104890_e157226_d_n9;
        var_wjunc_dn10 = assign104890_e157226_d_n10;
        var_wjunc_dn11 = assign104890_e157226_d_n11;
        var_wjunc_dn14 = assign104890_e157226_d_n14;

        let (assign104900_e157235,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104900_e157233: f64 = (p.p419 + 1e-25);
        (assign104900_e157233,)
    } else {
        (var_wrdrdjunc,)
    }
};
        var_wrdrdjunc = assign104900_e157235;

        let (assign104910_e157254, assign104910_e157254_d_n0, assign104910_e157254_d_n2, assign104910_e157254_d_n4, assign104910_e157254_d_n5, assign104910_e157254_d_n6, assign104910_e157254_d_n7, assign104910_e157254_d_n8, assign104910_e157254_d_n9, assign104910_e157254_d_n10, assign104910_e157254_d_n11, assign104910_e157254_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104910_e157245: f64 = (var_wdepl / var_wrdrdjunc);
        let assign104910_e157248: f64 = (var_wjunc / var_rd_xldld);
        let assign104910_e157249: f64 = (assign104910_e157245 + assign104910_e157248);
        let assign104910_e157250: f64 = (var_cx * assign104910_e157249);
        let assign104910_e157251: f64 = (1.0 - assign104910_e157250);
        let assign104910_e157252: f64 = (var_xmax * assign104910_e157251);
        (assign104910_e157252, (var_xmax * (-(var_cx * ((var_wdepl_dn0 / var_wrdrdjunc) + (var_wjunc_dn0 / var_rd_xldld))))), (var_xmax * (-(var_cx * ((var_wdepl_dn2 / var_wrdrdjunc) + (var_wjunc_dn2 / var_rd_xldld))))), (var_xmax * (-(var_cx * ((var_wdepl_dn4 / var_wrdrdjunc) + (var_wjunc_dn4 / var_rd_xldld))))), (var_xmax * (-(var_cx * ((var_wdepl_dn5 / var_wrdrdjunc) + (var_wjunc_dn5 / var_rd_xldld))))), (var_xmax * (-(var_cx * ((var_wdepl_dn6 / var_wrdrdjunc) + (var_wjunc_dn6 / var_rd_xldld))))), (var_xmax * (-(var_cx * ((var_wdepl_dn7 / var_wrdrdjunc) + (var_wjunc_dn7 / var_rd_xldld))))), (var_xmax * (-(var_cx * ((var_wdepl_dn8 / var_wrdrdjunc) + (var_wjunc_dn8 / var_rd_xldld))))), (var_xmax * (-(var_cx * ((var_wdepl_dn9 / var_wrdrdjunc) + (var_wjunc_dn9 / var_rd_xldld))))), (var_xmax * (-(var_cx * ((var_wdepl_dn10 / var_wrdrdjunc) + (var_wjunc_dn10 / var_rd_xldld))))), (var_xmax * (-(var_cx * ((var_wdepl_dn11 / var_wrdrdjunc) + (var_wjunc_dn11 / var_rd_xldld))))), (var_xmax * (-(var_cx * ((var_wdepl_dn14 / var_wrdrdjunc) + (var_wjunc_dn14 / var_rd_xldld))))),)
    } else {
        (var_xov, var_xov_dn0, var_xov_dn2, var_xov_dn4, var_xov_dn5, var_xov_dn6, var_xov_dn7, var_xov_dn8, var_xov_dn9, var_xov_dn10, var_xov_dn11, var_xov_dn14,)
    }
};
        var_xov = assign104910_e157254;
        var_xov_dn0 = assign104910_e157254_d_n0;
        var_xov_dn2 = assign104910_e157254_d_n2;
        var_xov_dn4 = assign104910_e157254_d_n4;
        var_xov_dn5 = assign104910_e157254_d_n5;
        var_xov_dn6 = assign104910_e157254_d_n6;
        var_xov_dn7 = assign104910_e157254_d_n7;
        var_xov_dn8 = assign104910_e157254_d_n8;
        var_xov_dn9 = assign104910_e157254_d_n9;
        var_xov_dn10 = assign104910_e157254_d_n10;
        var_xov_dn11 = assign104910_e157254_d_n11;
        var_xov_dn14 = assign104910_e157254_d_n14;

        let (assign104920_e157282, assign104920_e157282_d_n0, assign104920_e157282_d_n2, assign104920_e157282_d_n4, assign104920_e157282_d_n5, assign104920_e157282_d_n6, assign104920_e157282_d_n7, assign104920_e157282_d_n8, assign104920_e157282_d_n9, assign104920_e157282_d_n10, assign104920_e157282_d_n11, assign104920_e157282_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104920_e157261: f64 = (var_xov * var_xov);
        let assign104920_e157265: f64 = (1.0 - var_uc_rdrcx);
        let assign104920_e157267: f64 = (assign104920_e157265 * var_xmax);
        let assign104920_e157269: f64 = (assign104920_e157267 / 100.0);
        let assign104920_e157270: f64 = (4.0 * assign104920_e157269);
        let assign104920_e157273: f64 = (1.0 - var_uc_rdrcx);
        let assign104920_e157275: f64 = (assign104920_e157273 * var_xmax);
        let assign104920_e157277: f64 = (assign104920_e157275 / 100.0);
        let assign104920_e157278: f64 = (assign104920_e157270 * assign104920_e157277);
        let assign104920_e157279: f64 = (assign104920_e157261 + assign104920_e157278);
        let assign104920_e157280: f64 = (assign104920_e157279).sqrt();
        (assign104920_e157280, (((var_xov_dn0 * var_xov) + (var_xov * var_xov_dn0)) / (2.0 * assign104920_e157280)), (((var_xov_dn2 * var_xov) + (var_xov * var_xov_dn2)) / (2.0 * assign104920_e157280)), (((var_xov_dn4 * var_xov) + (var_xov * var_xov_dn4)) / (2.0 * assign104920_e157280)), (((var_xov_dn5 * var_xov) + (var_xov * var_xov_dn5)) / (2.0 * assign104920_e157280)), (((var_xov_dn6 * var_xov) + (var_xov * var_xov_dn6)) / (2.0 * assign104920_e157280)), (((var_xov_dn7 * var_xov) + (var_xov * var_xov_dn7)) / (2.0 * assign104920_e157280)), (((var_xov_dn8 * var_xov) + (var_xov * var_xov_dn8)) / (2.0 * assign104920_e157280)), (((var_xov_dn9 * var_xov) + (var_xov * var_xov_dn9)) / (2.0 * assign104920_e157280)), (((var_xov_dn10 * var_xov) + (var_xov * var_xov_dn10)) / (2.0 * assign104920_e157280)), (((var_xov_dn11 * var_xov) + (var_xov * var_xov_dn11)) / (2.0 * assign104920_e157280)), (((var_xov_dn14 * var_xov) + (var_xov * var_xov_dn14)) / (2.0 * assign104920_e157280)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign104920_e157282;
        var_tmf2_dn0 = assign104920_e157282_d_n0;
        var_tmf2_dn2 = assign104920_e157282_d_n2;
        var_tmf2_dn4 = assign104920_e157282_d_n4;
        var_tmf2_dn5 = assign104920_e157282_d_n5;
        var_tmf2_dn6 = assign104920_e157282_d_n6;
        var_tmf2_dn7 = assign104920_e157282_d_n7;
        var_tmf2_dn8 = assign104920_e157282_d_n8;
        var_tmf2_dn9 = assign104920_e157282_d_n9;
        var_tmf2_dn10 = assign104920_e157282_d_n10;
        var_tmf2_dn11 = assign104920_e157282_d_n11;
        var_tmf2_dn14 = assign104920_e157282_d_n14;

        let (assign104930_e157295, assign104930_e157295_d_n0, assign104930_e157295_d_n2, assign104930_e157295_d_n4, assign104930_e157295_d_n5, assign104930_e157295_d_n6, assign104930_e157295_d_n7, assign104930_e157295_d_n8, assign104930_e157295_d_n9, assign104930_e157295_d_n10, assign104930_e157295_d_n11, assign104930_e157295_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104930_e157291: f64 = (var_xov / var_tmf2);
        let assign104930_e157292: f64 = (1.0 + assign104930_e157291);
        let assign104930_e157293: f64 = (0.5 * assign104930_e157292);
        (assign104930_e157293, (0.5 * (((var_xov_dn0 * var_tmf2) - (var_xov * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_xov_dn2 * var_tmf2) - (var_xov * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_xov_dn4 * var_tmf2) - (var_xov * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_xov_dn5 * var_tmf2) - (var_xov * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_xov_dn6 * var_tmf2) - (var_xov * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_xov_dn7 * var_tmf2) - (var_xov * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_xov_dn8 * var_tmf2) - (var_xov * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_xov_dn9 * var_tmf2) - (var_xov * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_xov_dn10 * var_tmf2) - (var_xov * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_xov_dn11 * var_tmf2) - (var_xov * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_xov_dn14 * var_tmf2) - (var_xov * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_dn9, var_t9_dn10, var_t9_dn11, var_t9_dn14,)
    }
};
        var_t9 = assign104930_e157295;
        var_t9_dn0 = assign104930_e157295_d_n0;
        var_t9_dn2 = assign104930_e157295_d_n2;
        var_t9_dn4 = assign104930_e157295_d_n4;
        var_t9_dn5 = assign104930_e157295_d_n5;
        var_t9_dn6 = assign104930_e157295_d_n6;
        var_t9_dn7 = assign104930_e157295_d_n7;
        var_t9_dn8 = assign104930_e157295_d_n8;
        var_t9_dn9 = assign104930_e157295_d_n9;
        var_t9_dn10 = assign104930_e157295_d_n10;
        var_t9_dn11 = assign104930_e157295_d_n11;
        var_t9_dn14 = assign104930_e157295_d_n14;

        let (assign104940_e157306, assign104940_e157306_d_n0, assign104940_e157306_d_n2, assign104940_e157306_d_n4, assign104940_e157306_d_n5, assign104940_e157306_d_n6, assign104940_e157306_d_n7, assign104940_e157306_d_n8, assign104940_e157306_d_n9, assign104940_e157306_d_n10, assign104940_e157306_d_n11, assign104940_e157306_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104940_e157303: f64 = (var_xov + var_tmf2);
        let assign104940_e157304: f64 = (0.5 * assign104940_e157303);
        (assign104940_e157304, (0.5 * (var_xov_dn0 + var_tmf2_dn0)), (0.5 * (var_xov_dn2 + var_tmf2_dn2)), (0.5 * (var_xov_dn4 + var_tmf2_dn4)), (0.5 * (var_xov_dn5 + var_tmf2_dn5)), (0.5 * (var_xov_dn6 + var_tmf2_dn6)), (0.5 * (var_xov_dn7 + var_tmf2_dn7)), (0.5 * (var_xov_dn8 + var_tmf2_dn8)), (0.5 * (var_xov_dn9 + var_tmf2_dn9)), (0.5 * (var_xov_dn10 + var_tmf2_dn10)), (0.5 * (var_xov_dn11 + var_tmf2_dn11)), (0.5 * (var_xov_dn14 + var_tmf2_dn14)),)
    } else {
        (var_xov, var_xov_dn0, var_xov_dn2, var_xov_dn4, var_xov_dn5, var_xov_dn6, var_xov_dn7, var_xov_dn8, var_xov_dn9, var_xov_dn10, var_xov_dn11, var_xov_dn14,)
    }
};
        var_xov = assign104940_e157306;
        var_xov_dn0 = assign104940_e157306_d_n0;
        var_xov_dn2 = assign104940_e157306_d_n2;
        var_xov_dn4 = assign104940_e157306_d_n4;
        var_xov_dn5 = assign104940_e157306_d_n5;
        var_xov_dn6 = assign104940_e157306_d_n6;
        var_xov_dn7 = assign104940_e157306_d_n7;
        var_xov_dn8 = assign104940_e157306_d_n8;
        var_xov_dn9 = assign104940_e157306_d_n9;
        var_xov_dn10 = assign104940_e157306_d_n10;
        var_xov_dn11 = assign104940_e157306_d_n11;
        var_xov_dn14 = assign104940_e157306_d_n14;

        let assign104950_e157309: f64 = if var_xov < 0.0 { 1.0 } else { 0.0 };
        var_guard2384 = assign104950_e157309;

        let (assign104960_e157318, assign104960_e157318_d_n0, assign104960_e157318_d_n2, assign104960_e157318_d_n4, assign104960_e157318_d_n5, assign104960_e157318_d_n6, assign104960_e157318_d_n7, assign104960_e157318_d_n8, assign104960_e157318_d_n9, assign104960_e157318_d_n10, assign104960_e157318_d_n11, assign104960_e157318_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2384 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xov, var_xov_dn0, var_xov_dn2, var_xov_dn4, var_xov_dn5, var_xov_dn6, var_xov_dn7, var_xov_dn8, var_xov_dn9, var_xov_dn10, var_xov_dn11, var_xov_dn14,)
    }
};
        var_xov = assign104960_e157318;
        var_xov_dn0 = assign104960_e157318_d_n0;
        var_xov_dn2 = assign104960_e157318_d_n2;
        var_xov_dn4 = assign104960_e157318_d_n4;
        var_xov_dn5 = assign104960_e157318_d_n5;
        var_xov_dn6 = assign104960_e157318_d_n6;
        var_xov_dn7 = assign104960_e157318_d_n7;
        var_xov_dn8 = assign104960_e157318_d_n8;
        var_xov_dn9 = assign104960_e157318_d_n9;
        var_xov_dn10 = assign104960_e157318_d_n10;
        var_xov_dn11 = assign104960_e157318_d_n11;
        var_xov_dn14 = assign104960_e157318_d_n14;

        let (assign104970_e157327, assign104970_e157327_d_n0, assign104970_e157327_d_n2, assign104970_e157327_d_n4, assign104970_e157327_d_n5, assign104970_e157327_d_n6, assign104970_e157327_d_n7, assign104970_e157327_d_n8, assign104970_e157327_d_n9, assign104970_e157327_d_n10, assign104970_e157327_d_n11, assign104970_e157327_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2384 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_dn9, var_t9_dn10, var_t9_dn11, var_t9_dn14,)
    }
};
        var_t9 = assign104970_e157327;
        var_t9_dn0 = assign104970_e157327_d_n0;
        var_t9_dn2 = assign104970_e157327_d_n2;
        var_t9_dn4 = assign104970_e157327_d_n4;
        var_t9_dn5 = assign104970_e157327_d_n5;
        var_t9_dn6 = assign104970_e157327_d_n6;
        var_t9_dn7 = assign104970_e157327_d_n7;
        var_t9_dn8 = assign104970_e157327_d_n8;
        var_t9_dn9 = assign104970_e157327_d_n9;
        var_t9_dn10 = assign104970_e157327_d_n10;
        var_t9_dn11 = assign104970_e157327_d_n11;
        var_t9_dn14 = assign104970_e157327_d_n14;

        let (assign104980_e157338, assign104980_e157338_d_n0, assign104980_e157338_d_n2, assign104980_e157338_d_n4, assign104980_e157338_d_n5, assign104980_e157338_d_n6, assign104980_e157338_d_n7, assign104980_e157338_d_n8, assign104980_e157338_d_n9, assign104980_e157338_d_n10, assign104980_e157338_d_n11, assign104980_e157338_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104980_e157335: f64 = (var_ldrifte + p.p422);
        let assign104980_e157336: f64 = (1.6021918e-19 / assign104980_e157335);
        (assign104980_e157336, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign104980_e157338;
        var_t1_dn0 = assign104980_e157338_d_n0;
        var_t1_dn2 = assign104980_e157338_d_n2;
        var_t1_dn4 = assign104980_e157338_d_n4;
        var_t1_dn5 = assign104980_e157338_d_n5;
        var_t1_dn6 = assign104980_e157338_d_n6;
        var_t1_dn7 = assign104980_e157338_d_n7;
        var_t1_dn8 = assign104980_e157338_d_n8;
        var_t1_dn9 = assign104980_e157338_d_n9;
        var_t1_dn10 = assign104980_e157338_d_n10;
        var_t1_dn11 = assign104980_e157338_d_n11;
        var_t1_dn14 = assign104980_e157338_d_n14;

        let (assign104990_e157351, assign104990_e157351_d_n0, assign104990_e157351_d_n2, assign104990_e157351_d_n4, assign104990_e157351_d_n5, assign104990_e157351_d_n6, assign104990_e157351_d_n7, assign104990_e157351_d_n8, assign104990_e157351_d_n9, assign104990_e157351_d_n10, assign104990_e157351_d_n11, assign104990_e157351_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign104990_e157345: f64 = (var_t1 * var_xov);
        let assign104990_e157347: f64 = (assign104990_e157345 * var_mu__blk2358);
        let assign104990_e157349: f64 = (assign104990_e157347 * var_carr);
        (assign104990_e157349, ((((((var_t1_dn0 * var_xov) + (var_t1 * var_xov_dn0)) * var_mu__blk2358) + (assign104990_e157345 * var_mu__blk2358_dn0)) * var_carr) + (assign104990_e157347 * var_carr_dn0)), ((((((var_t1_dn2 * var_xov) + (var_t1 * var_xov_dn2)) * var_mu__blk2358) + (assign104990_e157345 * var_mu__blk2358_dn2)) * var_carr) + (assign104990_e157347 * var_carr_dn2)), ((((((var_t1_dn4 * var_xov) + (var_t1 * var_xov_dn4)) * var_mu__blk2358) + (assign104990_e157345 * var_mu__blk2358_dn4)) * var_carr) + (assign104990_e157347 * var_carr_dn4)), ((((((var_t1_dn5 * var_xov) + (var_t1 * var_xov_dn5)) * var_mu__blk2358) + (assign104990_e157345 * var_mu__blk2358_dn5)) * var_carr) + (assign104990_e157347 * var_carr_dn5)), ((((((var_t1_dn6 * var_xov) + (var_t1 * var_xov_dn6)) * var_mu__blk2358) + (assign104990_e157345 * var_mu__blk2358_dn6)) * var_carr) + (assign104990_e157347 * var_carr_dn6)), ((((((var_t1_dn7 * var_xov) + (var_t1 * var_xov_dn7)) * var_mu__blk2358) + (assign104990_e157345 * var_mu__blk2358_dn7)) * var_carr) + (assign104990_e157347 * var_carr_dn7)), ((((((var_t1_dn8 * var_xov) + (var_t1 * var_xov_dn8)) * var_mu__blk2358) + (assign104990_e157345 * var_mu__blk2358_dn8)) * var_carr) + (assign104990_e157347 * var_carr_dn8)), ((((((var_t1_dn9 * var_xov) + (var_t1 * var_xov_dn9)) * var_mu__blk2358) + (assign104990_e157345 * var_mu__blk2358_dn9)) * var_carr) + (assign104990_e157347 * var_carr_dn9)), ((((((var_t1_dn10 * var_xov) + (var_t1 * var_xov_dn10)) * var_mu__blk2358) + (assign104990_e157345 * var_mu__blk2358_dn10)) * var_carr) + (assign104990_e157347 * var_carr_dn10)), ((((((var_t1_dn11 * var_xov) + (var_t1 * var_xov_dn11)) * var_mu__blk2358) + (assign104990_e157345 * var_mu__blk2358_dn11)) * var_carr) + (assign104990_e157347 * var_carr_dn11)), ((((((var_t1_dn14 * var_xov) + (var_t1 * var_xov_dn14)) * var_mu__blk2358) + (assign104990_e157345 * var_mu__blk2358_dn14)) * var_carr) + (assign104990_e157347 * var_carr_dn14)),)
    } else {
        (var_gd, var_gd_dn0, var_gd_dn2, var_gd_dn4, var_gd_dn5, var_gd_dn6, var_gd_dn7, var_gd_dn8, var_gd_dn9, var_gd_dn10, var_gd_dn11, var_gd_dn14,)
    }
};
        var_gd = assign104990_e157351;
        var_gd_dn0 = assign104990_e157351_d_n0;
        var_gd_dn2 = assign104990_e157351_d_n2;
        var_gd_dn4 = assign104990_e157351_d_n4;
        var_gd_dn5 = assign104990_e157351_d_n5;
        var_gd_dn6 = assign104990_e157351_d_n6;
        var_gd_dn7 = assign104990_e157351_d_n7;
        var_gd_dn8 = assign104990_e157351_d_n8;
        var_gd_dn9 = assign104990_e157351_d_n9;
        var_gd_dn10 = assign104990_e157351_d_n10;
        var_gd_dn11 = assign104990_e157351_d_n11;
        var_gd_dn14 = assign104990_e157351_d_n14;

        let assign105000_e157355: f64 = 1e-25;
        let assign105000_e157360: f64 = if ((var_gd < assign105000_e157355) && (1e-25 >= 0.0)) { 1.0 } else { 0.0 };
        var_guard2385 = assign105000_e157360;

        let (assign105010_e157373, assign105010_e157373_d_n0, assign105010_e157373_d_n2, assign105010_e157373_d_n4, assign105010_e157373_d_n5, assign105010_e157373_d_n6, assign105010_e157373_d_n7, assign105010_e157373_d_n8, assign105010_e157373_d_n9, assign105010_e157373_d_n10, assign105010_e157373_d_n11, assign105010_e157373_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        let assign105010_e157369: f64 = 1e-25;
        let assign105010_e157371: f64 = (assign105010_e157369 - var_gd);
        (assign105010_e157371, (-var_gd_dn0), (-var_gd_dn2), (-var_gd_dn4), (-var_gd_dn5), (-var_gd_dn6), (-var_gd_dn7), (-var_gd_dn8), (-var_gd_dn9), (-var_gd_dn10), (-var_gd_dn11), (-var_gd_dn14),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign105010_e157373;
        var_tmf1_dn0 = assign105010_e157373_d_n0;
        var_tmf1_dn2 = assign105010_e157373_d_n2;
        var_tmf1_dn4 = assign105010_e157373_d_n4;
        var_tmf1_dn5 = assign105010_e157373_d_n5;
        var_tmf1_dn6 = assign105010_e157373_d_n6;
        var_tmf1_dn7 = assign105010_e157373_d_n7;
        var_tmf1_dn8 = assign105010_e157373_d_n8;
        var_tmf1_dn9 = assign105010_e157373_d_n9;
        var_tmf1_dn10 = assign105010_e157373_d_n10;
        var_tmf1_dn11 = assign105010_e157373_d_n11;
        var_tmf1_dn14 = assign105010_e157373_d_n14;

        let (assign105020_e157384, assign105020_e157384_d_n0, assign105020_e157384_d_n2, assign105020_e157384_d_n4, assign105020_e157384_d_n5, assign105020_e157384_d_n6, assign105020_e157384_d_n7, assign105020_e157384_d_n8, assign105020_e157384_d_n9, assign105020_e157384_d_n10, assign105020_e157384_d_n11, assign105020_e157384_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        let assign105020_e157382: f64 = (var_tmf1 * var_tmf1);
        (assign105020_e157382, ((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)), ((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)), ((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)), ((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)), ((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)), ((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)), ((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)), ((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)), ((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)), ((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)), ((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)),)
    } else {
        (var_x2, var_x2_dn0, var_x2_dn2, var_x2_dn4, var_x2_dn5, var_x2_dn6, var_x2_dn7, var_x2_dn8, var_x2_dn9, var_x2_dn10, var_x2_dn11, var_x2_dn14,)
    }
};
        var_x2 = assign105020_e157384;
        var_x2_dn0 = assign105020_e157384_d_n0;
        var_x2_dn2 = assign105020_e157384_d_n2;
        var_x2_dn4 = assign105020_e157384_d_n4;
        var_x2_dn5 = assign105020_e157384_d_n5;
        var_x2_dn6 = assign105020_e157384_d_n6;
        var_x2_dn7 = assign105020_e157384_d_n7;
        var_x2_dn8 = assign105020_e157384_d_n8;
        var_x2_dn9 = assign105020_e157384_d_n9;
        var_x2_dn10 = assign105020_e157384_d_n10;
        var_x2_dn11 = assign105020_e157384_d_n11;
        var_x2_dn14 = assign105020_e157384_d_n14;

        let (assign105030_e157395, assign105030_e157395_d_n0, assign105030_e157395_d_n2, assign105030_e157395_d_n4, assign105030_e157395_d_n5, assign105030_e157395_d_n6, assign105030_e157395_d_n7, assign105030_e157395_d_n8, assign105030_e157395_d_n9, assign105030_e157395_d_n10, assign105030_e157395_d_n11, assign105030_e157395_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        let assign105030_e157393: f64 = (1e-25 * 1e-25);
        (assign105030_e157393, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmax2, var_xmax2_dn0, var_xmax2_dn2, var_xmax2_dn4, var_xmax2_dn5, var_xmax2_dn6, var_xmax2_dn7, var_xmax2_dn8, var_xmax2_dn9, var_xmax2_dn10, var_xmax2_dn11, var_xmax2_dn14,)
    }
};
        var_xmax2 = assign105030_e157395;
        var_xmax2_dn0 = assign105030_e157395_d_n0;
        var_xmax2_dn2 = assign105030_e157395_d_n2;
        var_xmax2_dn4 = assign105030_e157395_d_n4;
        var_xmax2_dn5 = assign105030_e157395_d_n5;
        var_xmax2_dn6 = assign105030_e157395_d_n6;
        var_xmax2_dn7 = assign105030_e157395_d_n7;
        var_xmax2_dn8 = assign105030_e157395_d_n8;
        var_xmax2_dn9 = assign105030_e157395_d_n9;
        var_xmax2_dn10 = assign105030_e157395_d_n10;
        var_xmax2_dn11 = assign105030_e157395_d_n11;
        var_xmax2_dn14 = assign105030_e157395_d_n14;

        let (assign105040_e157404, assign105040_e157404_d_n0, assign105040_e157404_d_n2, assign105040_e157404_d_n4, assign105040_e157404_d_n5, assign105040_e157404_d_n6, assign105040_e157404_d_n7, assign105040_e157404_d_n8, assign105040_e157404_d_n9, assign105040_e157404_d_n10, assign105040_e157404_d_n11, assign105040_e157404_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn11, var_xp_dn14,)
    }
};
        var_xp = assign105040_e157404;
        var_xp_dn0 = assign105040_e157404_d_n0;
        var_xp_dn2 = assign105040_e157404_d_n2;
        var_xp_dn4 = assign105040_e157404_d_n4;
        var_xp_dn5 = assign105040_e157404_d_n5;
        var_xp_dn6 = assign105040_e157404_d_n6;
        var_xp_dn7 = assign105040_e157404_d_n7;
        var_xp_dn8 = assign105040_e157404_d_n8;
        var_xp_dn9 = assign105040_e157404_d_n9;
        var_xp_dn10 = assign105040_e157404_d_n10;
        var_xp_dn11 = assign105040_e157404_d_n11;
        var_xp_dn14 = assign105040_e157404_d_n14;

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
        *var_guard2384_slot = var_guard2384;
        *var_guard2385_slot = var_guard2385;
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
        *var_t9_slot = var_t9;
        *var_t9_dn0_slot = var_t9_dn0;
        *var_t9_dn10_slot = var_t9_dn10;
        *var_t9_dn11_slot = var_t9_dn11;
        *var_t9_dn14_slot = var_t9_dn14;
        *var_t9_dn2_slot = var_t9_dn2;
        *var_t9_dn4_slot = var_t9_dn4;
        *var_t9_dn5_slot = var_t9_dn5;
        *var_t9_dn6_slot = var_t9_dn6;
        *var_t9_dn7_slot = var_t9_dn7;
        *var_t9_dn8_slot = var_t9_dn8;
        *var_t9_dn9_slot = var_t9_dn9;
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
        *var_wjunc_slot = var_wjunc;
        *var_wjunc0_slot = var_wjunc0;
        *var_wjunc0_dn0_slot = var_wjunc0_dn0;
        *var_wjunc0_dn10_slot = var_wjunc0_dn10;
        *var_wjunc0_dn11_slot = var_wjunc0_dn11;
        *var_wjunc0_dn14_slot = var_wjunc0_dn14;
        *var_wjunc0_dn2_slot = var_wjunc0_dn2;
        *var_wjunc0_dn4_slot = var_wjunc0_dn4;
        *var_wjunc0_dn5_slot = var_wjunc0_dn5;
        *var_wjunc0_dn6_slot = var_wjunc0_dn6;
        *var_wjunc0_dn7_slot = var_wjunc0_dn7;
        *var_wjunc0_dn8_slot = var_wjunc0_dn8;
        *var_wjunc0_dn9_slot = var_wjunc0_dn9;
        *var_wjunc_dn0_slot = var_wjunc_dn0;
        *var_wjunc_dn10_slot = var_wjunc_dn10;
        *var_wjunc_dn11_slot = var_wjunc_dn11;
        *var_wjunc_dn14_slot = var_wjunc_dn14;
        *var_wjunc_dn2_slot = var_wjunc_dn2;
        *var_wjunc_dn4_slot = var_wjunc_dn4;
        *var_wjunc_dn5_slot = var_wjunc_dn5;
        *var_wjunc_dn6_slot = var_wjunc_dn6;
        *var_wjunc_dn7_slot = var_wjunc_dn7;
        *var_wjunc_dn8_slot = var_wjunc_dn8;
        *var_wjunc_dn9_slot = var_wjunc_dn9;
        *var_wrdrdjunc_slot = var_wrdrdjunc;
        *var_x2_slot = var_x2;
        *var_x2_dn0_slot = var_x2_dn0;
        *var_x2_dn10_slot = var_x2_dn10;
        *var_x2_dn11_slot = var_x2_dn11;
        *var_x2_dn14_slot = var_x2_dn14;
        *var_x2_dn2_slot = var_x2_dn2;
        *var_x2_dn4_slot = var_x2_dn4;
        *var_x2_dn5_slot = var_x2_dn5;
        *var_x2_dn6_slot = var_x2_dn6;
        *var_x2_dn7_slot = var_x2_dn7;
        *var_x2_dn8_slot = var_x2_dn8;
        *var_x2_dn9_slot = var_x2_dn9;
        *var_xmax2_slot = var_xmax2;
        *var_xmax2_dn0_slot = var_xmax2_dn0;
        *var_xmax2_dn10_slot = var_xmax2_dn10;
        *var_xmax2_dn11_slot = var_xmax2_dn11;
        *var_xmax2_dn14_slot = var_xmax2_dn14;
        *var_xmax2_dn2_slot = var_xmax2_dn2;
        *var_xmax2_dn4_slot = var_xmax2_dn4;
        *var_xmax2_dn5_slot = var_xmax2_dn5;
        *var_xmax2_dn6_slot = var_xmax2_dn6;
        *var_xmax2_dn7_slot = var_xmax2_dn7;
        *var_xmax2_dn8_slot = var_xmax2_dn8;
        *var_xmax2_dn9_slot = var_xmax2_dn9;
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
        *var_xp_slot = var_xp;
        *var_xp_dn0_slot = var_xp_dn0;
        *var_xp_dn10_slot = var_xp_dn10;
        *var_xp_dn11_slot = var_xp_dn11;
        *var_xp_dn14_slot = var_xp_dn14;
        *var_xp_dn2_slot = var_xp_dn2;
        *var_xp_dn4_slot = var_xp_dn4;
        *var_xp_dn5_slot = var_xp_dn5;
        *var_xp_dn6_slot = var_xp_dn6;
        *var_xp_dn7_slot = var_xp_dn7;
        *var_xp_dn8_slot = var_xp_dn8;
        *var_xp_dn9_slot = var_xp_dn9;
    }

    pub(super) fn stamp_transient_block_386(
        var_guard2340: f64,
        var_guard2360: f64,
        var_guard2385: f64,
        var_tmf1: f64,
        var_tmf1_dn0: f64,
        var_tmf1_dn10: f64,
        var_tmf1_dn11: f64,
        var_tmf1_dn14: f64,
        var_tmf1_dn2: f64,
        var_tmf1_dn4: f64,
        var_tmf1_dn5: f64,
        var_tmf1_dn6: f64,
        var_tmf1_dn7: f64,
        var_tmf1_dn8: f64,
        var_tmf1_dn9: f64,
        var_x2: f64,
        var_x2_dn0: f64,
        var_x2_dn10: f64,
        var_x2_dn11: f64,
        var_x2_dn14: f64,
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
        var_xmax2_dn11: f64,
        var_xmax2_dn14: f64,
        var_xmax2_dn2: f64,
        var_xmax2_dn4: f64,
        var_xmax2_dn5: f64,
        var_xmax2_dn6: f64,
        var_xmax2_dn7: f64,
        var_xmax2_dn8: f64,
        var_xmax2_dn9: f64,
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
        var_dnm_slot: &mut f64,
        var_dnm_dn0_slot: &mut f64,
        var_dnm_dn10_slot: &mut f64,
        var_dnm_dn11_slot: &mut f64,
        var_dnm_dn14_slot: &mut f64,
        var_dnm_dn2_slot: &mut f64,
        var_dnm_dn4_slot: &mut f64,
        var_dnm_dn5_slot: &mut f64,
        var_dnm_dn6_slot: &mut f64,
        var_dnm_dn7_slot: &mut f64,
        var_dnm_dn8_slot: &mut f64,
        var_dnm_dn9_slot: &mut f64,
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
        var_guard2386_slot: &mut f64,
        var_guard2387_slot: &mut f64,
        var_guard2388_slot: &mut f64,
        var_guard2389_slot: &mut f64,
        var_guard2390_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_mm_slot: &mut f64,
        var_rdd_slot: &mut f64,
        var_rdd_dn0_slot: &mut f64,
        var_rdd_dn10_slot: &mut f64,
        var_rdd_dn11_slot: &mut f64,
        var_rdd_dn14_slot: &mut f64,
        var_rdd_dn2_slot: &mut f64,
        var_rdd_dn4_slot: &mut f64,
        var_rdd_dn5_slot: &mut f64,
        var_rdd_dn6_slot: &mut f64,
        var_rdd_dn7_slot: &mut f64,
        var_rdd_dn8_slot: &mut f64,
        var_rdd_dn9_slot: &mut f64,
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
        var_tmf0_slot: &mut f64,
        var_tmf0_dn0_slot: &mut f64,
        var_tmf0_dn10_slot: &mut f64,
        var_tmf0_dn11_slot: &mut f64,
        var_tmf0_dn14_slot: &mut f64,
        var_tmf0_dn2_slot: &mut f64,
        var_tmf0_dn4_slot: &mut f64,
        var_tmf0_dn5_slot: &mut f64,
        var_tmf0_dn6_slot: &mut f64,
        var_tmf0_dn7_slot: &mut f64,
        var_tmf0_dn8_slot: &mut f64,
        var_tmf0_dn9_slot: &mut f64,
        var_xmp_slot: &mut f64,
        var_xmp_dn0_slot: &mut f64,
        var_xmp_dn10_slot: &mut f64,
        var_xmp_dn11_slot: &mut f64,
        var_xmp_dn14_slot: &mut f64,
        var_xmp_dn2_slot: &mut f64,
        var_xmp_dn4_slot: &mut f64,
        var_xmp_dn5_slot: &mut f64,
        var_xmp_dn6_slot: &mut f64,
        var_xmp_dn7_slot: &mut f64,
        var_xmp_dn8_slot: &mut f64,
        var_xmp_dn9_slot: &mut f64,
        var_xp_slot: &mut f64,
        var_xp_dn0_slot: &mut f64,
        var_xp_dn10_slot: &mut f64,
        var_xp_dn11_slot: &mut f64,
        var_xp_dn14_slot: &mut f64,
        var_xp_dn2_slot: &mut f64,
        var_xp_dn4_slot: &mut f64,
        var_xp_dn5_slot: &mut f64,
        var_xp_dn6_slot: &mut f64,
        var_xp_dn7_slot: &mut f64,
        var_xp_dn8_slot: &mut f64,
        var_xp_dn9_slot: &mut f64,
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
        let mut var_dnm: f64 = *var_dnm_slot;
        let mut var_dnm_dn0: f64 = *var_dnm_dn0_slot;
        let mut var_dnm_dn10: f64 = *var_dnm_dn10_slot;
        let mut var_dnm_dn11: f64 = *var_dnm_dn11_slot;
        let mut var_dnm_dn14: f64 = *var_dnm_dn14_slot;
        let mut var_dnm_dn2: f64 = *var_dnm_dn2_slot;
        let mut var_dnm_dn4: f64 = *var_dnm_dn4_slot;
        let mut var_dnm_dn5: f64 = *var_dnm_dn5_slot;
        let mut var_dnm_dn6: f64 = *var_dnm_dn6_slot;
        let mut var_dnm_dn7: f64 = *var_dnm_dn7_slot;
        let mut var_dnm_dn8: f64 = *var_dnm_dn8_slot;
        let mut var_dnm_dn9: f64 = *var_dnm_dn9_slot;
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
        let mut var_guard2386: f64 = *var_guard2386_slot;
        let mut var_guard2387: f64 = *var_guard2387_slot;
        let mut var_guard2388: f64 = *var_guard2388_slot;
        let mut var_guard2389: f64 = *var_guard2389_slot;
        let mut var_guard2390: f64 = *var_guard2390_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_mm: f64 = *var_mm_slot;
        let mut var_rdd: f64 = *var_rdd_slot;
        let mut var_rdd_dn0: f64 = *var_rdd_dn0_slot;
        let mut var_rdd_dn10: f64 = *var_rdd_dn10_slot;
        let mut var_rdd_dn11: f64 = *var_rdd_dn11_slot;
        let mut var_rdd_dn14: f64 = *var_rdd_dn14_slot;
        let mut var_rdd_dn2: f64 = *var_rdd_dn2_slot;
        let mut var_rdd_dn4: f64 = *var_rdd_dn4_slot;
        let mut var_rdd_dn5: f64 = *var_rdd_dn5_slot;
        let mut var_rdd_dn6: f64 = *var_rdd_dn6_slot;
        let mut var_rdd_dn7: f64 = *var_rdd_dn7_slot;
        let mut var_rdd_dn8: f64 = *var_rdd_dn8_slot;
        let mut var_rdd_dn9: f64 = *var_rdd_dn9_slot;
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
        let mut var_tmf0: f64 = *var_tmf0_slot;
        let mut var_tmf0_dn0: f64 = *var_tmf0_dn0_slot;
        let mut var_tmf0_dn10: f64 = *var_tmf0_dn10_slot;
        let mut var_tmf0_dn11: f64 = *var_tmf0_dn11_slot;
        let mut var_tmf0_dn14: f64 = *var_tmf0_dn14_slot;
        let mut var_tmf0_dn2: f64 = *var_tmf0_dn2_slot;
        let mut var_tmf0_dn4: f64 = *var_tmf0_dn4_slot;
        let mut var_tmf0_dn5: f64 = *var_tmf0_dn5_slot;
        let mut var_tmf0_dn6: f64 = *var_tmf0_dn6_slot;
        let mut var_tmf0_dn7: f64 = *var_tmf0_dn7_slot;
        let mut var_tmf0_dn8: f64 = *var_tmf0_dn8_slot;
        let mut var_tmf0_dn9: f64 = *var_tmf0_dn9_slot;
        let mut var_xmp: f64 = *var_xmp_slot;
        let mut var_xmp_dn0: f64 = *var_xmp_dn0_slot;
        let mut var_xmp_dn10: f64 = *var_xmp_dn10_slot;
        let mut var_xmp_dn11: f64 = *var_xmp_dn11_slot;
        let mut var_xmp_dn14: f64 = *var_xmp_dn14_slot;
        let mut var_xmp_dn2: f64 = *var_xmp_dn2_slot;
        let mut var_xmp_dn4: f64 = *var_xmp_dn4_slot;
        let mut var_xmp_dn5: f64 = *var_xmp_dn5_slot;
        let mut var_xmp_dn6: f64 = *var_xmp_dn6_slot;
        let mut var_xmp_dn7: f64 = *var_xmp_dn7_slot;
        let mut var_xmp_dn8: f64 = *var_xmp_dn8_slot;
        let mut var_xmp_dn9: f64 = *var_xmp_dn9_slot;
        let mut var_xp: f64 = *var_xp_slot;
        let mut var_xp_dn0: f64 = *var_xp_dn0_slot;
        let mut var_xp_dn10: f64 = *var_xp_dn10_slot;
        let mut var_xp_dn11: f64 = *var_xp_dn11_slot;
        let mut var_xp_dn14: f64 = *var_xp_dn14_slot;
        let mut var_xp_dn2: f64 = *var_xp_dn2_slot;
        let mut var_xp_dn4: f64 = *var_xp_dn4_slot;
        let mut var_xp_dn5: f64 = *var_xp_dn5_slot;
        let mut var_xp_dn6: f64 = *var_xp_dn6_slot;
        let mut var_xp_dn7: f64 = *var_xp_dn7_slot;
        let mut var_xp_dn8: f64 = *var_xp_dn8_slot;
        let mut var_xp_dn9: f64 = *var_xp_dn9_slot;

        let (assign105050_e157413, assign105050_e157413_d_n0, assign105050_e157413_d_n2, assign105050_e157413_d_n4, assign105050_e157413_d_n5, assign105050_e157413_d_n6, assign105050_e157413_d_n7, assign105050_e157413_d_n8, assign105050_e157413_d_n9, assign105050_e157413_d_n10, assign105050_e157413_d_n11, assign105050_e157413_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn11, var_xmp_dn14,)
    }
};
        var_xmp = assign105050_e157413;
        var_xmp_dn0 = assign105050_e157413_d_n0;
        var_xmp_dn2 = assign105050_e157413_d_n2;
        var_xmp_dn4 = assign105050_e157413_d_n4;
        var_xmp_dn5 = assign105050_e157413_d_n5;
        var_xmp_dn6 = assign105050_e157413_d_n6;
        var_xmp_dn7 = assign105050_e157413_d_n7;
        var_xmp_dn8 = assign105050_e157413_d_n8;
        var_xmp_dn9 = assign105050_e157413_d_n9;
        var_xmp_dn10 = assign105050_e157413_d_n10;
        var_xmp_dn11 = assign105050_e157413_d_n11;
        var_xmp_dn14 = assign105050_e157413_d_n14;

        let (assign105060_e157422,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign105060_e157422;

        let (assign105070_e157431,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        (0.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105070_e157431;

        let (assign105080_e157440, assign105080_e157440_d_n0, assign105080_e157440_d_n2, assign105080_e157440_d_n4, assign105080_e157440_d_n5, assign105080_e157440_d_n6, assign105080_e157440_d_n7, assign105080_e157440_d_n8, assign105080_e157440_d_n9, assign105080_e157440_d_n10, assign105080_e157440_d_n11, assign105080_e157440_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn11, var_arg_dn14,)
    }
};
        var_arg = assign105080_e157440;
        var_arg_dn0 = assign105080_e157440_d_n0;
        var_arg_dn2 = assign105080_e157440_d_n2;
        var_arg_dn4 = assign105080_e157440_d_n4;
        var_arg_dn5 = assign105080_e157440_d_n5;
        var_arg_dn6 = assign105080_e157440_d_n6;
        var_arg_dn7 = assign105080_e157440_d_n7;
        var_arg_dn8 = assign105080_e157440_d_n8;
        var_arg_dn9 = assign105080_e157440_d_n9;
        var_arg_dn10 = assign105080_e157440_d_n10;
        var_arg_dn11 = assign105080_e157440_d_n11;
        var_arg_dn14 = assign105080_e157440_d_n14;

        let (assign105090_e157449, assign105090_e157449_d_n0, assign105090_e157449_d_n2, assign105090_e157449_d_n4, assign105090_e157449_d_n5, assign105090_e157449_d_n6, assign105090_e157449_d_n7, assign105090_e157449_d_n8, assign105090_e157449_d_n9, assign105090_e157449_d_n10, assign105090_e157449_d_n11, assign105090_e157449_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn11, var_dnm_dn14,)
    }
};
        var_dnm = assign105090_e157449;
        var_dnm_dn0 = assign105090_e157449_d_n0;
        var_dnm_dn2 = assign105090_e157449_d_n2;
        var_dnm_dn4 = assign105090_e157449_d_n4;
        var_dnm_dn5 = assign105090_e157449_d_n5;
        var_dnm_dn6 = assign105090_e157449_d_n6;
        var_dnm_dn7 = assign105090_e157449_d_n7;
        var_dnm_dn8 = assign105090_e157449_d_n8;
        var_dnm_dn9 = assign105090_e157449_d_n9;
        var_dnm_dn10 = assign105090_e157449_d_n10;
        var_dnm_dn11 = assign105090_e157449_d_n11;
        var_dnm_dn14 = assign105090_e157449_d_n14;

        let (assign105100_e157460, assign105100_e157460_d_n0, assign105100_e157460_d_n2, assign105100_e157460_d_n4, assign105100_e157460_d_n5, assign105100_e157460_d_n6, assign105100_e157460_d_n7, assign105100_e157460_d_n8, assign105100_e157460_d_n9, assign105100_e157460_d_n10, assign105100_e157460_d_n11, assign105100_e157460_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        let assign105100_e157458: f64 = (var_xp * var_x2);
        (assign105100_e157458, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn14 * var_x2) + (var_xp * var_x2_dn14)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn11, var_xp_dn14,)
    }
};
        var_xp = assign105100_e157460;
        var_xp_dn0 = assign105100_e157460_d_n0;
        var_xp_dn2 = assign105100_e157460_d_n2;
        var_xp_dn4 = assign105100_e157460_d_n4;
        var_xp_dn5 = assign105100_e157460_d_n5;
        var_xp_dn6 = assign105100_e157460_d_n6;
        var_xp_dn7 = assign105100_e157460_d_n7;
        var_xp_dn8 = assign105100_e157460_d_n8;
        var_xp_dn9 = assign105100_e157460_d_n9;
        var_xp_dn10 = assign105100_e157460_d_n10;
        var_xp_dn11 = assign105100_e157460_d_n11;
        var_xp_dn14 = assign105100_e157460_d_n14;

        let (assign105110_e157471, assign105110_e157471_d_n0, assign105110_e157471_d_n2, assign105110_e157471_d_n4, assign105110_e157471_d_n5, assign105110_e157471_d_n6, assign105110_e157471_d_n7, assign105110_e157471_d_n8, assign105110_e157471_d_n9, assign105110_e157471_d_n10, assign105110_e157471_d_n11, assign105110_e157471_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        let assign105110_e157469: f64 = (var_xmp * var_xmax2);
        (assign105110_e157469, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn14 * var_xmax2) + (var_xmp * var_xmax2_dn14)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn11, var_xmp_dn14,)
    }
};
        var_xmp = assign105110_e157471;
        var_xmp_dn0 = assign105110_e157471_d_n0;
        var_xmp_dn2 = assign105110_e157471_d_n2;
        var_xmp_dn4 = assign105110_e157471_d_n4;
        var_xmp_dn5 = assign105110_e157471_d_n5;
        var_xmp_dn6 = assign105110_e157471_d_n6;
        var_xmp_dn7 = assign105110_e157471_d_n7;
        var_xmp_dn8 = assign105110_e157471_d_n8;
        var_xmp_dn9 = assign105110_e157471_d_n9;
        var_xmp_dn10 = assign105110_e157471_d_n10;
        var_xmp_dn11 = assign105110_e157471_d_n11;
        var_xmp_dn14 = assign105110_e157471_d_n14;

        let (assign105120_e157482, assign105120_e157482_d_n0, assign105120_e157482_d_n2, assign105120_e157482_d_n4, assign105120_e157482_d_n5, assign105120_e157482_d_n6, assign105120_e157482_d_n7, assign105120_e157482_d_n8, assign105120_e157482_d_n9, assign105120_e157482_d_n10, assign105120_e157482_d_n11, assign105120_e157482_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        let assign105120_e157480: f64 = (var_xp * var_x2);
        (assign105120_e157480, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn14 * var_x2) + (var_xp * var_x2_dn14)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn11, var_xp_dn14,)
    }
};
        var_xp = assign105120_e157482;
        var_xp_dn0 = assign105120_e157482_d_n0;
        var_xp_dn2 = assign105120_e157482_d_n2;
        var_xp_dn4 = assign105120_e157482_d_n4;
        var_xp_dn5 = assign105120_e157482_d_n5;
        var_xp_dn6 = assign105120_e157482_d_n6;
        var_xp_dn7 = assign105120_e157482_d_n7;
        var_xp_dn8 = assign105120_e157482_d_n8;
        var_xp_dn9 = assign105120_e157482_d_n9;
        var_xp_dn10 = assign105120_e157482_d_n10;
        var_xp_dn11 = assign105120_e157482_d_n11;
        var_xp_dn14 = assign105120_e157482_d_n14;

        let (assign105130_e157493, assign105130_e157493_d_n0, assign105130_e157493_d_n2, assign105130_e157493_d_n4, assign105130_e157493_d_n5, assign105130_e157493_d_n6, assign105130_e157493_d_n7, assign105130_e157493_d_n8, assign105130_e157493_d_n9, assign105130_e157493_d_n10, assign105130_e157493_d_n11, assign105130_e157493_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        let assign105130_e157491: f64 = (var_xmp * var_xmax2);
        (assign105130_e157491, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn14 * var_xmax2) + (var_xmp * var_xmax2_dn14)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn11, var_xmp_dn14,)
    }
};
        var_xmp = assign105130_e157493;
        var_xmp_dn0 = assign105130_e157493_d_n0;
        var_xmp_dn2 = assign105130_e157493_d_n2;
        var_xmp_dn4 = assign105130_e157493_d_n4;
        var_xmp_dn5 = assign105130_e157493_d_n5;
        var_xmp_dn6 = assign105130_e157493_d_n6;
        var_xmp_dn7 = assign105130_e157493_d_n7;
        var_xmp_dn8 = assign105130_e157493_d_n8;
        var_xmp_dn9 = assign105130_e157493_d_n9;
        var_xmp_dn10 = assign105130_e157493_d_n10;
        var_xmp_dn11 = assign105130_e157493_d_n11;
        var_xmp_dn14 = assign105130_e157493_d_n14;

        let (assign105140_e157504, assign105140_e157504_d_n0, assign105140_e157504_d_n2, assign105140_e157504_d_n4, assign105140_e157504_d_n5, assign105140_e157504_d_n6, assign105140_e157504_d_n7, assign105140_e157504_d_n8, assign105140_e157504_d_n9, assign105140_e157504_d_n10, assign105140_e157504_d_n11, assign105140_e157504_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        let assign105140_e157502: f64 = (var_xp + var_xmp);
        (assign105140_e157502, (var_xp_dn0 + var_xmp_dn0), (var_xp_dn2 + var_xmp_dn2), (var_xp_dn4 + var_xmp_dn4), (var_xp_dn5 + var_xmp_dn5), (var_xp_dn6 + var_xmp_dn6), (var_xp_dn7 + var_xmp_dn7), (var_xp_dn8 + var_xmp_dn8), (var_xp_dn9 + var_xmp_dn9), (var_xp_dn10 + var_xmp_dn10), (var_xp_dn11 + var_xmp_dn11), (var_xp_dn14 + var_xmp_dn14),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn11, var_arg_dn14,)
    }
};
        var_arg = assign105140_e157504;
        var_arg_dn0 = assign105140_e157504_d_n0;
        var_arg_dn2 = assign105140_e157504_d_n2;
        var_arg_dn4 = assign105140_e157504_d_n4;
        var_arg_dn5 = assign105140_e157504_d_n5;
        var_arg_dn6 = assign105140_e157504_d_n6;
        var_arg_dn7 = assign105140_e157504_d_n7;
        var_arg_dn8 = assign105140_e157504_d_n8;
        var_arg_dn9 = assign105140_e157504_d_n9;
        var_arg_dn10 = assign105140_e157504_d_n10;
        var_arg_dn11 = assign105140_e157504_d_n11;
        var_arg_dn14 = assign105140_e157504_d_n14;

        let (assign105150_e157513, assign105150_e157513_d_n0, assign105150_e157513_d_n2, assign105150_e157513_d_n4, assign105150_e157513_d_n5, assign105150_e157513_d_n6, assign105150_e157513_d_n7, assign105150_e157513_d_n8, assign105150_e157513_d_n9, assign105150_e157513_d_n10, assign105150_e157513_d_n11, assign105150_e157513_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn11, var_arg_dn14,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn11, var_dnm_dn14,)
    }
};
        var_dnm = assign105150_e157513;
        var_dnm_dn0 = assign105150_e157513_d_n0;
        var_dnm_dn2 = assign105150_e157513_d_n2;
        var_dnm_dn4 = assign105150_e157513_d_n4;
        var_dnm_dn5 = assign105150_e157513_d_n5;
        var_dnm_dn6 = assign105150_e157513_d_n6;
        var_dnm_dn7 = assign105150_e157513_d_n7;
        var_dnm_dn8 = assign105150_e157513_d_n8;
        var_dnm_dn9 = assign105150_e157513_d_n9;
        var_dnm_dn10 = assign105150_e157513_d_n10;
        var_dnm_dn11 = assign105150_e157513_d_n11;
        var_dnm_dn14 = assign105150_e157513_d_n14;

        let assign105160_e157528: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        var_guard2386 = assign105160_e157528;

        let assign105170_e157531: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard2387 = assign105170_e157531;

        let (assign105180_e157544,) = {
    if (((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) && (var_guard2386 != 0.0)) && (var_guard2387 != 0.0)) {
        (1.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105180_e157544;

        let assign105190_e157547: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard2388 = assign105190_e157547;

        let (assign105200_e157563,) = {
    if ((((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) && (var_guard2386 != 0.0)) && (var_guard2387 == 0.0)) && (var_guard2388 != 0.0)) {
        (2.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105200_e157563;

        let assign105210_e157566: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        var_guard2389 = assign105210_e157566;

        let (assign105220_e157585,) = {
    if (((((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) && (var_guard2386 != 0.0)) && (var_guard2387 == 0.0)) && (var_guard2388 == 0.0)) && (var_guard2389 != 0.0)) {
        (3.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105220_e157585;

        let assign105230_e157588: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        var_guard2390 = assign105230_e157588;

        let (assign105240_e157610,) = {
    if ((((((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) && (var_guard2386 != 0.0)) && (var_guard2387 == 0.0)) && (var_guard2388 == 0.0)) && (var_guard2389 == 0.0)) && (var_guard2390 != 0.0)) {
        (4.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105240_e157610;

        let (assign105250_e157621,) = {
    if ((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) && (var_guard2386 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign105250_e157621;

        let mut assign105260_loop_guard: usize = 0;
        while {
            let assign105260_cond_e157633: f64 = if (((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) && (var_guard2386 != 0.0)) && (var_m0 < var_mm)) { 1.0 } else { 0.0 };
            assign105260_cond_e157633 != 0.0
        } {
            assign105260_loop_guard += 1;
            assert!(assign105260_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign105260_body0_e157645, assign105260_body0_e157645_d_n0, assign105260_body0_e157645_d_n2, assign105260_body0_e157645_d_n4, assign105260_body0_e157645_d_n5, assign105260_body0_e157645_d_n6, assign105260_body0_e157645_d_n7, assign105260_body0_e157645_d_n8, assign105260_body0_e157645_d_n9, assign105260_body0_e157645_d_n10, assign105260_body0_e157645_d_n11, assign105260_body0_e157645_d_n14,) = {
    if ((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) && (var_guard2386 != 0.0)) {
        let assign105260_body0_e157643: f64 = (var_dnm).sqrt();
        (assign105260_body0_e157643, (var_dnm_dn0 / (2.0 * assign105260_body0_e157643)), (var_dnm_dn2 / (2.0 * assign105260_body0_e157643)), (var_dnm_dn4 / (2.0 * assign105260_body0_e157643)), (var_dnm_dn5 / (2.0 * assign105260_body0_e157643)), (var_dnm_dn6 / (2.0 * assign105260_body0_e157643)), (var_dnm_dn7 / (2.0 * assign105260_body0_e157643)), (var_dnm_dn8 / (2.0 * assign105260_body0_e157643)), (var_dnm_dn9 / (2.0 * assign105260_body0_e157643)), (var_dnm_dn10 / (2.0 * assign105260_body0_e157643)), (var_dnm_dn11 / (2.0 * assign105260_body0_e157643)), (var_dnm_dn14 / (2.0 * assign105260_body0_e157643)),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn11, var_dnm_dn14,)
    }
};
            var_dnm = assign105260_body0_e157645;
            var_dnm_dn0 = assign105260_body0_e157645_d_n0;
            var_dnm_dn2 = assign105260_body0_e157645_d_n2;
            var_dnm_dn4 = assign105260_body0_e157645_d_n4;
            var_dnm_dn5 = assign105260_body0_e157645_d_n5;
            var_dnm_dn6 = assign105260_body0_e157645_d_n6;
            var_dnm_dn7 = assign105260_body0_e157645_d_n7;
            var_dnm_dn8 = assign105260_body0_e157645_d_n8;
            var_dnm_dn9 = assign105260_body0_e157645_d_n9;
            var_dnm_dn10 = assign105260_body0_e157645_d_n10;
            var_dnm_dn11 = assign105260_body0_e157645_d_n11;
            var_dnm_dn14 = assign105260_body0_e157645_d_n14;
            let (assign105260_body1_e157658,) = {
    if ((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) && (var_guard2386 != 0.0)) {
        let assign105260_body1_e157656: f64 = (var_m0 + 1.0);
        (assign105260_body1_e157656,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign105260_body1_e157658;
        }

        let (assign105270_e157681, assign105270_e157681_d_n0, assign105270_e157681_d_n2, assign105270_e157681_d_n4, assign105270_e157681_d_n5, assign105270_e157681_d_n6, assign105270_e157681_d_n7, assign105270_e157681_d_n8, assign105270_e157681_d_n9, assign105270_e157681_d_n10, assign105270_e157681_d_n11, assign105270_e157681_d_n14,) = {
    if ((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) && (var_guard2386 == 0.0)) {
        let (assign105270_e157679, assign105270_e157679_d_n0, assign105270_e157679_d_n2, assign105270_e157679_d_n4, assign105270_e157679_d_n5, assign105270_e157679_d_n6, assign105270_e157679_d_n7, assign105270_e157679_d_n8, assign105270_e157679_d_n9, assign105270_e157679_d_n10, assign105270_e157679_d_n11, assign105270_e157679_d_n14,) = {
            if (var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign105270_e157676: f64 = (2.0 * 2.0);
                let assign105270_e157677: f64 = (1.0 / assign105270_e157676);
                let assign105270_e157678: f64 = (var_dnm).powf(assign105270_e157677);
                (assign105270_e157678, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((var_dnm).powf(assign105270_e157677 - 1.0) * var_dnm_dn0)) } } else { (assign105270_e157678 * (assign105270_e157677 * (var_dnm_dn0 / var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((var_dnm).powf(assign105270_e157677 - 1.0) * var_dnm_dn2)) } } else { (assign105270_e157678 * (assign105270_e157677 * (var_dnm_dn2 / var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((var_dnm).powf(assign105270_e157677 - 1.0) * var_dnm_dn4)) } } else { (assign105270_e157678 * (assign105270_e157677 * (var_dnm_dn4 / var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((var_dnm).powf(assign105270_e157677 - 1.0) * var_dnm_dn5)) } } else { (assign105270_e157678 * (assign105270_e157677 * (var_dnm_dn5 / var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((var_dnm).powf(assign105270_e157677 - 1.0) * var_dnm_dn6)) } } else { (assign105270_e157678 * (assign105270_e157677 * (var_dnm_dn6 / var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((var_dnm).powf(assign105270_e157677 - 1.0) * var_dnm_dn7)) } } else { (assign105270_e157678 * (assign105270_e157677 * (var_dnm_dn7 / var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((var_dnm).powf(assign105270_e157677 - 1.0) * var_dnm_dn8)) } } else { (assign105270_e157678 * (assign105270_e157677 * (var_dnm_dn8 / var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((var_dnm).powf(assign105270_e157677 - 1.0) * var_dnm_dn9)) } } else { (assign105270_e157678 * (assign105270_e157677 * (var_dnm_dn9 / var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((var_dnm).powf(assign105270_e157677 - 1.0) * var_dnm_dn10)) } } else { (assign105270_e157678 * (assign105270_e157677 * (var_dnm_dn10 / var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((var_dnm).powf(assign105270_e157677 - 1.0) * var_dnm_dn11)) } } else { (assign105270_e157678 * (assign105270_e157677 * (var_dnm_dn11 / var_dnm))) }, if 0.0 == 0.0 && ((assign105270_e157677) as f64).is_finite() && ((assign105270_e157677) as f64).fract() == 0.0 { if assign105270_e157677 == 0.0 { 0.0 } else { (assign105270_e157677 * ((var_dnm).powf(assign105270_e157677 - 1.0) * var_dnm_dn14)) } } else { (assign105270_e157678 * (assign105270_e157677 * (var_dnm_dn14 / var_dnm))) },)
            }
        };
        (assign105270_e157679, assign105270_e157679_d_n0, assign105270_e157679_d_n2, assign105270_e157679_d_n4, assign105270_e157679_d_n5, assign105270_e157679_d_n6, assign105270_e157679_d_n7, assign105270_e157679_d_n8, assign105270_e157679_d_n9, assign105270_e157679_d_n10, assign105270_e157679_d_n11, assign105270_e157679_d_n14,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn11, var_dnm_dn14,)
    }
};
        var_dnm = assign105270_e157681;
        var_dnm_dn0 = assign105270_e157681_d_n0;
        var_dnm_dn2 = assign105270_e157681_d_n2;
        var_dnm_dn4 = assign105270_e157681_d_n4;
        var_dnm_dn5 = assign105270_e157681_d_n5;
        var_dnm_dn6 = assign105270_e157681_d_n6;
        var_dnm_dn7 = assign105270_e157681_d_n7;
        var_dnm_dn8 = assign105270_e157681_d_n8;
        var_dnm_dn9 = assign105270_e157681_d_n9;
        var_dnm_dn10 = assign105270_e157681_d_n10;
        var_dnm_dn11 = assign105270_e157681_d_n11;
        var_dnm_dn14 = assign105270_e157681_d_n14;

        let (assign105280_e157692, assign105280_e157692_d_n0, assign105280_e157692_d_n2, assign105280_e157692_d_n4, assign105280_e157692_d_n5, assign105280_e157692_d_n6, assign105280_e157692_d_n7, assign105280_e157692_d_n8, assign105280_e157692_d_n9, assign105280_e157692_d_n10, assign105280_e157692_d_n11, assign105280_e157692_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        let assign105280_e157690: f64 = (1.0 / var_dnm);
        (assign105280_e157690, (-(var_dnm_dn0 / (var_dnm * var_dnm))), (-(var_dnm_dn2 / (var_dnm * var_dnm))), (-(var_dnm_dn4 / (var_dnm * var_dnm))), (-(var_dnm_dn5 / (var_dnm * var_dnm))), (-(var_dnm_dn6 / (var_dnm * var_dnm))), (-(var_dnm_dn7 / (var_dnm * var_dnm))), (-(var_dnm_dn8 / (var_dnm * var_dnm))), (-(var_dnm_dn9 / (var_dnm * var_dnm))), (-(var_dnm_dn10 / (var_dnm * var_dnm))), (-(var_dnm_dn11 / (var_dnm * var_dnm))), (-(var_dnm_dn14 / (var_dnm * var_dnm))),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn11, var_dnm_dn14,)
    }
};
        var_dnm = assign105280_e157692;
        var_dnm_dn0 = assign105280_e157692_d_n0;
        var_dnm_dn2 = assign105280_e157692_d_n2;
        var_dnm_dn4 = assign105280_e157692_d_n4;
        var_dnm_dn5 = assign105280_e157692_d_n5;
        var_dnm_dn6 = assign105280_e157692_d_n6;
        var_dnm_dn7 = assign105280_e157692_d_n7;
        var_dnm_dn8 = assign105280_e157692_d_n8;
        var_dnm_dn9 = assign105280_e157692_d_n9;
        var_dnm_dn10 = assign105280_e157692_d_n10;
        var_dnm_dn11 = assign105280_e157692_d_n11;
        var_dnm_dn14 = assign105280_e157692_d_n14;

        let (assign105290_e157705, assign105290_e157705_d_n0, assign105290_e157705_d_n2, assign105290_e157705_d_n4, assign105290_e157705_d_n5, assign105290_e157705_d_n6, assign105290_e157705_d_n7, assign105290_e157705_d_n8, assign105290_e157705_d_n9, assign105290_e157705_d_n10, assign105290_e157705_d_n11, assign105290_e157705_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        let assign105290_e157701: f64 = (var_tmf1 * 1e-25);
        let assign105290_e157703: f64 = (assign105290_e157701 * var_dnm);
        (assign105290_e157703, (((var_tmf1_dn0 * 1e-25) * var_dnm) + (assign105290_e157701 * var_dnm_dn0)), (((var_tmf1_dn2 * 1e-25) * var_dnm) + (assign105290_e157701 * var_dnm_dn2)), (((var_tmf1_dn4 * 1e-25) * var_dnm) + (assign105290_e157701 * var_dnm_dn4)), (((var_tmf1_dn5 * 1e-25) * var_dnm) + (assign105290_e157701 * var_dnm_dn5)), (((var_tmf1_dn6 * 1e-25) * var_dnm) + (assign105290_e157701 * var_dnm_dn6)), (((var_tmf1_dn7 * 1e-25) * var_dnm) + (assign105290_e157701 * var_dnm_dn7)), (((var_tmf1_dn8 * 1e-25) * var_dnm) + (assign105290_e157701 * var_dnm_dn8)), (((var_tmf1_dn9 * 1e-25) * var_dnm) + (assign105290_e157701 * var_dnm_dn9)), (((var_tmf1_dn10 * 1e-25) * var_dnm) + (assign105290_e157701 * var_dnm_dn10)), (((var_tmf1_dn11 * 1e-25) * var_dnm) + (assign105290_e157701 * var_dnm_dn11)), (((var_tmf1_dn14 * 1e-25) * var_dnm) + (assign105290_e157701 * var_dnm_dn14)),)
    } else {
        (var_tmf0, var_tmf0_dn0, var_tmf0_dn2, var_tmf0_dn4, var_tmf0_dn5, var_tmf0_dn6, var_tmf0_dn7, var_tmf0_dn8, var_tmf0_dn9, var_tmf0_dn10, var_tmf0_dn11, var_tmf0_dn14,)
    }
};
        var_tmf0 = assign105290_e157705;
        var_tmf0_dn0 = assign105290_e157705_d_n0;
        var_tmf0_dn2 = assign105290_e157705_d_n2;
        var_tmf0_dn4 = assign105290_e157705_d_n4;
        var_tmf0_dn5 = assign105290_e157705_d_n5;
        var_tmf0_dn6 = assign105290_e157705_d_n6;
        var_tmf0_dn7 = assign105290_e157705_d_n7;
        var_tmf0_dn8 = assign105290_e157705_d_n8;
        var_tmf0_dn9 = assign105290_e157705_d_n9;
        var_tmf0_dn10 = assign105290_e157705_d_n10;
        var_tmf0_dn11 = assign105290_e157705_d_n11;
        var_tmf0_dn14 = assign105290_e157705_d_n14;

        let (assign105300_e157720, assign105300_e157720_d_n0, assign105300_e157720_d_n2, assign105300_e157720_d_n4, assign105300_e157720_d_n5, assign105300_e157720_d_n6, assign105300_e157720_d_n7, assign105300_e157720_d_n8, assign105300_e157720_d_n9, assign105300_e157720_d_n10, assign105300_e157720_d_n11, assign105300_e157720_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        let assign105300_e157714: f64 = (1e-25 * var_xmp);
        let assign105300_e157716: f64 = (assign105300_e157714 * var_dnm);
        let assign105300_e157718: f64 = (assign105300_e157716 / var_arg);
        (assign105300_e157718, ((((((1e-25 * var_xmp_dn0) * var_dnm) + (assign105300_e157714 * var_dnm_dn0)) * var_arg) - (assign105300_e157716 * var_arg_dn0)) / (var_arg * var_arg)), ((((((1e-25 * var_xmp_dn2) * var_dnm) + (assign105300_e157714 * var_dnm_dn2)) * var_arg) - (assign105300_e157716 * var_arg_dn2)) / (var_arg * var_arg)), ((((((1e-25 * var_xmp_dn4) * var_dnm) + (assign105300_e157714 * var_dnm_dn4)) * var_arg) - (assign105300_e157716 * var_arg_dn4)) / (var_arg * var_arg)), ((((((1e-25 * var_xmp_dn5) * var_dnm) + (assign105300_e157714 * var_dnm_dn5)) * var_arg) - (assign105300_e157716 * var_arg_dn5)) / (var_arg * var_arg)), ((((((1e-25 * var_xmp_dn6) * var_dnm) + (assign105300_e157714 * var_dnm_dn6)) * var_arg) - (assign105300_e157716 * var_arg_dn6)) / (var_arg * var_arg)), ((((((1e-25 * var_xmp_dn7) * var_dnm) + (assign105300_e157714 * var_dnm_dn7)) * var_arg) - (assign105300_e157716 * var_arg_dn7)) / (var_arg * var_arg)), ((((((1e-25 * var_xmp_dn8) * var_dnm) + (assign105300_e157714 * var_dnm_dn8)) * var_arg) - (assign105300_e157716 * var_arg_dn8)) / (var_arg * var_arg)), ((((((1e-25 * var_xmp_dn9) * var_dnm) + (assign105300_e157714 * var_dnm_dn9)) * var_arg) - (assign105300_e157716 * var_arg_dn9)) / (var_arg * var_arg)), ((((((1e-25 * var_xmp_dn10) * var_dnm) + (assign105300_e157714 * var_dnm_dn10)) * var_arg) - (assign105300_e157716 * var_arg_dn10)) / (var_arg * var_arg)), ((((((1e-25 * var_xmp_dn11) * var_dnm) + (assign105300_e157714 * var_dnm_dn11)) * var_arg) - (assign105300_e157716 * var_arg_dn11)) / (var_arg * var_arg)), ((((((1e-25 * var_xmp_dn14) * var_dnm) + (assign105300_e157714 * var_dnm_dn14)) * var_arg) - (assign105300_e157716 * var_arg_dn14)) / (var_arg * var_arg)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign105300_e157720;
        var_t0_dn0 = assign105300_e157720_d_n0;
        var_t0_dn2 = assign105300_e157720_d_n2;
        var_t0_dn4 = assign105300_e157720_d_n4;
        var_t0_dn5 = assign105300_e157720_d_n5;
        var_t0_dn6 = assign105300_e157720_d_n6;
        var_t0_dn7 = assign105300_e157720_d_n7;
        var_t0_dn8 = assign105300_e157720_d_n8;
        var_t0_dn9 = assign105300_e157720_d_n9;
        var_t0_dn10 = assign105300_e157720_d_n10;
        var_t0_dn11 = assign105300_e157720_d_n11;
        var_t0_dn14 = assign105300_e157720_d_n14;

        let (assign105310_e157733, assign105310_e157733_d_n0, assign105310_e157733_d_n2, assign105310_e157733_d_n4, assign105310_e157733_d_n5, assign105310_e157733_d_n6, assign105310_e157733_d_n7, assign105310_e157733_d_n8, assign105310_e157733_d_n9, assign105310_e157733_d_n10, assign105310_e157733_d_n11, assign105310_e157733_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        let assign105310_e157729: f64 = 1e-25;
        let assign105310_e157731: f64 = (assign105310_e157729 - var_tmf0);
        (assign105310_e157731, (-var_tmf0_dn0), (-var_tmf0_dn2), (-var_tmf0_dn4), (-var_tmf0_dn5), (-var_tmf0_dn6), (-var_tmf0_dn7), (-var_tmf0_dn8), (-var_tmf0_dn9), (-var_tmf0_dn10), (-var_tmf0_dn11), (-var_tmf0_dn14),)
    } else {
        (var_gd, var_gd_dn0, var_gd_dn2, var_gd_dn4, var_gd_dn5, var_gd_dn6, var_gd_dn7, var_gd_dn8, var_gd_dn9, var_gd_dn10, var_gd_dn11, var_gd_dn14,)
    }
};
        var_gd = assign105310_e157733;
        var_gd_dn0 = assign105310_e157733_d_n0;
        var_gd_dn2 = assign105310_e157733_d_n2;
        var_gd_dn4 = assign105310_e157733_d_n4;
        var_gd_dn5 = assign105310_e157733_d_n5;
        var_gd_dn6 = assign105310_e157733_d_n6;
        var_gd_dn7 = assign105310_e157733_d_n7;
        var_gd_dn8 = assign105310_e157733_d_n8;
        var_gd_dn9 = assign105310_e157733_d_n9;
        var_gd_dn10 = assign105310_e157733_d_n10;
        var_gd_dn11 = assign105310_e157733_d_n11;
        var_gd_dn14 = assign105310_e157733_d_n14;

        let (assign105320_e157742, assign105320_e157742_d_n0, assign105320_e157742_d_n2, assign105320_e157742_d_n4, assign105320_e157742_d_n5, assign105320_e157742_d_n6, assign105320_e157742_d_n7, assign105320_e157742_d_n8, assign105320_e157742_d_n9, assign105320_e157742_d_n10, assign105320_e157742_d_n11, assign105320_e157742_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 != 0.0)) {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign105320_e157742;
        var_t0_dn0 = assign105320_e157742_d_n0;
        var_t0_dn2 = assign105320_e157742_d_n2;
        var_t0_dn4 = assign105320_e157742_d_n4;
        var_t0_dn5 = assign105320_e157742_d_n5;
        var_t0_dn6 = assign105320_e157742_d_n6;
        var_t0_dn7 = assign105320_e157742_d_n7;
        var_t0_dn8 = assign105320_e157742_d_n8;
        var_t0_dn9 = assign105320_e157742_d_n9;
        var_t0_dn10 = assign105320_e157742_d_n10;
        var_t0_dn11 = assign105320_e157742_d_n11;
        var_t0_dn14 = assign105320_e157742_d_n14;

        let (assign105330_e157752, assign105330_e157752_d_n0, assign105330_e157752_d_n2, assign105330_e157752_d_n4, assign105330_e157752_d_n5, assign105330_e157752_d_n6, assign105330_e157752_d_n7, assign105330_e157752_d_n8, assign105330_e157752_d_n9, assign105330_e157752_d_n10, assign105330_e157752_d_n11, assign105330_e157752_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 == 0.0)) {
        (var_gd, var_gd_dn0, var_gd_dn2, var_gd_dn4, var_gd_dn5, var_gd_dn6, var_gd_dn7, var_gd_dn8, var_gd_dn9, var_gd_dn10, var_gd_dn11, var_gd_dn14,)
    } else {
        (var_gd, var_gd_dn0, var_gd_dn2, var_gd_dn4, var_gd_dn5, var_gd_dn6, var_gd_dn7, var_gd_dn8, var_gd_dn9, var_gd_dn10, var_gd_dn11, var_gd_dn14,)
    }
};
        var_gd = assign105330_e157752;
        var_gd_dn0 = assign105330_e157752_d_n0;
        var_gd_dn2 = assign105330_e157752_d_n2;
        var_gd_dn4 = assign105330_e157752_d_n4;
        var_gd_dn5 = assign105330_e157752_d_n5;
        var_gd_dn6 = assign105330_e157752_d_n6;
        var_gd_dn7 = assign105330_e157752_d_n7;
        var_gd_dn8 = assign105330_e157752_d_n8;
        var_gd_dn9 = assign105330_e157752_d_n9;
        var_gd_dn10 = assign105330_e157752_d_n10;
        var_gd_dn11 = assign105330_e157752_d_n11;
        var_gd_dn14 = assign105330_e157752_d_n14;

        let (assign105340_e157762, assign105340_e157762_d_n0, assign105340_e157762_d_n2, assign105340_e157762_d_n4, assign105340_e157762_d_n5, assign105340_e157762_d_n6, assign105340_e157762_d_n7, assign105340_e157762_d_n8, assign105340_e157762_d_n9, assign105340_e157762_d_n10, assign105340_e157762_d_n11, assign105340_e157762_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2385 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign105340_e157762;
        var_t0_dn0 = assign105340_e157762_d_n0;
        var_t0_dn2 = assign105340_e157762_d_n2;
        var_t0_dn4 = assign105340_e157762_d_n4;
        var_t0_dn5 = assign105340_e157762_d_n5;
        var_t0_dn6 = assign105340_e157762_d_n6;
        var_t0_dn7 = assign105340_e157762_d_n7;
        var_t0_dn8 = assign105340_e157762_d_n8;
        var_t0_dn9 = assign105340_e157762_d_n9;
        var_t0_dn10 = assign105340_e157762_d_n10;
        var_t0_dn11 = assign105340_e157762_d_n11;
        var_t0_dn14 = assign105340_e157762_d_n14;

        let (assign105350_e157771, assign105350_e157771_d_n0, assign105350_e157771_d_n2, assign105350_e157771_d_n4, assign105350_e157771_d_n5, assign105350_e157771_d_n6, assign105350_e157771_d_n7, assign105350_e157771_d_n8, assign105350_e157771_d_n9, assign105350_e157771_d_n10, assign105350_e157771_d_n11, assign105350_e157771_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign105350_e157769: f64 = (1.0 / var_gd);
        (assign105350_e157769, (-(var_gd_dn0 / (var_gd * var_gd))), (-(var_gd_dn2 / (var_gd * var_gd))), (-(var_gd_dn4 / (var_gd * var_gd))), (-(var_gd_dn5 / (var_gd * var_gd))), (-(var_gd_dn6 / (var_gd * var_gd))), (-(var_gd_dn7 / (var_gd * var_gd))), (-(var_gd_dn8 / (var_gd * var_gd))), (-(var_gd_dn9 / (var_gd * var_gd))), (-(var_gd_dn10 / (var_gd * var_gd))), (-(var_gd_dn11 / (var_gd * var_gd))), (-(var_gd_dn14 / (var_gd * var_gd))),)
    } else {
        (var_rdd, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn11, var_rdd_dn14,)
    }
};
        var_rdd = assign105350_e157771;
        var_rdd_dn0 = assign105350_e157771_d_n0;
        var_rdd_dn2 = assign105350_e157771_d_n2;
        var_rdd_dn4 = assign105350_e157771_d_n4;
        var_rdd_dn5 = assign105350_e157771_d_n5;
        var_rdd_dn6 = assign105350_e157771_d_n6;
        var_rdd_dn7 = assign105350_e157771_d_n7;
        var_rdd_dn8 = assign105350_e157771_d_n8;
        var_rdd_dn9 = assign105350_e157771_d_n9;
        var_rdd_dn10 = assign105350_e157771_d_n10;
        var_rdd_dn11 = assign105350_e157771_d_n11;
        var_rdd_dn14 = assign105350_e157771_d_n14;

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
        *var_dnm_slot = var_dnm;
        *var_dnm_dn0_slot = var_dnm_dn0;
        *var_dnm_dn10_slot = var_dnm_dn10;
        *var_dnm_dn11_slot = var_dnm_dn11;
        *var_dnm_dn14_slot = var_dnm_dn14;
        *var_dnm_dn2_slot = var_dnm_dn2;
        *var_dnm_dn4_slot = var_dnm_dn4;
        *var_dnm_dn5_slot = var_dnm_dn5;
        *var_dnm_dn6_slot = var_dnm_dn6;
        *var_dnm_dn7_slot = var_dnm_dn7;
        *var_dnm_dn8_slot = var_dnm_dn8;
        *var_dnm_dn9_slot = var_dnm_dn9;
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
        *var_guard2386_slot = var_guard2386;
        *var_guard2387_slot = var_guard2387;
        *var_guard2388_slot = var_guard2388;
        *var_guard2389_slot = var_guard2389;
        *var_guard2390_slot = var_guard2390;
        *var_m0_slot = var_m0;
        *var_mm_slot = var_mm;
        *var_rdd_slot = var_rdd;
        *var_rdd_dn0_slot = var_rdd_dn0;
        *var_rdd_dn10_slot = var_rdd_dn10;
        *var_rdd_dn11_slot = var_rdd_dn11;
        *var_rdd_dn14_slot = var_rdd_dn14;
        *var_rdd_dn2_slot = var_rdd_dn2;
        *var_rdd_dn4_slot = var_rdd_dn4;
        *var_rdd_dn5_slot = var_rdd_dn5;
        *var_rdd_dn6_slot = var_rdd_dn6;
        *var_rdd_dn7_slot = var_rdd_dn7;
        *var_rdd_dn8_slot = var_rdd_dn8;
        *var_rdd_dn9_slot = var_rdd_dn9;
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
        *var_tmf0_slot = var_tmf0;
        *var_tmf0_dn0_slot = var_tmf0_dn0;
        *var_tmf0_dn10_slot = var_tmf0_dn10;
        *var_tmf0_dn11_slot = var_tmf0_dn11;
        *var_tmf0_dn14_slot = var_tmf0_dn14;
        *var_tmf0_dn2_slot = var_tmf0_dn2;
        *var_tmf0_dn4_slot = var_tmf0_dn4;
        *var_tmf0_dn5_slot = var_tmf0_dn5;
        *var_tmf0_dn6_slot = var_tmf0_dn6;
        *var_tmf0_dn7_slot = var_tmf0_dn7;
        *var_tmf0_dn8_slot = var_tmf0_dn8;
        *var_tmf0_dn9_slot = var_tmf0_dn9;
        *var_xmp_slot = var_xmp;
        *var_xmp_dn0_slot = var_xmp_dn0;
        *var_xmp_dn10_slot = var_xmp_dn10;
        *var_xmp_dn11_slot = var_xmp_dn11;
        *var_xmp_dn14_slot = var_xmp_dn14;
        *var_xmp_dn2_slot = var_xmp_dn2;
        *var_xmp_dn4_slot = var_xmp_dn4;
        *var_xmp_dn5_slot = var_xmp_dn5;
        *var_xmp_dn6_slot = var_xmp_dn6;
        *var_xmp_dn7_slot = var_xmp_dn7;
        *var_xmp_dn8_slot = var_xmp_dn8;
        *var_xmp_dn9_slot = var_xmp_dn9;
        *var_xp_slot = var_xp;
        *var_xp_dn0_slot = var_xp_dn0;
        *var_xp_dn10_slot = var_xp_dn10;
        *var_xp_dn11_slot = var_xp_dn11;
        *var_xp_dn14_slot = var_xp_dn14;
        *var_xp_dn2_slot = var_xp_dn2;
        *var_xp_dn4_slot = var_xp_dn4;
        *var_xp_dn5_slot = var_xp_dn5;
        *var_xp_dn6_slot = var_xp_dn6;
        *var_xp_dn7_slot = var_xp_dn7;
        *var_xp_dn8_slot = var_xp_dn8;
        *var_xp_dn9_slot = var_xp_dn9;
    }

    pub(super) fn stamp_transient_block_387(
        var_guard2340: f64,
        var_guard2360: f64,
        var_weffld_nf: f64,
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
        var_dnm_slot: &mut f64,
        var_dnm_dn0_slot: &mut f64,
        var_dnm_dn10_slot: &mut f64,
        var_dnm_dn11_slot: &mut f64,
        var_dnm_dn14_slot: &mut f64,
        var_dnm_dn2_slot: &mut f64,
        var_dnm_dn4_slot: &mut f64,
        var_dnm_dn5_slot: &mut f64,
        var_dnm_dn6_slot: &mut f64,
        var_dnm_dn7_slot: &mut f64,
        var_dnm_dn8_slot: &mut f64,
        var_dnm_dn9_slot: &mut f64,
        var_guard2391_slot: &mut f64,
        var_guard2392_slot: &mut f64,
        var_guard2393_slot: &mut f64,
        var_guard2394_slot: &mut f64,
        var_guard2395_slot: &mut f64,
        var_guard2396_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_mm_slot: &mut f64,
        var_rdd_slot: &mut f64,
        var_rdd_dn0_slot: &mut f64,
        var_rdd_dn10_slot: &mut f64,
        var_rdd_dn11_slot: &mut f64,
        var_rdd_dn14_slot: &mut f64,
        var_rdd_dn2_slot: &mut f64,
        var_rdd_dn4_slot: &mut f64,
        var_rdd_dn5_slot: &mut f64,
        var_rdd_dn6_slot: &mut f64,
        var_rdd_dn7_slot: &mut f64,
        var_rdd_dn8_slot: &mut f64,
        var_rdd_dn9_slot: &mut f64,
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
        var_tmf0_slot: &mut f64,
        var_tmf0_dn0_slot: &mut f64,
        var_tmf0_dn10_slot: &mut f64,
        var_tmf0_dn11_slot: &mut f64,
        var_tmf0_dn14_slot: &mut f64,
        var_tmf0_dn2_slot: &mut f64,
        var_tmf0_dn4_slot: &mut f64,
        var_tmf0_dn5_slot: &mut f64,
        var_tmf0_dn6_slot: &mut f64,
        var_tmf0_dn7_slot: &mut f64,
        var_tmf0_dn8_slot: &mut f64,
        var_tmf0_dn9_slot: &mut f64,
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
        var_x2_slot: &mut f64,
        var_x2_dn0_slot: &mut f64,
        var_x2_dn10_slot: &mut f64,
        var_x2_dn11_slot: &mut f64,
        var_x2_dn14_slot: &mut f64,
        var_x2_dn2_slot: &mut f64,
        var_x2_dn4_slot: &mut f64,
        var_x2_dn5_slot: &mut f64,
        var_x2_dn6_slot: &mut f64,
        var_x2_dn7_slot: &mut f64,
        var_x2_dn8_slot: &mut f64,
        var_x2_dn9_slot: &mut f64,
        var_xmax2_slot: &mut f64,
        var_xmax2_dn0_slot: &mut f64,
        var_xmax2_dn10_slot: &mut f64,
        var_xmax2_dn11_slot: &mut f64,
        var_xmax2_dn14_slot: &mut f64,
        var_xmax2_dn2_slot: &mut f64,
        var_xmax2_dn4_slot: &mut f64,
        var_xmax2_dn5_slot: &mut f64,
        var_xmax2_dn6_slot: &mut f64,
        var_xmax2_dn7_slot: &mut f64,
        var_xmax2_dn8_slot: &mut f64,
        var_xmax2_dn9_slot: &mut f64,
        var_xmp_slot: &mut f64,
        var_xmp_dn0_slot: &mut f64,
        var_xmp_dn10_slot: &mut f64,
        var_xmp_dn11_slot: &mut f64,
        var_xmp_dn14_slot: &mut f64,
        var_xmp_dn2_slot: &mut f64,
        var_xmp_dn4_slot: &mut f64,
        var_xmp_dn5_slot: &mut f64,
        var_xmp_dn6_slot: &mut f64,
        var_xmp_dn7_slot: &mut f64,
        var_xmp_dn8_slot: &mut f64,
        var_xmp_dn9_slot: &mut f64,
        var_xp_slot: &mut f64,
        var_xp_dn0_slot: &mut f64,
        var_xp_dn10_slot: &mut f64,
        var_xp_dn11_slot: &mut f64,
        var_xp_dn14_slot: &mut f64,
        var_xp_dn2_slot: &mut f64,
        var_xp_dn4_slot: &mut f64,
        var_xp_dn5_slot: &mut f64,
        var_xp_dn6_slot: &mut f64,
        var_xp_dn7_slot: &mut f64,
        var_xp_dn8_slot: &mut f64,
        var_xp_dn9_slot: &mut f64,
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
        let mut var_dnm: f64 = *var_dnm_slot;
        let mut var_dnm_dn0: f64 = *var_dnm_dn0_slot;
        let mut var_dnm_dn10: f64 = *var_dnm_dn10_slot;
        let mut var_dnm_dn11: f64 = *var_dnm_dn11_slot;
        let mut var_dnm_dn14: f64 = *var_dnm_dn14_slot;
        let mut var_dnm_dn2: f64 = *var_dnm_dn2_slot;
        let mut var_dnm_dn4: f64 = *var_dnm_dn4_slot;
        let mut var_dnm_dn5: f64 = *var_dnm_dn5_slot;
        let mut var_dnm_dn6: f64 = *var_dnm_dn6_slot;
        let mut var_dnm_dn7: f64 = *var_dnm_dn7_slot;
        let mut var_dnm_dn8: f64 = *var_dnm_dn8_slot;
        let mut var_dnm_dn9: f64 = *var_dnm_dn9_slot;
        let mut var_guard2391: f64 = *var_guard2391_slot;
        let mut var_guard2392: f64 = *var_guard2392_slot;
        let mut var_guard2393: f64 = *var_guard2393_slot;
        let mut var_guard2394: f64 = *var_guard2394_slot;
        let mut var_guard2395: f64 = *var_guard2395_slot;
        let mut var_guard2396: f64 = *var_guard2396_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_mm: f64 = *var_mm_slot;
        let mut var_rdd: f64 = *var_rdd_slot;
        let mut var_rdd_dn0: f64 = *var_rdd_dn0_slot;
        let mut var_rdd_dn10: f64 = *var_rdd_dn10_slot;
        let mut var_rdd_dn11: f64 = *var_rdd_dn11_slot;
        let mut var_rdd_dn14: f64 = *var_rdd_dn14_slot;
        let mut var_rdd_dn2: f64 = *var_rdd_dn2_slot;
        let mut var_rdd_dn4: f64 = *var_rdd_dn4_slot;
        let mut var_rdd_dn5: f64 = *var_rdd_dn5_slot;
        let mut var_rdd_dn6: f64 = *var_rdd_dn6_slot;
        let mut var_rdd_dn7: f64 = *var_rdd_dn7_slot;
        let mut var_rdd_dn8: f64 = *var_rdd_dn8_slot;
        let mut var_rdd_dn9: f64 = *var_rdd_dn9_slot;
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
        let mut var_tmf0: f64 = *var_tmf0_slot;
        let mut var_tmf0_dn0: f64 = *var_tmf0_dn0_slot;
        let mut var_tmf0_dn10: f64 = *var_tmf0_dn10_slot;
        let mut var_tmf0_dn11: f64 = *var_tmf0_dn11_slot;
        let mut var_tmf0_dn14: f64 = *var_tmf0_dn14_slot;
        let mut var_tmf0_dn2: f64 = *var_tmf0_dn2_slot;
        let mut var_tmf0_dn4: f64 = *var_tmf0_dn4_slot;
        let mut var_tmf0_dn5: f64 = *var_tmf0_dn5_slot;
        let mut var_tmf0_dn6: f64 = *var_tmf0_dn6_slot;
        let mut var_tmf0_dn7: f64 = *var_tmf0_dn7_slot;
        let mut var_tmf0_dn8: f64 = *var_tmf0_dn8_slot;
        let mut var_tmf0_dn9: f64 = *var_tmf0_dn9_slot;
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
        let mut var_x2: f64 = *var_x2_slot;
        let mut var_x2_dn0: f64 = *var_x2_dn0_slot;
        let mut var_x2_dn10: f64 = *var_x2_dn10_slot;
        let mut var_x2_dn11: f64 = *var_x2_dn11_slot;
        let mut var_x2_dn14: f64 = *var_x2_dn14_slot;
        let mut var_x2_dn2: f64 = *var_x2_dn2_slot;
        let mut var_x2_dn4: f64 = *var_x2_dn4_slot;
        let mut var_x2_dn5: f64 = *var_x2_dn5_slot;
        let mut var_x2_dn6: f64 = *var_x2_dn6_slot;
        let mut var_x2_dn7: f64 = *var_x2_dn7_slot;
        let mut var_x2_dn8: f64 = *var_x2_dn8_slot;
        let mut var_x2_dn9: f64 = *var_x2_dn9_slot;
        let mut var_xmax2: f64 = *var_xmax2_slot;
        let mut var_xmax2_dn0: f64 = *var_xmax2_dn0_slot;
        let mut var_xmax2_dn10: f64 = *var_xmax2_dn10_slot;
        let mut var_xmax2_dn11: f64 = *var_xmax2_dn11_slot;
        let mut var_xmax2_dn14: f64 = *var_xmax2_dn14_slot;
        let mut var_xmax2_dn2: f64 = *var_xmax2_dn2_slot;
        let mut var_xmax2_dn4: f64 = *var_xmax2_dn4_slot;
        let mut var_xmax2_dn5: f64 = *var_xmax2_dn5_slot;
        let mut var_xmax2_dn6: f64 = *var_xmax2_dn6_slot;
        let mut var_xmax2_dn7: f64 = *var_xmax2_dn7_slot;
        let mut var_xmax2_dn8: f64 = *var_xmax2_dn8_slot;
        let mut var_xmax2_dn9: f64 = *var_xmax2_dn9_slot;
        let mut var_xmp: f64 = *var_xmp_slot;
        let mut var_xmp_dn0: f64 = *var_xmp_dn0_slot;
        let mut var_xmp_dn10: f64 = *var_xmp_dn10_slot;
        let mut var_xmp_dn11: f64 = *var_xmp_dn11_slot;
        let mut var_xmp_dn14: f64 = *var_xmp_dn14_slot;
        let mut var_xmp_dn2: f64 = *var_xmp_dn2_slot;
        let mut var_xmp_dn4: f64 = *var_xmp_dn4_slot;
        let mut var_xmp_dn5: f64 = *var_xmp_dn5_slot;
        let mut var_xmp_dn6: f64 = *var_xmp_dn6_slot;
        let mut var_xmp_dn7: f64 = *var_xmp_dn7_slot;
        let mut var_xmp_dn8: f64 = *var_xmp_dn8_slot;
        let mut var_xmp_dn9: f64 = *var_xmp_dn9_slot;
        let mut var_xp: f64 = *var_xp_slot;
        let mut var_xp_dn0: f64 = *var_xp_dn0_slot;
        let mut var_xp_dn10: f64 = *var_xp_dn10_slot;
        let mut var_xp_dn11: f64 = *var_xp_dn11_slot;
        let mut var_xp_dn14: f64 = *var_xp_dn14_slot;
        let mut var_xp_dn2: f64 = *var_xp_dn2_slot;
        let mut var_xp_dn4: f64 = *var_xp_dn4_slot;
        let mut var_xp_dn5: f64 = *var_xp_dn5_slot;
        let mut var_xp_dn6: f64 = *var_xp_dn6_slot;
        let mut var_xp_dn7: f64 = *var_xp_dn7_slot;
        let mut var_xp_dn8: f64 = *var_xp_dn8_slot;
        let mut var_xp_dn9: f64 = *var_xp_dn9_slot;

        let (assign105360_e157780, assign105360_e157780_d_n0, assign105360_e157780_d_n2, assign105360_e157780_d_n4, assign105360_e157780_d_n5, assign105360_e157780_d_n6, assign105360_e157780_d_n7, assign105360_e157780_d_n8, assign105360_e157780_d_n9, assign105360_e157780_d_n10, assign105360_e157780_d_n11, assign105360_e157780_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign105360_e157778: f64 = (var_rdd / var_weffld_nf);
        (assign105360_e157778, (var_rdd_dn0 / var_weffld_nf), (var_rdd_dn2 / var_weffld_nf), (var_rdd_dn4 / var_weffld_nf), (var_rdd_dn5 / var_weffld_nf), (var_rdd_dn6 / var_weffld_nf), (var_rdd_dn7 / var_weffld_nf), (var_rdd_dn8 / var_weffld_nf), (var_rdd_dn9 / var_weffld_nf), (var_rdd_dn10 / var_weffld_nf), (var_rdd_dn11 / var_weffld_nf), (var_rdd_dn14 / var_weffld_nf),)
    } else {
        (var_rdd, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn11, var_rdd_dn14,)
    }
};
        var_rdd = assign105360_e157780;
        var_rdd_dn0 = assign105360_e157780_d_n0;
        var_rdd_dn2 = assign105360_e157780_d_n2;
        var_rdd_dn4 = assign105360_e157780_d_n4;
        var_rdd_dn5 = assign105360_e157780_d_n5;
        var_rdd_dn6 = assign105360_e157780_d_n6;
        var_rdd_dn7 = assign105360_e157780_d_n7;
        var_rdd_dn8 = assign105360_e157780_d_n8;
        var_rdd_dn9 = assign105360_e157780_d_n9;
        var_rdd_dn10 = assign105360_e157780_d_n10;
        var_rdd_dn11 = assign105360_e157780_d_n11;
        var_rdd_dn14 = assign105360_e157780_d_n14;

        let assign105370_e157784: f64 = (1000000.0 - 1000.0);
        let assign105370_e157789: f64 = if ((var_rdd > assign105370_e157784) && (1000.0 >= 0.0)) { 1.0 } else { 0.0 };
        var_guard2391 = assign105370_e157789;

        let (assign105380_e157802, assign105380_e157802_d_n0, assign105380_e157802_d_n2, assign105380_e157802_d_n4, assign105380_e157802_d_n5, assign105380_e157802_d_n6, assign105380_e157802_d_n7, assign105380_e157802_d_n8, assign105380_e157802_d_n9, assign105380_e157802_d_n10, assign105380_e157802_d_n11, assign105380_e157802_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        let assign105380_e157798: f64 = (var_rdd - 1000000.0);
        let assign105380_e157800: f64 = (assign105380_e157798 + 1000.0);
        (assign105380_e157800, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn11, var_rdd_dn14,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign105380_e157802;
        var_tmf1_dn0 = assign105380_e157802_d_n0;
        var_tmf1_dn2 = assign105380_e157802_d_n2;
        var_tmf1_dn4 = assign105380_e157802_d_n4;
        var_tmf1_dn5 = assign105380_e157802_d_n5;
        var_tmf1_dn6 = assign105380_e157802_d_n6;
        var_tmf1_dn7 = assign105380_e157802_d_n7;
        var_tmf1_dn8 = assign105380_e157802_d_n8;
        var_tmf1_dn9 = assign105380_e157802_d_n9;
        var_tmf1_dn10 = assign105380_e157802_d_n10;
        var_tmf1_dn11 = assign105380_e157802_d_n11;
        var_tmf1_dn14 = assign105380_e157802_d_n14;

        let (assign105390_e157813, assign105390_e157813_d_n0, assign105390_e157813_d_n2, assign105390_e157813_d_n4, assign105390_e157813_d_n5, assign105390_e157813_d_n6, assign105390_e157813_d_n7, assign105390_e157813_d_n8, assign105390_e157813_d_n9, assign105390_e157813_d_n10, assign105390_e157813_d_n11, assign105390_e157813_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        let assign105390_e157811: f64 = (var_tmf1 * var_tmf1);
        (assign105390_e157811, ((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)), ((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)), ((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)), ((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)), ((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)), ((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)), ((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)), ((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)), ((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)), ((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)), ((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)),)
    } else {
        (var_x2, var_x2_dn0, var_x2_dn2, var_x2_dn4, var_x2_dn5, var_x2_dn6, var_x2_dn7, var_x2_dn8, var_x2_dn9, var_x2_dn10, var_x2_dn11, var_x2_dn14,)
    }
};
        var_x2 = assign105390_e157813;
        var_x2_dn0 = assign105390_e157813_d_n0;
        var_x2_dn2 = assign105390_e157813_d_n2;
        var_x2_dn4 = assign105390_e157813_d_n4;
        var_x2_dn5 = assign105390_e157813_d_n5;
        var_x2_dn6 = assign105390_e157813_d_n6;
        var_x2_dn7 = assign105390_e157813_d_n7;
        var_x2_dn8 = assign105390_e157813_d_n8;
        var_x2_dn9 = assign105390_e157813_d_n9;
        var_x2_dn10 = assign105390_e157813_d_n10;
        var_x2_dn11 = assign105390_e157813_d_n11;
        var_x2_dn14 = assign105390_e157813_d_n14;

        let (assign105400_e157824, assign105400_e157824_d_n0, assign105400_e157824_d_n2, assign105400_e157824_d_n4, assign105400_e157824_d_n5, assign105400_e157824_d_n6, assign105400_e157824_d_n7, assign105400_e157824_d_n8, assign105400_e157824_d_n9, assign105400_e157824_d_n10, assign105400_e157824_d_n11, assign105400_e157824_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        let assign105400_e157822: f64 = (1000.0 * 1000.0);
        (assign105400_e157822, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmax2, var_xmax2_dn0, var_xmax2_dn2, var_xmax2_dn4, var_xmax2_dn5, var_xmax2_dn6, var_xmax2_dn7, var_xmax2_dn8, var_xmax2_dn9, var_xmax2_dn10, var_xmax2_dn11, var_xmax2_dn14,)
    }
};
        var_xmax2 = assign105400_e157824;
        var_xmax2_dn0 = assign105400_e157824_d_n0;
        var_xmax2_dn2 = assign105400_e157824_d_n2;
        var_xmax2_dn4 = assign105400_e157824_d_n4;
        var_xmax2_dn5 = assign105400_e157824_d_n5;
        var_xmax2_dn6 = assign105400_e157824_d_n6;
        var_xmax2_dn7 = assign105400_e157824_d_n7;
        var_xmax2_dn8 = assign105400_e157824_d_n8;
        var_xmax2_dn9 = assign105400_e157824_d_n9;
        var_xmax2_dn10 = assign105400_e157824_d_n10;
        var_xmax2_dn11 = assign105400_e157824_d_n11;
        var_xmax2_dn14 = assign105400_e157824_d_n14;

        let (assign105410_e157833, assign105410_e157833_d_n0, assign105410_e157833_d_n2, assign105410_e157833_d_n4, assign105410_e157833_d_n5, assign105410_e157833_d_n6, assign105410_e157833_d_n7, assign105410_e157833_d_n8, assign105410_e157833_d_n9, assign105410_e157833_d_n10, assign105410_e157833_d_n11, assign105410_e157833_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn11, var_xp_dn14,)
    }
};
        var_xp = assign105410_e157833;
        var_xp_dn0 = assign105410_e157833_d_n0;
        var_xp_dn2 = assign105410_e157833_d_n2;
        var_xp_dn4 = assign105410_e157833_d_n4;
        var_xp_dn5 = assign105410_e157833_d_n5;
        var_xp_dn6 = assign105410_e157833_d_n6;
        var_xp_dn7 = assign105410_e157833_d_n7;
        var_xp_dn8 = assign105410_e157833_d_n8;
        var_xp_dn9 = assign105410_e157833_d_n9;
        var_xp_dn10 = assign105410_e157833_d_n10;
        var_xp_dn11 = assign105410_e157833_d_n11;
        var_xp_dn14 = assign105410_e157833_d_n14;

        let (assign105420_e157842, assign105420_e157842_d_n0, assign105420_e157842_d_n2, assign105420_e157842_d_n4, assign105420_e157842_d_n5, assign105420_e157842_d_n6, assign105420_e157842_d_n7, assign105420_e157842_d_n8, assign105420_e157842_d_n9, assign105420_e157842_d_n10, assign105420_e157842_d_n11, assign105420_e157842_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn11, var_xmp_dn14,)
    }
};
        var_xmp = assign105420_e157842;
        var_xmp_dn0 = assign105420_e157842_d_n0;
        var_xmp_dn2 = assign105420_e157842_d_n2;
        var_xmp_dn4 = assign105420_e157842_d_n4;
        var_xmp_dn5 = assign105420_e157842_d_n5;
        var_xmp_dn6 = assign105420_e157842_d_n6;
        var_xmp_dn7 = assign105420_e157842_d_n7;
        var_xmp_dn8 = assign105420_e157842_d_n8;
        var_xmp_dn9 = assign105420_e157842_d_n9;
        var_xmp_dn10 = assign105420_e157842_d_n10;
        var_xmp_dn11 = assign105420_e157842_d_n11;
        var_xmp_dn14 = assign105420_e157842_d_n14;

        let (assign105430_e157851,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign105430_e157851;

        let (assign105440_e157860,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        (0.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105440_e157860;

        let (assign105450_e157869, assign105450_e157869_d_n0, assign105450_e157869_d_n2, assign105450_e157869_d_n4, assign105450_e157869_d_n5, assign105450_e157869_d_n6, assign105450_e157869_d_n7, assign105450_e157869_d_n8, assign105450_e157869_d_n9, assign105450_e157869_d_n10, assign105450_e157869_d_n11, assign105450_e157869_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn11, var_arg_dn14,)
    }
};
        var_arg = assign105450_e157869;
        var_arg_dn0 = assign105450_e157869_d_n0;
        var_arg_dn2 = assign105450_e157869_d_n2;
        var_arg_dn4 = assign105450_e157869_d_n4;
        var_arg_dn5 = assign105450_e157869_d_n5;
        var_arg_dn6 = assign105450_e157869_d_n6;
        var_arg_dn7 = assign105450_e157869_d_n7;
        var_arg_dn8 = assign105450_e157869_d_n8;
        var_arg_dn9 = assign105450_e157869_d_n9;
        var_arg_dn10 = assign105450_e157869_d_n10;
        var_arg_dn11 = assign105450_e157869_d_n11;
        var_arg_dn14 = assign105450_e157869_d_n14;

        let (assign105460_e157878, assign105460_e157878_d_n0, assign105460_e157878_d_n2, assign105460_e157878_d_n4, assign105460_e157878_d_n5, assign105460_e157878_d_n6, assign105460_e157878_d_n7, assign105460_e157878_d_n8, assign105460_e157878_d_n9, assign105460_e157878_d_n10, assign105460_e157878_d_n11, assign105460_e157878_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn11, var_dnm_dn14,)
    }
};
        var_dnm = assign105460_e157878;
        var_dnm_dn0 = assign105460_e157878_d_n0;
        var_dnm_dn2 = assign105460_e157878_d_n2;
        var_dnm_dn4 = assign105460_e157878_d_n4;
        var_dnm_dn5 = assign105460_e157878_d_n5;
        var_dnm_dn6 = assign105460_e157878_d_n6;
        var_dnm_dn7 = assign105460_e157878_d_n7;
        var_dnm_dn8 = assign105460_e157878_d_n8;
        var_dnm_dn9 = assign105460_e157878_d_n9;
        var_dnm_dn10 = assign105460_e157878_d_n10;
        var_dnm_dn11 = assign105460_e157878_d_n11;
        var_dnm_dn14 = assign105460_e157878_d_n14;

        let (assign105470_e157889, assign105470_e157889_d_n0, assign105470_e157889_d_n2, assign105470_e157889_d_n4, assign105470_e157889_d_n5, assign105470_e157889_d_n6, assign105470_e157889_d_n7, assign105470_e157889_d_n8, assign105470_e157889_d_n9, assign105470_e157889_d_n10, assign105470_e157889_d_n11, assign105470_e157889_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        let assign105470_e157887: f64 = (var_xp * var_x2);
        (assign105470_e157887, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn14 * var_x2) + (var_xp * var_x2_dn14)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn11, var_xp_dn14,)
    }
};
        var_xp = assign105470_e157889;
        var_xp_dn0 = assign105470_e157889_d_n0;
        var_xp_dn2 = assign105470_e157889_d_n2;
        var_xp_dn4 = assign105470_e157889_d_n4;
        var_xp_dn5 = assign105470_e157889_d_n5;
        var_xp_dn6 = assign105470_e157889_d_n6;
        var_xp_dn7 = assign105470_e157889_d_n7;
        var_xp_dn8 = assign105470_e157889_d_n8;
        var_xp_dn9 = assign105470_e157889_d_n9;
        var_xp_dn10 = assign105470_e157889_d_n10;
        var_xp_dn11 = assign105470_e157889_d_n11;
        var_xp_dn14 = assign105470_e157889_d_n14;

        let (assign105480_e157900, assign105480_e157900_d_n0, assign105480_e157900_d_n2, assign105480_e157900_d_n4, assign105480_e157900_d_n5, assign105480_e157900_d_n6, assign105480_e157900_d_n7, assign105480_e157900_d_n8, assign105480_e157900_d_n9, assign105480_e157900_d_n10, assign105480_e157900_d_n11, assign105480_e157900_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        let assign105480_e157898: f64 = (var_xmp * var_xmax2);
        (assign105480_e157898, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn14 * var_xmax2) + (var_xmp * var_xmax2_dn14)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn11, var_xmp_dn14,)
    }
};
        var_xmp = assign105480_e157900;
        var_xmp_dn0 = assign105480_e157900_d_n0;
        var_xmp_dn2 = assign105480_e157900_d_n2;
        var_xmp_dn4 = assign105480_e157900_d_n4;
        var_xmp_dn5 = assign105480_e157900_d_n5;
        var_xmp_dn6 = assign105480_e157900_d_n6;
        var_xmp_dn7 = assign105480_e157900_d_n7;
        var_xmp_dn8 = assign105480_e157900_d_n8;
        var_xmp_dn9 = assign105480_e157900_d_n9;
        var_xmp_dn10 = assign105480_e157900_d_n10;
        var_xmp_dn11 = assign105480_e157900_d_n11;
        var_xmp_dn14 = assign105480_e157900_d_n14;

        let (assign105490_e157911, assign105490_e157911_d_n0, assign105490_e157911_d_n2, assign105490_e157911_d_n4, assign105490_e157911_d_n5, assign105490_e157911_d_n6, assign105490_e157911_d_n7, assign105490_e157911_d_n8, assign105490_e157911_d_n9, assign105490_e157911_d_n10, assign105490_e157911_d_n11, assign105490_e157911_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        let assign105490_e157909: f64 = (var_xp * var_x2);
        (assign105490_e157909, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn14 * var_x2) + (var_xp * var_x2_dn14)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn11, var_xp_dn14,)
    }
};
        var_xp = assign105490_e157911;
        var_xp_dn0 = assign105490_e157911_d_n0;
        var_xp_dn2 = assign105490_e157911_d_n2;
        var_xp_dn4 = assign105490_e157911_d_n4;
        var_xp_dn5 = assign105490_e157911_d_n5;
        var_xp_dn6 = assign105490_e157911_d_n6;
        var_xp_dn7 = assign105490_e157911_d_n7;
        var_xp_dn8 = assign105490_e157911_d_n8;
        var_xp_dn9 = assign105490_e157911_d_n9;
        var_xp_dn10 = assign105490_e157911_d_n10;
        var_xp_dn11 = assign105490_e157911_d_n11;
        var_xp_dn14 = assign105490_e157911_d_n14;

        let (assign105500_e157922, assign105500_e157922_d_n0, assign105500_e157922_d_n2, assign105500_e157922_d_n4, assign105500_e157922_d_n5, assign105500_e157922_d_n6, assign105500_e157922_d_n7, assign105500_e157922_d_n8, assign105500_e157922_d_n9, assign105500_e157922_d_n10, assign105500_e157922_d_n11, assign105500_e157922_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        let assign105500_e157920: f64 = (var_xmp * var_xmax2);
        (assign105500_e157920, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn14 * var_xmax2) + (var_xmp * var_xmax2_dn14)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn11, var_xmp_dn14,)
    }
};
        var_xmp = assign105500_e157922;
        var_xmp_dn0 = assign105500_e157922_d_n0;
        var_xmp_dn2 = assign105500_e157922_d_n2;
        var_xmp_dn4 = assign105500_e157922_d_n4;
        var_xmp_dn5 = assign105500_e157922_d_n5;
        var_xmp_dn6 = assign105500_e157922_d_n6;
        var_xmp_dn7 = assign105500_e157922_d_n7;
        var_xmp_dn8 = assign105500_e157922_d_n8;
        var_xmp_dn9 = assign105500_e157922_d_n9;
        var_xmp_dn10 = assign105500_e157922_d_n10;
        var_xmp_dn11 = assign105500_e157922_d_n11;
        var_xmp_dn14 = assign105500_e157922_d_n14;

        let (assign105510_e157933, assign105510_e157933_d_n0, assign105510_e157933_d_n2, assign105510_e157933_d_n4, assign105510_e157933_d_n5, assign105510_e157933_d_n6, assign105510_e157933_d_n7, assign105510_e157933_d_n8, assign105510_e157933_d_n9, assign105510_e157933_d_n10, assign105510_e157933_d_n11, assign105510_e157933_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        let assign105510_e157931: f64 = (var_xp + var_xmp);
        (assign105510_e157931, (var_xp_dn0 + var_xmp_dn0), (var_xp_dn2 + var_xmp_dn2), (var_xp_dn4 + var_xmp_dn4), (var_xp_dn5 + var_xmp_dn5), (var_xp_dn6 + var_xmp_dn6), (var_xp_dn7 + var_xmp_dn7), (var_xp_dn8 + var_xmp_dn8), (var_xp_dn9 + var_xmp_dn9), (var_xp_dn10 + var_xmp_dn10), (var_xp_dn11 + var_xmp_dn11), (var_xp_dn14 + var_xmp_dn14),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn11, var_arg_dn14,)
    }
};
        var_arg = assign105510_e157933;
        var_arg_dn0 = assign105510_e157933_d_n0;
        var_arg_dn2 = assign105510_e157933_d_n2;
        var_arg_dn4 = assign105510_e157933_d_n4;
        var_arg_dn5 = assign105510_e157933_d_n5;
        var_arg_dn6 = assign105510_e157933_d_n6;
        var_arg_dn7 = assign105510_e157933_d_n7;
        var_arg_dn8 = assign105510_e157933_d_n8;
        var_arg_dn9 = assign105510_e157933_d_n9;
        var_arg_dn10 = assign105510_e157933_d_n10;
        var_arg_dn11 = assign105510_e157933_d_n11;
        var_arg_dn14 = assign105510_e157933_d_n14;

        let (assign105520_e157942, assign105520_e157942_d_n0, assign105520_e157942_d_n2, assign105520_e157942_d_n4, assign105520_e157942_d_n5, assign105520_e157942_d_n6, assign105520_e157942_d_n7, assign105520_e157942_d_n8, assign105520_e157942_d_n9, assign105520_e157942_d_n10, assign105520_e157942_d_n11, assign105520_e157942_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn11, var_arg_dn14,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn11, var_dnm_dn14,)
    }
};
        var_dnm = assign105520_e157942;
        var_dnm_dn0 = assign105520_e157942_d_n0;
        var_dnm_dn2 = assign105520_e157942_d_n2;
        var_dnm_dn4 = assign105520_e157942_d_n4;
        var_dnm_dn5 = assign105520_e157942_d_n5;
        var_dnm_dn6 = assign105520_e157942_d_n6;
        var_dnm_dn7 = assign105520_e157942_d_n7;
        var_dnm_dn8 = assign105520_e157942_d_n8;
        var_dnm_dn9 = assign105520_e157942_d_n9;
        var_dnm_dn10 = assign105520_e157942_d_n10;
        var_dnm_dn11 = assign105520_e157942_d_n11;
        var_dnm_dn14 = assign105520_e157942_d_n14;

        let assign105530_e157957: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        var_guard2392 = assign105530_e157957;

        let assign105540_e157960: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard2393 = assign105540_e157960;

        let (assign105550_e157973,) = {
    if (((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) && (var_guard2392 != 0.0)) && (var_guard2393 != 0.0)) {
        (1.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105550_e157973;

        let assign105560_e157976: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard2394 = assign105560_e157976;

        let (assign105570_e157992,) = {
    if ((((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) && (var_guard2392 != 0.0)) && (var_guard2393 == 0.0)) && (var_guard2394 != 0.0)) {
        (2.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105570_e157992;

        let assign105580_e157995: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        var_guard2395 = assign105580_e157995;

        let (assign105590_e158014,) = {
    if (((((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) && (var_guard2392 != 0.0)) && (var_guard2393 == 0.0)) && (var_guard2394 == 0.0)) && (var_guard2395 != 0.0)) {
        (3.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105590_e158014;

        let assign105600_e158017: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        var_guard2396 = assign105600_e158017;

        let (assign105610_e158039,) = {
    if ((((((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) && (var_guard2392 != 0.0)) && (var_guard2393 == 0.0)) && (var_guard2394 == 0.0)) && (var_guard2395 == 0.0)) && (var_guard2396 != 0.0)) {
        (4.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105610_e158039;

        let (assign105620_e158050,) = {
    if ((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) && (var_guard2392 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign105620_e158050;

        let mut assign105630_loop_guard: usize = 0;
        while {
            let assign105630_cond_e158062: f64 = if (((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) && (var_guard2392 != 0.0)) && (var_m0 < var_mm)) { 1.0 } else { 0.0 };
            assign105630_cond_e158062 != 0.0
        } {
            assign105630_loop_guard += 1;
            assert!(assign105630_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign105630_body0_e158074, assign105630_body0_e158074_d_n0, assign105630_body0_e158074_d_n2, assign105630_body0_e158074_d_n4, assign105630_body0_e158074_d_n5, assign105630_body0_e158074_d_n6, assign105630_body0_e158074_d_n7, assign105630_body0_e158074_d_n8, assign105630_body0_e158074_d_n9, assign105630_body0_e158074_d_n10, assign105630_body0_e158074_d_n11, assign105630_body0_e158074_d_n14,) = {
    if ((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) && (var_guard2392 != 0.0)) {
        let assign105630_body0_e158072: f64 = (var_dnm).sqrt();
        (assign105630_body0_e158072, (var_dnm_dn0 / (2.0 * assign105630_body0_e158072)), (var_dnm_dn2 / (2.0 * assign105630_body0_e158072)), (var_dnm_dn4 / (2.0 * assign105630_body0_e158072)), (var_dnm_dn5 / (2.0 * assign105630_body0_e158072)), (var_dnm_dn6 / (2.0 * assign105630_body0_e158072)), (var_dnm_dn7 / (2.0 * assign105630_body0_e158072)), (var_dnm_dn8 / (2.0 * assign105630_body0_e158072)), (var_dnm_dn9 / (2.0 * assign105630_body0_e158072)), (var_dnm_dn10 / (2.0 * assign105630_body0_e158072)), (var_dnm_dn11 / (2.0 * assign105630_body0_e158072)), (var_dnm_dn14 / (2.0 * assign105630_body0_e158072)),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn11, var_dnm_dn14,)
    }
};
            var_dnm = assign105630_body0_e158074;
            var_dnm_dn0 = assign105630_body0_e158074_d_n0;
            var_dnm_dn2 = assign105630_body0_e158074_d_n2;
            var_dnm_dn4 = assign105630_body0_e158074_d_n4;
            var_dnm_dn5 = assign105630_body0_e158074_d_n5;
            var_dnm_dn6 = assign105630_body0_e158074_d_n6;
            var_dnm_dn7 = assign105630_body0_e158074_d_n7;
            var_dnm_dn8 = assign105630_body0_e158074_d_n8;
            var_dnm_dn9 = assign105630_body0_e158074_d_n9;
            var_dnm_dn10 = assign105630_body0_e158074_d_n10;
            var_dnm_dn11 = assign105630_body0_e158074_d_n11;
            var_dnm_dn14 = assign105630_body0_e158074_d_n14;
            let (assign105630_body1_e158087,) = {
    if ((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) && (var_guard2392 != 0.0)) {
        let assign105630_body1_e158085: f64 = (var_m0 + 1.0);
        (assign105630_body1_e158085,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign105630_body1_e158087;
        }

        let (assign105640_e158110, assign105640_e158110_d_n0, assign105640_e158110_d_n2, assign105640_e158110_d_n4, assign105640_e158110_d_n5, assign105640_e158110_d_n6, assign105640_e158110_d_n7, assign105640_e158110_d_n8, assign105640_e158110_d_n9, assign105640_e158110_d_n10, assign105640_e158110_d_n11, assign105640_e158110_d_n14,) = {
    if ((((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) && (var_guard2392 == 0.0)) {
        let (assign105640_e158108, assign105640_e158108_d_n0, assign105640_e158108_d_n2, assign105640_e158108_d_n4, assign105640_e158108_d_n5, assign105640_e158108_d_n6, assign105640_e158108_d_n7, assign105640_e158108_d_n8, assign105640_e158108_d_n9, assign105640_e158108_d_n10, assign105640_e158108_d_n11, assign105640_e158108_d_n14,) = {
            if (var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign105640_e158105: f64 = (2.0 * 2.0);
                let assign105640_e158106: f64 = (1.0 / assign105640_e158105);
                let assign105640_e158107: f64 = (var_dnm).powf(assign105640_e158106);
                (assign105640_e158107, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((var_dnm).powf(assign105640_e158106 - 1.0) * var_dnm_dn0)) } } else { (assign105640_e158107 * (assign105640_e158106 * (var_dnm_dn0 / var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((var_dnm).powf(assign105640_e158106 - 1.0) * var_dnm_dn2)) } } else { (assign105640_e158107 * (assign105640_e158106 * (var_dnm_dn2 / var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((var_dnm).powf(assign105640_e158106 - 1.0) * var_dnm_dn4)) } } else { (assign105640_e158107 * (assign105640_e158106 * (var_dnm_dn4 / var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((var_dnm).powf(assign105640_e158106 - 1.0) * var_dnm_dn5)) } } else { (assign105640_e158107 * (assign105640_e158106 * (var_dnm_dn5 / var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((var_dnm).powf(assign105640_e158106 - 1.0) * var_dnm_dn6)) } } else { (assign105640_e158107 * (assign105640_e158106 * (var_dnm_dn6 / var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((var_dnm).powf(assign105640_e158106 - 1.0) * var_dnm_dn7)) } } else { (assign105640_e158107 * (assign105640_e158106 * (var_dnm_dn7 / var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((var_dnm).powf(assign105640_e158106 - 1.0) * var_dnm_dn8)) } } else { (assign105640_e158107 * (assign105640_e158106 * (var_dnm_dn8 / var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((var_dnm).powf(assign105640_e158106 - 1.0) * var_dnm_dn9)) } } else { (assign105640_e158107 * (assign105640_e158106 * (var_dnm_dn9 / var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((var_dnm).powf(assign105640_e158106 - 1.0) * var_dnm_dn10)) } } else { (assign105640_e158107 * (assign105640_e158106 * (var_dnm_dn10 / var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((var_dnm).powf(assign105640_e158106 - 1.0) * var_dnm_dn11)) } } else { (assign105640_e158107 * (assign105640_e158106 * (var_dnm_dn11 / var_dnm))) }, if 0.0 == 0.0 && ((assign105640_e158106) as f64).is_finite() && ((assign105640_e158106) as f64).fract() == 0.0 { if assign105640_e158106 == 0.0 { 0.0 } else { (assign105640_e158106 * ((var_dnm).powf(assign105640_e158106 - 1.0) * var_dnm_dn14)) } } else { (assign105640_e158107 * (assign105640_e158106 * (var_dnm_dn14 / var_dnm))) },)
            }
        };
        (assign105640_e158108, assign105640_e158108_d_n0, assign105640_e158108_d_n2, assign105640_e158108_d_n4, assign105640_e158108_d_n5, assign105640_e158108_d_n6, assign105640_e158108_d_n7, assign105640_e158108_d_n8, assign105640_e158108_d_n9, assign105640_e158108_d_n10, assign105640_e158108_d_n11, assign105640_e158108_d_n14,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn11, var_dnm_dn14,)
    }
};
        var_dnm = assign105640_e158110;
        var_dnm_dn0 = assign105640_e158110_d_n0;
        var_dnm_dn2 = assign105640_e158110_d_n2;
        var_dnm_dn4 = assign105640_e158110_d_n4;
        var_dnm_dn5 = assign105640_e158110_d_n5;
        var_dnm_dn6 = assign105640_e158110_d_n6;
        var_dnm_dn7 = assign105640_e158110_d_n7;
        var_dnm_dn8 = assign105640_e158110_d_n8;
        var_dnm_dn9 = assign105640_e158110_d_n9;
        var_dnm_dn10 = assign105640_e158110_d_n10;
        var_dnm_dn11 = assign105640_e158110_d_n11;
        var_dnm_dn14 = assign105640_e158110_d_n14;

        let (assign105650_e158121, assign105650_e158121_d_n0, assign105650_e158121_d_n2, assign105650_e158121_d_n4, assign105650_e158121_d_n5, assign105650_e158121_d_n6, assign105650_e158121_d_n7, assign105650_e158121_d_n8, assign105650_e158121_d_n9, assign105650_e158121_d_n10, assign105650_e158121_d_n11, assign105650_e158121_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        let assign105650_e158119: f64 = (1.0 / var_dnm);
        (assign105650_e158119, (-(var_dnm_dn0 / (var_dnm * var_dnm))), (-(var_dnm_dn2 / (var_dnm * var_dnm))), (-(var_dnm_dn4 / (var_dnm * var_dnm))), (-(var_dnm_dn5 / (var_dnm * var_dnm))), (-(var_dnm_dn6 / (var_dnm * var_dnm))), (-(var_dnm_dn7 / (var_dnm * var_dnm))), (-(var_dnm_dn8 / (var_dnm * var_dnm))), (-(var_dnm_dn9 / (var_dnm * var_dnm))), (-(var_dnm_dn10 / (var_dnm * var_dnm))), (-(var_dnm_dn11 / (var_dnm * var_dnm))), (-(var_dnm_dn14 / (var_dnm * var_dnm))),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn11, var_dnm_dn14,)
    }
};
        var_dnm = assign105650_e158121;
        var_dnm_dn0 = assign105650_e158121_d_n0;
        var_dnm_dn2 = assign105650_e158121_d_n2;
        var_dnm_dn4 = assign105650_e158121_d_n4;
        var_dnm_dn5 = assign105650_e158121_d_n5;
        var_dnm_dn6 = assign105650_e158121_d_n6;
        var_dnm_dn7 = assign105650_e158121_d_n7;
        var_dnm_dn8 = assign105650_e158121_d_n8;
        var_dnm_dn9 = assign105650_e158121_d_n9;
        var_dnm_dn10 = assign105650_e158121_d_n10;
        var_dnm_dn11 = assign105650_e158121_d_n11;
        var_dnm_dn14 = assign105650_e158121_d_n14;

        let (assign105660_e158134, assign105660_e158134_d_n0, assign105660_e158134_d_n2, assign105660_e158134_d_n4, assign105660_e158134_d_n5, assign105660_e158134_d_n6, assign105660_e158134_d_n7, assign105660_e158134_d_n8, assign105660_e158134_d_n9, assign105660_e158134_d_n10, assign105660_e158134_d_n11, assign105660_e158134_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        let assign105660_e158130: f64 = (var_tmf1 * 1000.0);
        let assign105660_e158132: f64 = (assign105660_e158130 * var_dnm);
        (assign105660_e158132, (((var_tmf1_dn0 * 1000.0) * var_dnm) + (assign105660_e158130 * var_dnm_dn0)), (((var_tmf1_dn2 * 1000.0) * var_dnm) + (assign105660_e158130 * var_dnm_dn2)), (((var_tmf1_dn4 * 1000.0) * var_dnm) + (assign105660_e158130 * var_dnm_dn4)), (((var_tmf1_dn5 * 1000.0) * var_dnm) + (assign105660_e158130 * var_dnm_dn5)), (((var_tmf1_dn6 * 1000.0) * var_dnm) + (assign105660_e158130 * var_dnm_dn6)), (((var_tmf1_dn7 * 1000.0) * var_dnm) + (assign105660_e158130 * var_dnm_dn7)), (((var_tmf1_dn8 * 1000.0) * var_dnm) + (assign105660_e158130 * var_dnm_dn8)), (((var_tmf1_dn9 * 1000.0) * var_dnm) + (assign105660_e158130 * var_dnm_dn9)), (((var_tmf1_dn10 * 1000.0) * var_dnm) + (assign105660_e158130 * var_dnm_dn10)), (((var_tmf1_dn11 * 1000.0) * var_dnm) + (assign105660_e158130 * var_dnm_dn11)), (((var_tmf1_dn14 * 1000.0) * var_dnm) + (assign105660_e158130 * var_dnm_dn14)),)
    } else {
        (var_tmf0, var_tmf0_dn0, var_tmf0_dn2, var_tmf0_dn4, var_tmf0_dn5, var_tmf0_dn6, var_tmf0_dn7, var_tmf0_dn8, var_tmf0_dn9, var_tmf0_dn10, var_tmf0_dn11, var_tmf0_dn14,)
    }
};
        var_tmf0 = assign105660_e158134;
        var_tmf0_dn0 = assign105660_e158134_d_n0;
        var_tmf0_dn2 = assign105660_e158134_d_n2;
        var_tmf0_dn4 = assign105660_e158134_d_n4;
        var_tmf0_dn5 = assign105660_e158134_d_n5;
        var_tmf0_dn6 = assign105660_e158134_d_n6;
        var_tmf0_dn7 = assign105660_e158134_d_n7;
        var_tmf0_dn8 = assign105660_e158134_d_n8;
        var_tmf0_dn9 = assign105660_e158134_d_n9;
        var_tmf0_dn10 = assign105660_e158134_d_n10;
        var_tmf0_dn11 = assign105660_e158134_d_n11;
        var_tmf0_dn14 = assign105660_e158134_d_n14;

        let (assign105670_e158149, assign105670_e158149_d_n0, assign105670_e158149_d_n2, assign105670_e158149_d_n4, assign105670_e158149_d_n5, assign105670_e158149_d_n6, assign105670_e158149_d_n7, assign105670_e158149_d_n8, assign105670_e158149_d_n9, assign105670_e158149_d_n10, assign105670_e158149_d_n11, assign105670_e158149_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        let assign105670_e158143: f64 = (1000.0 * var_xmp);
        let assign105670_e158145: f64 = (assign105670_e158143 * var_dnm);
        let assign105670_e158147: f64 = (assign105670_e158145 / var_arg);
        (assign105670_e158147, ((((((1000.0 * var_xmp_dn0) * var_dnm) + (assign105670_e158143 * var_dnm_dn0)) * var_arg) - (assign105670_e158145 * var_arg_dn0)) / (var_arg * var_arg)), ((((((1000.0 * var_xmp_dn2) * var_dnm) + (assign105670_e158143 * var_dnm_dn2)) * var_arg) - (assign105670_e158145 * var_arg_dn2)) / (var_arg * var_arg)), ((((((1000.0 * var_xmp_dn4) * var_dnm) + (assign105670_e158143 * var_dnm_dn4)) * var_arg) - (assign105670_e158145 * var_arg_dn4)) / (var_arg * var_arg)), ((((((1000.0 * var_xmp_dn5) * var_dnm) + (assign105670_e158143 * var_dnm_dn5)) * var_arg) - (assign105670_e158145 * var_arg_dn5)) / (var_arg * var_arg)), ((((((1000.0 * var_xmp_dn6) * var_dnm) + (assign105670_e158143 * var_dnm_dn6)) * var_arg) - (assign105670_e158145 * var_arg_dn6)) / (var_arg * var_arg)), ((((((1000.0 * var_xmp_dn7) * var_dnm) + (assign105670_e158143 * var_dnm_dn7)) * var_arg) - (assign105670_e158145 * var_arg_dn7)) / (var_arg * var_arg)), ((((((1000.0 * var_xmp_dn8) * var_dnm) + (assign105670_e158143 * var_dnm_dn8)) * var_arg) - (assign105670_e158145 * var_arg_dn8)) / (var_arg * var_arg)), ((((((1000.0 * var_xmp_dn9) * var_dnm) + (assign105670_e158143 * var_dnm_dn9)) * var_arg) - (assign105670_e158145 * var_arg_dn9)) / (var_arg * var_arg)), ((((((1000.0 * var_xmp_dn10) * var_dnm) + (assign105670_e158143 * var_dnm_dn10)) * var_arg) - (assign105670_e158145 * var_arg_dn10)) / (var_arg * var_arg)), ((((((1000.0 * var_xmp_dn11) * var_dnm) + (assign105670_e158143 * var_dnm_dn11)) * var_arg) - (assign105670_e158145 * var_arg_dn11)) / (var_arg * var_arg)), ((((((1000.0 * var_xmp_dn14) * var_dnm) + (assign105670_e158143 * var_dnm_dn14)) * var_arg) - (assign105670_e158145 * var_arg_dn14)) / (var_arg * var_arg)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign105670_e158149;
        var_t0_dn0 = assign105670_e158149_d_n0;
        var_t0_dn2 = assign105670_e158149_d_n2;
        var_t0_dn4 = assign105670_e158149_d_n4;
        var_t0_dn5 = assign105670_e158149_d_n5;
        var_t0_dn6 = assign105670_e158149_d_n6;
        var_t0_dn7 = assign105670_e158149_d_n7;
        var_t0_dn8 = assign105670_e158149_d_n8;
        var_t0_dn9 = assign105670_e158149_d_n9;
        var_t0_dn10 = assign105670_e158149_d_n10;
        var_t0_dn11 = assign105670_e158149_d_n11;
        var_t0_dn14 = assign105670_e158149_d_n14;

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
        *var_dnm_slot = var_dnm;
        *var_dnm_dn0_slot = var_dnm_dn0;
        *var_dnm_dn10_slot = var_dnm_dn10;
        *var_dnm_dn11_slot = var_dnm_dn11;
        *var_dnm_dn14_slot = var_dnm_dn14;
        *var_dnm_dn2_slot = var_dnm_dn2;
        *var_dnm_dn4_slot = var_dnm_dn4;
        *var_dnm_dn5_slot = var_dnm_dn5;
        *var_dnm_dn6_slot = var_dnm_dn6;
        *var_dnm_dn7_slot = var_dnm_dn7;
        *var_dnm_dn8_slot = var_dnm_dn8;
        *var_dnm_dn9_slot = var_dnm_dn9;
        *var_guard2391_slot = var_guard2391;
        *var_guard2392_slot = var_guard2392;
        *var_guard2393_slot = var_guard2393;
        *var_guard2394_slot = var_guard2394;
        *var_guard2395_slot = var_guard2395;
        *var_guard2396_slot = var_guard2396;
        *var_m0_slot = var_m0;
        *var_mm_slot = var_mm;
        *var_rdd_slot = var_rdd;
        *var_rdd_dn0_slot = var_rdd_dn0;
        *var_rdd_dn10_slot = var_rdd_dn10;
        *var_rdd_dn11_slot = var_rdd_dn11;
        *var_rdd_dn14_slot = var_rdd_dn14;
        *var_rdd_dn2_slot = var_rdd_dn2;
        *var_rdd_dn4_slot = var_rdd_dn4;
        *var_rdd_dn5_slot = var_rdd_dn5;
        *var_rdd_dn6_slot = var_rdd_dn6;
        *var_rdd_dn7_slot = var_rdd_dn7;
        *var_rdd_dn8_slot = var_rdd_dn8;
        *var_rdd_dn9_slot = var_rdd_dn9;
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
        *var_tmf0_slot = var_tmf0;
        *var_tmf0_dn0_slot = var_tmf0_dn0;
        *var_tmf0_dn10_slot = var_tmf0_dn10;
        *var_tmf0_dn11_slot = var_tmf0_dn11;
        *var_tmf0_dn14_slot = var_tmf0_dn14;
        *var_tmf0_dn2_slot = var_tmf0_dn2;
        *var_tmf0_dn4_slot = var_tmf0_dn4;
        *var_tmf0_dn5_slot = var_tmf0_dn5;
        *var_tmf0_dn6_slot = var_tmf0_dn6;
        *var_tmf0_dn7_slot = var_tmf0_dn7;
        *var_tmf0_dn8_slot = var_tmf0_dn8;
        *var_tmf0_dn9_slot = var_tmf0_dn9;
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
        *var_x2_slot = var_x2;
        *var_x2_dn0_slot = var_x2_dn0;
        *var_x2_dn10_slot = var_x2_dn10;
        *var_x2_dn11_slot = var_x2_dn11;
        *var_x2_dn14_slot = var_x2_dn14;
        *var_x2_dn2_slot = var_x2_dn2;
        *var_x2_dn4_slot = var_x2_dn4;
        *var_x2_dn5_slot = var_x2_dn5;
        *var_x2_dn6_slot = var_x2_dn6;
        *var_x2_dn7_slot = var_x2_dn7;
        *var_x2_dn8_slot = var_x2_dn8;
        *var_x2_dn9_slot = var_x2_dn9;
        *var_xmax2_slot = var_xmax2;
        *var_xmax2_dn0_slot = var_xmax2_dn0;
        *var_xmax2_dn10_slot = var_xmax2_dn10;
        *var_xmax2_dn11_slot = var_xmax2_dn11;
        *var_xmax2_dn14_slot = var_xmax2_dn14;
        *var_xmax2_dn2_slot = var_xmax2_dn2;
        *var_xmax2_dn4_slot = var_xmax2_dn4;
        *var_xmax2_dn5_slot = var_xmax2_dn5;
        *var_xmax2_dn6_slot = var_xmax2_dn6;
        *var_xmax2_dn7_slot = var_xmax2_dn7;
        *var_xmax2_dn8_slot = var_xmax2_dn8;
        *var_xmax2_dn9_slot = var_xmax2_dn9;
        *var_xmp_slot = var_xmp;
        *var_xmp_dn0_slot = var_xmp_dn0;
        *var_xmp_dn10_slot = var_xmp_dn10;
        *var_xmp_dn11_slot = var_xmp_dn11;
        *var_xmp_dn14_slot = var_xmp_dn14;
        *var_xmp_dn2_slot = var_xmp_dn2;
        *var_xmp_dn4_slot = var_xmp_dn4;
        *var_xmp_dn5_slot = var_xmp_dn5;
        *var_xmp_dn6_slot = var_xmp_dn6;
        *var_xmp_dn7_slot = var_xmp_dn7;
        *var_xmp_dn8_slot = var_xmp_dn8;
        *var_xmp_dn9_slot = var_xmp_dn9;
        *var_xp_slot = var_xp;
        *var_xp_dn0_slot = var_xp_dn0;
        *var_xp_dn10_slot = var_xp_dn10;
        *var_xp_dn11_slot = var_xp_dn11;
        *var_xp_dn14_slot = var_xp_dn14;
        *var_xp_dn2_slot = var_xp_dn2;
        *var_xp_dn4_slot = var_xp_dn4;
        *var_xp_dn5_slot = var_xp_dn5;
        *var_xp_dn6_slot = var_xp_dn6;
        *var_xp_dn7_slot = var_xp_dn7;
        *var_xp_dn8_slot = var_xp_dn8;
        *var_xp_dn9_slot = var_xp_dn9;
    }

    pub(super) fn stamp_transient_block_388(
        p: &Parameters,
        var_guard2340: f64,
        var_guard2360: f64,
        var_guard2391: f64,
        var_idse: f64,
        var_idse_dn0: f64,
        var_idse_dn10: f64,
        var_idse_dn11: f64,
        var_idse_dn14: f64,
        var_idse_dn2: f64,
        var_idse_dn4: f64,
        var_idse_dn5: f64,
        var_idse_dn6: f64,
        var_idse_dn7: f64,
        var_idse_dn8: f64,
        var_idse_dn9: f64,
        var_igbe: f64,
        var_igbe_dn0: f64,
        var_igbe_dn10: f64,
        var_igbe_dn11: f64,
        var_igbe_dn14: f64,
        var_igbe_dn2: f64,
        var_igbe_dn4: f64,
        var_igbe_dn5: f64,
        var_igbe_dn6: f64,
        var_igbe_dn7: f64,
        var_igbe_dn8: f64,
        var_igbe_dn9: f64,
        var_igde: f64,
        var_igde_dn0: f64,
        var_igde_dn10: f64,
        var_igde_dn11: f64,
        var_igde_dn14: f64,
        var_igde_dn2: f64,
        var_igde_dn4: f64,
        var_igde_dn5: f64,
        var_igde_dn6: f64,
        var_igde_dn7: f64,
        var_igde_dn8: f64,
        var_igde_dn9: f64,
        var_igse: f64,
        var_igse_dn0: f64,
        var_igse_dn10: f64,
        var_igse_dn11: f64,
        var_igse_dn14: f64,
        var_igse_dn2: f64,
        var_igse_dn4: f64,
        var_igse_dn5: f64,
        var_igse_dn6: f64,
        var_igse_dn7: f64,
        var_igse_dn8: f64,
        var_igse_dn9: f64,
        var_ldrift0: f64,
        var_mfactor: f64,
        var_mks_nsubsub: f64,
        var_mode: f64,
        var_qde: f64,
        var_qde_dn0: f64,
        var_qde_dn10: f64,
        var_qde_dn11: f64,
        var_qde_dn14: f64,
        var_qde_dn2: f64,
        var_qde_dn4: f64,
        var_qde_dn5: f64,
        var_qde_dn6: f64,
        var_qde_dn7: f64,
        var_qde_dn8: f64,
        var_qde_dn9: f64,
        var_qge: f64,
        var_qge_dn0: f64,
        var_qge_dn10: f64,
        var_qge_dn11: f64,
        var_qge_dn14: f64,
        var_qge_dn2: f64,
        var_qge_dn4: f64,
        var_qge_dn5: f64,
        var_qge_dn6: f64,
        var_qge_dn7: f64,
        var_qge_dn8: f64,
        var_qge_dn9: f64,
        var_qse: f64,
        var_qse_dn0: f64,
        var_qse_dn10: f64,
        var_qse_dn11: f64,
        var_qse_dn14: f64,
        var_qse_dn2: f64,
        var_qse_dn4: f64,
        var_qse_dn5: f64,
        var_qse_dn6: f64,
        var_qse_dn7: f64,
        var_qse_dn8: f64,
        var_qse_dn9: f64,
        var_rd0: f64,
        var_tmf0: f64,
        var_tmf0_dn0: f64,
        var_tmf0_dn10: f64,
        var_tmf0_dn11: f64,
        var_tmf0_dn14: f64,
        var_tmf0_dn2: f64,
        var_tmf0_dn4: f64,
        var_tmf0_dn5: f64,
        var_tmf0_dn6: f64,
        var_tmf0_dn7: f64,
        var_tmf0_dn8: f64,
        var_tmf0_dn9: f64,
        var_uc_nover: f64,
        var_vdsemodenml: f64,
        var_wdep: f64,
        var_wdep_dn0: f64,
        var_wdep_dn10: f64,
        var_wdep_dn11: f64,
        var_wdep_dn14: f64,
        var_wdep_dn2: f64,
        var_wdep_dn4: f64,
        var_wdep_dn5: f64,
        var_wdep_dn6: f64,
        var_wdep_dn7: f64,
        var_wdep_dn8: f64,
        var_wdep_dn9: f64,
        var_ddriftld_slot: &mut f64,
        var_ddriftld_dn0_slot: &mut f64,
        var_ddriftld_dn10_slot: &mut f64,
        var_ddriftld_dn11_slot: &mut f64,
        var_ddriftld_dn14_slot: &mut f64,
        var_ddriftld_dn2_slot: &mut f64,
        var_ddriftld_dn4_slot: &mut f64,
        var_ddriftld_dn5_slot: &mut f64,
        var_ddriftld_dn6_slot: &mut f64,
        var_ddriftld_dn7_slot: &mut f64,
        var_ddriftld_dn8_slot: &mut f64,
        var_ddriftld_dn9_slot: &mut f64,
        var_guard2397_slot: &mut f64,
        var_guard2399_slot: &mut f64,
        var_guard2400_slot: &mut f64,
        var_guard2401_slot: &mut f64,
        var_guard2402_slot: &mut f64,
        var_guard2403_slot: &mut f64,
        var_ids_slot: &mut f64,
        var_ids_dn0_slot: &mut f64,
        var_ids_dn10_slot: &mut f64,
        var_ids_dn11_slot: &mut f64,
        var_ids_dn14_slot: &mut f64,
        var_ids_dn2_slot: &mut f64,
        var_ids_dn4_slot: &mut f64,
        var_ids_dn5_slot: &mut f64,
        var_ids_dn6_slot: &mut f64,
        var_ids_dn7_slot: &mut f64,
        var_ids_dn8_slot: &mut f64,
        var_ids_dn9_slot: &mut f64,
        var_igb_slot: &mut f64,
        var_igb_dn0_slot: &mut f64,
        var_igb_dn10_slot: &mut f64,
        var_igb_dn11_slot: &mut f64,
        var_igb_dn14_slot: &mut f64,
        var_igb_dn2_slot: &mut f64,
        var_igb_dn4_slot: &mut f64,
        var_igb_dn5_slot: &mut f64,
        var_igb_dn6_slot: &mut f64,
        var_igb_dn7_slot: &mut f64,
        var_igb_dn8_slot: &mut f64,
        var_igb_dn9_slot: &mut f64,
        var_igd_slot: &mut f64,
        var_igd_dn0_slot: &mut f64,
        var_igd_dn10_slot: &mut f64,
        var_igd_dn11_slot: &mut f64,
        var_igd_dn14_slot: &mut f64,
        var_igd_dn2_slot: &mut f64,
        var_igd_dn4_slot: &mut f64,
        var_igd_dn5_slot: &mut f64,
        var_igd_dn6_slot: &mut f64,
        var_igd_dn7_slot: &mut f64,
        var_igd_dn8_slot: &mut f64,
        var_igd_dn9_slot: &mut f64,
        var_igs_slot: &mut f64,
        var_igs_dn0_slot: &mut f64,
        var_igs_dn10_slot: &mut f64,
        var_igs_dn11_slot: &mut f64,
        var_igs_dn14_slot: &mut f64,
        var_igs_dn2_slot: &mut f64,
        var_igs_dn4_slot: &mut f64,
        var_igs_dn5_slot: &mut f64,
        var_igs_dn6_slot: &mut f64,
        var_igs_dn7_slot: &mut f64,
        var_igs_dn8_slot: &mut f64,
        var_igs_dn9_slot: &mut f64,
        var_qb_slot: &mut f64,
        var_qb_dn0_slot: &mut f64,
        var_qb_dn10_slot: &mut f64,
        var_qb_dn11_slot: &mut f64,
        var_qb_dn14_slot: &mut f64,
        var_qb_dn2_slot: &mut f64,
        var_qb_dn4_slot: &mut f64,
        var_qb_dn5_slot: &mut f64,
        var_qb_dn6_slot: &mut f64,
        var_qb_dn7_slot: &mut f64,
        var_qb_dn8_slot: &mut f64,
        var_qb_dn9_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn0_slot: &mut f64,
        var_qd_dn10_slot: &mut f64,
        var_qd_dn11_slot: &mut f64,
        var_qd_dn14_slot: &mut f64,
        var_qd_dn2_slot: &mut f64,
        var_qd_dn4_slot: &mut f64,
        var_qd_dn5_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn7_slot: &mut f64,
        var_qd_dn8_slot: &mut f64,
        var_qd_dn9_slot: &mut f64,
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
        var_rdd_slot: &mut f64,
        var_rdd_dn0_slot: &mut f64,
        var_rdd_dn10_slot: &mut f64,
        var_rdd_dn11_slot: &mut f64,
        var_rdd_dn14_slot: &mut f64,
        var_rdd_dn2_slot: &mut f64,
        var_rdd_dn4_slot: &mut f64,
        var_rdd_dn5_slot: &mut f64,
        var_rdd_dn6_slot: &mut f64,
        var_rdd_dn7_slot: &mut f64,
        var_rdd_dn8_slot: &mut f64,
        var_rdd_dn9_slot: &mut f64,
        var_rdde_slot: &mut f64,
        var_rdde_dn0_slot: &mut f64,
        var_rdde_dn10_slot: &mut f64,
        var_rdde_dn11_slot: &mut f64,
        var_rdde_dn14_slot: &mut f64,
        var_rdde_dn2_slot: &mut f64,
        var_rdde_dn4_slot: &mut f64,
        var_rdde_dn5_slot: &mut f64,
        var_rdde_dn6_slot: &mut f64,
        var_rdde_dn7_slot: &mut f64,
        var_rdde_dn8_slot: &mut f64,
        var_rdde_dn9_slot: &mut f64,
        var_rsd_slot: &mut f64,
        var_rsd_dn0_slot: &mut f64,
        var_rsd_dn10_slot: &mut f64,
        var_rsd_dn11_slot: &mut f64,
        var_rsd_dn14_slot: &mut f64,
        var_rsd_dn2_slot: &mut f64,
        var_rsd_dn4_slot: &mut f64,
        var_rsd_dn5_slot: &mut f64,
        var_rsd_dn6_slot: &mut f64,
        var_rsd_dn7_slot: &mut f64,
        var_rsd_dn8_slot: &mut f64,
        var_rsd_dn9_slot: &mut f64,
        var_rsde_slot: &mut f64,
        var_rsde_dn0_slot: &mut f64,
        var_rsde_dn10_slot: &mut f64,
        var_rsde_dn11_slot: &mut f64,
        var_rsde_dn14_slot: &mut f64,
        var_rsde_dn2_slot: &mut f64,
        var_rsde_dn4_slot: &mut f64,
        var_rsde_dn5_slot: &mut f64,
        var_rsde_dn6_slot: &mut f64,
        var_rsde_dn7_slot: &mut f64,
        var_rsde_dn8_slot: &mut f64,
        var_rsde_dn9_slot: &mut f64,
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
    ) {
        let mut var_ddriftld: f64 = *var_ddriftld_slot;
        let mut var_ddriftld_dn0: f64 = *var_ddriftld_dn0_slot;
        let mut var_ddriftld_dn10: f64 = *var_ddriftld_dn10_slot;
        let mut var_ddriftld_dn11: f64 = *var_ddriftld_dn11_slot;
        let mut var_ddriftld_dn14: f64 = *var_ddriftld_dn14_slot;
        let mut var_ddriftld_dn2: f64 = *var_ddriftld_dn2_slot;
        let mut var_ddriftld_dn4: f64 = *var_ddriftld_dn4_slot;
        let mut var_ddriftld_dn5: f64 = *var_ddriftld_dn5_slot;
        let mut var_ddriftld_dn6: f64 = *var_ddriftld_dn6_slot;
        let mut var_ddriftld_dn7: f64 = *var_ddriftld_dn7_slot;
        let mut var_ddriftld_dn8: f64 = *var_ddriftld_dn8_slot;
        let mut var_ddriftld_dn9: f64 = *var_ddriftld_dn9_slot;
        let mut var_guard2397: f64 = *var_guard2397_slot;
        let mut var_guard2399: f64 = *var_guard2399_slot;
        let mut var_guard2400: f64 = *var_guard2400_slot;
        let mut var_guard2401: f64 = *var_guard2401_slot;
        let mut var_guard2402: f64 = *var_guard2402_slot;
        let mut var_guard2403: f64 = *var_guard2403_slot;
        let mut var_ids: f64 = *var_ids_slot;
        let mut var_ids_dn0: f64 = *var_ids_dn0_slot;
        let mut var_ids_dn10: f64 = *var_ids_dn10_slot;
        let mut var_ids_dn11: f64 = *var_ids_dn11_slot;
        let mut var_ids_dn14: f64 = *var_ids_dn14_slot;
        let mut var_ids_dn2: f64 = *var_ids_dn2_slot;
        let mut var_ids_dn4: f64 = *var_ids_dn4_slot;
        let mut var_ids_dn5: f64 = *var_ids_dn5_slot;
        let mut var_ids_dn6: f64 = *var_ids_dn6_slot;
        let mut var_ids_dn7: f64 = *var_ids_dn7_slot;
        let mut var_ids_dn8: f64 = *var_ids_dn8_slot;
        let mut var_ids_dn9: f64 = *var_ids_dn9_slot;
        let mut var_igb: f64 = *var_igb_slot;
        let mut var_igb_dn0: f64 = *var_igb_dn0_slot;
        let mut var_igb_dn10: f64 = *var_igb_dn10_slot;
        let mut var_igb_dn11: f64 = *var_igb_dn11_slot;
        let mut var_igb_dn14: f64 = *var_igb_dn14_slot;
        let mut var_igb_dn2: f64 = *var_igb_dn2_slot;
        let mut var_igb_dn4: f64 = *var_igb_dn4_slot;
        let mut var_igb_dn5: f64 = *var_igb_dn5_slot;
        let mut var_igb_dn6: f64 = *var_igb_dn6_slot;
        let mut var_igb_dn7: f64 = *var_igb_dn7_slot;
        let mut var_igb_dn8: f64 = *var_igb_dn8_slot;
        let mut var_igb_dn9: f64 = *var_igb_dn9_slot;
        let mut var_igd: f64 = *var_igd_slot;
        let mut var_igd_dn0: f64 = *var_igd_dn0_slot;
        let mut var_igd_dn10: f64 = *var_igd_dn10_slot;
        let mut var_igd_dn11: f64 = *var_igd_dn11_slot;
        let mut var_igd_dn14: f64 = *var_igd_dn14_slot;
        let mut var_igd_dn2: f64 = *var_igd_dn2_slot;
        let mut var_igd_dn4: f64 = *var_igd_dn4_slot;
        let mut var_igd_dn5: f64 = *var_igd_dn5_slot;
        let mut var_igd_dn6: f64 = *var_igd_dn6_slot;
        let mut var_igd_dn7: f64 = *var_igd_dn7_slot;
        let mut var_igd_dn8: f64 = *var_igd_dn8_slot;
        let mut var_igd_dn9: f64 = *var_igd_dn9_slot;
        let mut var_igs: f64 = *var_igs_slot;
        let mut var_igs_dn0: f64 = *var_igs_dn0_slot;
        let mut var_igs_dn10: f64 = *var_igs_dn10_slot;
        let mut var_igs_dn11: f64 = *var_igs_dn11_slot;
        let mut var_igs_dn14: f64 = *var_igs_dn14_slot;
        let mut var_igs_dn2: f64 = *var_igs_dn2_slot;
        let mut var_igs_dn4: f64 = *var_igs_dn4_slot;
        let mut var_igs_dn5: f64 = *var_igs_dn5_slot;
        let mut var_igs_dn6: f64 = *var_igs_dn6_slot;
        let mut var_igs_dn7: f64 = *var_igs_dn7_slot;
        let mut var_igs_dn8: f64 = *var_igs_dn8_slot;
        let mut var_igs_dn9: f64 = *var_igs_dn9_slot;
        let mut var_qb: f64 = *var_qb_slot;
        let mut var_qb_dn0: f64 = *var_qb_dn0_slot;
        let mut var_qb_dn10: f64 = *var_qb_dn10_slot;
        let mut var_qb_dn11: f64 = *var_qb_dn11_slot;
        let mut var_qb_dn14: f64 = *var_qb_dn14_slot;
        let mut var_qb_dn2: f64 = *var_qb_dn2_slot;
        let mut var_qb_dn4: f64 = *var_qb_dn4_slot;
        let mut var_qb_dn5: f64 = *var_qb_dn5_slot;
        let mut var_qb_dn6: f64 = *var_qb_dn6_slot;
        let mut var_qb_dn7: f64 = *var_qb_dn7_slot;
        let mut var_qb_dn8: f64 = *var_qb_dn8_slot;
        let mut var_qb_dn9: f64 = *var_qb_dn9_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn0: f64 = *var_qd_dn0_slot;
        let mut var_qd_dn10: f64 = *var_qd_dn10_slot;
        let mut var_qd_dn11: f64 = *var_qd_dn11_slot;
        let mut var_qd_dn14: f64 = *var_qd_dn14_slot;
        let mut var_qd_dn2: f64 = *var_qd_dn2_slot;
        let mut var_qd_dn4: f64 = *var_qd_dn4_slot;
        let mut var_qd_dn5: f64 = *var_qd_dn5_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn7: f64 = *var_qd_dn7_slot;
        let mut var_qd_dn8: f64 = *var_qd_dn8_slot;
        let mut var_qd_dn9: f64 = *var_qd_dn9_slot;
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
        let mut var_rdd: f64 = *var_rdd_slot;
        let mut var_rdd_dn0: f64 = *var_rdd_dn0_slot;
        let mut var_rdd_dn10: f64 = *var_rdd_dn10_slot;
        let mut var_rdd_dn11: f64 = *var_rdd_dn11_slot;
        let mut var_rdd_dn14: f64 = *var_rdd_dn14_slot;
        let mut var_rdd_dn2: f64 = *var_rdd_dn2_slot;
        let mut var_rdd_dn4: f64 = *var_rdd_dn4_slot;
        let mut var_rdd_dn5: f64 = *var_rdd_dn5_slot;
        let mut var_rdd_dn6: f64 = *var_rdd_dn6_slot;
        let mut var_rdd_dn7: f64 = *var_rdd_dn7_slot;
        let mut var_rdd_dn8: f64 = *var_rdd_dn8_slot;
        let mut var_rdd_dn9: f64 = *var_rdd_dn9_slot;
        let mut var_rdde: f64 = *var_rdde_slot;
        let mut var_rdde_dn0: f64 = *var_rdde_dn0_slot;
        let mut var_rdde_dn10: f64 = *var_rdde_dn10_slot;
        let mut var_rdde_dn11: f64 = *var_rdde_dn11_slot;
        let mut var_rdde_dn14: f64 = *var_rdde_dn14_slot;
        let mut var_rdde_dn2: f64 = *var_rdde_dn2_slot;
        let mut var_rdde_dn4: f64 = *var_rdde_dn4_slot;
        let mut var_rdde_dn5: f64 = *var_rdde_dn5_slot;
        let mut var_rdde_dn6: f64 = *var_rdde_dn6_slot;
        let mut var_rdde_dn7: f64 = *var_rdde_dn7_slot;
        let mut var_rdde_dn8: f64 = *var_rdde_dn8_slot;
        let mut var_rdde_dn9: f64 = *var_rdde_dn9_slot;
        let mut var_rsd: f64 = *var_rsd_slot;
        let mut var_rsd_dn0: f64 = *var_rsd_dn0_slot;
        let mut var_rsd_dn10: f64 = *var_rsd_dn10_slot;
        let mut var_rsd_dn11: f64 = *var_rsd_dn11_slot;
        let mut var_rsd_dn14: f64 = *var_rsd_dn14_slot;
        let mut var_rsd_dn2: f64 = *var_rsd_dn2_slot;
        let mut var_rsd_dn4: f64 = *var_rsd_dn4_slot;
        let mut var_rsd_dn5: f64 = *var_rsd_dn5_slot;
        let mut var_rsd_dn6: f64 = *var_rsd_dn6_slot;
        let mut var_rsd_dn7: f64 = *var_rsd_dn7_slot;
        let mut var_rsd_dn8: f64 = *var_rsd_dn8_slot;
        let mut var_rsd_dn9: f64 = *var_rsd_dn9_slot;
        let mut var_rsde: f64 = *var_rsde_slot;
        let mut var_rsde_dn0: f64 = *var_rsde_dn0_slot;
        let mut var_rsde_dn10: f64 = *var_rsde_dn10_slot;
        let mut var_rsde_dn11: f64 = *var_rsde_dn11_slot;
        let mut var_rsde_dn14: f64 = *var_rsde_dn14_slot;
        let mut var_rsde_dn2: f64 = *var_rsde_dn2_slot;
        let mut var_rsde_dn4: f64 = *var_rsde_dn4_slot;
        let mut var_rsde_dn5: f64 = *var_rsde_dn5_slot;
        let mut var_rsde_dn6: f64 = *var_rsde_dn6_slot;
        let mut var_rsde_dn7: f64 = *var_rsde_dn7_slot;
        let mut var_rsde_dn8: f64 = *var_rsde_dn8_slot;
        let mut var_rsde_dn9: f64 = *var_rsde_dn9_slot;
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

        let (assign105680_e158162, assign105680_e158162_d_n0, assign105680_e158162_d_n2, assign105680_e158162_d_n4, assign105680_e158162_d_n5, assign105680_e158162_d_n6, assign105680_e158162_d_n7, assign105680_e158162_d_n8, assign105680_e158162_d_n9, assign105680_e158162_d_n10, assign105680_e158162_d_n11, assign105680_e158162_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        let assign105680_e158158: f64 = (1000000.0 - 1000.0);
        let assign105680_e158160: f64 = (assign105680_e158158 + var_tmf0);
        (assign105680_e158160, var_tmf0_dn0, var_tmf0_dn2, var_tmf0_dn4, var_tmf0_dn5, var_tmf0_dn6, var_tmf0_dn7, var_tmf0_dn8, var_tmf0_dn9, var_tmf0_dn10, var_tmf0_dn11, var_tmf0_dn14,)
    } else {
        (var_rdd, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn11, var_rdd_dn14,)
    }
};
        var_rdd = assign105680_e158162;
        var_rdd_dn0 = assign105680_e158162_d_n0;
        var_rdd_dn2 = assign105680_e158162_d_n2;
        var_rdd_dn4 = assign105680_e158162_d_n4;
        var_rdd_dn5 = assign105680_e158162_d_n5;
        var_rdd_dn6 = assign105680_e158162_d_n6;
        var_rdd_dn7 = assign105680_e158162_d_n7;
        var_rdd_dn8 = assign105680_e158162_d_n8;
        var_rdd_dn9 = assign105680_e158162_d_n9;
        var_rdd_dn10 = assign105680_e158162_d_n10;
        var_rdd_dn11 = assign105680_e158162_d_n11;
        var_rdd_dn14 = assign105680_e158162_d_n14;

        let (assign105690_e158171, assign105690_e158171_d_n0, assign105690_e158171_d_n2, assign105690_e158171_d_n4, assign105690_e158171_d_n5, assign105690_e158171_d_n6, assign105690_e158171_d_n7, assign105690_e158171_d_n8, assign105690_e158171_d_n9, assign105690_e158171_d_n10, assign105690_e158171_d_n11, assign105690_e158171_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 != 0.0)) {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign105690_e158171;
        var_t0_dn0 = assign105690_e158171_d_n0;
        var_t0_dn2 = assign105690_e158171_d_n2;
        var_t0_dn4 = assign105690_e158171_d_n4;
        var_t0_dn5 = assign105690_e158171_d_n5;
        var_t0_dn6 = assign105690_e158171_d_n6;
        var_t0_dn7 = assign105690_e158171_d_n7;
        var_t0_dn8 = assign105690_e158171_d_n8;
        var_t0_dn9 = assign105690_e158171_d_n9;
        var_t0_dn10 = assign105690_e158171_d_n10;
        var_t0_dn11 = assign105690_e158171_d_n11;
        var_t0_dn14 = assign105690_e158171_d_n14;

        let (assign105700_e158181, assign105700_e158181_d_n0, assign105700_e158181_d_n2, assign105700_e158181_d_n4, assign105700_e158181_d_n5, assign105700_e158181_d_n6, assign105700_e158181_d_n7, assign105700_e158181_d_n8, assign105700_e158181_d_n9, assign105700_e158181_d_n10, assign105700_e158181_d_n11, assign105700_e158181_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 == 0.0)) {
        (var_rdd, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn11, var_rdd_dn14,)
    } else {
        (var_rdd, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn11, var_rdd_dn14,)
    }
};
        var_rdd = assign105700_e158181;
        var_rdd_dn0 = assign105700_e158181_d_n0;
        var_rdd_dn2 = assign105700_e158181_d_n2;
        var_rdd_dn4 = assign105700_e158181_d_n4;
        var_rdd_dn5 = assign105700_e158181_d_n5;
        var_rdd_dn6 = assign105700_e158181_d_n6;
        var_rdd_dn7 = assign105700_e158181_d_n7;
        var_rdd_dn8 = assign105700_e158181_d_n8;
        var_rdd_dn9 = assign105700_e158181_d_n9;
        var_rdd_dn10 = assign105700_e158181_d_n10;
        var_rdd_dn11 = assign105700_e158181_d_n11;
        var_rdd_dn14 = assign105700_e158181_d_n14;

        let (assign105710_e158191, assign105710_e158191_d_n0, assign105710_e158191_d_n2, assign105710_e158191_d_n4, assign105710_e158191_d_n5, assign105710_e158191_d_n6, assign105710_e158191_d_n7, assign105710_e158191_d_n8, assign105710_e158191_d_n9, assign105710_e158191_d_n10, assign105710_e158191_d_n11, assign105710_e158191_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2391 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign105710_e158191;
        var_t0_dn0 = assign105710_e158191_d_n0;
        var_t0_dn2 = assign105710_e158191_d_n2;
        var_t0_dn4 = assign105710_e158191_d_n4;
        var_t0_dn5 = assign105710_e158191_d_n5;
        var_t0_dn6 = assign105710_e158191_d_n6;
        var_t0_dn7 = assign105710_e158191_d_n7;
        var_t0_dn8 = assign105710_e158191_d_n8;
        var_t0_dn9 = assign105710_e158191_d_n9;
        var_t0_dn10 = assign105710_e158191_d_n10;
        var_t0_dn11 = assign105710_e158191_d_n11;
        var_t0_dn14 = assign105710_e158191_d_n14;

        let assign105720_e158198: f64 = (var_mks_nsubsub + var_uc_nover);
        let assign105720_e158199: f64 = (var_uc_nover * assign105720_e158198);
        let assign105720_e158202: f64 = if ((p.p54 == 1.0) && (assign105720_e158199 > 0.0)) { 1.0 } else { 0.0 };
        var_guard2397 = assign105720_e158202;

        let (assign105730_e158213, assign105730_e158213_d_n0, assign105730_e158213_d_n2, assign105730_e158213_d_n4, assign105730_e158213_d_n5, assign105730_e158213_d_n6, assign105730_e158213_d_n7, assign105730_e158213_d_n8, assign105730_e158213_d_n9, assign105730_e158213_d_n10, assign105730_e158213_d_n11, assign105730_e158213_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2397 != 0.0)) {
        let assign105730_e158211: f64 = (p.p334 - var_wdep);
        (assign105730_e158211, (-var_wdep_dn0), (-var_wdep_dn2), (-var_wdep_dn4), (-var_wdep_dn5), (-var_wdep_dn6), (-var_wdep_dn7), (-var_wdep_dn8), (-var_wdep_dn9), (-var_wdep_dn10), (-var_wdep_dn11), (-var_wdep_dn14),)
    } else {
        (var_ddriftld, var_ddriftld_dn0, var_ddriftld_dn2, var_ddriftld_dn4, var_ddriftld_dn5, var_ddriftld_dn6, var_ddriftld_dn7, var_ddriftld_dn8, var_ddriftld_dn9, var_ddriftld_dn10, var_ddriftld_dn11, var_ddriftld_dn14,)
    }
};
        var_ddriftld = assign105730_e158213;
        var_ddriftld_dn0 = assign105730_e158213_d_n0;
        var_ddriftld_dn2 = assign105730_e158213_d_n2;
        var_ddriftld_dn4 = assign105730_e158213_d_n4;
        var_ddriftld_dn5 = assign105730_e158213_d_n5;
        var_ddriftld_dn6 = assign105730_e158213_d_n6;
        var_ddriftld_dn7 = assign105730_e158213_d_n7;
        var_ddriftld_dn8 = assign105730_e158213_d_n8;
        var_ddriftld_dn9 = assign105730_e158213_d_n9;
        var_ddriftld_dn10 = assign105730_e158213_d_n10;
        var_ddriftld_dn11 = assign105730_e158213_d_n11;
        var_ddriftld_dn14 = assign105730_e158213_d_n14;

        let (assign105740_e158226, assign105740_e158226_d_n0, assign105740_e158226_d_n2, assign105740_e158226_d_n4, assign105740_e158226_d_n5, assign105740_e158226_d_n6, assign105740_e158226_d_n7, assign105740_e158226_d_n8, assign105740_e158226_d_n9, assign105740_e158226_d_n10, assign105740_e158226_d_n11, assign105740_e158226_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2397 != 0.0)) {
        let assign105740_e158222: f64 = (var_rdd * var_ldrift0);
        let assign105740_e158224: f64 = (assign105740_e158222 / var_ddriftld);
        (assign105740_e158224, ((((var_rdd_dn0 * var_ldrift0) * var_ddriftld) - (assign105740_e158222 * var_ddriftld_dn0)) / (var_ddriftld * var_ddriftld)), ((((var_rdd_dn2 * var_ldrift0) * var_ddriftld) - (assign105740_e158222 * var_ddriftld_dn2)) / (var_ddriftld * var_ddriftld)), ((((var_rdd_dn4 * var_ldrift0) * var_ddriftld) - (assign105740_e158222 * var_ddriftld_dn4)) / (var_ddriftld * var_ddriftld)), ((((var_rdd_dn5 * var_ldrift0) * var_ddriftld) - (assign105740_e158222 * var_ddriftld_dn5)) / (var_ddriftld * var_ddriftld)), ((((var_rdd_dn6 * var_ldrift0) * var_ddriftld) - (assign105740_e158222 * var_ddriftld_dn6)) / (var_ddriftld * var_ddriftld)), ((((var_rdd_dn7 * var_ldrift0) * var_ddriftld) - (assign105740_e158222 * var_ddriftld_dn7)) / (var_ddriftld * var_ddriftld)), ((((var_rdd_dn8 * var_ldrift0) * var_ddriftld) - (assign105740_e158222 * var_ddriftld_dn8)) / (var_ddriftld * var_ddriftld)), ((((var_rdd_dn9 * var_ldrift0) * var_ddriftld) - (assign105740_e158222 * var_ddriftld_dn9)) / (var_ddriftld * var_ddriftld)), ((((var_rdd_dn10 * var_ldrift0) * var_ddriftld) - (assign105740_e158222 * var_ddriftld_dn10)) / (var_ddriftld * var_ddriftld)), ((((var_rdd_dn11 * var_ldrift0) * var_ddriftld) - (assign105740_e158222 * var_ddriftld_dn11)) / (var_ddriftld * var_ddriftld)), ((((var_rdd_dn14 * var_ldrift0) * var_ddriftld) - (assign105740_e158222 * var_ddriftld_dn14)) / (var_ddriftld * var_ddriftld)),)
    } else {
        (var_rdd, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn11, var_rdd_dn14,)
    }
};
        var_rdd = assign105740_e158226;
        var_rdd_dn0 = assign105740_e158226_d_n0;
        var_rdd_dn2 = assign105740_e158226_d_n2;
        var_rdd_dn4 = assign105740_e158226_d_n4;
        var_rdd_dn5 = assign105740_e158226_d_n5;
        var_rdd_dn6 = assign105740_e158226_d_n6;
        var_rdd_dn7 = assign105740_e158226_d_n7;
        var_rdd_dn8 = assign105740_e158226_d_n8;
        var_rdd_dn9 = assign105740_e158226_d_n9;
        var_rdd_dn10 = assign105740_e158226_d_n10;
        var_rdd_dn11 = assign105740_e158226_d_n11;
        var_rdd_dn14 = assign105740_e158226_d_n14;

        let (assign105750_e158235, assign105750_e158235_d_n0, assign105750_e158235_d_n2, assign105750_e158235_d_n4, assign105750_e158235_d_n5, assign105750_e158235_d_n6, assign105750_e158235_d_n7, assign105750_e158235_d_n8, assign105750_e158235_d_n9, assign105750_e158235_d_n10, assign105750_e158235_d_n11, assign105750_e158235_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign105750_e158233: f64 = (var_rdd + var_rd0);
        (assign105750_e158233, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn11, var_rdd_dn14,)
    } else {
        (var_rdd, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn11, var_rdd_dn14,)
    }
};
        var_rdd = assign105750_e158235;
        var_rdd_dn0 = assign105750_e158235_d_n0;
        var_rdd_dn2 = assign105750_e158235_d_n2;
        var_rdd_dn4 = assign105750_e158235_d_n4;
        var_rdd_dn5 = assign105750_e158235_d_n5;
        var_rdd_dn6 = assign105750_e158235_d_n6;
        var_rdd_dn7 = assign105750_e158235_d_n7;
        var_rdd_dn8 = assign105750_e158235_d_n8;
        var_rdd_dn9 = assign105750_e158235_d_n9;
        var_rdd_dn10 = assign105750_e158235_d_n10;
        var_rdd_dn11 = assign105750_e158235_d_n11;
        var_rdd_dn14 = assign105750_e158235_d_n14;

        let assign105790_e158266: f64 = if var_rdd < p.p444 { 1.0 } else { 0.0 };
        var_guard2399 = assign105790_e158266;

        let (assign105800_e158275, assign105800_e158275_d_n0, assign105800_e158275_d_n2, assign105800_e158275_d_n4, assign105800_e158275_d_n5, assign105800_e158275_d_n6, assign105800_e158275_d_n7, assign105800_e158275_d_n8, assign105800_e158275_d_n9, assign105800_e158275_d_n10, assign105800_e158275_d_n11, assign105800_e158275_d_n14,) = {
    if (((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) && (var_guard2399 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdd, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn11, var_rdd_dn14,)
    }
};
        var_rdd = assign105800_e158275;
        var_rdd_dn0 = assign105800_e158275_d_n0;
        var_rdd_dn2 = assign105800_e158275_d_n2;
        var_rdd_dn4 = assign105800_e158275_d_n4;
        var_rdd_dn5 = assign105800_e158275_d_n5;
        var_rdd_dn6 = assign105800_e158275_d_n6;
        var_rdd_dn7 = assign105800_e158275_d_n7;
        var_rdd_dn8 = assign105800_e158275_d_n8;
        var_rdd_dn9 = assign105800_e158275_d_n9;
        var_rdd_dn10 = assign105800_e158275_d_n10;
        var_rdd_dn11 = assign105800_e158275_d_n11;
        var_rdd_dn14 = assign105800_e158275_d_n14;

        let (assign105810_e158284, assign105810_e158284_d_n0, assign105810_e158284_d_n2, assign105810_e158284_d_n4, assign105810_e158284_d_n5, assign105810_e158284_d_n6, assign105810_e158284_d_n7, assign105810_e158284_d_n8, assign105810_e158284_d_n9, assign105810_e158284_d_n10, assign105810_e158284_d_n11, assign105810_e158284_d_n14,) = {
    if ((var_guard2340 != 0.0) && (var_guard2360 == 0.0)) {
        let assign105810_e158282: f64 = (var_rdd / var_mfactor);
        (assign105810_e158282, (var_rdd_dn0 / var_mfactor), (var_rdd_dn2 / var_mfactor), (var_rdd_dn4 / var_mfactor), (var_rdd_dn5 / var_mfactor), (var_rdd_dn6 / var_mfactor), (var_rdd_dn7 / var_mfactor), (var_rdd_dn8 / var_mfactor), (var_rdd_dn9 / var_mfactor), (var_rdd_dn10 / var_mfactor), (var_rdd_dn11 / var_mfactor), (var_rdd_dn14 / var_mfactor),)
    } else {
        (var_rdde, var_rdde_dn0, var_rdde_dn2, var_rdde_dn4, var_rdde_dn5, var_rdde_dn6, var_rdde_dn7, var_rdde_dn8, var_rdde_dn9, var_rdde_dn10, var_rdde_dn11, var_rdde_dn14,)
    }
};
        var_rdde = assign105810_e158284;
        var_rdde_dn0 = assign105810_e158284_d_n0;
        var_rdde_dn2 = assign105810_e158284_d_n2;
        var_rdde_dn4 = assign105810_e158284_d_n4;
        var_rdde_dn5 = assign105810_e158284_d_n5;
        var_rdde_dn6 = assign105810_e158284_d_n6;
        var_rdde_dn7 = assign105810_e158284_d_n7;
        var_rdde_dn8 = assign105810_e158284_d_n8;
        var_rdde_dn9 = assign105810_e158284_d_n9;
        var_rdde_dn10 = assign105810_e158284_d_n10;
        var_rdde_dn11 = assign105810_e158284_d_n11;
        var_rdde_dn14 = assign105810_e158284_d_n14;

        let assign105820_e158287: f64 = if var_rdd < p.p444 { 1.0 } else { 0.0 };
        var_guard2400 = assign105820_e158287;

        let (assign105830_e158294, assign105830_e158294_d_n0, assign105830_e158294_d_n2, assign105830_e158294_d_n4, assign105830_e158294_d_n5, assign105830_e158294_d_n6, assign105830_e158294_d_n7, assign105830_e158294_d_n8, assign105830_e158294_d_n9, assign105830_e158294_d_n10, assign105830_e158294_d_n11, assign105830_e158294_d_n14,) = {
    if ((var_guard2340 == 0.0) && (var_guard2400 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdd, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn11, var_rdd_dn14,)
    }
};
        var_rdd = assign105830_e158294;
        var_rdd_dn0 = assign105830_e158294_d_n0;
        var_rdd_dn2 = assign105830_e158294_d_n2;
        var_rdd_dn4 = assign105830_e158294_d_n4;
        var_rdd_dn5 = assign105830_e158294_d_n5;
        var_rdd_dn6 = assign105830_e158294_d_n6;
        var_rdd_dn7 = assign105830_e158294_d_n7;
        var_rdd_dn8 = assign105830_e158294_d_n8;
        var_rdd_dn9 = assign105830_e158294_d_n9;
        var_rdd_dn10 = assign105830_e158294_d_n10;
        var_rdd_dn11 = assign105830_e158294_d_n11;
        var_rdd_dn14 = assign105830_e158294_d_n14;

        let assign105840_e158297: f64 = if var_rsd < p.p444 { 1.0 } else { 0.0 };
        var_guard2401 = assign105840_e158297;

        let (assign105850_e158304, assign105850_e158304_d_n0, assign105850_e158304_d_n2, assign105850_e158304_d_n4, assign105850_e158304_d_n5, assign105850_e158304_d_n6, assign105850_e158304_d_n7, assign105850_e158304_d_n8, assign105850_e158304_d_n9, assign105850_e158304_d_n10, assign105850_e158304_d_n11, assign105850_e158304_d_n14,) = {
    if ((var_guard2340 == 0.0) && (var_guard2401 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn4, var_rsd_dn5, var_rsd_dn6, var_rsd_dn7, var_rsd_dn8, var_rsd_dn9, var_rsd_dn10, var_rsd_dn11, var_rsd_dn14,)
    }
};
        var_rsd = assign105850_e158304;
        var_rsd_dn0 = assign105850_e158304_d_n0;
        var_rsd_dn2 = assign105850_e158304_d_n2;
        var_rsd_dn4 = assign105850_e158304_d_n4;
        var_rsd_dn5 = assign105850_e158304_d_n5;
        var_rsd_dn6 = assign105850_e158304_d_n6;
        var_rsd_dn7 = assign105850_e158304_d_n7;
        var_rsd_dn8 = assign105850_e158304_d_n8;
        var_rsd_dn9 = assign105850_e158304_d_n9;
        var_rsd_dn10 = assign105850_e158304_d_n10;
        var_rsd_dn11 = assign105850_e158304_d_n11;
        var_rsd_dn14 = assign105850_e158304_d_n14;

        let assign105860_e158307: f64 = if var_vdsemodenml > 0.0 { 1.0 } else { 0.0 };
        var_guard2402 = assign105860_e158307;

        let (assign105870_e158316, assign105870_e158316_d_n0, assign105870_e158316_d_n2, assign105870_e158316_d_n4, assign105870_e158316_d_n5, assign105870_e158316_d_n6, assign105870_e158316_d_n7, assign105870_e158316_d_n8, assign105870_e158316_d_n9, assign105870_e158316_d_n10, assign105870_e158316_d_n11, assign105870_e158316_d_n14,) = {
    if ((var_guard2340 == 0.0) && (var_guard2402 != 0.0)) {
        let assign105870_e158314: f64 = (var_rdd / var_mfactor);
        (assign105870_e158314, (var_rdd_dn0 / var_mfactor), (var_rdd_dn2 / var_mfactor), (var_rdd_dn4 / var_mfactor), (var_rdd_dn5 / var_mfactor), (var_rdd_dn6 / var_mfactor), (var_rdd_dn7 / var_mfactor), (var_rdd_dn8 / var_mfactor), (var_rdd_dn9 / var_mfactor), (var_rdd_dn10 / var_mfactor), (var_rdd_dn11 / var_mfactor), (var_rdd_dn14 / var_mfactor),)
    } else {
        (var_rdde, var_rdde_dn0, var_rdde_dn2, var_rdde_dn4, var_rdde_dn5, var_rdde_dn6, var_rdde_dn7, var_rdde_dn8, var_rdde_dn9, var_rdde_dn10, var_rdde_dn11, var_rdde_dn14,)
    }
};
        var_rdde = assign105870_e158316;
        var_rdde_dn0 = assign105870_e158316_d_n0;
        var_rdde_dn2 = assign105870_e158316_d_n2;
        var_rdde_dn4 = assign105870_e158316_d_n4;
        var_rdde_dn5 = assign105870_e158316_d_n5;
        var_rdde_dn6 = assign105870_e158316_d_n6;
        var_rdde_dn7 = assign105870_e158316_d_n7;
        var_rdde_dn8 = assign105870_e158316_d_n8;
        var_rdde_dn9 = assign105870_e158316_d_n9;
        var_rdde_dn10 = assign105870_e158316_d_n10;
        var_rdde_dn11 = assign105870_e158316_d_n11;
        var_rdde_dn14 = assign105870_e158316_d_n14;

        let (assign105880_e158325, assign105880_e158325_d_n0, assign105880_e158325_d_n2, assign105880_e158325_d_n4, assign105880_e158325_d_n5, assign105880_e158325_d_n6, assign105880_e158325_d_n7, assign105880_e158325_d_n8, assign105880_e158325_d_n9, assign105880_e158325_d_n10, assign105880_e158325_d_n11, assign105880_e158325_d_n14,) = {
    if ((var_guard2340 == 0.0) && (var_guard2402 != 0.0)) {
        let assign105880_e158323: f64 = (var_rsd / var_mfactor);
        (assign105880_e158323, (var_rsd_dn0 / var_mfactor), (var_rsd_dn2 / var_mfactor), (var_rsd_dn4 / var_mfactor), (var_rsd_dn5 / var_mfactor), (var_rsd_dn6 / var_mfactor), (var_rsd_dn7 / var_mfactor), (var_rsd_dn8 / var_mfactor), (var_rsd_dn9 / var_mfactor), (var_rsd_dn10 / var_mfactor), (var_rsd_dn11 / var_mfactor), (var_rsd_dn14 / var_mfactor),)
    } else {
        (var_rsde, var_rsde_dn0, var_rsde_dn2, var_rsde_dn4, var_rsde_dn5, var_rsde_dn6, var_rsde_dn7, var_rsde_dn8, var_rsde_dn9, var_rsde_dn10, var_rsde_dn11, var_rsde_dn14,)
    }
};
        var_rsde = assign105880_e158325;
        var_rsde_dn0 = assign105880_e158325_d_n0;
        var_rsde_dn2 = assign105880_e158325_d_n2;
        var_rsde_dn4 = assign105880_e158325_d_n4;
        var_rsde_dn5 = assign105880_e158325_d_n5;
        var_rsde_dn6 = assign105880_e158325_d_n6;
        var_rsde_dn7 = assign105880_e158325_d_n7;
        var_rsde_dn8 = assign105880_e158325_d_n8;
        var_rsde_dn9 = assign105880_e158325_d_n9;
        var_rsde_dn10 = assign105880_e158325_d_n10;
        var_rsde_dn11 = assign105880_e158325_d_n11;
        var_rsde_dn14 = assign105880_e158325_d_n14;

        let (assign105890_e158335, assign105890_e158335_d_n0, assign105890_e158335_d_n2, assign105890_e158335_d_n4, assign105890_e158335_d_n5, assign105890_e158335_d_n6, assign105890_e158335_d_n7, assign105890_e158335_d_n8, assign105890_e158335_d_n9, assign105890_e158335_d_n10, assign105890_e158335_d_n11, assign105890_e158335_d_n14,) = {
    if ((var_guard2340 == 0.0) && (var_guard2402 == 0.0)) {
        let assign105890_e158333: f64 = (var_rsd / var_mfactor);
        (assign105890_e158333, (var_rsd_dn0 / var_mfactor), (var_rsd_dn2 / var_mfactor), (var_rsd_dn4 / var_mfactor), (var_rsd_dn5 / var_mfactor), (var_rsd_dn6 / var_mfactor), (var_rsd_dn7 / var_mfactor), (var_rsd_dn8 / var_mfactor), (var_rsd_dn9 / var_mfactor), (var_rsd_dn10 / var_mfactor), (var_rsd_dn11 / var_mfactor), (var_rsd_dn14 / var_mfactor),)
    } else {
        (var_rdde, var_rdde_dn0, var_rdde_dn2, var_rdde_dn4, var_rdde_dn5, var_rdde_dn6, var_rdde_dn7, var_rdde_dn8, var_rdde_dn9, var_rdde_dn10, var_rdde_dn11, var_rdde_dn14,)
    }
};
        var_rdde = assign105890_e158335;
        var_rdde_dn0 = assign105890_e158335_d_n0;
        var_rdde_dn2 = assign105890_e158335_d_n2;
        var_rdde_dn4 = assign105890_e158335_d_n4;
        var_rdde_dn5 = assign105890_e158335_d_n5;
        var_rdde_dn6 = assign105890_e158335_d_n6;
        var_rdde_dn7 = assign105890_e158335_d_n7;
        var_rdde_dn8 = assign105890_e158335_d_n8;
        var_rdde_dn9 = assign105890_e158335_d_n9;
        var_rdde_dn10 = assign105890_e158335_d_n10;
        var_rdde_dn11 = assign105890_e158335_d_n11;
        var_rdde_dn14 = assign105890_e158335_d_n14;

        let (assign105900_e158345, assign105900_e158345_d_n0, assign105900_e158345_d_n2, assign105900_e158345_d_n4, assign105900_e158345_d_n5, assign105900_e158345_d_n6, assign105900_e158345_d_n7, assign105900_e158345_d_n8, assign105900_e158345_d_n9, assign105900_e158345_d_n10, assign105900_e158345_d_n11, assign105900_e158345_d_n14,) = {
    if ((var_guard2340 == 0.0) && (var_guard2402 == 0.0)) {
        let assign105900_e158343: f64 = (var_rdd / var_mfactor);
        (assign105900_e158343, (var_rdd_dn0 / var_mfactor), (var_rdd_dn2 / var_mfactor), (var_rdd_dn4 / var_mfactor), (var_rdd_dn5 / var_mfactor), (var_rdd_dn6 / var_mfactor), (var_rdd_dn7 / var_mfactor), (var_rdd_dn8 / var_mfactor), (var_rdd_dn9 / var_mfactor), (var_rdd_dn10 / var_mfactor), (var_rdd_dn11 / var_mfactor), (var_rdd_dn14 / var_mfactor),)
    } else {
        (var_rsde, var_rsde_dn0, var_rsde_dn2, var_rsde_dn4, var_rsde_dn5, var_rsde_dn6, var_rsde_dn7, var_rsde_dn8, var_rsde_dn9, var_rsde_dn10, var_rsde_dn11, var_rsde_dn14,)
    }
};
        var_rsde = assign105900_e158345;
        var_rsde_dn0 = assign105900_e158345_d_n0;
        var_rsde_dn2 = assign105900_e158345_d_n2;
        var_rsde_dn4 = assign105900_e158345_d_n4;
        var_rsde_dn5 = assign105900_e158345_d_n5;
        var_rsde_dn6 = assign105900_e158345_d_n6;
        var_rsde_dn7 = assign105900_e158345_d_n7;
        var_rsde_dn8 = assign105900_e158345_d_n8;
        var_rsde_dn9 = assign105900_e158345_d_n9;
        var_rsde_dn10 = assign105900_e158345_d_n10;
        var_rsde_dn11 = assign105900_e158345_d_n11;
        var_rsde_dn14 = assign105900_e158345_d_n14;

        var_rdd = var_rdde;
        var_rdd_dn0 = var_rdde_dn0;
        var_rdd_dn2 = var_rdde_dn2;
        var_rdd_dn4 = var_rdde_dn4;
        var_rdd_dn5 = var_rdde_dn5;
        var_rdd_dn6 = var_rdde_dn6;
        var_rdd_dn7 = var_rdde_dn7;
        var_rdd_dn8 = var_rdde_dn8;
        var_rdd_dn9 = var_rdde_dn9;
        var_rdd_dn10 = var_rdde_dn10;
        var_rdd_dn11 = var_rdde_dn11;
        var_rdd_dn14 = var_rdde_dn14;

        var_rsd = var_rsde;
        var_rsd_dn0 = var_rsde_dn0;
        var_rsd_dn2 = var_rsde_dn2;
        var_rsd_dn4 = var_rsde_dn4;
        var_rsd_dn5 = var_rsde_dn5;
        var_rsd_dn6 = var_rsde_dn6;
        var_rsd_dn7 = var_rsde_dn7;
        var_rsd_dn8 = var_rsde_dn8;
        var_rsd_dn9 = var_rsde_dn9;
        var_rsd_dn10 = var_rsde_dn10;
        var_rsd_dn11 = var_rsde_dn11;
        var_rsd_dn14 = var_rsde_dn14;

        var_igd = var_igde;
        var_igd_dn0 = var_igde_dn0;
        var_igd_dn2 = var_igde_dn2;
        var_igd_dn4 = var_igde_dn4;
        var_igd_dn5 = var_igde_dn5;
        var_igd_dn6 = var_igde_dn6;
        var_igd_dn7 = var_igde_dn7;
        var_igd_dn8 = var_igde_dn8;
        var_igd_dn9 = var_igde_dn9;
        var_igd_dn10 = var_igde_dn10;
        var_igd_dn11 = var_igde_dn11;
        var_igd_dn14 = var_igde_dn14;

        var_igs = var_igse;
        var_igs_dn0 = var_igse_dn0;
        var_igs_dn2 = var_igse_dn2;
        var_igs_dn4 = var_igse_dn4;
        var_igs_dn5 = var_igse_dn5;
        var_igs_dn6 = var_igse_dn6;
        var_igs_dn7 = var_igse_dn7;
        var_igs_dn8 = var_igse_dn8;
        var_igs_dn9 = var_igse_dn9;
        var_igs_dn10 = var_igse_dn10;
        var_igs_dn11 = var_igse_dn11;
        var_igs_dn14 = var_igse_dn14;

        var_igb = var_igbe;
        var_igb_dn0 = var_igbe_dn0;
        var_igb_dn2 = var_igbe_dn2;
        var_igb_dn4 = var_igbe_dn4;
        var_igb_dn5 = var_igbe_dn5;
        var_igb_dn6 = var_igbe_dn6;
        var_igb_dn7 = var_igbe_dn7;
        var_igb_dn8 = var_igbe_dn8;
        var_igb_dn9 = var_igbe_dn9;
        var_igb_dn10 = var_igbe_dn10;
        var_igb_dn11 = var_igbe_dn11;
        var_igb_dn14 = var_igbe_dn14;

        let assign105960_e158353: f64 = if var_mode > 0.0 { 1.0 } else { 0.0 };
        var_guard2403 = assign105960_e158353;

        let (assign105970_e158357, assign105970_e158357_d_n0, assign105970_e158357_d_n2, assign105970_e158357_d_n4, assign105970_e158357_d_n5, assign105970_e158357_d_n6, assign105970_e158357_d_n7, assign105970_e158357_d_n8, assign105970_e158357_d_n9, assign105970_e158357_d_n10, assign105970_e158357_d_n11, assign105970_e158357_d_n14,) = {
    if (var_guard2403 != 0.0) {
        (var_idse, var_idse_dn0, var_idse_dn2, var_idse_dn4, var_idse_dn5, var_idse_dn6, var_idse_dn7, var_idse_dn8, var_idse_dn9, var_idse_dn10, var_idse_dn11, var_idse_dn14,)
    } else {
        (var_ids, var_ids_dn0, var_ids_dn2, var_ids_dn4, var_ids_dn5, var_ids_dn6, var_ids_dn7, var_ids_dn8, var_ids_dn9, var_ids_dn10, var_ids_dn11, var_ids_dn14,)
    }
};
        var_ids = assign105970_e158357;
        var_ids_dn0 = assign105970_e158357_d_n0;
        var_ids_dn2 = assign105970_e158357_d_n2;
        var_ids_dn4 = assign105970_e158357_d_n4;
        var_ids_dn5 = assign105970_e158357_d_n5;
        var_ids_dn6 = assign105970_e158357_d_n6;
        var_ids_dn7 = assign105970_e158357_d_n7;
        var_ids_dn8 = assign105970_e158357_d_n8;
        var_ids_dn9 = assign105970_e158357_d_n9;
        var_ids_dn10 = assign105970_e158357_d_n10;
        var_ids_dn11 = assign105970_e158357_d_n11;
        var_ids_dn14 = assign105970_e158357_d_n14;

        let (assign105980_e158361, assign105980_e158361_d_n0, assign105980_e158361_d_n2, assign105980_e158361_d_n4, assign105980_e158361_d_n5, assign105980_e158361_d_n6, assign105980_e158361_d_n7, assign105980_e158361_d_n8, assign105980_e158361_d_n9, assign105980_e158361_d_n10, assign105980_e158361_d_n11, assign105980_e158361_d_n14,) = {
    if (var_guard2403 != 0.0) {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn4, var_qde_dn5, var_qde_dn6, var_qde_dn7, var_qde_dn8, var_qde_dn9, var_qde_dn10, var_qde_dn11, var_qde_dn14,)
    } else {
        (var_qd, var_qd_dn0, var_qd_dn2, var_qd_dn4, var_qd_dn5, var_qd_dn6, var_qd_dn7, var_qd_dn8, var_qd_dn9, var_qd_dn10, var_qd_dn11, var_qd_dn14,)
    }
};
        var_qd = assign105980_e158361;
        var_qd_dn0 = assign105980_e158361_d_n0;
        var_qd_dn2 = assign105980_e158361_d_n2;
        var_qd_dn4 = assign105980_e158361_d_n4;
        var_qd_dn5 = assign105980_e158361_d_n5;
        var_qd_dn6 = assign105980_e158361_d_n6;
        var_qd_dn7 = assign105980_e158361_d_n7;
        var_qd_dn8 = assign105980_e158361_d_n8;
        var_qd_dn9 = assign105980_e158361_d_n9;
        var_qd_dn10 = assign105980_e158361_d_n10;
        var_qd_dn11 = assign105980_e158361_d_n11;
        var_qd_dn14 = assign105980_e158361_d_n14;

        let (assign105990_e158365, assign105990_e158365_d_n0, assign105990_e158365_d_n2, assign105990_e158365_d_n4, assign105990_e158365_d_n5, assign105990_e158365_d_n6, assign105990_e158365_d_n7, assign105990_e158365_d_n8, assign105990_e158365_d_n9, assign105990_e158365_d_n10, assign105990_e158365_d_n11, assign105990_e158365_d_n14,) = {
    if (var_guard2403 != 0.0) {
        (var_qge, var_qge_dn0, var_qge_dn2, var_qge_dn4, var_qge_dn5, var_qge_dn6, var_qge_dn7, var_qge_dn8, var_qge_dn9, var_qge_dn10, var_qge_dn11, var_qge_dn14,)
    } else {
        (var_qg, var_qg_dn0, var_qg_dn2, var_qg_dn4, var_qg_dn5, var_qg_dn6, var_qg_dn7, var_qg_dn8, var_qg_dn9, var_qg_dn10, var_qg_dn11, var_qg_dn14,)
    }
};
        var_qg = assign105990_e158365;
        var_qg_dn0 = assign105990_e158365_d_n0;
        var_qg_dn2 = assign105990_e158365_d_n2;
        var_qg_dn4 = assign105990_e158365_d_n4;
        var_qg_dn5 = assign105990_e158365_d_n5;
        var_qg_dn6 = assign105990_e158365_d_n6;
        var_qg_dn7 = assign105990_e158365_d_n7;
        var_qg_dn8 = assign105990_e158365_d_n8;
        var_qg_dn9 = assign105990_e158365_d_n9;
        var_qg_dn10 = assign105990_e158365_d_n10;
        var_qg_dn11 = assign105990_e158365_d_n11;
        var_qg_dn14 = assign105990_e158365_d_n14;

        let (assign106000_e158369, assign106000_e158369_d_n0, assign106000_e158369_d_n2, assign106000_e158369_d_n4, assign106000_e158369_d_n5, assign106000_e158369_d_n6, assign106000_e158369_d_n7, assign106000_e158369_d_n8, assign106000_e158369_d_n9, assign106000_e158369_d_n10, assign106000_e158369_d_n11, assign106000_e158369_d_n14,) = {
    if (var_guard2403 != 0.0) {
        (var_qse, var_qse_dn0, var_qse_dn2, var_qse_dn4, var_qse_dn5, var_qse_dn6, var_qse_dn7, var_qse_dn8, var_qse_dn9, var_qse_dn10, var_qse_dn11, var_qse_dn14,)
    } else {
        (var_qs, var_qs_dn0, var_qs_dn2, var_qs_dn4, var_qs_dn5, var_qs_dn6, var_qs_dn7, var_qs_dn8, var_qs_dn9, var_qs_dn10, var_qs_dn11, var_qs_dn14,)
    }
};
        var_qs = assign106000_e158369;
        var_qs_dn0 = assign106000_e158369_d_n0;
        var_qs_dn2 = assign106000_e158369_d_n2;
        var_qs_dn4 = assign106000_e158369_d_n4;
        var_qs_dn5 = assign106000_e158369_d_n5;
        var_qs_dn6 = assign106000_e158369_d_n6;
        var_qs_dn7 = assign106000_e158369_d_n7;
        var_qs_dn8 = assign106000_e158369_d_n8;
        var_qs_dn9 = assign106000_e158369_d_n9;
        var_qs_dn10 = assign106000_e158369_d_n10;
        var_qs_dn11 = assign106000_e158369_d_n11;
        var_qs_dn14 = assign106000_e158369_d_n14;

        let (assign106010_e158378, assign106010_e158378_d_n0, assign106010_e158378_d_n2, assign106010_e158378_d_n4, assign106010_e158378_d_n5, assign106010_e158378_d_n6, assign106010_e158378_d_n7, assign106010_e158378_d_n8, assign106010_e158378_d_n9, assign106010_e158378_d_n10, assign106010_e158378_d_n11, assign106010_e158378_d_n14,) = {
    if (var_guard2403 != 0.0) {
        let assign106010_e158373: f64 = (var_qge + var_qde);
        let assign106010_e158375: f64 = (assign106010_e158373 + var_qse);
        let assign106010_e158376: f64 = (-assign106010_e158375);
        (assign106010_e158376, (-((var_qge_dn0 + var_qde_dn0) + var_qse_dn0)), (-((var_qge_dn2 + var_qde_dn2) + var_qse_dn2)), (-((var_qge_dn4 + var_qde_dn4) + var_qse_dn4)), (-((var_qge_dn5 + var_qde_dn5) + var_qse_dn5)), (-((var_qge_dn6 + var_qde_dn6) + var_qse_dn6)), (-((var_qge_dn7 + var_qde_dn7) + var_qse_dn7)), (-((var_qge_dn8 + var_qde_dn8) + var_qse_dn8)), (-((var_qge_dn9 + var_qde_dn9) + var_qse_dn9)), (-((var_qge_dn10 + var_qde_dn10) + var_qse_dn10)), (-((var_qge_dn11 + var_qde_dn11) + var_qse_dn11)), (-((var_qge_dn14 + var_qde_dn14) + var_qse_dn14)),)
    } else {
        (var_qb, var_qb_dn0, var_qb_dn2, var_qb_dn4, var_qb_dn5, var_qb_dn6, var_qb_dn7, var_qb_dn8, var_qb_dn9, var_qb_dn10, var_qb_dn11, var_qb_dn14,)
    }
};
        var_qb = assign106010_e158378;
        var_qb_dn0 = assign106010_e158378_d_n0;
        var_qb_dn2 = assign106010_e158378_d_n2;
        var_qb_dn4 = assign106010_e158378_d_n4;
        var_qb_dn5 = assign106010_e158378_d_n5;
        var_qb_dn6 = assign106010_e158378_d_n6;
        var_qb_dn7 = assign106010_e158378_d_n7;
        var_qb_dn8 = assign106010_e158378_d_n8;
        var_qb_dn9 = assign106010_e158378_d_n9;
        var_qb_dn10 = assign106010_e158378_d_n10;
        var_qb_dn11 = assign106010_e158378_d_n11;
        var_qb_dn14 = assign106010_e158378_d_n14;

        *var_ddriftld_slot = var_ddriftld;
        *var_ddriftld_dn0_slot = var_ddriftld_dn0;
        *var_ddriftld_dn10_slot = var_ddriftld_dn10;
        *var_ddriftld_dn11_slot = var_ddriftld_dn11;
        *var_ddriftld_dn14_slot = var_ddriftld_dn14;
        *var_ddriftld_dn2_slot = var_ddriftld_dn2;
        *var_ddriftld_dn4_slot = var_ddriftld_dn4;
        *var_ddriftld_dn5_slot = var_ddriftld_dn5;
        *var_ddriftld_dn6_slot = var_ddriftld_dn6;
        *var_ddriftld_dn7_slot = var_ddriftld_dn7;
        *var_ddriftld_dn8_slot = var_ddriftld_dn8;
        *var_ddriftld_dn9_slot = var_ddriftld_dn9;
        *var_guard2397_slot = var_guard2397;
        *var_guard2399_slot = var_guard2399;
        *var_guard2400_slot = var_guard2400;
        *var_guard2401_slot = var_guard2401;
        *var_guard2402_slot = var_guard2402;
        *var_guard2403_slot = var_guard2403;
        *var_ids_slot = var_ids;
        *var_ids_dn0_slot = var_ids_dn0;
        *var_ids_dn10_slot = var_ids_dn10;
        *var_ids_dn11_slot = var_ids_dn11;
        *var_ids_dn14_slot = var_ids_dn14;
        *var_ids_dn2_slot = var_ids_dn2;
        *var_ids_dn4_slot = var_ids_dn4;
        *var_ids_dn5_slot = var_ids_dn5;
        *var_ids_dn6_slot = var_ids_dn6;
        *var_ids_dn7_slot = var_ids_dn7;
        *var_ids_dn8_slot = var_ids_dn8;
        *var_ids_dn9_slot = var_ids_dn9;
        *var_igb_slot = var_igb;
        *var_igb_dn0_slot = var_igb_dn0;
        *var_igb_dn10_slot = var_igb_dn10;
        *var_igb_dn11_slot = var_igb_dn11;
        *var_igb_dn14_slot = var_igb_dn14;
        *var_igb_dn2_slot = var_igb_dn2;
        *var_igb_dn4_slot = var_igb_dn4;
        *var_igb_dn5_slot = var_igb_dn5;
        *var_igb_dn6_slot = var_igb_dn6;
        *var_igb_dn7_slot = var_igb_dn7;
        *var_igb_dn8_slot = var_igb_dn8;
        *var_igb_dn9_slot = var_igb_dn9;
        *var_igd_slot = var_igd;
        *var_igd_dn0_slot = var_igd_dn0;
        *var_igd_dn10_slot = var_igd_dn10;
        *var_igd_dn11_slot = var_igd_dn11;
        *var_igd_dn14_slot = var_igd_dn14;
        *var_igd_dn2_slot = var_igd_dn2;
        *var_igd_dn4_slot = var_igd_dn4;
        *var_igd_dn5_slot = var_igd_dn5;
        *var_igd_dn6_slot = var_igd_dn6;
        *var_igd_dn7_slot = var_igd_dn7;
        *var_igd_dn8_slot = var_igd_dn8;
        *var_igd_dn9_slot = var_igd_dn9;
        *var_igs_slot = var_igs;
        *var_igs_dn0_slot = var_igs_dn0;
        *var_igs_dn10_slot = var_igs_dn10;
        *var_igs_dn11_slot = var_igs_dn11;
        *var_igs_dn14_slot = var_igs_dn14;
        *var_igs_dn2_slot = var_igs_dn2;
        *var_igs_dn4_slot = var_igs_dn4;
        *var_igs_dn5_slot = var_igs_dn5;
        *var_igs_dn6_slot = var_igs_dn6;
        *var_igs_dn7_slot = var_igs_dn7;
        *var_igs_dn8_slot = var_igs_dn8;
        *var_igs_dn9_slot = var_igs_dn9;
        *var_qb_slot = var_qb;
        *var_qb_dn0_slot = var_qb_dn0;
        *var_qb_dn10_slot = var_qb_dn10;
        *var_qb_dn11_slot = var_qb_dn11;
        *var_qb_dn14_slot = var_qb_dn14;
        *var_qb_dn2_slot = var_qb_dn2;
        *var_qb_dn4_slot = var_qb_dn4;
        *var_qb_dn5_slot = var_qb_dn5;
        *var_qb_dn6_slot = var_qb_dn6;
        *var_qb_dn7_slot = var_qb_dn7;
        *var_qb_dn8_slot = var_qb_dn8;
        *var_qb_dn9_slot = var_qb_dn9;
        *var_qd_slot = var_qd;
        *var_qd_dn0_slot = var_qd_dn0;
        *var_qd_dn10_slot = var_qd_dn10;
        *var_qd_dn11_slot = var_qd_dn11;
        *var_qd_dn14_slot = var_qd_dn14;
        *var_qd_dn2_slot = var_qd_dn2;
        *var_qd_dn4_slot = var_qd_dn4;
        *var_qd_dn5_slot = var_qd_dn5;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn7_slot = var_qd_dn7;
        *var_qd_dn8_slot = var_qd_dn8;
        *var_qd_dn9_slot = var_qd_dn9;
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
        *var_rdd_slot = var_rdd;
        *var_rdd_dn0_slot = var_rdd_dn0;
        *var_rdd_dn10_slot = var_rdd_dn10;
        *var_rdd_dn11_slot = var_rdd_dn11;
        *var_rdd_dn14_slot = var_rdd_dn14;
        *var_rdd_dn2_slot = var_rdd_dn2;
        *var_rdd_dn4_slot = var_rdd_dn4;
        *var_rdd_dn5_slot = var_rdd_dn5;
        *var_rdd_dn6_slot = var_rdd_dn6;
        *var_rdd_dn7_slot = var_rdd_dn7;
        *var_rdd_dn8_slot = var_rdd_dn8;
        *var_rdd_dn9_slot = var_rdd_dn9;
        *var_rdde_slot = var_rdde;
        *var_rdde_dn0_slot = var_rdde_dn0;
        *var_rdde_dn10_slot = var_rdde_dn10;
        *var_rdde_dn11_slot = var_rdde_dn11;
        *var_rdde_dn14_slot = var_rdde_dn14;
        *var_rdde_dn2_slot = var_rdde_dn2;
        *var_rdde_dn4_slot = var_rdde_dn4;
        *var_rdde_dn5_slot = var_rdde_dn5;
        *var_rdde_dn6_slot = var_rdde_dn6;
        *var_rdde_dn7_slot = var_rdde_dn7;
        *var_rdde_dn8_slot = var_rdde_dn8;
        *var_rdde_dn9_slot = var_rdde_dn9;
        *var_rsd_slot = var_rsd;
        *var_rsd_dn0_slot = var_rsd_dn0;
        *var_rsd_dn10_slot = var_rsd_dn10;
        *var_rsd_dn11_slot = var_rsd_dn11;
        *var_rsd_dn14_slot = var_rsd_dn14;
        *var_rsd_dn2_slot = var_rsd_dn2;
        *var_rsd_dn4_slot = var_rsd_dn4;
        *var_rsd_dn5_slot = var_rsd_dn5;
        *var_rsd_dn6_slot = var_rsd_dn6;
        *var_rsd_dn7_slot = var_rsd_dn7;
        *var_rsd_dn8_slot = var_rsd_dn8;
        *var_rsd_dn9_slot = var_rsd_dn9;
        *var_rsde_slot = var_rsde;
        *var_rsde_dn0_slot = var_rsde_dn0;
        *var_rsde_dn10_slot = var_rsde_dn10;
        *var_rsde_dn11_slot = var_rsde_dn11;
        *var_rsde_dn14_slot = var_rsde_dn14;
        *var_rsde_dn2_slot = var_rsde_dn2;
        *var_rsde_dn4_slot = var_rsde_dn4;
        *var_rsde_dn5_slot = var_rsde_dn5;
        *var_rsde_dn6_slot = var_rsde_dn6;
        *var_rsde_dn7_slot = var_rsde_dn7;
        *var_rsde_dn8_slot = var_rsde_dn8;
        *var_rsde_dn9_slot = var_rsde_dn9;
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
    }

    pub(super) fn stamp_transient_block_389(
        var_flg_nqs: f64,
        var_guard2403: f64,
        var_ibjte: f64,
        var_ibjte_dn0: f64,
        var_ibjte_dn10: f64,
        var_ibjte_dn11: f64,
        var_ibjte_dn14: f64,
        var_ibjte_dn2: f64,
        var_ibjte_dn4: f64,
        var_ibjte_dn5: f64,
        var_ibjte_dn6: f64,
        var_ibjte_dn7: f64,
        var_ibjte_dn8: f64,
        var_ibjte_dn9: f64,
        var_ibreake: f64,
        var_ibreake_dn0: f64,
        var_ibreake_dn10: f64,
        var_ibreake_dn11: f64,
        var_ibreake_dn14: f64,
        var_ibreake_dn2: f64,
        var_ibreake_dn4: f64,
        var_ibreake_dn5: f64,
        var_ibreake_dn6: f64,
        var_ibreake_dn7: f64,
        var_ibreake_dn8: f64,
        var_ibreake_dn9: f64,
        var_idse: f64,
        var_idse_dn0: f64,
        var_idse_dn10: f64,
        var_idse_dn11: f64,
        var_idse_dn14: f64,
        var_idse_dn2: f64,
        var_idse_dn4: f64,
        var_idse_dn5: f64,
        var_idse_dn6: f64,
        var_idse_dn7: f64,
        var_idse_dn8: f64,
        var_idse_dn9: f64,
        var_idsibpce: f64,
        var_idsibpce_dn0: f64,
        var_idsibpce_dn10: f64,
        var_idsibpce_dn11: f64,
        var_idsibpce_dn14: f64,
        var_idsibpce_dn2: f64,
        var_idsibpce_dn4: f64,
        var_idsibpce_dn5: f64,
        var_idsibpce_dn6: f64,
        var_idsibpce_dn7: f64,
        var_idsibpce_dn8: f64,
        var_idsibpce_dn9: f64,
        var_igidle: f64,
        var_igidle_dn0: f64,
        var_igidle_dn10: f64,
        var_igidle_dn11: f64,
        var_igidle_dn14: f64,
        var_igidle_dn2: f64,
        var_igidle_dn4: f64,
        var_igidle_dn5: f64,
        var_igidle_dn6: f64,
        var_igidle_dn7: f64,
        var_igidle_dn8: f64,
        var_igidle_dn9: f64,
        var_igisle: f64,
        var_igisle_dn0: f64,
        var_igisle_dn10: f64,
        var_igisle_dn11: f64,
        var_igisle_dn14: f64,
        var_igisle_dn2: f64,
        var_igisle_dn4: f64,
        var_igisle_dn5: f64,
        var_igisle_dn6: f64,
        var_igisle_dn7: f64,
        var_igisle_dn8: f64,
        var_igisle_dn9: f64,
        var_isube: f64,
        var_isube_dn0: f64,
        var_isube_dn10: f64,
        var_isube_dn11: f64,
        var_isube_dn14: f64,
        var_isube_dn2: f64,
        var_isube_dn4: f64,
        var_isube_dn5: f64,
        var_isube_dn6: f64,
        var_isube_dn7: f64,
        var_isube_dn8: f64,
        var_isube_dn9: f64,
        var_isublde: f64,
        var_isublde_dn0: f64,
        var_isublde_dn10: f64,
        var_isublde_dn11: f64,
        var_isublde_dn14: f64,
        var_isublde_dn2: f64,
        var_isublde_dn4: f64,
        var_isublde_dn5: f64,
        var_isublde_dn6: f64,
        var_isublde_dn7: f64,
        var_isublde_dn8: f64,
        var_isublde_dn9: f64,
        var_qde: f64,
        var_qde_dn0: f64,
        var_qde_dn10: f64,
        var_qde_dn11: f64,
        var_qde_dn14: f64,
        var_qde_dn2: f64,
        var_qde_dn4: f64,
        var_qde_dn5: f64,
        var_qde_dn6: f64,
        var_qde_dn7: f64,
        var_qde_dn8: f64,
        var_qde_dn9: f64,
        var_qge: f64,
        var_qge_dn0: f64,
        var_qge_dn10: f64,
        var_qge_dn11: f64,
        var_qge_dn14: f64,
        var_qge_dn2: f64,
        var_qge_dn4: f64,
        var_qge_dn5: f64,
        var_qge_dn6: f64,
        var_qge_dn7: f64,
        var_qge_dn8: f64,
        var_qge_dn9: f64,
        var_qse: f64,
        var_qse_dn0: f64,
        var_qse_dn10: f64,
        var_qse_dn11: f64,
        var_qse_dn14: f64,
        var_qse_dn2: f64,
        var_qse_dn4: f64,
        var_qse_dn5: f64,
        var_qse_dn6: f64,
        var_qse_dn7: f64,
        var_qse_dn8: f64,
        var_qse_dn9: f64,
        var_xd: f64,
        var_xd_dn0: f64,
        var_xd_dn10: f64,
        var_xd_dn11: f64,
        var_xd_dn14: f64,
        var_xd_dn2: f64,
        var_xd_dn4: f64,
        var_xd_dn5: f64,
        var_xd_dn6: f64,
        var_xd_dn7: f64,
        var_xd_dn8: f64,
        var_xd_dn9: f64,
        var_ibjt_slot: &mut f64,
        var_ibjt_dn0_slot: &mut f64,
        var_ibjt_dn10_slot: &mut f64,
        var_ibjt_dn11_slot: &mut f64,
        var_ibjt_dn14_slot: &mut f64,
        var_ibjt_dn2_slot: &mut f64,
        var_ibjt_dn4_slot: &mut f64,
        var_ibjt_dn5_slot: &mut f64,
        var_ibjt_dn6_slot: &mut f64,
        var_ibjt_dn7_slot: &mut f64,
        var_ibjt_dn8_slot: &mut f64,
        var_ibjt_dn9_slot: &mut f64,
        var_ibjts_slot: &mut f64,
        var_ibjts_dn0_slot: &mut f64,
        var_ibjts_dn10_slot: &mut f64,
        var_ibjts_dn11_slot: &mut f64,
        var_ibjts_dn14_slot: &mut f64,
        var_ibjts_dn2_slot: &mut f64,
        var_ibjts_dn4_slot: &mut f64,
        var_ibjts_dn5_slot: &mut f64,
        var_ibjts_dn6_slot: &mut f64,
        var_ibjts_dn7_slot: &mut f64,
        var_ibjts_dn8_slot: &mut f64,
        var_ibjts_dn9_slot: &mut f64,
        var_ibreak_slot: &mut f64,
        var_ibreak_dn0_slot: &mut f64,
        var_ibreak_dn10_slot: &mut f64,
        var_ibreak_dn11_slot: &mut f64,
        var_ibreak_dn14_slot: &mut f64,
        var_ibreak_dn2_slot: &mut f64,
        var_ibreak_dn4_slot: &mut f64,
        var_ibreak_dn5_slot: &mut f64,
        var_ibreak_dn6_slot: &mut f64,
        var_ibreak_dn7_slot: &mut f64,
        var_ibreak_dn8_slot: &mut f64,
        var_ibreak_dn9_slot: &mut f64,
        var_ibreaks_slot: &mut f64,
        var_ibreaks_dn0_slot: &mut f64,
        var_ibreaks_dn10_slot: &mut f64,
        var_ibreaks_dn11_slot: &mut f64,
        var_ibreaks_dn14_slot: &mut f64,
        var_ibreaks_dn2_slot: &mut f64,
        var_ibreaks_dn4_slot: &mut f64,
        var_ibreaks_dn5_slot: &mut f64,
        var_ibreaks_dn6_slot: &mut f64,
        var_ibreaks_dn7_slot: &mut f64,
        var_ibreaks_dn8_slot: &mut f64,
        var_ibreaks_dn9_slot: &mut f64,
        var_ids_slot: &mut f64,
        var_ids_dn0_slot: &mut f64,
        var_ids_dn10_slot: &mut f64,
        var_ids_dn11_slot: &mut f64,
        var_ids_dn14_slot: &mut f64,
        var_ids_dn2_slot: &mut f64,
        var_ids_dn4_slot: &mut f64,
        var_ids_dn5_slot: &mut f64,
        var_ids_dn6_slot: &mut f64,
        var_ids_dn7_slot: &mut f64,
        var_ids_dn8_slot: &mut f64,
        var_ids_dn9_slot: &mut f64,
        var_idsibpc_slot: &mut f64,
        var_idsibpc_dn0_slot: &mut f64,
        var_idsibpc_dn10_slot: &mut f64,
        var_idsibpc_dn11_slot: &mut f64,
        var_idsibpc_dn14_slot: &mut f64,
        var_idsibpc_dn2_slot: &mut f64,
        var_idsibpc_dn4_slot: &mut f64,
        var_idsibpc_dn5_slot: &mut f64,
        var_idsibpc_dn6_slot: &mut f64,
        var_idsibpc_dn7_slot: &mut f64,
        var_idsibpc_dn8_slot: &mut f64,
        var_idsibpc_dn9_slot: &mut f64,
        var_idsibpcs_slot: &mut f64,
        var_idsibpcs_dn0_slot: &mut f64,
        var_idsibpcs_dn10_slot: &mut f64,
        var_idsibpcs_dn11_slot: &mut f64,
        var_idsibpcs_dn14_slot: &mut f64,
        var_idsibpcs_dn2_slot: &mut f64,
        var_idsibpcs_dn4_slot: &mut f64,
        var_idsibpcs_dn5_slot: &mut f64,
        var_idsibpcs_dn6_slot: &mut f64,
        var_idsibpcs_dn7_slot: &mut f64,
        var_idsibpcs_dn8_slot: &mut f64,
        var_idsibpcs_dn9_slot: &mut f64,
        var_igidl_slot: &mut f64,
        var_igidl_dn0_slot: &mut f64,
        var_igidl_dn10_slot: &mut f64,
        var_igidl_dn11_slot: &mut f64,
        var_igidl_dn14_slot: &mut f64,
        var_igidl_dn2_slot: &mut f64,
        var_igidl_dn4_slot: &mut f64,
        var_igidl_dn5_slot: &mut f64,
        var_igidl_dn6_slot: &mut f64,
        var_igidl_dn7_slot: &mut f64,
        var_igidl_dn8_slot: &mut f64,
        var_igidl_dn9_slot: &mut f64,
        var_igisl_slot: &mut f64,
        var_igisl_dn0_slot: &mut f64,
        var_igisl_dn10_slot: &mut f64,
        var_igisl_dn11_slot: &mut f64,
        var_igisl_dn14_slot: &mut f64,
        var_igisl_dn2_slot: &mut f64,
        var_igisl_dn4_slot: &mut f64,
        var_igisl_dn5_slot: &mut f64,
        var_igisl_dn6_slot: &mut f64,
        var_igisl_dn7_slot: &mut f64,
        var_igisl_dn8_slot: &mut f64,
        var_igisl_dn9_slot: &mut f64,
        var_isub_slot: &mut f64,
        var_isub_dn0_slot: &mut f64,
        var_isub_dn10_slot: &mut f64,
        var_isub_dn11_slot: &mut f64,
        var_isub_dn14_slot: &mut f64,
        var_isub_dn2_slot: &mut f64,
        var_isub_dn4_slot: &mut f64,
        var_isub_dn5_slot: &mut f64,
        var_isub_dn6_slot: &mut f64,
        var_isub_dn7_slot: &mut f64,
        var_isub_dn8_slot: &mut f64,
        var_isub_dn9_slot: &mut f64,
        var_isubld_slot: &mut f64,
        var_isubld_dn0_slot: &mut f64,
        var_isubld_dn10_slot: &mut f64,
        var_isubld_dn11_slot: &mut f64,
        var_isubld_dn14_slot: &mut f64,
        var_isubld_dn2_slot: &mut f64,
        var_isubld_dn4_slot: &mut f64,
        var_isubld_dn5_slot: &mut f64,
        var_isubld_dn6_slot: &mut f64,
        var_isubld_dn7_slot: &mut f64,
        var_isubld_dn8_slot: &mut f64,
        var_isubld_dn9_slot: &mut f64,
        var_isublds_slot: &mut f64,
        var_isublds_dn0_slot: &mut f64,
        var_isublds_dn10_slot: &mut f64,
        var_isublds_dn11_slot: &mut f64,
        var_isublds_dn14_slot: &mut f64,
        var_isublds_dn2_slot: &mut f64,
        var_isublds_dn4_slot: &mut f64,
        var_isublds_dn5_slot: &mut f64,
        var_isublds_dn6_slot: &mut f64,
        var_isublds_dn7_slot: &mut f64,
        var_isublds_dn8_slot: &mut f64,
        var_isublds_dn9_slot: &mut f64,
        var_isubs_slot: &mut f64,
        var_isubs_dn0_slot: &mut f64,
        var_isubs_dn10_slot: &mut f64,
        var_isubs_dn11_slot: &mut f64,
        var_isubs_dn14_slot: &mut f64,
        var_isubs_dn2_slot: &mut f64,
        var_isubs_dn4_slot: &mut f64,
        var_isubs_dn5_slot: &mut f64,
        var_isubs_dn6_slot: &mut f64,
        var_isubs_dn7_slot: &mut f64,
        var_isubs_dn8_slot: &mut f64,
        var_isubs_dn9_slot: &mut f64,
        var_qb_slot: &mut f64,
        var_qb_dn0_slot: &mut f64,
        var_qb_dn10_slot: &mut f64,
        var_qb_dn11_slot: &mut f64,
        var_qb_dn14_slot: &mut f64,
        var_qb_dn2_slot: &mut f64,
        var_qb_dn4_slot: &mut f64,
        var_qb_dn5_slot: &mut f64,
        var_qb_dn6_slot: &mut f64,
        var_qb_dn7_slot: &mut f64,
        var_qb_dn8_slot: &mut f64,
        var_qb_dn9_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn0_slot: &mut f64,
        var_qd_dn10_slot: &mut f64,
        var_qd_dn11_slot: &mut f64,
        var_qd_dn14_slot: &mut f64,
        var_qd_dn2_slot: &mut f64,
        var_qd_dn4_slot: &mut f64,
        var_qd_dn5_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn7_slot: &mut f64,
        var_qd_dn8_slot: &mut f64,
        var_qd_dn9_slot: &mut f64,
        var_qdrat_slot: &mut f64,
        var_qdrat_dn0_slot: &mut f64,
        var_qdrat_dn10_slot: &mut f64,
        var_qdrat_dn11_slot: &mut f64,
        var_qdrat_dn14_slot: &mut f64,
        var_qdrat_dn2_slot: &mut f64,
        var_qdrat_dn4_slot: &mut f64,
        var_qdrat_dn5_slot: &mut f64,
        var_qdrat_dn6_slot: &mut f64,
        var_qdrat_dn7_slot: &mut f64,
        var_qdrat_dn8_slot: &mut f64,
        var_qdrat_dn9_slot: &mut f64,
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
    ) {
        let mut var_ibjt: f64 = *var_ibjt_slot;
        let mut var_ibjt_dn0: f64 = *var_ibjt_dn0_slot;
        let mut var_ibjt_dn10: f64 = *var_ibjt_dn10_slot;
        let mut var_ibjt_dn11: f64 = *var_ibjt_dn11_slot;
        let mut var_ibjt_dn14: f64 = *var_ibjt_dn14_slot;
        let mut var_ibjt_dn2: f64 = *var_ibjt_dn2_slot;
        let mut var_ibjt_dn4: f64 = *var_ibjt_dn4_slot;
        let mut var_ibjt_dn5: f64 = *var_ibjt_dn5_slot;
        let mut var_ibjt_dn6: f64 = *var_ibjt_dn6_slot;
        let mut var_ibjt_dn7: f64 = *var_ibjt_dn7_slot;
        let mut var_ibjt_dn8: f64 = *var_ibjt_dn8_slot;
        let mut var_ibjt_dn9: f64 = *var_ibjt_dn9_slot;
        let mut var_ibjts: f64 = *var_ibjts_slot;
        let mut var_ibjts_dn0: f64 = *var_ibjts_dn0_slot;
        let mut var_ibjts_dn10: f64 = *var_ibjts_dn10_slot;
        let mut var_ibjts_dn11: f64 = *var_ibjts_dn11_slot;
        let mut var_ibjts_dn14: f64 = *var_ibjts_dn14_slot;
        let mut var_ibjts_dn2: f64 = *var_ibjts_dn2_slot;
        let mut var_ibjts_dn4: f64 = *var_ibjts_dn4_slot;
        let mut var_ibjts_dn5: f64 = *var_ibjts_dn5_slot;
        let mut var_ibjts_dn6: f64 = *var_ibjts_dn6_slot;
        let mut var_ibjts_dn7: f64 = *var_ibjts_dn7_slot;
        let mut var_ibjts_dn8: f64 = *var_ibjts_dn8_slot;
        let mut var_ibjts_dn9: f64 = *var_ibjts_dn9_slot;
        let mut var_ibreak: f64 = *var_ibreak_slot;
        let mut var_ibreak_dn0: f64 = *var_ibreak_dn0_slot;
        let mut var_ibreak_dn10: f64 = *var_ibreak_dn10_slot;
        let mut var_ibreak_dn11: f64 = *var_ibreak_dn11_slot;
        let mut var_ibreak_dn14: f64 = *var_ibreak_dn14_slot;
        let mut var_ibreak_dn2: f64 = *var_ibreak_dn2_slot;
        let mut var_ibreak_dn4: f64 = *var_ibreak_dn4_slot;
        let mut var_ibreak_dn5: f64 = *var_ibreak_dn5_slot;
        let mut var_ibreak_dn6: f64 = *var_ibreak_dn6_slot;
        let mut var_ibreak_dn7: f64 = *var_ibreak_dn7_slot;
        let mut var_ibreak_dn8: f64 = *var_ibreak_dn8_slot;
        let mut var_ibreak_dn9: f64 = *var_ibreak_dn9_slot;
        let mut var_ibreaks: f64 = *var_ibreaks_slot;
        let mut var_ibreaks_dn0: f64 = *var_ibreaks_dn0_slot;
        let mut var_ibreaks_dn10: f64 = *var_ibreaks_dn10_slot;
        let mut var_ibreaks_dn11: f64 = *var_ibreaks_dn11_slot;
        let mut var_ibreaks_dn14: f64 = *var_ibreaks_dn14_slot;
        let mut var_ibreaks_dn2: f64 = *var_ibreaks_dn2_slot;
        let mut var_ibreaks_dn4: f64 = *var_ibreaks_dn4_slot;
        let mut var_ibreaks_dn5: f64 = *var_ibreaks_dn5_slot;
        let mut var_ibreaks_dn6: f64 = *var_ibreaks_dn6_slot;
        let mut var_ibreaks_dn7: f64 = *var_ibreaks_dn7_slot;
        let mut var_ibreaks_dn8: f64 = *var_ibreaks_dn8_slot;
        let mut var_ibreaks_dn9: f64 = *var_ibreaks_dn9_slot;
        let mut var_ids: f64 = *var_ids_slot;
        let mut var_ids_dn0: f64 = *var_ids_dn0_slot;
        let mut var_ids_dn10: f64 = *var_ids_dn10_slot;
        let mut var_ids_dn11: f64 = *var_ids_dn11_slot;
        let mut var_ids_dn14: f64 = *var_ids_dn14_slot;
        let mut var_ids_dn2: f64 = *var_ids_dn2_slot;
        let mut var_ids_dn4: f64 = *var_ids_dn4_slot;
        let mut var_ids_dn5: f64 = *var_ids_dn5_slot;
        let mut var_ids_dn6: f64 = *var_ids_dn6_slot;
        let mut var_ids_dn7: f64 = *var_ids_dn7_slot;
        let mut var_ids_dn8: f64 = *var_ids_dn8_slot;
        let mut var_ids_dn9: f64 = *var_ids_dn9_slot;
        let mut var_idsibpc: f64 = *var_idsibpc_slot;
        let mut var_idsibpc_dn0: f64 = *var_idsibpc_dn0_slot;
        let mut var_idsibpc_dn10: f64 = *var_idsibpc_dn10_slot;
        let mut var_idsibpc_dn11: f64 = *var_idsibpc_dn11_slot;
        let mut var_idsibpc_dn14: f64 = *var_idsibpc_dn14_slot;
        let mut var_idsibpc_dn2: f64 = *var_idsibpc_dn2_slot;
        let mut var_idsibpc_dn4: f64 = *var_idsibpc_dn4_slot;
        let mut var_idsibpc_dn5: f64 = *var_idsibpc_dn5_slot;
        let mut var_idsibpc_dn6: f64 = *var_idsibpc_dn6_slot;
        let mut var_idsibpc_dn7: f64 = *var_idsibpc_dn7_slot;
        let mut var_idsibpc_dn8: f64 = *var_idsibpc_dn8_slot;
        let mut var_idsibpc_dn9: f64 = *var_idsibpc_dn9_slot;
        let mut var_idsibpcs: f64 = *var_idsibpcs_slot;
        let mut var_idsibpcs_dn0: f64 = *var_idsibpcs_dn0_slot;
        let mut var_idsibpcs_dn10: f64 = *var_idsibpcs_dn10_slot;
        let mut var_idsibpcs_dn11: f64 = *var_idsibpcs_dn11_slot;
        let mut var_idsibpcs_dn14: f64 = *var_idsibpcs_dn14_slot;
        let mut var_idsibpcs_dn2: f64 = *var_idsibpcs_dn2_slot;
        let mut var_idsibpcs_dn4: f64 = *var_idsibpcs_dn4_slot;
        let mut var_idsibpcs_dn5: f64 = *var_idsibpcs_dn5_slot;
        let mut var_idsibpcs_dn6: f64 = *var_idsibpcs_dn6_slot;
        let mut var_idsibpcs_dn7: f64 = *var_idsibpcs_dn7_slot;
        let mut var_idsibpcs_dn8: f64 = *var_idsibpcs_dn8_slot;
        let mut var_idsibpcs_dn9: f64 = *var_idsibpcs_dn9_slot;
        let mut var_igidl: f64 = *var_igidl_slot;
        let mut var_igidl_dn0: f64 = *var_igidl_dn0_slot;
        let mut var_igidl_dn10: f64 = *var_igidl_dn10_slot;
        let mut var_igidl_dn11: f64 = *var_igidl_dn11_slot;
        let mut var_igidl_dn14: f64 = *var_igidl_dn14_slot;
        let mut var_igidl_dn2: f64 = *var_igidl_dn2_slot;
        let mut var_igidl_dn4: f64 = *var_igidl_dn4_slot;
        let mut var_igidl_dn5: f64 = *var_igidl_dn5_slot;
        let mut var_igidl_dn6: f64 = *var_igidl_dn6_slot;
        let mut var_igidl_dn7: f64 = *var_igidl_dn7_slot;
        let mut var_igidl_dn8: f64 = *var_igidl_dn8_slot;
        let mut var_igidl_dn9: f64 = *var_igidl_dn9_slot;
        let mut var_igisl: f64 = *var_igisl_slot;
        let mut var_igisl_dn0: f64 = *var_igisl_dn0_slot;
        let mut var_igisl_dn10: f64 = *var_igisl_dn10_slot;
        let mut var_igisl_dn11: f64 = *var_igisl_dn11_slot;
        let mut var_igisl_dn14: f64 = *var_igisl_dn14_slot;
        let mut var_igisl_dn2: f64 = *var_igisl_dn2_slot;
        let mut var_igisl_dn4: f64 = *var_igisl_dn4_slot;
        let mut var_igisl_dn5: f64 = *var_igisl_dn5_slot;
        let mut var_igisl_dn6: f64 = *var_igisl_dn6_slot;
        let mut var_igisl_dn7: f64 = *var_igisl_dn7_slot;
        let mut var_igisl_dn8: f64 = *var_igisl_dn8_slot;
        let mut var_igisl_dn9: f64 = *var_igisl_dn9_slot;
        let mut var_isub: f64 = *var_isub_slot;
        let mut var_isub_dn0: f64 = *var_isub_dn0_slot;
        let mut var_isub_dn10: f64 = *var_isub_dn10_slot;
        let mut var_isub_dn11: f64 = *var_isub_dn11_slot;
        let mut var_isub_dn14: f64 = *var_isub_dn14_slot;
        let mut var_isub_dn2: f64 = *var_isub_dn2_slot;
        let mut var_isub_dn4: f64 = *var_isub_dn4_slot;
        let mut var_isub_dn5: f64 = *var_isub_dn5_slot;
        let mut var_isub_dn6: f64 = *var_isub_dn6_slot;
        let mut var_isub_dn7: f64 = *var_isub_dn7_slot;
        let mut var_isub_dn8: f64 = *var_isub_dn8_slot;
        let mut var_isub_dn9: f64 = *var_isub_dn9_slot;
        let mut var_isubld: f64 = *var_isubld_slot;
        let mut var_isubld_dn0: f64 = *var_isubld_dn0_slot;
        let mut var_isubld_dn10: f64 = *var_isubld_dn10_slot;
        let mut var_isubld_dn11: f64 = *var_isubld_dn11_slot;
        let mut var_isubld_dn14: f64 = *var_isubld_dn14_slot;
        let mut var_isubld_dn2: f64 = *var_isubld_dn2_slot;
        let mut var_isubld_dn4: f64 = *var_isubld_dn4_slot;
        let mut var_isubld_dn5: f64 = *var_isubld_dn5_slot;
        let mut var_isubld_dn6: f64 = *var_isubld_dn6_slot;
        let mut var_isubld_dn7: f64 = *var_isubld_dn7_slot;
        let mut var_isubld_dn8: f64 = *var_isubld_dn8_slot;
        let mut var_isubld_dn9: f64 = *var_isubld_dn9_slot;
        let mut var_isublds: f64 = *var_isublds_slot;
        let mut var_isublds_dn0: f64 = *var_isublds_dn0_slot;
        let mut var_isublds_dn10: f64 = *var_isublds_dn10_slot;
        let mut var_isublds_dn11: f64 = *var_isublds_dn11_slot;
        let mut var_isublds_dn14: f64 = *var_isublds_dn14_slot;
        let mut var_isublds_dn2: f64 = *var_isublds_dn2_slot;
        let mut var_isublds_dn4: f64 = *var_isublds_dn4_slot;
        let mut var_isublds_dn5: f64 = *var_isublds_dn5_slot;
        let mut var_isublds_dn6: f64 = *var_isublds_dn6_slot;
        let mut var_isublds_dn7: f64 = *var_isublds_dn7_slot;
        let mut var_isublds_dn8: f64 = *var_isublds_dn8_slot;
        let mut var_isublds_dn9: f64 = *var_isublds_dn9_slot;
        let mut var_isubs: f64 = *var_isubs_slot;
        let mut var_isubs_dn0: f64 = *var_isubs_dn0_slot;
        let mut var_isubs_dn10: f64 = *var_isubs_dn10_slot;
        let mut var_isubs_dn11: f64 = *var_isubs_dn11_slot;
        let mut var_isubs_dn14: f64 = *var_isubs_dn14_slot;
        let mut var_isubs_dn2: f64 = *var_isubs_dn2_slot;
        let mut var_isubs_dn4: f64 = *var_isubs_dn4_slot;
        let mut var_isubs_dn5: f64 = *var_isubs_dn5_slot;
        let mut var_isubs_dn6: f64 = *var_isubs_dn6_slot;
        let mut var_isubs_dn7: f64 = *var_isubs_dn7_slot;
        let mut var_isubs_dn8: f64 = *var_isubs_dn8_slot;
        let mut var_isubs_dn9: f64 = *var_isubs_dn9_slot;
        let mut var_qb: f64 = *var_qb_slot;
        let mut var_qb_dn0: f64 = *var_qb_dn0_slot;
        let mut var_qb_dn10: f64 = *var_qb_dn10_slot;
        let mut var_qb_dn11: f64 = *var_qb_dn11_slot;
        let mut var_qb_dn14: f64 = *var_qb_dn14_slot;
        let mut var_qb_dn2: f64 = *var_qb_dn2_slot;
        let mut var_qb_dn4: f64 = *var_qb_dn4_slot;
        let mut var_qb_dn5: f64 = *var_qb_dn5_slot;
        let mut var_qb_dn6: f64 = *var_qb_dn6_slot;
        let mut var_qb_dn7: f64 = *var_qb_dn7_slot;
        let mut var_qb_dn8: f64 = *var_qb_dn8_slot;
        let mut var_qb_dn9: f64 = *var_qb_dn9_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn0: f64 = *var_qd_dn0_slot;
        let mut var_qd_dn10: f64 = *var_qd_dn10_slot;
        let mut var_qd_dn11: f64 = *var_qd_dn11_slot;
        let mut var_qd_dn14: f64 = *var_qd_dn14_slot;
        let mut var_qd_dn2: f64 = *var_qd_dn2_slot;
        let mut var_qd_dn4: f64 = *var_qd_dn4_slot;
        let mut var_qd_dn5: f64 = *var_qd_dn5_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn7: f64 = *var_qd_dn7_slot;
        let mut var_qd_dn8: f64 = *var_qd_dn8_slot;
        let mut var_qd_dn9: f64 = *var_qd_dn9_slot;
        let mut var_qdrat: f64 = *var_qdrat_slot;
        let mut var_qdrat_dn0: f64 = *var_qdrat_dn0_slot;
        let mut var_qdrat_dn10: f64 = *var_qdrat_dn10_slot;
        let mut var_qdrat_dn11: f64 = *var_qdrat_dn11_slot;
        let mut var_qdrat_dn14: f64 = *var_qdrat_dn14_slot;
        let mut var_qdrat_dn2: f64 = *var_qdrat_dn2_slot;
        let mut var_qdrat_dn4: f64 = *var_qdrat_dn4_slot;
        let mut var_qdrat_dn5: f64 = *var_qdrat_dn5_slot;
        let mut var_qdrat_dn6: f64 = *var_qdrat_dn6_slot;
        let mut var_qdrat_dn7: f64 = *var_qdrat_dn7_slot;
        let mut var_qdrat_dn8: f64 = *var_qdrat_dn8_slot;
        let mut var_qdrat_dn9: f64 = *var_qdrat_dn9_slot;
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

        let (assign106020_e158382, assign106020_e158382_d_n0, assign106020_e158382_d_n2, assign106020_e158382_d_n4, assign106020_e158382_d_n5, assign106020_e158382_d_n6, assign106020_e158382_d_n7, assign106020_e158382_d_n8, assign106020_e158382_d_n9, assign106020_e158382_d_n10, assign106020_e158382_d_n11, assign106020_e158382_d_n14,) = {
    if (var_guard2403 != 0.0) {
        (var_isube, var_isube_dn0, var_isube_dn2, var_isube_dn4, var_isube_dn5, var_isube_dn6, var_isube_dn7, var_isube_dn8, var_isube_dn9, var_isube_dn10, var_isube_dn11, var_isube_dn14,)
    } else {
        (var_isub, var_isub_dn0, var_isub_dn2, var_isub_dn4, var_isub_dn5, var_isub_dn6, var_isub_dn7, var_isub_dn8, var_isub_dn9, var_isub_dn10, var_isub_dn11, var_isub_dn14,)
    }
};
        var_isub = assign106020_e158382;
        var_isub_dn0 = assign106020_e158382_d_n0;
        var_isub_dn2 = assign106020_e158382_d_n2;
        var_isub_dn4 = assign106020_e158382_d_n4;
        var_isub_dn5 = assign106020_e158382_d_n5;
        var_isub_dn6 = assign106020_e158382_d_n6;
        var_isub_dn7 = assign106020_e158382_d_n7;
        var_isub_dn8 = assign106020_e158382_d_n8;
        var_isub_dn9 = assign106020_e158382_d_n9;
        var_isub_dn10 = assign106020_e158382_d_n10;
        var_isub_dn11 = assign106020_e158382_d_n11;
        var_isub_dn14 = assign106020_e158382_d_n14;

        let (assign106030_e158386, assign106030_e158386_d_n0, assign106030_e158386_d_n2, assign106030_e158386_d_n4, assign106030_e158386_d_n5, assign106030_e158386_d_n6, assign106030_e158386_d_n7, assign106030_e158386_d_n8, assign106030_e158386_d_n9, assign106030_e158386_d_n10, assign106030_e158386_d_n11, assign106030_e158386_d_n14,) = {
    if (var_guard2403 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isubs, var_isubs_dn0, var_isubs_dn2, var_isubs_dn4, var_isubs_dn5, var_isubs_dn6, var_isubs_dn7, var_isubs_dn8, var_isubs_dn9, var_isubs_dn10, var_isubs_dn11, var_isubs_dn14,)
    }
};
        var_isubs = assign106030_e158386;
        var_isubs_dn0 = assign106030_e158386_d_n0;
        var_isubs_dn2 = assign106030_e158386_d_n2;
        var_isubs_dn4 = assign106030_e158386_d_n4;
        var_isubs_dn5 = assign106030_e158386_d_n5;
        var_isubs_dn6 = assign106030_e158386_d_n6;
        var_isubs_dn7 = assign106030_e158386_d_n7;
        var_isubs_dn8 = assign106030_e158386_d_n8;
        var_isubs_dn9 = assign106030_e158386_d_n9;
        var_isubs_dn10 = assign106030_e158386_d_n10;
        var_isubs_dn11 = assign106030_e158386_d_n11;
        var_isubs_dn14 = assign106030_e158386_d_n14;

        let (assign106040_e158390, assign106040_e158390_d_n0, assign106040_e158390_d_n2, assign106040_e158390_d_n4, assign106040_e158390_d_n5, assign106040_e158390_d_n6, assign106040_e158390_d_n7, assign106040_e158390_d_n8, assign106040_e158390_d_n9, assign106040_e158390_d_n10, assign106040_e158390_d_n11, assign106040_e158390_d_n14,) = {
    if (var_guard2403 != 0.0) {
        (var_isublde, var_isublde_dn0, var_isublde_dn2, var_isublde_dn4, var_isublde_dn5, var_isublde_dn6, var_isublde_dn7, var_isublde_dn8, var_isublde_dn9, var_isublde_dn10, var_isublde_dn11, var_isublde_dn14,)
    } else {
        (var_isubld, var_isubld_dn0, var_isubld_dn2, var_isubld_dn4, var_isubld_dn5, var_isubld_dn6, var_isubld_dn7, var_isubld_dn8, var_isubld_dn9, var_isubld_dn10, var_isubld_dn11, var_isubld_dn14,)
    }
};
        var_isubld = assign106040_e158390;
        var_isubld_dn0 = assign106040_e158390_d_n0;
        var_isubld_dn2 = assign106040_e158390_d_n2;
        var_isubld_dn4 = assign106040_e158390_d_n4;
        var_isubld_dn5 = assign106040_e158390_d_n5;
        var_isubld_dn6 = assign106040_e158390_d_n6;
        var_isubld_dn7 = assign106040_e158390_d_n7;
        var_isubld_dn8 = assign106040_e158390_d_n8;
        var_isubld_dn9 = assign106040_e158390_d_n9;
        var_isubld_dn10 = assign106040_e158390_d_n10;
        var_isubld_dn11 = assign106040_e158390_d_n11;
        var_isubld_dn14 = assign106040_e158390_d_n14;

        let (assign106050_e158394, assign106050_e158394_d_n0, assign106050_e158394_d_n2, assign106050_e158394_d_n4, assign106050_e158394_d_n5, assign106050_e158394_d_n6, assign106050_e158394_d_n7, assign106050_e158394_d_n8, assign106050_e158394_d_n9, assign106050_e158394_d_n10, assign106050_e158394_d_n11, assign106050_e158394_d_n14,) = {
    if (var_guard2403 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isublds, var_isublds_dn0, var_isublds_dn2, var_isublds_dn4, var_isublds_dn5, var_isublds_dn6, var_isublds_dn7, var_isublds_dn8, var_isublds_dn9, var_isublds_dn10, var_isublds_dn11, var_isublds_dn14,)
    }
};
        var_isublds = assign106050_e158394;
        var_isublds_dn0 = assign106050_e158394_d_n0;
        var_isublds_dn2 = assign106050_e158394_d_n2;
        var_isublds_dn4 = assign106050_e158394_d_n4;
        var_isublds_dn5 = assign106050_e158394_d_n5;
        var_isublds_dn6 = assign106050_e158394_d_n6;
        var_isublds_dn7 = assign106050_e158394_d_n7;
        var_isublds_dn8 = assign106050_e158394_d_n8;
        var_isublds_dn9 = assign106050_e158394_d_n9;
        var_isublds_dn10 = assign106050_e158394_d_n10;
        var_isublds_dn11 = assign106050_e158394_d_n11;
        var_isublds_dn14 = assign106050_e158394_d_n14;

        let (assign106060_e158398, assign106060_e158398_d_n0, assign106060_e158398_d_n2, assign106060_e158398_d_n4, assign106060_e158398_d_n5, assign106060_e158398_d_n6, assign106060_e158398_d_n7, assign106060_e158398_d_n8, assign106060_e158398_d_n9, assign106060_e158398_d_n10, assign106060_e158398_d_n11, assign106060_e158398_d_n14,) = {
    if (var_guard2403 != 0.0) {
        (var_idsibpce, var_idsibpce_dn0, var_idsibpce_dn2, var_idsibpce_dn4, var_idsibpce_dn5, var_idsibpce_dn6, var_idsibpce_dn7, var_idsibpce_dn8, var_idsibpce_dn9, var_idsibpce_dn10, var_idsibpce_dn11, var_idsibpce_dn14,)
    } else {
        (var_idsibpc, var_idsibpc_dn0, var_idsibpc_dn2, var_idsibpc_dn4, var_idsibpc_dn5, var_idsibpc_dn6, var_idsibpc_dn7, var_idsibpc_dn8, var_idsibpc_dn9, var_idsibpc_dn10, var_idsibpc_dn11, var_idsibpc_dn14,)
    }
};
        var_idsibpc = assign106060_e158398;
        var_idsibpc_dn0 = assign106060_e158398_d_n0;
        var_idsibpc_dn2 = assign106060_e158398_d_n2;
        var_idsibpc_dn4 = assign106060_e158398_d_n4;
        var_idsibpc_dn5 = assign106060_e158398_d_n5;
        var_idsibpc_dn6 = assign106060_e158398_d_n6;
        var_idsibpc_dn7 = assign106060_e158398_d_n7;
        var_idsibpc_dn8 = assign106060_e158398_d_n8;
        var_idsibpc_dn9 = assign106060_e158398_d_n9;
        var_idsibpc_dn10 = assign106060_e158398_d_n10;
        var_idsibpc_dn11 = assign106060_e158398_d_n11;
        var_idsibpc_dn14 = assign106060_e158398_d_n14;

        let (assign106070_e158402, assign106070_e158402_d_n0, assign106070_e158402_d_n2, assign106070_e158402_d_n4, assign106070_e158402_d_n5, assign106070_e158402_d_n6, assign106070_e158402_d_n7, assign106070_e158402_d_n8, assign106070_e158402_d_n9, assign106070_e158402_d_n10, assign106070_e158402_d_n11, assign106070_e158402_d_n14,) = {
    if (var_guard2403 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_idsibpcs, var_idsibpcs_dn0, var_idsibpcs_dn2, var_idsibpcs_dn4, var_idsibpcs_dn5, var_idsibpcs_dn6, var_idsibpcs_dn7, var_idsibpcs_dn8, var_idsibpcs_dn9, var_idsibpcs_dn10, var_idsibpcs_dn11, var_idsibpcs_dn14,)
    }
};
        var_idsibpcs = assign106070_e158402;
        var_idsibpcs_dn0 = assign106070_e158402_d_n0;
        var_idsibpcs_dn2 = assign106070_e158402_d_n2;
        var_idsibpcs_dn4 = assign106070_e158402_d_n4;
        var_idsibpcs_dn5 = assign106070_e158402_d_n5;
        var_idsibpcs_dn6 = assign106070_e158402_d_n6;
        var_idsibpcs_dn7 = assign106070_e158402_d_n7;
        var_idsibpcs_dn8 = assign106070_e158402_d_n8;
        var_idsibpcs_dn9 = assign106070_e158402_d_n9;
        var_idsibpcs_dn10 = assign106070_e158402_d_n10;
        var_idsibpcs_dn11 = assign106070_e158402_d_n11;
        var_idsibpcs_dn14 = assign106070_e158402_d_n14;

        let (assign106080_e158406, assign106080_e158406_d_n0, assign106080_e158406_d_n2, assign106080_e158406_d_n4, assign106080_e158406_d_n5, assign106080_e158406_d_n6, assign106080_e158406_d_n7, assign106080_e158406_d_n8, assign106080_e158406_d_n9, assign106080_e158406_d_n10, assign106080_e158406_d_n11, assign106080_e158406_d_n14,) = {
    if (var_guard2403 != 0.0) {
        (var_ibjte, var_ibjte_dn0, var_ibjte_dn2, var_ibjte_dn4, var_ibjte_dn5, var_ibjte_dn6, var_ibjte_dn7, var_ibjte_dn8, var_ibjte_dn9, var_ibjte_dn10, var_ibjte_dn11, var_ibjte_dn14,)
    } else {
        (var_ibjt, var_ibjt_dn0, var_ibjt_dn2, var_ibjt_dn4, var_ibjt_dn5, var_ibjt_dn6, var_ibjt_dn7, var_ibjt_dn8, var_ibjt_dn9, var_ibjt_dn10, var_ibjt_dn11, var_ibjt_dn14,)
    }
};
        var_ibjt = assign106080_e158406;
        var_ibjt_dn0 = assign106080_e158406_d_n0;
        var_ibjt_dn2 = assign106080_e158406_d_n2;
        var_ibjt_dn4 = assign106080_e158406_d_n4;
        var_ibjt_dn5 = assign106080_e158406_d_n5;
        var_ibjt_dn6 = assign106080_e158406_d_n6;
        var_ibjt_dn7 = assign106080_e158406_d_n7;
        var_ibjt_dn8 = assign106080_e158406_d_n8;
        var_ibjt_dn9 = assign106080_e158406_d_n9;
        var_ibjt_dn10 = assign106080_e158406_d_n10;
        var_ibjt_dn11 = assign106080_e158406_d_n11;
        var_ibjt_dn14 = assign106080_e158406_d_n14;

        let (assign106090_e158410, assign106090_e158410_d_n0, assign106090_e158410_d_n2, assign106090_e158410_d_n4, assign106090_e158410_d_n5, assign106090_e158410_d_n6, assign106090_e158410_d_n7, assign106090_e158410_d_n8, assign106090_e158410_d_n9, assign106090_e158410_d_n10, assign106090_e158410_d_n11, assign106090_e158410_d_n14,) = {
    if (var_guard2403 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibjts, var_ibjts_dn0, var_ibjts_dn2, var_ibjts_dn4, var_ibjts_dn5, var_ibjts_dn6, var_ibjts_dn7, var_ibjts_dn8, var_ibjts_dn9, var_ibjts_dn10, var_ibjts_dn11, var_ibjts_dn14,)
    }
};
        var_ibjts = assign106090_e158410;
        var_ibjts_dn0 = assign106090_e158410_d_n0;
        var_ibjts_dn2 = assign106090_e158410_d_n2;
        var_ibjts_dn4 = assign106090_e158410_d_n4;
        var_ibjts_dn5 = assign106090_e158410_d_n5;
        var_ibjts_dn6 = assign106090_e158410_d_n6;
        var_ibjts_dn7 = assign106090_e158410_d_n7;
        var_ibjts_dn8 = assign106090_e158410_d_n8;
        var_ibjts_dn9 = assign106090_e158410_d_n9;
        var_ibjts_dn10 = assign106090_e158410_d_n10;
        var_ibjts_dn11 = assign106090_e158410_d_n11;
        var_ibjts_dn14 = assign106090_e158410_d_n14;

        let (assign106100_e158414, assign106100_e158414_d_n0, assign106100_e158414_d_n2, assign106100_e158414_d_n4, assign106100_e158414_d_n5, assign106100_e158414_d_n6, assign106100_e158414_d_n7, assign106100_e158414_d_n8, assign106100_e158414_d_n9, assign106100_e158414_d_n10, assign106100_e158414_d_n11, assign106100_e158414_d_n14,) = {
    if (var_guard2403 != 0.0) {
        (var_ibreake, var_ibreake_dn0, var_ibreake_dn2, var_ibreake_dn4, var_ibreake_dn5, var_ibreake_dn6, var_ibreake_dn7, var_ibreake_dn8, var_ibreake_dn9, var_ibreake_dn10, var_ibreake_dn11, var_ibreake_dn14,)
    } else {
        (var_ibreak, var_ibreak_dn0, var_ibreak_dn2, var_ibreak_dn4, var_ibreak_dn5, var_ibreak_dn6, var_ibreak_dn7, var_ibreak_dn8, var_ibreak_dn9, var_ibreak_dn10, var_ibreak_dn11, var_ibreak_dn14,)
    }
};
        var_ibreak = assign106100_e158414;
        var_ibreak_dn0 = assign106100_e158414_d_n0;
        var_ibreak_dn2 = assign106100_e158414_d_n2;
        var_ibreak_dn4 = assign106100_e158414_d_n4;
        var_ibreak_dn5 = assign106100_e158414_d_n5;
        var_ibreak_dn6 = assign106100_e158414_d_n6;
        var_ibreak_dn7 = assign106100_e158414_d_n7;
        var_ibreak_dn8 = assign106100_e158414_d_n8;
        var_ibreak_dn9 = assign106100_e158414_d_n9;
        var_ibreak_dn10 = assign106100_e158414_d_n10;
        var_ibreak_dn11 = assign106100_e158414_d_n11;
        var_ibreak_dn14 = assign106100_e158414_d_n14;

        let (assign106110_e158418, assign106110_e158418_d_n0, assign106110_e158418_d_n2, assign106110_e158418_d_n4, assign106110_e158418_d_n5, assign106110_e158418_d_n6, assign106110_e158418_d_n7, assign106110_e158418_d_n8, assign106110_e158418_d_n9, assign106110_e158418_d_n10, assign106110_e158418_d_n11, assign106110_e158418_d_n14,) = {
    if (var_guard2403 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibreaks, var_ibreaks_dn0, var_ibreaks_dn2, var_ibreaks_dn4, var_ibreaks_dn5, var_ibreaks_dn6, var_ibreaks_dn7, var_ibreaks_dn8, var_ibreaks_dn9, var_ibreaks_dn10, var_ibreaks_dn11, var_ibreaks_dn14,)
    }
};
        var_ibreaks = assign106110_e158418;
        var_ibreaks_dn0 = assign106110_e158418_d_n0;
        var_ibreaks_dn2 = assign106110_e158418_d_n2;
        var_ibreaks_dn4 = assign106110_e158418_d_n4;
        var_ibreaks_dn5 = assign106110_e158418_d_n5;
        var_ibreaks_dn6 = assign106110_e158418_d_n6;
        var_ibreaks_dn7 = assign106110_e158418_d_n7;
        var_ibreaks_dn8 = assign106110_e158418_d_n8;
        var_ibreaks_dn9 = assign106110_e158418_d_n9;
        var_ibreaks_dn10 = assign106110_e158418_d_n10;
        var_ibreaks_dn11 = assign106110_e158418_d_n11;
        var_ibreaks_dn14 = assign106110_e158418_d_n14;

        let (assign106120_e158422, assign106120_e158422_d_n0, assign106120_e158422_d_n2, assign106120_e158422_d_n4, assign106120_e158422_d_n5, assign106120_e158422_d_n6, assign106120_e158422_d_n7, assign106120_e158422_d_n8, assign106120_e158422_d_n9, assign106120_e158422_d_n10, assign106120_e158422_d_n11, assign106120_e158422_d_n14,) = {
    if (var_guard2403 != 0.0) {
        (var_igidle, var_igidle_dn0, var_igidle_dn2, var_igidle_dn4, var_igidle_dn5, var_igidle_dn6, var_igidle_dn7, var_igidle_dn8, var_igidle_dn9, var_igidle_dn10, var_igidle_dn11, var_igidle_dn14,)
    } else {
        (var_igidl, var_igidl_dn0, var_igidl_dn2, var_igidl_dn4, var_igidl_dn5, var_igidl_dn6, var_igidl_dn7, var_igidl_dn8, var_igidl_dn9, var_igidl_dn10, var_igidl_dn11, var_igidl_dn14,)
    }
};
        var_igidl = assign106120_e158422;
        var_igidl_dn0 = assign106120_e158422_d_n0;
        var_igidl_dn2 = assign106120_e158422_d_n2;
        var_igidl_dn4 = assign106120_e158422_d_n4;
        var_igidl_dn5 = assign106120_e158422_d_n5;
        var_igidl_dn6 = assign106120_e158422_d_n6;
        var_igidl_dn7 = assign106120_e158422_d_n7;
        var_igidl_dn8 = assign106120_e158422_d_n8;
        var_igidl_dn9 = assign106120_e158422_d_n9;
        var_igidl_dn10 = assign106120_e158422_d_n10;
        var_igidl_dn11 = assign106120_e158422_d_n11;
        var_igidl_dn14 = assign106120_e158422_d_n14;

        let (assign106130_e158426, assign106130_e158426_d_n0, assign106130_e158426_d_n2, assign106130_e158426_d_n4, assign106130_e158426_d_n5, assign106130_e158426_d_n6, assign106130_e158426_d_n7, assign106130_e158426_d_n8, assign106130_e158426_d_n9, assign106130_e158426_d_n10, assign106130_e158426_d_n11, assign106130_e158426_d_n14,) = {
    if (var_guard2403 != 0.0) {
        (var_igisle, var_igisle_dn0, var_igisle_dn2, var_igisle_dn4, var_igisle_dn5, var_igisle_dn6, var_igisle_dn7, var_igisle_dn8, var_igisle_dn9, var_igisle_dn10, var_igisle_dn11, var_igisle_dn14,)
    } else {
        (var_igisl, var_igisl_dn0, var_igisl_dn2, var_igisl_dn4, var_igisl_dn5, var_igisl_dn6, var_igisl_dn7, var_igisl_dn8, var_igisl_dn9, var_igisl_dn10, var_igisl_dn11, var_igisl_dn14,)
    }
};
        var_igisl = assign106130_e158426;
        var_igisl_dn0 = assign106130_e158426_d_n0;
        var_igisl_dn2 = assign106130_e158426_d_n2;
        var_igisl_dn4 = assign106130_e158426_d_n4;
        var_igisl_dn5 = assign106130_e158426_d_n5;
        var_igisl_dn6 = assign106130_e158426_d_n6;
        var_igisl_dn7 = assign106130_e158426_d_n7;
        var_igisl_dn8 = assign106130_e158426_d_n8;
        var_igisl_dn9 = assign106130_e158426_d_n9;
        var_igisl_dn10 = assign106130_e158426_d_n10;
        var_igisl_dn11 = assign106130_e158426_d_n11;
        var_igisl_dn14 = assign106130_e158426_d_n14;

        let (assign106140_e158432, assign106140_e158432_d_n0, assign106140_e158432_d_n2, assign106140_e158432_d_n4, assign106140_e158432_d_n5, assign106140_e158432_d_n6, assign106140_e158432_d_n7, assign106140_e158432_d_n8, assign106140_e158432_d_n9, assign106140_e158432_d_n10, assign106140_e158432_d_n11, assign106140_e158432_d_n14,) = {
    if ((var_guard2403 != 0.0) && (var_flg_nqs != 0.0)) {
        (var_xd, var_xd_dn0, var_xd_dn2, var_xd_dn4, var_xd_dn5, var_xd_dn6, var_xd_dn7, var_xd_dn8, var_xd_dn9, var_xd_dn10, var_xd_dn11, var_xd_dn14,)
    } else {
        (var_qdrat, var_qdrat_dn0, var_qdrat_dn2, var_qdrat_dn4, var_qdrat_dn5, var_qdrat_dn6, var_qdrat_dn7, var_qdrat_dn8, var_qdrat_dn9, var_qdrat_dn10, var_qdrat_dn11, var_qdrat_dn14,)
    }
};
        var_qdrat = assign106140_e158432;
        var_qdrat_dn0 = assign106140_e158432_d_n0;
        var_qdrat_dn2 = assign106140_e158432_d_n2;
        var_qdrat_dn4 = assign106140_e158432_d_n4;
        var_qdrat_dn5 = assign106140_e158432_d_n5;
        var_qdrat_dn6 = assign106140_e158432_d_n6;
        var_qdrat_dn7 = assign106140_e158432_d_n7;
        var_qdrat_dn8 = assign106140_e158432_d_n8;
        var_qdrat_dn9 = assign106140_e158432_d_n9;
        var_qdrat_dn10 = assign106140_e158432_d_n10;
        var_qdrat_dn11 = assign106140_e158432_d_n11;
        var_qdrat_dn14 = assign106140_e158432_d_n14;

        let (assign106150_e158438, assign106150_e158438_d_n0, assign106150_e158438_d_n2, assign106150_e158438_d_n4, assign106150_e158438_d_n5, assign106150_e158438_d_n6, assign106150_e158438_d_n7, assign106150_e158438_d_n8, assign106150_e158438_d_n9, assign106150_e158438_d_n10, assign106150_e158438_d_n11, assign106150_e158438_d_n14,) = {
    if (var_guard2403 == 0.0) {
        let assign106150_e158436: f64 = (-var_idse);
        (assign106150_e158436, (-var_idse_dn0), (-var_idse_dn2), (-var_idse_dn4), (-var_idse_dn5), (-var_idse_dn6), (-var_idse_dn7), (-var_idse_dn8), (-var_idse_dn9), (-var_idse_dn10), (-var_idse_dn11), (-var_idse_dn14),)
    } else {
        (var_ids, var_ids_dn0, var_ids_dn2, var_ids_dn4, var_ids_dn5, var_ids_dn6, var_ids_dn7, var_ids_dn8, var_ids_dn9, var_ids_dn10, var_ids_dn11, var_ids_dn14,)
    }
};
        var_ids = assign106150_e158438;
        var_ids_dn0 = assign106150_e158438_d_n0;
        var_ids_dn2 = assign106150_e158438_d_n2;
        var_ids_dn4 = assign106150_e158438_d_n4;
        var_ids_dn5 = assign106150_e158438_d_n5;
        var_ids_dn6 = assign106150_e158438_d_n6;
        var_ids_dn7 = assign106150_e158438_d_n7;
        var_ids_dn8 = assign106150_e158438_d_n8;
        var_ids_dn9 = assign106150_e158438_d_n9;
        var_ids_dn10 = assign106150_e158438_d_n10;
        var_ids_dn11 = assign106150_e158438_d_n11;
        var_ids_dn14 = assign106150_e158438_d_n14;

        let (assign106160_e158443, assign106160_e158443_d_n0, assign106160_e158443_d_n2, assign106160_e158443_d_n4, assign106160_e158443_d_n5, assign106160_e158443_d_n6, assign106160_e158443_d_n7, assign106160_e158443_d_n8, assign106160_e158443_d_n9, assign106160_e158443_d_n10, assign106160_e158443_d_n11, assign106160_e158443_d_n14,) = {
    if (var_guard2403 == 0.0) {
        (var_qse, var_qse_dn0, var_qse_dn2, var_qse_dn4, var_qse_dn5, var_qse_dn6, var_qse_dn7, var_qse_dn8, var_qse_dn9, var_qse_dn10, var_qse_dn11, var_qse_dn14,)
    } else {
        (var_qd, var_qd_dn0, var_qd_dn2, var_qd_dn4, var_qd_dn5, var_qd_dn6, var_qd_dn7, var_qd_dn8, var_qd_dn9, var_qd_dn10, var_qd_dn11, var_qd_dn14,)
    }
};
        var_qd = assign106160_e158443;
        var_qd_dn0 = assign106160_e158443_d_n0;
        var_qd_dn2 = assign106160_e158443_d_n2;
        var_qd_dn4 = assign106160_e158443_d_n4;
        var_qd_dn5 = assign106160_e158443_d_n5;
        var_qd_dn6 = assign106160_e158443_d_n6;
        var_qd_dn7 = assign106160_e158443_d_n7;
        var_qd_dn8 = assign106160_e158443_d_n8;
        var_qd_dn9 = assign106160_e158443_d_n9;
        var_qd_dn10 = assign106160_e158443_d_n10;
        var_qd_dn11 = assign106160_e158443_d_n11;
        var_qd_dn14 = assign106160_e158443_d_n14;

        let (assign106170_e158448, assign106170_e158448_d_n0, assign106170_e158448_d_n2, assign106170_e158448_d_n4, assign106170_e158448_d_n5, assign106170_e158448_d_n6, assign106170_e158448_d_n7, assign106170_e158448_d_n8, assign106170_e158448_d_n9, assign106170_e158448_d_n10, assign106170_e158448_d_n11, assign106170_e158448_d_n14,) = {
    if (var_guard2403 == 0.0) {
        (var_qge, var_qge_dn0, var_qge_dn2, var_qge_dn4, var_qge_dn5, var_qge_dn6, var_qge_dn7, var_qge_dn8, var_qge_dn9, var_qge_dn10, var_qge_dn11, var_qge_dn14,)
    } else {
        (var_qg, var_qg_dn0, var_qg_dn2, var_qg_dn4, var_qg_dn5, var_qg_dn6, var_qg_dn7, var_qg_dn8, var_qg_dn9, var_qg_dn10, var_qg_dn11, var_qg_dn14,)
    }
};
        var_qg = assign106170_e158448;
        var_qg_dn0 = assign106170_e158448_d_n0;
        var_qg_dn2 = assign106170_e158448_d_n2;
        var_qg_dn4 = assign106170_e158448_d_n4;
        var_qg_dn5 = assign106170_e158448_d_n5;
        var_qg_dn6 = assign106170_e158448_d_n6;
        var_qg_dn7 = assign106170_e158448_d_n7;
        var_qg_dn8 = assign106170_e158448_d_n8;
        var_qg_dn9 = assign106170_e158448_d_n9;
        var_qg_dn10 = assign106170_e158448_d_n10;
        var_qg_dn11 = assign106170_e158448_d_n11;
        var_qg_dn14 = assign106170_e158448_d_n14;

        let (assign106180_e158453, assign106180_e158453_d_n0, assign106180_e158453_d_n2, assign106180_e158453_d_n4, assign106180_e158453_d_n5, assign106180_e158453_d_n6, assign106180_e158453_d_n7, assign106180_e158453_d_n8, assign106180_e158453_d_n9, assign106180_e158453_d_n10, assign106180_e158453_d_n11, assign106180_e158453_d_n14,) = {
    if (var_guard2403 == 0.0) {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn4, var_qde_dn5, var_qde_dn6, var_qde_dn7, var_qde_dn8, var_qde_dn9, var_qde_dn10, var_qde_dn11, var_qde_dn14,)
    } else {
        (var_qs, var_qs_dn0, var_qs_dn2, var_qs_dn4, var_qs_dn5, var_qs_dn6, var_qs_dn7, var_qs_dn8, var_qs_dn9, var_qs_dn10, var_qs_dn11, var_qs_dn14,)
    }
};
        var_qs = assign106180_e158453;
        var_qs_dn0 = assign106180_e158453_d_n0;
        var_qs_dn2 = assign106180_e158453_d_n2;
        var_qs_dn4 = assign106180_e158453_d_n4;
        var_qs_dn5 = assign106180_e158453_d_n5;
        var_qs_dn6 = assign106180_e158453_d_n6;
        var_qs_dn7 = assign106180_e158453_d_n7;
        var_qs_dn8 = assign106180_e158453_d_n8;
        var_qs_dn9 = assign106180_e158453_d_n9;
        var_qs_dn10 = assign106180_e158453_d_n10;
        var_qs_dn11 = assign106180_e158453_d_n11;
        var_qs_dn14 = assign106180_e158453_d_n14;

        let (assign106190_e158463, assign106190_e158463_d_n0, assign106190_e158463_d_n2, assign106190_e158463_d_n4, assign106190_e158463_d_n5, assign106190_e158463_d_n6, assign106190_e158463_d_n7, assign106190_e158463_d_n8, assign106190_e158463_d_n9, assign106190_e158463_d_n10, assign106190_e158463_d_n11, assign106190_e158463_d_n14,) = {
    if (var_guard2403 == 0.0) {
        let assign106190_e158458: f64 = (var_qge + var_qde);
        let assign106190_e158460: f64 = (assign106190_e158458 + var_qse);
        let assign106190_e158461: f64 = (-assign106190_e158460);
        (assign106190_e158461, (-((var_qge_dn0 + var_qde_dn0) + var_qse_dn0)), (-((var_qge_dn2 + var_qde_dn2) + var_qse_dn2)), (-((var_qge_dn4 + var_qde_dn4) + var_qse_dn4)), (-((var_qge_dn5 + var_qde_dn5) + var_qse_dn5)), (-((var_qge_dn6 + var_qde_dn6) + var_qse_dn6)), (-((var_qge_dn7 + var_qde_dn7) + var_qse_dn7)), (-((var_qge_dn8 + var_qde_dn8) + var_qse_dn8)), (-((var_qge_dn9 + var_qde_dn9) + var_qse_dn9)), (-((var_qge_dn10 + var_qde_dn10) + var_qse_dn10)), (-((var_qge_dn11 + var_qde_dn11) + var_qse_dn11)), (-((var_qge_dn14 + var_qde_dn14) + var_qse_dn14)),)
    } else {
        (var_qb, var_qb_dn0, var_qb_dn2, var_qb_dn4, var_qb_dn5, var_qb_dn6, var_qb_dn7, var_qb_dn8, var_qb_dn9, var_qb_dn10, var_qb_dn11, var_qb_dn14,)
    }
};
        var_qb = assign106190_e158463;
        var_qb_dn0 = assign106190_e158463_d_n0;
        var_qb_dn2 = assign106190_e158463_d_n2;
        var_qb_dn4 = assign106190_e158463_d_n4;
        var_qb_dn5 = assign106190_e158463_d_n5;
        var_qb_dn6 = assign106190_e158463_d_n6;
        var_qb_dn7 = assign106190_e158463_d_n7;
        var_qb_dn8 = assign106190_e158463_d_n8;
        var_qb_dn9 = assign106190_e158463_d_n9;
        var_qb_dn10 = assign106190_e158463_d_n10;
        var_qb_dn11 = assign106190_e158463_d_n11;
        var_qb_dn14 = assign106190_e158463_d_n14;

        let (assign106200_e158468, assign106200_e158468_d_n0, assign106200_e158468_d_n2, assign106200_e158468_d_n4, assign106200_e158468_d_n5, assign106200_e158468_d_n6, assign106200_e158468_d_n7, assign106200_e158468_d_n8, assign106200_e158468_d_n9, assign106200_e158468_d_n10, assign106200_e158468_d_n11, assign106200_e158468_d_n14,) = {
    if (var_guard2403 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isub, var_isub_dn0, var_isub_dn2, var_isub_dn4, var_isub_dn5, var_isub_dn6, var_isub_dn7, var_isub_dn8, var_isub_dn9, var_isub_dn10, var_isub_dn11, var_isub_dn14,)
    }
};
        var_isub = assign106200_e158468;
        var_isub_dn0 = assign106200_e158468_d_n0;
        var_isub_dn2 = assign106200_e158468_d_n2;
        var_isub_dn4 = assign106200_e158468_d_n4;
        var_isub_dn5 = assign106200_e158468_d_n5;
        var_isub_dn6 = assign106200_e158468_d_n6;
        var_isub_dn7 = assign106200_e158468_d_n7;
        var_isub_dn8 = assign106200_e158468_d_n8;
        var_isub_dn9 = assign106200_e158468_d_n9;
        var_isub_dn10 = assign106200_e158468_d_n10;
        var_isub_dn11 = assign106200_e158468_d_n11;
        var_isub_dn14 = assign106200_e158468_d_n14;

        let (assign106210_e158473, assign106210_e158473_d_n0, assign106210_e158473_d_n2, assign106210_e158473_d_n4, assign106210_e158473_d_n5, assign106210_e158473_d_n6, assign106210_e158473_d_n7, assign106210_e158473_d_n8, assign106210_e158473_d_n9, assign106210_e158473_d_n10, assign106210_e158473_d_n11, assign106210_e158473_d_n14,) = {
    if (var_guard2403 == 0.0) {
        (var_isube, var_isube_dn0, var_isube_dn2, var_isube_dn4, var_isube_dn5, var_isube_dn6, var_isube_dn7, var_isube_dn8, var_isube_dn9, var_isube_dn10, var_isube_dn11, var_isube_dn14,)
    } else {
        (var_isubs, var_isubs_dn0, var_isubs_dn2, var_isubs_dn4, var_isubs_dn5, var_isubs_dn6, var_isubs_dn7, var_isubs_dn8, var_isubs_dn9, var_isubs_dn10, var_isubs_dn11, var_isubs_dn14,)
    }
};
        var_isubs = assign106210_e158473;
        var_isubs_dn0 = assign106210_e158473_d_n0;
        var_isubs_dn2 = assign106210_e158473_d_n2;
        var_isubs_dn4 = assign106210_e158473_d_n4;
        var_isubs_dn5 = assign106210_e158473_d_n5;
        var_isubs_dn6 = assign106210_e158473_d_n6;
        var_isubs_dn7 = assign106210_e158473_d_n7;
        var_isubs_dn8 = assign106210_e158473_d_n8;
        var_isubs_dn9 = assign106210_e158473_d_n9;
        var_isubs_dn10 = assign106210_e158473_d_n10;
        var_isubs_dn11 = assign106210_e158473_d_n11;
        var_isubs_dn14 = assign106210_e158473_d_n14;

        let (assign106220_e158478, assign106220_e158478_d_n0, assign106220_e158478_d_n2, assign106220_e158478_d_n4, assign106220_e158478_d_n5, assign106220_e158478_d_n6, assign106220_e158478_d_n7, assign106220_e158478_d_n8, assign106220_e158478_d_n9, assign106220_e158478_d_n10, assign106220_e158478_d_n11, assign106220_e158478_d_n14,) = {
    if (var_guard2403 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isubld, var_isubld_dn0, var_isubld_dn2, var_isubld_dn4, var_isubld_dn5, var_isubld_dn6, var_isubld_dn7, var_isubld_dn8, var_isubld_dn9, var_isubld_dn10, var_isubld_dn11, var_isubld_dn14,)
    }
};
        var_isubld = assign106220_e158478;
        var_isubld_dn0 = assign106220_e158478_d_n0;
        var_isubld_dn2 = assign106220_e158478_d_n2;
        var_isubld_dn4 = assign106220_e158478_d_n4;
        var_isubld_dn5 = assign106220_e158478_d_n5;
        var_isubld_dn6 = assign106220_e158478_d_n6;
        var_isubld_dn7 = assign106220_e158478_d_n7;
        var_isubld_dn8 = assign106220_e158478_d_n8;
        var_isubld_dn9 = assign106220_e158478_d_n9;
        var_isubld_dn10 = assign106220_e158478_d_n10;
        var_isubld_dn11 = assign106220_e158478_d_n11;
        var_isubld_dn14 = assign106220_e158478_d_n14;

        let (assign106230_e158483, assign106230_e158483_d_n0, assign106230_e158483_d_n2, assign106230_e158483_d_n4, assign106230_e158483_d_n5, assign106230_e158483_d_n6, assign106230_e158483_d_n7, assign106230_e158483_d_n8, assign106230_e158483_d_n9, assign106230_e158483_d_n10, assign106230_e158483_d_n11, assign106230_e158483_d_n14,) = {
    if (var_guard2403 == 0.0) {
        (var_isublde, var_isublde_dn0, var_isublde_dn2, var_isublde_dn4, var_isublde_dn5, var_isublde_dn6, var_isublde_dn7, var_isublde_dn8, var_isublde_dn9, var_isublde_dn10, var_isublde_dn11, var_isublde_dn14,)
    } else {
        (var_isublds, var_isublds_dn0, var_isublds_dn2, var_isublds_dn4, var_isublds_dn5, var_isublds_dn6, var_isublds_dn7, var_isublds_dn8, var_isublds_dn9, var_isublds_dn10, var_isublds_dn11, var_isublds_dn14,)
    }
};
        var_isublds = assign106230_e158483;
        var_isublds_dn0 = assign106230_e158483_d_n0;
        var_isublds_dn2 = assign106230_e158483_d_n2;
        var_isublds_dn4 = assign106230_e158483_d_n4;
        var_isublds_dn5 = assign106230_e158483_d_n5;
        var_isublds_dn6 = assign106230_e158483_d_n6;
        var_isublds_dn7 = assign106230_e158483_d_n7;
        var_isublds_dn8 = assign106230_e158483_d_n8;
        var_isublds_dn9 = assign106230_e158483_d_n9;
        var_isublds_dn10 = assign106230_e158483_d_n10;
        var_isublds_dn11 = assign106230_e158483_d_n11;
        var_isublds_dn14 = assign106230_e158483_d_n14;

        let (assign106240_e158488, assign106240_e158488_d_n0, assign106240_e158488_d_n2, assign106240_e158488_d_n4, assign106240_e158488_d_n5, assign106240_e158488_d_n6, assign106240_e158488_d_n7, assign106240_e158488_d_n8, assign106240_e158488_d_n9, assign106240_e158488_d_n10, assign106240_e158488_d_n11, assign106240_e158488_d_n14,) = {
    if (var_guard2403 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_idsibpc, var_idsibpc_dn0, var_idsibpc_dn2, var_idsibpc_dn4, var_idsibpc_dn5, var_idsibpc_dn6, var_idsibpc_dn7, var_idsibpc_dn8, var_idsibpc_dn9, var_idsibpc_dn10, var_idsibpc_dn11, var_idsibpc_dn14,)
    }
};
        var_idsibpc = assign106240_e158488;
        var_idsibpc_dn0 = assign106240_e158488_d_n0;
        var_idsibpc_dn2 = assign106240_e158488_d_n2;
        var_idsibpc_dn4 = assign106240_e158488_d_n4;
        var_idsibpc_dn5 = assign106240_e158488_d_n5;
        var_idsibpc_dn6 = assign106240_e158488_d_n6;
        var_idsibpc_dn7 = assign106240_e158488_d_n7;
        var_idsibpc_dn8 = assign106240_e158488_d_n8;
        var_idsibpc_dn9 = assign106240_e158488_d_n9;
        var_idsibpc_dn10 = assign106240_e158488_d_n10;
        var_idsibpc_dn11 = assign106240_e158488_d_n11;
        var_idsibpc_dn14 = assign106240_e158488_d_n14;

        let (assign106250_e158493, assign106250_e158493_d_n0, assign106250_e158493_d_n2, assign106250_e158493_d_n4, assign106250_e158493_d_n5, assign106250_e158493_d_n6, assign106250_e158493_d_n7, assign106250_e158493_d_n8, assign106250_e158493_d_n9, assign106250_e158493_d_n10, assign106250_e158493_d_n11, assign106250_e158493_d_n14,) = {
    if (var_guard2403 == 0.0) {
        (var_idsibpce, var_idsibpce_dn0, var_idsibpce_dn2, var_idsibpce_dn4, var_idsibpce_dn5, var_idsibpce_dn6, var_idsibpce_dn7, var_idsibpce_dn8, var_idsibpce_dn9, var_idsibpce_dn10, var_idsibpce_dn11, var_idsibpce_dn14,)
    } else {
        (var_idsibpcs, var_idsibpcs_dn0, var_idsibpcs_dn2, var_idsibpcs_dn4, var_idsibpcs_dn5, var_idsibpcs_dn6, var_idsibpcs_dn7, var_idsibpcs_dn8, var_idsibpcs_dn9, var_idsibpcs_dn10, var_idsibpcs_dn11, var_idsibpcs_dn14,)
    }
};
        var_idsibpcs = assign106250_e158493;
        var_idsibpcs_dn0 = assign106250_e158493_d_n0;
        var_idsibpcs_dn2 = assign106250_e158493_d_n2;
        var_idsibpcs_dn4 = assign106250_e158493_d_n4;
        var_idsibpcs_dn5 = assign106250_e158493_d_n5;
        var_idsibpcs_dn6 = assign106250_e158493_d_n6;
        var_idsibpcs_dn7 = assign106250_e158493_d_n7;
        var_idsibpcs_dn8 = assign106250_e158493_d_n8;
        var_idsibpcs_dn9 = assign106250_e158493_d_n9;
        var_idsibpcs_dn10 = assign106250_e158493_d_n10;
        var_idsibpcs_dn11 = assign106250_e158493_d_n11;
        var_idsibpcs_dn14 = assign106250_e158493_d_n14;

        let (assign106260_e158498, assign106260_e158498_d_n0, assign106260_e158498_d_n2, assign106260_e158498_d_n4, assign106260_e158498_d_n5, assign106260_e158498_d_n6, assign106260_e158498_d_n7, assign106260_e158498_d_n8, assign106260_e158498_d_n9, assign106260_e158498_d_n10, assign106260_e158498_d_n11, assign106260_e158498_d_n14,) = {
    if (var_guard2403 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibjt, var_ibjt_dn0, var_ibjt_dn2, var_ibjt_dn4, var_ibjt_dn5, var_ibjt_dn6, var_ibjt_dn7, var_ibjt_dn8, var_ibjt_dn9, var_ibjt_dn10, var_ibjt_dn11, var_ibjt_dn14,)
    }
};
        var_ibjt = assign106260_e158498;
        var_ibjt_dn0 = assign106260_e158498_d_n0;
        var_ibjt_dn2 = assign106260_e158498_d_n2;
        var_ibjt_dn4 = assign106260_e158498_d_n4;
        var_ibjt_dn5 = assign106260_e158498_d_n5;
        var_ibjt_dn6 = assign106260_e158498_d_n6;
        var_ibjt_dn7 = assign106260_e158498_d_n7;
        var_ibjt_dn8 = assign106260_e158498_d_n8;
        var_ibjt_dn9 = assign106260_e158498_d_n9;
        var_ibjt_dn10 = assign106260_e158498_d_n10;
        var_ibjt_dn11 = assign106260_e158498_d_n11;
        var_ibjt_dn14 = assign106260_e158498_d_n14;

        *var_ibjt_slot = var_ibjt;
        *var_ibjt_dn0_slot = var_ibjt_dn0;
        *var_ibjt_dn10_slot = var_ibjt_dn10;
        *var_ibjt_dn11_slot = var_ibjt_dn11;
        *var_ibjt_dn14_slot = var_ibjt_dn14;
        *var_ibjt_dn2_slot = var_ibjt_dn2;
        *var_ibjt_dn4_slot = var_ibjt_dn4;
        *var_ibjt_dn5_slot = var_ibjt_dn5;
        *var_ibjt_dn6_slot = var_ibjt_dn6;
        *var_ibjt_dn7_slot = var_ibjt_dn7;
        *var_ibjt_dn8_slot = var_ibjt_dn8;
        *var_ibjt_dn9_slot = var_ibjt_dn9;
        *var_ibjts_slot = var_ibjts;
        *var_ibjts_dn0_slot = var_ibjts_dn0;
        *var_ibjts_dn10_slot = var_ibjts_dn10;
        *var_ibjts_dn11_slot = var_ibjts_dn11;
        *var_ibjts_dn14_slot = var_ibjts_dn14;
        *var_ibjts_dn2_slot = var_ibjts_dn2;
        *var_ibjts_dn4_slot = var_ibjts_dn4;
        *var_ibjts_dn5_slot = var_ibjts_dn5;
        *var_ibjts_dn6_slot = var_ibjts_dn6;
        *var_ibjts_dn7_slot = var_ibjts_dn7;
        *var_ibjts_dn8_slot = var_ibjts_dn8;
        *var_ibjts_dn9_slot = var_ibjts_dn9;
        *var_ibreak_slot = var_ibreak;
        *var_ibreak_dn0_slot = var_ibreak_dn0;
        *var_ibreak_dn10_slot = var_ibreak_dn10;
        *var_ibreak_dn11_slot = var_ibreak_dn11;
        *var_ibreak_dn14_slot = var_ibreak_dn14;
        *var_ibreak_dn2_slot = var_ibreak_dn2;
        *var_ibreak_dn4_slot = var_ibreak_dn4;
        *var_ibreak_dn5_slot = var_ibreak_dn5;
        *var_ibreak_dn6_slot = var_ibreak_dn6;
        *var_ibreak_dn7_slot = var_ibreak_dn7;
        *var_ibreak_dn8_slot = var_ibreak_dn8;
        *var_ibreak_dn9_slot = var_ibreak_dn9;
        *var_ibreaks_slot = var_ibreaks;
        *var_ibreaks_dn0_slot = var_ibreaks_dn0;
        *var_ibreaks_dn10_slot = var_ibreaks_dn10;
        *var_ibreaks_dn11_slot = var_ibreaks_dn11;
        *var_ibreaks_dn14_slot = var_ibreaks_dn14;
        *var_ibreaks_dn2_slot = var_ibreaks_dn2;
        *var_ibreaks_dn4_slot = var_ibreaks_dn4;
        *var_ibreaks_dn5_slot = var_ibreaks_dn5;
        *var_ibreaks_dn6_slot = var_ibreaks_dn6;
        *var_ibreaks_dn7_slot = var_ibreaks_dn7;
        *var_ibreaks_dn8_slot = var_ibreaks_dn8;
        *var_ibreaks_dn9_slot = var_ibreaks_dn9;
        *var_ids_slot = var_ids;
        *var_ids_dn0_slot = var_ids_dn0;
        *var_ids_dn10_slot = var_ids_dn10;
        *var_ids_dn11_slot = var_ids_dn11;
        *var_ids_dn14_slot = var_ids_dn14;
        *var_ids_dn2_slot = var_ids_dn2;
        *var_ids_dn4_slot = var_ids_dn4;
        *var_ids_dn5_slot = var_ids_dn5;
        *var_ids_dn6_slot = var_ids_dn6;
        *var_ids_dn7_slot = var_ids_dn7;
        *var_ids_dn8_slot = var_ids_dn8;
        *var_ids_dn9_slot = var_ids_dn9;
        *var_idsibpc_slot = var_idsibpc;
        *var_idsibpc_dn0_slot = var_idsibpc_dn0;
        *var_idsibpc_dn10_slot = var_idsibpc_dn10;
        *var_idsibpc_dn11_slot = var_idsibpc_dn11;
        *var_idsibpc_dn14_slot = var_idsibpc_dn14;
        *var_idsibpc_dn2_slot = var_idsibpc_dn2;
        *var_idsibpc_dn4_slot = var_idsibpc_dn4;
        *var_idsibpc_dn5_slot = var_idsibpc_dn5;
        *var_idsibpc_dn6_slot = var_idsibpc_dn6;
        *var_idsibpc_dn7_slot = var_idsibpc_dn7;
        *var_idsibpc_dn8_slot = var_idsibpc_dn8;
        *var_idsibpc_dn9_slot = var_idsibpc_dn9;
        *var_idsibpcs_slot = var_idsibpcs;
        *var_idsibpcs_dn0_slot = var_idsibpcs_dn0;
        *var_idsibpcs_dn10_slot = var_idsibpcs_dn10;
        *var_idsibpcs_dn11_slot = var_idsibpcs_dn11;
        *var_idsibpcs_dn14_slot = var_idsibpcs_dn14;
        *var_idsibpcs_dn2_slot = var_idsibpcs_dn2;
        *var_idsibpcs_dn4_slot = var_idsibpcs_dn4;
        *var_idsibpcs_dn5_slot = var_idsibpcs_dn5;
        *var_idsibpcs_dn6_slot = var_idsibpcs_dn6;
        *var_idsibpcs_dn7_slot = var_idsibpcs_dn7;
        *var_idsibpcs_dn8_slot = var_idsibpcs_dn8;
        *var_idsibpcs_dn9_slot = var_idsibpcs_dn9;
        *var_igidl_slot = var_igidl;
        *var_igidl_dn0_slot = var_igidl_dn0;
        *var_igidl_dn10_slot = var_igidl_dn10;
        *var_igidl_dn11_slot = var_igidl_dn11;
        *var_igidl_dn14_slot = var_igidl_dn14;
        *var_igidl_dn2_slot = var_igidl_dn2;
        *var_igidl_dn4_slot = var_igidl_dn4;
        *var_igidl_dn5_slot = var_igidl_dn5;
        *var_igidl_dn6_slot = var_igidl_dn6;
        *var_igidl_dn7_slot = var_igidl_dn7;
        *var_igidl_dn8_slot = var_igidl_dn8;
        *var_igidl_dn9_slot = var_igidl_dn9;
        *var_igisl_slot = var_igisl;
        *var_igisl_dn0_slot = var_igisl_dn0;
        *var_igisl_dn10_slot = var_igisl_dn10;
        *var_igisl_dn11_slot = var_igisl_dn11;
        *var_igisl_dn14_slot = var_igisl_dn14;
        *var_igisl_dn2_slot = var_igisl_dn2;
        *var_igisl_dn4_slot = var_igisl_dn4;
        *var_igisl_dn5_slot = var_igisl_dn5;
        *var_igisl_dn6_slot = var_igisl_dn6;
        *var_igisl_dn7_slot = var_igisl_dn7;
        *var_igisl_dn8_slot = var_igisl_dn8;
        *var_igisl_dn9_slot = var_igisl_dn9;
        *var_isub_slot = var_isub;
        *var_isub_dn0_slot = var_isub_dn0;
        *var_isub_dn10_slot = var_isub_dn10;
        *var_isub_dn11_slot = var_isub_dn11;
        *var_isub_dn14_slot = var_isub_dn14;
        *var_isub_dn2_slot = var_isub_dn2;
        *var_isub_dn4_slot = var_isub_dn4;
        *var_isub_dn5_slot = var_isub_dn5;
        *var_isub_dn6_slot = var_isub_dn6;
        *var_isub_dn7_slot = var_isub_dn7;
        *var_isub_dn8_slot = var_isub_dn8;
        *var_isub_dn9_slot = var_isub_dn9;
        *var_isubld_slot = var_isubld;
        *var_isubld_dn0_slot = var_isubld_dn0;
        *var_isubld_dn10_slot = var_isubld_dn10;
        *var_isubld_dn11_slot = var_isubld_dn11;
        *var_isubld_dn14_slot = var_isubld_dn14;
        *var_isubld_dn2_slot = var_isubld_dn2;
        *var_isubld_dn4_slot = var_isubld_dn4;
        *var_isubld_dn5_slot = var_isubld_dn5;
        *var_isubld_dn6_slot = var_isubld_dn6;
        *var_isubld_dn7_slot = var_isubld_dn7;
        *var_isubld_dn8_slot = var_isubld_dn8;
        *var_isubld_dn9_slot = var_isubld_dn9;
        *var_isublds_slot = var_isublds;
        *var_isublds_dn0_slot = var_isublds_dn0;
        *var_isublds_dn10_slot = var_isublds_dn10;
        *var_isublds_dn11_slot = var_isublds_dn11;
        *var_isublds_dn14_slot = var_isublds_dn14;
        *var_isublds_dn2_slot = var_isublds_dn2;
        *var_isublds_dn4_slot = var_isublds_dn4;
        *var_isublds_dn5_slot = var_isublds_dn5;
        *var_isublds_dn6_slot = var_isublds_dn6;
        *var_isublds_dn7_slot = var_isublds_dn7;
        *var_isublds_dn8_slot = var_isublds_dn8;
        *var_isublds_dn9_slot = var_isublds_dn9;
        *var_isubs_slot = var_isubs;
        *var_isubs_dn0_slot = var_isubs_dn0;
        *var_isubs_dn10_slot = var_isubs_dn10;
        *var_isubs_dn11_slot = var_isubs_dn11;
        *var_isubs_dn14_slot = var_isubs_dn14;
        *var_isubs_dn2_slot = var_isubs_dn2;
        *var_isubs_dn4_slot = var_isubs_dn4;
        *var_isubs_dn5_slot = var_isubs_dn5;
        *var_isubs_dn6_slot = var_isubs_dn6;
        *var_isubs_dn7_slot = var_isubs_dn7;
        *var_isubs_dn8_slot = var_isubs_dn8;
        *var_isubs_dn9_slot = var_isubs_dn9;
        *var_qb_slot = var_qb;
        *var_qb_dn0_slot = var_qb_dn0;
        *var_qb_dn10_slot = var_qb_dn10;
        *var_qb_dn11_slot = var_qb_dn11;
        *var_qb_dn14_slot = var_qb_dn14;
        *var_qb_dn2_slot = var_qb_dn2;
        *var_qb_dn4_slot = var_qb_dn4;
        *var_qb_dn5_slot = var_qb_dn5;
        *var_qb_dn6_slot = var_qb_dn6;
        *var_qb_dn7_slot = var_qb_dn7;
        *var_qb_dn8_slot = var_qb_dn8;
        *var_qb_dn9_slot = var_qb_dn9;
        *var_qd_slot = var_qd;
        *var_qd_dn0_slot = var_qd_dn0;
        *var_qd_dn10_slot = var_qd_dn10;
        *var_qd_dn11_slot = var_qd_dn11;
        *var_qd_dn14_slot = var_qd_dn14;
        *var_qd_dn2_slot = var_qd_dn2;
        *var_qd_dn4_slot = var_qd_dn4;
        *var_qd_dn5_slot = var_qd_dn5;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn7_slot = var_qd_dn7;
        *var_qd_dn8_slot = var_qd_dn8;
        *var_qd_dn9_slot = var_qd_dn9;
        *var_qdrat_slot = var_qdrat;
        *var_qdrat_dn0_slot = var_qdrat_dn0;
        *var_qdrat_dn10_slot = var_qdrat_dn10;
        *var_qdrat_dn11_slot = var_qdrat_dn11;
        *var_qdrat_dn14_slot = var_qdrat_dn14;
        *var_qdrat_dn2_slot = var_qdrat_dn2;
        *var_qdrat_dn4_slot = var_qdrat_dn4;
        *var_qdrat_dn5_slot = var_qdrat_dn5;
        *var_qdrat_dn6_slot = var_qdrat_dn6;
        *var_qdrat_dn7_slot = var_qdrat_dn7;
        *var_qdrat_dn8_slot = var_qdrat_dn8;
        *var_qdrat_dn9_slot = var_qdrat_dn9;
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
    }

    pub(super) fn stamp_transient_block_390(
        p: &Parameters,
        var_flg_nqs: f64,
        var_guard2403: f64,
        var_ibjte: f64,
        var_ibjte_dn0: f64,
        var_ibjte_dn10: f64,
        var_ibjte_dn11: f64,
        var_ibjte_dn14: f64,
        var_ibjte_dn2: f64,
        var_ibjte_dn4: f64,
        var_ibjte_dn5: f64,
        var_ibjte_dn6: f64,
        var_ibjte_dn7: f64,
        var_ibjte_dn8: f64,
        var_ibjte_dn9: f64,
        var_ibreake: f64,
        var_ibreake_dn0: f64,
        var_ibreake_dn10: f64,
        var_ibreake_dn11: f64,
        var_ibreake_dn14: f64,
        var_ibreake_dn2: f64,
        var_ibreake_dn4: f64,
        var_ibreake_dn5: f64,
        var_ibreake_dn6: f64,
        var_ibreake_dn7: f64,
        var_ibreake_dn8: f64,
        var_ibreake_dn9: f64,
        var_ids: f64,
        var_ids_dn0: f64,
        var_ids_dn10: f64,
        var_ids_dn11: f64,
        var_ids_dn14: f64,
        var_ids_dn2: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_ids_dn9: f64,
        var_igidle: f64,
        var_igidle_dn0: f64,
        var_igidle_dn10: f64,
        var_igidle_dn11: f64,
        var_igidle_dn14: f64,
        var_igidle_dn2: f64,
        var_igidle_dn4: f64,
        var_igidle_dn5: f64,
        var_igidle_dn6: f64,
        var_igidle_dn7: f64,
        var_igidle_dn8: f64,
        var_igidle_dn9: f64,
        var_igisle: f64,
        var_igisle_dn0: f64,
        var_igisle_dn10: f64,
        var_igisle_dn11: f64,
        var_igisle_dn14: f64,
        var_igisle_dn2: f64,
        var_igisle_dn4: f64,
        var_igisle_dn5: f64,
        var_igisle_dn6: f64,
        var_igisle_dn7: f64,
        var_igisle_dn8: f64,
        var_igisle_dn9: f64,
        var_powratio: f64,
        var_powratio_dn0: f64,
        var_powratio_dn10: f64,
        var_powratio_dn11: f64,
        var_powratio_dn14: f64,
        var_powratio_dn2: f64,
        var_powratio_dn4: f64,
        var_powratio_dn5: f64,
        var_powratio_dn6: f64,
        var_powratio_dn7: f64,
        var_powratio_dn8: f64,
        var_powratio_dn9: f64,
        var_qdexte: f64,
        var_qdexte_dn0: f64,
        var_qdexte_dn10: f64,
        var_qdexte_dn11: f64,
        var_qdexte_dn14: f64,
        var_qdexte_dn2: f64,
        var_qdexte_dn4: f64,
        var_qdexte_dn5: f64,
        var_qdexte_dn6: f64,
        var_qdexte_dn7: f64,
        var_qdexte_dn8: f64,
        var_qdexte_dn9: f64,
        var_qdov: f64,
        var_qdov_dn0: f64,
        var_qdov_dn10: f64,
        var_qdov_dn11: f64,
        var_qdov_dn14: f64,
        var_qdov_dn2: f64,
        var_qdov_dn4: f64,
        var_qdov_dn5: f64,
        var_qdov_dn6: f64,
        var_qdov_dn7: f64,
        var_qdov_dn8: f64,
        var_qdov_dn9: f64,
        var_qdp: f64,
        var_qdp_dn0: f64,
        var_qdp_dn2: f64,
        var_qdp_dn7: f64,
        var_qgexte: f64,
        var_qgexte_dn0: f64,
        var_qgexte_dn10: f64,
        var_qgexte_dn11: f64,
        var_qgexte_dn14: f64,
        var_qgexte_dn2: f64,
        var_qgexte_dn4: f64,
        var_qgexte_dn5: f64,
        var_qgexte_dn6: f64,
        var_qgexte_dn7: f64,
        var_qgexte_dn8: f64,
        var_qgexte_dn9: f64,
        var_qgov: f64,
        var_qgov_dn0: f64,
        var_qgov_dn10: f64,
        var_qgov_dn11: f64,
        var_qgov_dn14: f64,
        var_qgov_dn2: f64,
        var_qgov_dn4: f64,
        var_qgov_dn5: f64,
        var_qgov_dn6: f64,
        var_qgov_dn7: f64,
        var_qgov_dn8: f64,
        var_qgov_dn9: f64,
        var_qsexte: f64,
        var_qsexte_dn0: f64,
        var_qsexte_dn10: f64,
        var_qsexte_dn11: f64,
        var_qsexte_dn14: f64,
        var_qsexte_dn2: f64,
        var_qsexte_dn4: f64,
        var_qsexte_dn5: f64,
        var_qsexte_dn6: f64,
        var_qsexte_dn7: f64,
        var_qsexte_dn8: f64,
        var_qsexte_dn9: f64,
        var_qsov: f64,
        var_qsov_dn0: f64,
        var_qsov_dn10: f64,
        var_qsov_dn11: f64,
        var_qsov_dn14: f64,
        var_qsov_dn2: f64,
        var_qsov_dn4: f64,
        var_qsov_dn5: f64,
        var_qsov_dn6: f64,
        var_qsov_dn7: f64,
        var_qsov_dn8: f64,
        var_qsov_dn9: f64,
        var_qsp: f64,
        var_qsp_dn2: f64,
        var_qsp_dn7: f64,
        var_rth: f64,
        var_rth_dn0: f64,
        var_rth_dn10: f64,
        var_rth_dn11: f64,
        var_rth_dn14: f64,
        var_rth_dn2: f64,
        var_rth_dn4: f64,
        var_rth_dn5: f64,
        var_rth_dn6: f64,
        var_rth_dn7: f64,
        var_rth_dn8: f64,
        var_rth_dn9: f64,
        var_uc_powrat: f64,
        var_vdsei: f64,
        var_vdsei_dn0: f64,
        var_vdsei_dn2: f64,
        var_vdsi: f64,
        var_vdsi_dn6: f64,
        var_vdsi_dn8: f64,
        var_xd: f64,
        var_xd_dn0: f64,
        var_xd_dn10: f64,
        var_xd_dn11: f64,
        var_xd_dn14: f64,
        var_xd_dn2: f64,
        var_xd_dn4: f64,
        var_xd_dn5: f64,
        var_xd_dn6: f64,
        var_xd_dn7: f64,
        var_xd_dn8: f64,
        var_xd_dn9: f64,
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
        var_guard2404_slot: &mut f64,
        var_guard2405_slot: &mut f64,
        var_guard2406_slot: &mut f64,
        var_guard2407_slot: &mut f64,
        var_guard2408_slot: &mut f64,
        var_ibjts_slot: &mut f64,
        var_ibjts_dn0_slot: &mut f64,
        var_ibjts_dn10_slot: &mut f64,
        var_ibjts_dn11_slot: &mut f64,
        var_ibjts_dn14_slot: &mut f64,
        var_ibjts_dn2_slot: &mut f64,
        var_ibjts_dn4_slot: &mut f64,
        var_ibjts_dn5_slot: &mut f64,
        var_ibjts_dn6_slot: &mut f64,
        var_ibjts_dn7_slot: &mut f64,
        var_ibjts_dn8_slot: &mut f64,
        var_ibjts_dn9_slot: &mut f64,
        var_ibreak_slot: &mut f64,
        var_ibreak_dn0_slot: &mut f64,
        var_ibreak_dn10_slot: &mut f64,
        var_ibreak_dn11_slot: &mut f64,
        var_ibreak_dn14_slot: &mut f64,
        var_ibreak_dn2_slot: &mut f64,
        var_ibreak_dn4_slot: &mut f64,
        var_ibreak_dn5_slot: &mut f64,
        var_ibreak_dn6_slot: &mut f64,
        var_ibreak_dn7_slot: &mut f64,
        var_ibreak_dn8_slot: &mut f64,
        var_ibreak_dn9_slot: &mut f64,
        var_ibreaks_slot: &mut f64,
        var_ibreaks_dn0_slot: &mut f64,
        var_ibreaks_dn10_slot: &mut f64,
        var_ibreaks_dn11_slot: &mut f64,
        var_ibreaks_dn14_slot: &mut f64,
        var_ibreaks_dn2_slot: &mut f64,
        var_ibreaks_dn4_slot: &mut f64,
        var_ibreaks_dn5_slot: &mut f64,
        var_ibreaks_dn6_slot: &mut f64,
        var_ibreaks_dn7_slot: &mut f64,
        var_ibreaks_dn8_slot: &mut f64,
        var_ibreaks_dn9_slot: &mut f64,
        var_igidl_slot: &mut f64,
        var_igidl_dn0_slot: &mut f64,
        var_igidl_dn10_slot: &mut f64,
        var_igidl_dn11_slot: &mut f64,
        var_igidl_dn14_slot: &mut f64,
        var_igidl_dn2_slot: &mut f64,
        var_igidl_dn4_slot: &mut f64,
        var_igidl_dn5_slot: &mut f64,
        var_igidl_dn6_slot: &mut f64,
        var_igidl_dn7_slot: &mut f64,
        var_igidl_dn8_slot: &mut f64,
        var_igidl_dn9_slot: &mut f64,
        var_igisl_slot: &mut f64,
        var_igisl_dn0_slot: &mut f64,
        var_igisl_dn10_slot: &mut f64,
        var_igisl_dn11_slot: &mut f64,
        var_igisl_dn14_slot: &mut f64,
        var_igisl_dn2_slot: &mut f64,
        var_igisl_dn4_slot: &mut f64,
        var_igisl_dn5_slot: &mut f64,
        var_igisl_dn6_slot: &mut f64,
        var_igisl_dn7_slot: &mut f64,
        var_igisl_dn8_slot: &mut f64,
        var_igisl_dn9_slot: &mut f64,
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
        var_qb_slot: &mut f64,
        var_qb_dn0_slot: &mut f64,
        var_qb_dn10_slot: &mut f64,
        var_qb_dn11_slot: &mut f64,
        var_qb_dn14_slot: &mut f64,
        var_qb_dn2_slot: &mut f64,
        var_qb_dn4_slot: &mut f64,
        var_qb_dn5_slot: &mut f64,
        var_qb_dn6_slot: &mut f64,
        var_qb_dn7_slot: &mut f64,
        var_qb_dn8_slot: &mut f64,
        var_qb_dn9_slot: &mut f64,
        var_qbext_slot: &mut f64,
        var_qbext_dn0_slot: &mut f64,
        var_qbext_dn10_slot: &mut f64,
        var_qbext_dn11_slot: &mut f64,
        var_qbext_dn14_slot: &mut f64,
        var_qbext_dn2_slot: &mut f64,
        var_qbext_dn4_slot: &mut f64,
        var_qbext_dn5_slot: &mut f64,
        var_qbext_dn6_slot: &mut f64,
        var_qbext_dn7_slot: &mut f64,
        var_qbext_dn8_slot: &mut f64,
        var_qbext_dn9_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn0_slot: &mut f64,
        var_qd_dn10_slot: &mut f64,
        var_qd_dn11_slot: &mut f64,
        var_qd_dn14_slot: &mut f64,
        var_qd_dn2_slot: &mut f64,
        var_qd_dn4_slot: &mut f64,
        var_qd_dn5_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn7_slot: &mut f64,
        var_qd_dn8_slot: &mut f64,
        var_qd_dn9_slot: &mut f64,
        var_qdext_slot: &mut f64,
        var_qdext_dn0_slot: &mut f64,
        var_qdext_dn10_slot: &mut f64,
        var_qdext_dn11_slot: &mut f64,
        var_qdext_dn14_slot: &mut f64,
        var_qdext_dn2_slot: &mut f64,
        var_qdext_dn4_slot: &mut f64,
        var_qdext_dn5_slot: &mut f64,
        var_qdext_dn6_slot: &mut f64,
        var_qdext_dn7_slot: &mut f64,
        var_qdext_dn8_slot: &mut f64,
        var_qdext_dn9_slot: &mut f64,
        var_qdrat_slot: &mut f64,
        var_qdrat_dn0_slot: &mut f64,
        var_qdrat_dn10_slot: &mut f64,
        var_qdrat_dn11_slot: &mut f64,
        var_qdrat_dn14_slot: &mut f64,
        var_qdrat_dn2_slot: &mut f64,
        var_qdrat_dn4_slot: &mut f64,
        var_qdrat_dn5_slot: &mut f64,
        var_qdrat_dn6_slot: &mut f64,
        var_qdrat_dn7_slot: &mut f64,
        var_qdrat_dn8_slot: &mut f64,
        var_qdrat_dn9_slot: &mut f64,
        var_qfd_slot: &mut f64,
        var_qfd_dn0_slot: &mut f64,
        var_qfd_dn2_slot: &mut f64,
        var_qfd_dn7_slot: &mut f64,
        var_qfs_slot: &mut f64,
        var_qfs_dn2_slot: &mut f64,
        var_qfs_dn7_slot: &mut f64,
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
        var_qgext_slot: &mut f64,
        var_qgext_dn0_slot: &mut f64,
        var_qgext_dn10_slot: &mut f64,
        var_qgext_dn11_slot: &mut f64,
        var_qgext_dn14_slot: &mut f64,
        var_qgext_dn2_slot: &mut f64,
        var_qgext_dn4_slot: &mut f64,
        var_qgext_dn5_slot: &mut f64,
        var_qgext_dn6_slot: &mut f64,
        var_qgext_dn7_slot: &mut f64,
        var_qgext_dn8_slot: &mut f64,
        var_qgext_dn9_slot: &mut f64,
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
    ) {
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
        let mut var_guard2404: f64 = *var_guard2404_slot;
        let mut var_guard2405: f64 = *var_guard2405_slot;
        let mut var_guard2406: f64 = *var_guard2406_slot;
        let mut var_guard2407: f64 = *var_guard2407_slot;
        let mut var_guard2408: f64 = *var_guard2408_slot;
        let mut var_ibjts: f64 = *var_ibjts_slot;
        let mut var_ibjts_dn0: f64 = *var_ibjts_dn0_slot;
        let mut var_ibjts_dn10: f64 = *var_ibjts_dn10_slot;
        let mut var_ibjts_dn11: f64 = *var_ibjts_dn11_slot;
        let mut var_ibjts_dn14: f64 = *var_ibjts_dn14_slot;
        let mut var_ibjts_dn2: f64 = *var_ibjts_dn2_slot;
        let mut var_ibjts_dn4: f64 = *var_ibjts_dn4_slot;
        let mut var_ibjts_dn5: f64 = *var_ibjts_dn5_slot;
        let mut var_ibjts_dn6: f64 = *var_ibjts_dn6_slot;
        let mut var_ibjts_dn7: f64 = *var_ibjts_dn7_slot;
        let mut var_ibjts_dn8: f64 = *var_ibjts_dn8_slot;
        let mut var_ibjts_dn9: f64 = *var_ibjts_dn9_slot;
        let mut var_ibreak: f64 = *var_ibreak_slot;
        let mut var_ibreak_dn0: f64 = *var_ibreak_dn0_slot;
        let mut var_ibreak_dn10: f64 = *var_ibreak_dn10_slot;
        let mut var_ibreak_dn11: f64 = *var_ibreak_dn11_slot;
        let mut var_ibreak_dn14: f64 = *var_ibreak_dn14_slot;
        let mut var_ibreak_dn2: f64 = *var_ibreak_dn2_slot;
        let mut var_ibreak_dn4: f64 = *var_ibreak_dn4_slot;
        let mut var_ibreak_dn5: f64 = *var_ibreak_dn5_slot;
        let mut var_ibreak_dn6: f64 = *var_ibreak_dn6_slot;
        let mut var_ibreak_dn7: f64 = *var_ibreak_dn7_slot;
        let mut var_ibreak_dn8: f64 = *var_ibreak_dn8_slot;
        let mut var_ibreak_dn9: f64 = *var_ibreak_dn9_slot;
        let mut var_ibreaks: f64 = *var_ibreaks_slot;
        let mut var_ibreaks_dn0: f64 = *var_ibreaks_dn0_slot;
        let mut var_ibreaks_dn10: f64 = *var_ibreaks_dn10_slot;
        let mut var_ibreaks_dn11: f64 = *var_ibreaks_dn11_slot;
        let mut var_ibreaks_dn14: f64 = *var_ibreaks_dn14_slot;
        let mut var_ibreaks_dn2: f64 = *var_ibreaks_dn2_slot;
        let mut var_ibreaks_dn4: f64 = *var_ibreaks_dn4_slot;
        let mut var_ibreaks_dn5: f64 = *var_ibreaks_dn5_slot;
        let mut var_ibreaks_dn6: f64 = *var_ibreaks_dn6_slot;
        let mut var_ibreaks_dn7: f64 = *var_ibreaks_dn7_slot;
        let mut var_ibreaks_dn8: f64 = *var_ibreaks_dn8_slot;
        let mut var_ibreaks_dn9: f64 = *var_ibreaks_dn9_slot;
        let mut var_igidl: f64 = *var_igidl_slot;
        let mut var_igidl_dn0: f64 = *var_igidl_dn0_slot;
        let mut var_igidl_dn10: f64 = *var_igidl_dn10_slot;
        let mut var_igidl_dn11: f64 = *var_igidl_dn11_slot;
        let mut var_igidl_dn14: f64 = *var_igidl_dn14_slot;
        let mut var_igidl_dn2: f64 = *var_igidl_dn2_slot;
        let mut var_igidl_dn4: f64 = *var_igidl_dn4_slot;
        let mut var_igidl_dn5: f64 = *var_igidl_dn5_slot;
        let mut var_igidl_dn6: f64 = *var_igidl_dn6_slot;
        let mut var_igidl_dn7: f64 = *var_igidl_dn7_slot;
        let mut var_igidl_dn8: f64 = *var_igidl_dn8_slot;
        let mut var_igidl_dn9: f64 = *var_igidl_dn9_slot;
        let mut var_igisl: f64 = *var_igisl_slot;
        let mut var_igisl_dn0: f64 = *var_igisl_dn0_slot;
        let mut var_igisl_dn10: f64 = *var_igisl_dn10_slot;
        let mut var_igisl_dn11: f64 = *var_igisl_dn11_slot;
        let mut var_igisl_dn14: f64 = *var_igisl_dn14_slot;
        let mut var_igisl_dn2: f64 = *var_igisl_dn2_slot;
        let mut var_igisl_dn4: f64 = *var_igisl_dn4_slot;
        let mut var_igisl_dn5: f64 = *var_igisl_dn5_slot;
        let mut var_igisl_dn6: f64 = *var_igisl_dn6_slot;
        let mut var_igisl_dn7: f64 = *var_igisl_dn7_slot;
        let mut var_igisl_dn8: f64 = *var_igisl_dn8_slot;
        let mut var_igisl_dn9: f64 = *var_igisl_dn9_slot;
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
        let mut var_qb: f64 = *var_qb_slot;
        let mut var_qb_dn0: f64 = *var_qb_dn0_slot;
        let mut var_qb_dn10: f64 = *var_qb_dn10_slot;
        let mut var_qb_dn11: f64 = *var_qb_dn11_slot;
        let mut var_qb_dn14: f64 = *var_qb_dn14_slot;
        let mut var_qb_dn2: f64 = *var_qb_dn2_slot;
        let mut var_qb_dn4: f64 = *var_qb_dn4_slot;
        let mut var_qb_dn5: f64 = *var_qb_dn5_slot;
        let mut var_qb_dn6: f64 = *var_qb_dn6_slot;
        let mut var_qb_dn7: f64 = *var_qb_dn7_slot;
        let mut var_qb_dn8: f64 = *var_qb_dn8_slot;
        let mut var_qb_dn9: f64 = *var_qb_dn9_slot;
        let mut var_qbext: f64 = *var_qbext_slot;
        let mut var_qbext_dn0: f64 = *var_qbext_dn0_slot;
        let mut var_qbext_dn10: f64 = *var_qbext_dn10_slot;
        let mut var_qbext_dn11: f64 = *var_qbext_dn11_slot;
        let mut var_qbext_dn14: f64 = *var_qbext_dn14_slot;
        let mut var_qbext_dn2: f64 = *var_qbext_dn2_slot;
        let mut var_qbext_dn4: f64 = *var_qbext_dn4_slot;
        let mut var_qbext_dn5: f64 = *var_qbext_dn5_slot;
        let mut var_qbext_dn6: f64 = *var_qbext_dn6_slot;
        let mut var_qbext_dn7: f64 = *var_qbext_dn7_slot;
        let mut var_qbext_dn8: f64 = *var_qbext_dn8_slot;
        let mut var_qbext_dn9: f64 = *var_qbext_dn9_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn0: f64 = *var_qd_dn0_slot;
        let mut var_qd_dn10: f64 = *var_qd_dn10_slot;
        let mut var_qd_dn11: f64 = *var_qd_dn11_slot;
        let mut var_qd_dn14: f64 = *var_qd_dn14_slot;
        let mut var_qd_dn2: f64 = *var_qd_dn2_slot;
        let mut var_qd_dn4: f64 = *var_qd_dn4_slot;
        let mut var_qd_dn5: f64 = *var_qd_dn5_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn7: f64 = *var_qd_dn7_slot;
        let mut var_qd_dn8: f64 = *var_qd_dn8_slot;
        let mut var_qd_dn9: f64 = *var_qd_dn9_slot;
        let mut var_qdext: f64 = *var_qdext_slot;
        let mut var_qdext_dn0: f64 = *var_qdext_dn0_slot;
        let mut var_qdext_dn10: f64 = *var_qdext_dn10_slot;
        let mut var_qdext_dn11: f64 = *var_qdext_dn11_slot;
        let mut var_qdext_dn14: f64 = *var_qdext_dn14_slot;
        let mut var_qdext_dn2: f64 = *var_qdext_dn2_slot;
        let mut var_qdext_dn4: f64 = *var_qdext_dn4_slot;
        let mut var_qdext_dn5: f64 = *var_qdext_dn5_slot;
        let mut var_qdext_dn6: f64 = *var_qdext_dn6_slot;
        let mut var_qdext_dn7: f64 = *var_qdext_dn7_slot;
        let mut var_qdext_dn8: f64 = *var_qdext_dn8_slot;
        let mut var_qdext_dn9: f64 = *var_qdext_dn9_slot;
        let mut var_qdrat: f64 = *var_qdrat_slot;
        let mut var_qdrat_dn0: f64 = *var_qdrat_dn0_slot;
        let mut var_qdrat_dn10: f64 = *var_qdrat_dn10_slot;
        let mut var_qdrat_dn11: f64 = *var_qdrat_dn11_slot;
        let mut var_qdrat_dn14: f64 = *var_qdrat_dn14_slot;
        let mut var_qdrat_dn2: f64 = *var_qdrat_dn2_slot;
        let mut var_qdrat_dn4: f64 = *var_qdrat_dn4_slot;
        let mut var_qdrat_dn5: f64 = *var_qdrat_dn5_slot;
        let mut var_qdrat_dn6: f64 = *var_qdrat_dn6_slot;
        let mut var_qdrat_dn7: f64 = *var_qdrat_dn7_slot;
        let mut var_qdrat_dn8: f64 = *var_qdrat_dn8_slot;
        let mut var_qdrat_dn9: f64 = *var_qdrat_dn9_slot;
        let mut var_qfd: f64 = *var_qfd_slot;
        let mut var_qfd_dn0: f64 = *var_qfd_dn0_slot;
        let mut var_qfd_dn2: f64 = *var_qfd_dn2_slot;
        let mut var_qfd_dn7: f64 = *var_qfd_dn7_slot;
        let mut var_qfs: f64 = *var_qfs_slot;
        let mut var_qfs_dn2: f64 = *var_qfs_dn2_slot;
        let mut var_qfs_dn7: f64 = *var_qfs_dn7_slot;
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
        let mut var_qgext: f64 = *var_qgext_slot;
        let mut var_qgext_dn0: f64 = *var_qgext_dn0_slot;
        let mut var_qgext_dn10: f64 = *var_qgext_dn10_slot;
        let mut var_qgext_dn11: f64 = *var_qgext_dn11_slot;
        let mut var_qgext_dn14: f64 = *var_qgext_dn14_slot;
        let mut var_qgext_dn2: f64 = *var_qgext_dn2_slot;
        let mut var_qgext_dn4: f64 = *var_qgext_dn4_slot;
        let mut var_qgext_dn5: f64 = *var_qgext_dn5_slot;
        let mut var_qgext_dn6: f64 = *var_qgext_dn6_slot;
        let mut var_qgext_dn7: f64 = *var_qgext_dn7_slot;
        let mut var_qgext_dn8: f64 = *var_qgext_dn8_slot;
        let mut var_qgext_dn9: f64 = *var_qgext_dn9_slot;
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

        let (assign106270_e158503, assign106270_e158503_d_n0, assign106270_e158503_d_n2, assign106270_e158503_d_n4, assign106270_e158503_d_n5, assign106270_e158503_d_n6, assign106270_e158503_d_n7, assign106270_e158503_d_n8, assign106270_e158503_d_n9, assign106270_e158503_d_n10, assign106270_e158503_d_n11, assign106270_e158503_d_n14,) = {
    if (var_guard2403 == 0.0) {
        (var_ibjte, var_ibjte_dn0, var_ibjte_dn2, var_ibjte_dn4, var_ibjte_dn5, var_ibjte_dn6, var_ibjte_dn7, var_ibjte_dn8, var_ibjte_dn9, var_ibjte_dn10, var_ibjte_dn11, var_ibjte_dn14,)
    } else {
        (var_ibjts, var_ibjts_dn0, var_ibjts_dn2, var_ibjts_dn4, var_ibjts_dn5, var_ibjts_dn6, var_ibjts_dn7, var_ibjts_dn8, var_ibjts_dn9, var_ibjts_dn10, var_ibjts_dn11, var_ibjts_dn14,)
    }
};
        var_ibjts = assign106270_e158503;
        var_ibjts_dn0 = assign106270_e158503_d_n0;
        var_ibjts_dn2 = assign106270_e158503_d_n2;
        var_ibjts_dn4 = assign106270_e158503_d_n4;
        var_ibjts_dn5 = assign106270_e158503_d_n5;
        var_ibjts_dn6 = assign106270_e158503_d_n6;
        var_ibjts_dn7 = assign106270_e158503_d_n7;
        var_ibjts_dn8 = assign106270_e158503_d_n8;
        var_ibjts_dn9 = assign106270_e158503_d_n9;
        var_ibjts_dn10 = assign106270_e158503_d_n10;
        var_ibjts_dn11 = assign106270_e158503_d_n11;
        var_ibjts_dn14 = assign106270_e158503_d_n14;

        let (assign106280_e158508, assign106280_e158508_d_n0, assign106280_e158508_d_n2, assign106280_e158508_d_n4, assign106280_e158508_d_n5, assign106280_e158508_d_n6, assign106280_e158508_d_n7, assign106280_e158508_d_n8, assign106280_e158508_d_n9, assign106280_e158508_d_n10, assign106280_e158508_d_n11, assign106280_e158508_d_n14,) = {
    if (var_guard2403 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibreak, var_ibreak_dn0, var_ibreak_dn2, var_ibreak_dn4, var_ibreak_dn5, var_ibreak_dn6, var_ibreak_dn7, var_ibreak_dn8, var_ibreak_dn9, var_ibreak_dn10, var_ibreak_dn11, var_ibreak_dn14,)
    }
};
        var_ibreak = assign106280_e158508;
        var_ibreak_dn0 = assign106280_e158508_d_n0;
        var_ibreak_dn2 = assign106280_e158508_d_n2;
        var_ibreak_dn4 = assign106280_e158508_d_n4;
        var_ibreak_dn5 = assign106280_e158508_d_n5;
        var_ibreak_dn6 = assign106280_e158508_d_n6;
        var_ibreak_dn7 = assign106280_e158508_d_n7;
        var_ibreak_dn8 = assign106280_e158508_d_n8;
        var_ibreak_dn9 = assign106280_e158508_d_n9;
        var_ibreak_dn10 = assign106280_e158508_d_n10;
        var_ibreak_dn11 = assign106280_e158508_d_n11;
        var_ibreak_dn14 = assign106280_e158508_d_n14;

        let (assign106290_e158513, assign106290_e158513_d_n0, assign106290_e158513_d_n2, assign106290_e158513_d_n4, assign106290_e158513_d_n5, assign106290_e158513_d_n6, assign106290_e158513_d_n7, assign106290_e158513_d_n8, assign106290_e158513_d_n9, assign106290_e158513_d_n10, assign106290_e158513_d_n11, assign106290_e158513_d_n14,) = {
    if (var_guard2403 == 0.0) {
        (var_ibreake, var_ibreake_dn0, var_ibreake_dn2, var_ibreake_dn4, var_ibreake_dn5, var_ibreake_dn6, var_ibreake_dn7, var_ibreake_dn8, var_ibreake_dn9, var_ibreake_dn10, var_ibreake_dn11, var_ibreake_dn14,)
    } else {
        (var_ibreaks, var_ibreaks_dn0, var_ibreaks_dn2, var_ibreaks_dn4, var_ibreaks_dn5, var_ibreaks_dn6, var_ibreaks_dn7, var_ibreaks_dn8, var_ibreaks_dn9, var_ibreaks_dn10, var_ibreaks_dn11, var_ibreaks_dn14,)
    }
};
        var_ibreaks = assign106290_e158513;
        var_ibreaks_dn0 = assign106290_e158513_d_n0;
        var_ibreaks_dn2 = assign106290_e158513_d_n2;
        var_ibreaks_dn4 = assign106290_e158513_d_n4;
        var_ibreaks_dn5 = assign106290_e158513_d_n5;
        var_ibreaks_dn6 = assign106290_e158513_d_n6;
        var_ibreaks_dn7 = assign106290_e158513_d_n7;
        var_ibreaks_dn8 = assign106290_e158513_d_n8;
        var_ibreaks_dn9 = assign106290_e158513_d_n9;
        var_ibreaks_dn10 = assign106290_e158513_d_n10;
        var_ibreaks_dn11 = assign106290_e158513_d_n11;
        var_ibreaks_dn14 = assign106290_e158513_d_n14;

        let (assign106300_e158518, assign106300_e158518_d_n0, assign106300_e158518_d_n2, assign106300_e158518_d_n4, assign106300_e158518_d_n5, assign106300_e158518_d_n6, assign106300_e158518_d_n7, assign106300_e158518_d_n8, assign106300_e158518_d_n9, assign106300_e158518_d_n10, assign106300_e158518_d_n11, assign106300_e158518_d_n14,) = {
    if (var_guard2403 == 0.0) {
        (var_igisle, var_igisle_dn0, var_igisle_dn2, var_igisle_dn4, var_igisle_dn5, var_igisle_dn6, var_igisle_dn7, var_igisle_dn8, var_igisle_dn9, var_igisle_dn10, var_igisle_dn11, var_igisle_dn14,)
    } else {
        (var_igidl, var_igidl_dn0, var_igidl_dn2, var_igidl_dn4, var_igidl_dn5, var_igidl_dn6, var_igidl_dn7, var_igidl_dn8, var_igidl_dn9, var_igidl_dn10, var_igidl_dn11, var_igidl_dn14,)
    }
};
        var_igidl = assign106300_e158518;
        var_igidl_dn0 = assign106300_e158518_d_n0;
        var_igidl_dn2 = assign106300_e158518_d_n2;
        var_igidl_dn4 = assign106300_e158518_d_n4;
        var_igidl_dn5 = assign106300_e158518_d_n5;
        var_igidl_dn6 = assign106300_e158518_d_n6;
        var_igidl_dn7 = assign106300_e158518_d_n7;
        var_igidl_dn8 = assign106300_e158518_d_n8;
        var_igidl_dn9 = assign106300_e158518_d_n9;
        var_igidl_dn10 = assign106300_e158518_d_n10;
        var_igidl_dn11 = assign106300_e158518_d_n11;
        var_igidl_dn14 = assign106300_e158518_d_n14;

        let (assign106310_e158523, assign106310_e158523_d_n0, assign106310_e158523_d_n2, assign106310_e158523_d_n4, assign106310_e158523_d_n5, assign106310_e158523_d_n6, assign106310_e158523_d_n7, assign106310_e158523_d_n8, assign106310_e158523_d_n9, assign106310_e158523_d_n10, assign106310_e158523_d_n11, assign106310_e158523_d_n14,) = {
    if (var_guard2403 == 0.0) {
        (var_igidle, var_igidle_dn0, var_igidle_dn2, var_igidle_dn4, var_igidle_dn5, var_igidle_dn6, var_igidle_dn7, var_igidle_dn8, var_igidle_dn9, var_igidle_dn10, var_igidle_dn11, var_igidle_dn14,)
    } else {
        (var_igisl, var_igisl_dn0, var_igisl_dn2, var_igisl_dn4, var_igisl_dn5, var_igisl_dn6, var_igisl_dn7, var_igisl_dn8, var_igisl_dn9, var_igisl_dn10, var_igisl_dn11, var_igisl_dn14,)
    }
};
        var_igisl = assign106310_e158523;
        var_igisl_dn0 = assign106310_e158523_d_n0;
        var_igisl_dn2 = assign106310_e158523_d_n2;
        var_igisl_dn4 = assign106310_e158523_d_n4;
        var_igisl_dn5 = assign106310_e158523_d_n5;
        var_igisl_dn6 = assign106310_e158523_d_n6;
        var_igisl_dn7 = assign106310_e158523_d_n7;
        var_igisl_dn8 = assign106310_e158523_d_n8;
        var_igisl_dn9 = assign106310_e158523_d_n9;
        var_igisl_dn10 = assign106310_e158523_d_n10;
        var_igisl_dn11 = assign106310_e158523_d_n11;
        var_igisl_dn14 = assign106310_e158523_d_n14;

        let (assign106320_e158532, assign106320_e158532_d_n0, assign106320_e158532_d_n2, assign106320_e158532_d_n4, assign106320_e158532_d_n5, assign106320_e158532_d_n6, assign106320_e158532_d_n7, assign106320_e158532_d_n8, assign106320_e158532_d_n9, assign106320_e158532_d_n10, assign106320_e158532_d_n11, assign106320_e158532_d_n14,) = {
    if ((var_guard2403 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign106320_e158530: f64 = (1.0 - var_xd);
        (assign106320_e158530, (-var_xd_dn0), (-var_xd_dn2), (-var_xd_dn4), (-var_xd_dn5), (-var_xd_dn6), (-var_xd_dn7), (-var_xd_dn8), (-var_xd_dn9), (-var_xd_dn10), (-var_xd_dn11), (-var_xd_dn14),)
    } else {
        (var_qdrat, var_qdrat_dn0, var_qdrat_dn2, var_qdrat_dn4, var_qdrat_dn5, var_qdrat_dn6, var_qdrat_dn7, var_qdrat_dn8, var_qdrat_dn9, var_qdrat_dn10, var_qdrat_dn11, var_qdrat_dn14,)
    }
};
        var_qdrat = assign106320_e158532;
        var_qdrat_dn0 = assign106320_e158532_d_n0;
        var_qdrat_dn2 = assign106320_e158532_d_n2;
        var_qdrat_dn4 = assign106320_e158532_d_n4;
        var_qdrat_dn5 = assign106320_e158532_d_n5;
        var_qdrat_dn6 = assign106320_e158532_d_n6;
        var_qdrat_dn7 = assign106320_e158532_d_n7;
        var_qdrat_dn8 = assign106320_e158532_d_n8;
        var_qdrat_dn9 = assign106320_e158532_d_n9;
        var_qdrat_dn10 = assign106320_e158532_d_n10;
        var_qdrat_dn11 = assign106320_e158532_d_n11;
        var_qdrat_dn14 = assign106320_e158532_d_n14;

        let assign106330_e158535: f64 = (var_qg + var_qgov);
        var_qg = assign106330_e158535;
        var_qg_dn0 = (var_qg_dn0 + var_qgov_dn0);
        var_qg_dn2 = (var_qg_dn2 + var_qgov_dn2);
        var_qg_dn4 = (var_qg_dn4 + var_qgov_dn4);
        var_qg_dn5 = (var_qg_dn5 + var_qgov_dn5);
        var_qg_dn6 = (var_qg_dn6 + var_qgov_dn6);
        var_qg_dn7 = (var_qg_dn7 + var_qgov_dn7);
        var_qg_dn8 = (var_qg_dn8 + var_qgov_dn8);
        var_qg_dn9 = (var_qg_dn9 + var_qgov_dn9);
        var_qg_dn10 = (var_qg_dn10 + var_qgov_dn10);
        var_qg_dn11 = (var_qg_dn11 + var_qgov_dn11);
        var_qg_dn14 = (var_qg_dn14 + var_qgov_dn14);

        let assign106340_e158538: f64 = (var_qd + var_qdov);
        var_qd = assign106340_e158538;
        var_qd_dn0 = (var_qd_dn0 + var_qdov_dn0);
        var_qd_dn2 = (var_qd_dn2 + var_qdov_dn2);
        var_qd_dn4 = (var_qd_dn4 + var_qdov_dn4);
        var_qd_dn5 = (var_qd_dn5 + var_qdov_dn5);
        var_qd_dn6 = (var_qd_dn6 + var_qdov_dn6);
        var_qd_dn7 = (var_qd_dn7 + var_qdov_dn7);
        var_qd_dn8 = (var_qd_dn8 + var_qdov_dn8);
        var_qd_dn9 = (var_qd_dn9 + var_qdov_dn9);
        var_qd_dn10 = (var_qd_dn10 + var_qdov_dn10);
        var_qd_dn11 = (var_qd_dn11 + var_qdov_dn11);
        var_qd_dn14 = (var_qd_dn14 + var_qdov_dn14);

        let assign106350_e158541: f64 = (var_qs + var_qsov);
        var_qs = assign106350_e158541;
        var_qs_dn0 = (var_qs_dn0 + var_qsov_dn0);
        var_qs_dn2 = (var_qs_dn2 + var_qsov_dn2);
        var_qs_dn4 = (var_qs_dn4 + var_qsov_dn4);
        var_qs_dn5 = (var_qs_dn5 + var_qsov_dn5);
        var_qs_dn6 = (var_qs_dn6 + var_qsov_dn6);
        var_qs_dn7 = (var_qs_dn7 + var_qsov_dn7);
        var_qs_dn8 = (var_qs_dn8 + var_qsov_dn8);
        var_qs_dn9 = (var_qs_dn9 + var_qsov_dn9);
        var_qs_dn10 = (var_qs_dn10 + var_qsov_dn10);
        var_qs_dn11 = (var_qs_dn11 + var_qsov_dn11);
        var_qs_dn14 = (var_qs_dn14 + var_qsov_dn14);

        let assign106360_e158544: f64 = (var_qg + var_qd);
        let assign106360_e158546: f64 = (assign106360_e158544 + var_qs);
        let assign106360_e158547: f64 = (-assign106360_e158546);
        var_qb = assign106360_e158547;
        var_qb_dn0 = (-((var_qg_dn0 + var_qd_dn0) + var_qs_dn0));
        var_qb_dn2 = (-((var_qg_dn2 + var_qd_dn2) + var_qs_dn2));
        var_qb_dn4 = (-((var_qg_dn4 + var_qd_dn4) + var_qs_dn4));
        var_qb_dn5 = (-((var_qg_dn5 + var_qd_dn5) + var_qs_dn5));
        var_qb_dn6 = (-((var_qg_dn6 + var_qd_dn6) + var_qs_dn6));
        var_qb_dn7 = (-((var_qg_dn7 + var_qd_dn7) + var_qs_dn7));
        var_qb_dn8 = (-((var_qg_dn8 + var_qd_dn8) + var_qs_dn8));
        var_qb_dn9 = (-((var_qg_dn9 + var_qd_dn9) + var_qs_dn9));
        var_qb_dn10 = (-((var_qg_dn10 + var_qd_dn10) + var_qs_dn10));
        var_qb_dn11 = (-((var_qg_dn11 + var_qd_dn11) + var_qs_dn11));
        var_qb_dn14 = (-((var_qg_dn14 + var_qd_dn14) + var_qs_dn14));

        var_qfd = var_qdp;
        var_qfd_dn0 = var_qdp_dn0;
        var_qfd_dn2 = var_qdp_dn2;
        var_qfd_dn7 = var_qdp_dn7;

        var_qfs = var_qsp;
        var_qfs_dn2 = var_qsp_dn2;
        var_qfs_dn7 = var_qsp_dn7;

        var_qdext = var_qdexte;
        var_qdext_dn0 = var_qdexte_dn0;
        var_qdext_dn2 = var_qdexte_dn2;
        var_qdext_dn4 = var_qdexte_dn4;
        var_qdext_dn5 = var_qdexte_dn5;
        var_qdext_dn6 = var_qdexte_dn6;
        var_qdext_dn7 = var_qdexte_dn7;
        var_qdext_dn8 = var_qdexte_dn8;
        var_qdext_dn9 = var_qdexte_dn9;
        var_qdext_dn10 = var_qdexte_dn10;
        var_qdext_dn11 = var_qdexte_dn11;
        var_qdext_dn14 = var_qdexte_dn14;

        var_qgext = var_qgexte;
        var_qgext_dn0 = var_qgexte_dn0;
        var_qgext_dn2 = var_qgexte_dn2;
        var_qgext_dn4 = var_qgexte_dn4;
        var_qgext_dn5 = var_qgexte_dn5;
        var_qgext_dn6 = var_qgexte_dn6;
        var_qgext_dn7 = var_qgexte_dn7;
        var_qgext_dn8 = var_qgexte_dn8;
        var_qgext_dn9 = var_qgexte_dn9;
        var_qgext_dn10 = var_qgexte_dn10;
        var_qgext_dn11 = var_qgexte_dn11;
        var_qgext_dn14 = var_qgexte_dn14;

        let assign106410_e158554: f64 = (var_qgexte + var_qdexte);
        let assign106410_e158556: f64 = (assign106410_e158554 + var_qsexte);
        let assign106410_e158557: f64 = (-assign106410_e158556);
        var_qbext = assign106410_e158557;
        var_qbext_dn0 = (-((var_qgexte_dn0 + var_qdexte_dn0) + var_qsexte_dn0));
        var_qbext_dn2 = (-((var_qgexte_dn2 + var_qdexte_dn2) + var_qsexte_dn2));
        var_qbext_dn4 = (-((var_qgexte_dn4 + var_qdexte_dn4) + var_qsexte_dn4));
        var_qbext_dn5 = (-((var_qgexte_dn5 + var_qdexte_dn5) + var_qsexte_dn5));
        var_qbext_dn6 = (-((var_qgexte_dn6 + var_qdexte_dn6) + var_qsexte_dn6));
        var_qbext_dn7 = (-((var_qgexte_dn7 + var_qdexte_dn7) + var_qsexte_dn7));
        var_qbext_dn8 = (-((var_qgexte_dn8 + var_qdexte_dn8) + var_qsexte_dn8));
        var_qbext_dn9 = (-((var_qgexte_dn9 + var_qdexte_dn9) + var_qsexte_dn9));
        var_qbext_dn10 = (-((var_qgexte_dn10 + var_qdexte_dn10) + var_qsexte_dn10));
        var_qbext_dn11 = (-((var_qgexte_dn11 + var_qdexte_dn11) + var_qsexte_dn11));
        var_qbext_dn14 = (-((var_qgexte_dn14 + var_qdexte_dn14) + var_qsexte_dn14));

        let assign106420_e158560: f64 = if p.p53 > 0.0 { 1.0 } else { 0.0 };
        var_guard2404 = assign106420_e158560;

        let assign106430_e158563: f64 = if var_rth > 0.0001 { 1.0 } else { 0.0 };
        var_guard2405 = assign106430_e158563;

        let (assign106440_e158571, assign106440_e158571_d_n0, assign106440_e158571_d_n2, assign106440_e158571_d_n4, assign106440_e158571_d_n5, assign106440_e158571_d_n6, assign106440_e158571_d_n7, assign106440_e158571_d_n8, assign106440_e158571_d_n9, assign106440_e158571_d_n10, assign106440_e158571_d_n11, assign106440_e158571_d_n14,) = {
    if ((var_guard2404 != 0.0) && (var_guard2405 != 0.0)) {
        let assign106440_e158569: f64 = (1.0 / var_rth);
        (assign106440_e158569, (-(var_rth_dn0 / (var_rth * var_rth))), (-(var_rth_dn2 / (var_rth * var_rth))), (-(var_rth_dn4 / (var_rth * var_rth))), (-(var_rth_dn5 / (var_rth * var_rth))), (-(var_rth_dn6 / (var_rth * var_rth))), (-(var_rth_dn7 / (var_rth * var_rth))), (-(var_rth_dn8 / (var_rth * var_rth))), (-(var_rth_dn9 / (var_rth * var_rth))), (-(var_rth_dn10 / (var_rth * var_rth))), (-(var_rth_dn11 / (var_rth * var_rth))), (-(var_rth_dn14 / (var_rth * var_rth))),)
    } else {
        (var_gth, var_gth_dn0, var_gth_dn2, var_gth_dn4, var_gth_dn5, var_gth_dn6, var_gth_dn7, var_gth_dn8, var_gth_dn9, var_gth_dn10, var_gth_dn11, var_gth_dn14,)
    }
};
        var_gth = assign106440_e158571;
        var_gth_dn0 = assign106440_e158571_d_n0;
        var_gth_dn2 = assign106440_e158571_d_n2;
        var_gth_dn4 = assign106440_e158571_d_n4;
        var_gth_dn5 = assign106440_e158571_d_n5;
        var_gth_dn6 = assign106440_e158571_d_n6;
        var_gth_dn7 = assign106440_e158571_d_n7;
        var_gth_dn8 = assign106440_e158571_d_n8;
        var_gth_dn9 = assign106440_e158571_d_n9;
        var_gth_dn10 = assign106440_e158571_d_n10;
        var_gth_dn11 = assign106440_e158571_d_n11;
        var_gth_dn14 = assign106440_e158571_d_n14;

        let (assign106450_e158580, assign106450_e158580_d_n0, assign106450_e158580_d_n2, assign106450_e158580_d_n4, assign106450_e158580_d_n5, assign106450_e158580_d_n6, assign106450_e158580_d_n7, assign106450_e158580_d_n8, assign106450_e158580_d_n9, assign106450_e158580_d_n10, assign106450_e158580_d_n11, assign106450_e158580_d_n14,) = {
    if ((var_guard2404 != 0.0) && (var_guard2405 == 0.0)) {
        let assign106450_e158578: f64 = (1.0 / 0.0001);
        (assign106450_e158578, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_gth, var_gth_dn0, var_gth_dn2, var_gth_dn4, var_gth_dn5, var_gth_dn6, var_gth_dn7, var_gth_dn8, var_gth_dn9, var_gth_dn10, var_gth_dn11, var_gth_dn14,)
    }
};
        var_gth = assign106450_e158580;
        var_gth_dn0 = assign106450_e158580_d_n0;
        var_gth_dn2 = assign106450_e158580_d_n2;
        var_gth_dn4 = assign106450_e158580_d_n4;
        var_gth_dn5 = assign106450_e158580_d_n5;
        var_gth_dn6 = assign106450_e158580_d_n6;
        var_gth_dn7 = assign106450_e158580_d_n7;
        var_gth_dn8 = assign106450_e158580_d_n8;
        var_gth_dn9 = assign106450_e158580_d_n9;
        var_gth_dn10 = assign106450_e158580_d_n10;
        var_gth_dn11 = assign106450_e158580_d_n11;
        var_gth_dn14 = assign106450_e158580_d_n14;

        let assign106460_e158584: f64 = (var_vdsei - var_vdsi);
        let assign106460_e158585: f64 = (var_vdsi * assign106460_e158584);
        let assign106460_e158587: f64 = if assign106460_e158585 >= 0.0 { 1.0 } else { 0.0 };
        var_guard2406 = assign106460_e158587;

        let assign106470_e158590: f64 = if var_uc_powrat == 1.0 { 1.0 } else { 0.0 };
        var_guard2407 = assign106470_e158590;

        let (assign106480_e158598, assign106480_e158598_d_n0, assign106480_e158598_d_n2, assign106480_e158598_d_n4, assign106480_e158598_d_n5, assign106480_e158598_d_n6, assign106480_e158598_d_n7, assign106480_e158598_d_n8, assign106480_e158598_d_n9, assign106480_e158598_d_n10, assign106480_e158598_d_n11, assign106480_e158598_d_n14,) = {
    if (((var_guard2404 != 0.0) && (var_guard2406 != 0.0)) && (var_guard2407 != 0.0)) {
        (var_vdsei, var_vdsei_dn0, var_vdsei_dn2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_veffpower, var_veffpower_dn0, var_veffpower_dn2, var_veffpower_dn4, var_veffpower_dn5, var_veffpower_dn6, var_veffpower_dn7, var_veffpower_dn8, var_veffpower_dn9, var_veffpower_dn10, var_veffpower_dn11, var_veffpower_dn14,)
    }
};
        var_veffpower = assign106480_e158598;
        var_veffpower_dn0 = assign106480_e158598_d_n0;
        var_veffpower_dn2 = assign106480_e158598_d_n2;
        var_veffpower_dn4 = assign106480_e158598_d_n4;
        var_veffpower_dn5 = assign106480_e158598_d_n5;
        var_veffpower_dn6 = assign106480_e158598_d_n6;
        var_veffpower_dn7 = assign106480_e158598_d_n7;
        var_veffpower_dn8 = assign106480_e158598_d_n8;
        var_veffpower_dn9 = assign106480_e158598_d_n9;
        var_veffpower_dn10 = assign106480_e158598_d_n10;
        var_veffpower_dn11 = assign106480_e158598_d_n11;
        var_veffpower_dn14 = assign106480_e158598_d_n14;

        let (assign106490_e158613, assign106490_e158613_d_n0, assign106490_e158613_d_n2, assign106490_e158613_d_n4, assign106490_e158613_d_n5, assign106490_e158613_d_n6, assign106490_e158613_d_n7, assign106490_e158613_d_n8, assign106490_e158613_d_n9, assign106490_e158613_d_n10, assign106490_e158613_d_n11, assign106490_e158613_d_n14,) = {
    if (((var_guard2404 != 0.0) && (var_guard2406 != 0.0)) && (var_guard2407 == 0.0)) {
        let assign106490_e158609: f64 = (var_vdsei - var_vdsi);
        let assign106490_e158610: f64 = (var_powratio * assign106490_e158609);
        let assign106490_e158611: f64 = (var_vdsi + assign106490_e158610);
        (assign106490_e158611, ((var_powratio_dn0 * assign106490_e158609) + (var_powratio * var_vdsei_dn0)), ((var_powratio_dn2 * assign106490_e158609) + (var_powratio * var_vdsei_dn2)), (var_powratio_dn4 * assign106490_e158609), (var_powratio_dn5 * assign106490_e158609), (var_vdsi_dn6 + ((var_powratio_dn6 * assign106490_e158609) + (var_powratio * (-var_vdsi_dn6)))), (var_powratio_dn7 * assign106490_e158609), (var_vdsi_dn8 + ((var_powratio_dn8 * assign106490_e158609) + (var_powratio * (-var_vdsi_dn8)))), (var_powratio_dn9 * assign106490_e158609), (var_powratio_dn10 * assign106490_e158609), (var_powratio_dn11 * assign106490_e158609), (var_powratio_dn14 * assign106490_e158609),)
    } else {
        (var_veffpower, var_veffpower_dn0, var_veffpower_dn2, var_veffpower_dn4, var_veffpower_dn5, var_veffpower_dn6, var_veffpower_dn7, var_veffpower_dn8, var_veffpower_dn9, var_veffpower_dn10, var_veffpower_dn11, var_veffpower_dn14,)
    }
};
        var_veffpower = assign106490_e158613;
        var_veffpower_dn0 = assign106490_e158613_d_n0;
        var_veffpower_dn2 = assign106490_e158613_d_n2;
        var_veffpower_dn4 = assign106490_e158613_d_n4;
        var_veffpower_dn5 = assign106490_e158613_d_n5;
        var_veffpower_dn6 = assign106490_e158613_d_n6;
        var_veffpower_dn7 = assign106490_e158613_d_n7;
        var_veffpower_dn8 = assign106490_e158613_d_n8;
        var_veffpower_dn9 = assign106490_e158613_d_n9;
        var_veffpower_dn10 = assign106490_e158613_d_n10;
        var_veffpower_dn11 = assign106490_e158613_d_n11;
        var_veffpower_dn14 = assign106490_e158613_d_n14;

        let (assign106500_e158620, assign106500_e158620_d_n0, assign106500_e158620_d_n2, assign106500_e158620_d_n4, assign106500_e158620_d_n5, assign106500_e158620_d_n6, assign106500_e158620_d_n7, assign106500_e158620_d_n8, assign106500_e158620_d_n9, assign106500_e158620_d_n10, assign106500_e158620_d_n11, assign106500_e158620_d_n14,) = {
    if ((var_guard2404 != 0.0) && (var_guard2406 == 0.0)) {
        (var_vdsi, 0.0, 0.0, 0.0, 0.0, var_vdsi_dn6, 0.0, var_vdsi_dn8, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_veffpower, var_veffpower_dn0, var_veffpower_dn2, var_veffpower_dn4, var_veffpower_dn5, var_veffpower_dn6, var_veffpower_dn7, var_veffpower_dn8, var_veffpower_dn9, var_veffpower_dn10, var_veffpower_dn11, var_veffpower_dn14,)
    }
};
        var_veffpower = assign106500_e158620;
        var_veffpower_dn0 = assign106500_e158620_d_n0;
        var_veffpower_dn2 = assign106500_e158620_d_n2;
        var_veffpower_dn4 = assign106500_e158620_d_n4;
        var_veffpower_dn5 = assign106500_e158620_d_n5;
        var_veffpower_dn6 = assign106500_e158620_d_n6;
        var_veffpower_dn7 = assign106500_e158620_d_n7;
        var_veffpower_dn8 = assign106500_e158620_d_n8;
        var_veffpower_dn9 = assign106500_e158620_d_n9;
        var_veffpower_dn10 = assign106500_e158620_d_n10;
        var_veffpower_dn11 = assign106500_e158620_d_n11;
        var_veffpower_dn14 = assign106500_e158620_d_n14;

        let (assign106510_e158626, assign106510_e158626_d_n0, assign106510_e158626_d_n2, assign106510_e158626_d_n4, assign106510_e158626_d_n5, assign106510_e158626_d_n6, assign106510_e158626_d_n7, assign106510_e158626_d_n8, assign106510_e158626_d_n9, assign106510_e158626_d_n10, assign106510_e158626_d_n11, assign106510_e158626_d_n14,) = {
    if (var_guard2404 != 0.0) {
        let assign106510_e158624: f64 = (var_ids * var_veffpower);
        (assign106510_e158624, ((var_ids_dn0 * var_veffpower) + (var_ids * var_veffpower_dn0)), ((var_ids_dn2 * var_veffpower) + (var_ids * var_veffpower_dn2)), ((var_ids_dn4 * var_veffpower) + (var_ids * var_veffpower_dn4)), ((var_ids_dn5 * var_veffpower) + (var_ids * var_veffpower_dn5)), ((var_ids_dn6 * var_veffpower) + (var_ids * var_veffpower_dn6)), ((var_ids_dn7 * var_veffpower) + (var_ids * var_veffpower_dn7)), ((var_ids_dn8 * var_veffpower) + (var_ids * var_veffpower_dn8)), ((var_ids_dn9 * var_veffpower) + (var_ids * var_veffpower_dn9)), ((var_ids_dn10 * var_veffpower) + (var_ids * var_veffpower_dn10)), ((var_ids_dn11 * var_veffpower) + (var_ids * var_veffpower_dn11)), ((var_ids_dn14 * var_veffpower) + (var_ids * var_veffpower_dn14)),)
    } else {
        (var_p, var_p_dn0, var_p_dn2, var_p_dn4, var_p_dn5, var_p_dn6, var_p_dn7, var_p_dn8, var_p_dn9, var_p_dn10, var_p_dn11, var_p_dn14,)
    }
};
        var_p = assign106510_e158626;
        var_p_dn0 = assign106510_e158626_d_n0;
        var_p_dn2 = assign106510_e158626_d_n2;
        var_p_dn4 = assign106510_e158626_d_n4;
        var_p_dn5 = assign106510_e158626_d_n5;
        var_p_dn6 = assign106510_e158626_d_n6;
        var_p_dn7 = assign106510_e158626_d_n7;
        var_p_dn8 = assign106510_e158626_d_n8;
        var_p_dn9 = assign106510_e158626_d_n9;
        var_p_dn10 = assign106510_e158626_d_n10;
        var_p_dn11 = assign106510_e158626_d_n11;
        var_p_dn14 = assign106510_e158626_d_n14;

        let assign106520_e158629: f64 = if p.p53 == 1.0 { 1.0 } else { 0.0 };
        var_guard2408 = assign106520_e158629;

        let (assign106530_e158637, assign106530_e158637_d_n0, assign106530_e158637_d_n2, assign106530_e158637_d_n4, assign106530_e158637_d_n5, assign106530_e158637_d_n6, assign106530_e158637_d_n7, assign106530_e158637_d_n8, assign106530_e158637_d_n9, assign106530_e158637_d_n10, assign106530_e158637_d_n11, assign106530_e158637_d_n14,) = {
    if ((var_guard2404 != 0.0) && (var_guard2408 != 0.0)) {
        let assign106530_e158635: f64 = (p.p433 * var_gth);
        (assign106530_e158635, (p.p433 * var_gth_dn0), (p.p433 * var_gth_dn2), (p.p433 * var_gth_dn4), (p.p433 * var_gth_dn5), (p.p433 * var_gth_dn6), (p.p433 * var_gth_dn7), (p.p433 * var_gth_dn8), (p.p433 * var_gth_dn9), (p.p433 * var_gth_dn10), (p.p433 * var_gth_dn11), (p.p433 * var_gth_dn14),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign106530_e158637;
        var_t1_dn0 = assign106530_e158637_d_n0;
        var_t1_dn2 = assign106530_e158637_d_n2;
        var_t1_dn4 = assign106530_e158637_d_n4;
        var_t1_dn5 = assign106530_e158637_d_n5;
        var_t1_dn6 = assign106530_e158637_d_n6;
        var_t1_dn7 = assign106530_e158637_d_n7;
        var_t1_dn8 = assign106530_e158637_d_n8;
        var_t1_dn9 = assign106530_e158637_d_n9;
        var_t1_dn10 = assign106530_e158637_d_n10;
        var_t1_dn11 = assign106530_e158637_d_n11;
        var_t1_dn14 = assign106530_e158637_d_n14;

        let (assign106540_e158649, assign106540_e158649_d_n0, assign106540_e158649_d_n2, assign106540_e158649_d_n4, assign106540_e158649_d_n5, assign106540_e158649_d_n6, assign106540_e158649_d_n7, assign106540_e158649_d_n8, assign106540_e158649_d_n9, assign106540_e158649_d_n10, assign106540_e158649_d_n11, assign106540_e158649_d_n14,) = {
    if ((var_guard2404 != 0.0) && (var_guard2408 != 0.0)) {
        let assign106540_e158643: f64 = (var_t1 - var_p);
        let assign106540_e158646: f64 = (p.p337 * var_gth);
        let assign106540_e158647: f64 = (assign106540_e158643 - assign106540_e158646);
        (assign106540_e158647, ((var_t1_dn0 - var_p_dn0) - (p.p337 * var_gth_dn0)), ((var_t1_dn2 - var_p_dn2) - (p.p337 * var_gth_dn2)), ((var_t1_dn4 - var_p_dn4) - (p.p337 * var_gth_dn4)), ((var_t1_dn5 - var_p_dn5) - (p.p337 * var_gth_dn5)), ((var_t1_dn6 - var_p_dn6) - (p.p337 * var_gth_dn6)), ((var_t1_dn7 - var_p_dn7) - (p.p337 * var_gth_dn7)), ((var_t1_dn8 - var_p_dn8) - (p.p337 * var_gth_dn8)), ((var_t1_dn9 - var_p_dn9) - (p.p337 * var_gth_dn9)), ((var_t1_dn10 - var_p_dn10) - (p.p337 * var_gth_dn10)), ((var_t1_dn11 - var_p_dn11) - (p.p337 * var_gth_dn11)), ((var_t1_dn14 - var_p_dn14) - (p.p337 * var_gth_dn14)),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign106540_e158649;
        var_tmf1_dn0 = assign106540_e158649_d_n0;
        var_tmf1_dn2 = assign106540_e158649_d_n2;
        var_tmf1_dn4 = assign106540_e158649_d_n4;
        var_tmf1_dn5 = assign106540_e158649_d_n5;
        var_tmf1_dn6 = assign106540_e158649_d_n6;
        var_tmf1_dn7 = assign106540_e158649_d_n7;
        var_tmf1_dn8 = assign106540_e158649_d_n8;
        var_tmf1_dn9 = assign106540_e158649_d_n9;
        var_tmf1_dn10 = assign106540_e158649_d_n10;
        var_tmf1_dn11 = assign106540_e158649_d_n11;
        var_tmf1_dn14 = assign106540_e158649_d_n14;

        let (assign106550_e158661, assign106550_e158661_d_n0, assign106550_e158661_d_n2, assign106550_e158661_d_n4, assign106550_e158661_d_n5, assign106550_e158661_d_n6, assign106550_e158661_d_n7, assign106550_e158661_d_n8, assign106550_e158661_d_n9, assign106550_e158661_d_n10, assign106550_e158661_d_n11, assign106550_e158661_d_n14,) = {
    if ((var_guard2404 != 0.0) && (var_guard2408 != 0.0)) {
        let assign106550_e158655: f64 = (4.0 * var_t1);
        let assign106550_e158658: f64 = (p.p337 * var_gth);
        let assign106550_e158659: f64 = (assign106550_e158655 * assign106550_e158658);
        (assign106550_e158659, (((4.0 * var_t1_dn0) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * var_gth_dn0))), (((4.0 * var_t1_dn2) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * var_gth_dn2))), (((4.0 * var_t1_dn4) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * var_gth_dn4))), (((4.0 * var_t1_dn5) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * var_gth_dn5))), (((4.0 * var_t1_dn6) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * var_gth_dn6))), (((4.0 * var_t1_dn7) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * var_gth_dn7))), (((4.0 * var_t1_dn8) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * var_gth_dn8))), (((4.0 * var_t1_dn9) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * var_gth_dn9))), (((4.0 * var_t1_dn10) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * var_gth_dn10))), (((4.0 * var_t1_dn11) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * var_gth_dn11))), (((4.0 * var_t1_dn14) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * var_gth_dn14))),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign106550_e158661;
        var_tmf2_dn0 = assign106550_e158661_d_n0;
        var_tmf2_dn2 = assign106550_e158661_d_n2;
        var_tmf2_dn4 = assign106550_e158661_d_n4;
        var_tmf2_dn5 = assign106550_e158661_d_n5;
        var_tmf2_dn6 = assign106550_e158661_d_n6;
        var_tmf2_dn7 = assign106550_e158661_d_n7;
        var_tmf2_dn8 = assign106550_e158661_d_n8;
        var_tmf2_dn9 = assign106550_e158661_d_n9;
        var_tmf2_dn10 = assign106550_e158661_d_n10;
        var_tmf2_dn11 = assign106550_e158661_d_n11;
        var_tmf2_dn14 = assign106550_e158661_d_n14;

        let (assign106560_e158673, assign106560_e158673_d_n0, assign106560_e158673_d_n2, assign106560_e158673_d_n4, assign106560_e158673_d_n5, assign106560_e158673_d_n6, assign106560_e158673_d_n7, assign106560_e158673_d_n8, assign106560_e158673_d_n9, assign106560_e158673_d_n10, assign106560_e158673_d_n11, assign106560_e158673_d_n14,) = {
    if ((var_guard2404 != 0.0) && (var_guard2408 != 0.0)) {
        let (assign106560_e158671, assign106560_e158671_d_n0, assign106560_e158671_d_n2, assign106560_e158671_d_n4, assign106560_e158671_d_n5, assign106560_e158671_d_n6, assign106560_e158671_d_n7, assign106560_e158671_d_n8, assign106560_e158671_d_n9, assign106560_e158671_d_n10, assign106560_e158671_d_n11, assign106560_e158671_d_n14,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
            } else {
                let assign106560_e158670: f64 = (-var_tmf2);
                (assign106560_e158670, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
            }
        };
        (assign106560_e158671, assign106560_e158671_d_n0, assign106560_e158671_d_n2, assign106560_e158671_d_n4, assign106560_e158671_d_n5, assign106560_e158671_d_n6, assign106560_e158671_d_n7, assign106560_e158671_d_n8, assign106560_e158671_d_n9, assign106560_e158671_d_n10, assign106560_e158671_d_n11, assign106560_e158671_d_n14,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign106560_e158673;
        var_tmf2_dn0 = assign106560_e158673_d_n0;
        var_tmf2_dn2 = assign106560_e158673_d_n2;
        var_tmf2_dn4 = assign106560_e158673_d_n4;
        var_tmf2_dn5 = assign106560_e158673_d_n5;
        var_tmf2_dn6 = assign106560_e158673_d_n6;
        var_tmf2_dn7 = assign106560_e158673_d_n7;
        var_tmf2_dn8 = assign106560_e158673_d_n8;
        var_tmf2_dn9 = assign106560_e158673_d_n9;
        var_tmf2_dn10 = assign106560_e158673_d_n10;
        var_tmf2_dn11 = assign106560_e158673_d_n11;
        var_tmf2_dn14 = assign106560_e158673_d_n14;

        let (assign106570_e158684, assign106570_e158684_d_n0, assign106570_e158684_d_n2, assign106570_e158684_d_n4, assign106570_e158684_d_n5, assign106570_e158684_d_n6, assign106570_e158684_d_n7, assign106570_e158684_d_n8, assign106570_e158684_d_n9, assign106570_e158684_d_n10, assign106570_e158684_d_n11, assign106570_e158684_d_n14,) = {
    if ((var_guard2404 != 0.0) && (var_guard2408 != 0.0)) {
        let assign106570_e158679: f64 = (var_tmf1 * var_tmf1);
        let assign106570_e158681: f64 = (assign106570_e158679 + var_tmf2);
        let assign106570_e158682: f64 = (assign106570_e158681).sqrt();
        (assign106570_e158682, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign106570_e158682)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign106570_e158682)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign106570_e158682)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign106570_e158682)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign106570_e158682)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign106570_e158682)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign106570_e158682)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign106570_e158682)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign106570_e158682)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign106570_e158682)), ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign106570_e158682)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign106570_e158684;
        var_tmf2_dn0 = assign106570_e158684_d_n0;
        var_tmf2_dn2 = assign106570_e158684_d_n2;
        var_tmf2_dn4 = assign106570_e158684_d_n4;
        var_tmf2_dn5 = assign106570_e158684_d_n5;
        var_tmf2_dn6 = assign106570_e158684_d_n6;
        var_tmf2_dn7 = assign106570_e158684_d_n7;
        var_tmf2_dn8 = assign106570_e158684_d_n8;
        var_tmf2_dn9 = assign106570_e158684_d_n9;
        var_tmf2_dn10 = assign106570_e158684_d_n10;
        var_tmf2_dn11 = assign106570_e158684_d_n11;
        var_tmf2_dn14 = assign106570_e158684_d_n14;

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
        *var_guard2404_slot = var_guard2404;
        *var_guard2405_slot = var_guard2405;
        *var_guard2406_slot = var_guard2406;
        *var_guard2407_slot = var_guard2407;
        *var_guard2408_slot = var_guard2408;
        *var_ibjts_slot = var_ibjts;
        *var_ibjts_dn0_slot = var_ibjts_dn0;
        *var_ibjts_dn10_slot = var_ibjts_dn10;
        *var_ibjts_dn11_slot = var_ibjts_dn11;
        *var_ibjts_dn14_slot = var_ibjts_dn14;
        *var_ibjts_dn2_slot = var_ibjts_dn2;
        *var_ibjts_dn4_slot = var_ibjts_dn4;
        *var_ibjts_dn5_slot = var_ibjts_dn5;
        *var_ibjts_dn6_slot = var_ibjts_dn6;
        *var_ibjts_dn7_slot = var_ibjts_dn7;
        *var_ibjts_dn8_slot = var_ibjts_dn8;
        *var_ibjts_dn9_slot = var_ibjts_dn9;
        *var_ibreak_slot = var_ibreak;
        *var_ibreak_dn0_slot = var_ibreak_dn0;
        *var_ibreak_dn10_slot = var_ibreak_dn10;
        *var_ibreak_dn11_slot = var_ibreak_dn11;
        *var_ibreak_dn14_slot = var_ibreak_dn14;
        *var_ibreak_dn2_slot = var_ibreak_dn2;
        *var_ibreak_dn4_slot = var_ibreak_dn4;
        *var_ibreak_dn5_slot = var_ibreak_dn5;
        *var_ibreak_dn6_slot = var_ibreak_dn6;
        *var_ibreak_dn7_slot = var_ibreak_dn7;
        *var_ibreak_dn8_slot = var_ibreak_dn8;
        *var_ibreak_dn9_slot = var_ibreak_dn9;
        *var_ibreaks_slot = var_ibreaks;
        *var_ibreaks_dn0_slot = var_ibreaks_dn0;
        *var_ibreaks_dn10_slot = var_ibreaks_dn10;
        *var_ibreaks_dn11_slot = var_ibreaks_dn11;
        *var_ibreaks_dn14_slot = var_ibreaks_dn14;
        *var_ibreaks_dn2_slot = var_ibreaks_dn2;
        *var_ibreaks_dn4_slot = var_ibreaks_dn4;
        *var_ibreaks_dn5_slot = var_ibreaks_dn5;
        *var_ibreaks_dn6_slot = var_ibreaks_dn6;
        *var_ibreaks_dn7_slot = var_ibreaks_dn7;
        *var_ibreaks_dn8_slot = var_ibreaks_dn8;
        *var_ibreaks_dn9_slot = var_ibreaks_dn9;
        *var_igidl_slot = var_igidl;
        *var_igidl_dn0_slot = var_igidl_dn0;
        *var_igidl_dn10_slot = var_igidl_dn10;
        *var_igidl_dn11_slot = var_igidl_dn11;
        *var_igidl_dn14_slot = var_igidl_dn14;
        *var_igidl_dn2_slot = var_igidl_dn2;
        *var_igidl_dn4_slot = var_igidl_dn4;
        *var_igidl_dn5_slot = var_igidl_dn5;
        *var_igidl_dn6_slot = var_igidl_dn6;
        *var_igidl_dn7_slot = var_igidl_dn7;
        *var_igidl_dn8_slot = var_igidl_dn8;
        *var_igidl_dn9_slot = var_igidl_dn9;
        *var_igisl_slot = var_igisl;
        *var_igisl_dn0_slot = var_igisl_dn0;
        *var_igisl_dn10_slot = var_igisl_dn10;
        *var_igisl_dn11_slot = var_igisl_dn11;
        *var_igisl_dn14_slot = var_igisl_dn14;
        *var_igisl_dn2_slot = var_igisl_dn2;
        *var_igisl_dn4_slot = var_igisl_dn4;
        *var_igisl_dn5_slot = var_igisl_dn5;
        *var_igisl_dn6_slot = var_igisl_dn6;
        *var_igisl_dn7_slot = var_igisl_dn7;
        *var_igisl_dn8_slot = var_igisl_dn8;
        *var_igisl_dn9_slot = var_igisl_dn9;
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
        *var_qb_slot = var_qb;
        *var_qb_dn0_slot = var_qb_dn0;
        *var_qb_dn10_slot = var_qb_dn10;
        *var_qb_dn11_slot = var_qb_dn11;
        *var_qb_dn14_slot = var_qb_dn14;
        *var_qb_dn2_slot = var_qb_dn2;
        *var_qb_dn4_slot = var_qb_dn4;
        *var_qb_dn5_slot = var_qb_dn5;
        *var_qb_dn6_slot = var_qb_dn6;
        *var_qb_dn7_slot = var_qb_dn7;
        *var_qb_dn8_slot = var_qb_dn8;
        *var_qb_dn9_slot = var_qb_dn9;
        *var_qbext_slot = var_qbext;
        *var_qbext_dn0_slot = var_qbext_dn0;
        *var_qbext_dn10_slot = var_qbext_dn10;
        *var_qbext_dn11_slot = var_qbext_dn11;
        *var_qbext_dn14_slot = var_qbext_dn14;
        *var_qbext_dn2_slot = var_qbext_dn2;
        *var_qbext_dn4_slot = var_qbext_dn4;
        *var_qbext_dn5_slot = var_qbext_dn5;
        *var_qbext_dn6_slot = var_qbext_dn6;
        *var_qbext_dn7_slot = var_qbext_dn7;
        *var_qbext_dn8_slot = var_qbext_dn8;
        *var_qbext_dn9_slot = var_qbext_dn9;
        *var_qd_slot = var_qd;
        *var_qd_dn0_slot = var_qd_dn0;
        *var_qd_dn10_slot = var_qd_dn10;
        *var_qd_dn11_slot = var_qd_dn11;
        *var_qd_dn14_slot = var_qd_dn14;
        *var_qd_dn2_slot = var_qd_dn2;
        *var_qd_dn4_slot = var_qd_dn4;
        *var_qd_dn5_slot = var_qd_dn5;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn7_slot = var_qd_dn7;
        *var_qd_dn8_slot = var_qd_dn8;
        *var_qd_dn9_slot = var_qd_dn9;
        *var_qdext_slot = var_qdext;
        *var_qdext_dn0_slot = var_qdext_dn0;
        *var_qdext_dn10_slot = var_qdext_dn10;
        *var_qdext_dn11_slot = var_qdext_dn11;
        *var_qdext_dn14_slot = var_qdext_dn14;
        *var_qdext_dn2_slot = var_qdext_dn2;
        *var_qdext_dn4_slot = var_qdext_dn4;
        *var_qdext_dn5_slot = var_qdext_dn5;
        *var_qdext_dn6_slot = var_qdext_dn6;
        *var_qdext_dn7_slot = var_qdext_dn7;
        *var_qdext_dn8_slot = var_qdext_dn8;
        *var_qdext_dn9_slot = var_qdext_dn9;
        *var_qdrat_slot = var_qdrat;
        *var_qdrat_dn0_slot = var_qdrat_dn0;
        *var_qdrat_dn10_slot = var_qdrat_dn10;
        *var_qdrat_dn11_slot = var_qdrat_dn11;
        *var_qdrat_dn14_slot = var_qdrat_dn14;
        *var_qdrat_dn2_slot = var_qdrat_dn2;
        *var_qdrat_dn4_slot = var_qdrat_dn4;
        *var_qdrat_dn5_slot = var_qdrat_dn5;
        *var_qdrat_dn6_slot = var_qdrat_dn6;
        *var_qdrat_dn7_slot = var_qdrat_dn7;
        *var_qdrat_dn8_slot = var_qdrat_dn8;
        *var_qdrat_dn9_slot = var_qdrat_dn9;
        *var_qfd_slot = var_qfd;
        *var_qfd_dn0_slot = var_qfd_dn0;
        *var_qfd_dn2_slot = var_qfd_dn2;
        *var_qfd_dn7_slot = var_qfd_dn7;
        *var_qfs_slot = var_qfs;
        *var_qfs_dn2_slot = var_qfs_dn2;
        *var_qfs_dn7_slot = var_qfs_dn7;
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
        *var_qgext_slot = var_qgext;
        *var_qgext_dn0_slot = var_qgext_dn0;
        *var_qgext_dn10_slot = var_qgext_dn10;
        *var_qgext_dn11_slot = var_qgext_dn11;
        *var_qgext_dn14_slot = var_qgext_dn14;
        *var_qgext_dn2_slot = var_qgext_dn2;
        *var_qgext_dn4_slot = var_qgext_dn4;
        *var_qgext_dn5_slot = var_qgext_dn5;
        *var_qgext_dn6_slot = var_qgext_dn6;
        *var_qgext_dn7_slot = var_qgext_dn7;
        *var_qgext_dn8_slot = var_qgext_dn8;
        *var_qgext_dn9_slot = var_qgext_dn9;
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
    }

    pub(super) fn stamp_transient_block_391(
        p: &Parameters,
        var_flg_nqs: f64,
        var_guard2404: f64,
        var_guard2408: f64,
        var_ids: f64,
        var_ids_dn0: f64,
        var_ids_dn10: f64,
        var_ids_dn11: f64,
        var_ids_dn14: f64,
        var_ids_dn2: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_ids_dn9: f64,
        var_mode: f64,
        var_qb_nqs: f64,
        var_qb_nqs_dn13: f64,
        var_qbulk: f64,
        var_qbulk_dn0: f64,
        var_qbulk_dn10: f64,
        var_qbulk_dn11: f64,
        var_qbulk_dn14: f64,
        var_qbulk_dn2: f64,
        var_qbulk_dn4: f64,
        var_qbulk_dn5: f64,
        var_qbulk_dn6: f64,
        var_qbulk_dn7: f64,
        var_qbulk_dn8: f64,
        var_qbulk_dn9: f64,
        var_qdrat: f64,
        var_qdrat_dn0: f64,
        var_qdrat_dn10: f64,
        var_qdrat_dn11: f64,
        var_qdrat_dn14: f64,
        var_qdrat_dn2: f64,
        var_qdrat_dn4: f64,
        var_qdrat_dn5: f64,
        var_qdrat_dn6: f64,
        var_qdrat_dn7: f64,
        var_qdrat_dn8: f64,
        var_qdrat_dn9: f64,
        var_qg_dn6: f64,
        var_qg_dn8: f64,
        var_qi: f64,
        var_qi_dn0: f64,
        var_qi_dn10: f64,
        var_qi_dn11: f64,
        var_qi_dn14: f64,
        var_qi_dn2: f64,
        var_qi_dn4: f64,
        var_qi_dn5: f64,
        var_qi_dn6: f64,
        var_qi_dn7: f64,
        var_qi_dn8: f64,
        var_qi_dn9: f64,
        var_qi_nqs: f64,
        var_qi_nqs_dn12: f64,
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
        var_tmf1: f64,
        var_tmf1_dn0: f64,
        var_tmf1_dn10: f64,
        var_tmf1_dn11: f64,
        var_tmf1_dn14: f64,
        var_tmf1_dn2: f64,
        var_tmf1_dn4: f64,
        var_tmf1_dn5: f64,
        var_tmf1_dn6: f64,
        var_tmf1_dn7: f64,
        var_tmf1_dn8: f64,
        var_tmf1_dn9: f64,
        var_tmf2: f64,
        var_tmf2_dn0: f64,
        var_tmf2_dn10: f64,
        var_tmf2_dn11: f64,
        var_tmf2_dn14: f64,
        var_tmf2_dn2: f64,
        var_tmf2_dn4: f64,
        var_tmf2_dn5: f64,
        var_tmf2_dn6: f64,
        var_tmf2_dn7: f64,
        var_tmf2_dn8: f64,
        var_tmf2_dn9: f64,
        var_cgdbd_slot: &mut f64,
        var_cgdbd_dn0_slot: &mut f64,
        var_cgdbd_dn10_slot: &mut f64,
        var_cgdbd_dn11_slot: &mut f64,
        var_cgdbd_dn14_slot: &mut f64,
        var_cgdbd_dn2_slot: &mut f64,
        var_cgdbd_dn4_slot: &mut f64,
        var_cgdbd_dn5_slot: &mut f64,
        var_cgdbd_dn6_slot: &mut f64,
        var_cgdbd_dn7_slot: &mut f64,
        var_cgdbd_dn8_slot: &mut f64,
        var_cgdbd_dn9_slot: &mut f64,
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
        var_cgsbd_slot: &mut f64,
        var_cgsbd_dn0_slot: &mut f64,
        var_cgsbd_dn10_slot: &mut f64,
        var_cgsbd_dn11_slot: &mut f64,
        var_cgsbd_dn14_slot: &mut f64,
        var_cgsbd_dn2_slot: &mut f64,
        var_cgsbd_dn4_slot: &mut f64,
        var_cgsbd_dn5_slot: &mut f64,
        var_cgsbd_dn6_slot: &mut f64,
        var_cgsbd_dn7_slot: &mut f64,
        var_cgsbd_dn8_slot: &mut f64,
        var_cgsbd_dn9_slot: &mut f64,
        var_cqb_slot: &mut f64,
        var_cqi_slot: &mut f64,
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
        var_guard2409_slot: &mut f64,
        var_guard2410_slot: &mut f64,
        var_guard2411_slot: &mut f64,
        var_guard2413_slot: &mut f64,
        var_guard2414_slot: &mut f64,
        var_guard2417_slot: &mut f64,
        var_idse_slot: &mut f64,
        var_idse_dn0_slot: &mut f64,
        var_idse_dn10_slot: &mut f64,
        var_idse_dn11_slot: &mut f64,
        var_idse_dn14_slot: &mut f64,
        var_idse_dn2_slot: &mut f64,
        var_idse_dn4_slot: &mut f64,
        var_idse_dn5_slot: &mut f64,
        var_idse_dn6_slot: &mut f64,
        var_idse_dn7_slot: &mut f64,
        var_idse_dn8_slot: &mut f64,
        var_idse_dn9_slot: &mut f64,
        var_iqb_nqs_slot: &mut f64,
        var_iqb_nqs_dn0_slot: &mut f64,
        var_iqb_nqs_dn10_slot: &mut f64,
        var_iqb_nqs_dn11_slot: &mut f64,
        var_iqb_nqs_dn13_slot: &mut f64,
        var_iqb_nqs_dn14_slot: &mut f64,
        var_iqb_nqs_dn2_slot: &mut f64,
        var_iqb_nqs_dn4_slot: &mut f64,
        var_iqb_nqs_dn5_slot: &mut f64,
        var_iqb_nqs_dn6_slot: &mut f64,
        var_iqb_nqs_dn7_slot: &mut f64,
        var_iqb_nqs_dn8_slot: &mut f64,
        var_iqb_nqs_dn9_slot: &mut f64,
        var_iqi_nqs_slot: &mut f64,
        var_iqi_nqs_dn0_slot: &mut f64,
        var_iqi_nqs_dn10_slot: &mut f64,
        var_iqi_nqs_dn11_slot: &mut f64,
        var_iqi_nqs_dn12_slot: &mut f64,
        var_iqi_nqs_dn14_slot: &mut f64,
        var_iqi_nqs_dn2_slot: &mut f64,
        var_iqi_nqs_dn4_slot: &mut f64,
        var_iqi_nqs_dn5_slot: &mut f64,
        var_iqi_nqs_dn6_slot: &mut f64,
        var_iqi_nqs_dn7_slot: &mut f64,
        var_iqi_nqs_dn8_slot: &mut f64,
        var_iqi_nqs_dn9_slot: &mut f64,
        var_itemp_slot: &mut f64,
        var_itemp_dn0_slot: &mut f64,
        var_itemp_dn10_slot: &mut f64,
        var_itemp_dn11_slot: &mut f64,
        var_itemp_dn14_slot: &mut f64,
        var_itemp_dn2_slot: &mut f64,
        var_itemp_dn4_slot: &mut f64,
        var_itemp_dn5_slot: &mut f64,
        var_itemp_dn6_slot: &mut f64,
        var_itemp_dn7_slot: &mut f64,
        var_itemp_dn8_slot: &mut f64,
        var_itemp_dn9_slot: &mut f64,
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
        var_qg_nqs_slot: &mut f64,
        var_qg_nqs_dn12_slot: &mut f64,
        var_qg_nqs_dn13_slot: &mut f64,
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
        var_tau_slot: &mut f64,
        var_tau_dn0_slot: &mut f64,
        var_tau_dn10_slot: &mut f64,
        var_tau_dn11_slot: &mut f64,
        var_tau_dn14_slot: &mut f64,
        var_tau_dn2_slot: &mut f64,
        var_tau_dn4_slot: &mut f64,
        var_tau_dn5_slot: &mut f64,
        var_tau_dn6_slot: &mut f64,
        var_tau_dn7_slot: &mut f64,
        var_tau_dn8_slot: &mut f64,
        var_tau_dn9_slot: &mut f64,
        var_taub_slot: &mut f64,
        var_taub_dn0_slot: &mut f64,
        var_taub_dn10_slot: &mut f64,
        var_taub_dn11_slot: &mut f64,
        var_taub_dn14_slot: &mut f64,
        var_taub_dn2_slot: &mut f64,
        var_taub_dn4_slot: &mut f64,
        var_taub_dn5_slot: &mut f64,
        var_taub_dn6_slot: &mut f64,
        var_taub_dn7_slot: &mut f64,
        var_taub_dn8_slot: &mut f64,
        var_taub_dn9_slot: &mut f64,
    ) {
        let mut var_cgdbd: f64 = *var_cgdbd_slot;
        let mut var_cgdbd_dn0: f64 = *var_cgdbd_dn0_slot;
        let mut var_cgdbd_dn10: f64 = *var_cgdbd_dn10_slot;
        let mut var_cgdbd_dn11: f64 = *var_cgdbd_dn11_slot;
        let mut var_cgdbd_dn14: f64 = *var_cgdbd_dn14_slot;
        let mut var_cgdbd_dn2: f64 = *var_cgdbd_dn2_slot;
        let mut var_cgdbd_dn4: f64 = *var_cgdbd_dn4_slot;
        let mut var_cgdbd_dn5: f64 = *var_cgdbd_dn5_slot;
        let mut var_cgdbd_dn6: f64 = *var_cgdbd_dn6_slot;
        let mut var_cgdbd_dn7: f64 = *var_cgdbd_dn7_slot;
        let mut var_cgdbd_dn8: f64 = *var_cgdbd_dn8_slot;
        let mut var_cgdbd_dn9: f64 = *var_cgdbd_dn9_slot;
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
        let mut var_cgsbd: f64 = *var_cgsbd_slot;
        let mut var_cgsbd_dn0: f64 = *var_cgsbd_dn0_slot;
        let mut var_cgsbd_dn10: f64 = *var_cgsbd_dn10_slot;
        let mut var_cgsbd_dn11: f64 = *var_cgsbd_dn11_slot;
        let mut var_cgsbd_dn14: f64 = *var_cgsbd_dn14_slot;
        let mut var_cgsbd_dn2: f64 = *var_cgsbd_dn2_slot;
        let mut var_cgsbd_dn4: f64 = *var_cgsbd_dn4_slot;
        let mut var_cgsbd_dn5: f64 = *var_cgsbd_dn5_slot;
        let mut var_cgsbd_dn6: f64 = *var_cgsbd_dn6_slot;
        let mut var_cgsbd_dn7: f64 = *var_cgsbd_dn7_slot;
        let mut var_cgsbd_dn8: f64 = *var_cgsbd_dn8_slot;
        let mut var_cgsbd_dn9: f64 = *var_cgsbd_dn9_slot;
        let mut var_cqb: f64 = *var_cqb_slot;
        let mut var_cqi: f64 = *var_cqi_slot;
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
        let mut var_guard2409: f64 = *var_guard2409_slot;
        let mut var_guard2410: f64 = *var_guard2410_slot;
        let mut var_guard2411: f64 = *var_guard2411_slot;
        let mut var_guard2413: f64 = *var_guard2413_slot;
        let mut var_guard2414: f64 = *var_guard2414_slot;
        let mut var_guard2417: f64 = *var_guard2417_slot;
        let mut var_idse: f64 = *var_idse_slot;
        let mut var_idse_dn0: f64 = *var_idse_dn0_slot;
        let mut var_idse_dn10: f64 = *var_idse_dn10_slot;
        let mut var_idse_dn11: f64 = *var_idse_dn11_slot;
        let mut var_idse_dn14: f64 = *var_idse_dn14_slot;
        let mut var_idse_dn2: f64 = *var_idse_dn2_slot;
        let mut var_idse_dn4: f64 = *var_idse_dn4_slot;
        let mut var_idse_dn5: f64 = *var_idse_dn5_slot;
        let mut var_idse_dn6: f64 = *var_idse_dn6_slot;
        let mut var_idse_dn7: f64 = *var_idse_dn7_slot;
        let mut var_idse_dn8: f64 = *var_idse_dn8_slot;
        let mut var_idse_dn9: f64 = *var_idse_dn9_slot;
        let mut var_iqb_nqs: f64 = *var_iqb_nqs_slot;
        let mut var_iqb_nqs_dn0: f64 = *var_iqb_nqs_dn0_slot;
        let mut var_iqb_nqs_dn10: f64 = *var_iqb_nqs_dn10_slot;
        let mut var_iqb_nqs_dn11: f64 = *var_iqb_nqs_dn11_slot;
        let mut var_iqb_nqs_dn13: f64 = *var_iqb_nqs_dn13_slot;
        let mut var_iqb_nqs_dn14: f64 = *var_iqb_nqs_dn14_slot;
        let mut var_iqb_nqs_dn2: f64 = *var_iqb_nqs_dn2_slot;
        let mut var_iqb_nqs_dn4: f64 = *var_iqb_nqs_dn4_slot;
        let mut var_iqb_nqs_dn5: f64 = *var_iqb_nqs_dn5_slot;
        let mut var_iqb_nqs_dn6: f64 = *var_iqb_nqs_dn6_slot;
        let mut var_iqb_nqs_dn7: f64 = *var_iqb_nqs_dn7_slot;
        let mut var_iqb_nqs_dn8: f64 = *var_iqb_nqs_dn8_slot;
        let mut var_iqb_nqs_dn9: f64 = *var_iqb_nqs_dn9_slot;
        let mut var_iqi_nqs: f64 = *var_iqi_nqs_slot;
        let mut var_iqi_nqs_dn0: f64 = *var_iqi_nqs_dn0_slot;
        let mut var_iqi_nqs_dn10: f64 = *var_iqi_nqs_dn10_slot;
        let mut var_iqi_nqs_dn11: f64 = *var_iqi_nqs_dn11_slot;
        let mut var_iqi_nqs_dn12: f64 = *var_iqi_nqs_dn12_slot;
        let mut var_iqi_nqs_dn14: f64 = *var_iqi_nqs_dn14_slot;
        let mut var_iqi_nqs_dn2: f64 = *var_iqi_nqs_dn2_slot;
        let mut var_iqi_nqs_dn4: f64 = *var_iqi_nqs_dn4_slot;
        let mut var_iqi_nqs_dn5: f64 = *var_iqi_nqs_dn5_slot;
        let mut var_iqi_nqs_dn6: f64 = *var_iqi_nqs_dn6_slot;
        let mut var_iqi_nqs_dn7: f64 = *var_iqi_nqs_dn7_slot;
        let mut var_iqi_nqs_dn8: f64 = *var_iqi_nqs_dn8_slot;
        let mut var_iqi_nqs_dn9: f64 = *var_iqi_nqs_dn9_slot;
        let mut var_itemp: f64 = *var_itemp_slot;
        let mut var_itemp_dn0: f64 = *var_itemp_dn0_slot;
        let mut var_itemp_dn10: f64 = *var_itemp_dn10_slot;
        let mut var_itemp_dn11: f64 = *var_itemp_dn11_slot;
        let mut var_itemp_dn14: f64 = *var_itemp_dn14_slot;
        let mut var_itemp_dn2: f64 = *var_itemp_dn2_slot;
        let mut var_itemp_dn4: f64 = *var_itemp_dn4_slot;
        let mut var_itemp_dn5: f64 = *var_itemp_dn5_slot;
        let mut var_itemp_dn6: f64 = *var_itemp_dn6_slot;
        let mut var_itemp_dn7: f64 = *var_itemp_dn7_slot;
        let mut var_itemp_dn8: f64 = *var_itemp_dn8_slot;
        let mut var_itemp_dn9: f64 = *var_itemp_dn9_slot;
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
        let mut var_qg_nqs: f64 = *var_qg_nqs_slot;
        let mut var_qg_nqs_dn12: f64 = *var_qg_nqs_dn12_slot;
        let mut var_qg_nqs_dn13: f64 = *var_qg_nqs_dn13_slot;
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
        let mut var_tau: f64 = *var_tau_slot;
        let mut var_tau_dn0: f64 = *var_tau_dn0_slot;
        let mut var_tau_dn10: f64 = *var_tau_dn10_slot;
        let mut var_tau_dn11: f64 = *var_tau_dn11_slot;
        let mut var_tau_dn14: f64 = *var_tau_dn14_slot;
        let mut var_tau_dn2: f64 = *var_tau_dn2_slot;
        let mut var_tau_dn4: f64 = *var_tau_dn4_slot;
        let mut var_tau_dn5: f64 = *var_tau_dn5_slot;
        let mut var_tau_dn6: f64 = *var_tau_dn6_slot;
        let mut var_tau_dn7: f64 = *var_tau_dn7_slot;
        let mut var_tau_dn8: f64 = *var_tau_dn8_slot;
        let mut var_tau_dn9: f64 = *var_tau_dn9_slot;
        let mut var_taub: f64 = *var_taub_slot;
        let mut var_taub_dn0: f64 = *var_taub_dn0_slot;
        let mut var_taub_dn10: f64 = *var_taub_dn10_slot;
        let mut var_taub_dn11: f64 = *var_taub_dn11_slot;
        let mut var_taub_dn14: f64 = *var_taub_dn14_slot;
        let mut var_taub_dn2: f64 = *var_taub_dn2_slot;
        let mut var_taub_dn4: f64 = *var_taub_dn4_slot;
        let mut var_taub_dn5: f64 = *var_taub_dn5_slot;
        let mut var_taub_dn6: f64 = *var_taub_dn6_slot;
        let mut var_taub_dn7: f64 = *var_taub_dn7_slot;
        let mut var_taub_dn8: f64 = *var_taub_dn8_slot;
        let mut var_taub_dn9: f64 = *var_taub_dn9_slot;

        let (assign106580_e158696, assign106580_e158696_d_n0, assign106580_e158696_d_n2, assign106580_e158696_d_n4, assign106580_e158696_d_n5, assign106580_e158696_d_n6, assign106580_e158696_d_n7, assign106580_e158696_d_n8, assign106580_e158696_d_n9, assign106580_e158696_d_n10, assign106580_e158696_d_n11, assign106580_e158696_d_n14,) = {
    if ((var_guard2404 != 0.0) && (var_guard2408 != 0.0)) {
        let assign106580_e158692: f64 = (var_tmf1 / var_tmf2);
        let assign106580_e158693: f64 = (1.0 + assign106580_e158692);
        let assign106580_e158694: f64 = (0.5 * assign106580_e158693);
        (assign106580_e158694, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign106580_e158696;
        var_t0_dn0 = assign106580_e158696_d_n0;
        var_t0_dn2 = assign106580_e158696_d_n2;
        var_t0_dn4 = assign106580_e158696_d_n4;
        var_t0_dn5 = assign106580_e158696_d_n5;
        var_t0_dn6 = assign106580_e158696_d_n6;
        var_t0_dn7 = assign106580_e158696_d_n7;
        var_t0_dn8 = assign106580_e158696_d_n8;
        var_t0_dn9 = assign106580_e158696_d_n9;
        var_t0_dn10 = assign106580_e158696_d_n10;
        var_t0_dn11 = assign106580_e158696_d_n11;
        var_t0_dn14 = assign106580_e158696_d_n14;

        let (assign106590_e158708, assign106590_e158708_d_n0, assign106590_e158708_d_n2, assign106590_e158708_d_n4, assign106590_e158708_d_n5, assign106590_e158708_d_n6, assign106590_e158708_d_n7, assign106590_e158708_d_n8, assign106590_e158708_d_n9, assign106590_e158708_d_n10, assign106590_e158708_d_n11, assign106590_e158708_d_n14,) = {
    if ((var_guard2404 != 0.0) && (var_guard2408 != 0.0)) {
        let assign106590_e158704: f64 = (var_tmf1 + var_tmf2);
        let assign106590_e158705: f64 = (0.5 * assign106590_e158704);
        let assign106590_e158706: f64 = (var_t1 - assign106590_e158705);
        (assign106590_e158706, (var_t1_dn0 - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_t1_dn2 - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_t1_dn4 - (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_t1_dn5 - (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_t1_dn6 - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (var_t1_dn7 - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (var_t1_dn8 - (0.5 * (var_tmf1_dn8 + var_tmf2_dn8))), (var_t1_dn9 - (0.5 * (var_tmf1_dn9 + var_tmf2_dn9))), (var_t1_dn10 - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (var_t1_dn11 - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (var_t1_dn14 - (0.5 * (var_tmf1_dn14 + var_tmf2_dn14))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign106590_e158708;
        var_t2_dn0 = assign106590_e158708_d_n0;
        var_t2_dn2 = assign106590_e158708_d_n2;
        var_t2_dn4 = assign106590_e158708_d_n4;
        var_t2_dn5 = assign106590_e158708_d_n5;
        var_t2_dn6 = assign106590_e158708_d_n6;
        var_t2_dn7 = assign106590_e158708_d_n7;
        var_t2_dn8 = assign106590_e158708_d_n8;
        var_t2_dn9 = assign106590_e158708_d_n9;
        var_t2_dn10 = assign106590_e158708_d_n10;
        var_t2_dn11 = assign106590_e158708_d_n11;
        var_t2_dn14 = assign106590_e158708_d_n14;

        let (assign106600_e158714, assign106600_e158714_d_n0, assign106600_e158714_d_n2, assign106600_e158714_d_n4, assign106600_e158714_d_n5, assign106600_e158714_d_n6, assign106600_e158714_d_n7, assign106600_e158714_d_n8, assign106600_e158714_d_n9, assign106600_e158714_d_n10, assign106600_e158714_d_n11, assign106600_e158714_d_n14,) = {
    if ((var_guard2404 != 0.0) && (var_guard2408 != 0.0)) {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    } else {
        (var_p, var_p_dn0, var_p_dn2, var_p_dn4, var_p_dn5, var_p_dn6, var_p_dn7, var_p_dn8, var_p_dn9, var_p_dn10, var_p_dn11, var_p_dn14,)
    }
};
        var_p = assign106600_e158714;
        var_p_dn0 = assign106600_e158714_d_n0;
        var_p_dn2 = assign106600_e158714_d_n2;
        var_p_dn4 = assign106600_e158714_d_n4;
        var_p_dn5 = assign106600_e158714_d_n5;
        var_p_dn6 = assign106600_e158714_d_n6;
        var_p_dn7 = assign106600_e158714_d_n7;
        var_p_dn8 = assign106600_e158714_d_n8;
        var_p_dn9 = assign106600_e158714_d_n9;
        var_p_dn10 = assign106600_e158714_d_n10;
        var_p_dn11 = assign106600_e158714_d_n11;
        var_p_dn14 = assign106600_e158714_d_n14;

        let (assign106610_e158719, assign106610_e158719_d_n0, assign106610_e158719_d_n2, assign106610_e158719_d_n4, assign106610_e158719_d_n5, assign106610_e158719_d_n6, assign106610_e158719_d_n7, assign106610_e158719_d_n8, assign106610_e158719_d_n9, assign106610_e158719_d_n10, assign106610_e158719_d_n11, assign106610_e158719_d_n14,) = {
    if (var_guard2404 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_gth, var_gth_dn0, var_gth_dn2, var_gth_dn4, var_gth_dn5, var_gth_dn6, var_gth_dn7, var_gth_dn8, var_gth_dn9, var_gth_dn10, var_gth_dn11, var_gth_dn14,)
    }
};
        var_gth = assign106610_e158719;
        var_gth_dn0 = assign106610_e158719_d_n0;
        var_gth_dn2 = assign106610_e158719_d_n2;
        var_gth_dn4 = assign106610_e158719_d_n4;
        var_gth_dn5 = assign106610_e158719_d_n5;
        var_gth_dn6 = assign106610_e158719_d_n6;
        var_gth_dn7 = assign106610_e158719_d_n7;
        var_gth_dn8 = assign106610_e158719_d_n8;
        var_gth_dn9 = assign106610_e158719_d_n9;
        var_gth_dn10 = assign106610_e158719_d_n10;
        var_gth_dn11 = assign106610_e158719_d_n11;
        var_gth_dn14 = assign106610_e158719_d_n14;

        let (assign106620_e158724, assign106620_e158724_d_n0, assign106620_e158724_d_n2, assign106620_e158724_d_n4, assign106620_e158724_d_n5, assign106620_e158724_d_n6, assign106620_e158724_d_n7, assign106620_e158724_d_n8, assign106620_e158724_d_n9, assign106620_e158724_d_n10, assign106620_e158724_d_n11, assign106620_e158724_d_n14,) = {
    if (var_guard2404 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_p, var_p_dn0, var_p_dn2, var_p_dn4, var_p_dn5, var_p_dn6, var_p_dn7, var_p_dn8, var_p_dn9, var_p_dn10, var_p_dn11, var_p_dn14,)
    }
};
        var_p = assign106620_e158724;
        var_p_dn0 = assign106620_e158724_d_n0;
        var_p_dn2 = assign106620_e158724_d_n2;
        var_p_dn4 = assign106620_e158724_d_n4;
        var_p_dn5 = assign106620_e158724_d_n5;
        var_p_dn6 = assign106620_e158724_d_n6;
        var_p_dn7 = assign106620_e158724_d_n7;
        var_p_dn8 = assign106620_e158724_d_n8;
        var_p_dn9 = assign106620_e158724_d_n9;
        var_p_dn10 = assign106620_e158724_d_n10;
        var_p_dn11 = assign106620_e158724_d_n11;
        var_p_dn14 = assign106620_e158724_d_n14;

        let assign106630_e158727: f64 = if var_tau < 1e-15 { 1.0 } else { 0.0 };
        var_guard2409 = assign106630_e158727;

        let (assign106640_e158733, assign106640_e158733_d_n0, assign106640_e158733_d_n2, assign106640_e158733_d_n4, assign106640_e158733_d_n5, assign106640_e158733_d_n6, assign106640_e158733_d_n7, assign106640_e158733_d_n8, assign106640_e158733_d_n9, assign106640_e158733_d_n10, assign106640_e158733_d_n11, assign106640_e158733_d_n14,) = {
    if ((var_flg_nqs != 0.0) && (var_guard2409 != 0.0)) {
        (1e-15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tau, var_tau_dn0, var_tau_dn2, var_tau_dn4, var_tau_dn5, var_tau_dn6, var_tau_dn7, var_tau_dn8, var_tau_dn9, var_tau_dn10, var_tau_dn11, var_tau_dn14,)
    }
};
        var_tau = assign106640_e158733;
        var_tau_dn0 = assign106640_e158733_d_n0;
        var_tau_dn2 = assign106640_e158733_d_n2;
        var_tau_dn4 = assign106640_e158733_d_n4;
        var_tau_dn5 = assign106640_e158733_d_n5;
        var_tau_dn6 = assign106640_e158733_d_n6;
        var_tau_dn7 = assign106640_e158733_d_n7;
        var_tau_dn8 = assign106640_e158733_d_n8;
        var_tau_dn9 = assign106640_e158733_d_n9;
        var_tau_dn10 = assign106640_e158733_d_n10;
        var_tau_dn11 = assign106640_e158733_d_n11;
        var_tau_dn14 = assign106640_e158733_d_n14;

        let assign106650_e158736: f64 = if var_taub < 1e-15 { 1.0 } else { 0.0 };
        var_guard2410 = assign106650_e158736;

        let (assign106660_e158742, assign106660_e158742_d_n0, assign106660_e158742_d_n2, assign106660_e158742_d_n4, assign106660_e158742_d_n5, assign106660_e158742_d_n6, assign106660_e158742_d_n7, assign106660_e158742_d_n8, assign106660_e158742_d_n9, assign106660_e158742_d_n10, assign106660_e158742_d_n11, assign106660_e158742_d_n14,) = {
    if ((var_flg_nqs != 0.0) && (var_guard2410 != 0.0)) {
        (1e-15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_taub, var_taub_dn0, var_taub_dn2, var_taub_dn4, var_taub_dn5, var_taub_dn6, var_taub_dn7, var_taub_dn8, var_taub_dn9, var_taub_dn10, var_taub_dn11, var_taub_dn14,)
    }
};
        var_taub = assign106660_e158742;
        var_taub_dn0 = assign106660_e158742_d_n0;
        var_taub_dn2 = assign106660_e158742_d_n2;
        var_taub_dn4 = assign106660_e158742_d_n4;
        var_taub_dn5 = assign106660_e158742_d_n5;
        var_taub_dn6 = assign106660_e158742_d_n6;
        var_taub_dn7 = assign106660_e158742_d_n7;
        var_taub_dn8 = assign106660_e158742_d_n8;
        var_taub_dn9 = assign106660_e158742_d_n9;
        var_taub_dn10 = assign106660_e158742_d_n10;
        var_taub_dn11 = assign106660_e158742_d_n11;
        var_taub_dn14 = assign106660_e158742_d_n14;

        let (assign106670_e158750, assign106670_e158750_d_n0, assign106670_e158750_d_n2, assign106670_e158750_d_n4, assign106670_e158750_d_n5, assign106670_e158750_d_n6, assign106670_e158750_d_n7, assign106670_e158750_d_n8, assign106670_e158750_d_n9, assign106670_e158750_d_n10, assign106670_e158750_d_n11, assign106670_e158750_d_n12, assign106670_e158750_d_n14,) = {
    if (var_flg_nqs != 0.0) {
        let assign106670_e158746: f64 = (var_qi_nqs - var_qi);
        let assign106670_e158748: f64 = (assign106670_e158746 / var_tau);
        (assign106670_e158748, ((((-var_qi_dn0) * var_tau) - (assign106670_e158746 * var_tau_dn0)) / (var_tau * var_tau)), ((((-var_qi_dn2) * var_tau) - (assign106670_e158746 * var_tau_dn2)) / (var_tau * var_tau)), ((((-var_qi_dn4) * var_tau) - (assign106670_e158746 * var_tau_dn4)) / (var_tau * var_tau)), ((((-var_qi_dn5) * var_tau) - (assign106670_e158746 * var_tau_dn5)) / (var_tau * var_tau)), ((((-var_qi_dn6) * var_tau) - (assign106670_e158746 * var_tau_dn6)) / (var_tau * var_tau)), ((((-var_qi_dn7) * var_tau) - (assign106670_e158746 * var_tau_dn7)) / (var_tau * var_tau)), ((((-var_qi_dn8) * var_tau) - (assign106670_e158746 * var_tau_dn8)) / (var_tau * var_tau)), ((((-var_qi_dn9) * var_tau) - (assign106670_e158746 * var_tau_dn9)) / (var_tau * var_tau)), ((((-var_qi_dn10) * var_tau) - (assign106670_e158746 * var_tau_dn10)) / (var_tau * var_tau)), ((((-var_qi_dn11) * var_tau) - (assign106670_e158746 * var_tau_dn11)) / (var_tau * var_tau)), (var_qi_nqs_dn12 / var_tau), ((((-var_qi_dn14) * var_tau) - (assign106670_e158746 * var_tau_dn14)) / (var_tau * var_tau)),)
    } else {
        (var_iqi_nqs, var_iqi_nqs_dn0, var_iqi_nqs_dn2, var_iqi_nqs_dn4, var_iqi_nqs_dn5, var_iqi_nqs_dn6, var_iqi_nqs_dn7, var_iqi_nqs_dn8, var_iqi_nqs_dn9, var_iqi_nqs_dn10, var_iqi_nqs_dn11, var_iqi_nqs_dn12, var_iqi_nqs_dn14,)
    }
};
        var_iqi_nqs = assign106670_e158750;
        var_iqi_nqs_dn0 = assign106670_e158750_d_n0;
        var_iqi_nqs_dn2 = assign106670_e158750_d_n2;
        var_iqi_nqs_dn4 = assign106670_e158750_d_n4;
        var_iqi_nqs_dn5 = assign106670_e158750_d_n5;
        var_iqi_nqs_dn6 = assign106670_e158750_d_n6;
        var_iqi_nqs_dn7 = assign106670_e158750_d_n7;
        var_iqi_nqs_dn8 = assign106670_e158750_d_n8;
        var_iqi_nqs_dn9 = assign106670_e158750_d_n9;
        var_iqi_nqs_dn10 = assign106670_e158750_d_n10;
        var_iqi_nqs_dn11 = assign106670_e158750_d_n11;
        var_iqi_nqs_dn12 = assign106670_e158750_d_n12;
        var_iqi_nqs_dn14 = assign106670_e158750_d_n14;

        let (assign106680_e158758, assign106680_e158758_d_n0, assign106680_e158758_d_n2, assign106680_e158758_d_n4, assign106680_e158758_d_n5, assign106680_e158758_d_n6, assign106680_e158758_d_n7, assign106680_e158758_d_n8, assign106680_e158758_d_n9, assign106680_e158758_d_n10, assign106680_e158758_d_n11, assign106680_e158758_d_n13, assign106680_e158758_d_n14,) = {
    if (var_flg_nqs != 0.0) {
        let assign106680_e158754: f64 = (var_qb_nqs - var_qbulk);
        let assign106680_e158756: f64 = (assign106680_e158754 / var_taub);
        (assign106680_e158756, ((((-var_qbulk_dn0) * var_taub) - (assign106680_e158754 * var_taub_dn0)) / (var_taub * var_taub)), ((((-var_qbulk_dn2) * var_taub) - (assign106680_e158754 * var_taub_dn2)) / (var_taub * var_taub)), ((((-var_qbulk_dn4) * var_taub) - (assign106680_e158754 * var_taub_dn4)) / (var_taub * var_taub)), ((((-var_qbulk_dn5) * var_taub) - (assign106680_e158754 * var_taub_dn5)) / (var_taub * var_taub)), ((((-var_qbulk_dn6) * var_taub) - (assign106680_e158754 * var_taub_dn6)) / (var_taub * var_taub)), ((((-var_qbulk_dn7) * var_taub) - (assign106680_e158754 * var_taub_dn7)) / (var_taub * var_taub)), ((((-var_qbulk_dn8) * var_taub) - (assign106680_e158754 * var_taub_dn8)) / (var_taub * var_taub)), ((((-var_qbulk_dn9) * var_taub) - (assign106680_e158754 * var_taub_dn9)) / (var_taub * var_taub)), ((((-var_qbulk_dn10) * var_taub) - (assign106680_e158754 * var_taub_dn10)) / (var_taub * var_taub)), ((((-var_qbulk_dn11) * var_taub) - (assign106680_e158754 * var_taub_dn11)) / (var_taub * var_taub)), (var_qb_nqs_dn13 / var_taub), ((((-var_qbulk_dn14) * var_taub) - (assign106680_e158754 * var_taub_dn14)) / (var_taub * var_taub)),)
    } else {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn4, var_iqb_nqs_dn5, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn8, var_iqb_nqs_dn9, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn13, var_iqb_nqs_dn14,)
    }
};
        var_iqb_nqs = assign106680_e158758;
        var_iqb_nqs_dn0 = assign106680_e158758_d_n0;
        var_iqb_nqs_dn2 = assign106680_e158758_d_n2;
        var_iqb_nqs_dn4 = assign106680_e158758_d_n4;
        var_iqb_nqs_dn5 = assign106680_e158758_d_n5;
        var_iqb_nqs_dn6 = assign106680_e158758_d_n6;
        var_iqb_nqs_dn7 = assign106680_e158758_d_n7;
        var_iqb_nqs_dn8 = assign106680_e158758_d_n8;
        var_iqb_nqs_dn9 = assign106680_e158758_d_n9;
        var_iqb_nqs_dn10 = assign106680_e158758_d_n10;
        var_iqb_nqs_dn11 = assign106680_e158758_d_n11;
        var_iqb_nqs_dn13 = assign106680_e158758_d_n13;
        var_iqb_nqs_dn14 = assign106680_e158758_d_n14;

        let (assign106690_e158764, assign106690_e158764_d_n0, assign106690_e158764_d_n2, assign106690_e158764_d_n4, assign106690_e158764_d_n5, assign106690_e158764_d_n6, assign106690_e158764_d_n7, assign106690_e158764_d_n8, assign106690_e158764_d_n9, assign106690_e158764_d_n10, assign106690_e158764_d_n11, assign106690_e158764_d_n12, assign106690_e158764_d_n14,) = {
    if (var_flg_nqs != 0.0) {
        let assign106690_e158762: f64 = (var_qi_nqs * var_qdrat);
        (assign106690_e158762, (var_qi_nqs * var_qdrat_dn0), (var_qi_nqs * var_qdrat_dn2), (var_qi_nqs * var_qdrat_dn4), (var_qi_nqs * var_qdrat_dn5), (var_qi_nqs * var_qdrat_dn6), (var_qi_nqs * var_qdrat_dn7), (var_qi_nqs * var_qdrat_dn8), (var_qi_nqs * var_qdrat_dn9), (var_qi_nqs * var_qdrat_dn10), (var_qi_nqs * var_qdrat_dn11), (var_qi_nqs_dn12 * var_qdrat), (var_qi_nqs * var_qdrat_dn14),)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn4, var_qd_nqs_dn5, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn8, var_qd_nqs_dn9, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn14,)
    }
};
        var_qd_nqs = assign106690_e158764;
        var_qd_nqs_dn0 = assign106690_e158764_d_n0;
        var_qd_nqs_dn2 = assign106690_e158764_d_n2;
        var_qd_nqs_dn4 = assign106690_e158764_d_n4;
        var_qd_nqs_dn5 = assign106690_e158764_d_n5;
        var_qd_nqs_dn6 = assign106690_e158764_d_n6;
        var_qd_nqs_dn7 = assign106690_e158764_d_n7;
        var_qd_nqs_dn8 = assign106690_e158764_d_n8;
        var_qd_nqs_dn9 = assign106690_e158764_d_n9;
        var_qd_nqs_dn10 = assign106690_e158764_d_n10;
        var_qd_nqs_dn11 = assign106690_e158764_d_n11;
        var_qd_nqs_dn12 = assign106690_e158764_d_n12;
        var_qd_nqs_dn14 = assign106690_e158764_d_n14;

        let (assign106700_e158771, assign106700_e158771_d_n12, assign106700_e158771_d_n13,) = {
    if (var_flg_nqs != 0.0) {
        let assign106700_e158767: f64 = (-var_qi_nqs);
        let assign106700_e158769: f64 = (assign106700_e158767 - var_qb_nqs);
        (assign106700_e158769, (-var_qi_nqs_dn12), (-var_qb_nqs_dn13),)
    } else {
        (var_qg_nqs, var_qg_nqs_dn12, var_qg_nqs_dn13,)
    }
};
        var_qg_nqs = assign106700_e158771;
        var_qg_nqs_dn12 = assign106700_e158771_d_n12;
        var_qg_nqs_dn13 = assign106700_e158771_d_n13;

        let (assign106710_e158779, assign106710_e158779_d_n0, assign106710_e158779_d_n2, assign106710_e158779_d_n4, assign106710_e158779_d_n5, assign106710_e158779_d_n6, assign106710_e158779_d_n7, assign106710_e158779_d_n8, assign106710_e158779_d_n9, assign106710_e158779_d_n10, assign106710_e158779_d_n11, assign106710_e158779_d_n12, assign106710_e158779_d_n14,) = {
    if (var_flg_nqs != 0.0) {
        let assign106710_e158776: f64 = (1.0 - var_qdrat);
        let assign106710_e158777: f64 = (var_qi_nqs * assign106710_e158776);
        (assign106710_e158777, (var_qi_nqs * (-var_qdrat_dn0)), (var_qi_nqs * (-var_qdrat_dn2)), (var_qi_nqs * (-var_qdrat_dn4)), (var_qi_nqs * (-var_qdrat_dn5)), (var_qi_nqs * (-var_qdrat_dn6)), (var_qi_nqs * (-var_qdrat_dn7)), (var_qi_nqs * (-var_qdrat_dn8)), (var_qi_nqs * (-var_qdrat_dn9)), (var_qi_nqs * (-var_qdrat_dn10)), (var_qi_nqs * (-var_qdrat_dn11)), (var_qi_nqs_dn12 * assign106710_e158776), (var_qi_nqs * (-var_qdrat_dn14)),)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn4, var_qs_nqs_dn5, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn8, var_qs_nqs_dn9, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn14,)
    }
};
        var_qs_nqs = assign106710_e158779;
        var_qs_nqs_dn0 = assign106710_e158779_d_n0;
        var_qs_nqs_dn2 = assign106710_e158779_d_n2;
        var_qs_nqs_dn4 = assign106710_e158779_d_n4;
        var_qs_nqs_dn5 = assign106710_e158779_d_n5;
        var_qs_nqs_dn6 = assign106710_e158779_d_n6;
        var_qs_nqs_dn7 = assign106710_e158779_d_n7;
        var_qs_nqs_dn8 = assign106710_e158779_d_n8;
        var_qs_nqs_dn9 = assign106710_e158779_d_n9;
        var_qs_nqs_dn10 = assign106710_e158779_d_n10;
        var_qs_nqs_dn11 = assign106710_e158779_d_n11;
        var_qs_nqs_dn12 = assign106710_e158779_d_n12;
        var_qs_nqs_dn14 = assign106710_e158779_d_n14;

        let (assign106720_e158784, assign106720_e158784_d_n0, assign106720_e158784_d_n2, assign106720_e158784_d_n4, assign106720_e158784_d_n5, assign106720_e158784_d_n6, assign106720_e158784_d_n7, assign106720_e158784_d_n8, assign106720_e158784_d_n9, assign106720_e158784_d_n10, assign106720_e158784_d_n11, assign106720_e158784_d_n12, assign106720_e158784_d_n14,) = {
    if (var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iqi_nqs, var_iqi_nqs_dn0, var_iqi_nqs_dn2, var_iqi_nqs_dn4, var_iqi_nqs_dn5, var_iqi_nqs_dn6, var_iqi_nqs_dn7, var_iqi_nqs_dn8, var_iqi_nqs_dn9, var_iqi_nqs_dn10, var_iqi_nqs_dn11, var_iqi_nqs_dn12, var_iqi_nqs_dn14,)
    }
};
        var_iqi_nqs = assign106720_e158784;
        var_iqi_nqs_dn0 = assign106720_e158784_d_n0;
        var_iqi_nqs_dn2 = assign106720_e158784_d_n2;
        var_iqi_nqs_dn4 = assign106720_e158784_d_n4;
        var_iqi_nqs_dn5 = assign106720_e158784_d_n5;
        var_iqi_nqs_dn6 = assign106720_e158784_d_n6;
        var_iqi_nqs_dn7 = assign106720_e158784_d_n7;
        var_iqi_nqs_dn8 = assign106720_e158784_d_n8;
        var_iqi_nqs_dn9 = assign106720_e158784_d_n9;
        var_iqi_nqs_dn10 = assign106720_e158784_d_n10;
        var_iqi_nqs_dn11 = assign106720_e158784_d_n11;
        var_iqi_nqs_dn12 = assign106720_e158784_d_n12;
        var_iqi_nqs_dn14 = assign106720_e158784_d_n14;

        let (assign106730_e158789, assign106730_e158789_d_n0, assign106730_e158789_d_n2, assign106730_e158789_d_n4, assign106730_e158789_d_n5, assign106730_e158789_d_n6, assign106730_e158789_d_n7, assign106730_e158789_d_n8, assign106730_e158789_d_n9, assign106730_e158789_d_n10, assign106730_e158789_d_n11, assign106730_e158789_d_n13, assign106730_e158789_d_n14,) = {
    if (var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn4, var_iqb_nqs_dn5, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn8, var_iqb_nqs_dn9, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn13, var_iqb_nqs_dn14,)
    }
};
        var_iqb_nqs = assign106730_e158789;
        var_iqb_nqs_dn0 = assign106730_e158789_d_n0;
        var_iqb_nqs_dn2 = assign106730_e158789_d_n2;
        var_iqb_nqs_dn4 = assign106730_e158789_d_n4;
        var_iqb_nqs_dn5 = assign106730_e158789_d_n5;
        var_iqb_nqs_dn6 = assign106730_e158789_d_n6;
        var_iqb_nqs_dn7 = assign106730_e158789_d_n7;
        var_iqb_nqs_dn8 = assign106730_e158789_d_n8;
        var_iqb_nqs_dn9 = assign106730_e158789_d_n9;
        var_iqb_nqs_dn10 = assign106730_e158789_d_n10;
        var_iqb_nqs_dn11 = assign106730_e158789_d_n11;
        var_iqb_nqs_dn13 = assign106730_e158789_d_n13;
        var_iqb_nqs_dn14 = assign106730_e158789_d_n14;

        let (assign106740_e158794, assign106740_e158794_d_n0, assign106740_e158794_d_n2, assign106740_e158794_d_n4, assign106740_e158794_d_n5, assign106740_e158794_d_n6, assign106740_e158794_d_n7, assign106740_e158794_d_n8, assign106740_e158794_d_n9, assign106740_e158794_d_n10, assign106740_e158794_d_n11, assign106740_e158794_d_n12, assign106740_e158794_d_n14,) = {
    if (var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn4, var_qd_nqs_dn5, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn8, var_qd_nqs_dn9, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn14,)
    }
};
        var_qd_nqs = assign106740_e158794;
        var_qd_nqs_dn0 = assign106740_e158794_d_n0;
        var_qd_nqs_dn2 = assign106740_e158794_d_n2;
        var_qd_nqs_dn4 = assign106740_e158794_d_n4;
        var_qd_nqs_dn5 = assign106740_e158794_d_n5;
        var_qd_nqs_dn6 = assign106740_e158794_d_n6;
        var_qd_nqs_dn7 = assign106740_e158794_d_n7;
        var_qd_nqs_dn8 = assign106740_e158794_d_n8;
        var_qd_nqs_dn9 = assign106740_e158794_d_n9;
        var_qd_nqs_dn10 = assign106740_e158794_d_n10;
        var_qd_nqs_dn11 = assign106740_e158794_d_n11;
        var_qd_nqs_dn12 = assign106740_e158794_d_n12;
        var_qd_nqs_dn14 = assign106740_e158794_d_n14;

        let (assign106750_e158799, assign106750_e158799_d_n12, assign106750_e158799_d_n13,) = {
    if (var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_qg_nqs, var_qg_nqs_dn12, var_qg_nqs_dn13,)
    }
};
        var_qg_nqs = assign106750_e158799;
        var_qg_nqs_dn12 = assign106750_e158799_d_n12;
        var_qg_nqs_dn13 = assign106750_e158799_d_n13;

        let (assign106760_e158804, assign106760_e158804_d_n0, assign106760_e158804_d_n2, assign106760_e158804_d_n4, assign106760_e158804_d_n5, assign106760_e158804_d_n6, assign106760_e158804_d_n7, assign106760_e158804_d_n8, assign106760_e158804_d_n9, assign106760_e158804_d_n10, assign106760_e158804_d_n11, assign106760_e158804_d_n12, assign106760_e158804_d_n14,) = {
    if (var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn4, var_qs_nqs_dn5, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn8, var_qs_nqs_dn9, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn14,)
    }
};
        var_qs_nqs = assign106760_e158804;
        var_qs_nqs_dn0 = assign106760_e158804_d_n0;
        var_qs_nqs_dn2 = assign106760_e158804_d_n2;
        var_qs_nqs_dn4 = assign106760_e158804_d_n4;
        var_qs_nqs_dn5 = assign106760_e158804_d_n5;
        var_qs_nqs_dn6 = assign106760_e158804_d_n6;
        var_qs_nqs_dn7 = assign106760_e158804_d_n7;
        var_qs_nqs_dn8 = assign106760_e158804_d_n8;
        var_qs_nqs_dn9 = assign106760_e158804_d_n9;
        var_qs_nqs_dn10 = assign106760_e158804_d_n10;
        var_qs_nqs_dn11 = assign106760_e158804_d_n11;
        var_qs_nqs_dn12 = assign106760_e158804_d_n12;
        var_qs_nqs_dn14 = assign106760_e158804_d_n14;

        let assign106770_e158807: f64 = (p.p87 * var_mode);
        let assign106770_e158809: f64 = (assign106770_e158807 * var_ids);
        var_idse = assign106770_e158809;
        var_idse_dn0 = (assign106770_e158807 * var_ids_dn0);
        var_idse_dn2 = (assign106770_e158807 * var_ids_dn2);
        var_idse_dn4 = (assign106770_e158807 * var_ids_dn4);
        var_idse_dn5 = (assign106770_e158807 * var_ids_dn5);
        var_idse_dn6 = (assign106770_e158807 * var_ids_dn6);
        var_idse_dn7 = (assign106770_e158807 * var_ids_dn7);
        var_idse_dn8 = (assign106770_e158807 * var_ids_dn8);
        var_idse_dn9 = (assign106770_e158807 * var_ids_dn9);
        var_idse_dn10 = (assign106770_e158807 * var_ids_dn10);
        var_idse_dn11 = (assign106770_e158807 * var_ids_dn11);
        var_idse_dn14 = (assign106770_e158807 * var_ids_dn14);

        let assign106930_e158857: f64 = var_qg_dn6;
        var_cgdbd = assign106930_e158857;
        var_cgdbd_dn0 = 0.0;
        var_cgdbd_dn2 = 0.0;
        var_cgdbd_dn4 = 0.0;
        var_cgdbd_dn5 = 0.0;
        var_cgdbd_dn6 = 0.0;
        var_cgdbd_dn7 = 0.0;
        var_cgdbd_dn8 = 0.0;
        var_cgdbd_dn9 = 0.0;
        var_cgdbd_dn10 = 0.0;
        var_cgdbd_dn11 = 0.0;
        var_cgdbd_dn14 = 0.0;

        let assign106940_e158860: f64 = (p.p87 * var_cgdbd);
        var_cgdbd = assign106940_e158860;
        var_cgdbd_dn0 = (p.p87 * var_cgdbd_dn0);
        var_cgdbd_dn2 = (p.p87 * var_cgdbd_dn2);
        var_cgdbd_dn4 = (p.p87 * var_cgdbd_dn4);
        var_cgdbd_dn5 = (p.p87 * var_cgdbd_dn5);
        var_cgdbd_dn6 = (p.p87 * var_cgdbd_dn6);
        var_cgdbd_dn7 = (p.p87 * var_cgdbd_dn7);
        var_cgdbd_dn8 = (p.p87 * var_cgdbd_dn8);
        var_cgdbd_dn9 = (p.p87 * var_cgdbd_dn9);
        var_cgdbd_dn10 = (p.p87 * var_cgdbd_dn10);
        var_cgdbd_dn11 = (p.p87 * var_cgdbd_dn11);
        var_cgdbd_dn14 = (p.p87 * var_cgdbd_dn14);

        let assign106950_e158863: f64 = var_qg_dn8;
        var_cgsbd = assign106950_e158863;
        var_cgsbd_dn0 = 0.0;
        var_cgsbd_dn2 = 0.0;
        var_cgsbd_dn4 = 0.0;
        var_cgsbd_dn5 = 0.0;
        var_cgsbd_dn6 = 0.0;
        var_cgsbd_dn7 = 0.0;
        var_cgsbd_dn8 = 0.0;
        var_cgsbd_dn9 = 0.0;
        var_cgsbd_dn10 = 0.0;
        var_cgsbd_dn11 = 0.0;
        var_cgsbd_dn14 = 0.0;

        let assign106960_e158866: f64 = (p.p87 * var_cgsbd);
        var_cgsbd = assign106960_e158866;
        var_cgsbd_dn0 = (p.p87 * var_cgsbd_dn0);
        var_cgsbd_dn2 = (p.p87 * var_cgsbd_dn2);
        var_cgsbd_dn4 = (p.p87 * var_cgsbd_dn4);
        var_cgsbd_dn5 = (p.p87 * var_cgsbd_dn5);
        var_cgsbd_dn6 = (p.p87 * var_cgsbd_dn6);
        var_cgsbd_dn7 = (p.p87 * var_cgsbd_dn7);
        var_cgsbd_dn8 = (p.p87 * var_cgsbd_dn8);
        var_cgsbd_dn9 = (p.p87 * var_cgsbd_dn9);
        var_cgsbd_dn10 = (p.p87 * var_cgsbd_dn10);
        var_cgsbd_dn11 = (p.p87 * var_cgsbd_dn11);
        var_cgsbd_dn14 = (p.p87 * var_cgsbd_dn14);

        let assign107330_e158981: f64 = if var_mode == 1.0 { 1.0 } else { 0.0 };
        var_guard2411 = assign107330_e158981;

        let (assign107360_e158993, assign107360_e158993_d_n0, assign107360_e158993_d_n2, assign107360_e158993_d_n4, assign107360_e158993_d_n5, assign107360_e158993_d_n6, assign107360_e158993_d_n7, assign107360_e158993_d_n8, assign107360_e158993_d_n9, assign107360_e158993_d_n10, assign107360_e158993_d_n11, assign107360_e158993_d_n14,) = {
    if (var_guard2411 != 0.0) {
        (var_cgsbd, var_cgsbd_dn0, var_cgsbd_dn2, var_cgsbd_dn4, var_cgsbd_dn5, var_cgsbd_dn6, var_cgsbd_dn7, var_cgsbd_dn8, var_cgsbd_dn9, var_cgsbd_dn10, var_cgsbd_dn11, var_cgsbd_dn14,)
    } else {
        (var_cgsb, var_cgsb_dn0, var_cgsb_dn2, var_cgsb_dn4, var_cgsb_dn5, var_cgsb_dn6, var_cgsb_dn7, var_cgsb_dn8, var_cgsb_dn9, var_cgsb_dn10, var_cgsb_dn11, var_cgsb_dn14,)
    }
};
        var_cgsb = assign107360_e158993;
        var_cgsb_dn0 = assign107360_e158993_d_n0;
        var_cgsb_dn2 = assign107360_e158993_d_n2;
        var_cgsb_dn4 = assign107360_e158993_d_n4;
        var_cgsb_dn5 = assign107360_e158993_d_n5;
        var_cgsb_dn6 = assign107360_e158993_d_n6;
        var_cgsb_dn7 = assign107360_e158993_d_n7;
        var_cgsb_dn8 = assign107360_e158993_d_n8;
        var_cgsb_dn9 = assign107360_e158993_d_n9;
        var_cgsb_dn10 = assign107360_e158993_d_n10;
        var_cgsb_dn11 = assign107360_e158993_d_n11;
        var_cgsb_dn14 = assign107360_e158993_d_n14;

        let (assign107460_e159037, assign107460_e159037_d_n0, assign107460_e159037_d_n2, assign107460_e159037_d_n4, assign107460_e159037_d_n5, assign107460_e159037_d_n6, assign107460_e159037_d_n7, assign107460_e159037_d_n8, assign107460_e159037_d_n9, assign107460_e159037_d_n10, assign107460_e159037_d_n11, assign107460_e159037_d_n14,) = {
    if (var_guard2411 == 0.0) {
        (var_cgdbd, var_cgdbd_dn0, var_cgdbd_dn2, var_cgdbd_dn4, var_cgdbd_dn5, var_cgdbd_dn6, var_cgdbd_dn7, var_cgdbd_dn8, var_cgdbd_dn9, var_cgdbd_dn10, var_cgdbd_dn11, var_cgdbd_dn14,)
    } else {
        (var_cgsb, var_cgsb_dn0, var_cgsb_dn2, var_cgsb_dn4, var_cgsb_dn5, var_cgsb_dn6, var_cgsb_dn7, var_cgsb_dn8, var_cgsb_dn9, var_cgsb_dn10, var_cgsb_dn11, var_cgsb_dn14,)
    }
};
        var_cgsb = assign107460_e159037;
        var_cgsb_dn0 = assign107460_e159037_d_n0;
        var_cgsb_dn2 = assign107460_e159037_d_n2;
        var_cgsb_dn4 = assign107460_e159037_d_n4;
        var_cgsb_dn5 = assign107460_e159037_d_n5;
        var_cgsb_dn6 = assign107460_e159037_d_n6;
        var_cgsb_dn7 = assign107460_e159037_d_n7;
        var_cgsb_dn8 = assign107460_e159037_d_n8;
        var_cgsb_dn9 = assign107460_e159037_d_n9;
        var_cgsb_dn10 = assign107460_e159037_d_n10;
        var_cgsb_dn11 = assign107460_e159037_d_n11;
        var_cgsb_dn14 = assign107460_e159037_d_n14;

        let assign107690_e159100: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        var_guard2413 = assign107690_e159100;

        let assign107700_e159103: f64 = if p.p24 == 1.0 { 1.0 } else { 0.0 };
        var_guard2414 = assign107700_e159103;

        let assign107750_e159132: f64 = if p.p53 > 0.0 { 1.0 } else { 0.0 };
        var_guard2417 = assign107750_e159132;

        let (assign107760_e159136, assign107760_e159136_d_n0, assign107760_e159136_d_n2, assign107760_e159136_d_n4, assign107760_e159136_d_n5, assign107760_e159136_d_n6, assign107760_e159136_d_n7, assign107760_e159136_d_n8, assign107760_e159136_d_n9, assign107760_e159136_d_n10, assign107760_e159136_d_n11, assign107760_e159136_d_n14,) = {
    if (var_guard2417 != 0.0) {
        (var_p, var_p_dn0, var_p_dn2, var_p_dn4, var_p_dn5, var_p_dn6, var_p_dn7, var_p_dn8, var_p_dn9, var_p_dn10, var_p_dn11, var_p_dn14,)
    } else {
        (var_itemp, var_itemp_dn0, var_itemp_dn2, var_itemp_dn4, var_itemp_dn5, var_itemp_dn6, var_itemp_dn7, var_itemp_dn8, var_itemp_dn9, var_itemp_dn10, var_itemp_dn11, var_itemp_dn14,)
    }
};
        var_itemp = assign107760_e159136;
        var_itemp_dn0 = assign107760_e159136_d_n0;
        var_itemp_dn2 = assign107760_e159136_d_n2;
        var_itemp_dn4 = assign107760_e159136_d_n4;
        var_itemp_dn5 = assign107760_e159136_d_n5;
        var_itemp_dn6 = assign107760_e159136_d_n6;
        var_itemp_dn7 = assign107760_e159136_d_n7;
        var_itemp_dn8 = assign107760_e159136_d_n8;
        var_itemp_dn9 = assign107760_e159136_d_n9;
        var_itemp_dn10 = assign107760_e159136_d_n10;
        var_itemp_dn11 = assign107760_e159136_d_n11;
        var_itemp_dn14 = assign107760_e159136_d_n14;

        let (assign107780_e159145,) = {
    if (p.p28 != 0.0) {
        (1.0,)
    } else {
        (var_cqi,)
    }
};
        var_cqi = assign107780_e159145;

        let (assign107790_e159149,) = {
    if (p.p28 != 0.0) {
        (1.0,)
    } else {
        (var_cqb,)
    }
};
        var_cqb = assign107790_e159149;

        *var_cgdbd_slot = var_cgdbd;
        *var_cgdbd_dn0_slot = var_cgdbd_dn0;
        *var_cgdbd_dn10_slot = var_cgdbd_dn10;
        *var_cgdbd_dn11_slot = var_cgdbd_dn11;
        *var_cgdbd_dn14_slot = var_cgdbd_dn14;
        *var_cgdbd_dn2_slot = var_cgdbd_dn2;
        *var_cgdbd_dn4_slot = var_cgdbd_dn4;
        *var_cgdbd_dn5_slot = var_cgdbd_dn5;
        *var_cgdbd_dn6_slot = var_cgdbd_dn6;
        *var_cgdbd_dn7_slot = var_cgdbd_dn7;
        *var_cgdbd_dn8_slot = var_cgdbd_dn8;
        *var_cgdbd_dn9_slot = var_cgdbd_dn9;
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
        *var_cgsbd_slot = var_cgsbd;
        *var_cgsbd_dn0_slot = var_cgsbd_dn0;
        *var_cgsbd_dn10_slot = var_cgsbd_dn10;
        *var_cgsbd_dn11_slot = var_cgsbd_dn11;
        *var_cgsbd_dn14_slot = var_cgsbd_dn14;
        *var_cgsbd_dn2_slot = var_cgsbd_dn2;
        *var_cgsbd_dn4_slot = var_cgsbd_dn4;
        *var_cgsbd_dn5_slot = var_cgsbd_dn5;
        *var_cgsbd_dn6_slot = var_cgsbd_dn6;
        *var_cgsbd_dn7_slot = var_cgsbd_dn7;
        *var_cgsbd_dn8_slot = var_cgsbd_dn8;
        *var_cgsbd_dn9_slot = var_cgsbd_dn9;
        *var_cqb_slot = var_cqb;
        *var_cqi_slot = var_cqi;
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
        *var_guard2409_slot = var_guard2409;
        *var_guard2410_slot = var_guard2410;
        *var_guard2411_slot = var_guard2411;
        *var_guard2413_slot = var_guard2413;
        *var_guard2414_slot = var_guard2414;
        *var_guard2417_slot = var_guard2417;
        *var_idse_slot = var_idse;
        *var_idse_dn0_slot = var_idse_dn0;
        *var_idse_dn10_slot = var_idse_dn10;
        *var_idse_dn11_slot = var_idse_dn11;
        *var_idse_dn14_slot = var_idse_dn14;
        *var_idse_dn2_slot = var_idse_dn2;
        *var_idse_dn4_slot = var_idse_dn4;
        *var_idse_dn5_slot = var_idse_dn5;
        *var_idse_dn6_slot = var_idse_dn6;
        *var_idse_dn7_slot = var_idse_dn7;
        *var_idse_dn8_slot = var_idse_dn8;
        *var_idse_dn9_slot = var_idse_dn9;
        *var_iqb_nqs_slot = var_iqb_nqs;
        *var_iqb_nqs_dn0_slot = var_iqb_nqs_dn0;
        *var_iqb_nqs_dn10_slot = var_iqb_nqs_dn10;
        *var_iqb_nqs_dn11_slot = var_iqb_nqs_dn11;
        *var_iqb_nqs_dn13_slot = var_iqb_nqs_dn13;
        *var_iqb_nqs_dn14_slot = var_iqb_nqs_dn14;
        *var_iqb_nqs_dn2_slot = var_iqb_nqs_dn2;
        *var_iqb_nqs_dn4_slot = var_iqb_nqs_dn4;
        *var_iqb_nqs_dn5_slot = var_iqb_nqs_dn5;
        *var_iqb_nqs_dn6_slot = var_iqb_nqs_dn6;
        *var_iqb_nqs_dn7_slot = var_iqb_nqs_dn7;
        *var_iqb_nqs_dn8_slot = var_iqb_nqs_dn8;
        *var_iqb_nqs_dn9_slot = var_iqb_nqs_dn9;
        *var_iqi_nqs_slot = var_iqi_nqs;
        *var_iqi_nqs_dn0_slot = var_iqi_nqs_dn0;
        *var_iqi_nqs_dn10_slot = var_iqi_nqs_dn10;
        *var_iqi_nqs_dn11_slot = var_iqi_nqs_dn11;
        *var_iqi_nqs_dn12_slot = var_iqi_nqs_dn12;
        *var_iqi_nqs_dn14_slot = var_iqi_nqs_dn14;
        *var_iqi_nqs_dn2_slot = var_iqi_nqs_dn2;
        *var_iqi_nqs_dn4_slot = var_iqi_nqs_dn4;
        *var_iqi_nqs_dn5_slot = var_iqi_nqs_dn5;
        *var_iqi_nqs_dn6_slot = var_iqi_nqs_dn6;
        *var_iqi_nqs_dn7_slot = var_iqi_nqs_dn7;
        *var_iqi_nqs_dn8_slot = var_iqi_nqs_dn8;
        *var_iqi_nqs_dn9_slot = var_iqi_nqs_dn9;
        *var_itemp_slot = var_itemp;
        *var_itemp_dn0_slot = var_itemp_dn0;
        *var_itemp_dn10_slot = var_itemp_dn10;
        *var_itemp_dn11_slot = var_itemp_dn11;
        *var_itemp_dn14_slot = var_itemp_dn14;
        *var_itemp_dn2_slot = var_itemp_dn2;
        *var_itemp_dn4_slot = var_itemp_dn4;
        *var_itemp_dn5_slot = var_itemp_dn5;
        *var_itemp_dn6_slot = var_itemp_dn6;
        *var_itemp_dn7_slot = var_itemp_dn7;
        *var_itemp_dn8_slot = var_itemp_dn8;
        *var_itemp_dn9_slot = var_itemp_dn9;
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
        *var_qg_nqs_slot = var_qg_nqs;
        *var_qg_nqs_dn12_slot = var_qg_nqs_dn12;
        *var_qg_nqs_dn13_slot = var_qg_nqs_dn13;
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
        *var_tau_slot = var_tau;
        *var_tau_dn0_slot = var_tau_dn0;
        *var_tau_dn10_slot = var_tau_dn10;
        *var_tau_dn11_slot = var_tau_dn11;
        *var_tau_dn14_slot = var_tau_dn14;
        *var_tau_dn2_slot = var_tau_dn2;
        *var_tau_dn4_slot = var_tau_dn4;
        *var_tau_dn5_slot = var_tau_dn5;
        *var_tau_dn6_slot = var_tau_dn6;
        *var_tau_dn7_slot = var_tau_dn7;
        *var_tau_dn8_slot = var_tau_dn8;
        *var_tau_dn9_slot = var_tau_dn9;
        *var_taub_slot = var_taub;
        *var_taub_dn0_slot = var_taub_dn0;
        *var_taub_dn10_slot = var_taub_dn10;
        *var_taub_dn11_slot = var_taub_dn11;
        *var_taub_dn14_slot = var_taub_dn14;
        *var_taub_dn2_slot = var_taub_dn2;
        *var_taub_dn4_slot = var_taub_dn4;
        *var_taub_dn5_slot = var_taub_dn5;
        *var_taub_dn6_slot = var_taub_dn6;
        *var_taub_dn7_slot = var_taub_dn7;
        *var_taub_dn8_slot = var_taub_dn8;
        *var_taub_dn9_slot = var_taub_dn9;
    }

    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[623] = param_given[12];
        s.store_scalar(623, if s.b[623] { 1.0 } else { 0.0 });

        s.b[769] = param_given[268];
        s.store_scalar(769, if s.b[769] { 1.0 } else { 0.0 });

        s.b[768] = param_given[269];
        s.store_scalar(768, if s.b[768] { 1.0 } else { 0.0 });

        s.store_scalar(294, 0.0);

        s.store_scalar(295, 0.0);

        s.store_scalar(708, 0.0);

        s.store_scalar(4, 0.0);

        s.store_scalar(5, 0.0);

        s.store_scalar(321, 0.0);

        s.store_scalar(78, 0.0);

        s.store_scalar(74, 0.0);

        s.store_scalar(347, 0.0);

        s.store_scalar(697, 0.0);

        s.store_scalar(698, 0.0);

        s.store_scalar(69, 0.8);

        s.store_scalar(70, 0.4);

        s.store_scalar(77, 0.0);

        s.store_scalar(79, 0.0);

        s.store_scalar(80, 0.0);

        s.store_scalar(81, 0.0);

        s.store_scalar(83, 0.0);

        s.store_scalar(84, 0.0);

        s.store_scalar(85, 0.0);

        s.store_scalar(86, 0.0);

        s.store_scalar(87, 0.0);

        s.store_scalar(88, 0.0);

        s.store_scalar(89, 0.0);

        s.store_scalar(90, 0.0);

        s.store_scalar(91, 0.0);

        s.store_scalar(92, 0.0);

        s.store_scalar(93, 0.0);

        s.store_scalar(94, 0.0);

        s.store_scalar(95, 0.0);

        s.store_scalar(96, 0.0);

        s.store_scalar(97, 0.0);

        s.store_scalar(98, 0.0);

        s.store_scalar(99, 0.0);

        s.store_scalar(100, 0.0);

        s.store_scalar(101, 0.0);

        s.store_scalar(102, 0.0);

        s.store_scalar(103, 0.0);

        s.store_scalar(104, 0.0);

        s.store_scalar(105, 0.0);

        s.store_scalar(106, 0.0);

        s.store_scalar(107, 0.0);

        s.store_scalar(108, 0.0);

        s.store_scalar(109, 0.0);

        s.store_scalar(110, 0.0);

        s.store_scalar(111, 0.0);

        s.store_scalar(112, 0.0);

        s.store_scalar(113, 0.0);

        s.store_scalar(114, 0.0);

        s.store_scalar(115, 0.0);

        s.store_scalar(116, 0.0);

        s.store_scalar(415, 0.0);

        s.store_scalar(117, 0.0);

        s.store_scalar(118, 0.0);

        s.store_scalar(119, 0.0);

        s.store_scalar(120, 0.0);

        s.store_scalar(121, 0.0);

        s.store_scalar(122, 0.0);

        s.store_scalar(123, 0.0);

        s.store_scalar(124, 0.0);

        s.store_scalar(125, 0.0);

        s.store_scalar(126, 0.0);

        s.store_scalar(127, 0.0);

        s.store_scalar(128, 0.0);

        s.store_scalar(129, 0.0);

        s.store_scalar(130, 0.0);

        s.store_scalar(20, 0.0);

        s.store_scalar(131, 0.0);

        s.store_scalar(132, 0.0);

        s.store_scalar(133, 0.0);

        s.store_scalar(19, 0.0);

        s.store_scalar(134, 0.0);

        s.store_scalar(135, 0.0);

        s.store_scalar(137, 0.0);

        s.store_scalar(138, 0.0);

        s.store_scalar(139, 0.0);

        s.store_scalar(140, 0.0);

        s.store_scalar(141, 0.0);

        s.store_scalar(142, 0.0);

        s.store_scalar(143, 0.0);

        s.store_scalar(144, 0.0);

        s.store_scalar(145, 0.0);

        s.store_scalar(146, 0.0);

        s.store_scalar(147, 0.0);

        s.store_scalar(148, 0.0);

        s.store_scalar(149, 0.0);

        s.store_scalar(150, 0.0);

        s.store_scalar(151, 0.0);

        s.store_scalar(152, 0.0);

        s.store_scalar(153, 0.0);

        s.store_scalar(154, 0.0);

        s.store_scalar(155, 0.0);

        s.store_scalar(156, 0.0);

        s.store_scalar(157, 0.0);

        s.store_scalar(158, 0.0);

        s.store_scalar(159, 0.0);

        s.store_scalar(160, 0.0);

        s.store_scalar(161, 0.0);

        s.store_scalar(162, 0.0);

        s.store_scalar(163, 0.0);

        s.store_scalar(164, 0.0);

        s.store_scalar(165, 0.0);

        s.store_scalar(166, 0.0);

        s.store_scalar(167, 0.0);

        s.store_scalar(168, 0.0);

        s.store_scalar(169, 0.0);

        s.store_scalar(170, 0.0);

        s.store_scalar(171, 0.0);

        s.store_scalar(172, 0.0);

        s.store_scalar(173, 0.0);

        s.store_scalar(174, 0.0);

        s.store_scalar(175, 0.0);

        s.store_scalar(176, 0.0);

        s.store_scalar(177, 0.0);

        s.store_scalar(178, 0.0);

        s.store_scalar(179, 0.0);

        s.store_scalar(180, 0.0);

        s.store_scalar(181, 0.0);

        s.store_scalar(182, 0.0);

        s.store_scalar(184, 0.0);

        s.store_scalar(185, 0.0);

        s.store_scalar(186, 0.0);

        s.store_scalar(187, 0.0);

        s.store_scalar(188, 0.0);

        s.store_scalar(412, 0.0);

        s.store_scalar(189, 0.0);

        s.store_scalar(190, 0.0);

        s.store_scalar(191, 0.0);

        s.store_scalar(192, 0.0);

        s.store_scalar(193, 0.0);

        s.store_scalar(194, 0.0);

        s.store_scalar(195, 0.0);

        s.store_scalar(196, 0.0);

        s.store_scalar(197, 0.0);

        s.store_scalar(198, 0.0);

        s.store_scalar(205, 0.0);

        s.store_scalar(206, 0.0);

        s.store_scalar(207, 0.0);

        s.store_scalar(208, 0.0);

        s.store_scalar(209, 0.0);

        s.store_scalar(210, 0.0);

        s.store_scalar(211, 0.0);

        s.store_scalar(212, 0.0);

        s.store_scalar(213, 0.0);

        s.store_scalar(214, 0.0);

        s.store_scalar(215, 0.0);

        s.store_scalar(216, 0.0);

        s.store_scalar(217, 0.0);

        s.store_scalar(218, 0.0);

        s.store_scalar(219, 0.0);

        s.store_scalar(220, 0.0);

        s.store_scalar(221, 0.0);

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

        s.store_scalar(232, 0.0);

        s.store_scalar(233, 0.0);

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

        s.store_scalar(244, 0.0);

        s.store_scalar(245, 0.0);

        s.store_scalar(246, 0.0);

        s.store_scalar(247, 0.5);

        s.store_scalar(248, 0.0);

        s.store_scalar(249, 0.0);

        s.store_scalar(250, 0.0);

        s.store_scalar(251, 0.0);

        s.store_scalar(252, 0.0);

        s.store_scalar(253, 0.0);

        s.store_scalar(254, 0.0);

        s.store_scalar(255, 0.0);

        s.store_scalar(256, 0.0);

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

        s.store_scalar(268, 0.0);

        s.store_scalar(269, 0.0);

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

        s.store_scalar(280, 0.0);

        s.store_scalar(281, 0.0);

        s.store_scalar(282, 0.0);

        s.store_scalar(283, 0.0);

        s.store_scalar(285, 0.0);

        s.store_scalar(286, 0.0);

        s.store_scalar(289, 0.0);

        s.store_scalar(290, 0.0);

        s.store_scalar(291, 0.0);

        s.store_scalar(292, 0.0);

        s.store_scalar(293, 0.0);

        s.store_scalar(296, 0.0);

        s.store_scalar(297, 0.0);

        s.store_scalar(298, 0.0);

        s.store_scalar(299, 0.0);

        s.store_scalar(300, 0.0);

        s.store_scalar(301, 0.0);

        s.store_scalar(302, 0.0);

        s.store_scalar(303, 0.0);

        s.store_scalar(304, 0.0);

        s.store_scalar(305, 0.0);

        s.store_scalar(313, 0.0);

        s.store_scalar(314, 0.0);

        s.store_scalar(315, 0.0);

        s.store_scalar(316, 0.0);

        s.store_scalar(317, 0.0);

        s.store_scalar(318, 0.0);

        s.store_scalar(319, 0.0);

        s.store_scalar(320, 0.0);

        s.store_scalar(322, 0.0);

        s.store_scalar(323, 0.0);

        s.store_scalar(324, 0.0);

        s.store_scalar(328, 0.0);

        s.store_scalar(329, 0.0);

        s.store_scalar(330, 0.0);

        s.store_scalar(331, 0.0);

        s.store_scalar(332, 0.0);

        s.store_scalar(333, 0.0);

        s.store_scalar(334, 0.0);

        s.store_scalar(335, 0.0);

        s.store_scalar(336, 0.0);

        s.store_scalar(337, 0.0);

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scalar(338, 0.0);

        s.store_scalar(339, 0.0);

        s.store_scalar(340, 0.0);

        s.store_scalar(341, 0.0);

        s.store_scalar(342, 0.0);

        s.store_scalar(343, 0.0);

        s.store_scalar(344, 0.0);

        s.store_scalar(345, 0.0);

        s.store_scalar(346, 0.0);

        s.store_scalar(348, 0.0);

        s.store_scalar(349, 0.0);

        s.store_scalar(350, 0.0);

        s.store_scalar(351, 0.0);

        s.store_scalar(352, 0.0);

        s.store_scalar(353, 0.0);

        s.store_scalar(354, 0.0);

        s.store_scalar(355, 0.0);

        s.store_scalar(356, 0.0);

        s.store_scalar(357, 0.0);

        s.store_scalar(358, 0.0);

        s.store_scalar(359, 0.0);

        s.store_scalar(364, 0.0);

        s.store_scalar(366, 0.0);

        s.store_scalar(367, 0.0);

        s.store_scalar(368, 0.0);

        s.store_scalar(369, 0.0);

        s.store_scalar(370, 0.0);

        s.store_scalar(371, 0.0);

        s.store_scalar(372, 0.0);

        s.store_scalar(373, 0.0);

        s.store_scalar(374, 0.0);

        s.store_scalar(375, 0.0);

        s.store_scalar(376, 0.0);

        s.store_scalar(377, 0.0);

        s.store_scalar(380, 0.0);

        s.store_scalar(381, 0.0);

        s.store_scalar(382, 0.0);

        s.store_scalar(383, 0.0);

        s.store_scalar(387, 0.0);

        s.store_scalar(388, 0.0);

        s.store_scalar(389, 0.0);

        s.store_scalar(390, 0.0);

        s.store_scalar(391, 0.0);

        s.store_scalar(392, 0.0);

        s.store_scalar(393, 0.0);

        s.store_scalar(394, 0.0);

        s.store_scalar(395, 0.0);

        s.store_scalar(396, 0.0);

        s.store_scalar(397, 0.0);

        s.store_scalar(398, 0.0);

        s.store_scalar(399, 0.0);

        s.store_scalar(400, 0.0);

        s.store_scalar(402, 0.0);

        s.store_scalar(403, 0.0);

        s.store_scalar(404, 0.0);

        s.store_scalar(405, 0.0);

        s.store_scalar(385, p.p334);

        s.store_scalar(386, p.p334);

        s.store_scalar(409, 0.0);

        s.store_scalar(410, 0.0);

        s.store_scalar(434, 0.0093868);

        s.store_scalar(435, (-0.1047839));

        s.store_scalar(447, 0.0);

        s.store_scalar(573, 0.0);

        s.store_scalar(574, 0.0);

        s.store_scalar(575, 0.0);

        s.store_scalar(576, 0.0);

        s.store_scalar(577, 0.0);

        s.store_scalar(578, 0.0);

        s.store_scalar(579, 0.0);

        s.store_scalar(580, 0.0);

        s.store_scalar(581, 0.0);

        s.store_scalar(582, 0.0);

        s.store_scalar(583, 0.0);

        s.store_scalar(584, 0.0);

        s.store_scalar(585, 0.0);

        s.store_scalar(586, 0.0);

        s.store_scalar(587, 0.0);

        s.store_scalar(588, 0.0);

        s.store_scalar(589, 0.0);

        s.store_scalar(590, 0.0);

        s.store_scalar(591, 0.0);

        s.store_scalar(592, 0.0);

        s.store_scalar(593, 0.0);

        s.store_scalar(594, 0.0);

        s.store_scalar(595, 0.0);

        s.store_scalar(596, 0.0);

        s.store_scalar(597, 0.0);

        s.store_scalar(739, 0.0);

        s.store_scalar(598, 0.0);

        s.store_scalar(770, 0.0);

        s.store_scalar(727, 0.0);

        s.store_scalar(728, 0.0);

        s.store_scalar(729, 0.0);

        s.store_scalar(730, 0.0);

        s.store_scalar(731, 0.0);

        s.store_scalar(732, 0.0);

        s.store_scalar(733, 0.0);

        s.store_scalar(734, 0.0);

        s.store_scalar(735, 0.0);

        s.store_scalar(740, 0.0);

        s.store_scalar(18, 0.0);

        s.store_scalar(741, 0.0);

        s.store_scalar(745, 0.0);

        s.store_scalar(746, 0.0);

        s.store_scalar(747, 0.0);

        s.store_scalar(748, 0.0);

        s.store_scalar(751, 0.0);

        s.store_scalar(752, 0.0);

        s.store_scalar(753, 0.0);

        s.store_scalar(757, 0.0);

        s.store_scalar(682, 0.0);

        s.store_scalar(688, 0.0);

        s.store_scalar(689, 0.0);

        s.store_scalar(787, 0.0);

        s.store_scalar(794, 0.0);

        s.store_scalar(788, 0.0);

        s.store_scalar(690, 0.0);

        s.store_scalar(692, 0.0);

        s.store_scalar(691, 0.0);

        s.store_scalar(693, 0.0);

        s.store_scalar(795, 0.0);

        s.store_scalar(676, 0.0);

        s.store_scalar(681, 0.0);

        s.store_scalar(678, 0.0);

        s.store_scalar(686, 0.0);

        s.store_scalar(687, 0.0);

        s.store_scalar(694, 0.0);

        s.store_scalar(679, 0.0);

        s.store_scalar(683, 0.0);

        s.store_scalar(680, 0.0);

        s.store_scalar(677, 0.0);

        s.store_scalar(684, 0.0);

        s.store_scalar(685, 0.0);

        s.store_scalar(956, p.p436);

        s.store_scalar(959, p.p437);

        s.store_scalar(986, 0.0);

        s.store_scalar(987, 0.0);

        s.store_scalar(988, 0.0);

        s.store_scalar(961, 0.0);

        s.store_scalar(960, 0.0);

        s.store_scalar(427, p.p447);

        s.store_scalar(957, p.p193);

        s.store_scalar(977, 0.0);

        s.store_scalar(978, 0.0);

        s.store_scalar(421, 40.0);

        s.store_scalar(828, 0.0);

        s.store_scalar(829, 0.0);

        s.store_scalar(830, 0.0);

        s.store_scalar(831, 0.0);

        s.store_scalar(66, 0.0);

        s.store_scalar(65, 0.0);

        s.store_scalar(68, 0.0);

        s.store_scalar(67, 0.0);

        s.store_scalar(832, 0.0);

        s.store_scalar(833, 0.0);

        s.store_scalar(834, 0.0);

        s.store_scalar(835, 0.0);

        s.store_scalar(838, 0.0);

        s.store_scalar(839, 0.0);

        s.store_scalar(841, 0.0);

        s.store_scalar(842, 0.0);

        s.store_scalar(843, 0.0);

        s.store_scalar(844, 0.0);

        s.store_scalar(845, 0.0);

        s.store_scalar(846, 0.0);

        s.store_scalar(840, 0.0);

        s.store_scalar(857, 0.0);

        s.store_scalar(858, 0.0);

        s.store_scalar(859, 0.0);

        s.store_scalar(860, 0.0);

        s.store_scalar(865, 0.0);

        s.store_scalar(866, 0.0);

        s.store_scalar(867, 0.0);

        s.store_scalar(868, 0.0);

        s.store_scalar(849, 0.0);

        s.store_scalar(854, 0.0);

        s.store_scalar(847, 0.0);

        s.store_scalar(852, 0.0);

        s.store_scalar(851, 0.0);

        s.store_scalar(856, 0.0);

        s.store_scalar(848, 0.0);

        s.store_scalar(853, 0.0);

        s.store_scalar(850, 0.0);

        s.store_scalar(855, 0.0);

        s.store_scalar(946, 0.0);

        s.store_scalar(944, 0.0);

        s.store_scalar(947, 0.0);

        s.store_scalar(945, 0.0);

        s.store_scalar(948, 0.0);

        s.store_scalar(816, 0.0);

        s.store_scalar(873, 0.0);

        s.store_scalar(874, 0.0);

        s.store_scalar(875, 0.0);

        s.store_scalar(876, 0.0);

        s.store_scalar(877, 0.0);

        s.store_scalar(878, 0.0);

        s.store_scalar(879, 0.0);

        s.store_scalar(880, 0.0);

        s.store_scalar(881, 0.0);

        s.store_scalar(882, 0.0);

        s.store_scalar(883, 0.0);

        s.store_scalar(884, 0.0);

        s.store_scalar(360, 0.0);

        s.store_scalar(362, 0.0);

        s.store_scalar(361, 0.0);

        s.store_scalar(363, 0.0);

        s.store_scalar(603, 0.0);

        s.store_scalar(45, 0.0);

        s.store_scalar(46, 0.0);

        s.store_scalar(413, 0.0);

        s.store_scalar(932, 0.0);

        s.store_scalar(926, 0.0);

        s.store_scalar(927, 0.0);

        s.store_scalar(287, 0.0);

        s.store_scalar(407, 0.0);

        s.store_scalar(924, 0.0);

        s.store_scalar(925, 0.0);

        s.store_scalar(931, 0.0);

        s.store_scalar(990, 0.0);

        s.store_scalar(411, 0.0);

        s.store_scalar(288, 0.0);

        s.store_scalar(448, (if (p.p40 != 0.0) { 0.0 } else { p.p17 }));

        s.store_scalar(450, p.p104);

        s.store_scalar(451, p.p294);

        s.store_scalar(452, p.p222);

        s.store_scalar(453, p.p420);

        s.store_scalar(365, 1.0);

        s.b[1008] = (s.v[452] < 0.0);
        s.store_scalar(1008, if s.b[1008] { 1.0 } else { 0.0 });

        if s.b[1008] {
            s.store_scalar(452, 0.0);
        }

        s.b[1009] = (s.v[452] > 0.0);
        s.store_scalar(1009, if s.b[1009] { 1.0 } else { 0.0 });

        if s.b[1009] {
            s.store_scalar(452, 0.0);
        }

        s.b[1011] = (s.v[451] < 0.0);
        s.store_scalar(1011, if s.b[1011] { 1.0 } else { 0.0 });

        if s.b[1011] {
            s.store_scalar(451, 0.0);
        }

        s.b[1014] = (s.v[453] < 0.0);
        s.store_scalar(1014, if s.b[1014] { 1.0 } else { 0.0 });

        if s.b[1014] {
            s.store_scalar(453, 0.0);
        }

        s.b[1015] = (s.v[453] > 1.0);
        s.store_scalar(1015, if s.b[1015] { 1.0 } else { 0.0 });

        if s.b[1015] {
            s.store_scalar(453, 1.0);
        }

        s.store_scalar(964, p.p340);

        s.store_scalar(965, p.p343);

        s.store_scalar(963, p.p42);

        s.store_scalar(967, p.p354);

        s.store_scalar(969, p.p355);

        s.store_scalar(966, p.p346);

        s.store_scalar(968, p.p349);

        s.store_scalar(970, p.p352);

        s.store_scalar(972, p.p360);

        s.store_scalar(973, p.p367);

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scalar(976, p.p364);

        s.store_scalar(971, p.p377);

        s.store_scalar(974, p.p370);

        s.store_scalar(975, p.p371);

        s.b[1110] = ((s.v[963] < 3.0) && (s.v[963] > 0.0));
        s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });

        s.b[1113] = (s.v[964] < 5000000000000000.0);
        s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });

        if (s.b[1110] && s.b[1113]) {
            s.store_scalar(964, 5000000000000000.0);
        }

        s.b[1114] = (s.v[964] > 1e18);
        s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });

        if (s.b[1110] && s.b[1114]) {
            s.store_scalar(964, 1e18);
        }

        s.b[1117] = (s.v[965] < 1e-8);
        s.store_scalar(1117, if s.b[1117] { 1.0 } else { 0.0 });

        if (s.b[1110] && s.b[1117]) {
            s.store_scalar(965, 1e-8);
        }

        s.b[1118] = (s.v[965] > 1e-6);
        s.store_scalar(1118, if s.b[1118] { 1.0 } else { 0.0 });

        if (s.b[1110] && s.b[1118]) {
            s.store_scalar(965, 1e-6);
        }

        s.b[1121] = (s.v[966] < 1.0);
        s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });

        if (s.b[1110] && s.b[1121]) {
            s.store_scalar(966, 1.0);
        }

        s.b[1122] = (s.v[966] > 100000.0);
        s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });

        if (s.b[1110] && s.b[1122]) {
            s.store_scalar(966, 100000.0);
        }

        s.b[1125] = (s.v[967] < 1.0);
        s.store_scalar(1125, if s.b[1125] { 1.0 } else { 0.0 });

        if (s.b[1110] && s.b[1125]) {
            s.store_scalar(967, 1.0);
        }

        s.b[1126] = (s.v[967] > 100000.0);
        s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });

        if (s.b[1110] && s.b[1126]) {
            s.store_scalar(967, 100000.0);
        }

        s.b[1129] = (s.v[971] < 1.0);
        s.store_scalar(1129, if s.b[1129] { 1.0 } else { 0.0 });

        if (s.b[1110] && s.b[1129]) {
            s.store_scalar(971, 1.0);
        }

        s.b[1130] = (s.v[971] > 100000.0);
        s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });

        if (s.b[1110] && s.b[1130]) {
            s.store_scalar(971, 100000.0);
        }

        s.b[1133] = (s.v[975] < 0.1);
        s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });

        if (s.b[1110] && s.b[1133]) {
            s.store_scalar(975, 0.1);
        }

        s.b[1134] = (s.v[975] > 4.0);
        s.store_scalar(1134, if s.b[1134] { 1.0 } else { 0.0 });

        if (s.b[1110] && s.b[1134]) {
            s.store_scalar(975, 4.0);
        }

        s.b[1137] = (s.v[972] < 0.0);
        s.store_scalar(1137, if s.b[1137] { 1.0 } else { 0.0 });

        if (s.b[1110] && s.b[1137]) {
            s.store_scalar(972, 0.0);
        }

        s.b[1138] = (s.v[972] > 5.0);
        s.store_scalar(1138, if s.b[1138] { 1.0 } else { 0.0 });

        if (s.b[1110] && s.b[1138]) {
            s.store_scalar(972, 5.0);
        }

        s.b[1139] = (s.v[963] == 3.0);
        s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });

        s.b[1142] = (s.v[964] < 5000000000000000.0);
        s.store_scalar(1142, if s.b[1142] { 1.0 } else { 0.0 });

        if (((!s.b[1110]) && s.b[1139]) && s.b[1142]) {
            s.store_scalar(964, 5000000000000000.0);
        }

        s.b[1143] = (s.v[964] > 1e18);
        s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });

        if (((!s.b[1110]) && s.b[1139]) && s.b[1143]) {
            s.store_scalar(964, 1e18);
        }

        s.b[1146] = (s.v[965] < 1e-8);
        s.store_scalar(1146, if s.b[1146] { 1.0 } else { 0.0 });

        if (((!s.b[1110]) && s.b[1139]) && s.b[1146]) {
            s.store_scalar(965, 1e-8);
        }

        s.b[1147] = (s.v[965] > 1e-6);
        s.store_scalar(1147, if s.b[1147] { 1.0 } else { 0.0 });

        if (((!s.b[1110]) && s.b[1139]) && s.b[1147]) {
            s.store_scalar(965, 1e-6);
        }

        s.b[1150] = (s.v[966] < 1.0);
        s.store_scalar(1150, if s.b[1150] { 1.0 } else { 0.0 });

        if (((!s.b[1110]) && s.b[1139]) && s.b[1150]) {
            s.store_scalar(966, 1.0);
        }

        s.b[1151] = (s.v[966] > 10000000000.0);
        s.store_scalar(1151, if s.b[1151] { 1.0 } else { 0.0 });

        if (((!s.b[1110]) && s.b[1139]) && s.b[1151]) {
            s.store_scalar(966, 10000000000.0);
        }

        s.b[1154] = (s.v[971] < 100.0);
        s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });

        if (((!s.b[1110]) && s.b[1139]) && s.b[1154]) {
            s.store_scalar(971, 100.0);
        }

        s.b[1155] = (s.v[971] > 2000000000.0);
        s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });

        if (((!s.b[1110]) && s.b[1139]) && s.b[1155]) {
            s.store_scalar(971, 2000000000.0);
        }

        s.b[1158] = (s.v[972] < 0.0);
        s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });

        if (((!s.b[1110]) && s.b[1139]) && s.b[1158]) {
            s.store_scalar(972, 0.0);
        }

        s.b[1159] = (s.v[972] > 5.0);
        s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });

        if (((!s.b[1110]) && s.b[1139]) && s.b[1159]) {
            s.store_scalar(972, 5.0);
        }

        s.store_scalar(543, p.p96);

        s.b[1168] = (s.v[543] < p.p95);
        s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });

        if s.b[1168] {
            s.store_scalar(543, p.p95);
        }

        s.b[1169] = (s.v[543] > 5e-7);
        s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });

        if s.b[1169] {
            s.store_scalar(543, 5e-7);
        }

        s.store_scalar(545, (p.p120 / ((100.0) as f64).powf(p.p122)));

        s.store_scalar(546, (p.p123 / ((100.0) as f64).powf(p.p129)));

        s.store_scalar(547, (p.p198 / ((100.0) as f64).powf(p.p199)));

        s.store_scalar(548, (p.p200 / ((100.0) as f64).powf(p.p201)));

        s.store_scalar(549, (p.p183 / ((100.0) as f64).powf(p.p184)));

        s.store_scalar(550, (p.p202 / ((100.0) as f64).powf(p.p203)));

        s.store_scalar(551, (p.p190 / ((100.0) as f64).powf(p.p191)));

        s.store_scalar(552, (p.p186 / 100.0));

        s.store_scalar(553, (p.p192 / 100.0));

        s.store_scalar(554, (p.p73 * 100.0));

        s.store_scalar(555, (p.p311 / 100.0));

        s.store_scalar(556, (p.p312 / 100.0));

        s.store_scalar(557, (p.p313 / 100.0));

        s.store_scalar(558, (p.p314 / 100.0));

        s.store_scalar(544, (p.p336 / 1e-6));

        s.store_scalar(559, (p.p255 * 100.0));

        s.store_scalar(560, (p.p248 * 100.0));

        s.store_scalar(561, (p.p249 * 100.0));

        s.store_scalar(562, (p.p251 / 10000.0));

        s.store_scalar(563, (p.p266 * 10000.0));

        s.store_scalar(564, (p.p275 / 100.0));

        s.store_scalar(565, (p.p272 / 10000.0));

        s.store_scalar(572, (p.p273 / 10000.0));

        s.store_scalar(567, (p.p409 / 10000.0));

        s.store_scalar(568, (p.p412 / 100.0));

        s.store_scalar(569, (p.p413 / 10000.0));

        s.store_scalar(570, (p.p414 / 100.0));

        s.store_scale(964, 964, 1000000.0);

        s.store_scalar(489, (p.p453 / 1e-6));

        s.store_scalar(764, (p.p274 + 273.15));

        s.store_scalar(582, (p.p0 + p.p116));

        s.store_scalar(583, ((p.p1 / p.p7) + p.p117));

        s.store_scalar(576, (s.v[582] * 1000000.0));

        s.store_scalar(580, (s.v[583] * 1000000.0));

        s.store_scalar(774, ((s.v[576]) as f64).powf(p.p553));

        s.store_scalar(775, ((s.v[580]) as f64).powf(p.p554));

        s.store_scalar(776, (s.v[774] * s.v[775]));

        s.store_scalar(454, (((p.p89 + (p.p555 / s.v[774])) + (p.p643 / s.v[775])) + (p.p731 / s.v[776])));

        s.store_scalar(455, (((p.p92 + (p.p556 / s.v[774])) + (p.p644 / s.v[775])) + (p.p732 / s.v[776])));

        s.store_scalar(456, (((p.p93 + (p.p557 / s.v[774])) + (p.p645 / s.v[775])) + (p.p733 / s.v[776])));

        s.store_scalar(457, (((p.p94 + (p.p558 / s.v[774])) + (p.p646 / s.v[775])) + (p.p734 / s.v[776])));

        s.store_scalar(458, (((p.p110 + (p.p559 / s.v[774])) + (p.p647 / s.v[775])) + (p.p735 / s.v[776])));

        s.store_scalar(459, (((p.p111 + (p.p560 / s.v[774])) + (p.p648 / s.v[775])) + (p.p736 / s.v[776])));

        s.store_scalar(460, (((p.p112 + (p.p561 / s.v[774])) + (p.p649 / s.v[775])) + (p.p737 / s.v[776])));

        s.store_scalar(461, (((p.p126 + (p.p562 / s.v[774])) + (p.p650 / s.v[775])) + (p.p738 / s.v[776])));

        s.store_scalar(462, (((p.p136 + (p.p563 / s.v[774])) + (p.p651 / s.v[775])) + (p.p739 / s.v[776])));

        s.store_scalar(463, (((p.p138 + (p.p564 / s.v[774])) + (p.p652 / s.v[775])) + (p.p740 / s.v[776])));

        s.store_scalar(464, (((p.p141 + (p.p565 / s.v[774])) + (p.p653 / s.v[775])) + (p.p741 / s.v[776])));

        s.store_scalar(465, (((p.p144 + (p.p566 / s.v[774])) + (p.p654 / s.v[775])) + (p.p742 / s.v[776])));

        s.store_scalar(466, (((p.p145 + (p.p567 / s.v[774])) + (p.p655 / s.v[775])) + (p.p743 / s.v[776])));

        s.store_scalar(467, (((p.p146 + (p.p568 / s.v[774])) + (p.p656 / s.v[775])) + (p.p744 / s.v[776])));

        s.store_scalar(468, (((p.p147 + (p.p569 / s.v[774])) + (p.p657 / s.v[775])) + (p.p745 / s.v[776])));

        s.store_scalar(469, (((p.p148 + (p.p570 / s.v[774])) + (p.p658 / s.v[775])) + (p.p746 / s.v[776])));

        s.store_scalar(470, (((p.p149 + (p.p571 / s.v[774])) + (p.p659 / s.v[775])) + (p.p747 / s.v[776])));

        s.store_scalar(471, (((p.p151 + (p.p572 / s.v[774])) + (p.p660 / s.v[775])) + (p.p748 / s.v[776])));

        s.store_scalar(472, (((p.p154 + (p.p573 / s.v[774])) + (p.p661 / s.v[775])) + (p.p749 / s.v[776])));

        s.store_scalar(473, (((p.p157 + (p.p574 / s.v[774])) + (p.p662 / s.v[775])) + (p.p750 / s.v[776])));

        s.store_scalar(474, (((p.p158 + (p.p575 / s.v[774])) + (p.p663 / s.v[775])) + (p.p751 / s.v[776])));

        s.store_scalar(475, (((p.p159 + (p.p576 / s.v[774])) + (p.p664 / s.v[775])) + (p.p752 / s.v[776])));

        s.store_scalar(476, (((p.p161 + (p.p577 / s.v[774])) + (p.p665 / s.v[775])) + (p.p753 / s.v[776])));

        s.store_scalar(477, (((p.p169 + (p.p578 / s.v[774])) + (p.p666 / s.v[775])) + (p.p754 / s.v[776])));

        s.store_scalar(478, (((p.p170 + (p.p579 / s.v[774])) + (p.p667 / s.v[775])) + (p.p755 / s.v[776])));

        s.store_scalar(479, (((p.p172 + (p.p580 / s.v[774])) + (p.p668 / s.v[775])) + (p.p756 / s.v[776])));

        s.store_scalar(480, (((p.p177 + (p.p581 / s.v[774])) + (p.p669 / s.v[775])) + (p.p757 / s.v[776])));

        s.store_scalar(481, (((p.p179 + (p.p582 / s.v[774])) + (p.p670 / s.v[775])) + (p.p758 / s.v[776])));

        s.store_scalar(482, (((p.p180 + (p.p583 / s.v[774])) + (p.p671 / s.v[775])) + (p.p759 / s.v[776])));

        s.store_scalar(483, (((p.p185 + (p.p584 / s.v[774])) + (p.p672 / s.v[775])) + (p.p760 / s.v[776])));

        s.store_scalar(484, (((p.p182 + (p.p585 / s.v[774])) + (p.p673 / s.v[775])) + (p.p761 / s.v[776])));

        s.store_scalar(485, (((p.p181 + (p.p586 / s.v[774])) + (p.p674 / s.v[775])) + (p.p762 / s.v[776])));

        s.store_scalar(486, (((p.p187 + (p.p587 / s.v[774])) + (p.p675 / s.v[775])) + (p.p763 / s.v[776])));

        s.store_scalar(487, (((p.p188 + (p.p588 / s.v[774])) + (p.p676 / s.v[775])) + (p.p764 / s.v[776])));

        s.store_scalar(488, (((p.p189 + (p.p589 / s.v[774])) + (p.p677 / s.v[775])) + (p.p765 / s.v[776])));

        s.store_scalar(490, (((p.p194 + (p.p590 / s.v[774])) + (p.p678 / s.v[775])) + (p.p766 / s.v[776])));

        s.store_scalar(491, (((p.p195 + (p.p591 / s.v[774])) + (p.p679 / s.v[775])) + (p.p767 / s.v[776])));

        s.store_scalar(492, (((p.p196 + (p.p592 / s.v[774])) + (p.p680 / s.v[775])) + (p.p768 / s.v[776])));

        s.store_scalar(493, (((p.p197 + (p.p593 / s.v[774])) + (p.p681 / s.v[775])) + (p.p769 / s.v[776])));

        s.store_scalar(494, (((p.p204 + (p.p594 / s.v[774])) + (p.p682 / s.v[775])) + (p.p770 / s.v[776])));

        s.store_scalar(495, (((p.p205 + (p.p595 / s.v[774])) + (p.p683 / s.v[775])) + (p.p771 / s.v[776])));

        s.store_scalar(496, (((p.p210 + (p.p596 / s.v[774])) + (p.p684 / s.v[775])) + (p.p772 / s.v[776])));

        s.store_scalar(497, (((p.p211 + (p.p597 / s.v[774])) + (p.p685 / s.v[775])) + (p.p773 / s.v[776])));

        s.store_scalar(498, (((p.p212 + (p.p598 / s.v[774])) + (p.p686 / s.v[775])) + (p.p774 / s.v[776])));

        s.store_scalar(499, (((p.p214 + (p.p599 / s.v[774])) + (p.p687 / s.v[775])) + (p.p775 / s.v[776])));

        s.store_scalar(500, (((p.p215 + (p.p600 / s.v[774])) + (p.p688 / s.v[775])) + (p.p776 / s.v[776])));

        s.store_scalar(501, (((p.p216 + (p.p601 / s.v[774])) + (p.p689 / s.v[775])) + (p.p777 / s.v[776])));

        s.store_scalar(502, (((p.p217 + (p.p602 / s.v[774])) + (p.p690 / s.v[775])) + (p.p778 / s.v[776])));

        s.store_scalar(503, (((p.p218 + (p.p603 / s.v[774])) + (p.p691 / s.v[775])) + (p.p779 / s.v[776])));

        s.store_scalar(504, (((p.p219 + (p.p604 / s.v[774])) + (p.p692 / s.v[775])) + (p.p780 / s.v[776])));

        s.store_scalar(505, (((p.p269 + (p.p605 / s.v[774])) + (p.p693 / s.v[775])) + (p.p781 / s.v[776])));

        s.store_scalar(506, (((p.p268 + (p.p606 / s.v[774])) + (p.p694 / s.v[775])) + (p.p782 / s.v[776])));

        s.store_scalar(507, (((p.p226 + (p.p607 / s.v[774])) + (p.p695 / s.v[775])) + (p.p783 / s.v[776])));

        s.store_scalar(508, (((p.p227 + (p.p608 / s.v[774])) + (p.p696 / s.v[775])) + (p.p784 / s.v[776])));

        s.store_scalar(509, (((p.p228 + (p.p609 / s.v[774])) + (p.p697 / s.v[775])) + (p.p785 / s.v[776])));

        s.store_scalar(510, (((p.p232 + (p.p610 / s.v[774])) + (p.p698 / s.v[775])) + (p.p786 / s.v[776])));

        s.store_scalar(511, (((p.p240 + (p.p611 / s.v[774])) + (p.p699 / s.v[775])) + (p.p787 / s.v[776])));

        s.store_scalar(512, (((p.p241 + (p.p612 / s.v[774])) + (p.p700 / s.v[775])) + (p.p788 / s.v[776])));

        s.store_scalar(513, (((p.p245 + (p.p613 / s.v[774])) + (p.p701 / s.v[775])) + (p.p789 / s.v[776])));

        s.store_scalar(514, (((p.p246 + (p.p614 / s.v[774])) + (p.p702 / s.v[775])) + (p.p790 / s.v[776])));

        s.store_scalar(515, (((p.p247 + (p.p615 / s.v[774])) + (p.p703 / s.v[775])) + (p.p791 / s.v[776])));

        s.store_scalar(516, (((p.p250 + (p.p616 / s.v[774])) + (p.p704 / s.v[775])) + (p.p792 / s.v[776])));

        s.store_scalar(517, (((p.p253 + (p.p617 / s.v[774])) + (p.p705 / s.v[775])) + (p.p793 / s.v[776])));

        s.store_scalar(518, (((p.p254 + (p.p618 / s.v[774])) + (p.p706 / s.v[775])) + (p.p794 / s.v[776])));

        s.store_scalar(519, (((p.p256 + (p.p619 / s.v[774])) + (p.p707 / s.v[775])) + (p.p795 / s.v[776])));

        s.store_scalar(520, (((p.p257 + (p.p620 / s.v[774])) + (p.p708 / s.v[775])) + (p.p796 / s.v[776])));

        s.store_scalar(522, (((p.p265 + (p.p622 / s.v[774])) + (p.p710 / s.v[775])) + (p.p798 / s.v[776])));

        s.store_scalar(523, (((p.p278 + (p.p623 / s.v[774])) + (p.p711 / s.v[775])) + (p.p799 / s.v[776])));

        s.store_scalar(524, (((p.p281 + (p.p624 / s.v[774])) + (p.p712 / s.v[775])) + (p.p800 / s.v[776])));

        s.store_scalar(525, (((p.p79 + (p.p625 / s.v[774])) + (p.p713 / s.v[775])) + (p.p801 / s.v[776])));

        s.store_scalar(526, (((p.p86 + (p.p626 / s.v[774])) + (p.p714 / s.v[775])) + (p.p802 / s.v[776])));

        s.store_scalar(528, (((p.p76 + (p.p628 / s.v[774])) + (p.p716 / s.v[775])) + (p.p804 / s.v[776])));

        s.store_scalar(529, (((p.p81 + (p.p629 / s.v[774])) + (p.p717 / s.v[775])) + (p.p805 / s.v[776])));

        s.store_scalar(530, (((p.p74 + (p.p630 / s.v[774])) + (p.p718 / s.v[775])) + (p.p806 / s.v[776])));

        s.store_scalar(531, (((p.p298 + (p.p631 / s.v[774])) + (p.p719 / s.v[775])) + (p.p807 / s.v[776])));

        s.store_scalar(532, (((p.p83 + (p.p632 / s.v[774])) + (p.p720 / s.v[775])) + (p.p808 / s.v[776])));

        s.store_scalar(533, (((p.p84 + (p.p633 / s.v[774])) + (p.p721 / s.v[775])) + (p.p809 / s.v[776])));

        s.store_scalar(534, (((p.p62 + (p.p634 / s.v[774])) + (p.p722 / s.v[775])) + (p.p810 / s.v[776])));

        s.store_scalar(535, (((p.p59 + (p.p635 / s.v[774])) + (p.p723 / s.v[775])) + (p.p811 / s.v[776])));

        s.store_scalar(536, (((p.p60 + (p.p636 / s.v[774])) + (p.p724 / s.v[775])) + (p.p812 / s.v[776])));

        s.store_scalar(537, (((p.p85 + (p.p637 / s.v[774])) + (p.p725 / s.v[775])) + (p.p813 / s.v[776])));

        s.store_scalar(538, (((p.p82 + (p.p638 / s.v[774])) + (p.p726 / s.v[775])) + (p.p814 / s.v[776])));

        s.store_scalar(539, (((p.p61 + (p.p639 / s.v[774])) + (p.p727 / s.v[775])) + (p.p815 / s.v[776])));

        s.store_scalar(540, (((p.p75 + (p.p640 / s.v[774])) + (p.p728 / s.v[775])) + (p.p816 / s.v[776])));

        s.store_scalar(541, (((p.p80 + (p.p641 / s.v[774])) + (p.p729 / s.v[775])) + (p.p817 / s.v[776])));

        s.store_scalar(542, (((p.p77 + (p.p642 / s.v[774])) + (p.p730 / s.v[775])) + (p.p818 / s.v[776])));

        s.store_scalar(818, (((p.p493 + (p.p824 / s.v[774])) + (p.p839 / s.v[775])) + (p.p854 / s.v[776])));

        s.store_scalar(819, (((p.p494 + (p.p825 / s.v[774])) + (p.p840 / s.v[775])) + (p.p855 / s.v[776])));

        s.store_scalar(820, (((p.p496 + (p.p826 / s.v[774])) + (p.p841 / s.v[775])) + (p.p856 / s.v[776])));

        s.store_scalar(822, (((p.p515 + (p.p828 / s.v[774])) + (p.p843 / s.v[775])) + (p.p858 / s.v[776])));

        s.store_scalar(823, (((p.p516 + (p.p829 / s.v[774])) + (p.p844 / s.v[775])) + (p.p859 / s.v[776])));

        s.store_scalar(824, (((p.p517 + (p.p830 / s.v[774])) + (p.p845 / s.v[775])) + (p.p860 / s.v[776])));

        s.store_scalar(825, (((p.p519 + (p.p831 / s.v[774])) + (p.p846 / s.v[775])) + (p.p861 / s.v[776])));

        s.store_scalar(827, (((p.p538 + (p.p833 / s.v[774])) + (p.p848 / s.v[775])) + (p.p863 / s.v[776])));

        s.b[1185] = (s.v[963] != 0.0);
        s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });

        if s.b[1185] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p342));
            s.store_mul_offset_ad_rhs(964, 964, A::div_from_scalar(p.p341, s.ad_value(337)), 1.0);
        }

        s.b[1186] = (s.v[964] < 1e21);
        s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });

        if (s.b[1185] && s.b[1186]) {
            s.store_scalar(964, 1e21);
        }

        if s.b[1185] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p369));
            s.store_scaled_offset_ad(973, A::div_from_scalar(p.p368, s.ad_value(337)), 1.0, s.v[973]);
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p362));
            s.store_scaled_offset_ad(972, A::div_from_scalar(p.p361, s.ad_value(337)), 1.0, p.p360);
        }

        s.b[1187] = (s.v[972] < 0.0);
        s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });

        if (s.b[1185] && s.b[1187]) {
            s.store_scalar(972, 0.0);
        }

        if s.b[1185] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p348));
            s.store_scaled_offset_ad(966, A::div_from_scalar(p.p347, s.ad_value(337)), 1.0, p.p346);
        }

        s.b[1188] = (s.v[966] < 1.0);
        s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1185] && s.b[1188]) {
            s.store_scalar(966, 1.0);
        }

        if s.b[1185] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p351));
            s.store_scaled_offset_ad(968, A::div_from_scalar(p.p350, s.ad_value(337)), 1.0, p.p349);
        }

        s.b[1189] = (s.v[968] < 0.0);
        s.store_scalar(1189, if s.b[1189] { 1.0 } else { 0.0 });

        if (s.b[1185] && s.b[1189]) {
            s.store_scalar(968, 0.0);
        }

        if s.b[1185] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p357));
            s.store_scaled_offset_ad(967, A::div_from_scalar(p.p356, s.ad_value(337)), 1.0, p.p354);
        }

        s.b[1190] = (s.v[967] < 0.0);
        s.store_scalar(1190, if s.b[1190] { 1.0 } else { 0.0 });

        if (s.b[1185] && s.b[1190]) {
            s.store_scalar(967, 0.0);
        }

        if s.b[1185] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p359));
            s.store_scaled_offset_ad(969, A::div_from_scalar(p.p358, s.ad_value(337)), 1.0, p.p355);
        }

        s.b[1191] = (s.v[969] < 0.0);
        s.store_scalar(1191, if s.b[1191] { 1.0 } else { 0.0 });

        if (s.b[1185] && s.b[1191]) {
            s.store_scalar(969, 0.0);
        }

        if s.b[1185] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p373));
            s.store_scaled_offset_ad(974, A::div_from_scalar(p.p372, s.ad_value(337)), 1.0, s.v[974]);
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p375));
            s.store_mul_offset_ad_rhs(975, 975, A::div_from_scalar(p.p374, s.ad_value(337)), 1.0);
        }

        s.b[1192] = (s.v[975] < 0.1);
        s.store_scalar(1192, if s.b[1192] { 1.0 } else { 0.0 });

        if (s.b[1185] && s.b[1192]) {
            s.store_scalar(975, 0.1);
        }

        if (!s.b[1185]) {
            s.store_scalar(964, 0.0);
            s.store_scalar(973, 0.0);
            s.store_scalar(972, 0.0);
            s.store_scalar(966, 0.0);
            s.store_scalar(968, 0.0);
            s.store_scalar(967, 0.0);
            s.store_scalar(969, 0.0);
            s.store_scalar(974, 0.0);
            s.store_scalar(975, 0.0);
        }

        s.b[1244] = ((s.v[450] * s.v[451]) > 1.0);
        s.store_scalar(1244, if s.b[1244] { 1.0 } else { 0.0 });

        if s.b[1244] {
            s.store_div_from_scalar(450, 1.0, 451);
        }

        s.b[1246] = ((p.p40 == 1.0) && (((p.p19 > 0.0) && (s.v[459] == 0.0)) || ((p.p18 > 0.0) && (s.v[460] == 0.0))));
        s.store_scalar(1246, if s.b[1246] { 1.0 } else { 0.0 });

        if s.b[1246] {
            s.store_scalar(449, 0.0);
        }

        if (!s.b[1246]) {
            s.store_scalar(449, p.p40);
        }

        s.b[1247] = (s.v[449] == 1.0);
        s.store_scalar(1247, if s.b[1247] { 1.0 } else { 0.0 });

        if s.b[1247] {
            s.store_scalar(75, (if (p.p19 > 0.0) { 1.0 } else { 0.0 }));
        }

        if s.b[1247] {
            s.store_scalar(76, (if (p.p18 > 0.0) { 1.0 } else { 0.0 }));
        }

        s.b[1248] = ((p.p17 == 0.0) || (p.p17 == 2.0));
        s.store_scalar(1248, if s.b[1248] { 1.0 } else { 0.0 });

        if ((!s.b[1247]) && s.b[1248]) {
            s.store_scalar(75, 0.0);
            s.store_scalar(76, 0.0);
        }

        if ((!s.b[1247]) && (!s.b[1248])) {
            s.store_scalar(335, (((p.p130 * p.p2) * p.p7) + (((s.v[530] + s.v[538]) * (((p.p67 * s.v[536]) * 1000000.0) + s.v[534])) * (((p.p68 * p.p100) * 1000000.0) + p.p101))));
        }

        if ((!s.b[1247]) && (!s.b[1248])) {
            s.store_scalar(75, (if (s.v[335] > 0.0) { 1.0 } else { 0.0 }));
        }

        if ((!s.b[1247]) && (!s.b[1248])) {
            s.store_scalar(335, (((p.p131 * p.p3) * p.p7) + ((s.v[540] * (((p.p69 * s.v[536]) * 1000000.0) + s.v[534])) * (((p.p70 * p.p100) * 1000000.0) + p.p101))));
        }

        if ((!s.b[1247]) && (!s.b[1248])) {
            s.store_scalar(76, (if (s.v[335] > 0.0) { 1.0 } else { 0.0 }));
        }

        s.store_scalar(571, (p.p12 / 1e-6));

        s.store_scalar(554, (p.p73 * 100.0));

        s.store_scalar(463, (s.v[463] / 1e-6));

        s.store_scalar(464, (s.v[464] / 1e-6));

        s.store_scalar(494, (s.v[494] / 1e-6));

        s.store_scalar(459, (s.v[459] / 1e-6));

        s.store_scalar(460, (s.v[460] / 1e-6));

        s.store_scalar(502, (s.v[502] / 100.0));

        s.store_scalar(499, (s.v[499] / 100.0));

        s.store_scalar(454, (s.v[454] / 100.0));

        s.store_scalar(510, (s.v[510] * 10000.0));

        s.store_scalar(517, (s.v[517] / 100.0));

        s.store_scalar(518, (s.v[518] * 100.0));

        s.store_scalar(514, (s.v[514] * 100.0));

        s.store_scalar(520, (s.v[520] * 100.0));

        s.store_scalar(491, (s.v[491] * 100.0));

        s.store_scalar(511, (s.v[511] / 10.0));

        s.store_scalar(512, (s.v[512] * 100.0));

        s.store_scalar(522, (s.v[522] / 100.0));

        s.store_scalar(528, (s.v[528] / 1e-6));

        s.store_scalar(531, (s.v[531] / 100.0));

        s.store_scalar(532, (s.v[532] / 100.0));

        s.store_scalar(533, (s.v[533] / 100.0));

        s.store_scalar(538, (s.v[538] / 100.0));

        s.store_scalar(541, (s.v[541] / 100.0));

        s.store_scalar(458, (-s.v[458]));

        s.store_scale(973, 973, 0.01);

        s.store_scalar(81, p.p28);

        s.b[82] = ((p.p133 != 0.0) || (p.p134 != 0.0));
        s.store_scalar(82, if s.b[82] { 1.0 } else { 0.0 });

        s.b[1250] = (((p.p235 == 0.0) && (p.p237 == 0.0)) || (p.p236 == 0.0));
        s.store_scalar(1250, if s.b[1250] { 1.0 } else { 0.0 });

        if s.b[1250] {
            s.store_scalar(765, 0.0);
        }

        if (!s.b[1250]) {
            s.store_scalar(765, 1.0);
        }

        s.store_scalar(581, (s.v[580] * s.v[576]));

        s.store_scalar(777, (p.p289 * 1000000.0));

        s.store_scalar(616, (s.v[457] - (s.v[764] * (9.025e-5 + (s.v[764] * 1e-7)))));

        s.store_scalar(617, (8.8541878e-12 * p.p267));

        s.copy_ad(618, 452);

        s.b[1251] = (s.v[471] == 0.0);
        s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });

        if s.b[1251] {
            s.store_scalar(615, 0.0);
            s.store_scalar(642, 0.0);
        }

        if (!s.b[1251]) {
            s.store_scalar(615, 1.0);
            s.store_scalar(642, ((((1.0 + (1.0 / s.v[576]))) as f64).powf(p.p153) * s.v[471]));
        }

        s.store_scalar(619, (1.0 + (((s.v[576]) as f64).powf(p.p229) * p.p230)));

        s.store_scalar(335, ((1.0 / (p.p118 + (0.5 * p.p0))) + (1.0 / (p.p119 + (0.5 * p.p0)))));

        s.store_scalar(589, (2.0 / s.v[335]));

        s.b[1252] = (((p.p8 > 0.0) && (p.p9 > 0.0)) && ((p.p7 == 1.0) || ((p.p7 > 1.0) && (p.p10 > 0.0))));
        s.store_scalar(1252, if s.b[1252] { 1.0 } else { 0.0 });

        if s.b[1252] {
            s.store_scalar(335, 0.0);
            s.store_scalar(721, 0.0);
        }

        let mut assign10820_loop_guard: usize = 0;
        while {
            let assign10820_cond_e5729: f64 = if (s.b[1252] && (s.v[721] < p.p7)) { 1.0 } else { 0.0 };
            assign10820_cond_e5729 != 0.0
        } {
            assign10820_loop_guard += 1;
            assert!(assign10820_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[1252] {
                s.store_add_scaled_inputs3_mixed_iaa(335, 335, 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(721), (p.p10 + p.p0), (p.p8 + (0.5 * p.p0)))), 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(721), (p.p10 + p.p0), (p.p9 + (0.5 * p.p0)))), 1.0);
                s.store_offset(721, 721, 1.0);
            }
        }

        if s.b[1252] {
            s.store_div_from_scalar(588, (2.0 * p.p7), 335);
        }

        if (!s.b[1252]) {
            s.store_scalar(588, 0.0);
        }

        s.store_scalar(773, s.v[528]);

        s.store_scalar(620, s.v[476]);

        s.store_scalar(621, s.v[464]);

        s.store_scalar(622, s.v[463]);

        s.b[1253] = ((p.p32 == 1.0) && s.b[623]);
        s.store_scalar(1253, if s.b[1253] { 1.0 } else { 0.0 });

        if s.b[1253] {
            s.store_scalar(620, (s.v[620] * ((p.p282 * (((s.v[571]) as f64).ln() - ((s.v[622]) as f64).ln())) + 1.0)));
            s.store_scalar(621, ((s.v[621] + s.v[571]) - s.v[622]));
            s.store_scalar(773, ((s.v[773] + s.v[571]) - s.v[622]));
            s.store_scalar(622, s.v[571]);
        }

        s.store_scale(573, 620, ((1.0 + (p.p162 / ((s.v[580]) as f64).powf(p.p163))) * ((1.0 + (p.p164 / ((s.v[576]) as f64).powf(p.p165))) * (1.0 + (p.p167 / ((s.v[581]) as f64).powf(p.p168))))));

        s.b[1255] = (s.v[588] > 0.0);
        s.store_scalar(1255, if s.b[1255] { 1.0 } else { 0.0 });

        if s.b[1255] {
            s.store_scalar(335, (1.0 / (1.0 + s.v[500])));
            s.store_powf_ad(336, A::div_from_scalar(s.v[499], s.ad_value(588)), s.v[501]);
            s.store_scalar(337, (((s.v[499] / s.v[589])) as f64).powf(s.v[501]));
            s.store_div_scaled_product_offset_denominator(573, s.ad_value(573), A::offset(A::mul(s.ad_value(335), s.ad_value(336)), 1.0), 1.0, A::mul(s.ad_value(335), s.ad_value(337)), 1.0, 1.0);
        }

        s.store_scalar(624, ((p.p171 * (1.0 + (p.p173 / ((s.v[576]) as f64).powf(p.p176)))) * (1.0 + (p.p174 / ((s.v[580]) as f64).powf(p.p175)))));

        if (s.v[573] < 1e-25) {
            s.store_scalar(573, 1e-25);
        }

        if (s.v[624] < 1e-25) {
            s.store_scalar(624, 1e-25);
        }

        s.store_scalar(335, ((s.v[576]) as f64).powf(p.p156));

        s.store_scalar(625, (((s.v[472] * s.v[335]) / (s.v[335] + p.p155)) / 1.034943e-10));

        s.store_scalar(626, (s.v[473] / 1.034943e-10));

        s.store_scalar(627, ((p.p319 * (1.0 + (p.p320 / ((s.v[576]) as f64).powf(p.p321)))) * (1.0 + (p.p322 / ((s.v[580]) as f64).powf(p.p323)))));

        s.store_scalar(335, ((1.0 + (p.p386 / ((s.v[576]) as f64).powf(p.p387))) * (1.0 + (p.p388 / ((s.v[580]) as f64).powf(p.p389)))));

        s.store_scalar(633, (p.p384 * s.v[335]));

        s.store_scalar(634, (p.p385 * s.v[335]));

        s.store_scalar(574, (p.p97 + (s.v[545] / (((s.v[582] + p.p121)) as f64).powf(p.p122))));

        s.store_offset(575, 451, (s.v[545] / (((s.v[582] + p.p121)) as f64).powf(p.p122)));

        s.store_scalar(577, (p.p114 + (s.v[546] / (((s.v[583] + p.p128)) as f64).powf(p.p129))));

        s.store_scalar(578, (p.p295 + (s.v[546] / (((s.v[583] + p.p128)) as f64).powf(p.p129))));

        s.store_scalar(579, (p.p115 + (s.v[546] / (((s.v[583] + p.p128)) as f64).powf(p.p129))));

        s.store_sub_from_scalar_ad(162, s.v[582], A::offset(s.ad_value(575), s.v[574]));

        s.store_scalar(628, (s.v[582] + (p.p124 / ((s.v[581]) as f64).powf(p.p125))));

        s.store_scalar(629, (s.v[461] / ((s.v[581]) as f64).powf(p.p127)));

        s.store_scalar(335, (1.0 + (p.p206 / (((s.v[628] * 1000000.0)) as f64).powf(p.p207))));

        s.store_scalar(336, (1.0 + (p.p208 / ((s.v[580]) as f64).powf(p.p209))));

        s.store_scalar(495, ((s.v[495] * s.v[335]) * s.v[336]));

        s.store_scalar(163, (s.v[583] - (2.0 * s.v[577])));

        s.store_scalar(630, (s.v[583] - (2.0 * s.v[578])));

        s.store_scalar(631, (s.v[583] - (2.0 * s.v[579])));

        s.store_scalar(632, (s.v[163] * p.p7));

        s.store_scalar(635, (s.v[631] * p.p7));

        s.store_scale(584, 621, (1.0 + (p.p142 / ((s.v[580]) as f64).powf(p.p143))));

        s.store_scale(622, 622, (1.0 + (p.p233 / ((s.v[580]) as f64).powf(p.p234))));

        s.store_scale(335, 622, 1e-6);

        s.store_scale(336, 584, 1e-6);

        s.b[1263] = (s.v[335] < 1000000000000000.0);
        s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });

        if s.b[1263] {
            s.store_scalar(335, 1000000000000000.0);
        }

        s.store_scale(622, 335, 1000000.0);

        s.b[1265] = (s.v[336] < 1000000000000000.0);
        s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });

        if s.b[1265] {
            s.store_scalar(336, 1000000000000000.0);
        }

        s.store_scale(584, 336, 1000000.0);

        s.b[1266] = (s.v[588] > 0.0);
        s.store_scalar(1266, if s.b[1266] { 1.0 } else { 0.0 });

        if s.b[1266] {
            s.store_scalar(335, (1.0 / (1.0 + s.v[503])));
            s.store_powf_ad(336, A::div_from_scalar(s.v[502], s.ad_value(588)), s.v[504]);
            s.store_scalar(337, (((s.v[502] / s.v[589])) as f64).powf(s.v[504]));
            s.store_div_scaled_product_offset_denominator(585, s.ad_value(584), A::offset(A::mul(s.ad_value(335), s.ad_value(336)), 1.0), 1.0, A::mul(s.ad_value(335), s.ad_value(337)), 1.0, 1.0);
        }

        if (!s.b[1266]) {
            s.copy_ad(585, 584);
        }

        s.b[1267] = ((s.v[582] > p.p140) || (p.p140 <= 0.0));
        s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });

        if s.b[1267] {
            s.store_add_scaled_inputs(586, 622, ((s.v[582] - p.p140) * 1.0 / (s.v[582])), 585, (p.p140 * 1.0 / (s.v[582])));
        }

        if (!s.b[1267]) {
            s.store_add_scaled_inputs3_indices(586, 585, 1.0, 585, ((p.p140 - s.v[582]) * 1.0 / (p.p140)), 622, (-((p.p140 - s.v[582]) * 1.0 / (p.p140))));
        }

        s.store_scalar(337, ((0.5 * s.v[582]) - p.p140));

        s.store_scalar(781, ((s.v[337] - 1e-9) - 1e-10));

        s.store_scalar(782, ((4.0 * 1e-9) * 1e-10));

        if (!(s.v[782] > 0.0)) {
            s.store_scalar(782, (-s.v[782]));
        }

        s.store_sqrt_offset_input(782, 782, (s.v[781] * s.v[781]));

        s.store_scaled_offset_ad(334, A::div_from_scalar(s.v[781], s.ad_value(782)), 1.0, 0.5);

        s.store_offset_scaled(337, 782, 0.5, ((((s.v[781]) * (0.5))) + (1e-9)));

        s.store_div_from_scalar_offset_ad(335, 1.0, A::div_from_scalar(1.0, s.ad_value(337)), (1.0 / p.p220));

        if (0.0 >= s.v[335]) {
            s.store_scalar(336, 0.0);
        } else {
            s.copy_ad(336, 335);
        }

        s.store_add_scaled_product_right_sub(586, 586, 1.0, 336, 773, 622, 1.0 / (s.v[582]));

        s.store_scale(166, 586, 1.6021918e-19);

        s.store_scale(636, 166, 1.034943e-10);

        s.store_scale(637, 636, 2.0);

        s.b[1268] = ((s.v[582] <= (2.0 * p.p140)) && (p.p140 > 0.0));
        s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });

        if s.b[1268] {
            s.store_add_scaled_inputs4_indices(587, 585, 2.0, 585, (-(s.v[582] * 1.0 / (p.p140))), 622, (-(-(s.v[582] * 1.0 / (p.p140)))), 622, -1.0);
            s.store_ln_div(638, 587, 622);
        }

    }

    pub(super) fn stamp_reactive_block_4(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if (!s.b[1268]) {
            s.store_scalar(638, 0.0);
        }

        s.store_scalar(639, (((((2.0 * 1.6021918e-19) * s.v[494]) * 1.034943e-10)) as f64).sqrt());

        s.store_scalar(640, (1.0 / (s.v[494] * s.v[494])));

        s.store_scalar(641, ((1.0 + (s.v[542] / ((s.v[576]) as f64).powf(p.p231))) * (1.0 + (p.p238 / ((s.v[581]) as f64).powf(p.p239)))));

        s.store_scaled_ln_scaled_input(158, 586, 1.0 / (1.04e16), (2.0 / 38.68283));

        s.store_scaled_ln_scaled_input(159, 622, 1.0 / (1.04e16), (2.0 / 38.68283));

        s.b[1269] = (p.p51 == 1.0);
        s.store_scalar(1269, if s.b[1269] { 1.0 } else { 0.0 });

        if s.b[1269] {
            s.store_scalar(335, (p.p5 + (s.v[163] / (3.0 * p.p4))));
            s.store_scalar(336, (s.v[582] - p.p6));
        }

        s.b[1271] = (p.p130 > 0.0);
        s.store_scalar(1271, if s.b[1271] { 1.0 } else { 0.0 });

        if s.b[1271] {
            s.store_scalar(644, (p.p130 * p.p2));
            s.store_scalar(648, (p.p130 * p.p3));
        }

        if (!s.b[1271]) {
            s.store_scalar(644, 0.0);
            s.store_scalar(648, 0.0);
        }

        s.b[1272] = (p.p131 > 0.0);
        s.store_scalar(1272, if s.b[1272] { 1.0 } else { 0.0 });

        if s.b[1272] {
            s.store_scalar(648, (p.p131 * p.p3));
        }

        if (!s.b[1272]) {
            s.store_scalar(648, 0.0);
        }

        s.b[1273] = (s.v[449] == 0.0);
        s.store_scalar(1273, if s.b[1273] { 1.0 } else { 0.0 });

        s.b[1274] = ((s.v[530] > 0.0) || (s.v[540] > 0.0));
        s.store_scalar(1274, if s.b[1274] { 1.0 } else { 0.0 });

        if (s.b[1273] && s.b[1274]) {
            s.store_scalar(645, (1.0 + (p.p309 / ((s.v[581]) as f64).powf(p.p310))));
        }

        s.b[1275] = (s.v[538] != 0.0);
        s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });

        if ((s.b[1273] && s.b[1274]) && s.b[1275]) {
            s.store_scalar(341, (1.0 + (p.p303 / ((s.v[581]) as f64).powf(p.p304))));
            s.store_scalar(340, ((-p.p301) * ((s.v[576]) as f64).powf(p.p302)));
        }

        s.b[1276] = (s.v[340] > 60.0);
        s.store_scalar(1276, if s.b[1276] { 1.0 } else { 0.0 });

        if (((s.b[1273] && s.b[1274]) && s.b[1275]) && s.b[1276]) {
            s.store_scalar(340, 60.0);
        }

        if ((s.b[1273] && s.b[1274]) && s.b[1275]) {
            s.store_exp(340, 340);
            s.store_mul(646, 340, 341);
        }

        if ((s.b[1273] && s.b[1274]) && (!s.b[1275])) {
            s.store_scalar(646, 0.0);
        }

        if (s.b[1273] && (!s.b[1274])) {
            s.store_scalar(645, 0.0);
            s.store_scalar(646, 0.0);
        }

        s.b[1277] = (s.v[532] != 0.0);
        s.store_scalar(1277, if s.b[1277] { 1.0 } else { 0.0 });

        if (s.b[1273] && s.b[1277]) {
            s.store_scalar(336, (1.0 + (p.p307 / ((s.v[581]) as f64).powf(p.p308))));
            s.store_scalar(335, ((-p.p305) * ((s.v[576]) as f64).powf(p.p306)));
        }

        s.b[1278] = (s.v[335] > 60.0);
        s.store_scalar(1278, if s.b[1278] { 1.0 } else { 0.0 });

        if ((s.b[1273] && s.b[1277]) && s.b[1278]) {
            s.store_scalar(335, 60.0);
        }

        if (s.b[1273] && s.b[1277]) {
            s.store_exp(335, 335);
            s.store_scaled_mul(337, 336, 335, s.v[532]);
            s.store_scaled_add_sqrt_square_offset_rhs(647, 337, 337, ((((4.0 * 1e-6) / 100.0) * 1e-6) / 100.0), 0.5);
        }

        if (s.b[1273] && (!s.b[1277])) {
            s.store_scalar(647, 0.0);
        }

        if s.b[1273] {
            s.store_scalar(649, 0.0);
            s.store_scalar(614, 0.0);
            s.store_scalar(786, 0.0);
            s.store_scalar(652, 0.0);
            s.store_scalar(653, 0.0);
            s.store_scalar(654, 0.0);
        }

        if (!s.b[1273]) {
            s.store_sqrt_square_offset(649, 451, (p.p419 * p.p419));
            s.store_scalar(614, ((((p.p419 * p.p419) + (p.p97 * p.p97))) as f64).sqrt());
            s.store_scalar(786, (1.0 + (p.p424 / ((s.v[580]) as f64).powf(p.p425))));
            s.store_scalar(652, (1.0 + (p.p426 / ((s.v[576]) as f64).powf(p.p427))));
            s.store_scalar(653, (1.0 + (p.p428 / ((s.v[576]) as f64).powf(p.p429))));
            s.store_scalar(654, 1.0);
            s.store_scalar(645, 0.0);
            s.store_scalar(646, 0.0);
            s.store_scalar(647, 0.0);
        }

        s.b[1279] = (s.v[459] > 0.0);
        s.store_scalar(1279, if s.b[1279] { 1.0 } else { 0.0 });

        if s.b[1279] {
            s.store_scalar(650, ((2.0 * 1.034943e-10) / (1.6021918e-19 * s.v[459])));
            s.store_div_scaled_value_offset_denominator(651, s.ad_value(622), (((2.0 * 1.034943e-10) / 1.6021918e-19) * 1.0 / (s.v[459])), s.ad_value(622), s.v[459], 1.0);
        }

        if (!s.b[1279]) {
            s.store_scalar(650, 0.0);
            s.store_scalar(651, 0.0);
        }

        s.b[1284] = (p.p44 == 0.0);
        s.store_scalar(1284, if s.b[1284] { 1.0 } else { 0.0 });

        if s.b[1284] {
            s.store_scalar(335, ((p.p108 * s.v[576]) + p.p109));
        }

        s.b[1285] = (s.v[335] < 0.0);
        s.store_scalar(1285, if s.b[1285] { 1.0 } else { 0.0 });

        if (s.b[1284] && s.b[1285]) {
            s.store_scalar(335, 0.0);
        }

        if s.b[1284] {
            s.store_offset_ad(658, A::div_scaled_value_offset_denominator(s.ad_value(335), p.p107, s.ad_value(335), p.p107, 1.0), 1.0);
        }

        if (!s.b[1284]) {
            s.store_scalar(335, (p.p108 * s.v[576]));
        }

        s.b[1286] = (s.v[335] < 0.0);
        s.store_scalar(1286, if s.b[1286] { 1.0 } else { 0.0 });

        if ((!s.b[1284]) && s.b[1286]) {
            s.store_scalar(335, 0.0);
        }

        if (!s.b[1284]) {
            s.store_offset_ad(658, A::div_scaled_value_offset_denominator(s.ad_value(335), p.p107, s.ad_value(335), p.p107, 1.0), ((p.p109) + (1e-25)));
        }

        s.b[1288] = (s.v[658] < 0.1);
        s.store_scalar(1288, if s.b[1288] { 1.0 } else { 0.0 });

        if s.b[1288] {
            s.store_scalar(658, 0.1);
        }

        if (p.p23 != 0.0) {
            s.store_scalar(336, ((s.v[163]) as f64).powf(p.p201));
            s.store_div_scaled_value_offset_denominator(659, s.ad_value(336), (s.v[485] * (1.0 + (s.v[547] / ((s.v[582]) as f64).powf(p.p199)))), s.ad_value(336), s.v[548], 1.0);
            s.store_scalar(660, (s.v[484] * (1.0 + (s.v[549] / ((s.v[582]) as f64).powf(p.p184)))));
            s.store_scalar(661, (s.v[552] * (1.0 + (s.v[550] / ((s.v[582]) as f64).powf(p.p203)))));
            s.store_scalar(662, (s.v[481] * (1.0 + (s.v[551] / ((s.v[582]) as f64).powf(p.p191)))));
            s.store_scalar(663, (s.v[482] * (1.0 + (s.v[553] / s.v[582]))));
            s.copy_ad(668, 662);
            s.copy_ad(669, 663);
            s.copy_ad(665, 659);
            s.copy_ad(666, 660);
            s.copy_ad(667, 661);
        }

        if ((p.p23 != 0.0) && (p.p46 != 0.0)) {
            s.store_scalar(668, (s.v[486] * (1.0 + (s.v[551] / ((s.v[582]) as f64).powf(p.p191)))));
            s.store_scalar(669, (s.v[487] * (1.0 + (s.v[553] / s.v[582]))));
        }

        if (p.p23 != 0.0) {
            s.store_scalar(664, (p.p72 * (1.0 + (p.p102 / ((s.v[576]) as f64).powf(p.p103)))));
        }

        if (p.p23 == 0.0) {
            s.store_scalar(659, 0.0);
            s.store_scalar(660, 0.0);
            s.store_scalar(661, 0.0);
            s.store_scalar(662, 0.0);
            s.store_scalar(663, 0.0);
            s.store_scalar(664, 0.0);
            s.store_scalar(665, 0.0);
            s.store_scalar(666, 0.0);
            s.store_scalar(667, 0.0);
            s.store_scalar(668, 0.0);
            s.store_scalar(669, 0.0);
        }

        s.store_scalar(523, (if (s.v[523] != 0.0) { (s.v[523] * (1.0 + (p.p279 / ((s.v[576]) as f64).powf(p.p280)))) } else { 0.0 }));

        s.store_scalar(670, (((3.453133e-11 / (3.141592653589793 / 2.0)) * s.v[635]) * (((1.0 + (p.p225 / p.p95))) as f64).ln()));

        s.store_scalar(671, (if (p.p134 != 0.0) { (((1000000.0 * s.v[635]) * p.p134) / ((s.v[576]) as f64).powf(p.p135)) } else { 0.0 }));

        s.store_scalar(672, (p.p283 * ((s.v[576]) as f64).powf((-p.p286))));

        s.store_scalar(673, (p.p290 * ((s.v[576]) as f64).powf((-p.p291))));

        s.store_scalar(674, (p.p287 * (((s.v[576] + s.v[777])) as f64).powf((-p.p288))));

        s.store_scalar(766, (((s.v[541] / (s.v[365] * s.v[632])) * (1.0 + (p.p317 / ((s.v[576]) as f64).powf(p.p318)))) * (1.0 + (p.p315 / ((s.v[580]) as f64).powf(p.p316)))));

        s.store_scalar(766, (s.v[766] * (1.0 / ((p.p7) as f64).powf(p.p327))));

        s.store_scalar(675, ((((1.0 / ((p.p7) as f64).powf(p.p327)) / (s.v[365] * s.v[632])) * (1.0 + (p.p317 / ((s.v[576]) as f64).powf(p.p318)))) * (1.0 + (p.p315 / ((s.v[580]) as f64).powf(p.p316)))));

        s.b[1289] = ((p.p53 == 0.0) || (s.v[541] == 0.0));
        s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });

        if s.b[1289] {
            s.store_scalar(686, 0.0);
            s.store_scalar(687, 0.0);
            s.store_scalar(387, (ctx_temp + p.p11));
            s.copy_ad(388, 387);
            s.store_offset(387, 387, s.v[732]);
            s.store_offset(389, 388, (-s.v[764]));
            s.store_offset_square(390, 388, (-(s.v[764] * s.v[764])));
            s.store_offset(391, 387, (-s.v[764]));
            s.store_offset_square(392, 387, (-(s.v[764] * s.v[764])));
            s.store_scale(676, 387, 1.0 / (s.v[764]));
            s.store_ln(590, 676);
            s.store_sub_scaled_ad_lhs(393, A::sub_from_scalar(s.v[616], A::scale(s.ad_value(391), s.v[455])), 392, s.v[456]);
            s.store_sqrt(677, 393);
            s.store_div_from_scalar(335, 1.0, 387);
            s.store_scalar(336, (1.0 / s.v[764]));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(337, 335, p.p260, 336, (-p.p260), A::square(s.ad_value(335)), p.p261, A::square(s.ad_value(336)), (-p.p261), (s.v[616] + p.p259));
            s.store_sqrt(192, 337);
            s.store_mul(193, 337, 192);
            s.store_div_from_scalar_scaled_input(154, 1.6021918e-19, 387, 1.3806226e-23);
            s.store_div_from_scalar(155, 1.0, 154);
            s.store_square(156, 154);
            s.store_scalar(678, (1.6021918e-19 / (1.3806226e-23 * s.v[764])));
            s.store_scaled_mul_ad(394, A::exp_scaled_input(s.ad_value(590), 1.5), A::exp(A::add_scaled_product(s.ad_value(678), (s.v[616] / 2.0), s.ad_value(393), s.ad_value(154), (-1.0 / (2.0)))), 1.04e16);
            s.store_exp_scaled_input(335, 590, s.v[480]);
            s.store_div(679, 335, 573);
        }

        s.b[1290] = ((s.v[963] != 0.0) && (s.v[963] < 3.0));
        s.store_scalar(1290, if s.b[1290] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1290]) {
            s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div_scaled_product_by_product(210, s.ad_value(394), s.ad_value(394), 1.0, s.ad_value(964), s.ad_value(964), 1.0);
            s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));
            s.store_exp_scaled_input(335, 590, p.p380);
            s.store_div(977, 335, 971);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p379), p.p379));
            s.store_div(973, 973, 334);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1292] = (s.v[973] < 1000.0);
        s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });

        if ((s.b[1289] && s.b[1290]) && s.b[1292]) {
            s.store_scalar(973, 1000.0);
        }

        if (s.b[1289] && s.b[1290]) {
            s.store_div_ad_rhs(966, 966, A::powf(s.ad_value(676), p.p381));
            s.store_div_from_scalar_powf_ad(970, s.v[970], s.ad_value(676), p.p382);
        }

        s.b[1293] = (s.v[963] == 3.0);
        s.store_scalar(1293, if s.b[1293] { 1.0 } else { 0.0 });

        if ((s.b[1289] && (!s.b[1290])) && s.b[1293]) {
            s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div_scaled_product_by_product(210, s.ad_value(394), s.ad_value(394), 1.0, s.ad_value(964), s.ad_value(964), 1.0);
            s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));
            s.store_exp_scaled_input(335, 590, p.p380);
            s.store_div(977, 335, 971);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p379), p.p379));
            s.store_div(973, 973, 334);
        }

        s.b[1295] = (s.v[973] < 1000.0);
        s.store_scalar(1295, if s.b[1295] { 1.0 } else { 0.0 });

        if (((s.b[1289] && (!s.b[1290])) && s.b[1293]) && s.b[1295]) {
            s.store_scalar(973, 1000.0);
        }

        if ((s.b[1289] && (!s.b[1290])) && s.b[1293]) {
            s.store_div_ad_rhs(966, 966, A::powf(s.ad_value(676), p.p381));
            s.store_offset_scaled(976, 676, p.p365, (((((-1.0)) * (p.p365))) + (p.p364)));
        }

        if ((s.b[1289] && (!s.b[1290])) && (!s.b[1293])) {
            s.store_scalar(961, 0.0);
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_value_by_product(s.ad_value(586), s.v[489], s.ad_value(394), s.ad_value(394), 1.0));
            s.store_scalar(977, 0.0);
        }

        if s.b[1289] {
            s.store_mul(680, 638, 155);
            s.store_scale(335, 387, 1.0 / (s.v[764]));
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(335), 0.4, 1.8), 1.0, s.ad_value(335), s.ad_value(335), 0.1), A::scale_offset(s.ad_value(335), (-s.v[477]), s.v[477]));
        }

        s.b[1296] = (p.p39 != 2.0);
        s.store_scalar(1296, if s.b[1296] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1296]) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(389), p.p90, 1.0), 1.0, s.ad_value(390), p.p91));
        }

        if (s.b[1289] && (!s.b[1296])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(391), p.p90, 1.0), 1.0, s.ad_value(392), p.p91));
        }

        s.b[1298] = (p.p39 != 2.0);
        s.store_scalar(1298, if s.b[1298] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1298]) {
            s.store_add_scaled_inputs_ad_lhs(682, A::scale_offset(s.ad_value(389), p.p324, 1.0), s.v[627], 390, (p.p325 * s.v[627]));
            s.store_add_scaled_ad_lhs(335, A::scale_offset(s.ad_value(389), p.p390, 1.0), 390, p.p391);
            s.store_scale(688, 335, s.v[633]);
            s.store_scale(689, 335, s.v[634]);
        }

        if (s.b[1289] && (!s.b[1298])) {
            s.store_add_scaled_inputs_ad_lhs(682, A::scale_offset(s.ad_value(391), p.p324, 1.0), s.v[627], 392, (p.p325 * s.v[627]));
            s.store_add_scaled_ad_lhs(335, A::scale_offset(s.ad_value(391), p.p390, 1.0), 392, p.p391);
            s.store_scale(688, 335, s.v[633]);
            s.store_scale(689, 335, s.v[634]);
        }

        s.b[1300] = (s.v[682] < 0.0);
        s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1300]) {
            s.store_scalar(682, 0.0);
        }

        s.b[1302] = (s.v[688] < 0.0);
        s.store_scalar(1302, if s.b[1302] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1302]) {
            s.store_scalar(688, 0.0);
        }

        s.b[1304] = (s.v[689] < 0.0);
        s.store_scalar(1304, if s.b[1304] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1304]) {
            s.store_scalar(689, 0.0);
        }

        if (s.b[1289] && (p.p53 != 0.0)) {
            s.store_add_scaled_inputs_ad_lhs(766, A::scale_offset(s.ad_value(389), p.p328, s.v[541]), s.v[675], 390, (p.p329 * s.v[675]));
        }

        s.b[1306] = (s.v[766] < 0.0001);
        s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });

        if ((s.b[1289] && (p.p53 != 0.0)) && s.b[1306]) {
            s.store_scalar(766, 0.0001);
        }

        if s.b[1289] {
            s.store_add_scaled_ad_lhs(336, A::scale_offset(s.ad_value(389), p.p330, s.v[529]), 390, p.p331);
            s.store_offset(781, 336, (-0.05));
            s.store_scalar(782, 0.0);
        }

        if s.b[1289] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1289] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
            s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(336), (-0.05));
            s.store_scalar(782, (4.0 * 0.05));
        }

        if s.b[1289] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1289] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(683, 781, (-0.5), 782, (-0.5), 1.0);
            s.store_mul_scaled_ln_ad_rhs(157, 155, 2.0, A::div(s.ad_value(586), s.ad_value(394)));
            s.store_scalar(335, ((2.0 * 1.034943e-10) / 1.6021918e-19));
            s.store_sqrt_div(684, 335, 586);
            s.store_sqrt_div(685, 335, 621);
        }

        s.b[1307] = (s.v[963] == 0.0);
        s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1307]) {
            s.store_sqrt_mul_scaled_lhs(209, 586, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div(335, 394, 586);
            s.store_square(210, 335);
        }

        s.b[1308] = (s.v[963] == 0.0);
        s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });

        s.b[1309] = (s.v[459] != 0.0);
        s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });

        if ((s.b[1289] && s.b[1308]) && s.b[1309]) {
            s.store_mul_sqrt_ad_rhs(686, 209, A::div_from_scalar(s.v[459], s.ad_value(586)));
        }

        s.b[1310] = (s.v[460] != 0.0);
        s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });

        if ((s.b[1289] && s.b[1308]) && s.b[1310]) {
            s.store_mul_sqrt_ad_rhs(687, 209, A::div_from_scalar(s.v[460], s.ad_value(586)));
        }

        s.b[1311] = (s.v[459] != 0.0);
        s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });

        if ((s.b[1289] && (!s.b[1308])) && s.b[1311]) {
            s.store_mul_sqrt_ad_rhs(686, 209, A::div_from_scalar(s.v[459], s.ad_value(964)));
        }

        s.b[1312] = (s.v[460] != 0.0);
        s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });

        if ((s.b[1289] && (!s.b[1308])) && s.b[1312]) {
            s.store_mul_sqrt_ad_rhs(687, 209, A::div_from_scalar(s.v[460], s.ad_value(964)));
        }

        s.b[1313] = (s.v[449] == 0.0);
        s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });

        s.b[1314] = (s.v[530] > 0.0);
        s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });

        if ((s.b[1289] && s.b[1313]) && s.b[1314]) {
            s.store_scale(336, 645, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));
        }

        s.b[1315] = (p.p39 == 1.0);
        s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });

        if (((s.b[1289] && s.b[1313]) && s.b[1314]) && s.b[1315]) {
            s.store_mul_add_scaled_inputs_rhs(690, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[530]), 1.0, s.ad_value(390), s.v[556]);
            s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1314]) && s.b[1315]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1314]) && s.b[1315]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1314]) && (!s.b[1315])) {
            s.store_mul_add_scaled_inputs_rhs(690, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[530]), 1.0, s.ad_value(392), s.v[556]);
            s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1314]) && (!s.b[1315])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1314]) && (!s.b[1315])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));
        }

        if ((s.b[1289] && s.b[1313]) && (!s.b[1314])) {
            s.store_scalar(690, 0.0);
        }

        s.b[1316] = (s.v[540] > 0.0);
        s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });

        if ((s.b[1289] && s.b[1313]) && s.b[1316]) {
            s.store_scale(336, 645, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));
        }

        s.b[1317] = (p.p39 == 1.0);
        s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });

        if (((s.b[1289] && s.b[1313]) && s.b[1316]) && s.b[1317]) {
            s.store_mul_add_scaled_inputs_rhs(691, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[540]), 1.0, s.ad_value(390), s.v[556]);
            s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1316]) && s.b[1317]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1316]) && s.b[1317]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1316]) && (!s.b[1317])) {
            s.store_mul_add_scaled_inputs_rhs(691, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[540]), 1.0, s.ad_value(392), s.v[556]);
            s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1316]) && (!s.b[1317])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1316]) && (!s.b[1317])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));
        }

        if ((s.b[1289] && s.b[1313]) && (!s.b[1316])) {
            s.store_scalar(691, 0.0);
        }

        s.b[1318] = (s.v[538] > 0.0);
        s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });

        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {
            s.store_scale(338, 646, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));
            s.store_scalar(335, (((1.0 - s.v[535]) * p.p63) * 1000000.0));
            s.store_scalar(782, ((((p.p99 * p.p99) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());
            s.store_scaled_offset_ad(334, A::div_from_scalar(p.p99, s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_offset(336, 782, p.p99, 0.5);
        }

        s.b[1319] = (s.v[336] < 0.0);
        s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });

        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && s.b[1319]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {
            s.store_div_from_scalar(342, (-p.p98), 336);
            s.store_offset_scaled(337, 342, (p.p63 * 1000000.0), ((1.0) + (p.p98)));
            s.store_offset_add_scaled_product(781, s.ad_value(338), (-1.0), s.ad_value(337), s.ad_value(338), 1.0, (-0.01));
            s.store_scale(782, 338, (4.0 * 0.01));
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));
            s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));
        }

        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));
            s.store_offset_add_scaled_product(781, s.ad_value(341), 1.0, s.ad_value(335), s.ad_value(338), 1.0, (-5e-5));
            s.store_scalar(782, 0.0);
        }

        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
        }

        s.b[1320] = ((p.p39 == 0.0) || (p.p39 == 1.0));
        s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });

        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && s.b[1320]) {
            s.store_mul_add_scaled_inputs_rhs(692, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, s.ad_value(390), s.v[558]);
            s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && s.b[1320]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && s.b[1320]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && (!s.b[1320])) {
            s.store_mul_add_scaled_inputs_rhs(692, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, s.ad_value(392), s.v[558]);
            s.store_offset(781, 692, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && (!s.b[1320])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && (!s.b[1320])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(692, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {
            s.store_scale(338, 646, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));
            s.store_scalar(335, (((1.0 - s.v[535]) * p.p66) * 1000000.0));
            s.store_offset_scaled(337, 342, (p.p66 * 1000000.0), ((1.0) + (p.p98)));
            s.store_offset_add_scaled_product(781, s.ad_value(338), (-1.0), s.ad_value(337), s.ad_value(338), 1.0, (-0.01));
            s.store_scale(782, 338, (4.0 * 0.01));
        }

        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(339, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_offset_sub_scaled_inputs_indices(781, 338, (p.p98 + 1.0), 339, 1.0, (-5e-5));
            s.store_scale(782, 338, ((p.p98 + 1.0) * (4.0 * 5e-5)));
        }

        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 338, (p.p98 + 1.0), 781, (-0.5), 782, (-0.5));
            s.store_offset_add_scaled_product(781, s.ad_value(341), 1.0, s.ad_value(335), s.ad_value(338), 1.0, (-5e-5));
            s.store_scalar(782, 0.0);
        }

        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1289] && s.b[1313]) && s.b[1318]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
        }

        s.b[1321] = ((p.p39 == 0.0) || (p.p39 == 1.0));
        s.store_scalar(1321, if s.b[1321] { 1.0 } else { 0.0 });

        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && s.b[1321]) {
            s.store_mul_add_scaled_inputs_rhs(693, 336, A::scale_offset(s.ad_value(389), s.v[557], s.v[538]), 1.0, s.ad_value(390), s.v[558]);
            s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && s.b[1321]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && s.b[1321]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && (!s.b[1321])) {
            s.store_mul_add_scaled_inputs_rhs(693, 336, A::scale_offset(s.ad_value(391), s.v[557], s.v[538]), 1.0, s.ad_value(392), s.v[558]);
            s.store_offset(781, 693, (((-(0.005 * s.v[538]))) + ((-(0.01 * s.v[538])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[538])) * (0.01 * s.v[538])));
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && (!s.b[1321])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1289] && s.b[1313]) && s.b[1318]) && (!s.b[1321])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(693, 781, 0.5, 782, 0.5, (0.005 * s.v[538]));
        }

        if ((s.b[1289] && s.b[1313]) && (!s.b[1318])) {
            s.store_scalar(692, 0.0);
            s.store_scalar(693, 0.0);
        }

        if s.b[1289] {
            s.store_scaled_sqrt(139, 155, s.v[639]);
            s.store_square(694, 139);
            s.store_scaled_square(140, 394, s.v[640]);
            s.store_offset_scaled(427, 391, p.p448, p.p447);
            s.store_scalar(957, p.p193);
        }

        s.b[1324] = (s.v[957] < 0.0);
        s.store_scalar(1324, if s.b[1324] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1324]) {
            s.store_scalar(957, 0.0);
        }

        s.b[1325] = (s.v[957] > 0.005);
        s.store_scalar(1325, if s.b[1325] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1325]) {
            s.store_scalar(957, 0.005);
        }

        s.b[1326] = (s.v[449] > 0.0);
        s.store_scalar(1326, if s.b[1326] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1326]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p.p416);
            }
        }

        if (s.b[1289] && s.b[1326]) {
            s.store_div_from_scalar(794, s.v[569], 335);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p418), p.p418));
            s.store_div_from_scalar(795, s.v[570], 334);
            s.store_offset_scaled(959, 387, p.p439, (((((-s.v[764])) * (p.p439))) + (s.v[959])));
        }

        if (s.b[1289] && s.b[1326]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p.p415);
            }
        }

        if (s.b[1289] && s.b[1326]) {
            s.store_div_from_scalar(787, s.v[567], 335);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p417), p.p417));
            s.store_div_from_scalar(788, s.v[568], 334);
            s.store_offset_scaled(956, 387, p.p438, (((((-s.v[764])) * (p.p438))) + (s.v[956])));
        }

        s.b[1328] = (s.v[956] < 0.1);
        s.store_scalar(1328, if s.b[1328] { 1.0 } else { 0.0 });

        if ((s.b[1289] && s.b[1326]) && s.b[1328]) {
            s.store_scalar(956, 0.1);
        }

        if s.b[1289] {
            s.store_square(334, 676);
            s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (s.v[820])), s.v[818]);
            s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (p.p497)), s.v[819]);
            s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p499), 1.0 / (p.p498)), p.p495);
            s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (s.v[820])), s.v[818]);
            s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (p.p497)), s.v[819]);
            s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p509), 1.0 / (p.p498)), p.p495);
        }

        s.b[1329] = (p.p48 > 0.0);
        s.store_scalar(1329, if s.b[1329] { 1.0 } else { 0.0 });

        s.b[1330] = (p.p15 > s.v[632]);
        s.store_scalar(1330, if s.b[1330] { 1.0 } else { 0.0 });

        if ((s.b[1289] && s.b[1329]) && s.b[1330]) {
            s.store_scale(873, 828, p.p13);
            s.store_scale(874, 830, p.p13);
            s.store_scale(875, 829, (p.p15 - s.v[632]));
            s.store_scale(876, 831, (p.p15 - s.v[632]));
            s.store_scale(877, 836, s.v[632]);
            s.store_scale(878, 837, s.v[632]);
        }

        if ((s.b[1289] && s.b[1329]) && (!s.b[1330])) {
            s.store_scale(873, 828, p.p13);
            s.store_scale(874, 830, p.p13);
            s.store_scalar(875, 0.0);
            s.store_scalar(876, 0.0);
            s.store_scale(877, 836, p.p15);
            s.store_scale(878, 837, p.p15);
        }

    }

    pub(super) fn stamp_reactive_block_7(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        if (s.b[1289] && (!s.b[1329])) {
            s.store_scale(873, 828, p.p13);
            s.store_scale(874, 830, p.p13);
            s.store_scale(875, 829, p.p15);
            s.store_scale(876, 831, p.p15);
            s.store_scalar(877, 0.0);
            s.store_scalar(878, 0.0);
        }

        if s.b[1289] {
            s.store_add_scaled_inputs3_indices(847, 873, 1.0, 875, 1.0, 877, 1.0);
        }

        s.b[1331] = (s.v[847] > 0.0);
        s.store_scalar(1331, if s.b[1331] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1331]) {
            s.store_offset(336, 847, 1e-25);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(848, s.v[820], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[822], s.ad_value(336), 1.0, 1.0));
            s.store_exp_scaled_input_ad(849, A::offset(s.ad_value(676), (-1.0)), p.p512);
            s.store_div_from_scalar_div_from_scalar_ad(850, 1.0, s.v[820], s.ad_value(154));
            s.store_exp_mul(851, 848, 850);
        }

        if s.b[1289] {
            s.store_scale_ad(828, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (s.v[825])), s.v[823]);
            s.store_scale_ad(829, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (p.p520)), s.v[824]);
            s.store_scale_ad(836, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p522), 1.0 / (p.p521)), p.p518);
            s.store_scale_ad(830, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (s.v[825])), s.v[823]);
            s.store_scale_ad(831, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (p.p520)), s.v[824]);
            s.store_scale_ad(837, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p.p532), 1.0 / (p.p521)), p.p518);
        }

        s.b[1332] = (p.p48 > 0.0);
        s.store_scalar(1332, if s.b[1332] { 1.0 } else { 0.0 });

        s.b[1333] = (p.p16 > s.v[632]);
        s.store_scalar(1333, if s.b[1333] { 1.0 } else { 0.0 });

        if ((s.b[1289] && s.b[1332]) && s.b[1333]) {
            s.store_scale(879, 828, p.p14);
            s.store_scale(880, 830, p.p14);
            s.store_scale(881, 829, (p.p16 - s.v[632]));
            s.store_scale(882, 831, (p.p16 - s.v[632]));
            s.store_scale(883, 836, s.v[632]);
            s.store_scale(884, 837, s.v[632]);
        }

        if ((s.b[1289] && s.b[1332]) && (!s.b[1333])) {
            s.store_scale(879, 828, p.p14);
            s.store_scale(880, 830, p.p14);
            s.store_scalar(881, 0.0);
            s.store_scalar(882, 0.0);
            s.store_scale(883, 836, p.p16);
            s.store_scale(884, 837, p.p16);
        }

        if (s.b[1289] && (!s.b[1332])) {
            s.store_scale(879, 828, p.p14);
            s.store_scale(880, 830, p.p14);
            s.store_scale(881, 829, p.p16);
            s.store_scale(882, 831, p.p16);
            s.store_scalar(883, 0.0);
            s.store_scalar(884, 0.0);
        }

        if s.b[1289] {
            s.store_add_scaled_inputs3_indices(852, 879, 1.0, 881, 1.0, 883, 1.0);
        }

        s.b[1334] = (s.v[852] > 0.0);
        s.store_scalar(1334, if s.b[1334] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1334]) {
            s.store_offset(337, 852, 1e-25);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(853, s.v[825], 154, A::ln_offset_div_scaled_inputs(s.ad_value(334), s.v[827], s.ad_value(337), 1.0, 1.0));
            s.store_exp_scaled_input_ad(854, A::offset(s.ad_value(676), (-1.0)), p.p535);
            s.store_div_from_scalar_div_from_scalar_ad(855, 1.0, s.v[825], s.ad_value(154));
            s.store_exp_mul(856, 853, 855);
        }

        if s.b[1289] {
            s.store_offset_scaled(832, 391, ((p.p481) * ((p.p500 * p.p13))), (p.p500 * p.p13));
        }

        s.b[1335] = (p.p15 > s.v[632]);
        s.store_scalar(1335, if s.b[1335] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1335]) {
            s.store_offset_scaled(833, 391, ((p.p483) * ((p.p501 * (p.p15 - s.v[632])))), (p.p501 * (p.p15 - s.v[632])));
            s.store_offset_scaled(834, 391, ((p.p485) * ((p.p502 * s.v[632]))), (p.p502 * s.v[632]));
        }

        if (s.b[1289] && (!s.b[1335])) {
            s.store_scalar(833, 0.0);
            s.store_offset_scaled(834, 391, ((p.p485) * ((p.p502 * p.p15))), (p.p502 * p.p15));
        }

        s.b[1336] = (s.v[832] < 0.0);
        s.store_scalar(1336, if s.b[1336] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1336]) {
            s.store_scalar(832, 0.0);
        }

        s.b[1337] = (s.v[833] < 0.0);
        s.store_scalar(1337, if s.b[1337] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1337]) {
            s.store_scalar(833, 0.0);
        }

        s.b[1338] = (s.v[834] < 0.0);
        s.store_scalar(1338, if s.b[1338] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1338]) {
            s.store_scalar(834, 0.0);
        }

        if s.b[1289] {
            s.store_sub_from_scalar_scaled_input(841, p.p506, 391, p.p487);
            s.store_sub_from_scalar_scaled_input(842, p.p507, 391, p.p489);
            s.store_sub_from_scalar_scaled_input(843, p.p508, 391, p.p491);
        }

        s.b[1339] = ((s.v[841] < 0.01) && (p.p13 > 0.0));
        s.store_scalar(1339, if s.b[1339] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1339]) {
            s.store_scalar(841, 0.01);
        }

        s.b[1340] = ((s.v[842] < 0.01) && (p.p15 > s.v[632]));
        s.store_scalar(1340, if s.b[1340] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1340]) {
            s.store_scalar(842, 0.01);
        }

        s.b[1341] = ((s.v[843] < 0.01) && (p.p15 > 0.0));
        s.store_scalar(1341, if s.b[1341] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1341]) {
            s.store_scalar(843, 0.01);
        }

        if s.b[1289] {
            s.store_offset_scaled(835, 391, ((p.p482) * ((p.p523 * p.p14))), (p.p523 * p.p14));
        }

        s.b[1342] = (p.p16 > s.v[632]);
        s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1342]) {
            s.store_offset_scaled(838, 391, ((p.p484) * ((p.p524 * (p.p16 - s.v[632])))), (p.p524 * (p.p16 - s.v[632])));
            s.store_offset_scaled(839, 391, ((p.p486) * ((p.p525 * s.v[632]))), (p.p525 * s.v[632]));
        }

        if (s.b[1289] && (!s.b[1342])) {
            s.store_scalar(838, 0.0);
            s.store_offset_scaled(839, 391, ((p.p486) * ((p.p525 * p.p16))), (p.p525 * p.p16));
        }

        s.b[1343] = (s.v[835] < 0.0);
        s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1343]) {
            s.store_scalar(835, 0.0);
        }

        s.b[1344] = (s.v[838] < 0.0);
        s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1344]) {
            s.store_scalar(838, 0.0);
        }

        s.b[1345] = (s.v[839] < 0.0);
        s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1345]) {
            s.store_scalar(839, 0.0);
        }

        if s.b[1289] {
            s.store_sub_from_scalar_scaled_input(844, p.p529, 391, p.p488);
            s.store_sub_from_scalar_scaled_input(845, p.p530, 391, p.p490);
            s.store_sub_from_scalar_scaled_input(846, p.p531, 391, p.p492);
        }

        s.b[1346] = ((s.v[844] < 0.01) && (p.p14 > 0.0));
        s.store_scalar(1346, if s.b[1346] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1346]) {
            s.store_scalar(844, 0.01);
        }

        s.b[1347] = ((s.v[845] < 0.01) && (p.p16 > s.v[632]));
        s.store_scalar(1347, if s.b[1347] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1347]) {
            s.store_scalar(845, 0.01);
        }

        s.b[1348] = ((s.v[846] < 0.01) && (p.p16 > 0.0));
        s.store_scalar(1348, if s.b[1348] { 1.0 } else { 0.0 });

        if (s.b[1289] && s.b[1348]) {
            s.store_scalar(846, 0.01);
        }

        s.store_scaled_voltage(729, ctx, nodes, Some(6), Some(8), p.p87);

        s.store_scaled_voltage(731, ctx, nodes, Some(7), Some(8), p.p87);

        s.store_scaled_voltage(728, ctx, nodes, Some(9), Some(8), p.p87);

        s.store_scaled_voltage(733, ctx, nodes, Some(0), Some(2), p.p87);

        s.store_scaled_voltage(734, ctx, nodes, Some(7), Some(2), p.p87);

        s.store_scaled_voltage(735, ctx, nodes, Some(9), Some(2), p.p87);

        s.store_scaled_voltage(799, ctx, nodes, Some(0), Some(6), p.p87);

        s.store_scaled_voltage(804, ctx, nodes, Some(8), Some(2), p.p87);

        s.store_scaled_voltage(857, ctx, nodes, Some(11), Some(2), p.p87);

        s.store_scaled_voltage(858, ctx, nodes, Some(10), Some(0), p.p87);

        s.store_scaled_voltage(865, ctx, nodes, Some(9), Some(8), p.p87);

        s.store_scaled_voltage(866, ctx, nodes, Some(9), Some(6), p.p87);

        s.copy_ad(859, 857);

        s.copy_ad(860, 858);

        s.copy_ad(867, 865);

        s.copy_ad(868, 866);

        s.store_scaled_voltage(798, ctx, nodes, Some(4), Some(2), p.p87);

        if (s.v[81] != 0.0) {
            s.store_voltage(747, ctx, nodes, Some(12), None);
            s.store_voltage(748, ctx, nodes, Some(13), None);
        }

        if (s.v[81] == 0.0) {
            s.store_scalar(747, 0.0);
            s.store_scalar(748, 0.0);
        }

        s.store_sub(730, 731, 729);

        s.store_sub(727, 728, 729);

        s.b[1349] = (s.v[729] >= 0.0);
        s.store_scalar(1349, if s.b[1349] { 1.0 } else { 0.0 });

        if s.b[1349] {
            s.store_scalar(949, 1.0);
            s.copy_ad(790, 729);
            s.copy_ad(791, 731);
            s.copy_ad(792, 728);
            s.copy_ad(793, 733);
            s.copy_ad(796, 734);
            s.copy_ad(797, 735);
        }

        if (!s.b[1349]) {
            s.store_scalar(949, (-1.0));
            s.store_neg(790, 729);
            s.copy_ad(791, 730);
            s.copy_ad(792, 727);
            s.store_neg(793, 733);
            s.store_sub(796, 734, 733);
            s.store_sub(797, 735, 733);
        }

        s.b[1352] = ((p.p53 > 0.0) && (s.v[541] != 0.0));
        s.store_scalar(1352, if s.b[1352] { 1.0 } else { 0.0 });

        if s.b[1352] {
            s.store_voltage(732, ctx, nodes, Some(5), None);
        }

        s.b[1353] = (p.p53 == 2.0);
        s.store_scalar(1353, if s.b[1353] { 1.0 } else { 0.0 });

        if (s.b[1352] && s.b[1353]) {
            s.store_offset_sub_from_scalar_ad(781, p.p433, s.ad_value(732), (-(p.p337 * 10.0)));
            s.store_scalar(782, ((4.0 * p.p433) * (p.p337 * 10.0)));
        }

        if (s.b[1352] && s.b[1353]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1352] && s.b[1353]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(732, 781, (-0.5), 782, (-0.5), p.p433);
        }

        if s.b[1352] {
            s.store_scalar(387, (ctx_temp + p.p11));
            s.copy_ad(388, 387);
            s.store_add(387, 387, 732);
            s.store_offset(389, 388, (-s.v[764]));
            s.store_offset_square(390, 388, (-(s.v[764] * s.v[764])));
            s.store_offset(391, 387, (-s.v[764]));
        }

    }
}
