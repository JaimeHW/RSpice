#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_368(
        p: &Parameters,
        var_car: f64,
        var_guard2336: f64,
        var_guard2356: f64,
        var_guard2369: f64,
        var_guard2370: f64,
        var_ldrifte: f64,
        var_mu0: f64,
        var_mu0_dn0: f64,
        var_mu0_dn10: f64,
        var_mu0_dn13: f64,
        var_mu0_dn2: f64,
        var_mu0_dn4: f64,
        var_mu0_dn5: f64,
        var_mu0_dn6: f64,
        var_mu0_dn7: f64,
        var_mu0_dn8: f64,
        var_mu0_dn9: f64,
        var_noverd: f64,
        var_rd_qbuld: f64,
        var_rd_qbuld_dn0: f64,
        var_rd_qbuld_dn10: f64,
        var_rd_qbuld_dn13: f64,
        var_rd_qbuld_dn2: f64,
        var_rd_qbuld_dn4: f64,
        var_rd_qbuld_dn5: f64,
        var_rd_qbuld_dn6: f64,
        var_rd_qbuld_dn7: f64,
        var_rd_qbuld_dn8: f64,
        var_rd_qbuld_dn9: f64,
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
        var_t6: f64,
        var_t6_dn0: f64,
        var_t6_dn10: f64,
        var_t6_dn13: f64,
        var_t6_dn2: f64,
        var_t6_dn4: f64,
        var_t6_dn5: f64,
        var_t6_dn6: f64,
        var_t6_dn7: f64,
        var_t6_dn8: f64,
        var_t6_dn9: f64,
        var_vddpz: f64,
        var_vddpz_dn0: f64,
        var_vddpz_dn10: f64,
        var_vddpz_dn13: f64,
        var_vddpz_dn2: f64,
        var_vddpz_dn4: f64,
        var_vddpz_dn5: f64,
        var_vddpz_dn6: f64,
        var_vddpz_dn7: f64,
        var_vddpz_dn8: f64,
        var_vddpz_dn9: f64,
        var_carr_slot: &mut f64,
        var_carr1_slot: &mut f64,
        var_carr1_dn0_slot: &mut f64,
        var_carr1_dn10_slot: &mut f64,
        var_carr1_dn13_slot: &mut f64,
        var_carr1_dn2_slot: &mut f64,
        var_carr1_dn4_slot: &mut f64,
        var_carr1_dn5_slot: &mut f64,
        var_carr1_dn6_slot: &mut f64,
        var_carr1_dn7_slot: &mut f64,
        var_carr1_dn8_slot: &mut f64,
        var_carr1_dn9_slot: &mut f64,
        var_carr2_slot: &mut f64,
        var_carr2_dn0_slot: &mut f64,
        var_carr2_dn10_slot: &mut f64,
        var_carr2_dn13_slot: &mut f64,
        var_carr2_dn2_slot: &mut f64,
        var_carr2_dn4_slot: &mut f64,
        var_carr2_dn5_slot: &mut f64,
        var_carr2_dn6_slot: &mut f64,
        var_carr2_dn7_slot: &mut f64,
        var_carr2_dn8_slot: &mut f64,
        var_carr2_dn9_slot: &mut f64,
        var_carr_dn0_slot: &mut f64,
        var_carr_dn10_slot: &mut f64,
        var_carr_dn13_slot: &mut f64,
        var_carr_dn2_slot: &mut f64,
        var_carr_dn4_slot: &mut f64,
        var_carr_dn5_slot: &mut f64,
        var_carr_dn6_slot: &mut f64,
        var_carr_dn7_slot: &mut f64,
        var_carr_dn8_slot: &mut f64,
        var_carr_dn9_slot: &mut f64,
        var_guard2371_slot: &mut f64,
        var_guard2372_slot: &mut f64,
        var_mu__blk2354_slot: &mut f64,
        var_mu__blk2354_dn0_slot: &mut f64,
        var_mu__blk2354_dn10_slot: &mut f64,
        var_mu__blk2354_dn13_slot: &mut f64,
        var_mu__blk2354_dn2_slot: &mut f64,
        var_mu__blk2354_dn4_slot: &mut f64,
        var_mu__blk2354_dn5_slot: &mut f64,
        var_mu__blk2354_dn6_slot: &mut f64,
        var_mu__blk2354_dn7_slot: &mut f64,
        var_mu__blk2354_dn8_slot: &mut f64,
        var_mu__blk2354_dn9_slot: &mut f64,
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
    ) {
        let mut var_carr: f64 = *var_carr_slot;
        let mut var_carr1: f64 = *var_carr1_slot;
        let mut var_carr1_dn0: f64 = *var_carr1_dn0_slot;
        let mut var_carr1_dn10: f64 = *var_carr1_dn10_slot;
        let mut var_carr1_dn13: f64 = *var_carr1_dn13_slot;
        let mut var_carr1_dn2: f64 = *var_carr1_dn2_slot;
        let mut var_carr1_dn4: f64 = *var_carr1_dn4_slot;
        let mut var_carr1_dn5: f64 = *var_carr1_dn5_slot;
        let mut var_carr1_dn6: f64 = *var_carr1_dn6_slot;
        let mut var_carr1_dn7: f64 = *var_carr1_dn7_slot;
        let mut var_carr1_dn8: f64 = *var_carr1_dn8_slot;
        let mut var_carr1_dn9: f64 = *var_carr1_dn9_slot;
        let mut var_carr2: f64 = *var_carr2_slot;
        let mut var_carr2_dn0: f64 = *var_carr2_dn0_slot;
        let mut var_carr2_dn10: f64 = *var_carr2_dn10_slot;
        let mut var_carr2_dn13: f64 = *var_carr2_dn13_slot;
        let mut var_carr2_dn2: f64 = *var_carr2_dn2_slot;
        let mut var_carr2_dn4: f64 = *var_carr2_dn4_slot;
        let mut var_carr2_dn5: f64 = *var_carr2_dn5_slot;
        let mut var_carr2_dn6: f64 = *var_carr2_dn6_slot;
        let mut var_carr2_dn7: f64 = *var_carr2_dn7_slot;
        let mut var_carr2_dn8: f64 = *var_carr2_dn8_slot;
        let mut var_carr2_dn9: f64 = *var_carr2_dn9_slot;
        let mut var_carr_dn0: f64 = *var_carr_dn0_slot;
        let mut var_carr_dn10: f64 = *var_carr_dn10_slot;
        let mut var_carr_dn13: f64 = *var_carr_dn13_slot;
        let mut var_carr_dn2: f64 = *var_carr_dn2_slot;
        let mut var_carr_dn4: f64 = *var_carr_dn4_slot;
        let mut var_carr_dn5: f64 = *var_carr_dn5_slot;
        let mut var_carr_dn6: f64 = *var_carr_dn6_slot;
        let mut var_carr_dn7: f64 = *var_carr_dn7_slot;
        let mut var_carr_dn8: f64 = *var_carr_dn8_slot;
        let mut var_carr_dn9: f64 = *var_carr_dn9_slot;
        let mut var_guard2371: f64 = *var_guard2371_slot;
        let mut var_guard2372: f64 = *var_guard2372_slot;
        let mut var_mu__blk2354: f64 = *var_mu__blk2354_slot;
        let mut var_mu__blk2354_dn0: f64 = *var_mu__blk2354_dn0_slot;
        let mut var_mu__blk2354_dn10: f64 = *var_mu__blk2354_dn10_slot;
        let mut var_mu__blk2354_dn13: f64 = *var_mu__blk2354_dn13_slot;
        let mut var_mu__blk2354_dn2: f64 = *var_mu__blk2354_dn2_slot;
        let mut var_mu__blk2354_dn4: f64 = *var_mu__blk2354_dn4_slot;
        let mut var_mu__blk2354_dn5: f64 = *var_mu__blk2354_dn5_slot;
        let mut var_mu__blk2354_dn6: f64 = *var_mu__blk2354_dn6_slot;
        let mut var_mu__blk2354_dn7: f64 = *var_mu__blk2354_dn7_slot;
        let mut var_mu__blk2354_dn8: f64 = *var_mu__blk2354_dn8_slot;
        let mut var_mu__blk2354_dn9: f64 = *var_mu__blk2354_dn9_slot;
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

        let (assign104070_e156200, assign104070_e156200_d_n0, assign104070_e156200_d_n2, assign104070_e156200_d_n4, assign104070_e156200_d_n5, assign104070_e156200_d_n6, assign104070_e156200_d_n7, assign104070_e156200_d_n8, assign104070_e156200_d_n9, assign104070_e156200_d_n10, assign104070_e156200_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2369 == 0.0)) && (var_guard2370 == 0.0)) {
        let assign104070_e156198: f64 = (var_t4 * var_t6);
        (assign104070_e156198, ((var_t4_dn0 * var_t6) + (var_t4 * var_t6_dn0)), ((var_t4_dn2 * var_t6) + (var_t4 * var_t6_dn2)), ((var_t4_dn4 * var_t6) + (var_t4 * var_t6_dn4)), ((var_t4_dn5 * var_t6) + (var_t4 * var_t6_dn5)), ((var_t4_dn6 * var_t6) + (var_t4 * var_t6_dn6)), ((var_t4_dn7 * var_t6) + (var_t4 * var_t6_dn7)), ((var_t4_dn8 * var_t6) + (var_t4 * var_t6_dn8)), ((var_t4_dn9 * var_t6) + (var_t4 * var_t6_dn9)), ((var_t4_dn10 * var_t6) + (var_t4 * var_t6_dn10)), ((var_t4_dn13 * var_t6) + (var_t4 * var_t6_dn13)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn13,)
    }
};
        var_t5 = assign104070_e156200;
        var_t5_dn0 = assign104070_e156200_d_n0;
        var_t5_dn2 = assign104070_e156200_d_n2;
        var_t5_dn4 = assign104070_e156200_d_n4;
        var_t5_dn5 = assign104070_e156200_d_n5;
        var_t5_dn6 = assign104070_e156200_d_n6;
        var_t5_dn7 = assign104070_e156200_d_n7;
        var_t5_dn8 = assign104070_e156200_d_n8;
        var_t5_dn9 = assign104070_e156200_d_n9;
        var_t5_dn10 = assign104070_e156200_d_n10;
        var_t5_dn13 = assign104070_e156200_d_n13;

        let (assign104080_e156209, assign104080_e156209_d_n0, assign104080_e156209_d_n2, assign104080_e156209_d_n4, assign104080_e156209_d_n5, assign104080_e156209_d_n6, assign104080_e156209_d_n7, assign104080_e156209_d_n8, assign104080_e156209_d_n9, assign104080_e156209_d_n10, assign104080_e156209_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104080_e156207: f64 = (var_mu0 * var_t5);
        (assign104080_e156207, ((var_mu0_dn0 * var_t5) + (var_mu0 * var_t5_dn0)), ((var_mu0_dn2 * var_t5) + (var_mu0 * var_t5_dn2)), ((var_mu0_dn4 * var_t5) + (var_mu0 * var_t5_dn4)), ((var_mu0_dn5 * var_t5) + (var_mu0 * var_t5_dn5)), ((var_mu0_dn6 * var_t5) + (var_mu0 * var_t5_dn6)), ((var_mu0_dn7 * var_t5) + (var_mu0 * var_t5_dn7)), ((var_mu0_dn8 * var_t5) + (var_mu0 * var_t5_dn8)), ((var_mu0_dn9 * var_t5) + (var_mu0 * var_t5_dn9)), ((var_mu0_dn10 * var_t5) + (var_mu0 * var_t5_dn10)), ((var_mu0_dn13 * var_t5) + (var_mu0 * var_t5_dn13)),)
    } else {
        (var_mu__blk2354, var_mu__blk2354_dn0, var_mu__blk2354_dn2, var_mu__blk2354_dn4, var_mu__blk2354_dn5, var_mu__blk2354_dn6, var_mu__blk2354_dn7, var_mu__blk2354_dn8, var_mu__blk2354_dn9, var_mu__blk2354_dn10, var_mu__blk2354_dn13,)
    }
};
        var_mu__blk2354 = assign104080_e156209;
        var_mu__blk2354_dn0 = assign104080_e156209_d_n0;
        var_mu__blk2354_dn2 = assign104080_e156209_d_n2;
        var_mu__blk2354_dn4 = assign104080_e156209_d_n4;
        var_mu__blk2354_dn5 = assign104080_e156209_d_n5;
        var_mu__blk2354_dn6 = assign104080_e156209_d_n6;
        var_mu__blk2354_dn7 = assign104080_e156209_d_n7;
        var_mu__blk2354_dn8 = assign104080_e156209_d_n8;
        var_mu__blk2354_dn9 = assign104080_e156209_d_n9;
        var_mu__blk2354_dn10 = assign104080_e156209_d_n10;
        var_mu__blk2354_dn13 = assign104080_e156209_d_n13;

        let (assign104090_e156218, assign104090_e156218_d_n0, assign104090_e156218_d_n2, assign104090_e156218_d_n4, assign104090_e156218_d_n5, assign104090_e156218_d_n6, assign104090_e156218_d_n7, assign104090_e156218_d_n8, assign104090_e156218_d_n9, assign104090_e156218_d_n10, assign104090_e156218_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104090_e156216: f64 = (1.0 + var_t1);
        (assign104090_e156216, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn13,)
    }
};
        var_t4 = assign104090_e156218;
        var_t4_dn0 = assign104090_e156218_d_n0;
        var_t4_dn2 = assign104090_e156218_d_n2;
        var_t4_dn4 = assign104090_e156218_d_n4;
        var_t4_dn5 = assign104090_e156218_d_n5;
        var_t4_dn6 = assign104090_e156218_d_n6;
        var_t4_dn7 = assign104090_e156218_d_n7;
        var_t4_dn8 = assign104090_e156218_d_n8;
        var_t4_dn9 = assign104090_e156218_d_n9;
        var_t4_dn10 = assign104090_e156218_d_n10;
        var_t4_dn13 = assign104090_e156218_d_n13;

        let (assign104100_e156227, assign104100_e156227_d_n0, assign104100_e156227_d_n2, assign104100_e156227_d_n4, assign104100_e156227_d_n5, assign104100_e156227_d_n6, assign104100_e156227_d_n7, assign104100_e156227_d_n8, assign104100_e156227_d_n9, assign104100_e156227_d_n10, assign104100_e156227_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104100_e156225: f64 = (1.0 / var_t4);
        (assign104100_e156225, (-(var_t4_dn0 / (var_t4 * var_t4))), (-(var_t4_dn2 / (var_t4 * var_t4))), (-(var_t4_dn4 / (var_t4 * var_t4))), (-(var_t4_dn5 / (var_t4 * var_t4))), (-(var_t4_dn6 / (var_t4 * var_t4))), (-(var_t4_dn7 / (var_t4 * var_t4))), (-(var_t4_dn8 / (var_t4 * var_t4))), (-(var_t4_dn9 / (var_t4 * var_t4))), (-(var_t4_dn10 / (var_t4 * var_t4))), (-(var_t4_dn13 / (var_t4 * var_t4))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn13,)
    }
};
        var_t5 = assign104100_e156227;
        var_t5_dn0 = assign104100_e156227_d_n0;
        var_t5_dn2 = assign104100_e156227_d_n2;
        var_t5_dn4 = assign104100_e156227_d_n4;
        var_t5_dn5 = assign104100_e156227_d_n5;
        var_t5_dn6 = assign104100_e156227_d_n6;
        var_t5_dn7 = assign104100_e156227_d_n7;
        var_t5_dn8 = assign104100_e156227_d_n8;
        var_t5_dn9 = assign104100_e156227_d_n9;
        var_t5_dn10 = assign104100_e156227_d_n10;
        var_t5_dn13 = assign104100_e156227_d_n13;

        let (assign104110_e156246, assign104110_e156246_d_n0, assign104110_e156246_d_n2, assign104110_e156246_d_n4, assign104110_e156246_d_n5, assign104110_e156246_d_n6, assign104110_e156246_d_n7, assign104110_e156246_d_n8, assign104110_e156246_d_n9, assign104110_e156246_d_n10, assign104110_e156246_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104110_e156236: f64 = (1.0 - var_t5);
        let assign104110_e156237: f64 = (var_car * assign104110_e156236);
        let assign104110_e156239: f64 = (assign104110_e156237 * var_vddpz);
        let assign104110_e156242: f64 = (var_ldrifte - p.p423);
        let assign104110_e156243: f64 = (assign104110_e156239 / assign104110_e156242);
        let assign104110_e156244: f64 = (1.0 + assign104110_e156243);
        (assign104110_e156244, ((((var_car * (-var_t5_dn0)) * var_vddpz) + (assign104110_e156237 * var_vddpz_dn0)) / assign104110_e156242), ((((var_car * (-var_t5_dn2)) * var_vddpz) + (assign104110_e156237 * var_vddpz_dn2)) / assign104110_e156242), ((((var_car * (-var_t5_dn4)) * var_vddpz) + (assign104110_e156237 * var_vddpz_dn4)) / assign104110_e156242), ((((var_car * (-var_t5_dn5)) * var_vddpz) + (assign104110_e156237 * var_vddpz_dn5)) / assign104110_e156242), ((((var_car * (-var_t5_dn6)) * var_vddpz) + (assign104110_e156237 * var_vddpz_dn6)) / assign104110_e156242), ((((var_car * (-var_t5_dn7)) * var_vddpz) + (assign104110_e156237 * var_vddpz_dn7)) / assign104110_e156242), ((((var_car * (-var_t5_dn8)) * var_vddpz) + (assign104110_e156237 * var_vddpz_dn8)) / assign104110_e156242), ((((var_car * (-var_t5_dn9)) * var_vddpz) + (assign104110_e156237 * var_vddpz_dn9)) / assign104110_e156242), ((((var_car * (-var_t5_dn10)) * var_vddpz) + (assign104110_e156237 * var_vddpz_dn10)) / assign104110_e156242), ((((var_car * (-var_t5_dn13)) * var_vddpz) + (assign104110_e156237 * var_vddpz_dn13)) / assign104110_e156242),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn13,)
    }
};
        var_t4 = assign104110_e156246;
        var_t4_dn0 = assign104110_e156246_d_n0;
        var_t4_dn2 = assign104110_e156246_d_n2;
        var_t4_dn4 = assign104110_e156246_d_n4;
        var_t4_dn5 = assign104110_e156246_d_n5;
        var_t4_dn6 = assign104110_e156246_d_n6;
        var_t4_dn7 = assign104110_e156246_d_n7;
        var_t4_dn8 = assign104110_e156246_d_n8;
        var_t4_dn9 = assign104110_e156246_d_n9;
        var_t4_dn10 = assign104110_e156246_d_n10;
        var_t4_dn13 = assign104110_e156246_d_n13;

        let (assign104120_e156257, assign104120_e156257_d_n0, assign104120_e156257_d_n2, assign104120_e156257_d_n4, assign104120_e156257_d_n5, assign104120_e156257_d_n6, assign104120_e156257_d_n7, assign104120_e156257_d_n8, assign104120_e156257_d_n9, assign104120_e156257_d_n10, assign104120_e156257_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104120_e156253: f64 = var_t4;
        let assign104120_e156255: f64 = (assign104120_e156253 - 0.001);
        (assign104120_e156255, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn13,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn13,)
    }
};
        var_tmf1 = assign104120_e156257;
        var_tmf1_dn0 = assign104120_e156257_d_n0;
        var_tmf1_dn2 = assign104120_e156257_d_n2;
        var_tmf1_dn4 = assign104120_e156257_d_n4;
        var_tmf1_dn5 = assign104120_e156257_d_n5;
        var_tmf1_dn6 = assign104120_e156257_d_n6;
        var_tmf1_dn7 = assign104120_e156257_d_n7;
        var_tmf1_dn8 = assign104120_e156257_d_n8;
        var_tmf1_dn9 = assign104120_e156257_d_n9;
        var_tmf1_dn10 = assign104120_e156257_d_n10;
        var_tmf1_dn13 = assign104120_e156257_d_n13;

        let (assign104130_e156268, assign104130_e156268_d_n0, assign104130_e156268_d_n2, assign104130_e156268_d_n4, assign104130_e156268_d_n5, assign104130_e156268_d_n6, assign104130_e156268_d_n7, assign104130_e156268_d_n8, assign104130_e156268_d_n9, assign104130_e156268_d_n10, assign104130_e156268_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign104130_e156268;
        var_tmf2_dn0 = assign104130_e156268_d_n0;
        var_tmf2_dn2 = assign104130_e156268_d_n2;
        var_tmf2_dn4 = assign104130_e156268_d_n4;
        var_tmf2_dn5 = assign104130_e156268_d_n5;
        var_tmf2_dn6 = assign104130_e156268_d_n6;
        var_tmf2_dn7 = assign104130_e156268_d_n7;
        var_tmf2_dn8 = assign104130_e156268_d_n8;
        var_tmf2_dn9 = assign104130_e156268_d_n9;
        var_tmf2_dn10 = assign104130_e156268_d_n10;
        var_tmf2_dn13 = assign104130_e156268_d_n13;

        let (assign104140_e156281, assign104140_e156281_d_n0, assign104140_e156281_d_n2, assign104140_e156281_d_n4, assign104140_e156281_d_n5, assign104140_e156281_d_n6, assign104140_e156281_d_n7, assign104140_e156281_d_n8, assign104140_e156281_d_n9, assign104140_e156281_d_n10, assign104140_e156281_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let (assign104140_e156279, assign104140_e156279_d_n0, assign104140_e156279_d_n2, assign104140_e156279_d_n4, assign104140_e156279_d_n5, assign104140_e156279_d_n6, assign104140_e156279_d_n7, assign104140_e156279_d_n8, assign104140_e156279_d_n9, assign104140_e156279_d_n10, assign104140_e156279_d_n13,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
            } else {
                let assign104140_e156278: f64 = (-var_tmf2);
                (assign104140_e156278, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn13),)
            }
        };
        (assign104140_e156279, assign104140_e156279_d_n0, assign104140_e156279_d_n2, assign104140_e156279_d_n4, assign104140_e156279_d_n5, assign104140_e156279_d_n6, assign104140_e156279_d_n7, assign104140_e156279_d_n8, assign104140_e156279_d_n9, assign104140_e156279_d_n10, assign104140_e156279_d_n13,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign104140_e156281;
        var_tmf2_dn0 = assign104140_e156281_d_n0;
        var_tmf2_dn2 = assign104140_e156281_d_n2;
        var_tmf2_dn4 = assign104140_e156281_d_n4;
        var_tmf2_dn5 = assign104140_e156281_d_n5;
        var_tmf2_dn6 = assign104140_e156281_d_n6;
        var_tmf2_dn7 = assign104140_e156281_d_n7;
        var_tmf2_dn8 = assign104140_e156281_d_n8;
        var_tmf2_dn9 = assign104140_e156281_d_n9;
        var_tmf2_dn10 = assign104140_e156281_d_n10;
        var_tmf2_dn13 = assign104140_e156281_d_n13;

        let (assign104150_e156293, assign104150_e156293_d_n0, assign104150_e156293_d_n2, assign104150_e156293_d_n4, assign104150_e156293_d_n5, assign104150_e156293_d_n6, assign104150_e156293_d_n7, assign104150_e156293_d_n8, assign104150_e156293_d_n9, assign104150_e156293_d_n10, assign104150_e156293_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104150_e156288: f64 = (var_tmf1 * var_tmf1);
        let assign104150_e156290: f64 = (assign104150_e156288 + var_tmf2);
        let assign104150_e156291: f64 = (assign104150_e156290).sqrt();
        (assign104150_e156291, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign104150_e156291)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign104150_e156291)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign104150_e156291)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign104150_e156291)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign104150_e156291)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign104150_e156291)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign104150_e156291)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign104150_e156291)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign104150_e156291)), ((((var_tmf1_dn13 * var_tmf1) + (var_tmf1 * var_tmf1_dn13)) + var_tmf2_dn13) / (2.0 * assign104150_e156291)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign104150_e156293;
        var_tmf2_dn0 = assign104150_e156293_d_n0;
        var_tmf2_dn2 = assign104150_e156293_d_n2;
        var_tmf2_dn4 = assign104150_e156293_d_n4;
        var_tmf2_dn5 = assign104150_e156293_d_n5;
        var_tmf2_dn6 = assign104150_e156293_d_n6;
        var_tmf2_dn7 = assign104150_e156293_d_n7;
        var_tmf2_dn8 = assign104150_e156293_d_n8;
        var_tmf2_dn9 = assign104150_e156293_d_n9;
        var_tmf2_dn10 = assign104150_e156293_d_n10;
        var_tmf2_dn13 = assign104150_e156293_d_n13;

        let (assign104160_e156306, assign104160_e156306_d_n0, assign104160_e156306_d_n2, assign104160_e156306_d_n4, assign104160_e156306_d_n5, assign104160_e156306_d_n6, assign104160_e156306_d_n7, assign104160_e156306_d_n8, assign104160_e156306_d_n9, assign104160_e156306_d_n10, assign104160_e156306_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104160_e156302: f64 = (var_tmf1 / var_tmf2);
        let assign104160_e156303: f64 = (1.0 + assign104160_e156302);
        let assign104160_e156304: f64 = (0.5 * assign104160_e156303);
        (assign104160_e156304, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn13 * var_tmf2) - (var_tmf1 * var_tmf2_dn13)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign104160_e156306;
        var_t0_dn0 = assign104160_e156306_d_n0;
        var_t0_dn2 = assign104160_e156306_d_n2;
        var_t0_dn4 = assign104160_e156306_d_n4;
        var_t0_dn5 = assign104160_e156306_d_n5;
        var_t0_dn6 = assign104160_e156306_d_n6;
        var_t0_dn7 = assign104160_e156306_d_n7;
        var_t0_dn8 = assign104160_e156306_d_n8;
        var_t0_dn9 = assign104160_e156306_d_n9;
        var_t0_dn10 = assign104160_e156306_d_n10;
        var_t0_dn13 = assign104160_e156306_d_n13;

        let (assign104170_e156319, assign104170_e156319_d_n0, assign104170_e156319_d_n2, assign104170_e156319_d_n4, assign104170_e156319_d_n5, assign104170_e156319_d_n6, assign104170_e156319_d_n7, assign104170_e156319_d_n8, assign104170_e156319_d_n9, assign104170_e156319_d_n10, assign104170_e156319_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104170_e156315: f64 = (var_tmf1 + var_tmf2);
        let assign104170_e156316: f64 = (0.5 * assign104170_e156315);
        let assign104170_e156317: f64 = assign104170_e156316;
        (assign104170_e156317, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_dn6 + var_tmf2_dn6)), (0.5 * (var_tmf1_dn7 + var_tmf2_dn7)), (0.5 * (var_tmf1_dn8 + var_tmf2_dn8)), (0.5 * (var_tmf1_dn9 + var_tmf2_dn9)), (0.5 * (var_tmf1_dn10 + var_tmf2_dn10)), (0.5 * (var_tmf1_dn13 + var_tmf2_dn13)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn13,)
    }
};
        var_t5 = assign104170_e156319;
        var_t5_dn0 = assign104170_e156319_d_n0;
        var_t5_dn2 = assign104170_e156319_d_n2;
        var_t5_dn4 = assign104170_e156319_d_n4;
        var_t5_dn5 = assign104170_e156319_d_n5;
        var_t5_dn6 = assign104170_e156319_d_n6;
        var_t5_dn7 = assign104170_e156319_d_n7;
        var_t5_dn8 = assign104170_e156319_d_n8;
        var_t5_dn9 = assign104170_e156319_d_n9;
        var_t5_dn10 = assign104170_e156319_d_n10;
        var_t5_dn13 = assign104170_e156319_d_n13;

        let (assign104180_e156328, assign104180_e156328_d_n0, assign104180_e156328_d_n2, assign104180_e156328_d_n4, assign104180_e156328_d_n5, assign104180_e156328_d_n6, assign104180_e156328_d_n7, assign104180_e156328_d_n8, assign104180_e156328_d_n9, assign104180_e156328_d_n10, assign104180_e156328_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104180_e156326: f64 = (var_noverd * var_t5);
        (assign104180_e156326, (var_noverd * var_t5_dn0), (var_noverd * var_t5_dn2), (var_noverd * var_t5_dn4), (var_noverd * var_t5_dn5), (var_noverd * var_t5_dn6), (var_noverd * var_t5_dn7), (var_noverd * var_t5_dn8), (var_noverd * var_t5_dn9), (var_noverd * var_t5_dn10), (var_noverd * var_t5_dn13),)
    } else {
        (var_carr1, var_carr1_dn0, var_carr1_dn2, var_carr1_dn4, var_carr1_dn5, var_carr1_dn6, var_carr1_dn7, var_carr1_dn8, var_carr1_dn9, var_carr1_dn10, var_carr1_dn13,)
    }
};
        var_carr1 = assign104180_e156328;
        var_carr1_dn0 = assign104180_e156328_d_n0;
        var_carr1_dn2 = assign104180_e156328_d_n2;
        var_carr1_dn4 = assign104180_e156328_d_n4;
        var_carr1_dn5 = assign104180_e156328_d_n5;
        var_carr1_dn6 = assign104180_e156328_d_n6;
        var_carr1_dn7 = assign104180_e156328_d_n7;
        var_carr1_dn8 = assign104180_e156328_d_n8;
        var_carr1_dn9 = assign104180_e156328_d_n9;
        var_carr1_dn10 = assign104180_e156328_d_n10;
        var_carr1_dn13 = assign104180_e156328_d_n13;

        let (assign104190_e156339, assign104190_e156339_d_n0, assign104190_e156339_d_n2, assign104190_e156339_d_n4, assign104190_e156339_d_n5, assign104190_e156339_d_n6, assign104190_e156339_d_n7, assign104190_e156339_d_n8, assign104190_e156339_d_n9, assign104190_e156339_d_n10, assign104190_e156339_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104190_e156335: f64 = (var_rd_qbuld / 1.6021918e-19);
        let assign104190_e156337: f64 = (assign104190_e156335 * p.p430);
        (assign104190_e156337, ((var_rd_qbuld_dn0 / 1.6021918e-19) * p.p430), ((var_rd_qbuld_dn2 / 1.6021918e-19) * p.p430), ((var_rd_qbuld_dn4 / 1.6021918e-19) * p.p430), ((var_rd_qbuld_dn5 / 1.6021918e-19) * p.p430), ((var_rd_qbuld_dn6 / 1.6021918e-19) * p.p430), ((var_rd_qbuld_dn7 / 1.6021918e-19) * p.p430), ((var_rd_qbuld_dn8 / 1.6021918e-19) * p.p430), ((var_rd_qbuld_dn9 / 1.6021918e-19) * p.p430), ((var_rd_qbuld_dn10 / 1.6021918e-19) * p.p430), ((var_rd_qbuld_dn13 / 1.6021918e-19) * p.p430),)
    } else {
        (var_carr2, var_carr2_dn0, var_carr2_dn2, var_carr2_dn4, var_carr2_dn5, var_carr2_dn6, var_carr2_dn7, var_carr2_dn8, var_carr2_dn9, var_carr2_dn10, var_carr2_dn13,)
    }
};
        var_carr2 = assign104190_e156339;
        var_carr2_dn0 = assign104190_e156339_d_n0;
        var_carr2_dn2 = assign104190_e156339_d_n2;
        var_carr2_dn4 = assign104190_e156339_d_n4;
        var_carr2_dn5 = assign104190_e156339_d_n5;
        var_carr2_dn6 = assign104190_e156339_d_n6;
        var_carr2_dn7 = assign104190_e156339_d_n7;
        var_carr2_dn8 = assign104190_e156339_d_n8;
        var_carr2_dn9 = assign104190_e156339_d_n9;
        var_carr2_dn10 = assign104190_e156339_d_n10;
        var_carr2_dn13 = assign104190_e156339_d_n13;

        let (assign104200_e156352, assign104200_e156352_d_n0, assign104200_e156352_d_n2, assign104200_e156352_d_n4, assign104200_e156352_d_n5, assign104200_e156352_d_n6, assign104200_e156352_d_n7, assign104200_e156352_d_n8, assign104200_e156352_d_n9, assign104200_e156352_d_n10, assign104200_e156352_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104200_e156346: f64 = (var_carr1 - var_carr2);
        let assign104200_e156349: f64 = (var_carr1 * 0.001);
        let assign104200_e156350: f64 = (assign104200_e156346 - assign104200_e156349);
        (assign104200_e156350, ((var_carr1_dn0 - var_carr2_dn0) - (var_carr1_dn0 * 0.001)), ((var_carr1_dn2 - var_carr2_dn2) - (var_carr1_dn2 * 0.001)), ((var_carr1_dn4 - var_carr2_dn4) - (var_carr1_dn4 * 0.001)), ((var_carr1_dn5 - var_carr2_dn5) - (var_carr1_dn5 * 0.001)), ((var_carr1_dn6 - var_carr2_dn6) - (var_carr1_dn6 * 0.001)), ((var_carr1_dn7 - var_carr2_dn7) - (var_carr1_dn7 * 0.001)), ((var_carr1_dn8 - var_carr2_dn8) - (var_carr1_dn8 * 0.001)), ((var_carr1_dn9 - var_carr2_dn9) - (var_carr1_dn9 * 0.001)), ((var_carr1_dn10 - var_carr2_dn10) - (var_carr1_dn10 * 0.001)), ((var_carr1_dn13 - var_carr2_dn13) - (var_carr1_dn13 * 0.001)),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn13,)
    }
};
        var_tmf1 = assign104200_e156352;
        var_tmf1_dn0 = assign104200_e156352_d_n0;
        var_tmf1_dn2 = assign104200_e156352_d_n2;
        var_tmf1_dn4 = assign104200_e156352_d_n4;
        var_tmf1_dn5 = assign104200_e156352_d_n5;
        var_tmf1_dn6 = assign104200_e156352_d_n6;
        var_tmf1_dn7 = assign104200_e156352_d_n7;
        var_tmf1_dn8 = assign104200_e156352_d_n8;
        var_tmf1_dn9 = assign104200_e156352_d_n9;
        var_tmf1_dn10 = assign104200_e156352_d_n10;
        var_tmf1_dn13 = assign104200_e156352_d_n13;

        let (assign104210_e156365, assign104210_e156365_d_n0, assign104210_e156365_d_n2, assign104210_e156365_d_n4, assign104210_e156365_d_n5, assign104210_e156365_d_n6, assign104210_e156365_d_n7, assign104210_e156365_d_n8, assign104210_e156365_d_n9, assign104210_e156365_d_n10, assign104210_e156365_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104210_e156359: f64 = (4.0 * var_carr1);
        let assign104210_e156362: f64 = (var_carr1 * 0.001);
        let assign104210_e156363: f64 = (assign104210_e156359 * assign104210_e156362);
        (assign104210_e156363, (((4.0 * var_carr1_dn0) * assign104210_e156362) + (assign104210_e156359 * (var_carr1_dn0 * 0.001))), (((4.0 * var_carr1_dn2) * assign104210_e156362) + (assign104210_e156359 * (var_carr1_dn2 * 0.001))), (((4.0 * var_carr1_dn4) * assign104210_e156362) + (assign104210_e156359 * (var_carr1_dn4 * 0.001))), (((4.0 * var_carr1_dn5) * assign104210_e156362) + (assign104210_e156359 * (var_carr1_dn5 * 0.001))), (((4.0 * var_carr1_dn6) * assign104210_e156362) + (assign104210_e156359 * (var_carr1_dn6 * 0.001))), (((4.0 * var_carr1_dn7) * assign104210_e156362) + (assign104210_e156359 * (var_carr1_dn7 * 0.001))), (((4.0 * var_carr1_dn8) * assign104210_e156362) + (assign104210_e156359 * (var_carr1_dn8 * 0.001))), (((4.0 * var_carr1_dn9) * assign104210_e156362) + (assign104210_e156359 * (var_carr1_dn9 * 0.001))), (((4.0 * var_carr1_dn10) * assign104210_e156362) + (assign104210_e156359 * (var_carr1_dn10 * 0.001))), (((4.0 * var_carr1_dn13) * assign104210_e156362) + (assign104210_e156359 * (var_carr1_dn13 * 0.001))),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign104210_e156365;
        var_tmf2_dn0 = assign104210_e156365_d_n0;
        var_tmf2_dn2 = assign104210_e156365_d_n2;
        var_tmf2_dn4 = assign104210_e156365_d_n4;
        var_tmf2_dn5 = assign104210_e156365_d_n5;
        var_tmf2_dn6 = assign104210_e156365_d_n6;
        var_tmf2_dn7 = assign104210_e156365_d_n7;
        var_tmf2_dn8 = assign104210_e156365_d_n8;
        var_tmf2_dn9 = assign104210_e156365_d_n9;
        var_tmf2_dn10 = assign104210_e156365_d_n10;
        var_tmf2_dn13 = assign104210_e156365_d_n13;

        let (assign104220_e156378, assign104220_e156378_d_n0, assign104220_e156378_d_n2, assign104220_e156378_d_n4, assign104220_e156378_d_n5, assign104220_e156378_d_n6, assign104220_e156378_d_n7, assign104220_e156378_d_n8, assign104220_e156378_d_n9, assign104220_e156378_d_n10, assign104220_e156378_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let (assign104220_e156376, assign104220_e156376_d_n0, assign104220_e156376_d_n2, assign104220_e156376_d_n4, assign104220_e156376_d_n5, assign104220_e156376_d_n6, assign104220_e156376_d_n7, assign104220_e156376_d_n8, assign104220_e156376_d_n9, assign104220_e156376_d_n10, assign104220_e156376_d_n13,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
            } else {
                let assign104220_e156375: f64 = (-var_tmf2);
                (assign104220_e156375, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn13),)
            }
        };
        (assign104220_e156376, assign104220_e156376_d_n0, assign104220_e156376_d_n2, assign104220_e156376_d_n4, assign104220_e156376_d_n5, assign104220_e156376_d_n6, assign104220_e156376_d_n7, assign104220_e156376_d_n8, assign104220_e156376_d_n9, assign104220_e156376_d_n10, assign104220_e156376_d_n13,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign104220_e156378;
        var_tmf2_dn0 = assign104220_e156378_d_n0;
        var_tmf2_dn2 = assign104220_e156378_d_n2;
        var_tmf2_dn4 = assign104220_e156378_d_n4;
        var_tmf2_dn5 = assign104220_e156378_d_n5;
        var_tmf2_dn6 = assign104220_e156378_d_n6;
        var_tmf2_dn7 = assign104220_e156378_d_n7;
        var_tmf2_dn8 = assign104220_e156378_d_n8;
        var_tmf2_dn9 = assign104220_e156378_d_n9;
        var_tmf2_dn10 = assign104220_e156378_d_n10;
        var_tmf2_dn13 = assign104220_e156378_d_n13;

        let (assign104230_e156390, assign104230_e156390_d_n0, assign104230_e156390_d_n2, assign104230_e156390_d_n4, assign104230_e156390_d_n5, assign104230_e156390_d_n6, assign104230_e156390_d_n7, assign104230_e156390_d_n8, assign104230_e156390_d_n9, assign104230_e156390_d_n10, assign104230_e156390_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104230_e156385: f64 = (var_tmf1 * var_tmf1);
        let assign104230_e156387: f64 = (assign104230_e156385 + var_tmf2);
        let assign104230_e156388: f64 = (assign104230_e156387).sqrt();
        (assign104230_e156388, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign104230_e156388)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign104230_e156388)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign104230_e156388)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign104230_e156388)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign104230_e156388)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign104230_e156388)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign104230_e156388)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign104230_e156388)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign104230_e156388)), ((((var_tmf1_dn13 * var_tmf1) + (var_tmf1 * var_tmf1_dn13)) + var_tmf2_dn13) / (2.0 * assign104230_e156388)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign104230_e156390;
        var_tmf2_dn0 = assign104230_e156390_d_n0;
        var_tmf2_dn2 = assign104230_e156390_d_n2;
        var_tmf2_dn4 = assign104230_e156390_d_n4;
        var_tmf2_dn5 = assign104230_e156390_d_n5;
        var_tmf2_dn6 = assign104230_e156390_d_n6;
        var_tmf2_dn7 = assign104230_e156390_d_n7;
        var_tmf2_dn8 = assign104230_e156390_d_n8;
        var_tmf2_dn9 = assign104230_e156390_d_n9;
        var_tmf2_dn10 = assign104230_e156390_d_n10;
        var_tmf2_dn13 = assign104230_e156390_d_n13;

        let (assign104240_e156403, assign104240_e156403_d_n0, assign104240_e156403_d_n2, assign104240_e156403_d_n4, assign104240_e156403_d_n5, assign104240_e156403_d_n6, assign104240_e156403_d_n7, assign104240_e156403_d_n8, assign104240_e156403_d_n9, assign104240_e156403_d_n10, assign104240_e156403_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104240_e156399: f64 = (var_tmf1 / var_tmf2);
        let assign104240_e156400: f64 = (1.0 + assign104240_e156399);
        let assign104240_e156401: f64 = (0.5 * assign104240_e156400);
        (assign104240_e156401, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn13 * var_tmf2) - (var_tmf1 * var_tmf2_dn13)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign104240_e156403;
        var_t0_dn0 = assign104240_e156403_d_n0;
        var_t0_dn2 = assign104240_e156403_d_n2;
        var_t0_dn4 = assign104240_e156403_d_n4;
        var_t0_dn5 = assign104240_e156403_d_n5;
        var_t0_dn6 = assign104240_e156403_d_n6;
        var_t0_dn7 = assign104240_e156403_d_n7;
        var_t0_dn8 = assign104240_e156403_d_n8;
        var_t0_dn9 = assign104240_e156403_d_n9;
        var_t0_dn10 = assign104240_e156403_d_n10;
        var_t0_dn13 = assign104240_e156403_d_n13;

        let (assign104250_e156416, assign104250_e156416_d_n0, assign104250_e156416_d_n2, assign104250_e156416_d_n4, assign104250_e156416_d_n5, assign104250_e156416_d_n6, assign104250_e156416_d_n7, assign104250_e156416_d_n8, assign104250_e156416_d_n9, assign104250_e156416_d_n10, assign104250_e156416_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104250_e156412: f64 = (var_tmf1 + var_tmf2);
        let assign104250_e156413: f64 = (0.5 * assign104250_e156412);
        let assign104250_e156414: f64 = (var_carr1 - assign104250_e156413);
        (assign104250_e156414, (var_carr1_dn0 - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_carr1_dn2 - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_carr1_dn4 - (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_carr1_dn5 - (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_carr1_dn6 - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (var_carr1_dn7 - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (var_carr1_dn8 - (0.5 * (var_tmf1_dn8 + var_tmf2_dn8))), (var_carr1_dn9 - (0.5 * (var_tmf1_dn9 + var_tmf2_dn9))), (var_carr1_dn10 - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (var_carr1_dn13 - (0.5 * (var_tmf1_dn13 + var_tmf2_dn13))),)
    } else {
        (var_carr2, var_carr2_dn0, var_carr2_dn2, var_carr2_dn4, var_carr2_dn5, var_carr2_dn6, var_carr2_dn7, var_carr2_dn8, var_carr2_dn9, var_carr2_dn10, var_carr2_dn13,)
    }
};
        var_carr2 = assign104250_e156416;
        var_carr2_dn0 = assign104250_e156416_d_n0;
        var_carr2_dn2 = assign104250_e156416_d_n2;
        var_carr2_dn4 = assign104250_e156416_d_n4;
        var_carr2_dn5 = assign104250_e156416_d_n5;
        var_carr2_dn6 = assign104250_e156416_d_n6;
        var_carr2_dn7 = assign104250_e156416_d_n7;
        var_carr2_dn8 = assign104250_e156416_d_n8;
        var_carr2_dn9 = assign104250_e156416_d_n9;
        var_carr2_dn10 = assign104250_e156416_d_n10;
        var_carr2_dn13 = assign104250_e156416_d_n13;

        let (assign104260_e156425, assign104260_e156425_d_n0, assign104260_e156425_d_n2, assign104260_e156425_d_n4, assign104260_e156425_d_n5, assign104260_e156425_d_n6, assign104260_e156425_d_n7, assign104260_e156425_d_n8, assign104260_e156425_d_n9, assign104260_e156425_d_n10, assign104260_e156425_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104260_e156423: f64 = (var_carr1 - var_carr2);
        (assign104260_e156423, (var_carr1_dn0 - var_carr2_dn0), (var_carr1_dn2 - var_carr2_dn2), (var_carr1_dn4 - var_carr2_dn4), (var_carr1_dn5 - var_carr2_dn5), (var_carr1_dn6 - var_carr2_dn6), (var_carr1_dn7 - var_carr2_dn7), (var_carr1_dn8 - var_carr2_dn8), (var_carr1_dn9 - var_carr2_dn9), (var_carr1_dn10 - var_carr2_dn10), (var_carr1_dn13 - var_carr2_dn13),)
    } else {
        (var_carr, var_carr_dn0, var_carr_dn2, var_carr_dn4, var_carr_dn5, var_carr_dn6, var_carr_dn7, var_carr_dn8, var_carr_dn9, var_carr_dn10, var_carr_dn13,)
    }
};
        var_carr = assign104260_e156425;
        var_carr_dn0 = assign104260_e156425_d_n0;
        var_carr_dn2 = assign104260_e156425_d_n2;
        var_carr_dn4 = assign104260_e156425_d_n4;
        var_carr_dn5 = assign104260_e156425_d_n5;
        var_carr_dn6 = assign104260_e156425_d_n6;
        var_carr_dn7 = assign104260_e156425_d_n7;
        var_carr_dn8 = assign104260_e156425_d_n8;
        var_carr_dn9 = assign104260_e156425_d_n9;
        var_carr_dn10 = assign104260_e156425_d_n10;
        var_carr_dn13 = assign104260_e156425_d_n13;

        let assign104270_e156432: f64 = if ((p.p441 > 0.0) && (p.p440 > 1.0)) { 1.0 } else { 0.0 };
        var_guard2371 = assign104270_e156432;

        let assign104280_e156436: f64 = (var_noverd * p.p440);
        let assign104280_e156439: f64 = (var_noverd * p.p441);
        let assign104280_e156440: f64 = (assign104280_e156436 - assign104280_e156439);
        let assign104280_e156444: f64 = (var_noverd * p.p441);
        let assign104280_e156447: f64 = if ((var_carr > assign104280_e156440) && (assign104280_e156444 >= 0.0)) { 1.0 } else { 0.0 };
        var_guard2372 = assign104280_e156447;

        let (assign104290_e156466, assign104290_e156466_d_n0, assign104290_e156466_d_n2, assign104290_e156466_d_n4, assign104290_e156466_d_n5, assign104290_e156466_d_n6, assign104290_e156466_d_n7, assign104290_e156466_d_n8, assign104290_e156466_d_n9, assign104290_e156466_d_n10, assign104290_e156466_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        let assign104290_e156459: f64 = (var_noverd * p.p440);
        let assign104290_e156460: f64 = (var_carr - assign104290_e156459);
        let assign104290_e156463: f64 = (var_noverd * p.p441);
        let assign104290_e156464: f64 = (assign104290_e156460 + assign104290_e156463);
        (assign104290_e156464, var_carr_dn0, var_carr_dn2, var_carr_dn4, var_carr_dn5, var_carr_dn6, var_carr_dn7, var_carr_dn8, var_carr_dn9, var_carr_dn10, var_carr_dn13,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn13,)
    }
};
        var_tmf1 = assign104290_e156466;
        var_tmf1_dn0 = assign104290_e156466_d_n0;
        var_tmf1_dn2 = assign104290_e156466_d_n2;
        var_tmf1_dn4 = assign104290_e156466_d_n4;
        var_tmf1_dn5 = assign104290_e156466_d_n5;
        var_tmf1_dn6 = assign104290_e156466_d_n6;
        var_tmf1_dn7 = assign104290_e156466_d_n7;
        var_tmf1_dn8 = assign104290_e156466_d_n8;
        var_tmf1_dn9 = assign104290_e156466_d_n9;
        var_tmf1_dn10 = assign104290_e156466_d_n10;
        var_tmf1_dn13 = assign104290_e156466_d_n13;

        let (assign104300_e156479, assign104300_e156479_d_n0, assign104300_e156479_d_n2, assign104300_e156479_d_n4, assign104300_e156479_d_n5, assign104300_e156479_d_n6, assign104300_e156479_d_n7, assign104300_e156479_d_n8, assign104300_e156479_d_n9, assign104300_e156479_d_n10, assign104300_e156479_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        let assign104300_e156477: f64 = (var_tmf1 * var_tmf1);
        (assign104300_e156477, ((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)), ((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)), ((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)), ((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)), ((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)), ((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)), ((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)), ((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)), ((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)), ((var_tmf1_dn13 * var_tmf1) + (var_tmf1 * var_tmf1_dn13)),)
    } else {
        (var_x2, var_x2_dn0, var_x2_dn2, var_x2_dn4, var_x2_dn5, var_x2_dn6, var_x2_dn7, var_x2_dn8, var_x2_dn9, var_x2_dn10, var_x2_dn13,)
    }
};
        var_x2 = assign104300_e156479;
        var_x2_dn0 = assign104300_e156479_d_n0;
        var_x2_dn2 = assign104300_e156479_d_n2;
        var_x2_dn4 = assign104300_e156479_d_n4;
        var_x2_dn5 = assign104300_e156479_d_n5;
        var_x2_dn6 = assign104300_e156479_d_n6;
        var_x2_dn7 = assign104300_e156479_d_n7;
        var_x2_dn8 = assign104300_e156479_d_n8;
        var_x2_dn9 = assign104300_e156479_d_n9;
        var_x2_dn10 = assign104300_e156479_d_n10;
        var_x2_dn13 = assign104300_e156479_d_n13;

        let (assign104310_e156496, assign104310_e156496_d_n0, assign104310_e156496_d_n2, assign104310_e156496_d_n4, assign104310_e156496_d_n5, assign104310_e156496_d_n6, assign104310_e156496_d_n7, assign104310_e156496_d_n8, assign104310_e156496_d_n9, assign104310_e156496_d_n10, assign104310_e156496_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        let assign104310_e156490: f64 = (var_noverd * p.p441);
        let assign104310_e156493: f64 = (var_noverd * p.p441);
        let assign104310_e156494: f64 = (assign104310_e156490 * assign104310_e156493);
        (assign104310_e156494, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmax2, var_xmax2_dn0, var_xmax2_dn2, var_xmax2_dn4, var_xmax2_dn5, var_xmax2_dn6, var_xmax2_dn7, var_xmax2_dn8, var_xmax2_dn9, var_xmax2_dn10, var_xmax2_dn13,)
    }
};
        var_xmax2 = assign104310_e156496;
        var_xmax2_dn0 = assign104310_e156496_d_n0;
        var_xmax2_dn2 = assign104310_e156496_d_n2;
        var_xmax2_dn4 = assign104310_e156496_d_n4;
        var_xmax2_dn5 = assign104310_e156496_d_n5;
        var_xmax2_dn6 = assign104310_e156496_d_n6;
        var_xmax2_dn7 = assign104310_e156496_d_n7;
        var_xmax2_dn8 = assign104310_e156496_d_n8;
        var_xmax2_dn9 = assign104310_e156496_d_n9;
        var_xmax2_dn10 = assign104310_e156496_d_n10;
        var_xmax2_dn13 = assign104310_e156496_d_n13;

        *var_carr_slot = var_carr;
        *var_carr1_slot = var_carr1;
        *var_carr1_dn0_slot = var_carr1_dn0;
        *var_carr1_dn10_slot = var_carr1_dn10;
        *var_carr1_dn13_slot = var_carr1_dn13;
        *var_carr1_dn2_slot = var_carr1_dn2;
        *var_carr1_dn4_slot = var_carr1_dn4;
        *var_carr1_dn5_slot = var_carr1_dn5;
        *var_carr1_dn6_slot = var_carr1_dn6;
        *var_carr1_dn7_slot = var_carr1_dn7;
        *var_carr1_dn8_slot = var_carr1_dn8;
        *var_carr1_dn9_slot = var_carr1_dn9;
        *var_carr2_slot = var_carr2;
        *var_carr2_dn0_slot = var_carr2_dn0;
        *var_carr2_dn10_slot = var_carr2_dn10;
        *var_carr2_dn13_slot = var_carr2_dn13;
        *var_carr2_dn2_slot = var_carr2_dn2;
        *var_carr2_dn4_slot = var_carr2_dn4;
        *var_carr2_dn5_slot = var_carr2_dn5;
        *var_carr2_dn6_slot = var_carr2_dn6;
        *var_carr2_dn7_slot = var_carr2_dn7;
        *var_carr2_dn8_slot = var_carr2_dn8;
        *var_carr2_dn9_slot = var_carr2_dn9;
        *var_carr_dn0_slot = var_carr_dn0;
        *var_carr_dn10_slot = var_carr_dn10;
        *var_carr_dn13_slot = var_carr_dn13;
        *var_carr_dn2_slot = var_carr_dn2;
        *var_carr_dn4_slot = var_carr_dn4;
        *var_carr_dn5_slot = var_carr_dn5;
        *var_carr_dn6_slot = var_carr_dn6;
        *var_carr_dn7_slot = var_carr_dn7;
        *var_carr_dn8_slot = var_carr_dn8;
        *var_carr_dn9_slot = var_carr_dn9;
        *var_guard2371_slot = var_guard2371;
        *var_guard2372_slot = var_guard2372;
        *var_mu__blk2354_slot = var_mu__blk2354;
        *var_mu__blk2354_dn0_slot = var_mu__blk2354_dn0;
        *var_mu__blk2354_dn10_slot = var_mu__blk2354_dn10;
        *var_mu__blk2354_dn13_slot = var_mu__blk2354_dn13;
        *var_mu__blk2354_dn2_slot = var_mu__blk2354_dn2;
        *var_mu__blk2354_dn4_slot = var_mu__blk2354_dn4;
        *var_mu__blk2354_dn5_slot = var_mu__blk2354_dn5;
        *var_mu__blk2354_dn6_slot = var_mu__blk2354_dn6;
        *var_mu__blk2354_dn7_slot = var_mu__blk2354_dn7;
        *var_mu__blk2354_dn8_slot = var_mu__blk2354_dn8;
        *var_mu__blk2354_dn9_slot = var_mu__blk2354_dn9;
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
    }

    pub(super) fn stamp_transient_block_369(
        p: &Parameters,
        var_guard2336: f64,
        var_guard2356: f64,
        var_guard2371: f64,
        var_guard2372: f64,
        var_noverd: f64,
        var_rd_ps0ld: f64,
        var_rd_ps0ld_dn0: f64,
        var_rd_ps0ld_dn10: f64,
        var_rd_ps0ld_dn13: f64,
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
        var_tmf1_dn13: f64,
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
        var_carr_slot: &mut f64,
        var_carr_dn0_slot: &mut f64,
        var_carr_dn10_slot: &mut f64,
        var_carr_dn13_slot: &mut f64,
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
        var_dnm_dn13_slot: &mut f64,
        var_dnm_dn2_slot: &mut f64,
        var_dnm_dn4_slot: &mut f64,
        var_dnm_dn5_slot: &mut f64,
        var_dnm_dn6_slot: &mut f64,
        var_dnm_dn7_slot: &mut f64,
        var_dnm_dn8_slot: &mut f64,
        var_dnm_dn9_slot: &mut f64,
        var_guard2373_slot: &mut f64,
        var_guard2374_slot: &mut f64,
        var_guard2375_slot: &mut f64,
        var_guard2376_slot: &mut f64,
        var_guard2377_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_mm_slot: &mut f64,
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
        let mut var_carr: f64 = *var_carr_slot;
        let mut var_carr_dn0: f64 = *var_carr_dn0_slot;
        let mut var_carr_dn10: f64 = *var_carr_dn10_slot;
        let mut var_carr_dn13: f64 = *var_carr_dn13_slot;
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
        let mut var_dnm_dn13: f64 = *var_dnm_dn13_slot;
        let mut var_dnm_dn2: f64 = *var_dnm_dn2_slot;
        let mut var_dnm_dn4: f64 = *var_dnm_dn4_slot;
        let mut var_dnm_dn5: f64 = *var_dnm_dn5_slot;
        let mut var_dnm_dn6: f64 = *var_dnm_dn6_slot;
        let mut var_dnm_dn7: f64 = *var_dnm_dn7_slot;
        let mut var_dnm_dn8: f64 = *var_dnm_dn8_slot;
        let mut var_dnm_dn9: f64 = *var_dnm_dn9_slot;
        let mut var_guard2373: f64 = *var_guard2373_slot;
        let mut var_guard2374: f64 = *var_guard2374_slot;
        let mut var_guard2375: f64 = *var_guard2375_slot;
        let mut var_guard2376: f64 = *var_guard2376_slot;
        let mut var_guard2377: f64 = *var_guard2377_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_mm: f64 = *var_mm_slot;
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

        let (assign104320_e156507, assign104320_e156507_d_n0, assign104320_e156507_d_n2, assign104320_e156507_d_n4, assign104320_e156507_d_n5, assign104320_e156507_d_n6, assign104320_e156507_d_n7, assign104320_e156507_d_n8, assign104320_e156507_d_n9, assign104320_e156507_d_n10, assign104320_e156507_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign104320_e156507;
        var_xp_dn0 = assign104320_e156507_d_n0;
        var_xp_dn2 = assign104320_e156507_d_n2;
        var_xp_dn4 = assign104320_e156507_d_n4;
        var_xp_dn5 = assign104320_e156507_d_n5;
        var_xp_dn6 = assign104320_e156507_d_n6;
        var_xp_dn7 = assign104320_e156507_d_n7;
        var_xp_dn8 = assign104320_e156507_d_n8;
        var_xp_dn9 = assign104320_e156507_d_n9;
        var_xp_dn10 = assign104320_e156507_d_n10;
        var_xp_dn13 = assign104320_e156507_d_n13;

        let (assign104330_e156518, assign104330_e156518_d_n0, assign104330_e156518_d_n2, assign104330_e156518_d_n4, assign104330_e156518_d_n5, assign104330_e156518_d_n6, assign104330_e156518_d_n7, assign104330_e156518_d_n8, assign104330_e156518_d_n9, assign104330_e156518_d_n10, assign104330_e156518_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign104330_e156518;
        var_xmp_dn0 = assign104330_e156518_d_n0;
        var_xmp_dn2 = assign104330_e156518_d_n2;
        var_xmp_dn4 = assign104330_e156518_d_n4;
        var_xmp_dn5 = assign104330_e156518_d_n5;
        var_xmp_dn6 = assign104330_e156518_d_n6;
        var_xmp_dn7 = assign104330_e156518_d_n7;
        var_xmp_dn8 = assign104330_e156518_d_n8;
        var_xmp_dn9 = assign104330_e156518_d_n9;
        var_xmp_dn10 = assign104330_e156518_d_n10;
        var_xmp_dn13 = assign104330_e156518_d_n13;

        let (assign104340_e156529,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign104340_e156529;

        let (assign104350_e156540,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        (0.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign104350_e156540;

        let (assign104360_e156551, assign104360_e156551_d_n0, assign104360_e156551_d_n2, assign104360_e156551_d_n4, assign104360_e156551_d_n5, assign104360_e156551_d_n6, assign104360_e156551_d_n7, assign104360_e156551_d_n8, assign104360_e156551_d_n9, assign104360_e156551_d_n10, assign104360_e156551_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    }
};
        var_arg = assign104360_e156551;
        var_arg_dn0 = assign104360_e156551_d_n0;
        var_arg_dn2 = assign104360_e156551_d_n2;
        var_arg_dn4 = assign104360_e156551_d_n4;
        var_arg_dn5 = assign104360_e156551_d_n5;
        var_arg_dn6 = assign104360_e156551_d_n6;
        var_arg_dn7 = assign104360_e156551_d_n7;
        var_arg_dn8 = assign104360_e156551_d_n8;
        var_arg_dn9 = assign104360_e156551_d_n9;
        var_arg_dn10 = assign104360_e156551_d_n10;
        var_arg_dn13 = assign104360_e156551_d_n13;

        let (assign104370_e156562, assign104370_e156562_d_n0, assign104370_e156562_d_n2, assign104370_e156562_d_n4, assign104370_e156562_d_n5, assign104370_e156562_d_n6, assign104370_e156562_d_n7, assign104370_e156562_d_n8, assign104370_e156562_d_n9, assign104370_e156562_d_n10, assign104370_e156562_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign104370_e156562;
        var_dnm_dn0 = assign104370_e156562_d_n0;
        var_dnm_dn2 = assign104370_e156562_d_n2;
        var_dnm_dn4 = assign104370_e156562_d_n4;
        var_dnm_dn5 = assign104370_e156562_d_n5;
        var_dnm_dn6 = assign104370_e156562_d_n6;
        var_dnm_dn7 = assign104370_e156562_d_n7;
        var_dnm_dn8 = assign104370_e156562_d_n8;
        var_dnm_dn9 = assign104370_e156562_d_n9;
        var_dnm_dn10 = assign104370_e156562_d_n10;
        var_dnm_dn13 = assign104370_e156562_d_n13;

        let (assign104380_e156573,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign104380_e156573;

        let mut assign104390_loop_guard: usize = 0;
        while {
            let assign104390_cond_e156585: f64 = if (((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) && (var_m0 < p.p442)) { 1.0 } else { 0.0 };
            assign104390_cond_e156585 != 0.0
        } {
            assign104390_loop_guard += 1;
            assert!(assign104390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign104390_body0_e156598, assign104390_body0_e156598_d_n0, assign104390_body0_e156598_d_n2, assign104390_body0_e156598_d_n4, assign104390_body0_e156598_d_n5, assign104390_body0_e156598_d_n6, assign104390_body0_e156598_d_n7, assign104390_body0_e156598_d_n8, assign104390_body0_e156598_d_n9, assign104390_body0_e156598_d_n10, assign104390_body0_e156598_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        let assign104390_body0_e156596: f64 = (var_xp * var_x2);
        (assign104390_body0_e156596, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn13 * var_x2) + (var_xp * var_x2_dn13)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
            var_xp = assign104390_body0_e156598;
            var_xp_dn0 = assign104390_body0_e156598_d_n0;
            var_xp_dn2 = assign104390_body0_e156598_d_n2;
            var_xp_dn4 = assign104390_body0_e156598_d_n4;
            var_xp_dn5 = assign104390_body0_e156598_d_n5;
            var_xp_dn6 = assign104390_body0_e156598_d_n6;
            var_xp_dn7 = assign104390_body0_e156598_d_n7;
            var_xp_dn8 = assign104390_body0_e156598_d_n8;
            var_xp_dn9 = assign104390_body0_e156598_d_n9;
            var_xp_dn10 = assign104390_body0_e156598_d_n10;
            var_xp_dn13 = assign104390_body0_e156598_d_n13;
            let (assign104390_body1_e156611, assign104390_body1_e156611_d_n0, assign104390_body1_e156611_d_n2, assign104390_body1_e156611_d_n4, assign104390_body1_e156611_d_n5, assign104390_body1_e156611_d_n6, assign104390_body1_e156611_d_n7, assign104390_body1_e156611_d_n8, assign104390_body1_e156611_d_n9, assign104390_body1_e156611_d_n10, assign104390_body1_e156611_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        let assign104390_body1_e156609: f64 = (var_xmp * var_xmax2);
        (assign104390_body1_e156609, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn13 * var_xmax2) + (var_xmp * var_xmax2_dn13)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
            var_xmp = assign104390_body1_e156611;
            var_xmp_dn0 = assign104390_body1_e156611_d_n0;
            var_xmp_dn2 = assign104390_body1_e156611_d_n2;
            var_xmp_dn4 = assign104390_body1_e156611_d_n4;
            var_xmp_dn5 = assign104390_body1_e156611_d_n5;
            var_xmp_dn6 = assign104390_body1_e156611_d_n6;
            var_xmp_dn7 = assign104390_body1_e156611_d_n7;
            var_xmp_dn8 = assign104390_body1_e156611_d_n8;
            var_xmp_dn9 = assign104390_body1_e156611_d_n9;
            var_xmp_dn10 = assign104390_body1_e156611_d_n10;
            var_xmp_dn13 = assign104390_body1_e156611_d_n13;
            let (assign104390_body2_e156624,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        let assign104390_body2_e156622: f64 = (var_m0 + 1.0);
        (assign104390_body2_e156622,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign104390_body2_e156624;
        }

        let (assign104400_e156637, assign104400_e156637_d_n0, assign104400_e156637_d_n2, assign104400_e156637_d_n4, assign104400_e156637_d_n5, assign104400_e156637_d_n6, assign104400_e156637_d_n7, assign104400_e156637_d_n8, assign104400_e156637_d_n9, assign104400_e156637_d_n10, assign104400_e156637_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        let assign104400_e156635: f64 = (var_xp + var_xmp);
        (assign104400_e156635, (var_xp_dn0 + var_xmp_dn0), (var_xp_dn2 + var_xmp_dn2), (var_xp_dn4 + var_xmp_dn4), (var_xp_dn5 + var_xmp_dn5), (var_xp_dn6 + var_xmp_dn6), (var_xp_dn7 + var_xmp_dn7), (var_xp_dn8 + var_xmp_dn8), (var_xp_dn9 + var_xmp_dn9), (var_xp_dn10 + var_xmp_dn10), (var_xp_dn13 + var_xmp_dn13),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    }
};
        var_arg = assign104400_e156637;
        var_arg_dn0 = assign104400_e156637_d_n0;
        var_arg_dn2 = assign104400_e156637_d_n2;
        var_arg_dn4 = assign104400_e156637_d_n4;
        var_arg_dn5 = assign104400_e156637_d_n5;
        var_arg_dn6 = assign104400_e156637_d_n6;
        var_arg_dn7 = assign104400_e156637_d_n7;
        var_arg_dn8 = assign104400_e156637_d_n8;
        var_arg_dn9 = assign104400_e156637_d_n9;
        var_arg_dn10 = assign104400_e156637_d_n10;
        var_arg_dn13 = assign104400_e156637_d_n13;

        let (assign104410_e156648, assign104410_e156648_d_n0, assign104410_e156648_d_n2, assign104410_e156648_d_n4, assign104410_e156648_d_n5, assign104410_e156648_d_n6, assign104410_e156648_d_n7, assign104410_e156648_d_n8, assign104410_e156648_d_n9, assign104410_e156648_d_n10, assign104410_e156648_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign104410_e156648;
        var_dnm_dn0 = assign104410_e156648_d_n0;
        var_dnm_dn2 = assign104410_e156648_d_n2;
        var_dnm_dn4 = assign104410_e156648_d_n4;
        var_dnm_dn5 = assign104410_e156648_d_n5;
        var_dnm_dn6 = assign104410_e156648_d_n6;
        var_dnm_dn7 = assign104410_e156648_d_n7;
        var_dnm_dn8 = assign104410_e156648_d_n8;
        var_dnm_dn9 = assign104410_e156648_d_n9;
        var_dnm_dn10 = assign104410_e156648_d_n10;
        var_dnm_dn13 = assign104410_e156648_d_n13;

        let assign104420_e156663: f64 = if ((((p.p442 == 1.0) || (p.p442 == 2.0)) || (p.p442 == 4.0)) || (p.p442 == 8.0)) { 1.0 } else { 0.0 };
        var_guard2373 = assign104420_e156663;

        let assign104430_e156666: f64 = if p.p442 == 1.0 { 1.0 } else { 0.0 };
        var_guard2374 = assign104430_e156666;

        let (assign104440_e156681,) = {
    if ((((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) && (var_guard2373 != 0.0)) && (var_guard2374 != 0.0)) {
        (1.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign104440_e156681;

        let assign104450_e156684: f64 = if p.p442 == 2.0 { 1.0 } else { 0.0 };
        var_guard2375 = assign104450_e156684;

        let (assign104460_e156702,) = {
    if (((((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) && (var_guard2373 != 0.0)) && (var_guard2374 == 0.0)) && (var_guard2375 != 0.0)) {
        (2.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign104460_e156702;

        let assign104470_e156705: f64 = if p.p442 == 4.0 { 1.0 } else { 0.0 };
        var_guard2376 = assign104470_e156705;

        let (assign104480_e156726,) = {
    if ((((((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) && (var_guard2373 != 0.0)) && (var_guard2374 == 0.0)) && (var_guard2375 == 0.0)) && (var_guard2376 != 0.0)) {
        (3.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign104480_e156726;

        let assign104490_e156729: f64 = if p.p442 == 8.0 { 1.0 } else { 0.0 };
        var_guard2377 = assign104490_e156729;

        let (assign104500_e156753,) = {
    if (((((((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) && (var_guard2373 != 0.0)) && (var_guard2374 == 0.0)) && (var_guard2375 == 0.0)) && (var_guard2376 == 0.0)) && (var_guard2377 != 0.0)) {
        (4.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign104500_e156753;

        let (assign104510_e156766,) = {
    if (((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) && (var_guard2373 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign104510_e156766;

        let mut assign104520_loop_guard: usize = 0;
        while {
            let assign104520_cond_e156780: f64 = if ((((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) && (var_guard2373 != 0.0)) && (var_m0 < var_mm)) { 1.0 } else { 0.0 };
            assign104520_cond_e156780 != 0.0
        } {
            assign104520_loop_guard += 1;
            assert!(assign104520_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign104520_body0_e156794, assign104520_body0_e156794_d_n0, assign104520_body0_e156794_d_n2, assign104520_body0_e156794_d_n4, assign104520_body0_e156794_d_n5, assign104520_body0_e156794_d_n6, assign104520_body0_e156794_d_n7, assign104520_body0_e156794_d_n8, assign104520_body0_e156794_d_n9, assign104520_body0_e156794_d_n10, assign104520_body0_e156794_d_n13,) = {
    if (((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) && (var_guard2373 != 0.0)) {
        let assign104520_body0_e156792: f64 = (var_dnm).sqrt();
        (assign104520_body0_e156792, (var_dnm_dn0 / (2.0 * assign104520_body0_e156792)), (var_dnm_dn2 / (2.0 * assign104520_body0_e156792)), (var_dnm_dn4 / (2.0 * assign104520_body0_e156792)), (var_dnm_dn5 / (2.0 * assign104520_body0_e156792)), (var_dnm_dn6 / (2.0 * assign104520_body0_e156792)), (var_dnm_dn7 / (2.0 * assign104520_body0_e156792)), (var_dnm_dn8 / (2.0 * assign104520_body0_e156792)), (var_dnm_dn9 / (2.0 * assign104520_body0_e156792)), (var_dnm_dn10 / (2.0 * assign104520_body0_e156792)), (var_dnm_dn13 / (2.0 * assign104520_body0_e156792)),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
            var_dnm = assign104520_body0_e156794;
            var_dnm_dn0 = assign104520_body0_e156794_d_n0;
            var_dnm_dn2 = assign104520_body0_e156794_d_n2;
            var_dnm_dn4 = assign104520_body0_e156794_d_n4;
            var_dnm_dn5 = assign104520_body0_e156794_d_n5;
            var_dnm_dn6 = assign104520_body0_e156794_d_n6;
            var_dnm_dn7 = assign104520_body0_e156794_d_n7;
            var_dnm_dn8 = assign104520_body0_e156794_d_n8;
            var_dnm_dn9 = assign104520_body0_e156794_d_n9;
            var_dnm_dn10 = assign104520_body0_e156794_d_n10;
            var_dnm_dn13 = assign104520_body0_e156794_d_n13;
            let (assign104520_body1_e156809,) = {
    if (((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) && (var_guard2373 != 0.0)) {
        let assign104520_body1_e156807: f64 = (var_m0 + 1.0);
        (assign104520_body1_e156807,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign104520_body1_e156809;
        }

        let (assign104530_e156834, assign104530_e156834_d_n0, assign104530_e156834_d_n2, assign104530_e156834_d_n4, assign104530_e156834_d_n5, assign104530_e156834_d_n6, assign104530_e156834_d_n7, assign104530_e156834_d_n8, assign104530_e156834_d_n9, assign104530_e156834_d_n10, assign104530_e156834_d_n13,) = {
    if (((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) && (var_guard2373 == 0.0)) {
        let (assign104530_e156832, assign104530_e156832_d_n0, assign104530_e156832_d_n2, assign104530_e156832_d_n4, assign104530_e156832_d_n5, assign104530_e156832_d_n6, assign104530_e156832_d_n7, assign104530_e156832_d_n8, assign104530_e156832_d_n9, assign104530_e156832_d_n10, assign104530_e156832_d_n13,) = {
            if (var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign104530_e156829: f64 = (2.0 * p.p442);
                let assign104530_e156830: f64 = (1.0 / assign104530_e156829);
                let assign104530_e156831: f64 = (var_dnm).powf(assign104530_e156830);
                (assign104530_e156831, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((var_dnm).powf(assign104530_e156830 - 1.0) * var_dnm_dn0)) } } else { (assign104530_e156831 * (assign104530_e156830 * (var_dnm_dn0 / var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((var_dnm).powf(assign104530_e156830 - 1.0) * var_dnm_dn2)) } } else { (assign104530_e156831 * (assign104530_e156830 * (var_dnm_dn2 / var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((var_dnm).powf(assign104530_e156830 - 1.0) * var_dnm_dn4)) } } else { (assign104530_e156831 * (assign104530_e156830 * (var_dnm_dn4 / var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((var_dnm).powf(assign104530_e156830 - 1.0) * var_dnm_dn5)) } } else { (assign104530_e156831 * (assign104530_e156830 * (var_dnm_dn5 / var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((var_dnm).powf(assign104530_e156830 - 1.0) * var_dnm_dn6)) } } else { (assign104530_e156831 * (assign104530_e156830 * (var_dnm_dn6 / var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((var_dnm).powf(assign104530_e156830 - 1.0) * var_dnm_dn7)) } } else { (assign104530_e156831 * (assign104530_e156830 * (var_dnm_dn7 / var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((var_dnm).powf(assign104530_e156830 - 1.0) * var_dnm_dn8)) } } else { (assign104530_e156831 * (assign104530_e156830 * (var_dnm_dn8 / var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((var_dnm).powf(assign104530_e156830 - 1.0) * var_dnm_dn9)) } } else { (assign104530_e156831 * (assign104530_e156830 * (var_dnm_dn9 / var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((var_dnm).powf(assign104530_e156830 - 1.0) * var_dnm_dn10)) } } else { (assign104530_e156831 * (assign104530_e156830 * (var_dnm_dn10 / var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((var_dnm).powf(assign104530_e156830 - 1.0) * var_dnm_dn13)) } } else { (assign104530_e156831 * (assign104530_e156830 * (var_dnm_dn13 / var_dnm))) },)
            }
        };
        (assign104530_e156832, assign104530_e156832_d_n0, assign104530_e156832_d_n2, assign104530_e156832_d_n4, assign104530_e156832_d_n5, assign104530_e156832_d_n6, assign104530_e156832_d_n7, assign104530_e156832_d_n8, assign104530_e156832_d_n9, assign104530_e156832_d_n10, assign104530_e156832_d_n13,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign104530_e156834;
        var_dnm_dn0 = assign104530_e156834_d_n0;
        var_dnm_dn2 = assign104530_e156834_d_n2;
        var_dnm_dn4 = assign104530_e156834_d_n4;
        var_dnm_dn5 = assign104530_e156834_d_n5;
        var_dnm_dn6 = assign104530_e156834_d_n6;
        var_dnm_dn7 = assign104530_e156834_d_n7;
        var_dnm_dn8 = assign104530_e156834_d_n8;
        var_dnm_dn9 = assign104530_e156834_d_n9;
        var_dnm_dn10 = assign104530_e156834_d_n10;
        var_dnm_dn13 = assign104530_e156834_d_n13;

        let (assign104540_e156847, assign104540_e156847_d_n0, assign104540_e156847_d_n2, assign104540_e156847_d_n4, assign104540_e156847_d_n5, assign104540_e156847_d_n6, assign104540_e156847_d_n7, assign104540_e156847_d_n8, assign104540_e156847_d_n9, assign104540_e156847_d_n10, assign104540_e156847_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        let assign104540_e156845: f64 = (1.0 / var_dnm);
        (assign104540_e156845, (-(var_dnm_dn0 / (var_dnm * var_dnm))), (-(var_dnm_dn2 / (var_dnm * var_dnm))), (-(var_dnm_dn4 / (var_dnm * var_dnm))), (-(var_dnm_dn5 / (var_dnm * var_dnm))), (-(var_dnm_dn6 / (var_dnm * var_dnm))), (-(var_dnm_dn7 / (var_dnm * var_dnm))), (-(var_dnm_dn8 / (var_dnm * var_dnm))), (-(var_dnm_dn9 / (var_dnm * var_dnm))), (-(var_dnm_dn10 / (var_dnm * var_dnm))), (-(var_dnm_dn13 / (var_dnm * var_dnm))),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign104540_e156847;
        var_dnm_dn0 = assign104540_e156847_d_n0;
        var_dnm_dn2 = assign104540_e156847_d_n2;
        var_dnm_dn4 = assign104540_e156847_d_n4;
        var_dnm_dn5 = assign104540_e156847_d_n5;
        var_dnm_dn6 = assign104540_e156847_d_n6;
        var_dnm_dn7 = assign104540_e156847_d_n7;
        var_dnm_dn8 = assign104540_e156847_d_n8;
        var_dnm_dn9 = assign104540_e156847_d_n9;
        var_dnm_dn10 = assign104540_e156847_d_n10;
        var_dnm_dn13 = assign104540_e156847_d_n13;

        let (assign104550_e156864, assign104550_e156864_d_n0, assign104550_e156864_d_n2, assign104550_e156864_d_n4, assign104550_e156864_d_n5, assign104550_e156864_d_n6, assign104550_e156864_d_n7, assign104550_e156864_d_n8, assign104550_e156864_d_n9, assign104550_e156864_d_n10, assign104550_e156864_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        let assign104550_e156859: f64 = (var_noverd * p.p441);
        let assign104550_e156860: f64 = (var_tmf1 * assign104550_e156859);
        let assign104550_e156862: f64 = (assign104550_e156860 * var_dnm);
        (assign104550_e156862, (((var_tmf1_dn0 * assign104550_e156859) * var_dnm) + (assign104550_e156860 * var_dnm_dn0)), (((var_tmf1_dn2 * assign104550_e156859) * var_dnm) + (assign104550_e156860 * var_dnm_dn2)), (((var_tmf1_dn4 * assign104550_e156859) * var_dnm) + (assign104550_e156860 * var_dnm_dn4)), (((var_tmf1_dn5 * assign104550_e156859) * var_dnm) + (assign104550_e156860 * var_dnm_dn5)), (((var_tmf1_dn6 * assign104550_e156859) * var_dnm) + (assign104550_e156860 * var_dnm_dn6)), (((var_tmf1_dn7 * assign104550_e156859) * var_dnm) + (assign104550_e156860 * var_dnm_dn7)), (((var_tmf1_dn8 * assign104550_e156859) * var_dnm) + (assign104550_e156860 * var_dnm_dn8)), (((var_tmf1_dn9 * assign104550_e156859) * var_dnm) + (assign104550_e156860 * var_dnm_dn9)), (((var_tmf1_dn10 * assign104550_e156859) * var_dnm) + (assign104550_e156860 * var_dnm_dn10)), (((var_tmf1_dn13 * assign104550_e156859) * var_dnm) + (assign104550_e156860 * var_dnm_dn13)),)
    } else {
        (var_tmf0, var_tmf0_dn0, var_tmf0_dn2, var_tmf0_dn4, var_tmf0_dn5, var_tmf0_dn6, var_tmf0_dn7, var_tmf0_dn8, var_tmf0_dn9, var_tmf0_dn10, var_tmf0_dn13,)
    }
};
        var_tmf0 = assign104550_e156864;
        var_tmf0_dn0 = assign104550_e156864_d_n0;
        var_tmf0_dn2 = assign104550_e156864_d_n2;
        var_tmf0_dn4 = assign104550_e156864_d_n4;
        var_tmf0_dn5 = assign104550_e156864_d_n5;
        var_tmf0_dn6 = assign104550_e156864_d_n6;
        var_tmf0_dn7 = assign104550_e156864_d_n7;
        var_tmf0_dn8 = assign104550_e156864_d_n8;
        var_tmf0_dn9 = assign104550_e156864_d_n9;
        var_tmf0_dn10 = assign104550_e156864_d_n10;
        var_tmf0_dn13 = assign104550_e156864_d_n13;

        let (assign104560_e156883, assign104560_e156883_d_n0, assign104560_e156883_d_n2, assign104560_e156883_d_n4, assign104560_e156883_d_n5, assign104560_e156883_d_n6, assign104560_e156883_d_n7, assign104560_e156883_d_n8, assign104560_e156883_d_n9, assign104560_e156883_d_n10, assign104560_e156883_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        let assign104560_e156875: f64 = (var_noverd * p.p441);
        let assign104560_e156877: f64 = (assign104560_e156875 * var_xmp);
        let assign104560_e156879: f64 = (assign104560_e156877 * var_dnm);
        let assign104560_e156881: f64 = (assign104560_e156879 / var_arg);
        (assign104560_e156881, ((((((assign104560_e156875 * var_xmp_dn0) * var_dnm) + (assign104560_e156877 * var_dnm_dn0)) * var_arg) - (assign104560_e156879 * var_arg_dn0)) / (var_arg * var_arg)), ((((((assign104560_e156875 * var_xmp_dn2) * var_dnm) + (assign104560_e156877 * var_dnm_dn2)) * var_arg) - (assign104560_e156879 * var_arg_dn2)) / (var_arg * var_arg)), ((((((assign104560_e156875 * var_xmp_dn4) * var_dnm) + (assign104560_e156877 * var_dnm_dn4)) * var_arg) - (assign104560_e156879 * var_arg_dn4)) / (var_arg * var_arg)), ((((((assign104560_e156875 * var_xmp_dn5) * var_dnm) + (assign104560_e156877 * var_dnm_dn5)) * var_arg) - (assign104560_e156879 * var_arg_dn5)) / (var_arg * var_arg)), ((((((assign104560_e156875 * var_xmp_dn6) * var_dnm) + (assign104560_e156877 * var_dnm_dn6)) * var_arg) - (assign104560_e156879 * var_arg_dn6)) / (var_arg * var_arg)), ((((((assign104560_e156875 * var_xmp_dn7) * var_dnm) + (assign104560_e156877 * var_dnm_dn7)) * var_arg) - (assign104560_e156879 * var_arg_dn7)) / (var_arg * var_arg)), ((((((assign104560_e156875 * var_xmp_dn8) * var_dnm) + (assign104560_e156877 * var_dnm_dn8)) * var_arg) - (assign104560_e156879 * var_arg_dn8)) / (var_arg * var_arg)), ((((((assign104560_e156875 * var_xmp_dn9) * var_dnm) + (assign104560_e156877 * var_dnm_dn9)) * var_arg) - (assign104560_e156879 * var_arg_dn9)) / (var_arg * var_arg)), ((((((assign104560_e156875 * var_xmp_dn10) * var_dnm) + (assign104560_e156877 * var_dnm_dn10)) * var_arg) - (assign104560_e156879 * var_arg_dn10)) / (var_arg * var_arg)), ((((((assign104560_e156875 * var_xmp_dn13) * var_dnm) + (assign104560_e156877 * var_dnm_dn13)) * var_arg) - (assign104560_e156879 * var_arg_dn13)) / (var_arg * var_arg)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign104560_e156883;
        var_t0_dn0 = assign104560_e156883_d_n0;
        var_t0_dn2 = assign104560_e156883_d_n2;
        var_t0_dn4 = assign104560_e156883_d_n4;
        var_t0_dn5 = assign104560_e156883_d_n5;
        var_t0_dn6 = assign104560_e156883_d_n6;
        var_t0_dn7 = assign104560_e156883_d_n7;
        var_t0_dn8 = assign104560_e156883_d_n8;
        var_t0_dn9 = assign104560_e156883_d_n9;
        var_t0_dn10 = assign104560_e156883_d_n10;
        var_t0_dn13 = assign104560_e156883_d_n13;

        let (assign104570_e156902, assign104570_e156902_d_n0, assign104570_e156902_d_n2, assign104570_e156902_d_n4, assign104570_e156902_d_n5, assign104570_e156902_d_n6, assign104570_e156902_d_n7, assign104570_e156902_d_n8, assign104570_e156902_d_n9, assign104570_e156902_d_n10, assign104570_e156902_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        let assign104570_e156894: f64 = (var_noverd * p.p440);
        let assign104570_e156897: f64 = (var_noverd * p.p441);
        let assign104570_e156898: f64 = (assign104570_e156894 - assign104570_e156897);
        let assign104570_e156900: f64 = (assign104570_e156898 + var_tmf0);
        (assign104570_e156900, var_tmf0_dn0, var_tmf0_dn2, var_tmf0_dn4, var_tmf0_dn5, var_tmf0_dn6, var_tmf0_dn7, var_tmf0_dn8, var_tmf0_dn9, var_tmf0_dn10, var_tmf0_dn13,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign104570_e156902;
        var_t2_dn0 = assign104570_e156902_d_n0;
        var_t2_dn2 = assign104570_e156902_d_n2;
        var_t2_dn4 = assign104570_e156902_d_n4;
        var_t2_dn5 = assign104570_e156902_d_n5;
        var_t2_dn6 = assign104570_e156902_d_n6;
        var_t2_dn7 = assign104570_e156902_d_n7;
        var_t2_dn8 = assign104570_e156902_d_n8;
        var_t2_dn9 = assign104570_e156902_d_n9;
        var_t2_dn10 = assign104570_e156902_d_n10;
        var_t2_dn13 = assign104570_e156902_d_n13;

        let (assign104580_e156913, assign104580_e156913_d_n0, assign104580_e156913_d_n2, assign104580_e156913_d_n4, assign104580_e156913_d_n5, assign104580_e156913_d_n6, assign104580_e156913_d_n7, assign104580_e156913_d_n8, assign104580_e156913_d_n9, assign104580_e156913_d_n10, assign104580_e156913_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 != 0.0)) {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign104580_e156913;
        var_t0_dn0 = assign104580_e156913_d_n0;
        var_t0_dn2 = assign104580_e156913_d_n2;
        var_t0_dn4 = assign104580_e156913_d_n4;
        var_t0_dn5 = assign104580_e156913_d_n5;
        var_t0_dn6 = assign104580_e156913_d_n6;
        var_t0_dn7 = assign104580_e156913_d_n7;
        var_t0_dn8 = assign104580_e156913_d_n8;
        var_t0_dn9 = assign104580_e156913_d_n9;
        var_t0_dn10 = assign104580_e156913_d_n10;
        var_t0_dn13 = assign104580_e156913_d_n13;

        let (assign104590_e156925, assign104590_e156925_d_n0, assign104590_e156925_d_n2, assign104590_e156925_d_n4, assign104590_e156925_d_n5, assign104590_e156925_d_n6, assign104590_e156925_d_n7, assign104590_e156925_d_n8, assign104590_e156925_d_n9, assign104590_e156925_d_n10, assign104590_e156925_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 == 0.0)) {
        (var_carr, var_carr_dn0, var_carr_dn2, var_carr_dn4, var_carr_dn5, var_carr_dn6, var_carr_dn7, var_carr_dn8, var_carr_dn9, var_carr_dn10, var_carr_dn13,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign104590_e156925;
        var_t2_dn0 = assign104590_e156925_d_n0;
        var_t2_dn2 = assign104590_e156925_d_n2;
        var_t2_dn4 = assign104590_e156925_d_n4;
        var_t2_dn5 = assign104590_e156925_d_n5;
        var_t2_dn6 = assign104590_e156925_d_n6;
        var_t2_dn7 = assign104590_e156925_d_n7;
        var_t2_dn8 = assign104590_e156925_d_n8;
        var_t2_dn9 = assign104590_e156925_d_n9;
        var_t2_dn10 = assign104590_e156925_d_n10;
        var_t2_dn13 = assign104590_e156925_d_n13;

        let (assign104600_e156937, assign104600_e156937_d_n0, assign104600_e156937_d_n2, assign104600_e156937_d_n4, assign104600_e156937_d_n5, assign104600_e156937_d_n6, assign104600_e156937_d_n7, assign104600_e156937_d_n8, assign104600_e156937_d_n9, assign104600_e156937_d_n10, assign104600_e156937_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) && (var_guard2372 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign104600_e156937;
        var_t0_dn0 = assign104600_e156937_d_n0;
        var_t0_dn2 = assign104600_e156937_d_n2;
        var_t0_dn4 = assign104600_e156937_d_n4;
        var_t0_dn5 = assign104600_e156937_d_n5;
        var_t0_dn6 = assign104600_e156937_d_n6;
        var_t0_dn7 = assign104600_e156937_d_n7;
        var_t0_dn8 = assign104600_e156937_d_n8;
        var_t0_dn9 = assign104600_e156937_d_n9;
        var_t0_dn10 = assign104600_e156937_d_n10;
        var_t0_dn13 = assign104600_e156937_d_n13;

        let (assign104610_e156946, assign104610_e156946_d_n0, assign104610_e156946_d_n2, assign104610_e156946_d_n4, assign104610_e156946_d_n5, assign104610_e156946_d_n6, assign104610_e156946_d_n7, assign104610_e156946_d_n8, assign104610_e156946_d_n9, assign104610_e156946_d_n10, assign104610_e156946_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2371 != 0.0)) {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    } else {
        (var_carr, var_carr_dn0, var_carr_dn2, var_carr_dn4, var_carr_dn5, var_carr_dn6, var_carr_dn7, var_carr_dn8, var_carr_dn9, var_carr_dn10, var_carr_dn13,)
    }
};
        var_carr = assign104610_e156946;
        var_carr_dn0 = assign104610_e156946_d_n0;
        var_carr_dn2 = assign104610_e156946_d_n2;
        var_carr_dn4 = assign104610_e156946_d_n4;
        var_carr_dn5 = assign104610_e156946_d_n5;
        var_carr_dn6 = assign104610_e156946_d_n6;
        var_carr_dn7 = assign104610_e156946_d_n7;
        var_carr_dn8 = assign104610_e156946_d_n8;
        var_carr_dn9 = assign104610_e156946_d_n9;
        var_carr_dn10 = assign104610_e156946_d_n10;
        var_carr_dn13 = assign104610_e156946_d_n13;

        let (assign104620_e156954, assign104620_e156954_d_n0, assign104620_e156954_d_n2, assign104620_e156954_d_n4, assign104620_e156954_d_n5, assign104620_e156954_d_n6, assign104620_e156954_d_n7, assign104620_e156954_d_n8, assign104620_e156954_d_n9, assign104620_e156954_d_n10, assign104620_e156954_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104620_e156952: f64 = (-var_rd_ps0ld);
        (assign104620_e156952, (-var_rd_ps0ld_dn0), (-var_rd_ps0ld_dn2), (-var_rd_ps0ld_dn4), (-var_rd_ps0ld_dn5), (-var_rd_ps0ld_dn6), (-var_rd_ps0ld_dn7), (-var_rd_ps0ld_dn8), (-var_rd_ps0ld_dn9), (-var_rd_ps0ld_dn10), (-var_rd_ps0ld_dn13),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign104620_e156954;
        var_t0_dn0 = assign104620_e156954_d_n0;
        var_t0_dn2 = assign104620_e156954_d_n2;
        var_t0_dn4 = assign104620_e156954_d_n4;
        var_t0_dn5 = assign104620_e156954_d_n5;
        var_t0_dn6 = assign104620_e156954_d_n6;
        var_t0_dn7 = assign104620_e156954_d_n7;
        var_t0_dn8 = assign104620_e156954_d_n8;
        var_t0_dn9 = assign104620_e156954_d_n9;
        var_t0_dn10 = assign104620_e156954_d_n10;
        var_t0_dn13 = assign104620_e156954_d_n13;

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
        *var_carr_slot = var_carr;
        *var_carr_dn0_slot = var_carr_dn0;
        *var_carr_dn10_slot = var_carr_dn10;
        *var_carr_dn13_slot = var_carr_dn13;
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
        *var_dnm_dn13_slot = var_dnm_dn13;
        *var_dnm_dn2_slot = var_dnm_dn2;
        *var_dnm_dn4_slot = var_dnm_dn4;
        *var_dnm_dn5_slot = var_dnm_dn5;
        *var_dnm_dn6_slot = var_dnm_dn6;
        *var_dnm_dn7_slot = var_dnm_dn7;
        *var_dnm_dn8_slot = var_dnm_dn8;
        *var_dnm_dn9_slot = var_dnm_dn9;
        *var_guard2373_slot = var_guard2373;
        *var_guard2374_slot = var_guard2374;
        *var_guard2375_slot = var_guard2375;
        *var_guard2376_slot = var_guard2376;
        *var_guard2377_slot = var_guard2377;
        *var_m0_slot = var_m0;
        *var_mm_slot = var_mm;
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
    }

    pub(super) fn stamp_transient_block_370(
        p: &Parameters,
        var_cx: f64,
        var_guard2336: f64,
        var_guard2356: f64,
        var_kdep: f64,
        var_kjunc: f64,
        var_kjunc_dn0: f64,
        var_kjunc_dn10: f64,
        var_kjunc_dn13: f64,
        var_kjunc_dn2: f64,
        var_kjunc_dn4: f64,
        var_kjunc_dn5: f64,
        var_kjunc_dn6: f64,
        var_kjunc_dn7: f64,
        var_kjunc_dn8: f64,
        var_kjunc_dn9: f64,
        var_rd_xldld: f64,
        var_vbs__blk2353: f64,
        var_vbs__blk2353_dn7: f64,
        var_vbs__blk2353_dn8: f64,
        var_vds__blk2352: f64,
        var_vds__blk2352_dn5: f64,
        var_vds__blk2352_dn7: f64,
        var_xmax: f64,
        var_guard2378_slot: &mut f64,
        var_guard2379_slot: &mut f64,
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
        var_wdepl_slot: &mut f64,
        var_wdepl_dn0_slot: &mut f64,
        var_wdepl_dn10_slot: &mut f64,
        var_wdepl_dn13_slot: &mut f64,
        var_wdepl_dn2_slot: &mut f64,
        var_wdepl_dn4_slot: &mut f64,
        var_wdepl_dn5_slot: &mut f64,
        var_wdepl_dn6_slot: &mut f64,
        var_wdepl_dn7_slot: &mut f64,
        var_wdepl_dn8_slot: &mut f64,
        var_wdepl_dn9_slot: &mut f64,
        var_wjunc_slot: &mut f64,
        var_wjunc0_slot: &mut f64,
        var_wjunc0_dn0_slot: &mut f64,
        var_wjunc0_dn10_slot: &mut f64,
        var_wjunc0_dn13_slot: &mut f64,
        var_wjunc0_dn2_slot: &mut f64,
        var_wjunc0_dn4_slot: &mut f64,
        var_wjunc0_dn5_slot: &mut f64,
        var_wjunc0_dn6_slot: &mut f64,
        var_wjunc0_dn7_slot: &mut f64,
        var_wjunc0_dn8_slot: &mut f64,
        var_wjunc0_dn9_slot: &mut f64,
        var_wjunc_dn0_slot: &mut f64,
        var_wjunc_dn10_slot: &mut f64,
        var_wjunc_dn13_slot: &mut f64,
        var_wjunc_dn2_slot: &mut f64,
        var_wjunc_dn4_slot: &mut f64,
        var_wjunc_dn5_slot: &mut f64,
        var_wjunc_dn6_slot: &mut f64,
        var_wjunc_dn7_slot: &mut f64,
        var_wjunc_dn8_slot: &mut f64,
        var_wjunc_dn9_slot: &mut f64,
        var_wrdrdjunc_slot: &mut f64,
        var_xov_slot: &mut f64,
        var_xov_dn0_slot: &mut f64,
        var_xov_dn10_slot: &mut f64,
        var_xov_dn13_slot: &mut f64,
        var_xov_dn2_slot: &mut f64,
        var_xov_dn4_slot: &mut f64,
        var_xov_dn5_slot: &mut f64,
        var_xov_dn6_slot: &mut f64,
        var_xov_dn7_slot: &mut f64,
        var_xov_dn8_slot: &mut f64,
        var_xov_dn9_slot: &mut f64,
    ) {
        let mut var_guard2378: f64 = *var_guard2378_slot;
        let mut var_guard2379: f64 = *var_guard2379_slot;
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
        let mut var_wdepl: f64 = *var_wdepl_slot;
        let mut var_wdepl_dn0: f64 = *var_wdepl_dn0_slot;
        let mut var_wdepl_dn10: f64 = *var_wdepl_dn10_slot;
        let mut var_wdepl_dn13: f64 = *var_wdepl_dn13_slot;
        let mut var_wdepl_dn2: f64 = *var_wdepl_dn2_slot;
        let mut var_wdepl_dn4: f64 = *var_wdepl_dn4_slot;
        let mut var_wdepl_dn5: f64 = *var_wdepl_dn5_slot;
        let mut var_wdepl_dn6: f64 = *var_wdepl_dn6_slot;
        let mut var_wdepl_dn7: f64 = *var_wdepl_dn7_slot;
        let mut var_wdepl_dn8: f64 = *var_wdepl_dn8_slot;
        let mut var_wdepl_dn9: f64 = *var_wdepl_dn9_slot;
        let mut var_wjunc: f64 = *var_wjunc_slot;
        let mut var_wjunc0: f64 = *var_wjunc0_slot;
        let mut var_wjunc0_dn0: f64 = *var_wjunc0_dn0_slot;
        let mut var_wjunc0_dn10: f64 = *var_wjunc0_dn10_slot;
        let mut var_wjunc0_dn13: f64 = *var_wjunc0_dn13_slot;
        let mut var_wjunc0_dn2: f64 = *var_wjunc0_dn2_slot;
        let mut var_wjunc0_dn4: f64 = *var_wjunc0_dn4_slot;
        let mut var_wjunc0_dn5: f64 = *var_wjunc0_dn5_slot;
        let mut var_wjunc0_dn6: f64 = *var_wjunc0_dn6_slot;
        let mut var_wjunc0_dn7: f64 = *var_wjunc0_dn7_slot;
        let mut var_wjunc0_dn8: f64 = *var_wjunc0_dn8_slot;
        let mut var_wjunc0_dn9: f64 = *var_wjunc0_dn9_slot;
        let mut var_wjunc_dn0: f64 = *var_wjunc_dn0_slot;
        let mut var_wjunc_dn10: f64 = *var_wjunc_dn10_slot;
        let mut var_wjunc_dn13: f64 = *var_wjunc_dn13_slot;
        let mut var_wjunc_dn2: f64 = *var_wjunc_dn2_slot;
        let mut var_wjunc_dn4: f64 = *var_wjunc_dn4_slot;
        let mut var_wjunc_dn5: f64 = *var_wjunc_dn5_slot;
        let mut var_wjunc_dn6: f64 = *var_wjunc_dn6_slot;
        let mut var_wjunc_dn7: f64 = *var_wjunc_dn7_slot;
        let mut var_wjunc_dn8: f64 = *var_wjunc_dn8_slot;
        let mut var_wjunc_dn9: f64 = *var_wjunc_dn9_slot;
        let mut var_wrdrdjunc: f64 = *var_wrdrdjunc_slot;
        let mut var_xov: f64 = *var_xov_slot;
        let mut var_xov_dn0: f64 = *var_xov_dn0_slot;
        let mut var_xov_dn10: f64 = *var_xov_dn10_slot;
        let mut var_xov_dn13: f64 = *var_xov_dn13_slot;
        let mut var_xov_dn2: f64 = *var_xov_dn2_slot;
        let mut var_xov_dn4: f64 = *var_xov_dn4_slot;
        let mut var_xov_dn5: f64 = *var_xov_dn5_slot;
        let mut var_xov_dn6: f64 = *var_xov_dn6_slot;
        let mut var_xov_dn7: f64 = *var_xov_dn7_slot;
        let mut var_xov_dn8: f64 = *var_xov_dn8_slot;
        let mut var_xov_dn9: f64 = *var_xov_dn9_slot;

        let (assign104630_e156970, assign104630_e156970_d_n0, assign104630_e156970_d_n2, assign104630_e156970_d_n4, assign104630_e156970_d_n5, assign104630_e156970_d_n6, assign104630_e156970_d_n7, assign104630_e156970_d_n8, assign104630_e156970_d_n9, assign104630_e156970_d_n10, assign104630_e156970_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104630_e156961: f64 = (var_t0 * var_t0);
        let assign104630_e156964: f64 = (4.0 * 0.01);
        let assign104630_e156966: f64 = (assign104630_e156964 * 0.01);
        let assign104630_e156967: f64 = (assign104630_e156961 + assign104630_e156966);
        let assign104630_e156968: f64 = (assign104630_e156967).sqrt();
        (assign104630_e156968, (((var_t0_dn0 * var_t0) + (var_t0 * var_t0_dn0)) / (2.0 * assign104630_e156968)), (((var_t0_dn2 * var_t0) + (var_t0 * var_t0_dn2)) / (2.0 * assign104630_e156968)), (((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4)) / (2.0 * assign104630_e156968)), (((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5)) / (2.0 * assign104630_e156968)), (((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6)) / (2.0 * assign104630_e156968)), (((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7)) / (2.0 * assign104630_e156968)), (((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8)) / (2.0 * assign104630_e156968)), (((var_t0_dn9 * var_t0) + (var_t0 * var_t0_dn9)) / (2.0 * assign104630_e156968)), (((var_t0_dn10 * var_t0) + (var_t0 * var_t0_dn10)) / (2.0 * assign104630_e156968)), (((var_t0_dn13 * var_t0) + (var_t0 * var_t0_dn13)) / (2.0 * assign104630_e156968)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign104630_e156970;
        var_tmf2_dn0 = assign104630_e156970_d_n0;
        var_tmf2_dn2 = assign104630_e156970_d_n2;
        var_tmf2_dn4 = assign104630_e156970_d_n4;
        var_tmf2_dn5 = assign104630_e156970_d_n5;
        var_tmf2_dn6 = assign104630_e156970_d_n6;
        var_tmf2_dn7 = assign104630_e156970_d_n7;
        var_tmf2_dn8 = assign104630_e156970_d_n8;
        var_tmf2_dn9 = assign104630_e156970_d_n9;
        var_tmf2_dn10 = assign104630_e156970_d_n10;
        var_tmf2_dn13 = assign104630_e156970_d_n13;

        let (assign104640_e156983, assign104640_e156983_d_n0, assign104640_e156983_d_n2, assign104640_e156983_d_n4, assign104640_e156983_d_n5, assign104640_e156983_d_n6, assign104640_e156983_d_n7, assign104640_e156983_d_n8, assign104640_e156983_d_n9, assign104640_e156983_d_n10, assign104640_e156983_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104640_e156979: f64 = (var_t0 / var_tmf2);
        let assign104640_e156980: f64 = (1.0 + assign104640_e156979);
        let assign104640_e156981: f64 = (0.5 * assign104640_e156980);
        (assign104640_e156981, (0.5 * (((var_t0_dn0 * var_tmf2) - (var_t0 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t0_dn2 * var_tmf2) - (var_t0 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t0_dn4 * var_tmf2) - (var_t0 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t0_dn5 * var_tmf2) - (var_t0 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t0_dn6 * var_tmf2) - (var_t0 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t0_dn7 * var_tmf2) - (var_t0 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t0_dn8 * var_tmf2) - (var_t0 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t0_dn9 * var_tmf2) - (var_t0 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t0_dn10 * var_tmf2) - (var_t0 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t0_dn13 * var_tmf2) - (var_t0 * var_tmf2_dn13)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_dn9, var_t9_dn10, var_t9_dn13,)
    }
};
        var_t9 = assign104640_e156983;
        var_t9_dn0 = assign104640_e156983_d_n0;
        var_t9_dn2 = assign104640_e156983_d_n2;
        var_t9_dn4 = assign104640_e156983_d_n4;
        var_t9_dn5 = assign104640_e156983_d_n5;
        var_t9_dn6 = assign104640_e156983_d_n6;
        var_t9_dn7 = assign104640_e156983_d_n7;
        var_t9_dn8 = assign104640_e156983_d_n8;
        var_t9_dn9 = assign104640_e156983_d_n9;
        var_t9_dn10 = assign104640_e156983_d_n10;
        var_t9_dn13 = assign104640_e156983_d_n13;

        let (assign104650_e156994, assign104650_e156994_d_n0, assign104650_e156994_d_n2, assign104650_e156994_d_n4, assign104650_e156994_d_n5, assign104650_e156994_d_n6, assign104650_e156994_d_n7, assign104650_e156994_d_n8, assign104650_e156994_d_n9, assign104650_e156994_d_n10, assign104650_e156994_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104650_e156991: f64 = (var_t0 + var_tmf2);
        let assign104650_e156992: f64 = (0.5 * assign104650_e156991);
        (assign104650_e156992, (0.5 * (var_t0_dn0 + var_tmf2_dn0)), (0.5 * (var_t0_dn2 + var_tmf2_dn2)), (0.5 * (var_t0_dn4 + var_tmf2_dn4)), (0.5 * (var_t0_dn5 + var_tmf2_dn5)), (0.5 * (var_t0_dn6 + var_tmf2_dn6)), (0.5 * (var_t0_dn7 + var_tmf2_dn7)), (0.5 * (var_t0_dn8 + var_tmf2_dn8)), (0.5 * (var_t0_dn9 + var_tmf2_dn9)), (0.5 * (var_t0_dn10 + var_tmf2_dn10)), (0.5 * (var_t0_dn13 + var_tmf2_dn13)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign104650_e156994;
        var_t0_dn0 = assign104650_e156994_d_n0;
        var_t0_dn2 = assign104650_e156994_d_n2;
        var_t0_dn4 = assign104650_e156994_d_n4;
        var_t0_dn5 = assign104650_e156994_d_n5;
        var_t0_dn6 = assign104650_e156994_d_n6;
        var_t0_dn7 = assign104650_e156994_d_n7;
        var_t0_dn8 = assign104650_e156994_d_n8;
        var_t0_dn9 = assign104650_e156994_d_n9;
        var_t0_dn10 = assign104650_e156994_d_n10;
        var_t0_dn13 = assign104650_e156994_d_n13;

        let assign104660_e156997: f64 = if var_t0 < 0.0 { 1.0 } else { 0.0 };
        var_guard2378 = assign104660_e156997;

        let (assign104670_e157006, assign104670_e157006_d_n0, assign104670_e157006_d_n2, assign104670_e157006_d_n4, assign104670_e157006_d_n5, assign104670_e157006_d_n6, assign104670_e157006_d_n7, assign104670_e157006_d_n8, assign104670_e157006_d_n9, assign104670_e157006_d_n10, assign104670_e157006_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2378 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign104670_e157006;
        var_t0_dn0 = assign104670_e157006_d_n0;
        var_t0_dn2 = assign104670_e157006_d_n2;
        var_t0_dn4 = assign104670_e157006_d_n4;
        var_t0_dn5 = assign104670_e157006_d_n5;
        var_t0_dn6 = assign104670_e157006_d_n6;
        var_t0_dn7 = assign104670_e157006_d_n7;
        var_t0_dn8 = assign104670_e157006_d_n8;
        var_t0_dn9 = assign104670_e157006_d_n9;
        var_t0_dn10 = assign104670_e157006_d_n10;
        var_t0_dn13 = assign104670_e157006_d_n13;

        let (assign104680_e157015, assign104680_e157015_d_n0, assign104680_e157015_d_n2, assign104680_e157015_d_n4, assign104680_e157015_d_n5, assign104680_e157015_d_n6, assign104680_e157015_d_n7, assign104680_e157015_d_n8, assign104680_e157015_d_n9, assign104680_e157015_d_n10, assign104680_e157015_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2378 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_dn9, var_t9_dn10, var_t9_dn13,)
    }
};
        var_t9 = assign104680_e157015;
        var_t9_dn0 = assign104680_e157015_d_n0;
        var_t9_dn2 = assign104680_e157015_d_n2;
        var_t9_dn4 = assign104680_e157015_d_n4;
        var_t9_dn5 = assign104680_e157015_d_n5;
        var_t9_dn6 = assign104680_e157015_d_n6;
        var_t9_dn7 = assign104680_e157015_d_n7;
        var_t9_dn8 = assign104680_e157015_d_n8;
        var_t9_dn9 = assign104680_e157015_d_n9;
        var_t9_dn10 = assign104680_e157015_d_n10;
        var_t9_dn13 = assign104680_e157015_d_n13;

        let (assign104690_e157026, assign104690_e157026_d_n0, assign104690_e157026_d_n2, assign104690_e157026_d_n4, assign104690_e157026_d_n5, assign104690_e157026_d_n6, assign104690_e157026_d_n7, assign104690_e157026_d_n8, assign104690_e157026_d_n9, assign104690_e157026_d_n10, assign104690_e157026_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104690_e157023: f64 = (10.0 * 2.220446049250313e-16);
        let assign104690_e157024: f64 = (var_t0 + assign104690_e157023);
        (assign104690_e157024, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign104690_e157026;
        var_t0_dn0 = assign104690_e157026_d_n0;
        var_t0_dn2 = assign104690_e157026_d_n2;
        var_t0_dn4 = assign104690_e157026_d_n4;
        var_t0_dn5 = assign104690_e157026_d_n5;
        var_t0_dn6 = assign104690_e157026_d_n6;
        var_t0_dn7 = assign104690_e157026_d_n7;
        var_t0_dn8 = assign104690_e157026_d_n8;
        var_t0_dn9 = assign104690_e157026_d_n9;
        var_t0_dn10 = assign104690_e157026_d_n10;
        var_t0_dn13 = assign104690_e157026_d_n13;

        let (assign104700_e157036, assign104700_e157036_d_n0, assign104700_e157036_d_n2, assign104700_e157036_d_n4, assign104700_e157036_d_n5, assign104700_e157036_d_n6, assign104700_e157036_d_n7, assign104700_e157036_d_n8, assign104700_e157036_d_n9, assign104700_e157036_d_n10, assign104700_e157036_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104700_e157033: f64 = (var_kdep * var_t0);
        let assign104700_e157034: f64 = (assign104700_e157033).sqrt();
        (assign104700_e157034, ((var_kdep * var_t0_dn0) / (2.0 * assign104700_e157034)), ((var_kdep * var_t0_dn2) / (2.0 * assign104700_e157034)), ((var_kdep * var_t0_dn4) / (2.0 * assign104700_e157034)), ((var_kdep * var_t0_dn5) / (2.0 * assign104700_e157034)), ((var_kdep * var_t0_dn6) / (2.0 * assign104700_e157034)), ((var_kdep * var_t0_dn7) / (2.0 * assign104700_e157034)), ((var_kdep * var_t0_dn8) / (2.0 * assign104700_e157034)), ((var_kdep * var_t0_dn9) / (2.0 * assign104700_e157034)), ((var_kdep * var_t0_dn10) / (2.0 * assign104700_e157034)), ((var_kdep * var_t0_dn13) / (2.0 * assign104700_e157034)),)
    } else {
        (var_wdepl, var_wdepl_dn0, var_wdepl_dn2, var_wdepl_dn4, var_wdepl_dn5, var_wdepl_dn6, var_wdepl_dn7, var_wdepl_dn8, var_wdepl_dn9, var_wdepl_dn10, var_wdepl_dn13,)
    }
};
        var_wdepl = assign104700_e157036;
        var_wdepl_dn0 = assign104700_e157036_d_n0;
        var_wdepl_dn2 = assign104700_e157036_d_n2;
        var_wdepl_dn4 = assign104700_e157036_d_n4;
        var_wdepl_dn5 = assign104700_e157036_d_n5;
        var_wdepl_dn6 = assign104700_e157036_d_n6;
        var_wdepl_dn7 = assign104700_e157036_d_n7;
        var_wdepl_dn8 = assign104700_e157036_d_n8;
        var_wdepl_dn9 = assign104700_e157036_d_n9;
        var_wdepl_dn10 = assign104700_e157036_d_n10;
        var_wdepl_dn13 = assign104700_e157036_d_n13;

        let (assign104710_e157047, assign104710_e157047_d_n0, assign104710_e157047_d_n2, assign104710_e157047_d_n4, assign104710_e157047_d_n5, assign104710_e157047_d_n6, assign104710_e157047_d_n7, assign104710_e157047_d_n8, assign104710_e157047_d_n9, assign104710_e157047_d_n10, assign104710_e157047_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104710_e157043: f64 = (var_vds__blk2352 - var_vbs__blk2353);
        let assign104710_e157045: f64 = (assign104710_e157043 + p.p137);
        (assign104710_e157045, 0.0, 0.0, 0.0, var_vds__blk2352_dn5, 0.0, (var_vds__blk2352_dn7 - var_vbs__blk2353_dn7), (-var_vbs__blk2353_dn8), 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign104710_e157047;
        var_t2_dn0 = assign104710_e157047_d_n0;
        var_t2_dn2 = assign104710_e157047_d_n2;
        var_t2_dn4 = assign104710_e157047_d_n4;
        var_t2_dn5 = assign104710_e157047_d_n5;
        var_t2_dn6 = assign104710_e157047_d_n6;
        var_t2_dn7 = assign104710_e157047_d_n7;
        var_t2_dn8 = assign104710_e157047_d_n8;
        var_t2_dn9 = assign104710_e157047_d_n9;
        var_t2_dn10 = assign104710_e157047_d_n10;
        var_t2_dn13 = assign104710_e157047_d_n13;

        let (assign104720_e157063, assign104720_e157063_d_n0, assign104720_e157063_d_n2, assign104720_e157063_d_n4, assign104720_e157063_d_n5, assign104720_e157063_d_n6, assign104720_e157063_d_n7, assign104720_e157063_d_n8, assign104720_e157063_d_n9, assign104720_e157063_d_n10, assign104720_e157063_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104720_e157054: f64 = (var_t2 * var_t2);
        let assign104720_e157057: f64 = (4.0 * 0.01);
        let assign104720_e157059: f64 = (assign104720_e157057 * 0.01);
        let assign104720_e157060: f64 = (assign104720_e157054 + assign104720_e157059);
        let assign104720_e157061: f64 = (assign104720_e157060).sqrt();
        (assign104720_e157061, (((var_t2_dn0 * var_t2) + (var_t2 * var_t2_dn0)) / (2.0 * assign104720_e157061)), (((var_t2_dn2 * var_t2) + (var_t2 * var_t2_dn2)) / (2.0 * assign104720_e157061)), (((var_t2_dn4 * var_t2) + (var_t2 * var_t2_dn4)) / (2.0 * assign104720_e157061)), (((var_t2_dn5 * var_t2) + (var_t2 * var_t2_dn5)) / (2.0 * assign104720_e157061)), (((var_t2_dn6 * var_t2) + (var_t2 * var_t2_dn6)) / (2.0 * assign104720_e157061)), (((var_t2_dn7 * var_t2) + (var_t2 * var_t2_dn7)) / (2.0 * assign104720_e157061)), (((var_t2_dn8 * var_t2) + (var_t2 * var_t2_dn8)) / (2.0 * assign104720_e157061)), (((var_t2_dn9 * var_t2) + (var_t2 * var_t2_dn9)) / (2.0 * assign104720_e157061)), (((var_t2_dn10 * var_t2) + (var_t2 * var_t2_dn10)) / (2.0 * assign104720_e157061)), (((var_t2_dn13 * var_t2) + (var_t2 * var_t2_dn13)) / (2.0 * assign104720_e157061)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign104720_e157063;
        var_tmf2_dn0 = assign104720_e157063_d_n0;
        var_tmf2_dn2 = assign104720_e157063_d_n2;
        var_tmf2_dn4 = assign104720_e157063_d_n4;
        var_tmf2_dn5 = assign104720_e157063_d_n5;
        var_tmf2_dn6 = assign104720_e157063_d_n6;
        var_tmf2_dn7 = assign104720_e157063_d_n7;
        var_tmf2_dn8 = assign104720_e157063_d_n8;
        var_tmf2_dn9 = assign104720_e157063_d_n9;
        var_tmf2_dn10 = assign104720_e157063_d_n10;
        var_tmf2_dn13 = assign104720_e157063_d_n13;

        let (assign104730_e157076, assign104730_e157076_d_n0, assign104730_e157076_d_n2, assign104730_e157076_d_n4, assign104730_e157076_d_n5, assign104730_e157076_d_n6, assign104730_e157076_d_n7, assign104730_e157076_d_n8, assign104730_e157076_d_n9, assign104730_e157076_d_n10, assign104730_e157076_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104730_e157072: f64 = (var_t2 / var_tmf2);
        let assign104730_e157073: f64 = (1.0 + assign104730_e157072);
        let assign104730_e157074: f64 = (0.5 * assign104730_e157073);
        (assign104730_e157074, (0.5 * (((var_t2_dn0 * var_tmf2) - (var_t2 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t2_dn2 * var_tmf2) - (var_t2 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t2_dn4 * var_tmf2) - (var_t2 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t2_dn5 * var_tmf2) - (var_t2 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t2_dn6 * var_tmf2) - (var_t2 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t2_dn7 * var_tmf2) - (var_t2 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t2_dn8 * var_tmf2) - (var_t2 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t2_dn9 * var_tmf2) - (var_t2 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t2_dn10 * var_tmf2) - (var_t2 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t2_dn13 * var_tmf2) - (var_t2 * var_tmf2_dn13)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_dn9, var_t9_dn10, var_t9_dn13,)
    }
};
        var_t9 = assign104730_e157076;
        var_t9_dn0 = assign104730_e157076_d_n0;
        var_t9_dn2 = assign104730_e157076_d_n2;
        var_t9_dn4 = assign104730_e157076_d_n4;
        var_t9_dn5 = assign104730_e157076_d_n5;
        var_t9_dn6 = assign104730_e157076_d_n6;
        var_t9_dn7 = assign104730_e157076_d_n7;
        var_t9_dn8 = assign104730_e157076_d_n8;
        var_t9_dn9 = assign104730_e157076_d_n9;
        var_t9_dn10 = assign104730_e157076_d_n10;
        var_t9_dn13 = assign104730_e157076_d_n13;

        let (assign104740_e157087, assign104740_e157087_d_n0, assign104740_e157087_d_n2, assign104740_e157087_d_n4, assign104740_e157087_d_n5, assign104740_e157087_d_n6, assign104740_e157087_d_n7, assign104740_e157087_d_n8, assign104740_e157087_d_n9, assign104740_e157087_d_n10, assign104740_e157087_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104740_e157084: f64 = (var_t2 + var_tmf2);
        let assign104740_e157085: f64 = (0.5 * assign104740_e157084);
        (assign104740_e157085, (0.5 * (var_t2_dn0 + var_tmf2_dn0)), (0.5 * (var_t2_dn2 + var_tmf2_dn2)), (0.5 * (var_t2_dn4 + var_tmf2_dn4)), (0.5 * (var_t2_dn5 + var_tmf2_dn5)), (0.5 * (var_t2_dn6 + var_tmf2_dn6)), (0.5 * (var_t2_dn7 + var_tmf2_dn7)), (0.5 * (var_t2_dn8 + var_tmf2_dn8)), (0.5 * (var_t2_dn9 + var_tmf2_dn9)), (0.5 * (var_t2_dn10 + var_tmf2_dn10)), (0.5 * (var_t2_dn13 + var_tmf2_dn13)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign104740_e157087;
        var_t2_dn0 = assign104740_e157087_d_n0;
        var_t2_dn2 = assign104740_e157087_d_n2;
        var_t2_dn4 = assign104740_e157087_d_n4;
        var_t2_dn5 = assign104740_e157087_d_n5;
        var_t2_dn6 = assign104740_e157087_d_n6;
        var_t2_dn7 = assign104740_e157087_d_n7;
        var_t2_dn8 = assign104740_e157087_d_n8;
        var_t2_dn9 = assign104740_e157087_d_n9;
        var_t2_dn10 = assign104740_e157087_d_n10;
        var_t2_dn13 = assign104740_e157087_d_n13;

        let assign104750_e157090: f64 = if var_t2 < 0.0 { 1.0 } else { 0.0 };
        var_guard2379 = assign104750_e157090;

        let (assign104760_e157099, assign104760_e157099_d_n0, assign104760_e157099_d_n2, assign104760_e157099_d_n4, assign104760_e157099_d_n5, assign104760_e157099_d_n6, assign104760_e157099_d_n7, assign104760_e157099_d_n8, assign104760_e157099_d_n9, assign104760_e157099_d_n10, assign104760_e157099_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2379 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign104760_e157099;
        var_t2_dn0 = assign104760_e157099_d_n0;
        var_t2_dn2 = assign104760_e157099_d_n2;
        var_t2_dn4 = assign104760_e157099_d_n4;
        var_t2_dn5 = assign104760_e157099_d_n5;
        var_t2_dn6 = assign104760_e157099_d_n6;
        var_t2_dn7 = assign104760_e157099_d_n7;
        var_t2_dn8 = assign104760_e157099_d_n8;
        var_t2_dn9 = assign104760_e157099_d_n9;
        var_t2_dn10 = assign104760_e157099_d_n10;
        var_t2_dn13 = assign104760_e157099_d_n13;

        let (assign104770_e157108, assign104770_e157108_d_n0, assign104770_e157108_d_n2, assign104770_e157108_d_n4, assign104770_e157108_d_n5, assign104770_e157108_d_n6, assign104770_e157108_d_n7, assign104770_e157108_d_n8, assign104770_e157108_d_n9, assign104770_e157108_d_n10, assign104770_e157108_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2379 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_dn9, var_t9_dn10, var_t9_dn13,)
    }
};
        var_t9 = assign104770_e157108;
        var_t9_dn0 = assign104770_e157108_d_n0;
        var_t9_dn2 = assign104770_e157108_d_n2;
        var_t9_dn4 = assign104770_e157108_d_n4;
        var_t9_dn5 = assign104770_e157108_d_n5;
        var_t9_dn6 = assign104770_e157108_d_n6;
        var_t9_dn7 = assign104770_e157108_d_n7;
        var_t9_dn8 = assign104770_e157108_d_n8;
        var_t9_dn9 = assign104770_e157108_d_n9;
        var_t9_dn10 = assign104770_e157108_d_n10;
        var_t9_dn13 = assign104770_e157108_d_n13;

        let (assign104780_e157119, assign104780_e157119_d_n0, assign104780_e157119_d_n2, assign104780_e157119_d_n4, assign104780_e157119_d_n5, assign104780_e157119_d_n6, assign104780_e157119_d_n7, assign104780_e157119_d_n8, assign104780_e157119_d_n9, assign104780_e157119_d_n10, assign104780_e157119_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104780_e157116: f64 = (10.0 * 2.220446049250313e-16);
        let assign104780_e157117: f64 = (var_t2 + assign104780_e157116);
        (assign104780_e157117, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign104780_e157119;
        var_t2_dn0 = assign104780_e157119_d_n0;
        var_t2_dn2 = assign104780_e157119_d_n2;
        var_t2_dn4 = assign104780_e157119_d_n4;
        var_t2_dn5 = assign104780_e157119_d_n5;
        var_t2_dn6 = assign104780_e157119_d_n6;
        var_t2_dn7 = assign104780_e157119_d_n7;
        var_t2_dn8 = assign104780_e157119_d_n8;
        var_t2_dn9 = assign104780_e157119_d_n9;
        var_t2_dn10 = assign104780_e157119_d_n10;
        var_t2_dn13 = assign104780_e157119_d_n13;

        let (assign104790_e157129, assign104790_e157129_d_n0, assign104790_e157129_d_n2, assign104790_e157129_d_n4, assign104790_e157129_d_n5, assign104790_e157129_d_n6, assign104790_e157129_d_n7, assign104790_e157129_d_n8, assign104790_e157129_d_n9, assign104790_e157129_d_n10, assign104790_e157129_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104790_e157126: f64 = (var_kjunc * var_t2);
        let assign104790_e157127: f64 = (assign104790_e157126).sqrt();
        (assign104790_e157127, (((var_kjunc_dn0 * var_t2) + (var_kjunc * var_t2_dn0)) / (2.0 * assign104790_e157127)), (((var_kjunc_dn2 * var_t2) + (var_kjunc * var_t2_dn2)) / (2.0 * assign104790_e157127)), (((var_kjunc_dn4 * var_t2) + (var_kjunc * var_t2_dn4)) / (2.0 * assign104790_e157127)), (((var_kjunc_dn5 * var_t2) + (var_kjunc * var_t2_dn5)) / (2.0 * assign104790_e157127)), (((var_kjunc_dn6 * var_t2) + (var_kjunc * var_t2_dn6)) / (2.0 * assign104790_e157127)), (((var_kjunc_dn7 * var_t2) + (var_kjunc * var_t2_dn7)) / (2.0 * assign104790_e157127)), (((var_kjunc_dn8 * var_t2) + (var_kjunc * var_t2_dn8)) / (2.0 * assign104790_e157127)), (((var_kjunc_dn9 * var_t2) + (var_kjunc * var_t2_dn9)) / (2.0 * assign104790_e157127)), (((var_kjunc_dn10 * var_t2) + (var_kjunc * var_t2_dn10)) / (2.0 * assign104790_e157127)), (((var_kjunc_dn13 * var_t2) + (var_kjunc * var_t2_dn13)) / (2.0 * assign104790_e157127)),)
    } else {
        (var_wjunc0, var_wjunc0_dn0, var_wjunc0_dn2, var_wjunc0_dn4, var_wjunc0_dn5, var_wjunc0_dn6, var_wjunc0_dn7, var_wjunc0_dn8, var_wjunc0_dn9, var_wjunc0_dn10, var_wjunc0_dn13,)
    }
};
        var_wjunc0 = assign104790_e157129;
        var_wjunc0_dn0 = assign104790_e157129_d_n0;
        var_wjunc0_dn2 = assign104790_e157129_d_n2;
        var_wjunc0_dn4 = assign104790_e157129_d_n4;
        var_wjunc0_dn5 = assign104790_e157129_d_n5;
        var_wjunc0_dn6 = assign104790_e157129_d_n6;
        var_wjunc0_dn7 = assign104790_e157129_d_n7;
        var_wjunc0_dn8 = assign104790_e157129_d_n8;
        var_wjunc0_dn9 = assign104790_e157129_d_n9;
        var_wjunc0_dn10 = assign104790_e157129_d_n10;
        var_wjunc0_dn13 = assign104790_e157129_d_n13;

        let (assign104800_e157142, assign104800_e157142_d_n0, assign104800_e157142_d_n2, assign104800_e157142_d_n4, assign104800_e157142_d_n5, assign104800_e157142_d_n6, assign104800_e157142_d_n7, assign104800_e157142_d_n8, assign104800_e157142_d_n9, assign104800_e157142_d_n10, assign104800_e157142_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104800_e157136: f64 = (var_rd_xldld - var_wjunc0);
        let assign104800_e157139: f64 = (0.01 * var_rd_xldld);
        let assign104800_e157140: f64 = (assign104800_e157136 - assign104800_e157139);
        (assign104800_e157140, (-var_wjunc0_dn0), (-var_wjunc0_dn2), (-var_wjunc0_dn4), (-var_wjunc0_dn5), (-var_wjunc0_dn6), (-var_wjunc0_dn7), (-var_wjunc0_dn8), (-var_wjunc0_dn9), (-var_wjunc0_dn10), (-var_wjunc0_dn13),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn13,)
    }
};
        var_tmf1 = assign104800_e157142;
        var_tmf1_dn0 = assign104800_e157142_d_n0;
        var_tmf1_dn2 = assign104800_e157142_d_n2;
        var_tmf1_dn4 = assign104800_e157142_d_n4;
        var_tmf1_dn5 = assign104800_e157142_d_n5;
        var_tmf1_dn6 = assign104800_e157142_d_n6;
        var_tmf1_dn7 = assign104800_e157142_d_n7;
        var_tmf1_dn8 = assign104800_e157142_d_n8;
        var_tmf1_dn9 = assign104800_e157142_d_n9;
        var_tmf1_dn10 = assign104800_e157142_d_n10;
        var_tmf1_dn13 = assign104800_e157142_d_n13;

        let (assign104810_e157155, assign104810_e157155_d_n0, assign104810_e157155_d_n2, assign104810_e157155_d_n4, assign104810_e157155_d_n5, assign104810_e157155_d_n6, assign104810_e157155_d_n7, assign104810_e157155_d_n8, assign104810_e157155_d_n9, assign104810_e157155_d_n10, assign104810_e157155_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104810_e157149: f64 = (4.0 * var_rd_xldld);
        let assign104810_e157152: f64 = (0.01 * var_rd_xldld);
        let assign104810_e157153: f64 = (assign104810_e157149 * assign104810_e157152);
        (assign104810_e157153, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign104810_e157155;
        var_tmf2_dn0 = assign104810_e157155_d_n0;
        var_tmf2_dn2 = assign104810_e157155_d_n2;
        var_tmf2_dn4 = assign104810_e157155_d_n4;
        var_tmf2_dn5 = assign104810_e157155_d_n5;
        var_tmf2_dn6 = assign104810_e157155_d_n6;
        var_tmf2_dn7 = assign104810_e157155_d_n7;
        var_tmf2_dn8 = assign104810_e157155_d_n8;
        var_tmf2_dn9 = assign104810_e157155_d_n9;
        var_tmf2_dn10 = assign104810_e157155_d_n10;
        var_tmf2_dn13 = assign104810_e157155_d_n13;

        let (assign104820_e157168, assign104820_e157168_d_n0, assign104820_e157168_d_n2, assign104820_e157168_d_n4, assign104820_e157168_d_n5, assign104820_e157168_d_n6, assign104820_e157168_d_n7, assign104820_e157168_d_n8, assign104820_e157168_d_n9, assign104820_e157168_d_n10, assign104820_e157168_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let (assign104820_e157166, assign104820_e157166_d_n0, assign104820_e157166_d_n2, assign104820_e157166_d_n4, assign104820_e157166_d_n5, assign104820_e157166_d_n6, assign104820_e157166_d_n7, assign104820_e157166_d_n8, assign104820_e157166_d_n9, assign104820_e157166_d_n10, assign104820_e157166_d_n13,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
            } else {
                let assign104820_e157165: f64 = (-var_tmf2);
                (assign104820_e157165, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn13),)
            }
        };
        (assign104820_e157166, assign104820_e157166_d_n0, assign104820_e157166_d_n2, assign104820_e157166_d_n4, assign104820_e157166_d_n5, assign104820_e157166_d_n6, assign104820_e157166_d_n7, assign104820_e157166_d_n8, assign104820_e157166_d_n9, assign104820_e157166_d_n10, assign104820_e157166_d_n13,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign104820_e157168;
        var_tmf2_dn0 = assign104820_e157168_d_n0;
        var_tmf2_dn2 = assign104820_e157168_d_n2;
        var_tmf2_dn4 = assign104820_e157168_d_n4;
        var_tmf2_dn5 = assign104820_e157168_d_n5;
        var_tmf2_dn6 = assign104820_e157168_d_n6;
        var_tmf2_dn7 = assign104820_e157168_d_n7;
        var_tmf2_dn8 = assign104820_e157168_d_n8;
        var_tmf2_dn9 = assign104820_e157168_d_n9;
        var_tmf2_dn10 = assign104820_e157168_d_n10;
        var_tmf2_dn13 = assign104820_e157168_d_n13;

        let (assign104830_e157180, assign104830_e157180_d_n0, assign104830_e157180_d_n2, assign104830_e157180_d_n4, assign104830_e157180_d_n5, assign104830_e157180_d_n6, assign104830_e157180_d_n7, assign104830_e157180_d_n8, assign104830_e157180_d_n9, assign104830_e157180_d_n10, assign104830_e157180_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104830_e157175: f64 = (var_tmf1 * var_tmf1);
        let assign104830_e157177: f64 = (assign104830_e157175 + var_tmf2);
        let assign104830_e157178: f64 = (assign104830_e157177).sqrt();
        (assign104830_e157178, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign104830_e157178)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign104830_e157178)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign104830_e157178)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign104830_e157178)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign104830_e157178)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign104830_e157178)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign104830_e157178)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign104830_e157178)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign104830_e157178)), ((((var_tmf1_dn13 * var_tmf1) + (var_tmf1 * var_tmf1_dn13)) + var_tmf2_dn13) / (2.0 * assign104830_e157178)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign104830_e157180;
        var_tmf2_dn0 = assign104830_e157180_d_n0;
        var_tmf2_dn2 = assign104830_e157180_d_n2;
        var_tmf2_dn4 = assign104830_e157180_d_n4;
        var_tmf2_dn5 = assign104830_e157180_d_n5;
        var_tmf2_dn6 = assign104830_e157180_d_n6;
        var_tmf2_dn7 = assign104830_e157180_d_n7;
        var_tmf2_dn8 = assign104830_e157180_d_n8;
        var_tmf2_dn9 = assign104830_e157180_d_n9;
        var_tmf2_dn10 = assign104830_e157180_d_n10;
        var_tmf2_dn13 = assign104830_e157180_d_n13;

        let (assign104840_e157193, assign104840_e157193_d_n0, assign104840_e157193_d_n2, assign104840_e157193_d_n4, assign104840_e157193_d_n5, assign104840_e157193_d_n6, assign104840_e157193_d_n7, assign104840_e157193_d_n8, assign104840_e157193_d_n9, assign104840_e157193_d_n10, assign104840_e157193_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104840_e157189: f64 = (var_tmf1 / var_tmf2);
        let assign104840_e157190: f64 = (1.0 + assign104840_e157189);
        let assign104840_e157191: f64 = (0.5 * assign104840_e157190);
        (assign104840_e157191, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn13 * var_tmf2) - (var_tmf1 * var_tmf2_dn13)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign104840_e157193;
        var_t0_dn0 = assign104840_e157193_d_n0;
        var_t0_dn2 = assign104840_e157193_d_n2;
        var_t0_dn4 = assign104840_e157193_d_n4;
        var_t0_dn5 = assign104840_e157193_d_n5;
        var_t0_dn6 = assign104840_e157193_d_n6;
        var_t0_dn7 = assign104840_e157193_d_n7;
        var_t0_dn8 = assign104840_e157193_d_n8;
        var_t0_dn9 = assign104840_e157193_d_n9;
        var_t0_dn10 = assign104840_e157193_d_n10;
        var_t0_dn13 = assign104840_e157193_d_n13;

        let (assign104850_e157206, assign104850_e157206_d_n0, assign104850_e157206_d_n2, assign104850_e157206_d_n4, assign104850_e157206_d_n5, assign104850_e157206_d_n6, assign104850_e157206_d_n7, assign104850_e157206_d_n8, assign104850_e157206_d_n9, assign104850_e157206_d_n10, assign104850_e157206_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104850_e157202: f64 = (var_tmf1 + var_tmf2);
        let assign104850_e157203: f64 = (0.5 * assign104850_e157202);
        let assign104850_e157204: f64 = (var_rd_xldld - assign104850_e157203);
        (assign104850_e157204, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (-(0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (-(0.5 * (var_tmf1_dn8 + var_tmf2_dn8))), (-(0.5 * (var_tmf1_dn9 + var_tmf2_dn9))), (-(0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (-(0.5 * (var_tmf1_dn13 + var_tmf2_dn13))),)
    } else {
        (var_wjunc, var_wjunc_dn0, var_wjunc_dn2, var_wjunc_dn4, var_wjunc_dn5, var_wjunc_dn6, var_wjunc_dn7, var_wjunc_dn8, var_wjunc_dn9, var_wjunc_dn10, var_wjunc_dn13,)
    }
};
        var_wjunc = assign104850_e157206;
        var_wjunc_dn0 = assign104850_e157206_d_n0;
        var_wjunc_dn2 = assign104850_e157206_d_n2;
        var_wjunc_dn4 = assign104850_e157206_d_n4;
        var_wjunc_dn5 = assign104850_e157206_d_n5;
        var_wjunc_dn6 = assign104850_e157206_d_n6;
        var_wjunc_dn7 = assign104850_e157206_d_n7;
        var_wjunc_dn8 = assign104850_e157206_d_n8;
        var_wjunc_dn9 = assign104850_e157206_d_n9;
        var_wjunc_dn10 = assign104850_e157206_d_n10;
        var_wjunc_dn13 = assign104850_e157206_d_n13;

        let (assign104860_e157215,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104860_e157213: f64 = (p.p419 + 1e-25);
        (assign104860_e157213,)
    } else {
        (var_wrdrdjunc,)
    }
};
        var_wrdrdjunc = assign104860_e157215;

        let (assign104870_e157234, assign104870_e157234_d_n0, assign104870_e157234_d_n2, assign104870_e157234_d_n4, assign104870_e157234_d_n5, assign104870_e157234_d_n6, assign104870_e157234_d_n7, assign104870_e157234_d_n8, assign104870_e157234_d_n9, assign104870_e157234_d_n10, assign104870_e157234_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104870_e157225: f64 = (var_wdepl / var_wrdrdjunc);
        let assign104870_e157228: f64 = (var_wjunc / var_rd_xldld);
        let assign104870_e157229: f64 = (assign104870_e157225 + assign104870_e157228);
        let assign104870_e157230: f64 = (var_cx * assign104870_e157229);
        let assign104870_e157231: f64 = (1.0 - assign104870_e157230);
        let assign104870_e157232: f64 = (var_xmax * assign104870_e157231);
        (assign104870_e157232, (var_xmax * (-(var_cx * ((var_wdepl_dn0 / var_wrdrdjunc) + (var_wjunc_dn0 / var_rd_xldld))))), (var_xmax * (-(var_cx * ((var_wdepl_dn2 / var_wrdrdjunc) + (var_wjunc_dn2 / var_rd_xldld))))), (var_xmax * (-(var_cx * ((var_wdepl_dn4 / var_wrdrdjunc) + (var_wjunc_dn4 / var_rd_xldld))))), (var_xmax * (-(var_cx * ((var_wdepl_dn5 / var_wrdrdjunc) + (var_wjunc_dn5 / var_rd_xldld))))), (var_xmax * (-(var_cx * ((var_wdepl_dn6 / var_wrdrdjunc) + (var_wjunc_dn6 / var_rd_xldld))))), (var_xmax * (-(var_cx * ((var_wdepl_dn7 / var_wrdrdjunc) + (var_wjunc_dn7 / var_rd_xldld))))), (var_xmax * (-(var_cx * ((var_wdepl_dn8 / var_wrdrdjunc) + (var_wjunc_dn8 / var_rd_xldld))))), (var_xmax * (-(var_cx * ((var_wdepl_dn9 / var_wrdrdjunc) + (var_wjunc_dn9 / var_rd_xldld))))), (var_xmax * (-(var_cx * ((var_wdepl_dn10 / var_wrdrdjunc) + (var_wjunc_dn10 / var_rd_xldld))))), (var_xmax * (-(var_cx * ((var_wdepl_dn13 / var_wrdrdjunc) + (var_wjunc_dn13 / var_rd_xldld))))),)
    } else {
        (var_xov, var_xov_dn0, var_xov_dn2, var_xov_dn4, var_xov_dn5, var_xov_dn6, var_xov_dn7, var_xov_dn8, var_xov_dn9, var_xov_dn10, var_xov_dn13,)
    }
};
        var_xov = assign104870_e157234;
        var_xov_dn0 = assign104870_e157234_d_n0;
        var_xov_dn2 = assign104870_e157234_d_n2;
        var_xov_dn4 = assign104870_e157234_d_n4;
        var_xov_dn5 = assign104870_e157234_d_n5;
        var_xov_dn6 = assign104870_e157234_d_n6;
        var_xov_dn7 = assign104870_e157234_d_n7;
        var_xov_dn8 = assign104870_e157234_d_n8;
        var_xov_dn9 = assign104870_e157234_d_n9;
        var_xov_dn10 = assign104870_e157234_d_n10;
        var_xov_dn13 = assign104870_e157234_d_n13;

        *var_guard2378_slot = var_guard2378;
        *var_guard2379_slot = var_guard2379;
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
        *var_wdepl_slot = var_wdepl;
        *var_wdepl_dn0_slot = var_wdepl_dn0;
        *var_wdepl_dn10_slot = var_wdepl_dn10;
        *var_wdepl_dn13_slot = var_wdepl_dn13;
        *var_wdepl_dn2_slot = var_wdepl_dn2;
        *var_wdepl_dn4_slot = var_wdepl_dn4;
        *var_wdepl_dn5_slot = var_wdepl_dn5;
        *var_wdepl_dn6_slot = var_wdepl_dn6;
        *var_wdepl_dn7_slot = var_wdepl_dn7;
        *var_wdepl_dn8_slot = var_wdepl_dn8;
        *var_wdepl_dn9_slot = var_wdepl_dn9;
        *var_wjunc_slot = var_wjunc;
        *var_wjunc0_slot = var_wjunc0;
        *var_wjunc0_dn0_slot = var_wjunc0_dn0;
        *var_wjunc0_dn10_slot = var_wjunc0_dn10;
        *var_wjunc0_dn13_slot = var_wjunc0_dn13;
        *var_wjunc0_dn2_slot = var_wjunc0_dn2;
        *var_wjunc0_dn4_slot = var_wjunc0_dn4;
        *var_wjunc0_dn5_slot = var_wjunc0_dn5;
        *var_wjunc0_dn6_slot = var_wjunc0_dn6;
        *var_wjunc0_dn7_slot = var_wjunc0_dn7;
        *var_wjunc0_dn8_slot = var_wjunc0_dn8;
        *var_wjunc0_dn9_slot = var_wjunc0_dn9;
        *var_wjunc_dn0_slot = var_wjunc_dn0;
        *var_wjunc_dn10_slot = var_wjunc_dn10;
        *var_wjunc_dn13_slot = var_wjunc_dn13;
        *var_wjunc_dn2_slot = var_wjunc_dn2;
        *var_wjunc_dn4_slot = var_wjunc_dn4;
        *var_wjunc_dn5_slot = var_wjunc_dn5;
        *var_wjunc_dn6_slot = var_wjunc_dn6;
        *var_wjunc_dn7_slot = var_wjunc_dn7;
        *var_wjunc_dn8_slot = var_wjunc_dn8;
        *var_wjunc_dn9_slot = var_wjunc_dn9;
        *var_wrdrdjunc_slot = var_wrdrdjunc;
        *var_xov_slot = var_xov;
        *var_xov_dn0_slot = var_xov_dn0;
        *var_xov_dn10_slot = var_xov_dn10;
        *var_xov_dn13_slot = var_xov_dn13;
        *var_xov_dn2_slot = var_xov_dn2;
        *var_xov_dn4_slot = var_xov_dn4;
        *var_xov_dn5_slot = var_xov_dn5;
        *var_xov_dn6_slot = var_xov_dn6;
        *var_xov_dn7_slot = var_xov_dn7;
        *var_xov_dn8_slot = var_xov_dn8;
        *var_xov_dn9_slot = var_xov_dn9;
    }

    pub(super) fn stamp_transient_block_371(
        p: &Parameters,
        var_carr: f64,
        var_carr_dn0: f64,
        var_carr_dn10: f64,
        var_carr_dn13: f64,
        var_carr_dn2: f64,
        var_carr_dn4: f64,
        var_carr_dn5: f64,
        var_carr_dn6: f64,
        var_carr_dn7: f64,
        var_carr_dn8: f64,
        var_carr_dn9: f64,
        var_guard2336: f64,
        var_guard2356: f64,
        var_ldrifte: f64,
        var_mu__blk2354: f64,
        var_mu__blk2354_dn0: f64,
        var_mu__blk2354_dn10: f64,
        var_mu__blk2354_dn13: f64,
        var_mu__blk2354_dn2: f64,
        var_mu__blk2354_dn4: f64,
        var_mu__blk2354_dn5: f64,
        var_mu__blk2354_dn6: f64,
        var_mu__blk2354_dn7: f64,
        var_mu__blk2354_dn8: f64,
        var_mu__blk2354_dn9: f64,
        var_uc_rdrcx: f64,
        var_xmax: f64,
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
        var_gd_slot: &mut f64,
        var_gd_dn0_slot: &mut f64,
        var_gd_dn10_slot: &mut f64,
        var_gd_dn13_slot: &mut f64,
        var_gd_dn2_slot: &mut f64,
        var_gd_dn4_slot: &mut f64,
        var_gd_dn5_slot: &mut f64,
        var_gd_dn6_slot: &mut f64,
        var_gd_dn7_slot: &mut f64,
        var_gd_dn8_slot: &mut f64,
        var_gd_dn9_slot: &mut f64,
        var_guard2380_slot: &mut f64,
        var_guard2381_slot: &mut f64,
        var_guard2382_slot: &mut f64,
        var_guard2383_slot: &mut f64,
        var_guard2384_slot: &mut f64,
        var_guard2385_slot: &mut f64,
        var_guard2386_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_mm_slot: &mut f64,
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
        var_xov_slot: &mut f64,
        var_xov_dn0_slot: &mut f64,
        var_xov_dn10_slot: &mut f64,
        var_xov_dn13_slot: &mut f64,
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
        var_xp_dn13_slot: &mut f64,
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
        let mut var_arg_dn13: f64 = *var_arg_dn13_slot;
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
        let mut var_dnm_dn13: f64 = *var_dnm_dn13_slot;
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
        let mut var_gd_dn13: f64 = *var_gd_dn13_slot;
        let mut var_gd_dn2: f64 = *var_gd_dn2_slot;
        let mut var_gd_dn4: f64 = *var_gd_dn4_slot;
        let mut var_gd_dn5: f64 = *var_gd_dn5_slot;
        let mut var_gd_dn6: f64 = *var_gd_dn6_slot;
        let mut var_gd_dn7: f64 = *var_gd_dn7_slot;
        let mut var_gd_dn8: f64 = *var_gd_dn8_slot;
        let mut var_gd_dn9: f64 = *var_gd_dn9_slot;
        let mut var_guard2380: f64 = *var_guard2380_slot;
        let mut var_guard2381: f64 = *var_guard2381_slot;
        let mut var_guard2382: f64 = *var_guard2382_slot;
        let mut var_guard2383: f64 = *var_guard2383_slot;
        let mut var_guard2384: f64 = *var_guard2384_slot;
        let mut var_guard2385: f64 = *var_guard2385_slot;
        let mut var_guard2386: f64 = *var_guard2386_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_mm: f64 = *var_mm_slot;
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
        let mut var_xov: f64 = *var_xov_slot;
        let mut var_xov_dn0: f64 = *var_xov_dn0_slot;
        let mut var_xov_dn10: f64 = *var_xov_dn10_slot;
        let mut var_xov_dn13: f64 = *var_xov_dn13_slot;
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
        let mut var_xp_dn13: f64 = *var_xp_dn13_slot;
        let mut var_xp_dn2: f64 = *var_xp_dn2_slot;
        let mut var_xp_dn4: f64 = *var_xp_dn4_slot;
        let mut var_xp_dn5: f64 = *var_xp_dn5_slot;
        let mut var_xp_dn6: f64 = *var_xp_dn6_slot;
        let mut var_xp_dn7: f64 = *var_xp_dn7_slot;
        let mut var_xp_dn8: f64 = *var_xp_dn8_slot;
        let mut var_xp_dn9: f64 = *var_xp_dn9_slot;

        let (assign104880_e157262, assign104880_e157262_d_n0, assign104880_e157262_d_n2, assign104880_e157262_d_n4, assign104880_e157262_d_n5, assign104880_e157262_d_n6, assign104880_e157262_d_n7, assign104880_e157262_d_n8, assign104880_e157262_d_n9, assign104880_e157262_d_n10, assign104880_e157262_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104880_e157241: f64 = (var_xov * var_xov);
        let assign104880_e157245: f64 = (1.0 - var_uc_rdrcx);
        let assign104880_e157247: f64 = (assign104880_e157245 * var_xmax);
        let assign104880_e157249: f64 = (assign104880_e157247 / 100.0);
        let assign104880_e157250: f64 = (4.0 * assign104880_e157249);
        let assign104880_e157253: f64 = (1.0 - var_uc_rdrcx);
        let assign104880_e157255: f64 = (assign104880_e157253 * var_xmax);
        let assign104880_e157257: f64 = (assign104880_e157255 / 100.0);
        let assign104880_e157258: f64 = (assign104880_e157250 * assign104880_e157257);
        let assign104880_e157259: f64 = (assign104880_e157241 + assign104880_e157258);
        let assign104880_e157260: f64 = (assign104880_e157259).sqrt();
        (assign104880_e157260, (((var_xov_dn0 * var_xov) + (var_xov * var_xov_dn0)) / (2.0 * assign104880_e157260)), (((var_xov_dn2 * var_xov) + (var_xov * var_xov_dn2)) / (2.0 * assign104880_e157260)), (((var_xov_dn4 * var_xov) + (var_xov * var_xov_dn4)) / (2.0 * assign104880_e157260)), (((var_xov_dn5 * var_xov) + (var_xov * var_xov_dn5)) / (2.0 * assign104880_e157260)), (((var_xov_dn6 * var_xov) + (var_xov * var_xov_dn6)) / (2.0 * assign104880_e157260)), (((var_xov_dn7 * var_xov) + (var_xov * var_xov_dn7)) / (2.0 * assign104880_e157260)), (((var_xov_dn8 * var_xov) + (var_xov * var_xov_dn8)) / (2.0 * assign104880_e157260)), (((var_xov_dn9 * var_xov) + (var_xov * var_xov_dn9)) / (2.0 * assign104880_e157260)), (((var_xov_dn10 * var_xov) + (var_xov * var_xov_dn10)) / (2.0 * assign104880_e157260)), (((var_xov_dn13 * var_xov) + (var_xov * var_xov_dn13)) / (2.0 * assign104880_e157260)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign104880_e157262;
        var_tmf2_dn0 = assign104880_e157262_d_n0;
        var_tmf2_dn2 = assign104880_e157262_d_n2;
        var_tmf2_dn4 = assign104880_e157262_d_n4;
        var_tmf2_dn5 = assign104880_e157262_d_n5;
        var_tmf2_dn6 = assign104880_e157262_d_n6;
        var_tmf2_dn7 = assign104880_e157262_d_n7;
        var_tmf2_dn8 = assign104880_e157262_d_n8;
        var_tmf2_dn9 = assign104880_e157262_d_n9;
        var_tmf2_dn10 = assign104880_e157262_d_n10;
        var_tmf2_dn13 = assign104880_e157262_d_n13;

        let (assign104890_e157275, assign104890_e157275_d_n0, assign104890_e157275_d_n2, assign104890_e157275_d_n4, assign104890_e157275_d_n5, assign104890_e157275_d_n6, assign104890_e157275_d_n7, assign104890_e157275_d_n8, assign104890_e157275_d_n9, assign104890_e157275_d_n10, assign104890_e157275_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104890_e157271: f64 = (var_xov / var_tmf2);
        let assign104890_e157272: f64 = (1.0 + assign104890_e157271);
        let assign104890_e157273: f64 = (0.5 * assign104890_e157272);
        (assign104890_e157273, (0.5 * (((var_xov_dn0 * var_tmf2) - (var_xov * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_xov_dn2 * var_tmf2) - (var_xov * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_xov_dn4 * var_tmf2) - (var_xov * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_xov_dn5 * var_tmf2) - (var_xov * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_xov_dn6 * var_tmf2) - (var_xov * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_xov_dn7 * var_tmf2) - (var_xov * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_xov_dn8 * var_tmf2) - (var_xov * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_xov_dn9 * var_tmf2) - (var_xov * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_xov_dn10 * var_tmf2) - (var_xov * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_xov_dn13 * var_tmf2) - (var_xov * var_tmf2_dn13)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_dn9, var_t9_dn10, var_t9_dn13,)
    }
};
        var_t9 = assign104890_e157275;
        var_t9_dn0 = assign104890_e157275_d_n0;
        var_t9_dn2 = assign104890_e157275_d_n2;
        var_t9_dn4 = assign104890_e157275_d_n4;
        var_t9_dn5 = assign104890_e157275_d_n5;
        var_t9_dn6 = assign104890_e157275_d_n6;
        var_t9_dn7 = assign104890_e157275_d_n7;
        var_t9_dn8 = assign104890_e157275_d_n8;
        var_t9_dn9 = assign104890_e157275_d_n9;
        var_t9_dn10 = assign104890_e157275_d_n10;
        var_t9_dn13 = assign104890_e157275_d_n13;

        let (assign104900_e157286, assign104900_e157286_d_n0, assign104900_e157286_d_n2, assign104900_e157286_d_n4, assign104900_e157286_d_n5, assign104900_e157286_d_n6, assign104900_e157286_d_n7, assign104900_e157286_d_n8, assign104900_e157286_d_n9, assign104900_e157286_d_n10, assign104900_e157286_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104900_e157283: f64 = (var_xov + var_tmf2);
        let assign104900_e157284: f64 = (0.5 * assign104900_e157283);
        (assign104900_e157284, (0.5 * (var_xov_dn0 + var_tmf2_dn0)), (0.5 * (var_xov_dn2 + var_tmf2_dn2)), (0.5 * (var_xov_dn4 + var_tmf2_dn4)), (0.5 * (var_xov_dn5 + var_tmf2_dn5)), (0.5 * (var_xov_dn6 + var_tmf2_dn6)), (0.5 * (var_xov_dn7 + var_tmf2_dn7)), (0.5 * (var_xov_dn8 + var_tmf2_dn8)), (0.5 * (var_xov_dn9 + var_tmf2_dn9)), (0.5 * (var_xov_dn10 + var_tmf2_dn10)), (0.5 * (var_xov_dn13 + var_tmf2_dn13)),)
    } else {
        (var_xov, var_xov_dn0, var_xov_dn2, var_xov_dn4, var_xov_dn5, var_xov_dn6, var_xov_dn7, var_xov_dn8, var_xov_dn9, var_xov_dn10, var_xov_dn13,)
    }
};
        var_xov = assign104900_e157286;
        var_xov_dn0 = assign104900_e157286_d_n0;
        var_xov_dn2 = assign104900_e157286_d_n2;
        var_xov_dn4 = assign104900_e157286_d_n4;
        var_xov_dn5 = assign104900_e157286_d_n5;
        var_xov_dn6 = assign104900_e157286_d_n6;
        var_xov_dn7 = assign104900_e157286_d_n7;
        var_xov_dn8 = assign104900_e157286_d_n8;
        var_xov_dn9 = assign104900_e157286_d_n9;
        var_xov_dn10 = assign104900_e157286_d_n10;
        var_xov_dn13 = assign104900_e157286_d_n13;

        let assign104910_e157289: f64 = if var_xov < 0.0 { 1.0 } else { 0.0 };
        var_guard2380 = assign104910_e157289;

        let (assign104920_e157298, assign104920_e157298_d_n0, assign104920_e157298_d_n2, assign104920_e157298_d_n4, assign104920_e157298_d_n5, assign104920_e157298_d_n6, assign104920_e157298_d_n7, assign104920_e157298_d_n8, assign104920_e157298_d_n9, assign104920_e157298_d_n10, assign104920_e157298_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2380 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xov, var_xov_dn0, var_xov_dn2, var_xov_dn4, var_xov_dn5, var_xov_dn6, var_xov_dn7, var_xov_dn8, var_xov_dn9, var_xov_dn10, var_xov_dn13,)
    }
};
        var_xov = assign104920_e157298;
        var_xov_dn0 = assign104920_e157298_d_n0;
        var_xov_dn2 = assign104920_e157298_d_n2;
        var_xov_dn4 = assign104920_e157298_d_n4;
        var_xov_dn5 = assign104920_e157298_d_n5;
        var_xov_dn6 = assign104920_e157298_d_n6;
        var_xov_dn7 = assign104920_e157298_d_n7;
        var_xov_dn8 = assign104920_e157298_d_n8;
        var_xov_dn9 = assign104920_e157298_d_n9;
        var_xov_dn10 = assign104920_e157298_d_n10;
        var_xov_dn13 = assign104920_e157298_d_n13;

        let (assign104930_e157307, assign104930_e157307_d_n0, assign104930_e157307_d_n2, assign104930_e157307_d_n4, assign104930_e157307_d_n5, assign104930_e157307_d_n6, assign104930_e157307_d_n7, assign104930_e157307_d_n8, assign104930_e157307_d_n9, assign104930_e157307_d_n10, assign104930_e157307_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2380 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_dn9, var_t9_dn10, var_t9_dn13,)
    }
};
        var_t9 = assign104930_e157307;
        var_t9_dn0 = assign104930_e157307_d_n0;
        var_t9_dn2 = assign104930_e157307_d_n2;
        var_t9_dn4 = assign104930_e157307_d_n4;
        var_t9_dn5 = assign104930_e157307_d_n5;
        var_t9_dn6 = assign104930_e157307_d_n6;
        var_t9_dn7 = assign104930_e157307_d_n7;
        var_t9_dn8 = assign104930_e157307_d_n8;
        var_t9_dn9 = assign104930_e157307_d_n9;
        var_t9_dn10 = assign104930_e157307_d_n10;
        var_t9_dn13 = assign104930_e157307_d_n13;

        let (assign104940_e157318, assign104940_e157318_d_n0, assign104940_e157318_d_n2, assign104940_e157318_d_n4, assign104940_e157318_d_n5, assign104940_e157318_d_n6, assign104940_e157318_d_n7, assign104940_e157318_d_n8, assign104940_e157318_d_n9, assign104940_e157318_d_n10, assign104940_e157318_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104940_e157315: f64 = (var_ldrifte + p.p422);
        let assign104940_e157316: f64 = (1.6021918e-19 / assign104940_e157315);
        (assign104940_e157316, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign104940_e157318;
        var_t1_dn0 = assign104940_e157318_d_n0;
        var_t1_dn2 = assign104940_e157318_d_n2;
        var_t1_dn4 = assign104940_e157318_d_n4;
        var_t1_dn5 = assign104940_e157318_d_n5;
        var_t1_dn6 = assign104940_e157318_d_n6;
        var_t1_dn7 = assign104940_e157318_d_n7;
        var_t1_dn8 = assign104940_e157318_d_n8;
        var_t1_dn9 = assign104940_e157318_d_n9;
        var_t1_dn10 = assign104940_e157318_d_n10;
        var_t1_dn13 = assign104940_e157318_d_n13;

        let (assign104950_e157331, assign104950_e157331_d_n0, assign104950_e157331_d_n2, assign104950_e157331_d_n4, assign104950_e157331_d_n5, assign104950_e157331_d_n6, assign104950_e157331_d_n7, assign104950_e157331_d_n8, assign104950_e157331_d_n9, assign104950_e157331_d_n10, assign104950_e157331_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign104950_e157325: f64 = (var_t1 * var_xov);
        let assign104950_e157327: f64 = (assign104950_e157325 * var_mu__blk2354);
        let assign104950_e157329: f64 = (assign104950_e157327 * var_carr);
        (assign104950_e157329, ((((((var_t1_dn0 * var_xov) + (var_t1 * var_xov_dn0)) * var_mu__blk2354) + (assign104950_e157325 * var_mu__blk2354_dn0)) * var_carr) + (assign104950_e157327 * var_carr_dn0)), ((((((var_t1_dn2 * var_xov) + (var_t1 * var_xov_dn2)) * var_mu__blk2354) + (assign104950_e157325 * var_mu__blk2354_dn2)) * var_carr) + (assign104950_e157327 * var_carr_dn2)), ((((((var_t1_dn4 * var_xov) + (var_t1 * var_xov_dn4)) * var_mu__blk2354) + (assign104950_e157325 * var_mu__blk2354_dn4)) * var_carr) + (assign104950_e157327 * var_carr_dn4)), ((((((var_t1_dn5 * var_xov) + (var_t1 * var_xov_dn5)) * var_mu__blk2354) + (assign104950_e157325 * var_mu__blk2354_dn5)) * var_carr) + (assign104950_e157327 * var_carr_dn5)), ((((((var_t1_dn6 * var_xov) + (var_t1 * var_xov_dn6)) * var_mu__blk2354) + (assign104950_e157325 * var_mu__blk2354_dn6)) * var_carr) + (assign104950_e157327 * var_carr_dn6)), ((((((var_t1_dn7 * var_xov) + (var_t1 * var_xov_dn7)) * var_mu__blk2354) + (assign104950_e157325 * var_mu__blk2354_dn7)) * var_carr) + (assign104950_e157327 * var_carr_dn7)), ((((((var_t1_dn8 * var_xov) + (var_t1 * var_xov_dn8)) * var_mu__blk2354) + (assign104950_e157325 * var_mu__blk2354_dn8)) * var_carr) + (assign104950_e157327 * var_carr_dn8)), ((((((var_t1_dn9 * var_xov) + (var_t1 * var_xov_dn9)) * var_mu__blk2354) + (assign104950_e157325 * var_mu__blk2354_dn9)) * var_carr) + (assign104950_e157327 * var_carr_dn9)), ((((((var_t1_dn10 * var_xov) + (var_t1 * var_xov_dn10)) * var_mu__blk2354) + (assign104950_e157325 * var_mu__blk2354_dn10)) * var_carr) + (assign104950_e157327 * var_carr_dn10)), ((((((var_t1_dn13 * var_xov) + (var_t1 * var_xov_dn13)) * var_mu__blk2354) + (assign104950_e157325 * var_mu__blk2354_dn13)) * var_carr) + (assign104950_e157327 * var_carr_dn13)),)
    } else {
        (var_gd, var_gd_dn0, var_gd_dn2, var_gd_dn4, var_gd_dn5, var_gd_dn6, var_gd_dn7, var_gd_dn8, var_gd_dn9, var_gd_dn10, var_gd_dn13,)
    }
};
        var_gd = assign104950_e157331;
        var_gd_dn0 = assign104950_e157331_d_n0;
        var_gd_dn2 = assign104950_e157331_d_n2;
        var_gd_dn4 = assign104950_e157331_d_n4;
        var_gd_dn5 = assign104950_e157331_d_n5;
        var_gd_dn6 = assign104950_e157331_d_n6;
        var_gd_dn7 = assign104950_e157331_d_n7;
        var_gd_dn8 = assign104950_e157331_d_n8;
        var_gd_dn9 = assign104950_e157331_d_n9;
        var_gd_dn10 = assign104950_e157331_d_n10;
        var_gd_dn13 = assign104950_e157331_d_n13;

        let assign104960_e157335: f64 = 1e-25;
        let assign104960_e157340: f64 = if ((var_gd < assign104960_e157335) && (1e-25 >= 0.0)) { 1.0 } else { 0.0 };
        var_guard2381 = assign104960_e157340;

        let (assign104970_e157353, assign104970_e157353_d_n0, assign104970_e157353_d_n2, assign104970_e157353_d_n4, assign104970_e157353_d_n5, assign104970_e157353_d_n6, assign104970_e157353_d_n7, assign104970_e157353_d_n8, assign104970_e157353_d_n9, assign104970_e157353_d_n10, assign104970_e157353_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        let assign104970_e157349: f64 = 1e-25;
        let assign104970_e157351: f64 = (assign104970_e157349 - var_gd);
        (assign104970_e157351, (-var_gd_dn0), (-var_gd_dn2), (-var_gd_dn4), (-var_gd_dn5), (-var_gd_dn6), (-var_gd_dn7), (-var_gd_dn8), (-var_gd_dn9), (-var_gd_dn10), (-var_gd_dn13),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn13,)
    }
};
        var_tmf1 = assign104970_e157353;
        var_tmf1_dn0 = assign104970_e157353_d_n0;
        var_tmf1_dn2 = assign104970_e157353_d_n2;
        var_tmf1_dn4 = assign104970_e157353_d_n4;
        var_tmf1_dn5 = assign104970_e157353_d_n5;
        var_tmf1_dn6 = assign104970_e157353_d_n6;
        var_tmf1_dn7 = assign104970_e157353_d_n7;
        var_tmf1_dn8 = assign104970_e157353_d_n8;
        var_tmf1_dn9 = assign104970_e157353_d_n9;
        var_tmf1_dn10 = assign104970_e157353_d_n10;
        var_tmf1_dn13 = assign104970_e157353_d_n13;

        let (assign104980_e157364, assign104980_e157364_d_n0, assign104980_e157364_d_n2, assign104980_e157364_d_n4, assign104980_e157364_d_n5, assign104980_e157364_d_n6, assign104980_e157364_d_n7, assign104980_e157364_d_n8, assign104980_e157364_d_n9, assign104980_e157364_d_n10, assign104980_e157364_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        let assign104980_e157362: f64 = (var_tmf1 * var_tmf1);
        (assign104980_e157362, ((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)), ((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)), ((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)), ((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)), ((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)), ((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)), ((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)), ((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)), ((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)), ((var_tmf1_dn13 * var_tmf1) + (var_tmf1 * var_tmf1_dn13)),)
    } else {
        (var_x2, var_x2_dn0, var_x2_dn2, var_x2_dn4, var_x2_dn5, var_x2_dn6, var_x2_dn7, var_x2_dn8, var_x2_dn9, var_x2_dn10, var_x2_dn13,)
    }
};
        var_x2 = assign104980_e157364;
        var_x2_dn0 = assign104980_e157364_d_n0;
        var_x2_dn2 = assign104980_e157364_d_n2;
        var_x2_dn4 = assign104980_e157364_d_n4;
        var_x2_dn5 = assign104980_e157364_d_n5;
        var_x2_dn6 = assign104980_e157364_d_n6;
        var_x2_dn7 = assign104980_e157364_d_n7;
        var_x2_dn8 = assign104980_e157364_d_n8;
        var_x2_dn9 = assign104980_e157364_d_n9;
        var_x2_dn10 = assign104980_e157364_d_n10;
        var_x2_dn13 = assign104980_e157364_d_n13;

        let (assign104990_e157375, assign104990_e157375_d_n0, assign104990_e157375_d_n2, assign104990_e157375_d_n4, assign104990_e157375_d_n5, assign104990_e157375_d_n6, assign104990_e157375_d_n7, assign104990_e157375_d_n8, assign104990_e157375_d_n9, assign104990_e157375_d_n10, assign104990_e157375_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        let assign104990_e157373: f64 = (1e-25 * 1e-25);
        (assign104990_e157373, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmax2, var_xmax2_dn0, var_xmax2_dn2, var_xmax2_dn4, var_xmax2_dn5, var_xmax2_dn6, var_xmax2_dn7, var_xmax2_dn8, var_xmax2_dn9, var_xmax2_dn10, var_xmax2_dn13,)
    }
};
        var_xmax2 = assign104990_e157375;
        var_xmax2_dn0 = assign104990_e157375_d_n0;
        var_xmax2_dn2 = assign104990_e157375_d_n2;
        var_xmax2_dn4 = assign104990_e157375_d_n4;
        var_xmax2_dn5 = assign104990_e157375_d_n5;
        var_xmax2_dn6 = assign104990_e157375_d_n6;
        var_xmax2_dn7 = assign104990_e157375_d_n7;
        var_xmax2_dn8 = assign104990_e157375_d_n8;
        var_xmax2_dn9 = assign104990_e157375_d_n9;
        var_xmax2_dn10 = assign104990_e157375_d_n10;
        var_xmax2_dn13 = assign104990_e157375_d_n13;

        let (assign105000_e157384, assign105000_e157384_d_n0, assign105000_e157384_d_n2, assign105000_e157384_d_n4, assign105000_e157384_d_n5, assign105000_e157384_d_n6, assign105000_e157384_d_n7, assign105000_e157384_d_n8, assign105000_e157384_d_n9, assign105000_e157384_d_n10, assign105000_e157384_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign105000_e157384;
        var_xp_dn0 = assign105000_e157384_d_n0;
        var_xp_dn2 = assign105000_e157384_d_n2;
        var_xp_dn4 = assign105000_e157384_d_n4;
        var_xp_dn5 = assign105000_e157384_d_n5;
        var_xp_dn6 = assign105000_e157384_d_n6;
        var_xp_dn7 = assign105000_e157384_d_n7;
        var_xp_dn8 = assign105000_e157384_d_n8;
        var_xp_dn9 = assign105000_e157384_d_n9;
        var_xp_dn10 = assign105000_e157384_d_n10;
        var_xp_dn13 = assign105000_e157384_d_n13;

        let (assign105010_e157393, assign105010_e157393_d_n0, assign105010_e157393_d_n2, assign105010_e157393_d_n4, assign105010_e157393_d_n5, assign105010_e157393_d_n6, assign105010_e157393_d_n7, assign105010_e157393_d_n8, assign105010_e157393_d_n9, assign105010_e157393_d_n10, assign105010_e157393_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign105010_e157393;
        var_xmp_dn0 = assign105010_e157393_d_n0;
        var_xmp_dn2 = assign105010_e157393_d_n2;
        var_xmp_dn4 = assign105010_e157393_d_n4;
        var_xmp_dn5 = assign105010_e157393_d_n5;
        var_xmp_dn6 = assign105010_e157393_d_n6;
        var_xmp_dn7 = assign105010_e157393_d_n7;
        var_xmp_dn8 = assign105010_e157393_d_n8;
        var_xmp_dn9 = assign105010_e157393_d_n9;
        var_xmp_dn10 = assign105010_e157393_d_n10;
        var_xmp_dn13 = assign105010_e157393_d_n13;

        let (assign105020_e157402,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign105020_e157402;

        let (assign105030_e157411,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        (0.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105030_e157411;

        let (assign105040_e157420, assign105040_e157420_d_n0, assign105040_e157420_d_n2, assign105040_e157420_d_n4, assign105040_e157420_d_n5, assign105040_e157420_d_n6, assign105040_e157420_d_n7, assign105040_e157420_d_n8, assign105040_e157420_d_n9, assign105040_e157420_d_n10, assign105040_e157420_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    }
};
        var_arg = assign105040_e157420;
        var_arg_dn0 = assign105040_e157420_d_n0;
        var_arg_dn2 = assign105040_e157420_d_n2;
        var_arg_dn4 = assign105040_e157420_d_n4;
        var_arg_dn5 = assign105040_e157420_d_n5;
        var_arg_dn6 = assign105040_e157420_d_n6;
        var_arg_dn7 = assign105040_e157420_d_n7;
        var_arg_dn8 = assign105040_e157420_d_n8;
        var_arg_dn9 = assign105040_e157420_d_n9;
        var_arg_dn10 = assign105040_e157420_d_n10;
        var_arg_dn13 = assign105040_e157420_d_n13;

        let (assign105050_e157429, assign105050_e157429_d_n0, assign105050_e157429_d_n2, assign105050_e157429_d_n4, assign105050_e157429_d_n5, assign105050_e157429_d_n6, assign105050_e157429_d_n7, assign105050_e157429_d_n8, assign105050_e157429_d_n9, assign105050_e157429_d_n10, assign105050_e157429_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign105050_e157429;
        var_dnm_dn0 = assign105050_e157429_d_n0;
        var_dnm_dn2 = assign105050_e157429_d_n2;
        var_dnm_dn4 = assign105050_e157429_d_n4;
        var_dnm_dn5 = assign105050_e157429_d_n5;
        var_dnm_dn6 = assign105050_e157429_d_n6;
        var_dnm_dn7 = assign105050_e157429_d_n7;
        var_dnm_dn8 = assign105050_e157429_d_n8;
        var_dnm_dn9 = assign105050_e157429_d_n9;
        var_dnm_dn10 = assign105050_e157429_d_n10;
        var_dnm_dn13 = assign105050_e157429_d_n13;

        let (assign105060_e157440, assign105060_e157440_d_n0, assign105060_e157440_d_n2, assign105060_e157440_d_n4, assign105060_e157440_d_n5, assign105060_e157440_d_n6, assign105060_e157440_d_n7, assign105060_e157440_d_n8, assign105060_e157440_d_n9, assign105060_e157440_d_n10, assign105060_e157440_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        let assign105060_e157438: f64 = (var_xp * var_x2);
        (assign105060_e157438, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn13 * var_x2) + (var_xp * var_x2_dn13)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign105060_e157440;
        var_xp_dn0 = assign105060_e157440_d_n0;
        var_xp_dn2 = assign105060_e157440_d_n2;
        var_xp_dn4 = assign105060_e157440_d_n4;
        var_xp_dn5 = assign105060_e157440_d_n5;
        var_xp_dn6 = assign105060_e157440_d_n6;
        var_xp_dn7 = assign105060_e157440_d_n7;
        var_xp_dn8 = assign105060_e157440_d_n8;
        var_xp_dn9 = assign105060_e157440_d_n9;
        var_xp_dn10 = assign105060_e157440_d_n10;
        var_xp_dn13 = assign105060_e157440_d_n13;

        let (assign105070_e157451, assign105070_e157451_d_n0, assign105070_e157451_d_n2, assign105070_e157451_d_n4, assign105070_e157451_d_n5, assign105070_e157451_d_n6, assign105070_e157451_d_n7, assign105070_e157451_d_n8, assign105070_e157451_d_n9, assign105070_e157451_d_n10, assign105070_e157451_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        let assign105070_e157449: f64 = (var_xmp * var_xmax2);
        (assign105070_e157449, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn13 * var_xmax2) + (var_xmp * var_xmax2_dn13)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign105070_e157451;
        var_xmp_dn0 = assign105070_e157451_d_n0;
        var_xmp_dn2 = assign105070_e157451_d_n2;
        var_xmp_dn4 = assign105070_e157451_d_n4;
        var_xmp_dn5 = assign105070_e157451_d_n5;
        var_xmp_dn6 = assign105070_e157451_d_n6;
        var_xmp_dn7 = assign105070_e157451_d_n7;
        var_xmp_dn8 = assign105070_e157451_d_n8;
        var_xmp_dn9 = assign105070_e157451_d_n9;
        var_xmp_dn10 = assign105070_e157451_d_n10;
        var_xmp_dn13 = assign105070_e157451_d_n13;

        let (assign105080_e157462, assign105080_e157462_d_n0, assign105080_e157462_d_n2, assign105080_e157462_d_n4, assign105080_e157462_d_n5, assign105080_e157462_d_n6, assign105080_e157462_d_n7, assign105080_e157462_d_n8, assign105080_e157462_d_n9, assign105080_e157462_d_n10, assign105080_e157462_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        let assign105080_e157460: f64 = (var_xp * var_x2);
        (assign105080_e157460, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn13 * var_x2) + (var_xp * var_x2_dn13)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign105080_e157462;
        var_xp_dn0 = assign105080_e157462_d_n0;
        var_xp_dn2 = assign105080_e157462_d_n2;
        var_xp_dn4 = assign105080_e157462_d_n4;
        var_xp_dn5 = assign105080_e157462_d_n5;
        var_xp_dn6 = assign105080_e157462_d_n6;
        var_xp_dn7 = assign105080_e157462_d_n7;
        var_xp_dn8 = assign105080_e157462_d_n8;
        var_xp_dn9 = assign105080_e157462_d_n9;
        var_xp_dn10 = assign105080_e157462_d_n10;
        var_xp_dn13 = assign105080_e157462_d_n13;

        let (assign105090_e157473, assign105090_e157473_d_n0, assign105090_e157473_d_n2, assign105090_e157473_d_n4, assign105090_e157473_d_n5, assign105090_e157473_d_n6, assign105090_e157473_d_n7, assign105090_e157473_d_n8, assign105090_e157473_d_n9, assign105090_e157473_d_n10, assign105090_e157473_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        let assign105090_e157471: f64 = (var_xmp * var_xmax2);
        (assign105090_e157471, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn13 * var_xmax2) + (var_xmp * var_xmax2_dn13)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign105090_e157473;
        var_xmp_dn0 = assign105090_e157473_d_n0;
        var_xmp_dn2 = assign105090_e157473_d_n2;
        var_xmp_dn4 = assign105090_e157473_d_n4;
        var_xmp_dn5 = assign105090_e157473_d_n5;
        var_xmp_dn6 = assign105090_e157473_d_n6;
        var_xmp_dn7 = assign105090_e157473_d_n7;
        var_xmp_dn8 = assign105090_e157473_d_n8;
        var_xmp_dn9 = assign105090_e157473_d_n9;
        var_xmp_dn10 = assign105090_e157473_d_n10;
        var_xmp_dn13 = assign105090_e157473_d_n13;

        let (assign105100_e157484, assign105100_e157484_d_n0, assign105100_e157484_d_n2, assign105100_e157484_d_n4, assign105100_e157484_d_n5, assign105100_e157484_d_n6, assign105100_e157484_d_n7, assign105100_e157484_d_n8, assign105100_e157484_d_n9, assign105100_e157484_d_n10, assign105100_e157484_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        let assign105100_e157482: f64 = (var_xp + var_xmp);
        (assign105100_e157482, (var_xp_dn0 + var_xmp_dn0), (var_xp_dn2 + var_xmp_dn2), (var_xp_dn4 + var_xmp_dn4), (var_xp_dn5 + var_xmp_dn5), (var_xp_dn6 + var_xmp_dn6), (var_xp_dn7 + var_xmp_dn7), (var_xp_dn8 + var_xmp_dn8), (var_xp_dn9 + var_xmp_dn9), (var_xp_dn10 + var_xmp_dn10), (var_xp_dn13 + var_xmp_dn13),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    }
};
        var_arg = assign105100_e157484;
        var_arg_dn0 = assign105100_e157484_d_n0;
        var_arg_dn2 = assign105100_e157484_d_n2;
        var_arg_dn4 = assign105100_e157484_d_n4;
        var_arg_dn5 = assign105100_e157484_d_n5;
        var_arg_dn6 = assign105100_e157484_d_n6;
        var_arg_dn7 = assign105100_e157484_d_n7;
        var_arg_dn8 = assign105100_e157484_d_n8;
        var_arg_dn9 = assign105100_e157484_d_n9;
        var_arg_dn10 = assign105100_e157484_d_n10;
        var_arg_dn13 = assign105100_e157484_d_n13;

        let (assign105110_e157493, assign105110_e157493_d_n0, assign105110_e157493_d_n2, assign105110_e157493_d_n4, assign105110_e157493_d_n5, assign105110_e157493_d_n6, assign105110_e157493_d_n7, assign105110_e157493_d_n8, assign105110_e157493_d_n9, assign105110_e157493_d_n10, assign105110_e157493_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign105110_e157493;
        var_dnm_dn0 = assign105110_e157493_d_n0;
        var_dnm_dn2 = assign105110_e157493_d_n2;
        var_dnm_dn4 = assign105110_e157493_d_n4;
        var_dnm_dn5 = assign105110_e157493_d_n5;
        var_dnm_dn6 = assign105110_e157493_d_n6;
        var_dnm_dn7 = assign105110_e157493_d_n7;
        var_dnm_dn8 = assign105110_e157493_d_n8;
        var_dnm_dn9 = assign105110_e157493_d_n9;
        var_dnm_dn10 = assign105110_e157493_d_n10;
        var_dnm_dn13 = assign105110_e157493_d_n13;

        let assign105120_e157508: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        var_guard2382 = assign105120_e157508;

        let assign105130_e157511: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard2383 = assign105130_e157511;

        let (assign105140_e157524,) = {
    if (((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) && (var_guard2382 != 0.0)) && (var_guard2383 != 0.0)) {
        (1.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105140_e157524;

        let assign105150_e157527: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard2384 = assign105150_e157527;

        let (assign105160_e157543,) = {
    if ((((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) && (var_guard2382 != 0.0)) && (var_guard2383 == 0.0)) && (var_guard2384 != 0.0)) {
        (2.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105160_e157543;

        let assign105170_e157546: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        var_guard2385 = assign105170_e157546;

        let (assign105180_e157565,) = {
    if (((((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) && (var_guard2382 != 0.0)) && (var_guard2383 == 0.0)) && (var_guard2384 == 0.0)) && (var_guard2385 != 0.0)) {
        (3.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105180_e157565;

        let assign105190_e157568: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        var_guard2386 = assign105190_e157568;

        let (assign105200_e157590,) = {
    if ((((((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) && (var_guard2382 != 0.0)) && (var_guard2383 == 0.0)) && (var_guard2384 == 0.0)) && (var_guard2385 == 0.0)) && (var_guard2386 != 0.0)) {
        (4.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105200_e157590;

        let (assign105210_e157601,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) && (var_guard2382 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign105210_e157601;

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
        *var_gd_slot = var_gd;
        *var_gd_dn0_slot = var_gd_dn0;
        *var_gd_dn10_slot = var_gd_dn10;
        *var_gd_dn13_slot = var_gd_dn13;
        *var_gd_dn2_slot = var_gd_dn2;
        *var_gd_dn4_slot = var_gd_dn4;
        *var_gd_dn5_slot = var_gd_dn5;
        *var_gd_dn6_slot = var_gd_dn6;
        *var_gd_dn7_slot = var_gd_dn7;
        *var_gd_dn8_slot = var_gd_dn8;
        *var_gd_dn9_slot = var_gd_dn9;
        *var_guard2380_slot = var_guard2380;
        *var_guard2381_slot = var_guard2381;
        *var_guard2382_slot = var_guard2382;
        *var_guard2383_slot = var_guard2383;
        *var_guard2384_slot = var_guard2384;
        *var_guard2385_slot = var_guard2385;
        *var_guard2386_slot = var_guard2386;
        *var_m0_slot = var_m0;
        *var_mm_slot = var_mm;
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
        *var_xov_slot = var_xov;
        *var_xov_dn0_slot = var_xov_dn0;
        *var_xov_dn10_slot = var_xov_dn10;
        *var_xov_dn13_slot = var_xov_dn13;
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
        *var_xp_dn13_slot = var_xp_dn13;
        *var_xp_dn2_slot = var_xp_dn2;
        *var_xp_dn4_slot = var_xp_dn4;
        *var_xp_dn5_slot = var_xp_dn5;
        *var_xp_dn6_slot = var_xp_dn6;
        *var_xp_dn7_slot = var_xp_dn7;
        *var_xp_dn8_slot = var_xp_dn8;
        *var_xp_dn9_slot = var_xp_dn9;
    }

    pub(super) fn stamp_transient_block_372(
        var_guard2336: f64,
        var_guard2356: f64,
        var_guard2381: f64,
        var_guard2382: f64,
        var_weffld_nf: f64,
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
        var_gd_slot: &mut f64,
        var_gd_dn0_slot: &mut f64,
        var_gd_dn10_slot: &mut f64,
        var_gd_dn13_slot: &mut f64,
        var_gd_dn2_slot: &mut f64,
        var_gd_dn4_slot: &mut f64,
        var_gd_dn5_slot: &mut f64,
        var_gd_dn6_slot: &mut f64,
        var_gd_dn7_slot: &mut f64,
        var_gd_dn8_slot: &mut f64,
        var_gd_dn9_slot: &mut f64,
        var_guard2387_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_mm_slot: &mut f64,
        var_rdd_slot: &mut f64,
        var_rdd_dn0_slot: &mut f64,
        var_rdd_dn10_slot: &mut f64,
        var_rdd_dn13_slot: &mut f64,
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
        var_t0_dn13_slot: &mut f64,
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
        var_tmf0_dn13_slot: &mut f64,
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
        var_tmf1_dn13_slot: &mut f64,
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
        var_x2_dn13_slot: &mut f64,
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
        var_xmax2_dn13_slot: &mut f64,
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
        var_xmp_dn13_slot: &mut f64,
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
        var_xp_dn13_slot: &mut f64,
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
        let mut var_arg_dn13: f64 = *var_arg_dn13_slot;
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
        let mut var_dnm_dn13: f64 = *var_dnm_dn13_slot;
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
        let mut var_gd_dn13: f64 = *var_gd_dn13_slot;
        let mut var_gd_dn2: f64 = *var_gd_dn2_slot;
        let mut var_gd_dn4: f64 = *var_gd_dn4_slot;
        let mut var_gd_dn5: f64 = *var_gd_dn5_slot;
        let mut var_gd_dn6: f64 = *var_gd_dn6_slot;
        let mut var_gd_dn7: f64 = *var_gd_dn7_slot;
        let mut var_gd_dn8: f64 = *var_gd_dn8_slot;
        let mut var_gd_dn9: f64 = *var_gd_dn9_slot;
        let mut var_guard2387: f64 = *var_guard2387_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_mm: f64 = *var_mm_slot;
        let mut var_rdd: f64 = *var_rdd_slot;
        let mut var_rdd_dn0: f64 = *var_rdd_dn0_slot;
        let mut var_rdd_dn10: f64 = *var_rdd_dn10_slot;
        let mut var_rdd_dn13: f64 = *var_rdd_dn13_slot;
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
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
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
        let mut var_tmf0_dn13: f64 = *var_tmf0_dn13_slot;
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
        let mut var_tmf1_dn13: f64 = *var_tmf1_dn13_slot;
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
        let mut var_x2_dn13: f64 = *var_x2_dn13_slot;
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
        let mut var_xmax2_dn13: f64 = *var_xmax2_dn13_slot;
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
        let mut var_xmp_dn13: f64 = *var_xmp_dn13_slot;
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
        let mut var_xp_dn13: f64 = *var_xp_dn13_slot;
        let mut var_xp_dn2: f64 = *var_xp_dn2_slot;
        let mut var_xp_dn4: f64 = *var_xp_dn4_slot;
        let mut var_xp_dn5: f64 = *var_xp_dn5_slot;
        let mut var_xp_dn6: f64 = *var_xp_dn6_slot;
        let mut var_xp_dn7: f64 = *var_xp_dn7_slot;
        let mut var_xp_dn8: f64 = *var_xp_dn8_slot;
        let mut var_xp_dn9: f64 = *var_xp_dn9_slot;

        let mut assign105220_loop_guard: usize = 0;
        while {
            let assign105220_cond_e157613: f64 = if (((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) && (var_guard2382 != 0.0)) && (var_m0 < var_mm)) { 1.0 } else { 0.0 };
            assign105220_cond_e157613 != 0.0
        } {
            assign105220_loop_guard += 1;
            assert!(assign105220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign105220_body0_e157625, assign105220_body0_e157625_d_n0, assign105220_body0_e157625_d_n2, assign105220_body0_e157625_d_n4, assign105220_body0_e157625_d_n5, assign105220_body0_e157625_d_n6, assign105220_body0_e157625_d_n7, assign105220_body0_e157625_d_n8, assign105220_body0_e157625_d_n9, assign105220_body0_e157625_d_n10, assign105220_body0_e157625_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) && (var_guard2382 != 0.0)) {
        let assign105220_body0_e157623: f64 = (var_dnm).sqrt();
        (assign105220_body0_e157623, (var_dnm_dn0 / (2.0 * assign105220_body0_e157623)), (var_dnm_dn2 / (2.0 * assign105220_body0_e157623)), (var_dnm_dn4 / (2.0 * assign105220_body0_e157623)), (var_dnm_dn5 / (2.0 * assign105220_body0_e157623)), (var_dnm_dn6 / (2.0 * assign105220_body0_e157623)), (var_dnm_dn7 / (2.0 * assign105220_body0_e157623)), (var_dnm_dn8 / (2.0 * assign105220_body0_e157623)), (var_dnm_dn9 / (2.0 * assign105220_body0_e157623)), (var_dnm_dn10 / (2.0 * assign105220_body0_e157623)), (var_dnm_dn13 / (2.0 * assign105220_body0_e157623)),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
            var_dnm = assign105220_body0_e157625;
            var_dnm_dn0 = assign105220_body0_e157625_d_n0;
            var_dnm_dn2 = assign105220_body0_e157625_d_n2;
            var_dnm_dn4 = assign105220_body0_e157625_d_n4;
            var_dnm_dn5 = assign105220_body0_e157625_d_n5;
            var_dnm_dn6 = assign105220_body0_e157625_d_n6;
            var_dnm_dn7 = assign105220_body0_e157625_d_n7;
            var_dnm_dn8 = assign105220_body0_e157625_d_n8;
            var_dnm_dn9 = assign105220_body0_e157625_d_n9;
            var_dnm_dn10 = assign105220_body0_e157625_d_n10;
            var_dnm_dn13 = assign105220_body0_e157625_d_n13;
            let (assign105220_body1_e157638,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) && (var_guard2382 != 0.0)) {
        let assign105220_body1_e157636: f64 = (var_m0 + 1.0);
        (assign105220_body1_e157636,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign105220_body1_e157638;
        }

        let (assign105230_e157661, assign105230_e157661_d_n0, assign105230_e157661_d_n2, assign105230_e157661_d_n4, assign105230_e157661_d_n5, assign105230_e157661_d_n6, assign105230_e157661_d_n7, assign105230_e157661_d_n8, assign105230_e157661_d_n9, assign105230_e157661_d_n10, assign105230_e157661_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) && (var_guard2382 == 0.0)) {
        let (assign105230_e157659, assign105230_e157659_d_n0, assign105230_e157659_d_n2, assign105230_e157659_d_n4, assign105230_e157659_d_n5, assign105230_e157659_d_n6, assign105230_e157659_d_n7, assign105230_e157659_d_n8, assign105230_e157659_d_n9, assign105230_e157659_d_n10, assign105230_e157659_d_n13,) = {
            if (var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign105230_e157656: f64 = (2.0 * 2.0);
                let assign105230_e157657: f64 = (1.0 / assign105230_e157656);
                let assign105230_e157658: f64 = (var_dnm).powf(assign105230_e157657);
                (assign105230_e157658, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((var_dnm).powf(assign105230_e157657 - 1.0) * var_dnm_dn0)) } } else { (assign105230_e157658 * (assign105230_e157657 * (var_dnm_dn0 / var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((var_dnm).powf(assign105230_e157657 - 1.0) * var_dnm_dn2)) } } else { (assign105230_e157658 * (assign105230_e157657 * (var_dnm_dn2 / var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((var_dnm).powf(assign105230_e157657 - 1.0) * var_dnm_dn4)) } } else { (assign105230_e157658 * (assign105230_e157657 * (var_dnm_dn4 / var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((var_dnm).powf(assign105230_e157657 - 1.0) * var_dnm_dn5)) } } else { (assign105230_e157658 * (assign105230_e157657 * (var_dnm_dn5 / var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((var_dnm).powf(assign105230_e157657 - 1.0) * var_dnm_dn6)) } } else { (assign105230_e157658 * (assign105230_e157657 * (var_dnm_dn6 / var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((var_dnm).powf(assign105230_e157657 - 1.0) * var_dnm_dn7)) } } else { (assign105230_e157658 * (assign105230_e157657 * (var_dnm_dn7 / var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((var_dnm).powf(assign105230_e157657 - 1.0) * var_dnm_dn8)) } } else { (assign105230_e157658 * (assign105230_e157657 * (var_dnm_dn8 / var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((var_dnm).powf(assign105230_e157657 - 1.0) * var_dnm_dn9)) } } else { (assign105230_e157658 * (assign105230_e157657 * (var_dnm_dn9 / var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((var_dnm).powf(assign105230_e157657 - 1.0) * var_dnm_dn10)) } } else { (assign105230_e157658 * (assign105230_e157657 * (var_dnm_dn10 / var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((var_dnm).powf(assign105230_e157657 - 1.0) * var_dnm_dn13)) } } else { (assign105230_e157658 * (assign105230_e157657 * (var_dnm_dn13 / var_dnm))) },)
            }
        };
        (assign105230_e157659, assign105230_e157659_d_n0, assign105230_e157659_d_n2, assign105230_e157659_d_n4, assign105230_e157659_d_n5, assign105230_e157659_d_n6, assign105230_e157659_d_n7, assign105230_e157659_d_n8, assign105230_e157659_d_n9, assign105230_e157659_d_n10, assign105230_e157659_d_n13,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign105230_e157661;
        var_dnm_dn0 = assign105230_e157661_d_n0;
        var_dnm_dn2 = assign105230_e157661_d_n2;
        var_dnm_dn4 = assign105230_e157661_d_n4;
        var_dnm_dn5 = assign105230_e157661_d_n5;
        var_dnm_dn6 = assign105230_e157661_d_n6;
        var_dnm_dn7 = assign105230_e157661_d_n7;
        var_dnm_dn8 = assign105230_e157661_d_n8;
        var_dnm_dn9 = assign105230_e157661_d_n9;
        var_dnm_dn10 = assign105230_e157661_d_n10;
        var_dnm_dn13 = assign105230_e157661_d_n13;

        let (assign105240_e157672, assign105240_e157672_d_n0, assign105240_e157672_d_n2, assign105240_e157672_d_n4, assign105240_e157672_d_n5, assign105240_e157672_d_n6, assign105240_e157672_d_n7, assign105240_e157672_d_n8, assign105240_e157672_d_n9, assign105240_e157672_d_n10, assign105240_e157672_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        let assign105240_e157670: f64 = (1.0 / var_dnm);
        (assign105240_e157670, (-(var_dnm_dn0 / (var_dnm * var_dnm))), (-(var_dnm_dn2 / (var_dnm * var_dnm))), (-(var_dnm_dn4 / (var_dnm * var_dnm))), (-(var_dnm_dn5 / (var_dnm * var_dnm))), (-(var_dnm_dn6 / (var_dnm * var_dnm))), (-(var_dnm_dn7 / (var_dnm * var_dnm))), (-(var_dnm_dn8 / (var_dnm * var_dnm))), (-(var_dnm_dn9 / (var_dnm * var_dnm))), (-(var_dnm_dn10 / (var_dnm * var_dnm))), (-(var_dnm_dn13 / (var_dnm * var_dnm))),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign105240_e157672;
        var_dnm_dn0 = assign105240_e157672_d_n0;
        var_dnm_dn2 = assign105240_e157672_d_n2;
        var_dnm_dn4 = assign105240_e157672_d_n4;
        var_dnm_dn5 = assign105240_e157672_d_n5;
        var_dnm_dn6 = assign105240_e157672_d_n6;
        var_dnm_dn7 = assign105240_e157672_d_n7;
        var_dnm_dn8 = assign105240_e157672_d_n8;
        var_dnm_dn9 = assign105240_e157672_d_n9;
        var_dnm_dn10 = assign105240_e157672_d_n10;
        var_dnm_dn13 = assign105240_e157672_d_n13;

        let (assign105250_e157685, assign105250_e157685_d_n0, assign105250_e157685_d_n2, assign105250_e157685_d_n4, assign105250_e157685_d_n5, assign105250_e157685_d_n6, assign105250_e157685_d_n7, assign105250_e157685_d_n8, assign105250_e157685_d_n9, assign105250_e157685_d_n10, assign105250_e157685_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        let assign105250_e157681: f64 = (var_tmf1 * 1e-25);
        let assign105250_e157683: f64 = (assign105250_e157681 * var_dnm);
        (assign105250_e157683, (((var_tmf1_dn0 * 1e-25) * var_dnm) + (assign105250_e157681 * var_dnm_dn0)), (((var_tmf1_dn2 * 1e-25) * var_dnm) + (assign105250_e157681 * var_dnm_dn2)), (((var_tmf1_dn4 * 1e-25) * var_dnm) + (assign105250_e157681 * var_dnm_dn4)), (((var_tmf1_dn5 * 1e-25) * var_dnm) + (assign105250_e157681 * var_dnm_dn5)), (((var_tmf1_dn6 * 1e-25) * var_dnm) + (assign105250_e157681 * var_dnm_dn6)), (((var_tmf1_dn7 * 1e-25) * var_dnm) + (assign105250_e157681 * var_dnm_dn7)), (((var_tmf1_dn8 * 1e-25) * var_dnm) + (assign105250_e157681 * var_dnm_dn8)), (((var_tmf1_dn9 * 1e-25) * var_dnm) + (assign105250_e157681 * var_dnm_dn9)), (((var_tmf1_dn10 * 1e-25) * var_dnm) + (assign105250_e157681 * var_dnm_dn10)), (((var_tmf1_dn13 * 1e-25) * var_dnm) + (assign105250_e157681 * var_dnm_dn13)),)
    } else {
        (var_tmf0, var_tmf0_dn0, var_tmf0_dn2, var_tmf0_dn4, var_tmf0_dn5, var_tmf0_dn6, var_tmf0_dn7, var_tmf0_dn8, var_tmf0_dn9, var_tmf0_dn10, var_tmf0_dn13,)
    }
};
        var_tmf0 = assign105250_e157685;
        var_tmf0_dn0 = assign105250_e157685_d_n0;
        var_tmf0_dn2 = assign105250_e157685_d_n2;
        var_tmf0_dn4 = assign105250_e157685_d_n4;
        var_tmf0_dn5 = assign105250_e157685_d_n5;
        var_tmf0_dn6 = assign105250_e157685_d_n6;
        var_tmf0_dn7 = assign105250_e157685_d_n7;
        var_tmf0_dn8 = assign105250_e157685_d_n8;
        var_tmf0_dn9 = assign105250_e157685_d_n9;
        var_tmf0_dn10 = assign105250_e157685_d_n10;
        var_tmf0_dn13 = assign105250_e157685_d_n13;

        let (assign105260_e157700, assign105260_e157700_d_n0, assign105260_e157700_d_n2, assign105260_e157700_d_n4, assign105260_e157700_d_n5, assign105260_e157700_d_n6, assign105260_e157700_d_n7, assign105260_e157700_d_n8, assign105260_e157700_d_n9, assign105260_e157700_d_n10, assign105260_e157700_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        let assign105260_e157694: f64 = (1e-25 * var_xmp);
        let assign105260_e157696: f64 = (assign105260_e157694 * var_dnm);
        let assign105260_e157698: f64 = (assign105260_e157696 / var_arg);
        (assign105260_e157698, ((((((1e-25 * var_xmp_dn0) * var_dnm) + (assign105260_e157694 * var_dnm_dn0)) * var_arg) - (assign105260_e157696 * var_arg_dn0)) / (var_arg * var_arg)), ((((((1e-25 * var_xmp_dn2) * var_dnm) + (assign105260_e157694 * var_dnm_dn2)) * var_arg) - (assign105260_e157696 * var_arg_dn2)) / (var_arg * var_arg)), ((((((1e-25 * var_xmp_dn4) * var_dnm) + (assign105260_e157694 * var_dnm_dn4)) * var_arg) - (assign105260_e157696 * var_arg_dn4)) / (var_arg * var_arg)), ((((((1e-25 * var_xmp_dn5) * var_dnm) + (assign105260_e157694 * var_dnm_dn5)) * var_arg) - (assign105260_e157696 * var_arg_dn5)) / (var_arg * var_arg)), ((((((1e-25 * var_xmp_dn6) * var_dnm) + (assign105260_e157694 * var_dnm_dn6)) * var_arg) - (assign105260_e157696 * var_arg_dn6)) / (var_arg * var_arg)), ((((((1e-25 * var_xmp_dn7) * var_dnm) + (assign105260_e157694 * var_dnm_dn7)) * var_arg) - (assign105260_e157696 * var_arg_dn7)) / (var_arg * var_arg)), ((((((1e-25 * var_xmp_dn8) * var_dnm) + (assign105260_e157694 * var_dnm_dn8)) * var_arg) - (assign105260_e157696 * var_arg_dn8)) / (var_arg * var_arg)), ((((((1e-25 * var_xmp_dn9) * var_dnm) + (assign105260_e157694 * var_dnm_dn9)) * var_arg) - (assign105260_e157696 * var_arg_dn9)) / (var_arg * var_arg)), ((((((1e-25 * var_xmp_dn10) * var_dnm) + (assign105260_e157694 * var_dnm_dn10)) * var_arg) - (assign105260_e157696 * var_arg_dn10)) / (var_arg * var_arg)), ((((((1e-25 * var_xmp_dn13) * var_dnm) + (assign105260_e157694 * var_dnm_dn13)) * var_arg) - (assign105260_e157696 * var_arg_dn13)) / (var_arg * var_arg)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign105260_e157700;
        var_t0_dn0 = assign105260_e157700_d_n0;
        var_t0_dn2 = assign105260_e157700_d_n2;
        var_t0_dn4 = assign105260_e157700_d_n4;
        var_t0_dn5 = assign105260_e157700_d_n5;
        var_t0_dn6 = assign105260_e157700_d_n6;
        var_t0_dn7 = assign105260_e157700_d_n7;
        var_t0_dn8 = assign105260_e157700_d_n8;
        var_t0_dn9 = assign105260_e157700_d_n9;
        var_t0_dn10 = assign105260_e157700_d_n10;
        var_t0_dn13 = assign105260_e157700_d_n13;

        let (assign105270_e157713, assign105270_e157713_d_n0, assign105270_e157713_d_n2, assign105270_e157713_d_n4, assign105270_e157713_d_n5, assign105270_e157713_d_n6, assign105270_e157713_d_n7, assign105270_e157713_d_n8, assign105270_e157713_d_n9, assign105270_e157713_d_n10, assign105270_e157713_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        let assign105270_e157709: f64 = 1e-25;
        let assign105270_e157711: f64 = (assign105270_e157709 - var_tmf0);
        (assign105270_e157711, (-var_tmf0_dn0), (-var_tmf0_dn2), (-var_tmf0_dn4), (-var_tmf0_dn5), (-var_tmf0_dn6), (-var_tmf0_dn7), (-var_tmf0_dn8), (-var_tmf0_dn9), (-var_tmf0_dn10), (-var_tmf0_dn13),)
    } else {
        (var_gd, var_gd_dn0, var_gd_dn2, var_gd_dn4, var_gd_dn5, var_gd_dn6, var_gd_dn7, var_gd_dn8, var_gd_dn9, var_gd_dn10, var_gd_dn13,)
    }
};
        var_gd = assign105270_e157713;
        var_gd_dn0 = assign105270_e157713_d_n0;
        var_gd_dn2 = assign105270_e157713_d_n2;
        var_gd_dn4 = assign105270_e157713_d_n4;
        var_gd_dn5 = assign105270_e157713_d_n5;
        var_gd_dn6 = assign105270_e157713_d_n6;
        var_gd_dn7 = assign105270_e157713_d_n7;
        var_gd_dn8 = assign105270_e157713_d_n8;
        var_gd_dn9 = assign105270_e157713_d_n9;
        var_gd_dn10 = assign105270_e157713_d_n10;
        var_gd_dn13 = assign105270_e157713_d_n13;

        let (assign105280_e157722, assign105280_e157722_d_n0, assign105280_e157722_d_n2, assign105280_e157722_d_n4, assign105280_e157722_d_n5, assign105280_e157722_d_n6, assign105280_e157722_d_n7, assign105280_e157722_d_n8, assign105280_e157722_d_n9, assign105280_e157722_d_n10, assign105280_e157722_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 != 0.0)) {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign105280_e157722;
        var_t0_dn0 = assign105280_e157722_d_n0;
        var_t0_dn2 = assign105280_e157722_d_n2;
        var_t0_dn4 = assign105280_e157722_d_n4;
        var_t0_dn5 = assign105280_e157722_d_n5;
        var_t0_dn6 = assign105280_e157722_d_n6;
        var_t0_dn7 = assign105280_e157722_d_n7;
        var_t0_dn8 = assign105280_e157722_d_n8;
        var_t0_dn9 = assign105280_e157722_d_n9;
        var_t0_dn10 = assign105280_e157722_d_n10;
        var_t0_dn13 = assign105280_e157722_d_n13;

        let (assign105290_e157732, assign105290_e157732_d_n0, assign105290_e157732_d_n2, assign105290_e157732_d_n4, assign105290_e157732_d_n5, assign105290_e157732_d_n6, assign105290_e157732_d_n7, assign105290_e157732_d_n8, assign105290_e157732_d_n9, assign105290_e157732_d_n10, assign105290_e157732_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 == 0.0)) {
        (var_gd, var_gd_dn0, var_gd_dn2, var_gd_dn4, var_gd_dn5, var_gd_dn6, var_gd_dn7, var_gd_dn8, var_gd_dn9, var_gd_dn10, var_gd_dn13,)
    } else {
        (var_gd, var_gd_dn0, var_gd_dn2, var_gd_dn4, var_gd_dn5, var_gd_dn6, var_gd_dn7, var_gd_dn8, var_gd_dn9, var_gd_dn10, var_gd_dn13,)
    }
};
        var_gd = assign105290_e157732;
        var_gd_dn0 = assign105290_e157732_d_n0;
        var_gd_dn2 = assign105290_e157732_d_n2;
        var_gd_dn4 = assign105290_e157732_d_n4;
        var_gd_dn5 = assign105290_e157732_d_n5;
        var_gd_dn6 = assign105290_e157732_d_n6;
        var_gd_dn7 = assign105290_e157732_d_n7;
        var_gd_dn8 = assign105290_e157732_d_n8;
        var_gd_dn9 = assign105290_e157732_d_n9;
        var_gd_dn10 = assign105290_e157732_d_n10;
        var_gd_dn13 = assign105290_e157732_d_n13;

        let (assign105300_e157742, assign105300_e157742_d_n0, assign105300_e157742_d_n2, assign105300_e157742_d_n4, assign105300_e157742_d_n5, assign105300_e157742_d_n6, assign105300_e157742_d_n7, assign105300_e157742_d_n8, assign105300_e157742_d_n9, assign105300_e157742_d_n10, assign105300_e157742_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2381 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign105300_e157742;
        var_t0_dn0 = assign105300_e157742_d_n0;
        var_t0_dn2 = assign105300_e157742_d_n2;
        var_t0_dn4 = assign105300_e157742_d_n4;
        var_t0_dn5 = assign105300_e157742_d_n5;
        var_t0_dn6 = assign105300_e157742_d_n6;
        var_t0_dn7 = assign105300_e157742_d_n7;
        var_t0_dn8 = assign105300_e157742_d_n8;
        var_t0_dn9 = assign105300_e157742_d_n9;
        var_t0_dn10 = assign105300_e157742_d_n10;
        var_t0_dn13 = assign105300_e157742_d_n13;

        let (assign105310_e157751, assign105310_e157751_d_n0, assign105310_e157751_d_n2, assign105310_e157751_d_n4, assign105310_e157751_d_n5, assign105310_e157751_d_n6, assign105310_e157751_d_n7, assign105310_e157751_d_n8, assign105310_e157751_d_n9, assign105310_e157751_d_n10, assign105310_e157751_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign105310_e157749: f64 = (1.0 / var_gd);
        (assign105310_e157749, (-(var_gd_dn0 / (var_gd * var_gd))), (-(var_gd_dn2 / (var_gd * var_gd))), (-(var_gd_dn4 / (var_gd * var_gd))), (-(var_gd_dn5 / (var_gd * var_gd))), (-(var_gd_dn6 / (var_gd * var_gd))), (-(var_gd_dn7 / (var_gd * var_gd))), (-(var_gd_dn8 / (var_gd * var_gd))), (-(var_gd_dn9 / (var_gd * var_gd))), (-(var_gd_dn10 / (var_gd * var_gd))), (-(var_gd_dn13 / (var_gd * var_gd))),)
    } else {
        (var_rdd, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn13,)
    }
};
        var_rdd = assign105310_e157751;
        var_rdd_dn0 = assign105310_e157751_d_n0;
        var_rdd_dn2 = assign105310_e157751_d_n2;
        var_rdd_dn4 = assign105310_e157751_d_n4;
        var_rdd_dn5 = assign105310_e157751_d_n5;
        var_rdd_dn6 = assign105310_e157751_d_n6;
        var_rdd_dn7 = assign105310_e157751_d_n7;
        var_rdd_dn8 = assign105310_e157751_d_n8;
        var_rdd_dn9 = assign105310_e157751_d_n9;
        var_rdd_dn10 = assign105310_e157751_d_n10;
        var_rdd_dn13 = assign105310_e157751_d_n13;

        let (assign105320_e157760, assign105320_e157760_d_n0, assign105320_e157760_d_n2, assign105320_e157760_d_n4, assign105320_e157760_d_n5, assign105320_e157760_d_n6, assign105320_e157760_d_n7, assign105320_e157760_d_n8, assign105320_e157760_d_n9, assign105320_e157760_d_n10, assign105320_e157760_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign105320_e157758: f64 = (var_rdd / var_weffld_nf);
        (assign105320_e157758, (var_rdd_dn0 / var_weffld_nf), (var_rdd_dn2 / var_weffld_nf), (var_rdd_dn4 / var_weffld_nf), (var_rdd_dn5 / var_weffld_nf), (var_rdd_dn6 / var_weffld_nf), (var_rdd_dn7 / var_weffld_nf), (var_rdd_dn8 / var_weffld_nf), (var_rdd_dn9 / var_weffld_nf), (var_rdd_dn10 / var_weffld_nf), (var_rdd_dn13 / var_weffld_nf),)
    } else {
        (var_rdd, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn13,)
    }
};
        var_rdd = assign105320_e157760;
        var_rdd_dn0 = assign105320_e157760_d_n0;
        var_rdd_dn2 = assign105320_e157760_d_n2;
        var_rdd_dn4 = assign105320_e157760_d_n4;
        var_rdd_dn5 = assign105320_e157760_d_n5;
        var_rdd_dn6 = assign105320_e157760_d_n6;
        var_rdd_dn7 = assign105320_e157760_d_n7;
        var_rdd_dn8 = assign105320_e157760_d_n8;
        var_rdd_dn9 = assign105320_e157760_d_n9;
        var_rdd_dn10 = assign105320_e157760_d_n10;
        var_rdd_dn13 = assign105320_e157760_d_n13;

        let assign105330_e157764: f64 = (1000000.0 - 1000.0);
        let assign105330_e157769: f64 = if ((var_rdd > assign105330_e157764) && (1000.0 >= 0.0)) { 1.0 } else { 0.0 };
        var_guard2387 = assign105330_e157769;

        let (assign105340_e157782, assign105340_e157782_d_n0, assign105340_e157782_d_n2, assign105340_e157782_d_n4, assign105340_e157782_d_n5, assign105340_e157782_d_n6, assign105340_e157782_d_n7, assign105340_e157782_d_n8, assign105340_e157782_d_n9, assign105340_e157782_d_n10, assign105340_e157782_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        let assign105340_e157778: f64 = (var_rdd - 1000000.0);
        let assign105340_e157780: f64 = (assign105340_e157778 + 1000.0);
        (assign105340_e157780, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn13,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn13,)
    }
};
        var_tmf1 = assign105340_e157782;
        var_tmf1_dn0 = assign105340_e157782_d_n0;
        var_tmf1_dn2 = assign105340_e157782_d_n2;
        var_tmf1_dn4 = assign105340_e157782_d_n4;
        var_tmf1_dn5 = assign105340_e157782_d_n5;
        var_tmf1_dn6 = assign105340_e157782_d_n6;
        var_tmf1_dn7 = assign105340_e157782_d_n7;
        var_tmf1_dn8 = assign105340_e157782_d_n8;
        var_tmf1_dn9 = assign105340_e157782_d_n9;
        var_tmf1_dn10 = assign105340_e157782_d_n10;
        var_tmf1_dn13 = assign105340_e157782_d_n13;

        let (assign105350_e157793, assign105350_e157793_d_n0, assign105350_e157793_d_n2, assign105350_e157793_d_n4, assign105350_e157793_d_n5, assign105350_e157793_d_n6, assign105350_e157793_d_n7, assign105350_e157793_d_n8, assign105350_e157793_d_n9, assign105350_e157793_d_n10, assign105350_e157793_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        let assign105350_e157791: f64 = (var_tmf1 * var_tmf1);
        (assign105350_e157791, ((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)), ((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)), ((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)), ((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)), ((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)), ((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)), ((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)), ((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)), ((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)), ((var_tmf1_dn13 * var_tmf1) + (var_tmf1 * var_tmf1_dn13)),)
    } else {
        (var_x2, var_x2_dn0, var_x2_dn2, var_x2_dn4, var_x2_dn5, var_x2_dn6, var_x2_dn7, var_x2_dn8, var_x2_dn9, var_x2_dn10, var_x2_dn13,)
    }
};
        var_x2 = assign105350_e157793;
        var_x2_dn0 = assign105350_e157793_d_n0;
        var_x2_dn2 = assign105350_e157793_d_n2;
        var_x2_dn4 = assign105350_e157793_d_n4;
        var_x2_dn5 = assign105350_e157793_d_n5;
        var_x2_dn6 = assign105350_e157793_d_n6;
        var_x2_dn7 = assign105350_e157793_d_n7;
        var_x2_dn8 = assign105350_e157793_d_n8;
        var_x2_dn9 = assign105350_e157793_d_n9;
        var_x2_dn10 = assign105350_e157793_d_n10;
        var_x2_dn13 = assign105350_e157793_d_n13;

        let (assign105360_e157804, assign105360_e157804_d_n0, assign105360_e157804_d_n2, assign105360_e157804_d_n4, assign105360_e157804_d_n5, assign105360_e157804_d_n6, assign105360_e157804_d_n7, assign105360_e157804_d_n8, assign105360_e157804_d_n9, assign105360_e157804_d_n10, assign105360_e157804_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        let assign105360_e157802: f64 = (1000.0 * 1000.0);
        (assign105360_e157802, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmax2, var_xmax2_dn0, var_xmax2_dn2, var_xmax2_dn4, var_xmax2_dn5, var_xmax2_dn6, var_xmax2_dn7, var_xmax2_dn8, var_xmax2_dn9, var_xmax2_dn10, var_xmax2_dn13,)
    }
};
        var_xmax2 = assign105360_e157804;
        var_xmax2_dn0 = assign105360_e157804_d_n0;
        var_xmax2_dn2 = assign105360_e157804_d_n2;
        var_xmax2_dn4 = assign105360_e157804_d_n4;
        var_xmax2_dn5 = assign105360_e157804_d_n5;
        var_xmax2_dn6 = assign105360_e157804_d_n6;
        var_xmax2_dn7 = assign105360_e157804_d_n7;
        var_xmax2_dn8 = assign105360_e157804_d_n8;
        var_xmax2_dn9 = assign105360_e157804_d_n9;
        var_xmax2_dn10 = assign105360_e157804_d_n10;
        var_xmax2_dn13 = assign105360_e157804_d_n13;

        let (assign105370_e157813, assign105370_e157813_d_n0, assign105370_e157813_d_n2, assign105370_e157813_d_n4, assign105370_e157813_d_n5, assign105370_e157813_d_n6, assign105370_e157813_d_n7, assign105370_e157813_d_n8, assign105370_e157813_d_n9, assign105370_e157813_d_n10, assign105370_e157813_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign105370_e157813;
        var_xp_dn0 = assign105370_e157813_d_n0;
        var_xp_dn2 = assign105370_e157813_d_n2;
        var_xp_dn4 = assign105370_e157813_d_n4;
        var_xp_dn5 = assign105370_e157813_d_n5;
        var_xp_dn6 = assign105370_e157813_d_n6;
        var_xp_dn7 = assign105370_e157813_d_n7;
        var_xp_dn8 = assign105370_e157813_d_n8;
        var_xp_dn9 = assign105370_e157813_d_n9;
        var_xp_dn10 = assign105370_e157813_d_n10;
        var_xp_dn13 = assign105370_e157813_d_n13;

        let (assign105380_e157822, assign105380_e157822_d_n0, assign105380_e157822_d_n2, assign105380_e157822_d_n4, assign105380_e157822_d_n5, assign105380_e157822_d_n6, assign105380_e157822_d_n7, assign105380_e157822_d_n8, assign105380_e157822_d_n9, assign105380_e157822_d_n10, assign105380_e157822_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign105380_e157822;
        var_xmp_dn0 = assign105380_e157822_d_n0;
        var_xmp_dn2 = assign105380_e157822_d_n2;
        var_xmp_dn4 = assign105380_e157822_d_n4;
        var_xmp_dn5 = assign105380_e157822_d_n5;
        var_xmp_dn6 = assign105380_e157822_d_n6;
        var_xmp_dn7 = assign105380_e157822_d_n7;
        var_xmp_dn8 = assign105380_e157822_d_n8;
        var_xmp_dn9 = assign105380_e157822_d_n9;
        var_xmp_dn10 = assign105380_e157822_d_n10;
        var_xmp_dn13 = assign105380_e157822_d_n13;

        let (assign105390_e157831,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign105390_e157831;

        let (assign105400_e157840,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        (0.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105400_e157840;

        let (assign105410_e157849, assign105410_e157849_d_n0, assign105410_e157849_d_n2, assign105410_e157849_d_n4, assign105410_e157849_d_n5, assign105410_e157849_d_n6, assign105410_e157849_d_n7, assign105410_e157849_d_n8, assign105410_e157849_d_n9, assign105410_e157849_d_n10, assign105410_e157849_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    }
};
        var_arg = assign105410_e157849;
        var_arg_dn0 = assign105410_e157849_d_n0;
        var_arg_dn2 = assign105410_e157849_d_n2;
        var_arg_dn4 = assign105410_e157849_d_n4;
        var_arg_dn5 = assign105410_e157849_d_n5;
        var_arg_dn6 = assign105410_e157849_d_n6;
        var_arg_dn7 = assign105410_e157849_d_n7;
        var_arg_dn8 = assign105410_e157849_d_n8;
        var_arg_dn9 = assign105410_e157849_d_n9;
        var_arg_dn10 = assign105410_e157849_d_n10;
        var_arg_dn13 = assign105410_e157849_d_n13;

        let (assign105420_e157858, assign105420_e157858_d_n0, assign105420_e157858_d_n2, assign105420_e157858_d_n4, assign105420_e157858_d_n5, assign105420_e157858_d_n6, assign105420_e157858_d_n7, assign105420_e157858_d_n8, assign105420_e157858_d_n9, assign105420_e157858_d_n10, assign105420_e157858_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign105420_e157858;
        var_dnm_dn0 = assign105420_e157858_d_n0;
        var_dnm_dn2 = assign105420_e157858_d_n2;
        var_dnm_dn4 = assign105420_e157858_d_n4;
        var_dnm_dn5 = assign105420_e157858_d_n5;
        var_dnm_dn6 = assign105420_e157858_d_n6;
        var_dnm_dn7 = assign105420_e157858_d_n7;
        var_dnm_dn8 = assign105420_e157858_d_n8;
        var_dnm_dn9 = assign105420_e157858_d_n9;
        var_dnm_dn10 = assign105420_e157858_d_n10;
        var_dnm_dn13 = assign105420_e157858_d_n13;

        let (assign105430_e157869, assign105430_e157869_d_n0, assign105430_e157869_d_n2, assign105430_e157869_d_n4, assign105430_e157869_d_n5, assign105430_e157869_d_n6, assign105430_e157869_d_n7, assign105430_e157869_d_n8, assign105430_e157869_d_n9, assign105430_e157869_d_n10, assign105430_e157869_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        let assign105430_e157867: f64 = (var_xp * var_x2);
        (assign105430_e157867, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn13 * var_x2) + (var_xp * var_x2_dn13)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign105430_e157869;
        var_xp_dn0 = assign105430_e157869_d_n0;
        var_xp_dn2 = assign105430_e157869_d_n2;
        var_xp_dn4 = assign105430_e157869_d_n4;
        var_xp_dn5 = assign105430_e157869_d_n5;
        var_xp_dn6 = assign105430_e157869_d_n6;
        var_xp_dn7 = assign105430_e157869_d_n7;
        var_xp_dn8 = assign105430_e157869_d_n8;
        var_xp_dn9 = assign105430_e157869_d_n9;
        var_xp_dn10 = assign105430_e157869_d_n10;
        var_xp_dn13 = assign105430_e157869_d_n13;

        let (assign105440_e157880, assign105440_e157880_d_n0, assign105440_e157880_d_n2, assign105440_e157880_d_n4, assign105440_e157880_d_n5, assign105440_e157880_d_n6, assign105440_e157880_d_n7, assign105440_e157880_d_n8, assign105440_e157880_d_n9, assign105440_e157880_d_n10, assign105440_e157880_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        let assign105440_e157878: f64 = (var_xmp * var_xmax2);
        (assign105440_e157878, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn13 * var_xmax2) + (var_xmp * var_xmax2_dn13)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign105440_e157880;
        var_xmp_dn0 = assign105440_e157880_d_n0;
        var_xmp_dn2 = assign105440_e157880_d_n2;
        var_xmp_dn4 = assign105440_e157880_d_n4;
        var_xmp_dn5 = assign105440_e157880_d_n5;
        var_xmp_dn6 = assign105440_e157880_d_n6;
        var_xmp_dn7 = assign105440_e157880_d_n7;
        var_xmp_dn8 = assign105440_e157880_d_n8;
        var_xmp_dn9 = assign105440_e157880_d_n9;
        var_xmp_dn10 = assign105440_e157880_d_n10;
        var_xmp_dn13 = assign105440_e157880_d_n13;

        let (assign105450_e157891, assign105450_e157891_d_n0, assign105450_e157891_d_n2, assign105450_e157891_d_n4, assign105450_e157891_d_n5, assign105450_e157891_d_n6, assign105450_e157891_d_n7, assign105450_e157891_d_n8, assign105450_e157891_d_n9, assign105450_e157891_d_n10, assign105450_e157891_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        let assign105450_e157889: f64 = (var_xp * var_x2);
        (assign105450_e157889, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn13 * var_x2) + (var_xp * var_x2_dn13)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign105450_e157891;
        var_xp_dn0 = assign105450_e157891_d_n0;
        var_xp_dn2 = assign105450_e157891_d_n2;
        var_xp_dn4 = assign105450_e157891_d_n4;
        var_xp_dn5 = assign105450_e157891_d_n5;
        var_xp_dn6 = assign105450_e157891_d_n6;
        var_xp_dn7 = assign105450_e157891_d_n7;
        var_xp_dn8 = assign105450_e157891_d_n8;
        var_xp_dn9 = assign105450_e157891_d_n9;
        var_xp_dn10 = assign105450_e157891_d_n10;
        var_xp_dn13 = assign105450_e157891_d_n13;

        let (assign105460_e157902, assign105460_e157902_d_n0, assign105460_e157902_d_n2, assign105460_e157902_d_n4, assign105460_e157902_d_n5, assign105460_e157902_d_n6, assign105460_e157902_d_n7, assign105460_e157902_d_n8, assign105460_e157902_d_n9, assign105460_e157902_d_n10, assign105460_e157902_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        let assign105460_e157900: f64 = (var_xmp * var_xmax2);
        (assign105460_e157900, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn13 * var_xmax2) + (var_xmp * var_xmax2_dn13)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign105460_e157902;
        var_xmp_dn0 = assign105460_e157902_d_n0;
        var_xmp_dn2 = assign105460_e157902_d_n2;
        var_xmp_dn4 = assign105460_e157902_d_n4;
        var_xmp_dn5 = assign105460_e157902_d_n5;
        var_xmp_dn6 = assign105460_e157902_d_n6;
        var_xmp_dn7 = assign105460_e157902_d_n7;
        var_xmp_dn8 = assign105460_e157902_d_n8;
        var_xmp_dn9 = assign105460_e157902_d_n9;
        var_xmp_dn10 = assign105460_e157902_d_n10;
        var_xmp_dn13 = assign105460_e157902_d_n13;

        let (assign105470_e157913, assign105470_e157913_d_n0, assign105470_e157913_d_n2, assign105470_e157913_d_n4, assign105470_e157913_d_n5, assign105470_e157913_d_n6, assign105470_e157913_d_n7, assign105470_e157913_d_n8, assign105470_e157913_d_n9, assign105470_e157913_d_n10, assign105470_e157913_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        let assign105470_e157911: f64 = (var_xp + var_xmp);
        (assign105470_e157911, (var_xp_dn0 + var_xmp_dn0), (var_xp_dn2 + var_xmp_dn2), (var_xp_dn4 + var_xmp_dn4), (var_xp_dn5 + var_xmp_dn5), (var_xp_dn6 + var_xmp_dn6), (var_xp_dn7 + var_xmp_dn7), (var_xp_dn8 + var_xmp_dn8), (var_xp_dn9 + var_xmp_dn9), (var_xp_dn10 + var_xmp_dn10), (var_xp_dn13 + var_xmp_dn13),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    }
};
        var_arg = assign105470_e157913;
        var_arg_dn0 = assign105470_e157913_d_n0;
        var_arg_dn2 = assign105470_e157913_d_n2;
        var_arg_dn4 = assign105470_e157913_d_n4;
        var_arg_dn5 = assign105470_e157913_d_n5;
        var_arg_dn6 = assign105470_e157913_d_n6;
        var_arg_dn7 = assign105470_e157913_d_n7;
        var_arg_dn8 = assign105470_e157913_d_n8;
        var_arg_dn9 = assign105470_e157913_d_n9;
        var_arg_dn10 = assign105470_e157913_d_n10;
        var_arg_dn13 = assign105470_e157913_d_n13;

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
        *var_gd_slot = var_gd;
        *var_gd_dn0_slot = var_gd_dn0;
        *var_gd_dn10_slot = var_gd_dn10;
        *var_gd_dn13_slot = var_gd_dn13;
        *var_gd_dn2_slot = var_gd_dn2;
        *var_gd_dn4_slot = var_gd_dn4;
        *var_gd_dn5_slot = var_gd_dn5;
        *var_gd_dn6_slot = var_gd_dn6;
        *var_gd_dn7_slot = var_gd_dn7;
        *var_gd_dn8_slot = var_gd_dn8;
        *var_gd_dn9_slot = var_gd_dn9;
        *var_guard2387_slot = var_guard2387;
        *var_m0_slot = var_m0;
        *var_mm_slot = var_mm;
        *var_rdd_slot = var_rdd;
        *var_rdd_dn0_slot = var_rdd_dn0;
        *var_rdd_dn10_slot = var_rdd_dn10;
        *var_rdd_dn13_slot = var_rdd_dn13;
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
        *var_t0_dn13_slot = var_t0_dn13;
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
        *var_tmf0_dn13_slot = var_tmf0_dn13;
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
        *var_tmf1_dn13_slot = var_tmf1_dn13;
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
        *var_x2_dn13_slot = var_x2_dn13;
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
        *var_xmax2_dn13_slot = var_xmax2_dn13;
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
        *var_xmp_dn13_slot = var_xmp_dn13;
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
        *var_xp_dn13_slot = var_xp_dn13;
        *var_xp_dn2_slot = var_xp_dn2;
        *var_xp_dn4_slot = var_xp_dn4;
        *var_xp_dn5_slot = var_xp_dn5;
        *var_xp_dn6_slot = var_xp_dn6;
        *var_xp_dn7_slot = var_xp_dn7;
        *var_xp_dn8_slot = var_xp_dn8;
        *var_xp_dn9_slot = var_xp_dn9;
    }

    pub(super) fn stamp_transient_block_373(
        p: &Parameters,
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
        var_guard2336: f64,
        var_guard2356: f64,
        var_guard2387: f64,
        var_ldrift0: f64,
        var_mfactor: f64,
        var_mks_nsubsub: f64,
        var_rd0: f64,
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
        var_uc_nover: f64,
        var_vdsemodenml: f64,
        var_wdep: f64,
        var_wdep_dn0: f64,
        var_wdep_dn10: f64,
        var_wdep_dn13: f64,
        var_wdep_dn2: f64,
        var_wdep_dn4: f64,
        var_wdep_dn5: f64,
        var_wdep_dn6: f64,
        var_wdep_dn7: f64,
        var_wdep_dn8: f64,
        var_wdep_dn9: f64,
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
        var_ddriftld_slot: &mut f64,
        var_ddriftld_dn0_slot: &mut f64,
        var_ddriftld_dn10_slot: &mut f64,
        var_ddriftld_dn13_slot: &mut f64,
        var_ddriftld_dn2_slot: &mut f64,
        var_ddriftld_dn4_slot: &mut f64,
        var_ddriftld_dn5_slot: &mut f64,
        var_ddriftld_dn6_slot: &mut f64,
        var_ddriftld_dn7_slot: &mut f64,
        var_ddriftld_dn8_slot: &mut f64,
        var_ddriftld_dn9_slot: &mut f64,
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
        var_guard2388_slot: &mut f64,
        var_guard2389_slot: &mut f64,
        var_guard2390_slot: &mut f64,
        var_guard2391_slot: &mut f64,
        var_guard2392_slot: &mut f64,
        var_guard2393_slot: &mut f64,
        var_guard2395_slot: &mut f64,
        var_guard2396_slot: &mut f64,
        var_guard2397_slot: &mut f64,
        var_guard2398_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_mm_slot: &mut f64,
        var_rdd_slot: &mut f64,
        var_rdd_dn0_slot: &mut f64,
        var_rdd_dn10_slot: &mut f64,
        var_rdd_dn13_slot: &mut f64,
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
        var_rdde_dn13_slot: &mut f64,
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
        var_rsd_dn13_slot: &mut f64,
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
        var_rsde_dn13_slot: &mut f64,
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
        var_t0_dn13_slot: &mut f64,
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
        var_tmf0_dn13_slot: &mut f64,
        var_tmf0_dn2_slot: &mut f64,
        var_tmf0_dn4_slot: &mut f64,
        var_tmf0_dn5_slot: &mut f64,
        var_tmf0_dn6_slot: &mut f64,
        var_tmf0_dn7_slot: &mut f64,
        var_tmf0_dn8_slot: &mut f64,
        var_tmf0_dn9_slot: &mut f64,
    ) {
        let mut var_ddriftld: f64 = *var_ddriftld_slot;
        let mut var_ddriftld_dn0: f64 = *var_ddriftld_dn0_slot;
        let mut var_ddriftld_dn10: f64 = *var_ddriftld_dn10_slot;
        let mut var_ddriftld_dn13: f64 = *var_ddriftld_dn13_slot;
        let mut var_ddriftld_dn2: f64 = *var_ddriftld_dn2_slot;
        let mut var_ddriftld_dn4: f64 = *var_ddriftld_dn4_slot;
        let mut var_ddriftld_dn5: f64 = *var_ddriftld_dn5_slot;
        let mut var_ddriftld_dn6: f64 = *var_ddriftld_dn6_slot;
        let mut var_ddriftld_dn7: f64 = *var_ddriftld_dn7_slot;
        let mut var_ddriftld_dn8: f64 = *var_ddriftld_dn8_slot;
        let mut var_ddriftld_dn9: f64 = *var_ddriftld_dn9_slot;
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
        let mut var_guard2388: f64 = *var_guard2388_slot;
        let mut var_guard2389: f64 = *var_guard2389_slot;
        let mut var_guard2390: f64 = *var_guard2390_slot;
        let mut var_guard2391: f64 = *var_guard2391_slot;
        let mut var_guard2392: f64 = *var_guard2392_slot;
        let mut var_guard2393: f64 = *var_guard2393_slot;
        let mut var_guard2395: f64 = *var_guard2395_slot;
        let mut var_guard2396: f64 = *var_guard2396_slot;
        let mut var_guard2397: f64 = *var_guard2397_slot;
        let mut var_guard2398: f64 = *var_guard2398_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_mm: f64 = *var_mm_slot;
        let mut var_rdd: f64 = *var_rdd_slot;
        let mut var_rdd_dn0: f64 = *var_rdd_dn0_slot;
        let mut var_rdd_dn10: f64 = *var_rdd_dn10_slot;
        let mut var_rdd_dn13: f64 = *var_rdd_dn13_slot;
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
        let mut var_rdde_dn13: f64 = *var_rdde_dn13_slot;
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
        let mut var_rsd_dn13: f64 = *var_rsd_dn13_slot;
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
        let mut var_rsde_dn13: f64 = *var_rsde_dn13_slot;
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
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
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
        let mut var_tmf0_dn13: f64 = *var_tmf0_dn13_slot;
        let mut var_tmf0_dn2: f64 = *var_tmf0_dn2_slot;
        let mut var_tmf0_dn4: f64 = *var_tmf0_dn4_slot;
        let mut var_tmf0_dn5: f64 = *var_tmf0_dn5_slot;
        let mut var_tmf0_dn6: f64 = *var_tmf0_dn6_slot;
        let mut var_tmf0_dn7: f64 = *var_tmf0_dn7_slot;
        let mut var_tmf0_dn8: f64 = *var_tmf0_dn8_slot;
        let mut var_tmf0_dn9: f64 = *var_tmf0_dn9_slot;

        let (assign105480_e157922, assign105480_e157922_d_n0, assign105480_e157922_d_n2, assign105480_e157922_d_n4, assign105480_e157922_d_n5, assign105480_e157922_d_n6, assign105480_e157922_d_n7, assign105480_e157922_d_n8, assign105480_e157922_d_n9, assign105480_e157922_d_n10, assign105480_e157922_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign105480_e157922;
        var_dnm_dn0 = assign105480_e157922_d_n0;
        var_dnm_dn2 = assign105480_e157922_d_n2;
        var_dnm_dn4 = assign105480_e157922_d_n4;
        var_dnm_dn5 = assign105480_e157922_d_n5;
        var_dnm_dn6 = assign105480_e157922_d_n6;
        var_dnm_dn7 = assign105480_e157922_d_n7;
        var_dnm_dn8 = assign105480_e157922_d_n8;
        var_dnm_dn9 = assign105480_e157922_d_n9;
        var_dnm_dn10 = assign105480_e157922_d_n10;
        var_dnm_dn13 = assign105480_e157922_d_n13;

        let assign105490_e157937: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        var_guard2388 = assign105490_e157937;

        let assign105500_e157940: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard2389 = assign105500_e157940;

        let (assign105510_e157953,) = {
    if (((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) && (var_guard2388 != 0.0)) && (var_guard2389 != 0.0)) {
        (1.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105510_e157953;

        let assign105520_e157956: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard2390 = assign105520_e157956;

        let (assign105530_e157972,) = {
    if ((((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) && (var_guard2388 != 0.0)) && (var_guard2389 == 0.0)) && (var_guard2390 != 0.0)) {
        (2.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105530_e157972;

        let assign105540_e157975: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        var_guard2391 = assign105540_e157975;

        let (assign105550_e157994,) = {
    if (((((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) && (var_guard2388 != 0.0)) && (var_guard2389 == 0.0)) && (var_guard2390 == 0.0)) && (var_guard2391 != 0.0)) {
        (3.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105550_e157994;

        let assign105560_e157997: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        var_guard2392 = assign105560_e157997;

        let (assign105570_e158019,) = {
    if ((((((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) && (var_guard2388 != 0.0)) && (var_guard2389 == 0.0)) && (var_guard2390 == 0.0)) && (var_guard2391 == 0.0)) && (var_guard2392 != 0.0)) {
        (4.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign105570_e158019;

        let (assign105580_e158030,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) && (var_guard2388 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign105580_e158030;

        let mut assign105590_loop_guard: usize = 0;
        while {
            let assign105590_cond_e158042: f64 = if (((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) && (var_guard2388 != 0.0)) && (var_m0 < var_mm)) { 1.0 } else { 0.0 };
            assign105590_cond_e158042 != 0.0
        } {
            assign105590_loop_guard += 1;
            assert!(assign105590_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign105590_body0_e158054, assign105590_body0_e158054_d_n0, assign105590_body0_e158054_d_n2, assign105590_body0_e158054_d_n4, assign105590_body0_e158054_d_n5, assign105590_body0_e158054_d_n6, assign105590_body0_e158054_d_n7, assign105590_body0_e158054_d_n8, assign105590_body0_e158054_d_n9, assign105590_body0_e158054_d_n10, assign105590_body0_e158054_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) && (var_guard2388 != 0.0)) {
        let assign105590_body0_e158052: f64 = (var_dnm).sqrt();
        (assign105590_body0_e158052, (var_dnm_dn0 / (2.0 * assign105590_body0_e158052)), (var_dnm_dn2 / (2.0 * assign105590_body0_e158052)), (var_dnm_dn4 / (2.0 * assign105590_body0_e158052)), (var_dnm_dn5 / (2.0 * assign105590_body0_e158052)), (var_dnm_dn6 / (2.0 * assign105590_body0_e158052)), (var_dnm_dn7 / (2.0 * assign105590_body0_e158052)), (var_dnm_dn8 / (2.0 * assign105590_body0_e158052)), (var_dnm_dn9 / (2.0 * assign105590_body0_e158052)), (var_dnm_dn10 / (2.0 * assign105590_body0_e158052)), (var_dnm_dn13 / (2.0 * assign105590_body0_e158052)),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
            var_dnm = assign105590_body0_e158054;
            var_dnm_dn0 = assign105590_body0_e158054_d_n0;
            var_dnm_dn2 = assign105590_body0_e158054_d_n2;
            var_dnm_dn4 = assign105590_body0_e158054_d_n4;
            var_dnm_dn5 = assign105590_body0_e158054_d_n5;
            var_dnm_dn6 = assign105590_body0_e158054_d_n6;
            var_dnm_dn7 = assign105590_body0_e158054_d_n7;
            var_dnm_dn8 = assign105590_body0_e158054_d_n8;
            var_dnm_dn9 = assign105590_body0_e158054_d_n9;
            var_dnm_dn10 = assign105590_body0_e158054_d_n10;
            var_dnm_dn13 = assign105590_body0_e158054_d_n13;
            let (assign105590_body1_e158067,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) && (var_guard2388 != 0.0)) {
        let assign105590_body1_e158065: f64 = (var_m0 + 1.0);
        (assign105590_body1_e158065,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign105590_body1_e158067;
        }

        let (assign105600_e158090, assign105600_e158090_d_n0, assign105600_e158090_d_n2, assign105600_e158090_d_n4, assign105600_e158090_d_n5, assign105600_e158090_d_n6, assign105600_e158090_d_n7, assign105600_e158090_d_n8, assign105600_e158090_d_n9, assign105600_e158090_d_n10, assign105600_e158090_d_n13,) = {
    if ((((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) && (var_guard2388 == 0.0)) {
        let (assign105600_e158088, assign105600_e158088_d_n0, assign105600_e158088_d_n2, assign105600_e158088_d_n4, assign105600_e158088_d_n5, assign105600_e158088_d_n6, assign105600_e158088_d_n7, assign105600_e158088_d_n8, assign105600_e158088_d_n9, assign105600_e158088_d_n10, assign105600_e158088_d_n13,) = {
            if (var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign105600_e158085: f64 = (2.0 * 2.0);
                let assign105600_e158086: f64 = (1.0 / assign105600_e158085);
                let assign105600_e158087: f64 = (var_dnm).powf(assign105600_e158086);
                (assign105600_e158087, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((var_dnm).powf(assign105600_e158086 - 1.0) * var_dnm_dn0)) } } else { (assign105600_e158087 * (assign105600_e158086 * (var_dnm_dn0 / var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((var_dnm).powf(assign105600_e158086 - 1.0) * var_dnm_dn2)) } } else { (assign105600_e158087 * (assign105600_e158086 * (var_dnm_dn2 / var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((var_dnm).powf(assign105600_e158086 - 1.0) * var_dnm_dn4)) } } else { (assign105600_e158087 * (assign105600_e158086 * (var_dnm_dn4 / var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((var_dnm).powf(assign105600_e158086 - 1.0) * var_dnm_dn5)) } } else { (assign105600_e158087 * (assign105600_e158086 * (var_dnm_dn5 / var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((var_dnm).powf(assign105600_e158086 - 1.0) * var_dnm_dn6)) } } else { (assign105600_e158087 * (assign105600_e158086 * (var_dnm_dn6 / var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((var_dnm).powf(assign105600_e158086 - 1.0) * var_dnm_dn7)) } } else { (assign105600_e158087 * (assign105600_e158086 * (var_dnm_dn7 / var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((var_dnm).powf(assign105600_e158086 - 1.0) * var_dnm_dn8)) } } else { (assign105600_e158087 * (assign105600_e158086 * (var_dnm_dn8 / var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((var_dnm).powf(assign105600_e158086 - 1.0) * var_dnm_dn9)) } } else { (assign105600_e158087 * (assign105600_e158086 * (var_dnm_dn9 / var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((var_dnm).powf(assign105600_e158086 - 1.0) * var_dnm_dn10)) } } else { (assign105600_e158087 * (assign105600_e158086 * (var_dnm_dn10 / var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((var_dnm).powf(assign105600_e158086 - 1.0) * var_dnm_dn13)) } } else { (assign105600_e158087 * (assign105600_e158086 * (var_dnm_dn13 / var_dnm))) },)
            }
        };
        (assign105600_e158088, assign105600_e158088_d_n0, assign105600_e158088_d_n2, assign105600_e158088_d_n4, assign105600_e158088_d_n5, assign105600_e158088_d_n6, assign105600_e158088_d_n7, assign105600_e158088_d_n8, assign105600_e158088_d_n9, assign105600_e158088_d_n10, assign105600_e158088_d_n13,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign105600_e158090;
        var_dnm_dn0 = assign105600_e158090_d_n0;
        var_dnm_dn2 = assign105600_e158090_d_n2;
        var_dnm_dn4 = assign105600_e158090_d_n4;
        var_dnm_dn5 = assign105600_e158090_d_n5;
        var_dnm_dn6 = assign105600_e158090_d_n6;
        var_dnm_dn7 = assign105600_e158090_d_n7;
        var_dnm_dn8 = assign105600_e158090_d_n8;
        var_dnm_dn9 = assign105600_e158090_d_n9;
        var_dnm_dn10 = assign105600_e158090_d_n10;
        var_dnm_dn13 = assign105600_e158090_d_n13;

        let (assign105610_e158101, assign105610_e158101_d_n0, assign105610_e158101_d_n2, assign105610_e158101_d_n4, assign105610_e158101_d_n5, assign105610_e158101_d_n6, assign105610_e158101_d_n7, assign105610_e158101_d_n8, assign105610_e158101_d_n9, assign105610_e158101_d_n10, assign105610_e158101_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        let assign105610_e158099: f64 = (1.0 / var_dnm);
        (assign105610_e158099, (-(var_dnm_dn0 / (var_dnm * var_dnm))), (-(var_dnm_dn2 / (var_dnm * var_dnm))), (-(var_dnm_dn4 / (var_dnm * var_dnm))), (-(var_dnm_dn5 / (var_dnm * var_dnm))), (-(var_dnm_dn6 / (var_dnm * var_dnm))), (-(var_dnm_dn7 / (var_dnm * var_dnm))), (-(var_dnm_dn8 / (var_dnm * var_dnm))), (-(var_dnm_dn9 / (var_dnm * var_dnm))), (-(var_dnm_dn10 / (var_dnm * var_dnm))), (-(var_dnm_dn13 / (var_dnm * var_dnm))),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign105610_e158101;
        var_dnm_dn0 = assign105610_e158101_d_n0;
        var_dnm_dn2 = assign105610_e158101_d_n2;
        var_dnm_dn4 = assign105610_e158101_d_n4;
        var_dnm_dn5 = assign105610_e158101_d_n5;
        var_dnm_dn6 = assign105610_e158101_d_n6;
        var_dnm_dn7 = assign105610_e158101_d_n7;
        var_dnm_dn8 = assign105610_e158101_d_n8;
        var_dnm_dn9 = assign105610_e158101_d_n9;
        var_dnm_dn10 = assign105610_e158101_d_n10;
        var_dnm_dn13 = assign105610_e158101_d_n13;

        let (assign105620_e158114, assign105620_e158114_d_n0, assign105620_e158114_d_n2, assign105620_e158114_d_n4, assign105620_e158114_d_n5, assign105620_e158114_d_n6, assign105620_e158114_d_n7, assign105620_e158114_d_n8, assign105620_e158114_d_n9, assign105620_e158114_d_n10, assign105620_e158114_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        let assign105620_e158110: f64 = (var_tmf1 * 1000.0);
        let assign105620_e158112: f64 = (assign105620_e158110 * var_dnm);
        (assign105620_e158112, (((var_tmf1_dn0 * 1000.0) * var_dnm) + (assign105620_e158110 * var_dnm_dn0)), (((var_tmf1_dn2 * 1000.0) * var_dnm) + (assign105620_e158110 * var_dnm_dn2)), (((var_tmf1_dn4 * 1000.0) * var_dnm) + (assign105620_e158110 * var_dnm_dn4)), (((var_tmf1_dn5 * 1000.0) * var_dnm) + (assign105620_e158110 * var_dnm_dn5)), (((var_tmf1_dn6 * 1000.0) * var_dnm) + (assign105620_e158110 * var_dnm_dn6)), (((var_tmf1_dn7 * 1000.0) * var_dnm) + (assign105620_e158110 * var_dnm_dn7)), (((var_tmf1_dn8 * 1000.0) * var_dnm) + (assign105620_e158110 * var_dnm_dn8)), (((var_tmf1_dn9 * 1000.0) * var_dnm) + (assign105620_e158110 * var_dnm_dn9)), (((var_tmf1_dn10 * 1000.0) * var_dnm) + (assign105620_e158110 * var_dnm_dn10)), (((var_tmf1_dn13 * 1000.0) * var_dnm) + (assign105620_e158110 * var_dnm_dn13)),)
    } else {
        (var_tmf0, var_tmf0_dn0, var_tmf0_dn2, var_tmf0_dn4, var_tmf0_dn5, var_tmf0_dn6, var_tmf0_dn7, var_tmf0_dn8, var_tmf0_dn9, var_tmf0_dn10, var_tmf0_dn13,)
    }
};
        var_tmf0 = assign105620_e158114;
        var_tmf0_dn0 = assign105620_e158114_d_n0;
        var_tmf0_dn2 = assign105620_e158114_d_n2;
        var_tmf0_dn4 = assign105620_e158114_d_n4;
        var_tmf0_dn5 = assign105620_e158114_d_n5;
        var_tmf0_dn6 = assign105620_e158114_d_n6;
        var_tmf0_dn7 = assign105620_e158114_d_n7;
        var_tmf0_dn8 = assign105620_e158114_d_n8;
        var_tmf0_dn9 = assign105620_e158114_d_n9;
        var_tmf0_dn10 = assign105620_e158114_d_n10;
        var_tmf0_dn13 = assign105620_e158114_d_n13;

        let (assign105630_e158129, assign105630_e158129_d_n0, assign105630_e158129_d_n2, assign105630_e158129_d_n4, assign105630_e158129_d_n5, assign105630_e158129_d_n6, assign105630_e158129_d_n7, assign105630_e158129_d_n8, assign105630_e158129_d_n9, assign105630_e158129_d_n10, assign105630_e158129_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        let assign105630_e158123: f64 = (1000.0 * var_xmp);
        let assign105630_e158125: f64 = (assign105630_e158123 * var_dnm);
        let assign105630_e158127: f64 = (assign105630_e158125 / var_arg);
        (assign105630_e158127, ((((((1000.0 * var_xmp_dn0) * var_dnm) + (assign105630_e158123 * var_dnm_dn0)) * var_arg) - (assign105630_e158125 * var_arg_dn0)) / (var_arg * var_arg)), ((((((1000.0 * var_xmp_dn2) * var_dnm) + (assign105630_e158123 * var_dnm_dn2)) * var_arg) - (assign105630_e158125 * var_arg_dn2)) / (var_arg * var_arg)), ((((((1000.0 * var_xmp_dn4) * var_dnm) + (assign105630_e158123 * var_dnm_dn4)) * var_arg) - (assign105630_e158125 * var_arg_dn4)) / (var_arg * var_arg)), ((((((1000.0 * var_xmp_dn5) * var_dnm) + (assign105630_e158123 * var_dnm_dn5)) * var_arg) - (assign105630_e158125 * var_arg_dn5)) / (var_arg * var_arg)), ((((((1000.0 * var_xmp_dn6) * var_dnm) + (assign105630_e158123 * var_dnm_dn6)) * var_arg) - (assign105630_e158125 * var_arg_dn6)) / (var_arg * var_arg)), ((((((1000.0 * var_xmp_dn7) * var_dnm) + (assign105630_e158123 * var_dnm_dn7)) * var_arg) - (assign105630_e158125 * var_arg_dn7)) / (var_arg * var_arg)), ((((((1000.0 * var_xmp_dn8) * var_dnm) + (assign105630_e158123 * var_dnm_dn8)) * var_arg) - (assign105630_e158125 * var_arg_dn8)) / (var_arg * var_arg)), ((((((1000.0 * var_xmp_dn9) * var_dnm) + (assign105630_e158123 * var_dnm_dn9)) * var_arg) - (assign105630_e158125 * var_arg_dn9)) / (var_arg * var_arg)), ((((((1000.0 * var_xmp_dn10) * var_dnm) + (assign105630_e158123 * var_dnm_dn10)) * var_arg) - (assign105630_e158125 * var_arg_dn10)) / (var_arg * var_arg)), ((((((1000.0 * var_xmp_dn13) * var_dnm) + (assign105630_e158123 * var_dnm_dn13)) * var_arg) - (assign105630_e158125 * var_arg_dn13)) / (var_arg * var_arg)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign105630_e158129;
        var_t0_dn0 = assign105630_e158129_d_n0;
        var_t0_dn2 = assign105630_e158129_d_n2;
        var_t0_dn4 = assign105630_e158129_d_n4;
        var_t0_dn5 = assign105630_e158129_d_n5;
        var_t0_dn6 = assign105630_e158129_d_n6;
        var_t0_dn7 = assign105630_e158129_d_n7;
        var_t0_dn8 = assign105630_e158129_d_n8;
        var_t0_dn9 = assign105630_e158129_d_n9;
        var_t0_dn10 = assign105630_e158129_d_n10;
        var_t0_dn13 = assign105630_e158129_d_n13;

        let (assign105640_e158142, assign105640_e158142_d_n0, assign105640_e158142_d_n2, assign105640_e158142_d_n4, assign105640_e158142_d_n5, assign105640_e158142_d_n6, assign105640_e158142_d_n7, assign105640_e158142_d_n8, assign105640_e158142_d_n9, assign105640_e158142_d_n10, assign105640_e158142_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        let assign105640_e158138: f64 = (1000000.0 - 1000.0);
        let assign105640_e158140: f64 = (assign105640_e158138 + var_tmf0);
        (assign105640_e158140, var_tmf0_dn0, var_tmf0_dn2, var_tmf0_dn4, var_tmf0_dn5, var_tmf0_dn6, var_tmf0_dn7, var_tmf0_dn8, var_tmf0_dn9, var_tmf0_dn10, var_tmf0_dn13,)
    } else {
        (var_rdd, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn13,)
    }
};
        var_rdd = assign105640_e158142;
        var_rdd_dn0 = assign105640_e158142_d_n0;
        var_rdd_dn2 = assign105640_e158142_d_n2;
        var_rdd_dn4 = assign105640_e158142_d_n4;
        var_rdd_dn5 = assign105640_e158142_d_n5;
        var_rdd_dn6 = assign105640_e158142_d_n6;
        var_rdd_dn7 = assign105640_e158142_d_n7;
        var_rdd_dn8 = assign105640_e158142_d_n8;
        var_rdd_dn9 = assign105640_e158142_d_n9;
        var_rdd_dn10 = assign105640_e158142_d_n10;
        var_rdd_dn13 = assign105640_e158142_d_n13;

        let (assign105650_e158151, assign105650_e158151_d_n0, assign105650_e158151_d_n2, assign105650_e158151_d_n4, assign105650_e158151_d_n5, assign105650_e158151_d_n6, assign105650_e158151_d_n7, assign105650_e158151_d_n8, assign105650_e158151_d_n9, assign105650_e158151_d_n10, assign105650_e158151_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 != 0.0)) {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign105650_e158151;
        var_t0_dn0 = assign105650_e158151_d_n0;
        var_t0_dn2 = assign105650_e158151_d_n2;
        var_t0_dn4 = assign105650_e158151_d_n4;
        var_t0_dn5 = assign105650_e158151_d_n5;
        var_t0_dn6 = assign105650_e158151_d_n6;
        var_t0_dn7 = assign105650_e158151_d_n7;
        var_t0_dn8 = assign105650_e158151_d_n8;
        var_t0_dn9 = assign105650_e158151_d_n9;
        var_t0_dn10 = assign105650_e158151_d_n10;
        var_t0_dn13 = assign105650_e158151_d_n13;

        let (assign105660_e158161, assign105660_e158161_d_n0, assign105660_e158161_d_n2, assign105660_e158161_d_n4, assign105660_e158161_d_n5, assign105660_e158161_d_n6, assign105660_e158161_d_n7, assign105660_e158161_d_n8, assign105660_e158161_d_n9, assign105660_e158161_d_n10, assign105660_e158161_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 == 0.0)) {
        (var_rdd, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn13,)
    } else {
        (var_rdd, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn13,)
    }
};
        var_rdd = assign105660_e158161;
        var_rdd_dn0 = assign105660_e158161_d_n0;
        var_rdd_dn2 = assign105660_e158161_d_n2;
        var_rdd_dn4 = assign105660_e158161_d_n4;
        var_rdd_dn5 = assign105660_e158161_d_n5;
        var_rdd_dn6 = assign105660_e158161_d_n6;
        var_rdd_dn7 = assign105660_e158161_d_n7;
        var_rdd_dn8 = assign105660_e158161_d_n8;
        var_rdd_dn9 = assign105660_e158161_d_n9;
        var_rdd_dn10 = assign105660_e158161_d_n10;
        var_rdd_dn13 = assign105660_e158161_d_n13;

        let (assign105670_e158171, assign105670_e158171_d_n0, assign105670_e158171_d_n2, assign105670_e158171_d_n4, assign105670_e158171_d_n5, assign105670_e158171_d_n6, assign105670_e158171_d_n7, assign105670_e158171_d_n8, assign105670_e158171_d_n9, assign105670_e158171_d_n10, assign105670_e158171_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2387 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign105670_e158171;
        var_t0_dn0 = assign105670_e158171_d_n0;
        var_t0_dn2 = assign105670_e158171_d_n2;
        var_t0_dn4 = assign105670_e158171_d_n4;
        var_t0_dn5 = assign105670_e158171_d_n5;
        var_t0_dn6 = assign105670_e158171_d_n6;
        var_t0_dn7 = assign105670_e158171_d_n7;
        var_t0_dn8 = assign105670_e158171_d_n8;
        var_t0_dn9 = assign105670_e158171_d_n9;
        var_t0_dn10 = assign105670_e158171_d_n10;
        var_t0_dn13 = assign105670_e158171_d_n13;

        let assign105680_e158178: f64 = (var_mks_nsubsub + var_uc_nover);
        let assign105680_e158179: f64 = (var_uc_nover * assign105680_e158178);
        let assign105680_e158182: f64 = if ((p.p54 == 1.0) && (assign105680_e158179 > 0.0)) { 1.0 } else { 0.0 };
        var_guard2393 = assign105680_e158182;

        let (assign105690_e158193, assign105690_e158193_d_n0, assign105690_e158193_d_n2, assign105690_e158193_d_n4, assign105690_e158193_d_n5, assign105690_e158193_d_n6, assign105690_e158193_d_n7, assign105690_e158193_d_n8, assign105690_e158193_d_n9, assign105690_e158193_d_n10, assign105690_e158193_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2393 != 0.0)) {
        let assign105690_e158191: f64 = (p.p334 - var_wdep);
        (assign105690_e158191, (-var_wdep_dn0), (-var_wdep_dn2), (-var_wdep_dn4), (-var_wdep_dn5), (-var_wdep_dn6), (-var_wdep_dn7), (-var_wdep_dn8), (-var_wdep_dn9), (-var_wdep_dn10), (-var_wdep_dn13),)
    } else {
        (var_ddriftld, var_ddriftld_dn0, var_ddriftld_dn2, var_ddriftld_dn4, var_ddriftld_dn5, var_ddriftld_dn6, var_ddriftld_dn7, var_ddriftld_dn8, var_ddriftld_dn9, var_ddriftld_dn10, var_ddriftld_dn13,)
    }
};
        var_ddriftld = assign105690_e158193;
        var_ddriftld_dn0 = assign105690_e158193_d_n0;
        var_ddriftld_dn2 = assign105690_e158193_d_n2;
        var_ddriftld_dn4 = assign105690_e158193_d_n4;
        var_ddriftld_dn5 = assign105690_e158193_d_n5;
        var_ddriftld_dn6 = assign105690_e158193_d_n6;
        var_ddriftld_dn7 = assign105690_e158193_d_n7;
        var_ddriftld_dn8 = assign105690_e158193_d_n8;
        var_ddriftld_dn9 = assign105690_e158193_d_n9;
        var_ddriftld_dn10 = assign105690_e158193_d_n10;
        var_ddriftld_dn13 = assign105690_e158193_d_n13;

        let (assign105700_e158206, assign105700_e158206_d_n0, assign105700_e158206_d_n2, assign105700_e158206_d_n4, assign105700_e158206_d_n5, assign105700_e158206_d_n6, assign105700_e158206_d_n7, assign105700_e158206_d_n8, assign105700_e158206_d_n9, assign105700_e158206_d_n10, assign105700_e158206_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2393 != 0.0)) {
        let assign105700_e158202: f64 = (var_rdd * var_ldrift0);
        let assign105700_e158204: f64 = (assign105700_e158202 / var_ddriftld);
        (assign105700_e158204, ((((var_rdd_dn0 * var_ldrift0) * var_ddriftld) - (assign105700_e158202 * var_ddriftld_dn0)) / (var_ddriftld * var_ddriftld)), ((((var_rdd_dn2 * var_ldrift0) * var_ddriftld) - (assign105700_e158202 * var_ddriftld_dn2)) / (var_ddriftld * var_ddriftld)), ((((var_rdd_dn4 * var_ldrift0) * var_ddriftld) - (assign105700_e158202 * var_ddriftld_dn4)) / (var_ddriftld * var_ddriftld)), ((((var_rdd_dn5 * var_ldrift0) * var_ddriftld) - (assign105700_e158202 * var_ddriftld_dn5)) / (var_ddriftld * var_ddriftld)), ((((var_rdd_dn6 * var_ldrift0) * var_ddriftld) - (assign105700_e158202 * var_ddriftld_dn6)) / (var_ddriftld * var_ddriftld)), ((((var_rdd_dn7 * var_ldrift0) * var_ddriftld) - (assign105700_e158202 * var_ddriftld_dn7)) / (var_ddriftld * var_ddriftld)), ((((var_rdd_dn8 * var_ldrift0) * var_ddriftld) - (assign105700_e158202 * var_ddriftld_dn8)) / (var_ddriftld * var_ddriftld)), ((((var_rdd_dn9 * var_ldrift0) * var_ddriftld) - (assign105700_e158202 * var_ddriftld_dn9)) / (var_ddriftld * var_ddriftld)), ((((var_rdd_dn10 * var_ldrift0) * var_ddriftld) - (assign105700_e158202 * var_ddriftld_dn10)) / (var_ddriftld * var_ddriftld)), ((((var_rdd_dn13 * var_ldrift0) * var_ddriftld) - (assign105700_e158202 * var_ddriftld_dn13)) / (var_ddriftld * var_ddriftld)),)
    } else {
        (var_rdd, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn13,)
    }
};
        var_rdd = assign105700_e158206;
        var_rdd_dn0 = assign105700_e158206_d_n0;
        var_rdd_dn2 = assign105700_e158206_d_n2;
        var_rdd_dn4 = assign105700_e158206_d_n4;
        var_rdd_dn5 = assign105700_e158206_d_n5;
        var_rdd_dn6 = assign105700_e158206_d_n6;
        var_rdd_dn7 = assign105700_e158206_d_n7;
        var_rdd_dn8 = assign105700_e158206_d_n8;
        var_rdd_dn9 = assign105700_e158206_d_n9;
        var_rdd_dn10 = assign105700_e158206_d_n10;
        var_rdd_dn13 = assign105700_e158206_d_n13;

        let (assign105710_e158215, assign105710_e158215_d_n0, assign105710_e158215_d_n2, assign105710_e158215_d_n4, assign105710_e158215_d_n5, assign105710_e158215_d_n6, assign105710_e158215_d_n7, assign105710_e158215_d_n8, assign105710_e158215_d_n9, assign105710_e158215_d_n10, assign105710_e158215_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign105710_e158213: f64 = (var_rdd + var_rd0);
        (assign105710_e158213, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn13,)
    } else {
        (var_rdd, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn13,)
    }
};
        var_rdd = assign105710_e158215;
        var_rdd_dn0 = assign105710_e158215_d_n0;
        var_rdd_dn2 = assign105710_e158215_d_n2;
        var_rdd_dn4 = assign105710_e158215_d_n4;
        var_rdd_dn5 = assign105710_e158215_d_n5;
        var_rdd_dn6 = assign105710_e158215_d_n6;
        var_rdd_dn7 = assign105710_e158215_d_n7;
        var_rdd_dn8 = assign105710_e158215_d_n8;
        var_rdd_dn9 = assign105710_e158215_d_n9;
        var_rdd_dn10 = assign105710_e158215_d_n10;
        var_rdd_dn13 = assign105710_e158215_d_n13;

        let assign105750_e158246: f64 = if var_rdd < p.p444 { 1.0 } else { 0.0 };
        var_guard2395 = assign105750_e158246;

        let (assign105760_e158255, assign105760_e158255_d_n0, assign105760_e158255_d_n2, assign105760_e158255_d_n4, assign105760_e158255_d_n5, assign105760_e158255_d_n6, assign105760_e158255_d_n7, assign105760_e158255_d_n8, assign105760_e158255_d_n9, assign105760_e158255_d_n10, assign105760_e158255_d_n13,) = {
    if (((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) && (var_guard2395 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdd, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn13,)
    }
};
        var_rdd = assign105760_e158255;
        var_rdd_dn0 = assign105760_e158255_d_n0;
        var_rdd_dn2 = assign105760_e158255_d_n2;
        var_rdd_dn4 = assign105760_e158255_d_n4;
        var_rdd_dn5 = assign105760_e158255_d_n5;
        var_rdd_dn6 = assign105760_e158255_d_n6;
        var_rdd_dn7 = assign105760_e158255_d_n7;
        var_rdd_dn8 = assign105760_e158255_d_n8;
        var_rdd_dn9 = assign105760_e158255_d_n9;
        var_rdd_dn10 = assign105760_e158255_d_n10;
        var_rdd_dn13 = assign105760_e158255_d_n13;

        let (assign105770_e158264, assign105770_e158264_d_n0, assign105770_e158264_d_n2, assign105770_e158264_d_n4, assign105770_e158264_d_n5, assign105770_e158264_d_n6, assign105770_e158264_d_n7, assign105770_e158264_d_n8, assign105770_e158264_d_n9, assign105770_e158264_d_n10, assign105770_e158264_d_n13,) = {
    if ((var_guard2336 != 0.0) && (var_guard2356 == 0.0)) {
        let assign105770_e158262: f64 = (var_rdd / var_mfactor);
        (assign105770_e158262, (var_rdd_dn0 / var_mfactor), (var_rdd_dn2 / var_mfactor), (var_rdd_dn4 / var_mfactor), (var_rdd_dn5 / var_mfactor), (var_rdd_dn6 / var_mfactor), (var_rdd_dn7 / var_mfactor), (var_rdd_dn8 / var_mfactor), (var_rdd_dn9 / var_mfactor), (var_rdd_dn10 / var_mfactor), (var_rdd_dn13 / var_mfactor),)
    } else {
        (var_rdde, var_rdde_dn0, var_rdde_dn2, var_rdde_dn4, var_rdde_dn5, var_rdde_dn6, var_rdde_dn7, var_rdde_dn8, var_rdde_dn9, var_rdde_dn10, var_rdde_dn13,)
    }
};
        var_rdde = assign105770_e158264;
        var_rdde_dn0 = assign105770_e158264_d_n0;
        var_rdde_dn2 = assign105770_e158264_d_n2;
        var_rdde_dn4 = assign105770_e158264_d_n4;
        var_rdde_dn5 = assign105770_e158264_d_n5;
        var_rdde_dn6 = assign105770_e158264_d_n6;
        var_rdde_dn7 = assign105770_e158264_d_n7;
        var_rdde_dn8 = assign105770_e158264_d_n8;
        var_rdde_dn9 = assign105770_e158264_d_n9;
        var_rdde_dn10 = assign105770_e158264_d_n10;
        var_rdde_dn13 = assign105770_e158264_d_n13;

        let assign105780_e158267: f64 = if var_rdd < p.p444 { 1.0 } else { 0.0 };
        var_guard2396 = assign105780_e158267;

        let (assign105790_e158274, assign105790_e158274_d_n0, assign105790_e158274_d_n2, assign105790_e158274_d_n4, assign105790_e158274_d_n5, assign105790_e158274_d_n6, assign105790_e158274_d_n7, assign105790_e158274_d_n8, assign105790_e158274_d_n9, assign105790_e158274_d_n10, assign105790_e158274_d_n13,) = {
    if ((var_guard2336 == 0.0) && (var_guard2396 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdd, var_rdd_dn0, var_rdd_dn2, var_rdd_dn4, var_rdd_dn5, var_rdd_dn6, var_rdd_dn7, var_rdd_dn8, var_rdd_dn9, var_rdd_dn10, var_rdd_dn13,)
    }
};
        var_rdd = assign105790_e158274;
        var_rdd_dn0 = assign105790_e158274_d_n0;
        var_rdd_dn2 = assign105790_e158274_d_n2;
        var_rdd_dn4 = assign105790_e158274_d_n4;
        var_rdd_dn5 = assign105790_e158274_d_n5;
        var_rdd_dn6 = assign105790_e158274_d_n6;
        var_rdd_dn7 = assign105790_e158274_d_n7;
        var_rdd_dn8 = assign105790_e158274_d_n8;
        var_rdd_dn9 = assign105790_e158274_d_n9;
        var_rdd_dn10 = assign105790_e158274_d_n10;
        var_rdd_dn13 = assign105790_e158274_d_n13;

        let assign105800_e158277: f64 = if var_rsd < p.p444 { 1.0 } else { 0.0 };
        var_guard2397 = assign105800_e158277;

        let (assign105810_e158284, assign105810_e158284_d_n0, assign105810_e158284_d_n2, assign105810_e158284_d_n4, assign105810_e158284_d_n5, assign105810_e158284_d_n6, assign105810_e158284_d_n7, assign105810_e158284_d_n8, assign105810_e158284_d_n9, assign105810_e158284_d_n10, assign105810_e158284_d_n13,) = {
    if ((var_guard2336 == 0.0) && (var_guard2397 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rsd, var_rsd_dn0, var_rsd_dn2, var_rsd_dn4, var_rsd_dn5, var_rsd_dn6, var_rsd_dn7, var_rsd_dn8, var_rsd_dn9, var_rsd_dn10, var_rsd_dn13,)
    }
};
        var_rsd = assign105810_e158284;
        var_rsd_dn0 = assign105810_e158284_d_n0;
        var_rsd_dn2 = assign105810_e158284_d_n2;
        var_rsd_dn4 = assign105810_e158284_d_n4;
        var_rsd_dn5 = assign105810_e158284_d_n5;
        var_rsd_dn6 = assign105810_e158284_d_n6;
        var_rsd_dn7 = assign105810_e158284_d_n7;
        var_rsd_dn8 = assign105810_e158284_d_n8;
        var_rsd_dn9 = assign105810_e158284_d_n9;
        var_rsd_dn10 = assign105810_e158284_d_n10;
        var_rsd_dn13 = assign105810_e158284_d_n13;

        let assign105820_e158287: f64 = if var_vdsemodenml > 0.0 { 1.0 } else { 0.0 };
        var_guard2398 = assign105820_e158287;

        let (assign105830_e158296, assign105830_e158296_d_n0, assign105830_e158296_d_n2, assign105830_e158296_d_n4, assign105830_e158296_d_n5, assign105830_e158296_d_n6, assign105830_e158296_d_n7, assign105830_e158296_d_n8, assign105830_e158296_d_n9, assign105830_e158296_d_n10, assign105830_e158296_d_n13,) = {
    if ((var_guard2336 == 0.0) && (var_guard2398 != 0.0)) {
        let assign105830_e158294: f64 = (var_rdd / var_mfactor);
        (assign105830_e158294, (var_rdd_dn0 / var_mfactor), (var_rdd_dn2 / var_mfactor), (var_rdd_dn4 / var_mfactor), (var_rdd_dn5 / var_mfactor), (var_rdd_dn6 / var_mfactor), (var_rdd_dn7 / var_mfactor), (var_rdd_dn8 / var_mfactor), (var_rdd_dn9 / var_mfactor), (var_rdd_dn10 / var_mfactor), (var_rdd_dn13 / var_mfactor),)
    } else {
        (var_rdde, var_rdde_dn0, var_rdde_dn2, var_rdde_dn4, var_rdde_dn5, var_rdde_dn6, var_rdde_dn7, var_rdde_dn8, var_rdde_dn9, var_rdde_dn10, var_rdde_dn13,)
    }
};
        var_rdde = assign105830_e158296;
        var_rdde_dn0 = assign105830_e158296_d_n0;
        var_rdde_dn2 = assign105830_e158296_d_n2;
        var_rdde_dn4 = assign105830_e158296_d_n4;
        var_rdde_dn5 = assign105830_e158296_d_n5;
        var_rdde_dn6 = assign105830_e158296_d_n6;
        var_rdde_dn7 = assign105830_e158296_d_n7;
        var_rdde_dn8 = assign105830_e158296_d_n8;
        var_rdde_dn9 = assign105830_e158296_d_n9;
        var_rdde_dn10 = assign105830_e158296_d_n10;
        var_rdde_dn13 = assign105830_e158296_d_n13;

        let (assign105840_e158305, assign105840_e158305_d_n0, assign105840_e158305_d_n2, assign105840_e158305_d_n4, assign105840_e158305_d_n5, assign105840_e158305_d_n6, assign105840_e158305_d_n7, assign105840_e158305_d_n8, assign105840_e158305_d_n9, assign105840_e158305_d_n10, assign105840_e158305_d_n13,) = {
    if ((var_guard2336 == 0.0) && (var_guard2398 != 0.0)) {
        let assign105840_e158303: f64 = (var_rsd / var_mfactor);
        (assign105840_e158303, (var_rsd_dn0 / var_mfactor), (var_rsd_dn2 / var_mfactor), (var_rsd_dn4 / var_mfactor), (var_rsd_dn5 / var_mfactor), (var_rsd_dn6 / var_mfactor), (var_rsd_dn7 / var_mfactor), (var_rsd_dn8 / var_mfactor), (var_rsd_dn9 / var_mfactor), (var_rsd_dn10 / var_mfactor), (var_rsd_dn13 / var_mfactor),)
    } else {
        (var_rsde, var_rsde_dn0, var_rsde_dn2, var_rsde_dn4, var_rsde_dn5, var_rsde_dn6, var_rsde_dn7, var_rsde_dn8, var_rsde_dn9, var_rsde_dn10, var_rsde_dn13,)
    }
};
        var_rsde = assign105840_e158305;
        var_rsde_dn0 = assign105840_e158305_d_n0;
        var_rsde_dn2 = assign105840_e158305_d_n2;
        var_rsde_dn4 = assign105840_e158305_d_n4;
        var_rsde_dn5 = assign105840_e158305_d_n5;
        var_rsde_dn6 = assign105840_e158305_d_n6;
        var_rsde_dn7 = assign105840_e158305_d_n7;
        var_rsde_dn8 = assign105840_e158305_d_n8;
        var_rsde_dn9 = assign105840_e158305_d_n9;
        var_rsde_dn10 = assign105840_e158305_d_n10;
        var_rsde_dn13 = assign105840_e158305_d_n13;

        let (assign105850_e158315, assign105850_e158315_d_n0, assign105850_e158315_d_n2, assign105850_e158315_d_n4, assign105850_e158315_d_n5, assign105850_e158315_d_n6, assign105850_e158315_d_n7, assign105850_e158315_d_n8, assign105850_e158315_d_n9, assign105850_e158315_d_n10, assign105850_e158315_d_n13,) = {
    if ((var_guard2336 == 0.0) && (var_guard2398 == 0.0)) {
        let assign105850_e158313: f64 = (var_rsd / var_mfactor);
        (assign105850_e158313, (var_rsd_dn0 / var_mfactor), (var_rsd_dn2 / var_mfactor), (var_rsd_dn4 / var_mfactor), (var_rsd_dn5 / var_mfactor), (var_rsd_dn6 / var_mfactor), (var_rsd_dn7 / var_mfactor), (var_rsd_dn8 / var_mfactor), (var_rsd_dn9 / var_mfactor), (var_rsd_dn10 / var_mfactor), (var_rsd_dn13 / var_mfactor),)
    } else {
        (var_rdde, var_rdde_dn0, var_rdde_dn2, var_rdde_dn4, var_rdde_dn5, var_rdde_dn6, var_rdde_dn7, var_rdde_dn8, var_rdde_dn9, var_rdde_dn10, var_rdde_dn13,)
    }
};
        var_rdde = assign105850_e158315;
        var_rdde_dn0 = assign105850_e158315_d_n0;
        var_rdde_dn2 = assign105850_e158315_d_n2;
        var_rdde_dn4 = assign105850_e158315_d_n4;
        var_rdde_dn5 = assign105850_e158315_d_n5;
        var_rdde_dn6 = assign105850_e158315_d_n6;
        var_rdde_dn7 = assign105850_e158315_d_n7;
        var_rdde_dn8 = assign105850_e158315_d_n8;
        var_rdde_dn9 = assign105850_e158315_d_n9;
        var_rdde_dn10 = assign105850_e158315_d_n10;
        var_rdde_dn13 = assign105850_e158315_d_n13;

        *var_ddriftld_slot = var_ddriftld;
        *var_ddriftld_dn0_slot = var_ddriftld_dn0;
        *var_ddriftld_dn10_slot = var_ddriftld_dn10;
        *var_ddriftld_dn13_slot = var_ddriftld_dn13;
        *var_ddriftld_dn2_slot = var_ddriftld_dn2;
        *var_ddriftld_dn4_slot = var_ddriftld_dn4;
        *var_ddriftld_dn5_slot = var_ddriftld_dn5;
        *var_ddriftld_dn6_slot = var_ddriftld_dn6;
        *var_ddriftld_dn7_slot = var_ddriftld_dn7;
        *var_ddriftld_dn8_slot = var_ddriftld_dn8;
        *var_ddriftld_dn9_slot = var_ddriftld_dn9;
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
        *var_guard2388_slot = var_guard2388;
        *var_guard2389_slot = var_guard2389;
        *var_guard2390_slot = var_guard2390;
        *var_guard2391_slot = var_guard2391;
        *var_guard2392_slot = var_guard2392;
        *var_guard2393_slot = var_guard2393;
        *var_guard2395_slot = var_guard2395;
        *var_guard2396_slot = var_guard2396;
        *var_guard2397_slot = var_guard2397;
        *var_guard2398_slot = var_guard2398;
        *var_m0_slot = var_m0;
        *var_mm_slot = var_mm;
        *var_rdd_slot = var_rdd;
        *var_rdd_dn0_slot = var_rdd_dn0;
        *var_rdd_dn10_slot = var_rdd_dn10;
        *var_rdd_dn13_slot = var_rdd_dn13;
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
        *var_rdde_dn13_slot = var_rdde_dn13;
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
        *var_rsd_dn13_slot = var_rsd_dn13;
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
        *var_rsde_dn13_slot = var_rsde_dn13;
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
        *var_t0_dn13_slot = var_t0_dn13;
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
        *var_tmf0_dn13_slot = var_tmf0_dn13;
        *var_tmf0_dn2_slot = var_tmf0_dn2;
        *var_tmf0_dn4_slot = var_tmf0_dn4;
        *var_tmf0_dn5_slot = var_tmf0_dn5;
        *var_tmf0_dn6_slot = var_tmf0_dn6;
        *var_tmf0_dn7_slot = var_tmf0_dn7;
        *var_tmf0_dn8_slot = var_tmf0_dn8;
        *var_tmf0_dn9_slot = var_tmf0_dn9;
    }

    pub(super) fn stamp_transient_block_374(
        var_flg_nqs: f64,
        var_guard2336: f64,
        var_guard2398: f64,
        var_ibjte: f64,
        var_ibjte_dn0: f64,
        var_ibjte_dn10: f64,
        var_ibjte_dn13: f64,
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
        var_ibreake_dn13: f64,
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
        var_idse_dn13: f64,
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
        var_idsibpce_dn13: f64,
        var_idsibpce_dn2: f64,
        var_idsibpce_dn4: f64,
        var_idsibpce_dn5: f64,
        var_idsibpce_dn6: f64,
        var_idsibpce_dn7: f64,
        var_idsibpce_dn8: f64,
        var_idsibpce_dn9: f64,
        var_igbe: f64,
        var_igbe_dn0: f64,
        var_igbe_dn10: f64,
        var_igbe_dn13: f64,
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
        var_igde_dn13: f64,
        var_igde_dn2: f64,
        var_igde_dn4: f64,
        var_igde_dn5: f64,
        var_igde_dn6: f64,
        var_igde_dn7: f64,
        var_igde_dn8: f64,
        var_igde_dn9: f64,
        var_igidle: f64,
        var_igidle_dn0: f64,
        var_igidle_dn10: f64,
        var_igidle_dn13: f64,
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
        var_igisle_dn13: f64,
        var_igisle_dn2: f64,
        var_igisle_dn4: f64,
        var_igisle_dn5: f64,
        var_igisle_dn6: f64,
        var_igisle_dn7: f64,
        var_igisle_dn8: f64,
        var_igisle_dn9: f64,
        var_igse: f64,
        var_igse_dn0: f64,
        var_igse_dn10: f64,
        var_igse_dn13: f64,
        var_igse_dn2: f64,
        var_igse_dn4: f64,
        var_igse_dn5: f64,
        var_igse_dn6: f64,
        var_igse_dn7: f64,
        var_igse_dn8: f64,
        var_igse_dn9: f64,
        var_isube: f64,
        var_isube_dn0: f64,
        var_isube_dn10: f64,
        var_isube_dn13: f64,
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
        var_isublde_dn13: f64,
        var_isublde_dn2: f64,
        var_isublde_dn4: f64,
        var_isublde_dn5: f64,
        var_isublde_dn6: f64,
        var_isublde_dn7: f64,
        var_isublde_dn8: f64,
        var_isublde_dn9: f64,
        var_mfactor: f64,
        var_mode: f64,
        var_qde: f64,
        var_qde_dn0: f64,
        var_qde_dn10: f64,
        var_qde_dn13: f64,
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
        var_qge_dn13: f64,
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
        var_qse_dn13: f64,
        var_qse_dn2: f64,
        var_qse_dn4: f64,
        var_qse_dn5: f64,
        var_qse_dn6: f64,
        var_qse_dn7: f64,
        var_qse_dn8: f64,
        var_qse_dn9: f64,
        var_rdde: f64,
        var_rdde_dn0: f64,
        var_rdde_dn10: f64,
        var_rdde_dn13: f64,
        var_rdde_dn2: f64,
        var_rdde_dn4: f64,
        var_rdde_dn5: f64,
        var_rdde_dn6: f64,
        var_rdde_dn7: f64,
        var_rdde_dn8: f64,
        var_rdde_dn9: f64,
        var_xd: f64,
        var_xd_dn0: f64,
        var_xd_dn10: f64,
        var_xd_dn13: f64,
        var_xd_dn2: f64,
        var_xd_dn4: f64,
        var_xd_dn5: f64,
        var_xd_dn6: f64,
        var_xd_dn7: f64,
        var_xd_dn8: f64,
        var_xd_dn9: f64,
        var_guard2399_slot: &mut f64,
        var_ibjt_slot: &mut f64,
        var_ibjt_dn0_slot: &mut f64,
        var_ibjt_dn10_slot: &mut f64,
        var_ibjt_dn13_slot: &mut f64,
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
        var_ibjts_dn13_slot: &mut f64,
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
        var_ibreak_dn13_slot: &mut f64,
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
        var_ibreaks_dn13_slot: &mut f64,
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
        var_ids_dn13_slot: &mut f64,
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
        var_idsibpc_dn13_slot: &mut f64,
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
        var_idsibpcs_dn13_slot: &mut f64,
        var_idsibpcs_dn2_slot: &mut f64,
        var_idsibpcs_dn4_slot: &mut f64,
        var_idsibpcs_dn5_slot: &mut f64,
        var_idsibpcs_dn6_slot: &mut f64,
        var_idsibpcs_dn7_slot: &mut f64,
        var_idsibpcs_dn8_slot: &mut f64,
        var_idsibpcs_dn9_slot: &mut f64,
        var_igb_slot: &mut f64,
        var_igb_dn0_slot: &mut f64,
        var_igb_dn10_slot: &mut f64,
        var_igb_dn13_slot: &mut f64,
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
        var_igd_dn13_slot: &mut f64,
        var_igd_dn2_slot: &mut f64,
        var_igd_dn4_slot: &mut f64,
        var_igd_dn5_slot: &mut f64,
        var_igd_dn6_slot: &mut f64,
        var_igd_dn7_slot: &mut f64,
        var_igd_dn8_slot: &mut f64,
        var_igd_dn9_slot: &mut f64,
        var_igidl_slot: &mut f64,
        var_igidl_dn0_slot: &mut f64,
        var_igidl_dn10_slot: &mut f64,
        var_igidl_dn13_slot: &mut f64,
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
        var_igisl_dn13_slot: &mut f64,
        var_igisl_dn2_slot: &mut f64,
        var_igisl_dn4_slot: &mut f64,
        var_igisl_dn5_slot: &mut f64,
        var_igisl_dn6_slot: &mut f64,
        var_igisl_dn7_slot: &mut f64,
        var_igisl_dn8_slot: &mut f64,
        var_igisl_dn9_slot: &mut f64,
        var_igs_slot: &mut f64,
        var_igs_dn0_slot: &mut f64,
        var_igs_dn10_slot: &mut f64,
        var_igs_dn13_slot: &mut f64,
        var_igs_dn2_slot: &mut f64,
        var_igs_dn4_slot: &mut f64,
        var_igs_dn5_slot: &mut f64,
        var_igs_dn6_slot: &mut f64,
        var_igs_dn7_slot: &mut f64,
        var_igs_dn8_slot: &mut f64,
        var_igs_dn9_slot: &mut f64,
        var_isub_slot: &mut f64,
        var_isub_dn0_slot: &mut f64,
        var_isub_dn10_slot: &mut f64,
        var_isub_dn13_slot: &mut f64,
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
        var_isubld_dn13_slot: &mut f64,
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
        var_isublds_dn13_slot: &mut f64,
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
        var_isubs_dn13_slot: &mut f64,
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
        var_qb_dn13_slot: &mut f64,
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
        var_qd_dn13_slot: &mut f64,
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
        var_qdrat_dn13_slot: &mut f64,
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
        var_qg_dn13_slot: &mut f64,
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
        var_qs_dn13_slot: &mut f64,
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
        var_rdd_dn13_slot: &mut f64,
        var_rdd_dn2_slot: &mut f64,
        var_rdd_dn4_slot: &mut f64,
        var_rdd_dn5_slot: &mut f64,
        var_rdd_dn6_slot: &mut f64,
        var_rdd_dn7_slot: &mut f64,
        var_rdd_dn8_slot: &mut f64,
        var_rdd_dn9_slot: &mut f64,
        var_rsd_slot: &mut f64,
        var_rsd_dn0_slot: &mut f64,
        var_rsd_dn10_slot: &mut f64,
        var_rsd_dn13_slot: &mut f64,
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
        var_rsde_dn13_slot: &mut f64,
        var_rsde_dn2_slot: &mut f64,
        var_rsde_dn4_slot: &mut f64,
        var_rsde_dn5_slot: &mut f64,
        var_rsde_dn6_slot: &mut f64,
        var_rsde_dn7_slot: &mut f64,
        var_rsde_dn8_slot: &mut f64,
        var_rsde_dn9_slot: &mut f64,
    ) {
        let mut var_guard2399: f64 = *var_guard2399_slot;
        let mut var_ibjt: f64 = *var_ibjt_slot;
        let mut var_ibjt_dn0: f64 = *var_ibjt_dn0_slot;
        let mut var_ibjt_dn10: f64 = *var_ibjt_dn10_slot;
        let mut var_ibjt_dn13: f64 = *var_ibjt_dn13_slot;
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
        let mut var_ibjts_dn13: f64 = *var_ibjts_dn13_slot;
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
        let mut var_ibreak_dn13: f64 = *var_ibreak_dn13_slot;
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
        let mut var_ibreaks_dn13: f64 = *var_ibreaks_dn13_slot;
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
        let mut var_ids_dn13: f64 = *var_ids_dn13_slot;
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
        let mut var_idsibpc_dn13: f64 = *var_idsibpc_dn13_slot;
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
        let mut var_idsibpcs_dn13: f64 = *var_idsibpcs_dn13_slot;
        let mut var_idsibpcs_dn2: f64 = *var_idsibpcs_dn2_slot;
        let mut var_idsibpcs_dn4: f64 = *var_idsibpcs_dn4_slot;
        let mut var_idsibpcs_dn5: f64 = *var_idsibpcs_dn5_slot;
        let mut var_idsibpcs_dn6: f64 = *var_idsibpcs_dn6_slot;
        let mut var_idsibpcs_dn7: f64 = *var_idsibpcs_dn7_slot;
        let mut var_idsibpcs_dn8: f64 = *var_idsibpcs_dn8_slot;
        let mut var_idsibpcs_dn9: f64 = *var_idsibpcs_dn9_slot;
        let mut var_igb: f64 = *var_igb_slot;
        let mut var_igb_dn0: f64 = *var_igb_dn0_slot;
        let mut var_igb_dn10: f64 = *var_igb_dn10_slot;
        let mut var_igb_dn13: f64 = *var_igb_dn13_slot;
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
        let mut var_igd_dn13: f64 = *var_igd_dn13_slot;
        let mut var_igd_dn2: f64 = *var_igd_dn2_slot;
        let mut var_igd_dn4: f64 = *var_igd_dn4_slot;
        let mut var_igd_dn5: f64 = *var_igd_dn5_slot;
        let mut var_igd_dn6: f64 = *var_igd_dn6_slot;
        let mut var_igd_dn7: f64 = *var_igd_dn7_slot;
        let mut var_igd_dn8: f64 = *var_igd_dn8_slot;
        let mut var_igd_dn9: f64 = *var_igd_dn9_slot;
        let mut var_igidl: f64 = *var_igidl_slot;
        let mut var_igidl_dn0: f64 = *var_igidl_dn0_slot;
        let mut var_igidl_dn10: f64 = *var_igidl_dn10_slot;
        let mut var_igidl_dn13: f64 = *var_igidl_dn13_slot;
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
        let mut var_igisl_dn13: f64 = *var_igisl_dn13_slot;
        let mut var_igisl_dn2: f64 = *var_igisl_dn2_slot;
        let mut var_igisl_dn4: f64 = *var_igisl_dn4_slot;
        let mut var_igisl_dn5: f64 = *var_igisl_dn5_slot;
        let mut var_igisl_dn6: f64 = *var_igisl_dn6_slot;
        let mut var_igisl_dn7: f64 = *var_igisl_dn7_slot;
        let mut var_igisl_dn8: f64 = *var_igisl_dn8_slot;
        let mut var_igisl_dn9: f64 = *var_igisl_dn9_slot;
        let mut var_igs: f64 = *var_igs_slot;
        let mut var_igs_dn0: f64 = *var_igs_dn0_slot;
        let mut var_igs_dn10: f64 = *var_igs_dn10_slot;
        let mut var_igs_dn13: f64 = *var_igs_dn13_slot;
        let mut var_igs_dn2: f64 = *var_igs_dn2_slot;
        let mut var_igs_dn4: f64 = *var_igs_dn4_slot;
        let mut var_igs_dn5: f64 = *var_igs_dn5_slot;
        let mut var_igs_dn6: f64 = *var_igs_dn6_slot;
        let mut var_igs_dn7: f64 = *var_igs_dn7_slot;
        let mut var_igs_dn8: f64 = *var_igs_dn8_slot;
        let mut var_igs_dn9: f64 = *var_igs_dn9_slot;
        let mut var_isub: f64 = *var_isub_slot;
        let mut var_isub_dn0: f64 = *var_isub_dn0_slot;
        let mut var_isub_dn10: f64 = *var_isub_dn10_slot;
        let mut var_isub_dn13: f64 = *var_isub_dn13_slot;
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
        let mut var_isubld_dn13: f64 = *var_isubld_dn13_slot;
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
        let mut var_isublds_dn13: f64 = *var_isublds_dn13_slot;
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
        let mut var_isubs_dn13: f64 = *var_isubs_dn13_slot;
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
        let mut var_qb_dn13: f64 = *var_qb_dn13_slot;
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
        let mut var_qd_dn13: f64 = *var_qd_dn13_slot;
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
        let mut var_qdrat_dn13: f64 = *var_qdrat_dn13_slot;
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
        let mut var_qg_dn13: f64 = *var_qg_dn13_slot;
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
        let mut var_qs_dn13: f64 = *var_qs_dn13_slot;
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
        let mut var_rdd_dn13: f64 = *var_rdd_dn13_slot;
        let mut var_rdd_dn2: f64 = *var_rdd_dn2_slot;
        let mut var_rdd_dn4: f64 = *var_rdd_dn4_slot;
        let mut var_rdd_dn5: f64 = *var_rdd_dn5_slot;
        let mut var_rdd_dn6: f64 = *var_rdd_dn6_slot;
        let mut var_rdd_dn7: f64 = *var_rdd_dn7_slot;
        let mut var_rdd_dn8: f64 = *var_rdd_dn8_slot;
        let mut var_rdd_dn9: f64 = *var_rdd_dn9_slot;
        let mut var_rsd: f64 = *var_rsd_slot;
        let mut var_rsd_dn0: f64 = *var_rsd_dn0_slot;
        let mut var_rsd_dn10: f64 = *var_rsd_dn10_slot;
        let mut var_rsd_dn13: f64 = *var_rsd_dn13_slot;
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
        let mut var_rsde_dn13: f64 = *var_rsde_dn13_slot;
        let mut var_rsde_dn2: f64 = *var_rsde_dn2_slot;
        let mut var_rsde_dn4: f64 = *var_rsde_dn4_slot;
        let mut var_rsde_dn5: f64 = *var_rsde_dn5_slot;
        let mut var_rsde_dn6: f64 = *var_rsde_dn6_slot;
        let mut var_rsde_dn7: f64 = *var_rsde_dn7_slot;
        let mut var_rsde_dn8: f64 = *var_rsde_dn8_slot;
        let mut var_rsde_dn9: f64 = *var_rsde_dn9_slot;

        let (assign105860_e158325, assign105860_e158325_d_n0, assign105860_e158325_d_n2, assign105860_e158325_d_n4, assign105860_e158325_d_n5, assign105860_e158325_d_n6, assign105860_e158325_d_n7, assign105860_e158325_d_n8, assign105860_e158325_d_n9, assign105860_e158325_d_n10, assign105860_e158325_d_n13,) = {
    if ((var_guard2336 == 0.0) && (var_guard2398 == 0.0)) {
        let assign105860_e158323: f64 = (var_rdd / var_mfactor);
        (assign105860_e158323, (var_rdd_dn0 / var_mfactor), (var_rdd_dn2 / var_mfactor), (var_rdd_dn4 / var_mfactor), (var_rdd_dn5 / var_mfactor), (var_rdd_dn6 / var_mfactor), (var_rdd_dn7 / var_mfactor), (var_rdd_dn8 / var_mfactor), (var_rdd_dn9 / var_mfactor), (var_rdd_dn10 / var_mfactor), (var_rdd_dn13 / var_mfactor),)
    } else {
        (var_rsde, var_rsde_dn0, var_rsde_dn2, var_rsde_dn4, var_rsde_dn5, var_rsde_dn6, var_rsde_dn7, var_rsde_dn8, var_rsde_dn9, var_rsde_dn10, var_rsde_dn13,)
    }
};
        var_rsde = assign105860_e158325;
        var_rsde_dn0 = assign105860_e158325_d_n0;
        var_rsde_dn2 = assign105860_e158325_d_n2;
        var_rsde_dn4 = assign105860_e158325_d_n4;
        var_rsde_dn5 = assign105860_e158325_d_n5;
        var_rsde_dn6 = assign105860_e158325_d_n6;
        var_rsde_dn7 = assign105860_e158325_d_n7;
        var_rsde_dn8 = assign105860_e158325_d_n8;
        var_rsde_dn9 = assign105860_e158325_d_n9;
        var_rsde_dn10 = assign105860_e158325_d_n10;
        var_rsde_dn13 = assign105860_e158325_d_n13;

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
        var_rdd_dn13 = var_rdde_dn13;

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
        var_rsd_dn13 = var_rsde_dn13;

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
        var_igd_dn13 = var_igde_dn13;

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
        var_igs_dn13 = var_igse_dn13;

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
        var_igb_dn13 = var_igbe_dn13;

        let assign105920_e158333: f64 = if var_mode > 0.0 { 1.0 } else { 0.0 };
        var_guard2399 = assign105920_e158333;

        let (assign105930_e158337, assign105930_e158337_d_n0, assign105930_e158337_d_n2, assign105930_e158337_d_n4, assign105930_e158337_d_n5, assign105930_e158337_d_n6, assign105930_e158337_d_n7, assign105930_e158337_d_n8, assign105930_e158337_d_n9, assign105930_e158337_d_n10, assign105930_e158337_d_n13,) = {
    if (var_guard2399 != 0.0) {
        (var_idse, var_idse_dn0, var_idse_dn2, var_idse_dn4, var_idse_dn5, var_idse_dn6, var_idse_dn7, var_idse_dn8, var_idse_dn9, var_idse_dn10, var_idse_dn13,)
    } else {
        (var_ids, var_ids_dn0, var_ids_dn2, var_ids_dn4, var_ids_dn5, var_ids_dn6, var_ids_dn7, var_ids_dn8, var_ids_dn9, var_ids_dn10, var_ids_dn13,)
    }
};
        var_ids = assign105930_e158337;
        var_ids_dn0 = assign105930_e158337_d_n0;
        var_ids_dn2 = assign105930_e158337_d_n2;
        var_ids_dn4 = assign105930_e158337_d_n4;
        var_ids_dn5 = assign105930_e158337_d_n5;
        var_ids_dn6 = assign105930_e158337_d_n6;
        var_ids_dn7 = assign105930_e158337_d_n7;
        var_ids_dn8 = assign105930_e158337_d_n8;
        var_ids_dn9 = assign105930_e158337_d_n9;
        var_ids_dn10 = assign105930_e158337_d_n10;
        var_ids_dn13 = assign105930_e158337_d_n13;

        let (assign105940_e158341, assign105940_e158341_d_n0, assign105940_e158341_d_n2, assign105940_e158341_d_n4, assign105940_e158341_d_n5, assign105940_e158341_d_n6, assign105940_e158341_d_n7, assign105940_e158341_d_n8, assign105940_e158341_d_n9, assign105940_e158341_d_n10, assign105940_e158341_d_n13,) = {
    if (var_guard2399 != 0.0) {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn4, var_qde_dn5, var_qde_dn6, var_qde_dn7, var_qde_dn8, var_qde_dn9, var_qde_dn10, var_qde_dn13,)
    } else {
        (var_qd, var_qd_dn0, var_qd_dn2, var_qd_dn4, var_qd_dn5, var_qd_dn6, var_qd_dn7, var_qd_dn8, var_qd_dn9, var_qd_dn10, var_qd_dn13,)
    }
};
        var_qd = assign105940_e158341;
        var_qd_dn0 = assign105940_e158341_d_n0;
        var_qd_dn2 = assign105940_e158341_d_n2;
        var_qd_dn4 = assign105940_e158341_d_n4;
        var_qd_dn5 = assign105940_e158341_d_n5;
        var_qd_dn6 = assign105940_e158341_d_n6;
        var_qd_dn7 = assign105940_e158341_d_n7;
        var_qd_dn8 = assign105940_e158341_d_n8;
        var_qd_dn9 = assign105940_e158341_d_n9;
        var_qd_dn10 = assign105940_e158341_d_n10;
        var_qd_dn13 = assign105940_e158341_d_n13;

        let (assign105950_e158345, assign105950_e158345_d_n0, assign105950_e158345_d_n2, assign105950_e158345_d_n4, assign105950_e158345_d_n5, assign105950_e158345_d_n6, assign105950_e158345_d_n7, assign105950_e158345_d_n8, assign105950_e158345_d_n9, assign105950_e158345_d_n10, assign105950_e158345_d_n13,) = {
    if (var_guard2399 != 0.0) {
        (var_qge, var_qge_dn0, var_qge_dn2, var_qge_dn4, var_qge_dn5, var_qge_dn6, var_qge_dn7, var_qge_dn8, var_qge_dn9, var_qge_dn10, var_qge_dn13,)
    } else {
        (var_qg, var_qg_dn0, var_qg_dn2, var_qg_dn4, var_qg_dn5, var_qg_dn6, var_qg_dn7, var_qg_dn8, var_qg_dn9, var_qg_dn10, var_qg_dn13,)
    }
};
        var_qg = assign105950_e158345;
        var_qg_dn0 = assign105950_e158345_d_n0;
        var_qg_dn2 = assign105950_e158345_d_n2;
        var_qg_dn4 = assign105950_e158345_d_n4;
        var_qg_dn5 = assign105950_e158345_d_n5;
        var_qg_dn6 = assign105950_e158345_d_n6;
        var_qg_dn7 = assign105950_e158345_d_n7;
        var_qg_dn8 = assign105950_e158345_d_n8;
        var_qg_dn9 = assign105950_e158345_d_n9;
        var_qg_dn10 = assign105950_e158345_d_n10;
        var_qg_dn13 = assign105950_e158345_d_n13;

        let (assign105960_e158349, assign105960_e158349_d_n0, assign105960_e158349_d_n2, assign105960_e158349_d_n4, assign105960_e158349_d_n5, assign105960_e158349_d_n6, assign105960_e158349_d_n7, assign105960_e158349_d_n8, assign105960_e158349_d_n9, assign105960_e158349_d_n10, assign105960_e158349_d_n13,) = {
    if (var_guard2399 != 0.0) {
        (var_qse, var_qse_dn0, var_qse_dn2, var_qse_dn4, var_qse_dn5, var_qse_dn6, var_qse_dn7, var_qse_dn8, var_qse_dn9, var_qse_dn10, var_qse_dn13,)
    } else {
        (var_qs, var_qs_dn0, var_qs_dn2, var_qs_dn4, var_qs_dn5, var_qs_dn6, var_qs_dn7, var_qs_dn8, var_qs_dn9, var_qs_dn10, var_qs_dn13,)
    }
};
        var_qs = assign105960_e158349;
        var_qs_dn0 = assign105960_e158349_d_n0;
        var_qs_dn2 = assign105960_e158349_d_n2;
        var_qs_dn4 = assign105960_e158349_d_n4;
        var_qs_dn5 = assign105960_e158349_d_n5;
        var_qs_dn6 = assign105960_e158349_d_n6;
        var_qs_dn7 = assign105960_e158349_d_n7;
        var_qs_dn8 = assign105960_e158349_d_n8;
        var_qs_dn9 = assign105960_e158349_d_n9;
        var_qs_dn10 = assign105960_e158349_d_n10;
        var_qs_dn13 = assign105960_e158349_d_n13;

        let (assign105970_e158358, assign105970_e158358_d_n0, assign105970_e158358_d_n2, assign105970_e158358_d_n4, assign105970_e158358_d_n5, assign105970_e158358_d_n6, assign105970_e158358_d_n7, assign105970_e158358_d_n8, assign105970_e158358_d_n9, assign105970_e158358_d_n10, assign105970_e158358_d_n13,) = {
    if (var_guard2399 != 0.0) {
        let assign105970_e158353: f64 = (var_qge + var_qde);
        let assign105970_e158355: f64 = (assign105970_e158353 + var_qse);
        let assign105970_e158356: f64 = (-assign105970_e158355);
        (assign105970_e158356, (-((var_qge_dn0 + var_qde_dn0) + var_qse_dn0)), (-((var_qge_dn2 + var_qde_dn2) + var_qse_dn2)), (-((var_qge_dn4 + var_qde_dn4) + var_qse_dn4)), (-((var_qge_dn5 + var_qde_dn5) + var_qse_dn5)), (-((var_qge_dn6 + var_qde_dn6) + var_qse_dn6)), (-((var_qge_dn7 + var_qde_dn7) + var_qse_dn7)), (-((var_qge_dn8 + var_qde_dn8) + var_qse_dn8)), (-((var_qge_dn9 + var_qde_dn9) + var_qse_dn9)), (-((var_qge_dn10 + var_qde_dn10) + var_qse_dn10)), (-((var_qge_dn13 + var_qde_dn13) + var_qse_dn13)),)
    } else {
        (var_qb, var_qb_dn0, var_qb_dn2, var_qb_dn4, var_qb_dn5, var_qb_dn6, var_qb_dn7, var_qb_dn8, var_qb_dn9, var_qb_dn10, var_qb_dn13,)
    }
};
        var_qb = assign105970_e158358;
        var_qb_dn0 = assign105970_e158358_d_n0;
        var_qb_dn2 = assign105970_e158358_d_n2;
        var_qb_dn4 = assign105970_e158358_d_n4;
        var_qb_dn5 = assign105970_e158358_d_n5;
        var_qb_dn6 = assign105970_e158358_d_n6;
        var_qb_dn7 = assign105970_e158358_d_n7;
        var_qb_dn8 = assign105970_e158358_d_n8;
        var_qb_dn9 = assign105970_e158358_d_n9;
        var_qb_dn10 = assign105970_e158358_d_n10;
        var_qb_dn13 = assign105970_e158358_d_n13;

        let (assign105980_e158362, assign105980_e158362_d_n0, assign105980_e158362_d_n2, assign105980_e158362_d_n4, assign105980_e158362_d_n5, assign105980_e158362_d_n6, assign105980_e158362_d_n7, assign105980_e158362_d_n8, assign105980_e158362_d_n9, assign105980_e158362_d_n10, assign105980_e158362_d_n13,) = {
    if (var_guard2399 != 0.0) {
        (var_isube, var_isube_dn0, var_isube_dn2, var_isube_dn4, var_isube_dn5, var_isube_dn6, var_isube_dn7, var_isube_dn8, var_isube_dn9, var_isube_dn10, var_isube_dn13,)
    } else {
        (var_isub, var_isub_dn0, var_isub_dn2, var_isub_dn4, var_isub_dn5, var_isub_dn6, var_isub_dn7, var_isub_dn8, var_isub_dn9, var_isub_dn10, var_isub_dn13,)
    }
};
        var_isub = assign105980_e158362;
        var_isub_dn0 = assign105980_e158362_d_n0;
        var_isub_dn2 = assign105980_e158362_d_n2;
        var_isub_dn4 = assign105980_e158362_d_n4;
        var_isub_dn5 = assign105980_e158362_d_n5;
        var_isub_dn6 = assign105980_e158362_d_n6;
        var_isub_dn7 = assign105980_e158362_d_n7;
        var_isub_dn8 = assign105980_e158362_d_n8;
        var_isub_dn9 = assign105980_e158362_d_n9;
        var_isub_dn10 = assign105980_e158362_d_n10;
        var_isub_dn13 = assign105980_e158362_d_n13;

        let (assign105990_e158366, assign105990_e158366_d_n0, assign105990_e158366_d_n2, assign105990_e158366_d_n4, assign105990_e158366_d_n5, assign105990_e158366_d_n6, assign105990_e158366_d_n7, assign105990_e158366_d_n8, assign105990_e158366_d_n9, assign105990_e158366_d_n10, assign105990_e158366_d_n13,) = {
    if (var_guard2399 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isubs, var_isubs_dn0, var_isubs_dn2, var_isubs_dn4, var_isubs_dn5, var_isubs_dn6, var_isubs_dn7, var_isubs_dn8, var_isubs_dn9, var_isubs_dn10, var_isubs_dn13,)
    }
};
        var_isubs = assign105990_e158366;
        var_isubs_dn0 = assign105990_e158366_d_n0;
        var_isubs_dn2 = assign105990_e158366_d_n2;
        var_isubs_dn4 = assign105990_e158366_d_n4;
        var_isubs_dn5 = assign105990_e158366_d_n5;
        var_isubs_dn6 = assign105990_e158366_d_n6;
        var_isubs_dn7 = assign105990_e158366_d_n7;
        var_isubs_dn8 = assign105990_e158366_d_n8;
        var_isubs_dn9 = assign105990_e158366_d_n9;
        var_isubs_dn10 = assign105990_e158366_d_n10;
        var_isubs_dn13 = assign105990_e158366_d_n13;

        let (assign106000_e158370, assign106000_e158370_d_n0, assign106000_e158370_d_n2, assign106000_e158370_d_n4, assign106000_e158370_d_n5, assign106000_e158370_d_n6, assign106000_e158370_d_n7, assign106000_e158370_d_n8, assign106000_e158370_d_n9, assign106000_e158370_d_n10, assign106000_e158370_d_n13,) = {
    if (var_guard2399 != 0.0) {
        (var_isublde, var_isublde_dn0, var_isublde_dn2, var_isublde_dn4, var_isublde_dn5, var_isublde_dn6, var_isublde_dn7, var_isublde_dn8, var_isublde_dn9, var_isublde_dn10, var_isublde_dn13,)
    } else {
        (var_isubld, var_isubld_dn0, var_isubld_dn2, var_isubld_dn4, var_isubld_dn5, var_isubld_dn6, var_isubld_dn7, var_isubld_dn8, var_isubld_dn9, var_isubld_dn10, var_isubld_dn13,)
    }
};
        var_isubld = assign106000_e158370;
        var_isubld_dn0 = assign106000_e158370_d_n0;
        var_isubld_dn2 = assign106000_e158370_d_n2;
        var_isubld_dn4 = assign106000_e158370_d_n4;
        var_isubld_dn5 = assign106000_e158370_d_n5;
        var_isubld_dn6 = assign106000_e158370_d_n6;
        var_isubld_dn7 = assign106000_e158370_d_n7;
        var_isubld_dn8 = assign106000_e158370_d_n8;
        var_isubld_dn9 = assign106000_e158370_d_n9;
        var_isubld_dn10 = assign106000_e158370_d_n10;
        var_isubld_dn13 = assign106000_e158370_d_n13;

        let (assign106010_e158374, assign106010_e158374_d_n0, assign106010_e158374_d_n2, assign106010_e158374_d_n4, assign106010_e158374_d_n5, assign106010_e158374_d_n6, assign106010_e158374_d_n7, assign106010_e158374_d_n8, assign106010_e158374_d_n9, assign106010_e158374_d_n10, assign106010_e158374_d_n13,) = {
    if (var_guard2399 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isublds, var_isublds_dn0, var_isublds_dn2, var_isublds_dn4, var_isublds_dn5, var_isublds_dn6, var_isublds_dn7, var_isublds_dn8, var_isublds_dn9, var_isublds_dn10, var_isublds_dn13,)
    }
};
        var_isublds = assign106010_e158374;
        var_isublds_dn0 = assign106010_e158374_d_n0;
        var_isublds_dn2 = assign106010_e158374_d_n2;
        var_isublds_dn4 = assign106010_e158374_d_n4;
        var_isublds_dn5 = assign106010_e158374_d_n5;
        var_isublds_dn6 = assign106010_e158374_d_n6;
        var_isublds_dn7 = assign106010_e158374_d_n7;
        var_isublds_dn8 = assign106010_e158374_d_n8;
        var_isublds_dn9 = assign106010_e158374_d_n9;
        var_isublds_dn10 = assign106010_e158374_d_n10;
        var_isublds_dn13 = assign106010_e158374_d_n13;

        let (assign106020_e158378, assign106020_e158378_d_n0, assign106020_e158378_d_n2, assign106020_e158378_d_n4, assign106020_e158378_d_n5, assign106020_e158378_d_n6, assign106020_e158378_d_n7, assign106020_e158378_d_n8, assign106020_e158378_d_n9, assign106020_e158378_d_n10, assign106020_e158378_d_n13,) = {
    if (var_guard2399 != 0.0) {
        (var_idsibpce, var_idsibpce_dn0, var_idsibpce_dn2, var_idsibpce_dn4, var_idsibpce_dn5, var_idsibpce_dn6, var_idsibpce_dn7, var_idsibpce_dn8, var_idsibpce_dn9, var_idsibpce_dn10, var_idsibpce_dn13,)
    } else {
        (var_idsibpc, var_idsibpc_dn0, var_idsibpc_dn2, var_idsibpc_dn4, var_idsibpc_dn5, var_idsibpc_dn6, var_idsibpc_dn7, var_idsibpc_dn8, var_idsibpc_dn9, var_idsibpc_dn10, var_idsibpc_dn13,)
    }
};
        var_idsibpc = assign106020_e158378;
        var_idsibpc_dn0 = assign106020_e158378_d_n0;
        var_idsibpc_dn2 = assign106020_e158378_d_n2;
        var_idsibpc_dn4 = assign106020_e158378_d_n4;
        var_idsibpc_dn5 = assign106020_e158378_d_n5;
        var_idsibpc_dn6 = assign106020_e158378_d_n6;
        var_idsibpc_dn7 = assign106020_e158378_d_n7;
        var_idsibpc_dn8 = assign106020_e158378_d_n8;
        var_idsibpc_dn9 = assign106020_e158378_d_n9;
        var_idsibpc_dn10 = assign106020_e158378_d_n10;
        var_idsibpc_dn13 = assign106020_e158378_d_n13;

        let (assign106030_e158382, assign106030_e158382_d_n0, assign106030_e158382_d_n2, assign106030_e158382_d_n4, assign106030_e158382_d_n5, assign106030_e158382_d_n6, assign106030_e158382_d_n7, assign106030_e158382_d_n8, assign106030_e158382_d_n9, assign106030_e158382_d_n10, assign106030_e158382_d_n13,) = {
    if (var_guard2399 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_idsibpcs, var_idsibpcs_dn0, var_idsibpcs_dn2, var_idsibpcs_dn4, var_idsibpcs_dn5, var_idsibpcs_dn6, var_idsibpcs_dn7, var_idsibpcs_dn8, var_idsibpcs_dn9, var_idsibpcs_dn10, var_idsibpcs_dn13,)
    }
};
        var_idsibpcs = assign106030_e158382;
        var_idsibpcs_dn0 = assign106030_e158382_d_n0;
        var_idsibpcs_dn2 = assign106030_e158382_d_n2;
        var_idsibpcs_dn4 = assign106030_e158382_d_n4;
        var_idsibpcs_dn5 = assign106030_e158382_d_n5;
        var_idsibpcs_dn6 = assign106030_e158382_d_n6;
        var_idsibpcs_dn7 = assign106030_e158382_d_n7;
        var_idsibpcs_dn8 = assign106030_e158382_d_n8;
        var_idsibpcs_dn9 = assign106030_e158382_d_n9;
        var_idsibpcs_dn10 = assign106030_e158382_d_n10;
        var_idsibpcs_dn13 = assign106030_e158382_d_n13;

        let (assign106040_e158386, assign106040_e158386_d_n0, assign106040_e158386_d_n2, assign106040_e158386_d_n4, assign106040_e158386_d_n5, assign106040_e158386_d_n6, assign106040_e158386_d_n7, assign106040_e158386_d_n8, assign106040_e158386_d_n9, assign106040_e158386_d_n10, assign106040_e158386_d_n13,) = {
    if (var_guard2399 != 0.0) {
        (var_ibjte, var_ibjte_dn0, var_ibjte_dn2, var_ibjte_dn4, var_ibjte_dn5, var_ibjte_dn6, var_ibjte_dn7, var_ibjte_dn8, var_ibjte_dn9, var_ibjte_dn10, var_ibjte_dn13,)
    } else {
        (var_ibjt, var_ibjt_dn0, var_ibjt_dn2, var_ibjt_dn4, var_ibjt_dn5, var_ibjt_dn6, var_ibjt_dn7, var_ibjt_dn8, var_ibjt_dn9, var_ibjt_dn10, var_ibjt_dn13,)
    }
};
        var_ibjt = assign106040_e158386;
        var_ibjt_dn0 = assign106040_e158386_d_n0;
        var_ibjt_dn2 = assign106040_e158386_d_n2;
        var_ibjt_dn4 = assign106040_e158386_d_n4;
        var_ibjt_dn5 = assign106040_e158386_d_n5;
        var_ibjt_dn6 = assign106040_e158386_d_n6;
        var_ibjt_dn7 = assign106040_e158386_d_n7;
        var_ibjt_dn8 = assign106040_e158386_d_n8;
        var_ibjt_dn9 = assign106040_e158386_d_n9;
        var_ibjt_dn10 = assign106040_e158386_d_n10;
        var_ibjt_dn13 = assign106040_e158386_d_n13;

        let (assign106050_e158390, assign106050_e158390_d_n0, assign106050_e158390_d_n2, assign106050_e158390_d_n4, assign106050_e158390_d_n5, assign106050_e158390_d_n6, assign106050_e158390_d_n7, assign106050_e158390_d_n8, assign106050_e158390_d_n9, assign106050_e158390_d_n10, assign106050_e158390_d_n13,) = {
    if (var_guard2399 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibjts, var_ibjts_dn0, var_ibjts_dn2, var_ibjts_dn4, var_ibjts_dn5, var_ibjts_dn6, var_ibjts_dn7, var_ibjts_dn8, var_ibjts_dn9, var_ibjts_dn10, var_ibjts_dn13,)
    }
};
        var_ibjts = assign106050_e158390;
        var_ibjts_dn0 = assign106050_e158390_d_n0;
        var_ibjts_dn2 = assign106050_e158390_d_n2;
        var_ibjts_dn4 = assign106050_e158390_d_n4;
        var_ibjts_dn5 = assign106050_e158390_d_n5;
        var_ibjts_dn6 = assign106050_e158390_d_n6;
        var_ibjts_dn7 = assign106050_e158390_d_n7;
        var_ibjts_dn8 = assign106050_e158390_d_n8;
        var_ibjts_dn9 = assign106050_e158390_d_n9;
        var_ibjts_dn10 = assign106050_e158390_d_n10;
        var_ibjts_dn13 = assign106050_e158390_d_n13;

        let (assign106060_e158394, assign106060_e158394_d_n0, assign106060_e158394_d_n2, assign106060_e158394_d_n4, assign106060_e158394_d_n5, assign106060_e158394_d_n6, assign106060_e158394_d_n7, assign106060_e158394_d_n8, assign106060_e158394_d_n9, assign106060_e158394_d_n10, assign106060_e158394_d_n13,) = {
    if (var_guard2399 != 0.0) {
        (var_ibreake, var_ibreake_dn0, var_ibreake_dn2, var_ibreake_dn4, var_ibreake_dn5, var_ibreake_dn6, var_ibreake_dn7, var_ibreake_dn8, var_ibreake_dn9, var_ibreake_dn10, var_ibreake_dn13,)
    } else {
        (var_ibreak, var_ibreak_dn0, var_ibreak_dn2, var_ibreak_dn4, var_ibreak_dn5, var_ibreak_dn6, var_ibreak_dn7, var_ibreak_dn8, var_ibreak_dn9, var_ibreak_dn10, var_ibreak_dn13,)
    }
};
        var_ibreak = assign106060_e158394;
        var_ibreak_dn0 = assign106060_e158394_d_n0;
        var_ibreak_dn2 = assign106060_e158394_d_n2;
        var_ibreak_dn4 = assign106060_e158394_d_n4;
        var_ibreak_dn5 = assign106060_e158394_d_n5;
        var_ibreak_dn6 = assign106060_e158394_d_n6;
        var_ibreak_dn7 = assign106060_e158394_d_n7;
        var_ibreak_dn8 = assign106060_e158394_d_n8;
        var_ibreak_dn9 = assign106060_e158394_d_n9;
        var_ibreak_dn10 = assign106060_e158394_d_n10;
        var_ibreak_dn13 = assign106060_e158394_d_n13;

        let (assign106070_e158398, assign106070_e158398_d_n0, assign106070_e158398_d_n2, assign106070_e158398_d_n4, assign106070_e158398_d_n5, assign106070_e158398_d_n6, assign106070_e158398_d_n7, assign106070_e158398_d_n8, assign106070_e158398_d_n9, assign106070_e158398_d_n10, assign106070_e158398_d_n13,) = {
    if (var_guard2399 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibreaks, var_ibreaks_dn0, var_ibreaks_dn2, var_ibreaks_dn4, var_ibreaks_dn5, var_ibreaks_dn6, var_ibreaks_dn7, var_ibreaks_dn8, var_ibreaks_dn9, var_ibreaks_dn10, var_ibreaks_dn13,)
    }
};
        var_ibreaks = assign106070_e158398;
        var_ibreaks_dn0 = assign106070_e158398_d_n0;
        var_ibreaks_dn2 = assign106070_e158398_d_n2;
        var_ibreaks_dn4 = assign106070_e158398_d_n4;
        var_ibreaks_dn5 = assign106070_e158398_d_n5;
        var_ibreaks_dn6 = assign106070_e158398_d_n6;
        var_ibreaks_dn7 = assign106070_e158398_d_n7;
        var_ibreaks_dn8 = assign106070_e158398_d_n8;
        var_ibreaks_dn9 = assign106070_e158398_d_n9;
        var_ibreaks_dn10 = assign106070_e158398_d_n10;
        var_ibreaks_dn13 = assign106070_e158398_d_n13;

        let (assign106080_e158402, assign106080_e158402_d_n0, assign106080_e158402_d_n2, assign106080_e158402_d_n4, assign106080_e158402_d_n5, assign106080_e158402_d_n6, assign106080_e158402_d_n7, assign106080_e158402_d_n8, assign106080_e158402_d_n9, assign106080_e158402_d_n10, assign106080_e158402_d_n13,) = {
    if (var_guard2399 != 0.0) {
        (var_igidle, var_igidle_dn0, var_igidle_dn2, var_igidle_dn4, var_igidle_dn5, var_igidle_dn6, var_igidle_dn7, var_igidle_dn8, var_igidle_dn9, var_igidle_dn10, var_igidle_dn13,)
    } else {
        (var_igidl, var_igidl_dn0, var_igidl_dn2, var_igidl_dn4, var_igidl_dn5, var_igidl_dn6, var_igidl_dn7, var_igidl_dn8, var_igidl_dn9, var_igidl_dn10, var_igidl_dn13,)
    }
};
        var_igidl = assign106080_e158402;
        var_igidl_dn0 = assign106080_e158402_d_n0;
        var_igidl_dn2 = assign106080_e158402_d_n2;
        var_igidl_dn4 = assign106080_e158402_d_n4;
        var_igidl_dn5 = assign106080_e158402_d_n5;
        var_igidl_dn6 = assign106080_e158402_d_n6;
        var_igidl_dn7 = assign106080_e158402_d_n7;
        var_igidl_dn8 = assign106080_e158402_d_n8;
        var_igidl_dn9 = assign106080_e158402_d_n9;
        var_igidl_dn10 = assign106080_e158402_d_n10;
        var_igidl_dn13 = assign106080_e158402_d_n13;

        let (assign106090_e158406, assign106090_e158406_d_n0, assign106090_e158406_d_n2, assign106090_e158406_d_n4, assign106090_e158406_d_n5, assign106090_e158406_d_n6, assign106090_e158406_d_n7, assign106090_e158406_d_n8, assign106090_e158406_d_n9, assign106090_e158406_d_n10, assign106090_e158406_d_n13,) = {
    if (var_guard2399 != 0.0) {
        (var_igisle, var_igisle_dn0, var_igisle_dn2, var_igisle_dn4, var_igisle_dn5, var_igisle_dn6, var_igisle_dn7, var_igisle_dn8, var_igisle_dn9, var_igisle_dn10, var_igisle_dn13,)
    } else {
        (var_igisl, var_igisl_dn0, var_igisl_dn2, var_igisl_dn4, var_igisl_dn5, var_igisl_dn6, var_igisl_dn7, var_igisl_dn8, var_igisl_dn9, var_igisl_dn10, var_igisl_dn13,)
    }
};
        var_igisl = assign106090_e158406;
        var_igisl_dn0 = assign106090_e158406_d_n0;
        var_igisl_dn2 = assign106090_e158406_d_n2;
        var_igisl_dn4 = assign106090_e158406_d_n4;
        var_igisl_dn5 = assign106090_e158406_d_n5;
        var_igisl_dn6 = assign106090_e158406_d_n6;
        var_igisl_dn7 = assign106090_e158406_d_n7;
        var_igisl_dn8 = assign106090_e158406_d_n8;
        var_igisl_dn9 = assign106090_e158406_d_n9;
        var_igisl_dn10 = assign106090_e158406_d_n10;
        var_igisl_dn13 = assign106090_e158406_d_n13;

        let (assign106100_e158412, assign106100_e158412_d_n0, assign106100_e158412_d_n2, assign106100_e158412_d_n4, assign106100_e158412_d_n5, assign106100_e158412_d_n6, assign106100_e158412_d_n7, assign106100_e158412_d_n8, assign106100_e158412_d_n9, assign106100_e158412_d_n10, assign106100_e158412_d_n13,) = {
    if ((var_guard2399 != 0.0) && (var_flg_nqs != 0.0)) {
        (var_xd, var_xd_dn0, var_xd_dn2, var_xd_dn4, var_xd_dn5, var_xd_dn6, var_xd_dn7, var_xd_dn8, var_xd_dn9, var_xd_dn10, var_xd_dn13,)
    } else {
        (var_qdrat, var_qdrat_dn0, var_qdrat_dn2, var_qdrat_dn4, var_qdrat_dn5, var_qdrat_dn6, var_qdrat_dn7, var_qdrat_dn8, var_qdrat_dn9, var_qdrat_dn10, var_qdrat_dn13,)
    }
};
        var_qdrat = assign106100_e158412;
        var_qdrat_dn0 = assign106100_e158412_d_n0;
        var_qdrat_dn2 = assign106100_e158412_d_n2;
        var_qdrat_dn4 = assign106100_e158412_d_n4;
        var_qdrat_dn5 = assign106100_e158412_d_n5;
        var_qdrat_dn6 = assign106100_e158412_d_n6;
        var_qdrat_dn7 = assign106100_e158412_d_n7;
        var_qdrat_dn8 = assign106100_e158412_d_n8;
        var_qdrat_dn9 = assign106100_e158412_d_n9;
        var_qdrat_dn10 = assign106100_e158412_d_n10;
        var_qdrat_dn13 = assign106100_e158412_d_n13;

        let (assign106110_e158418, assign106110_e158418_d_n0, assign106110_e158418_d_n2, assign106110_e158418_d_n4, assign106110_e158418_d_n5, assign106110_e158418_d_n6, assign106110_e158418_d_n7, assign106110_e158418_d_n8, assign106110_e158418_d_n9, assign106110_e158418_d_n10, assign106110_e158418_d_n13,) = {
    if (var_guard2399 == 0.0) {
        let assign106110_e158416: f64 = (-var_idse);
        (assign106110_e158416, (-var_idse_dn0), (-var_idse_dn2), (-var_idse_dn4), (-var_idse_dn5), (-var_idse_dn6), (-var_idse_dn7), (-var_idse_dn8), (-var_idse_dn9), (-var_idse_dn10), (-var_idse_dn13),)
    } else {
        (var_ids, var_ids_dn0, var_ids_dn2, var_ids_dn4, var_ids_dn5, var_ids_dn6, var_ids_dn7, var_ids_dn8, var_ids_dn9, var_ids_dn10, var_ids_dn13,)
    }
};
        var_ids = assign106110_e158418;
        var_ids_dn0 = assign106110_e158418_d_n0;
        var_ids_dn2 = assign106110_e158418_d_n2;
        var_ids_dn4 = assign106110_e158418_d_n4;
        var_ids_dn5 = assign106110_e158418_d_n5;
        var_ids_dn6 = assign106110_e158418_d_n6;
        var_ids_dn7 = assign106110_e158418_d_n7;
        var_ids_dn8 = assign106110_e158418_d_n8;
        var_ids_dn9 = assign106110_e158418_d_n9;
        var_ids_dn10 = assign106110_e158418_d_n10;
        var_ids_dn13 = assign106110_e158418_d_n13;

        let (assign106120_e158423, assign106120_e158423_d_n0, assign106120_e158423_d_n2, assign106120_e158423_d_n4, assign106120_e158423_d_n5, assign106120_e158423_d_n6, assign106120_e158423_d_n7, assign106120_e158423_d_n8, assign106120_e158423_d_n9, assign106120_e158423_d_n10, assign106120_e158423_d_n13,) = {
    if (var_guard2399 == 0.0) {
        (var_qse, var_qse_dn0, var_qse_dn2, var_qse_dn4, var_qse_dn5, var_qse_dn6, var_qse_dn7, var_qse_dn8, var_qse_dn9, var_qse_dn10, var_qse_dn13,)
    } else {
        (var_qd, var_qd_dn0, var_qd_dn2, var_qd_dn4, var_qd_dn5, var_qd_dn6, var_qd_dn7, var_qd_dn8, var_qd_dn9, var_qd_dn10, var_qd_dn13,)
    }
};
        var_qd = assign106120_e158423;
        var_qd_dn0 = assign106120_e158423_d_n0;
        var_qd_dn2 = assign106120_e158423_d_n2;
        var_qd_dn4 = assign106120_e158423_d_n4;
        var_qd_dn5 = assign106120_e158423_d_n5;
        var_qd_dn6 = assign106120_e158423_d_n6;
        var_qd_dn7 = assign106120_e158423_d_n7;
        var_qd_dn8 = assign106120_e158423_d_n8;
        var_qd_dn9 = assign106120_e158423_d_n9;
        var_qd_dn10 = assign106120_e158423_d_n10;
        var_qd_dn13 = assign106120_e158423_d_n13;

        let (assign106130_e158428, assign106130_e158428_d_n0, assign106130_e158428_d_n2, assign106130_e158428_d_n4, assign106130_e158428_d_n5, assign106130_e158428_d_n6, assign106130_e158428_d_n7, assign106130_e158428_d_n8, assign106130_e158428_d_n9, assign106130_e158428_d_n10, assign106130_e158428_d_n13,) = {
    if (var_guard2399 == 0.0) {
        (var_qge, var_qge_dn0, var_qge_dn2, var_qge_dn4, var_qge_dn5, var_qge_dn6, var_qge_dn7, var_qge_dn8, var_qge_dn9, var_qge_dn10, var_qge_dn13,)
    } else {
        (var_qg, var_qg_dn0, var_qg_dn2, var_qg_dn4, var_qg_dn5, var_qg_dn6, var_qg_dn7, var_qg_dn8, var_qg_dn9, var_qg_dn10, var_qg_dn13,)
    }
};
        var_qg = assign106130_e158428;
        var_qg_dn0 = assign106130_e158428_d_n0;
        var_qg_dn2 = assign106130_e158428_d_n2;
        var_qg_dn4 = assign106130_e158428_d_n4;
        var_qg_dn5 = assign106130_e158428_d_n5;
        var_qg_dn6 = assign106130_e158428_d_n6;
        var_qg_dn7 = assign106130_e158428_d_n7;
        var_qg_dn8 = assign106130_e158428_d_n8;
        var_qg_dn9 = assign106130_e158428_d_n9;
        var_qg_dn10 = assign106130_e158428_d_n10;
        var_qg_dn13 = assign106130_e158428_d_n13;

        let (assign106140_e158433, assign106140_e158433_d_n0, assign106140_e158433_d_n2, assign106140_e158433_d_n4, assign106140_e158433_d_n5, assign106140_e158433_d_n6, assign106140_e158433_d_n7, assign106140_e158433_d_n8, assign106140_e158433_d_n9, assign106140_e158433_d_n10, assign106140_e158433_d_n13,) = {
    if (var_guard2399 == 0.0) {
        (var_qde, var_qde_dn0, var_qde_dn2, var_qde_dn4, var_qde_dn5, var_qde_dn6, var_qde_dn7, var_qde_dn8, var_qde_dn9, var_qde_dn10, var_qde_dn13,)
    } else {
        (var_qs, var_qs_dn0, var_qs_dn2, var_qs_dn4, var_qs_dn5, var_qs_dn6, var_qs_dn7, var_qs_dn8, var_qs_dn9, var_qs_dn10, var_qs_dn13,)
    }
};
        var_qs = assign106140_e158433;
        var_qs_dn0 = assign106140_e158433_d_n0;
        var_qs_dn2 = assign106140_e158433_d_n2;
        var_qs_dn4 = assign106140_e158433_d_n4;
        var_qs_dn5 = assign106140_e158433_d_n5;
        var_qs_dn6 = assign106140_e158433_d_n6;
        var_qs_dn7 = assign106140_e158433_d_n7;
        var_qs_dn8 = assign106140_e158433_d_n8;
        var_qs_dn9 = assign106140_e158433_d_n9;
        var_qs_dn10 = assign106140_e158433_d_n10;
        var_qs_dn13 = assign106140_e158433_d_n13;

        *var_guard2399_slot = var_guard2399;
        *var_ibjt_slot = var_ibjt;
        *var_ibjt_dn0_slot = var_ibjt_dn0;
        *var_ibjt_dn10_slot = var_ibjt_dn10;
        *var_ibjt_dn13_slot = var_ibjt_dn13;
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
        *var_ibjts_dn13_slot = var_ibjts_dn13;
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
        *var_ibreak_dn13_slot = var_ibreak_dn13;
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
        *var_ibreaks_dn13_slot = var_ibreaks_dn13;
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
        *var_ids_dn13_slot = var_ids_dn13;
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
        *var_idsibpc_dn13_slot = var_idsibpc_dn13;
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
        *var_idsibpcs_dn13_slot = var_idsibpcs_dn13;
        *var_idsibpcs_dn2_slot = var_idsibpcs_dn2;
        *var_idsibpcs_dn4_slot = var_idsibpcs_dn4;
        *var_idsibpcs_dn5_slot = var_idsibpcs_dn5;
        *var_idsibpcs_dn6_slot = var_idsibpcs_dn6;
        *var_idsibpcs_dn7_slot = var_idsibpcs_dn7;
        *var_idsibpcs_dn8_slot = var_idsibpcs_dn8;
        *var_idsibpcs_dn9_slot = var_idsibpcs_dn9;
        *var_igb_slot = var_igb;
        *var_igb_dn0_slot = var_igb_dn0;
        *var_igb_dn10_slot = var_igb_dn10;
        *var_igb_dn13_slot = var_igb_dn13;
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
        *var_igd_dn13_slot = var_igd_dn13;
        *var_igd_dn2_slot = var_igd_dn2;
        *var_igd_dn4_slot = var_igd_dn4;
        *var_igd_dn5_slot = var_igd_dn5;
        *var_igd_dn6_slot = var_igd_dn6;
        *var_igd_dn7_slot = var_igd_dn7;
        *var_igd_dn8_slot = var_igd_dn8;
        *var_igd_dn9_slot = var_igd_dn9;
        *var_igidl_slot = var_igidl;
        *var_igidl_dn0_slot = var_igidl_dn0;
        *var_igidl_dn10_slot = var_igidl_dn10;
        *var_igidl_dn13_slot = var_igidl_dn13;
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
        *var_igisl_dn13_slot = var_igisl_dn13;
        *var_igisl_dn2_slot = var_igisl_dn2;
        *var_igisl_dn4_slot = var_igisl_dn4;
        *var_igisl_dn5_slot = var_igisl_dn5;
        *var_igisl_dn6_slot = var_igisl_dn6;
        *var_igisl_dn7_slot = var_igisl_dn7;
        *var_igisl_dn8_slot = var_igisl_dn8;
        *var_igisl_dn9_slot = var_igisl_dn9;
        *var_igs_slot = var_igs;
        *var_igs_dn0_slot = var_igs_dn0;
        *var_igs_dn10_slot = var_igs_dn10;
        *var_igs_dn13_slot = var_igs_dn13;
        *var_igs_dn2_slot = var_igs_dn2;
        *var_igs_dn4_slot = var_igs_dn4;
        *var_igs_dn5_slot = var_igs_dn5;
        *var_igs_dn6_slot = var_igs_dn6;
        *var_igs_dn7_slot = var_igs_dn7;
        *var_igs_dn8_slot = var_igs_dn8;
        *var_igs_dn9_slot = var_igs_dn9;
        *var_isub_slot = var_isub;
        *var_isub_dn0_slot = var_isub_dn0;
        *var_isub_dn10_slot = var_isub_dn10;
        *var_isub_dn13_slot = var_isub_dn13;
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
        *var_isubld_dn13_slot = var_isubld_dn13;
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
        *var_isublds_dn13_slot = var_isublds_dn13;
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
        *var_isubs_dn13_slot = var_isubs_dn13;
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
        *var_qb_dn13_slot = var_qb_dn13;
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
        *var_qd_dn13_slot = var_qd_dn13;
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
        *var_qdrat_dn13_slot = var_qdrat_dn13;
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
        *var_qg_dn13_slot = var_qg_dn13;
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
        *var_qs_dn13_slot = var_qs_dn13;
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
        *var_rdd_dn13_slot = var_rdd_dn13;
        *var_rdd_dn2_slot = var_rdd_dn2;
        *var_rdd_dn4_slot = var_rdd_dn4;
        *var_rdd_dn5_slot = var_rdd_dn5;
        *var_rdd_dn6_slot = var_rdd_dn6;
        *var_rdd_dn7_slot = var_rdd_dn7;
        *var_rdd_dn8_slot = var_rdd_dn8;
        *var_rdd_dn9_slot = var_rdd_dn9;
        *var_rsd_slot = var_rsd;
        *var_rsd_dn0_slot = var_rsd_dn0;
        *var_rsd_dn10_slot = var_rsd_dn10;
        *var_rsd_dn13_slot = var_rsd_dn13;
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
        *var_rsde_dn13_slot = var_rsde_dn13;
        *var_rsde_dn2_slot = var_rsde_dn2;
        *var_rsde_dn4_slot = var_rsde_dn4;
        *var_rsde_dn5_slot = var_rsde_dn5;
        *var_rsde_dn6_slot = var_rsde_dn6;
        *var_rsde_dn7_slot = var_rsde_dn7;
        *var_rsde_dn8_slot = var_rsde_dn8;
        *var_rsde_dn9_slot = var_rsde_dn9;
    }

    pub(super) fn stamp_transient_block_375(
        p: &Parameters,
        var_flg_nqs: f64,
        var_guard2399: f64,
        var_ibjte: f64,
        var_ibjte_dn0: f64,
        var_ibjte_dn10: f64,
        var_ibjte_dn13: f64,
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
        var_ibreake_dn13: f64,
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
        var_ids_dn13: f64,
        var_ids_dn2: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_ids_dn9: f64,
        var_idsibpce: f64,
        var_idsibpce_dn0: f64,
        var_idsibpce_dn10: f64,
        var_idsibpce_dn13: f64,
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
        var_igidle_dn13: f64,
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
        var_igisle_dn13: f64,
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
        var_isube_dn13: f64,
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
        var_isublde_dn13: f64,
        var_isublde_dn2: f64,
        var_isublde_dn4: f64,
        var_isublde_dn5: f64,
        var_isublde_dn6: f64,
        var_isublde_dn7: f64,
        var_isublde_dn8: f64,
        var_isublde_dn9: f64,
        var_powratio: f64,
        var_powratio_dn0: f64,
        var_powratio_dn10: f64,
        var_powratio_dn13: f64,
        var_powratio_dn2: f64,
        var_powratio_dn4: f64,
        var_powratio_dn5: f64,
        var_powratio_dn6: f64,
        var_powratio_dn7: f64,
        var_powratio_dn8: f64,
        var_powratio_dn9: f64,
        var_qde: f64,
        var_qde_dn0: f64,
        var_qde_dn10: f64,
        var_qde_dn13: f64,
        var_qde_dn2: f64,
        var_qde_dn4: f64,
        var_qde_dn5: f64,
        var_qde_dn6: f64,
        var_qde_dn7: f64,
        var_qde_dn8: f64,
        var_qde_dn9: f64,
        var_qdexte: f64,
        var_qdexte_dn0: f64,
        var_qdexte_dn10: f64,
        var_qdexte_dn13: f64,
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
        var_qdov_dn13: f64,
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
        var_qdp_dn6: f64,
        var_qge: f64,
        var_qge_dn0: f64,
        var_qge_dn10: f64,
        var_qge_dn13: f64,
        var_qge_dn2: f64,
        var_qge_dn4: f64,
        var_qge_dn5: f64,
        var_qge_dn6: f64,
        var_qge_dn7: f64,
        var_qge_dn8: f64,
        var_qge_dn9: f64,
        var_qgexte: f64,
        var_qgexte_dn0: f64,
        var_qgexte_dn10: f64,
        var_qgexte_dn13: f64,
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
        var_qgov_dn13: f64,
        var_qgov_dn2: f64,
        var_qgov_dn4: f64,
        var_qgov_dn5: f64,
        var_qgov_dn6: f64,
        var_qgov_dn7: f64,
        var_qgov_dn8: f64,
        var_qgov_dn9: f64,
        var_qse: f64,
        var_qse_dn0: f64,
        var_qse_dn10: f64,
        var_qse_dn13: f64,
        var_qse_dn2: f64,
        var_qse_dn4: f64,
        var_qse_dn5: f64,
        var_qse_dn6: f64,
        var_qse_dn7: f64,
        var_qse_dn8: f64,
        var_qse_dn9: f64,
        var_qsexte: f64,
        var_qsexte_dn0: f64,
        var_qsexte_dn10: f64,
        var_qsexte_dn13: f64,
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
        var_qsov_dn13: f64,
        var_qsov_dn2: f64,
        var_qsov_dn4: f64,
        var_qsov_dn5: f64,
        var_qsov_dn6: f64,
        var_qsov_dn7: f64,
        var_qsov_dn8: f64,
        var_qsov_dn9: f64,
        var_qsp: f64,
        var_qsp_dn2: f64,
        var_qsp_dn6: f64,
        var_rth: f64,
        var_rth_dn0: f64,
        var_rth_dn10: f64,
        var_rth_dn13: f64,
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
        var_vdsi_dn5: f64,
        var_vdsi_dn7: f64,
        var_xd: f64,
        var_xd_dn0: f64,
        var_xd_dn10: f64,
        var_xd_dn13: f64,
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
        var_gth_dn13_slot: &mut f64,
        var_gth_dn2_slot: &mut f64,
        var_gth_dn4_slot: &mut f64,
        var_gth_dn5_slot: &mut f64,
        var_gth_dn6_slot: &mut f64,
        var_gth_dn7_slot: &mut f64,
        var_gth_dn8_slot: &mut f64,
        var_gth_dn9_slot: &mut f64,
        var_guard2400_slot: &mut f64,
        var_guard2401_slot: &mut f64,
        var_guard2402_slot: &mut f64,
        var_guard2403_slot: &mut f64,
        var_guard2404_slot: &mut f64,
        var_ibjt_slot: &mut f64,
        var_ibjt_dn0_slot: &mut f64,
        var_ibjt_dn10_slot: &mut f64,
        var_ibjt_dn13_slot: &mut f64,
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
        var_ibjts_dn13_slot: &mut f64,
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
        var_ibreak_dn13_slot: &mut f64,
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
        var_ibreaks_dn13_slot: &mut f64,
        var_ibreaks_dn2_slot: &mut f64,
        var_ibreaks_dn4_slot: &mut f64,
        var_ibreaks_dn5_slot: &mut f64,
        var_ibreaks_dn6_slot: &mut f64,
        var_ibreaks_dn7_slot: &mut f64,
        var_ibreaks_dn8_slot: &mut f64,
        var_ibreaks_dn9_slot: &mut f64,
        var_idsibpc_slot: &mut f64,
        var_idsibpc_dn0_slot: &mut f64,
        var_idsibpc_dn10_slot: &mut f64,
        var_idsibpc_dn13_slot: &mut f64,
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
        var_idsibpcs_dn13_slot: &mut f64,
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
        var_igidl_dn13_slot: &mut f64,
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
        var_igisl_dn13_slot: &mut f64,
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
        var_isub_dn13_slot: &mut f64,
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
        var_isubld_dn13_slot: &mut f64,
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
        var_isublds_dn13_slot: &mut f64,
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
        var_isubs_dn13_slot: &mut f64,
        var_isubs_dn2_slot: &mut f64,
        var_isubs_dn4_slot: &mut f64,
        var_isubs_dn5_slot: &mut f64,
        var_isubs_dn6_slot: &mut f64,
        var_isubs_dn7_slot: &mut f64,
        var_isubs_dn8_slot: &mut f64,
        var_isubs_dn9_slot: &mut f64,
        var_p_slot: &mut f64,
        var_p_dn0_slot: &mut f64,
        var_p_dn10_slot: &mut f64,
        var_p_dn13_slot: &mut f64,
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
        var_qb_dn13_slot: &mut f64,
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
        var_qbext_dn13_slot: &mut f64,
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
        var_qd_dn13_slot: &mut f64,
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
        var_qdext_dn13_slot: &mut f64,
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
        var_qdrat_dn13_slot: &mut f64,
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
        var_qfd_dn6_slot: &mut f64,
        var_qfs_slot: &mut f64,
        var_qfs_dn2_slot: &mut f64,
        var_qfs_dn6_slot: &mut f64,
        var_qg_slot: &mut f64,
        var_qg_dn0_slot: &mut f64,
        var_qg_dn10_slot: &mut f64,
        var_qg_dn13_slot: &mut f64,
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
        var_qgext_dn13_slot: &mut f64,
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
        var_qs_dn13_slot: &mut f64,
        var_qs_dn2_slot: &mut f64,
        var_qs_dn4_slot: &mut f64,
        var_qs_dn5_slot: &mut f64,
        var_qs_dn6_slot: &mut f64,
        var_qs_dn7_slot: &mut f64,
        var_qs_dn8_slot: &mut f64,
        var_qs_dn9_slot: &mut f64,
        var_veffpower_slot: &mut f64,
        var_veffpower_dn0_slot: &mut f64,
        var_veffpower_dn10_slot: &mut f64,
        var_veffpower_dn13_slot: &mut f64,
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
        let mut var_gth_dn13: f64 = *var_gth_dn13_slot;
        let mut var_gth_dn2: f64 = *var_gth_dn2_slot;
        let mut var_gth_dn4: f64 = *var_gth_dn4_slot;
        let mut var_gth_dn5: f64 = *var_gth_dn5_slot;
        let mut var_gth_dn6: f64 = *var_gth_dn6_slot;
        let mut var_gth_dn7: f64 = *var_gth_dn7_slot;
        let mut var_gth_dn8: f64 = *var_gth_dn8_slot;
        let mut var_gth_dn9: f64 = *var_gth_dn9_slot;
        let mut var_guard2400: f64 = *var_guard2400_slot;
        let mut var_guard2401: f64 = *var_guard2401_slot;
        let mut var_guard2402: f64 = *var_guard2402_slot;
        let mut var_guard2403: f64 = *var_guard2403_slot;
        let mut var_guard2404: f64 = *var_guard2404_slot;
        let mut var_ibjt: f64 = *var_ibjt_slot;
        let mut var_ibjt_dn0: f64 = *var_ibjt_dn0_slot;
        let mut var_ibjt_dn10: f64 = *var_ibjt_dn10_slot;
        let mut var_ibjt_dn13: f64 = *var_ibjt_dn13_slot;
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
        let mut var_ibjts_dn13: f64 = *var_ibjts_dn13_slot;
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
        let mut var_ibreak_dn13: f64 = *var_ibreak_dn13_slot;
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
        let mut var_ibreaks_dn13: f64 = *var_ibreaks_dn13_slot;
        let mut var_ibreaks_dn2: f64 = *var_ibreaks_dn2_slot;
        let mut var_ibreaks_dn4: f64 = *var_ibreaks_dn4_slot;
        let mut var_ibreaks_dn5: f64 = *var_ibreaks_dn5_slot;
        let mut var_ibreaks_dn6: f64 = *var_ibreaks_dn6_slot;
        let mut var_ibreaks_dn7: f64 = *var_ibreaks_dn7_slot;
        let mut var_ibreaks_dn8: f64 = *var_ibreaks_dn8_slot;
        let mut var_ibreaks_dn9: f64 = *var_ibreaks_dn9_slot;
        let mut var_idsibpc: f64 = *var_idsibpc_slot;
        let mut var_idsibpc_dn0: f64 = *var_idsibpc_dn0_slot;
        let mut var_idsibpc_dn10: f64 = *var_idsibpc_dn10_slot;
        let mut var_idsibpc_dn13: f64 = *var_idsibpc_dn13_slot;
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
        let mut var_idsibpcs_dn13: f64 = *var_idsibpcs_dn13_slot;
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
        let mut var_igidl_dn13: f64 = *var_igidl_dn13_slot;
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
        let mut var_igisl_dn13: f64 = *var_igisl_dn13_slot;
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
        let mut var_isub_dn13: f64 = *var_isub_dn13_slot;
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
        let mut var_isubld_dn13: f64 = *var_isubld_dn13_slot;
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
        let mut var_isublds_dn13: f64 = *var_isublds_dn13_slot;
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
        let mut var_isubs_dn13: f64 = *var_isubs_dn13_slot;
        let mut var_isubs_dn2: f64 = *var_isubs_dn2_slot;
        let mut var_isubs_dn4: f64 = *var_isubs_dn4_slot;
        let mut var_isubs_dn5: f64 = *var_isubs_dn5_slot;
        let mut var_isubs_dn6: f64 = *var_isubs_dn6_slot;
        let mut var_isubs_dn7: f64 = *var_isubs_dn7_slot;
        let mut var_isubs_dn8: f64 = *var_isubs_dn8_slot;
        let mut var_isubs_dn9: f64 = *var_isubs_dn9_slot;
        let mut var_p: f64 = *var_p_slot;
        let mut var_p_dn0: f64 = *var_p_dn0_slot;
        let mut var_p_dn10: f64 = *var_p_dn10_slot;
        let mut var_p_dn13: f64 = *var_p_dn13_slot;
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
        let mut var_qb_dn13: f64 = *var_qb_dn13_slot;
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
        let mut var_qbext_dn13: f64 = *var_qbext_dn13_slot;
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
        let mut var_qd_dn13: f64 = *var_qd_dn13_slot;
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
        let mut var_qdext_dn13: f64 = *var_qdext_dn13_slot;
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
        let mut var_qdrat_dn13: f64 = *var_qdrat_dn13_slot;
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
        let mut var_qfd_dn6: f64 = *var_qfd_dn6_slot;
        let mut var_qfs: f64 = *var_qfs_slot;
        let mut var_qfs_dn2: f64 = *var_qfs_dn2_slot;
        let mut var_qfs_dn6: f64 = *var_qfs_dn6_slot;
        let mut var_qg: f64 = *var_qg_slot;
        let mut var_qg_dn0: f64 = *var_qg_dn0_slot;
        let mut var_qg_dn10: f64 = *var_qg_dn10_slot;
        let mut var_qg_dn13: f64 = *var_qg_dn13_slot;
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
        let mut var_qgext_dn13: f64 = *var_qgext_dn13_slot;
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
        let mut var_qs_dn13: f64 = *var_qs_dn13_slot;
        let mut var_qs_dn2: f64 = *var_qs_dn2_slot;
        let mut var_qs_dn4: f64 = *var_qs_dn4_slot;
        let mut var_qs_dn5: f64 = *var_qs_dn5_slot;
        let mut var_qs_dn6: f64 = *var_qs_dn6_slot;
        let mut var_qs_dn7: f64 = *var_qs_dn7_slot;
        let mut var_qs_dn8: f64 = *var_qs_dn8_slot;
        let mut var_qs_dn9: f64 = *var_qs_dn9_slot;
        let mut var_veffpower: f64 = *var_veffpower_slot;
        let mut var_veffpower_dn0: f64 = *var_veffpower_dn0_slot;
        let mut var_veffpower_dn10: f64 = *var_veffpower_dn10_slot;
        let mut var_veffpower_dn13: f64 = *var_veffpower_dn13_slot;
        let mut var_veffpower_dn2: f64 = *var_veffpower_dn2_slot;
        let mut var_veffpower_dn4: f64 = *var_veffpower_dn4_slot;
        let mut var_veffpower_dn5: f64 = *var_veffpower_dn5_slot;
        let mut var_veffpower_dn6: f64 = *var_veffpower_dn6_slot;
        let mut var_veffpower_dn7: f64 = *var_veffpower_dn7_slot;
        let mut var_veffpower_dn8: f64 = *var_veffpower_dn8_slot;
        let mut var_veffpower_dn9: f64 = *var_veffpower_dn9_slot;

        let (assign106150_e158443, assign106150_e158443_d_n0, assign106150_e158443_d_n2, assign106150_e158443_d_n4, assign106150_e158443_d_n5, assign106150_e158443_d_n6, assign106150_e158443_d_n7, assign106150_e158443_d_n8, assign106150_e158443_d_n9, assign106150_e158443_d_n10, assign106150_e158443_d_n13,) = {
    if (var_guard2399 == 0.0) {
        let assign106150_e158438: f64 = (var_qge + var_qde);
        let assign106150_e158440: f64 = (assign106150_e158438 + var_qse);
        let assign106150_e158441: f64 = (-assign106150_e158440);
        (assign106150_e158441, (-((var_qge_dn0 + var_qde_dn0) + var_qse_dn0)), (-((var_qge_dn2 + var_qde_dn2) + var_qse_dn2)), (-((var_qge_dn4 + var_qde_dn4) + var_qse_dn4)), (-((var_qge_dn5 + var_qde_dn5) + var_qse_dn5)), (-((var_qge_dn6 + var_qde_dn6) + var_qse_dn6)), (-((var_qge_dn7 + var_qde_dn7) + var_qse_dn7)), (-((var_qge_dn8 + var_qde_dn8) + var_qse_dn8)), (-((var_qge_dn9 + var_qde_dn9) + var_qse_dn9)), (-((var_qge_dn10 + var_qde_dn10) + var_qse_dn10)), (-((var_qge_dn13 + var_qde_dn13) + var_qse_dn13)),)
    } else {
        (var_qb, var_qb_dn0, var_qb_dn2, var_qb_dn4, var_qb_dn5, var_qb_dn6, var_qb_dn7, var_qb_dn8, var_qb_dn9, var_qb_dn10, var_qb_dn13,)
    }
};
        var_qb = assign106150_e158443;
        var_qb_dn0 = assign106150_e158443_d_n0;
        var_qb_dn2 = assign106150_e158443_d_n2;
        var_qb_dn4 = assign106150_e158443_d_n4;
        var_qb_dn5 = assign106150_e158443_d_n5;
        var_qb_dn6 = assign106150_e158443_d_n6;
        var_qb_dn7 = assign106150_e158443_d_n7;
        var_qb_dn8 = assign106150_e158443_d_n8;
        var_qb_dn9 = assign106150_e158443_d_n9;
        var_qb_dn10 = assign106150_e158443_d_n10;
        var_qb_dn13 = assign106150_e158443_d_n13;

        let (assign106160_e158448, assign106160_e158448_d_n0, assign106160_e158448_d_n2, assign106160_e158448_d_n4, assign106160_e158448_d_n5, assign106160_e158448_d_n6, assign106160_e158448_d_n7, assign106160_e158448_d_n8, assign106160_e158448_d_n9, assign106160_e158448_d_n10, assign106160_e158448_d_n13,) = {
    if (var_guard2399 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isub, var_isub_dn0, var_isub_dn2, var_isub_dn4, var_isub_dn5, var_isub_dn6, var_isub_dn7, var_isub_dn8, var_isub_dn9, var_isub_dn10, var_isub_dn13,)
    }
};
        var_isub = assign106160_e158448;
        var_isub_dn0 = assign106160_e158448_d_n0;
        var_isub_dn2 = assign106160_e158448_d_n2;
        var_isub_dn4 = assign106160_e158448_d_n4;
        var_isub_dn5 = assign106160_e158448_d_n5;
        var_isub_dn6 = assign106160_e158448_d_n6;
        var_isub_dn7 = assign106160_e158448_d_n7;
        var_isub_dn8 = assign106160_e158448_d_n8;
        var_isub_dn9 = assign106160_e158448_d_n9;
        var_isub_dn10 = assign106160_e158448_d_n10;
        var_isub_dn13 = assign106160_e158448_d_n13;

        let (assign106170_e158453, assign106170_e158453_d_n0, assign106170_e158453_d_n2, assign106170_e158453_d_n4, assign106170_e158453_d_n5, assign106170_e158453_d_n6, assign106170_e158453_d_n7, assign106170_e158453_d_n8, assign106170_e158453_d_n9, assign106170_e158453_d_n10, assign106170_e158453_d_n13,) = {
    if (var_guard2399 == 0.0) {
        (var_isube, var_isube_dn0, var_isube_dn2, var_isube_dn4, var_isube_dn5, var_isube_dn6, var_isube_dn7, var_isube_dn8, var_isube_dn9, var_isube_dn10, var_isube_dn13,)
    } else {
        (var_isubs, var_isubs_dn0, var_isubs_dn2, var_isubs_dn4, var_isubs_dn5, var_isubs_dn6, var_isubs_dn7, var_isubs_dn8, var_isubs_dn9, var_isubs_dn10, var_isubs_dn13,)
    }
};
        var_isubs = assign106170_e158453;
        var_isubs_dn0 = assign106170_e158453_d_n0;
        var_isubs_dn2 = assign106170_e158453_d_n2;
        var_isubs_dn4 = assign106170_e158453_d_n4;
        var_isubs_dn5 = assign106170_e158453_d_n5;
        var_isubs_dn6 = assign106170_e158453_d_n6;
        var_isubs_dn7 = assign106170_e158453_d_n7;
        var_isubs_dn8 = assign106170_e158453_d_n8;
        var_isubs_dn9 = assign106170_e158453_d_n9;
        var_isubs_dn10 = assign106170_e158453_d_n10;
        var_isubs_dn13 = assign106170_e158453_d_n13;

        let (assign106180_e158458, assign106180_e158458_d_n0, assign106180_e158458_d_n2, assign106180_e158458_d_n4, assign106180_e158458_d_n5, assign106180_e158458_d_n6, assign106180_e158458_d_n7, assign106180_e158458_d_n8, assign106180_e158458_d_n9, assign106180_e158458_d_n10, assign106180_e158458_d_n13,) = {
    if (var_guard2399 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isubld, var_isubld_dn0, var_isubld_dn2, var_isubld_dn4, var_isubld_dn5, var_isubld_dn6, var_isubld_dn7, var_isubld_dn8, var_isubld_dn9, var_isubld_dn10, var_isubld_dn13,)
    }
};
        var_isubld = assign106180_e158458;
        var_isubld_dn0 = assign106180_e158458_d_n0;
        var_isubld_dn2 = assign106180_e158458_d_n2;
        var_isubld_dn4 = assign106180_e158458_d_n4;
        var_isubld_dn5 = assign106180_e158458_d_n5;
        var_isubld_dn6 = assign106180_e158458_d_n6;
        var_isubld_dn7 = assign106180_e158458_d_n7;
        var_isubld_dn8 = assign106180_e158458_d_n8;
        var_isubld_dn9 = assign106180_e158458_d_n9;
        var_isubld_dn10 = assign106180_e158458_d_n10;
        var_isubld_dn13 = assign106180_e158458_d_n13;

        let (assign106190_e158463, assign106190_e158463_d_n0, assign106190_e158463_d_n2, assign106190_e158463_d_n4, assign106190_e158463_d_n5, assign106190_e158463_d_n6, assign106190_e158463_d_n7, assign106190_e158463_d_n8, assign106190_e158463_d_n9, assign106190_e158463_d_n10, assign106190_e158463_d_n13,) = {
    if (var_guard2399 == 0.0) {
        (var_isublde, var_isublde_dn0, var_isublde_dn2, var_isublde_dn4, var_isublde_dn5, var_isublde_dn6, var_isublde_dn7, var_isublde_dn8, var_isublde_dn9, var_isublde_dn10, var_isublde_dn13,)
    } else {
        (var_isublds, var_isublds_dn0, var_isublds_dn2, var_isublds_dn4, var_isublds_dn5, var_isublds_dn6, var_isublds_dn7, var_isublds_dn8, var_isublds_dn9, var_isublds_dn10, var_isublds_dn13,)
    }
};
        var_isublds = assign106190_e158463;
        var_isublds_dn0 = assign106190_e158463_d_n0;
        var_isublds_dn2 = assign106190_e158463_d_n2;
        var_isublds_dn4 = assign106190_e158463_d_n4;
        var_isublds_dn5 = assign106190_e158463_d_n5;
        var_isublds_dn6 = assign106190_e158463_d_n6;
        var_isublds_dn7 = assign106190_e158463_d_n7;
        var_isublds_dn8 = assign106190_e158463_d_n8;
        var_isublds_dn9 = assign106190_e158463_d_n9;
        var_isublds_dn10 = assign106190_e158463_d_n10;
        var_isublds_dn13 = assign106190_e158463_d_n13;

        let (assign106200_e158468, assign106200_e158468_d_n0, assign106200_e158468_d_n2, assign106200_e158468_d_n4, assign106200_e158468_d_n5, assign106200_e158468_d_n6, assign106200_e158468_d_n7, assign106200_e158468_d_n8, assign106200_e158468_d_n9, assign106200_e158468_d_n10, assign106200_e158468_d_n13,) = {
    if (var_guard2399 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_idsibpc, var_idsibpc_dn0, var_idsibpc_dn2, var_idsibpc_dn4, var_idsibpc_dn5, var_idsibpc_dn6, var_idsibpc_dn7, var_idsibpc_dn8, var_idsibpc_dn9, var_idsibpc_dn10, var_idsibpc_dn13,)
    }
};
        var_idsibpc = assign106200_e158468;
        var_idsibpc_dn0 = assign106200_e158468_d_n0;
        var_idsibpc_dn2 = assign106200_e158468_d_n2;
        var_idsibpc_dn4 = assign106200_e158468_d_n4;
        var_idsibpc_dn5 = assign106200_e158468_d_n5;
        var_idsibpc_dn6 = assign106200_e158468_d_n6;
        var_idsibpc_dn7 = assign106200_e158468_d_n7;
        var_idsibpc_dn8 = assign106200_e158468_d_n8;
        var_idsibpc_dn9 = assign106200_e158468_d_n9;
        var_idsibpc_dn10 = assign106200_e158468_d_n10;
        var_idsibpc_dn13 = assign106200_e158468_d_n13;

        let (assign106210_e158473, assign106210_e158473_d_n0, assign106210_e158473_d_n2, assign106210_e158473_d_n4, assign106210_e158473_d_n5, assign106210_e158473_d_n6, assign106210_e158473_d_n7, assign106210_e158473_d_n8, assign106210_e158473_d_n9, assign106210_e158473_d_n10, assign106210_e158473_d_n13,) = {
    if (var_guard2399 == 0.0) {
        (var_idsibpce, var_idsibpce_dn0, var_idsibpce_dn2, var_idsibpce_dn4, var_idsibpce_dn5, var_idsibpce_dn6, var_idsibpce_dn7, var_idsibpce_dn8, var_idsibpce_dn9, var_idsibpce_dn10, var_idsibpce_dn13,)
    } else {
        (var_idsibpcs, var_idsibpcs_dn0, var_idsibpcs_dn2, var_idsibpcs_dn4, var_idsibpcs_dn5, var_idsibpcs_dn6, var_idsibpcs_dn7, var_idsibpcs_dn8, var_idsibpcs_dn9, var_idsibpcs_dn10, var_idsibpcs_dn13,)
    }
};
        var_idsibpcs = assign106210_e158473;
        var_idsibpcs_dn0 = assign106210_e158473_d_n0;
        var_idsibpcs_dn2 = assign106210_e158473_d_n2;
        var_idsibpcs_dn4 = assign106210_e158473_d_n4;
        var_idsibpcs_dn5 = assign106210_e158473_d_n5;
        var_idsibpcs_dn6 = assign106210_e158473_d_n6;
        var_idsibpcs_dn7 = assign106210_e158473_d_n7;
        var_idsibpcs_dn8 = assign106210_e158473_d_n8;
        var_idsibpcs_dn9 = assign106210_e158473_d_n9;
        var_idsibpcs_dn10 = assign106210_e158473_d_n10;
        var_idsibpcs_dn13 = assign106210_e158473_d_n13;

        let (assign106220_e158478, assign106220_e158478_d_n0, assign106220_e158478_d_n2, assign106220_e158478_d_n4, assign106220_e158478_d_n5, assign106220_e158478_d_n6, assign106220_e158478_d_n7, assign106220_e158478_d_n8, assign106220_e158478_d_n9, assign106220_e158478_d_n10, assign106220_e158478_d_n13,) = {
    if (var_guard2399 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibjt, var_ibjt_dn0, var_ibjt_dn2, var_ibjt_dn4, var_ibjt_dn5, var_ibjt_dn6, var_ibjt_dn7, var_ibjt_dn8, var_ibjt_dn9, var_ibjt_dn10, var_ibjt_dn13,)
    }
};
        var_ibjt = assign106220_e158478;
        var_ibjt_dn0 = assign106220_e158478_d_n0;
        var_ibjt_dn2 = assign106220_e158478_d_n2;
        var_ibjt_dn4 = assign106220_e158478_d_n4;
        var_ibjt_dn5 = assign106220_e158478_d_n5;
        var_ibjt_dn6 = assign106220_e158478_d_n6;
        var_ibjt_dn7 = assign106220_e158478_d_n7;
        var_ibjt_dn8 = assign106220_e158478_d_n8;
        var_ibjt_dn9 = assign106220_e158478_d_n9;
        var_ibjt_dn10 = assign106220_e158478_d_n10;
        var_ibjt_dn13 = assign106220_e158478_d_n13;

        let (assign106230_e158483, assign106230_e158483_d_n0, assign106230_e158483_d_n2, assign106230_e158483_d_n4, assign106230_e158483_d_n5, assign106230_e158483_d_n6, assign106230_e158483_d_n7, assign106230_e158483_d_n8, assign106230_e158483_d_n9, assign106230_e158483_d_n10, assign106230_e158483_d_n13,) = {
    if (var_guard2399 == 0.0) {
        (var_ibjte, var_ibjte_dn0, var_ibjte_dn2, var_ibjte_dn4, var_ibjte_dn5, var_ibjte_dn6, var_ibjte_dn7, var_ibjte_dn8, var_ibjte_dn9, var_ibjte_dn10, var_ibjte_dn13,)
    } else {
        (var_ibjts, var_ibjts_dn0, var_ibjts_dn2, var_ibjts_dn4, var_ibjts_dn5, var_ibjts_dn6, var_ibjts_dn7, var_ibjts_dn8, var_ibjts_dn9, var_ibjts_dn10, var_ibjts_dn13,)
    }
};
        var_ibjts = assign106230_e158483;
        var_ibjts_dn0 = assign106230_e158483_d_n0;
        var_ibjts_dn2 = assign106230_e158483_d_n2;
        var_ibjts_dn4 = assign106230_e158483_d_n4;
        var_ibjts_dn5 = assign106230_e158483_d_n5;
        var_ibjts_dn6 = assign106230_e158483_d_n6;
        var_ibjts_dn7 = assign106230_e158483_d_n7;
        var_ibjts_dn8 = assign106230_e158483_d_n8;
        var_ibjts_dn9 = assign106230_e158483_d_n9;
        var_ibjts_dn10 = assign106230_e158483_d_n10;
        var_ibjts_dn13 = assign106230_e158483_d_n13;

        let (assign106240_e158488, assign106240_e158488_d_n0, assign106240_e158488_d_n2, assign106240_e158488_d_n4, assign106240_e158488_d_n5, assign106240_e158488_d_n6, assign106240_e158488_d_n7, assign106240_e158488_d_n8, assign106240_e158488_d_n9, assign106240_e158488_d_n10, assign106240_e158488_d_n13,) = {
    if (var_guard2399 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibreak, var_ibreak_dn0, var_ibreak_dn2, var_ibreak_dn4, var_ibreak_dn5, var_ibreak_dn6, var_ibreak_dn7, var_ibreak_dn8, var_ibreak_dn9, var_ibreak_dn10, var_ibreak_dn13,)
    }
};
        var_ibreak = assign106240_e158488;
        var_ibreak_dn0 = assign106240_e158488_d_n0;
        var_ibreak_dn2 = assign106240_e158488_d_n2;
        var_ibreak_dn4 = assign106240_e158488_d_n4;
        var_ibreak_dn5 = assign106240_e158488_d_n5;
        var_ibreak_dn6 = assign106240_e158488_d_n6;
        var_ibreak_dn7 = assign106240_e158488_d_n7;
        var_ibreak_dn8 = assign106240_e158488_d_n8;
        var_ibreak_dn9 = assign106240_e158488_d_n9;
        var_ibreak_dn10 = assign106240_e158488_d_n10;
        var_ibreak_dn13 = assign106240_e158488_d_n13;

        let (assign106250_e158493, assign106250_e158493_d_n0, assign106250_e158493_d_n2, assign106250_e158493_d_n4, assign106250_e158493_d_n5, assign106250_e158493_d_n6, assign106250_e158493_d_n7, assign106250_e158493_d_n8, assign106250_e158493_d_n9, assign106250_e158493_d_n10, assign106250_e158493_d_n13,) = {
    if (var_guard2399 == 0.0) {
        (var_ibreake, var_ibreake_dn0, var_ibreake_dn2, var_ibreake_dn4, var_ibreake_dn5, var_ibreake_dn6, var_ibreake_dn7, var_ibreake_dn8, var_ibreake_dn9, var_ibreake_dn10, var_ibreake_dn13,)
    } else {
        (var_ibreaks, var_ibreaks_dn0, var_ibreaks_dn2, var_ibreaks_dn4, var_ibreaks_dn5, var_ibreaks_dn6, var_ibreaks_dn7, var_ibreaks_dn8, var_ibreaks_dn9, var_ibreaks_dn10, var_ibreaks_dn13,)
    }
};
        var_ibreaks = assign106250_e158493;
        var_ibreaks_dn0 = assign106250_e158493_d_n0;
        var_ibreaks_dn2 = assign106250_e158493_d_n2;
        var_ibreaks_dn4 = assign106250_e158493_d_n4;
        var_ibreaks_dn5 = assign106250_e158493_d_n5;
        var_ibreaks_dn6 = assign106250_e158493_d_n6;
        var_ibreaks_dn7 = assign106250_e158493_d_n7;
        var_ibreaks_dn8 = assign106250_e158493_d_n8;
        var_ibreaks_dn9 = assign106250_e158493_d_n9;
        var_ibreaks_dn10 = assign106250_e158493_d_n10;
        var_ibreaks_dn13 = assign106250_e158493_d_n13;

        let (assign106260_e158498, assign106260_e158498_d_n0, assign106260_e158498_d_n2, assign106260_e158498_d_n4, assign106260_e158498_d_n5, assign106260_e158498_d_n6, assign106260_e158498_d_n7, assign106260_e158498_d_n8, assign106260_e158498_d_n9, assign106260_e158498_d_n10, assign106260_e158498_d_n13,) = {
    if (var_guard2399 == 0.0) {
        (var_igisle, var_igisle_dn0, var_igisle_dn2, var_igisle_dn4, var_igisle_dn5, var_igisle_dn6, var_igisle_dn7, var_igisle_dn8, var_igisle_dn9, var_igisle_dn10, var_igisle_dn13,)
    } else {
        (var_igidl, var_igidl_dn0, var_igidl_dn2, var_igidl_dn4, var_igidl_dn5, var_igidl_dn6, var_igidl_dn7, var_igidl_dn8, var_igidl_dn9, var_igidl_dn10, var_igidl_dn13,)
    }
};
        var_igidl = assign106260_e158498;
        var_igidl_dn0 = assign106260_e158498_d_n0;
        var_igidl_dn2 = assign106260_e158498_d_n2;
        var_igidl_dn4 = assign106260_e158498_d_n4;
        var_igidl_dn5 = assign106260_e158498_d_n5;
        var_igidl_dn6 = assign106260_e158498_d_n6;
        var_igidl_dn7 = assign106260_e158498_d_n7;
        var_igidl_dn8 = assign106260_e158498_d_n8;
        var_igidl_dn9 = assign106260_e158498_d_n9;
        var_igidl_dn10 = assign106260_e158498_d_n10;
        var_igidl_dn13 = assign106260_e158498_d_n13;

        let (assign106270_e158503, assign106270_e158503_d_n0, assign106270_e158503_d_n2, assign106270_e158503_d_n4, assign106270_e158503_d_n5, assign106270_e158503_d_n6, assign106270_e158503_d_n7, assign106270_e158503_d_n8, assign106270_e158503_d_n9, assign106270_e158503_d_n10, assign106270_e158503_d_n13,) = {
    if (var_guard2399 == 0.0) {
        (var_igidle, var_igidle_dn0, var_igidle_dn2, var_igidle_dn4, var_igidle_dn5, var_igidle_dn6, var_igidle_dn7, var_igidle_dn8, var_igidle_dn9, var_igidle_dn10, var_igidle_dn13,)
    } else {
        (var_igisl, var_igisl_dn0, var_igisl_dn2, var_igisl_dn4, var_igisl_dn5, var_igisl_dn6, var_igisl_dn7, var_igisl_dn8, var_igisl_dn9, var_igisl_dn10, var_igisl_dn13,)
    }
};
        var_igisl = assign106270_e158503;
        var_igisl_dn0 = assign106270_e158503_d_n0;
        var_igisl_dn2 = assign106270_e158503_d_n2;
        var_igisl_dn4 = assign106270_e158503_d_n4;
        var_igisl_dn5 = assign106270_e158503_d_n5;
        var_igisl_dn6 = assign106270_e158503_d_n6;
        var_igisl_dn7 = assign106270_e158503_d_n7;
        var_igisl_dn8 = assign106270_e158503_d_n8;
        var_igisl_dn9 = assign106270_e158503_d_n9;
        var_igisl_dn10 = assign106270_e158503_d_n10;
        var_igisl_dn13 = assign106270_e158503_d_n13;

        let (assign106280_e158512, assign106280_e158512_d_n0, assign106280_e158512_d_n2, assign106280_e158512_d_n4, assign106280_e158512_d_n5, assign106280_e158512_d_n6, assign106280_e158512_d_n7, assign106280_e158512_d_n8, assign106280_e158512_d_n9, assign106280_e158512_d_n10, assign106280_e158512_d_n13,) = {
    if ((var_guard2399 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign106280_e158510: f64 = (1.0 - var_xd);
        (assign106280_e158510, (-var_xd_dn0), (-var_xd_dn2), (-var_xd_dn4), (-var_xd_dn5), (-var_xd_dn6), (-var_xd_dn7), (-var_xd_dn8), (-var_xd_dn9), (-var_xd_dn10), (-var_xd_dn13),)
    } else {
        (var_qdrat, var_qdrat_dn0, var_qdrat_dn2, var_qdrat_dn4, var_qdrat_dn5, var_qdrat_dn6, var_qdrat_dn7, var_qdrat_dn8, var_qdrat_dn9, var_qdrat_dn10, var_qdrat_dn13,)
    }
};
        var_qdrat = assign106280_e158512;
        var_qdrat_dn0 = assign106280_e158512_d_n0;
        var_qdrat_dn2 = assign106280_e158512_d_n2;
        var_qdrat_dn4 = assign106280_e158512_d_n4;
        var_qdrat_dn5 = assign106280_e158512_d_n5;
        var_qdrat_dn6 = assign106280_e158512_d_n6;
        var_qdrat_dn7 = assign106280_e158512_d_n7;
        var_qdrat_dn8 = assign106280_e158512_d_n8;
        var_qdrat_dn9 = assign106280_e158512_d_n9;
        var_qdrat_dn10 = assign106280_e158512_d_n10;
        var_qdrat_dn13 = assign106280_e158512_d_n13;

        let assign106290_e158515: f64 = (var_qg + var_qgov);
        var_qg = assign106290_e158515;
        var_qg_dn0 = (var_qg_dn0 + var_qgov_dn0);
        var_qg_dn2 = (var_qg_dn2 + var_qgov_dn2);
        var_qg_dn4 = (var_qg_dn4 + var_qgov_dn4);
        var_qg_dn5 = (var_qg_dn5 + var_qgov_dn5);
        var_qg_dn6 = (var_qg_dn6 + var_qgov_dn6);
        var_qg_dn7 = (var_qg_dn7 + var_qgov_dn7);
        var_qg_dn8 = (var_qg_dn8 + var_qgov_dn8);
        var_qg_dn9 = (var_qg_dn9 + var_qgov_dn9);
        var_qg_dn10 = (var_qg_dn10 + var_qgov_dn10);
        var_qg_dn13 = (var_qg_dn13 + var_qgov_dn13);

        let assign106300_e158518: f64 = (var_qd + var_qdov);
        var_qd = assign106300_e158518;
        var_qd_dn0 = (var_qd_dn0 + var_qdov_dn0);
        var_qd_dn2 = (var_qd_dn2 + var_qdov_dn2);
        var_qd_dn4 = (var_qd_dn4 + var_qdov_dn4);
        var_qd_dn5 = (var_qd_dn5 + var_qdov_dn5);
        var_qd_dn6 = (var_qd_dn6 + var_qdov_dn6);
        var_qd_dn7 = (var_qd_dn7 + var_qdov_dn7);
        var_qd_dn8 = (var_qd_dn8 + var_qdov_dn8);
        var_qd_dn9 = (var_qd_dn9 + var_qdov_dn9);
        var_qd_dn10 = (var_qd_dn10 + var_qdov_dn10);
        var_qd_dn13 = (var_qd_dn13 + var_qdov_dn13);

        let assign106310_e158521: f64 = (var_qs + var_qsov);
        var_qs = assign106310_e158521;
        var_qs_dn0 = (var_qs_dn0 + var_qsov_dn0);
        var_qs_dn2 = (var_qs_dn2 + var_qsov_dn2);
        var_qs_dn4 = (var_qs_dn4 + var_qsov_dn4);
        var_qs_dn5 = (var_qs_dn5 + var_qsov_dn5);
        var_qs_dn6 = (var_qs_dn6 + var_qsov_dn6);
        var_qs_dn7 = (var_qs_dn7 + var_qsov_dn7);
        var_qs_dn8 = (var_qs_dn8 + var_qsov_dn8);
        var_qs_dn9 = (var_qs_dn9 + var_qsov_dn9);
        var_qs_dn10 = (var_qs_dn10 + var_qsov_dn10);
        var_qs_dn13 = (var_qs_dn13 + var_qsov_dn13);

        let assign106320_e158524: f64 = (var_qg + var_qd);
        let assign106320_e158526: f64 = (assign106320_e158524 + var_qs);
        let assign106320_e158527: f64 = (-assign106320_e158526);
        var_qb = assign106320_e158527;
        var_qb_dn0 = (-((var_qg_dn0 + var_qd_dn0) + var_qs_dn0));
        var_qb_dn2 = (-((var_qg_dn2 + var_qd_dn2) + var_qs_dn2));
        var_qb_dn4 = (-((var_qg_dn4 + var_qd_dn4) + var_qs_dn4));
        var_qb_dn5 = (-((var_qg_dn5 + var_qd_dn5) + var_qs_dn5));
        var_qb_dn6 = (-((var_qg_dn6 + var_qd_dn6) + var_qs_dn6));
        var_qb_dn7 = (-((var_qg_dn7 + var_qd_dn7) + var_qs_dn7));
        var_qb_dn8 = (-((var_qg_dn8 + var_qd_dn8) + var_qs_dn8));
        var_qb_dn9 = (-((var_qg_dn9 + var_qd_dn9) + var_qs_dn9));
        var_qb_dn10 = (-((var_qg_dn10 + var_qd_dn10) + var_qs_dn10));
        var_qb_dn13 = (-((var_qg_dn13 + var_qd_dn13) + var_qs_dn13));

        var_qfd = var_qdp;
        var_qfd_dn0 = var_qdp_dn0;
        var_qfd_dn2 = var_qdp_dn2;
        var_qfd_dn6 = var_qdp_dn6;

        var_qfs = var_qsp;
        var_qfs_dn2 = var_qsp_dn2;
        var_qfs_dn6 = var_qsp_dn6;

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
        var_qdext_dn13 = var_qdexte_dn13;

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
        var_qgext_dn13 = var_qgexte_dn13;

        let assign106370_e158534: f64 = (var_qgexte + var_qdexte);
        let assign106370_e158536: f64 = (assign106370_e158534 + var_qsexte);
        let assign106370_e158537: f64 = (-assign106370_e158536);
        var_qbext = assign106370_e158537;
        var_qbext_dn0 = (-((var_qgexte_dn0 + var_qdexte_dn0) + var_qsexte_dn0));
        var_qbext_dn2 = (-((var_qgexte_dn2 + var_qdexte_dn2) + var_qsexte_dn2));
        var_qbext_dn4 = (-((var_qgexte_dn4 + var_qdexte_dn4) + var_qsexte_dn4));
        var_qbext_dn5 = (-((var_qgexte_dn5 + var_qdexte_dn5) + var_qsexte_dn5));
        var_qbext_dn6 = (-((var_qgexte_dn6 + var_qdexte_dn6) + var_qsexte_dn6));
        var_qbext_dn7 = (-((var_qgexte_dn7 + var_qdexte_dn7) + var_qsexte_dn7));
        var_qbext_dn8 = (-((var_qgexte_dn8 + var_qdexte_dn8) + var_qsexte_dn8));
        var_qbext_dn9 = (-((var_qgexte_dn9 + var_qdexte_dn9) + var_qsexte_dn9));
        var_qbext_dn10 = (-((var_qgexte_dn10 + var_qdexte_dn10) + var_qsexte_dn10));
        var_qbext_dn13 = (-((var_qgexte_dn13 + var_qdexte_dn13) + var_qsexte_dn13));

        let assign106380_e158540: f64 = if p.p53 > 0.0 { 1.0 } else { 0.0 };
        var_guard2400 = assign106380_e158540;

        let assign106390_e158543: f64 = if var_rth > 0.0001 { 1.0 } else { 0.0 };
        var_guard2401 = assign106390_e158543;

        let (assign106400_e158551, assign106400_e158551_d_n0, assign106400_e158551_d_n2, assign106400_e158551_d_n4, assign106400_e158551_d_n5, assign106400_e158551_d_n6, assign106400_e158551_d_n7, assign106400_e158551_d_n8, assign106400_e158551_d_n9, assign106400_e158551_d_n10, assign106400_e158551_d_n13,) = {
    if ((var_guard2400 != 0.0) && (var_guard2401 != 0.0)) {
        let assign106400_e158549: f64 = (1.0 / var_rth);
        (assign106400_e158549, (-(var_rth_dn0 / (var_rth * var_rth))), (-(var_rth_dn2 / (var_rth * var_rth))), (-(var_rth_dn4 / (var_rth * var_rth))), (-(var_rth_dn5 / (var_rth * var_rth))), (-(var_rth_dn6 / (var_rth * var_rth))), (-(var_rth_dn7 / (var_rth * var_rth))), (-(var_rth_dn8 / (var_rth * var_rth))), (-(var_rth_dn9 / (var_rth * var_rth))), (-(var_rth_dn10 / (var_rth * var_rth))), (-(var_rth_dn13 / (var_rth * var_rth))),)
    } else {
        (var_gth, var_gth_dn0, var_gth_dn2, var_gth_dn4, var_gth_dn5, var_gth_dn6, var_gth_dn7, var_gth_dn8, var_gth_dn9, var_gth_dn10, var_gth_dn13,)
    }
};
        var_gth = assign106400_e158551;
        var_gth_dn0 = assign106400_e158551_d_n0;
        var_gth_dn2 = assign106400_e158551_d_n2;
        var_gth_dn4 = assign106400_e158551_d_n4;
        var_gth_dn5 = assign106400_e158551_d_n5;
        var_gth_dn6 = assign106400_e158551_d_n6;
        var_gth_dn7 = assign106400_e158551_d_n7;
        var_gth_dn8 = assign106400_e158551_d_n8;
        var_gth_dn9 = assign106400_e158551_d_n9;
        var_gth_dn10 = assign106400_e158551_d_n10;
        var_gth_dn13 = assign106400_e158551_d_n13;

        let (assign106410_e158560, assign106410_e158560_d_n0, assign106410_e158560_d_n2, assign106410_e158560_d_n4, assign106410_e158560_d_n5, assign106410_e158560_d_n6, assign106410_e158560_d_n7, assign106410_e158560_d_n8, assign106410_e158560_d_n9, assign106410_e158560_d_n10, assign106410_e158560_d_n13,) = {
    if ((var_guard2400 != 0.0) && (var_guard2401 == 0.0)) {
        let assign106410_e158558: f64 = (1.0 / 0.0001);
        (assign106410_e158558, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_gth, var_gth_dn0, var_gth_dn2, var_gth_dn4, var_gth_dn5, var_gth_dn6, var_gth_dn7, var_gth_dn8, var_gth_dn9, var_gth_dn10, var_gth_dn13,)
    }
};
        var_gth = assign106410_e158560;
        var_gth_dn0 = assign106410_e158560_d_n0;
        var_gth_dn2 = assign106410_e158560_d_n2;
        var_gth_dn4 = assign106410_e158560_d_n4;
        var_gth_dn5 = assign106410_e158560_d_n5;
        var_gth_dn6 = assign106410_e158560_d_n6;
        var_gth_dn7 = assign106410_e158560_d_n7;
        var_gth_dn8 = assign106410_e158560_d_n8;
        var_gth_dn9 = assign106410_e158560_d_n9;
        var_gth_dn10 = assign106410_e158560_d_n10;
        var_gth_dn13 = assign106410_e158560_d_n13;

        let assign106420_e158564: f64 = (var_vdsei - var_vdsi);
        let assign106420_e158565: f64 = (var_vdsi * assign106420_e158564);
        let assign106420_e158567: f64 = if assign106420_e158565 >= 0.0 { 1.0 } else { 0.0 };
        var_guard2402 = assign106420_e158567;

        let assign106430_e158570: f64 = if var_uc_powrat == 1.0 { 1.0 } else { 0.0 };
        var_guard2403 = assign106430_e158570;

        let (assign106440_e158578, assign106440_e158578_d_n0, assign106440_e158578_d_n2, assign106440_e158578_d_n4, assign106440_e158578_d_n5, assign106440_e158578_d_n6, assign106440_e158578_d_n7, assign106440_e158578_d_n8, assign106440_e158578_d_n9, assign106440_e158578_d_n10, assign106440_e158578_d_n13,) = {
    if (((var_guard2400 != 0.0) && (var_guard2402 != 0.0)) && (var_guard2403 != 0.0)) {
        (var_vdsei, var_vdsei_dn0, var_vdsei_dn2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_veffpower, var_veffpower_dn0, var_veffpower_dn2, var_veffpower_dn4, var_veffpower_dn5, var_veffpower_dn6, var_veffpower_dn7, var_veffpower_dn8, var_veffpower_dn9, var_veffpower_dn10, var_veffpower_dn13,)
    }
};
        var_veffpower = assign106440_e158578;
        var_veffpower_dn0 = assign106440_e158578_d_n0;
        var_veffpower_dn2 = assign106440_e158578_d_n2;
        var_veffpower_dn4 = assign106440_e158578_d_n4;
        var_veffpower_dn5 = assign106440_e158578_d_n5;
        var_veffpower_dn6 = assign106440_e158578_d_n6;
        var_veffpower_dn7 = assign106440_e158578_d_n7;
        var_veffpower_dn8 = assign106440_e158578_d_n8;
        var_veffpower_dn9 = assign106440_e158578_d_n9;
        var_veffpower_dn10 = assign106440_e158578_d_n10;
        var_veffpower_dn13 = assign106440_e158578_d_n13;

        let (assign106450_e158593, assign106450_e158593_d_n0, assign106450_e158593_d_n2, assign106450_e158593_d_n4, assign106450_e158593_d_n5, assign106450_e158593_d_n6, assign106450_e158593_d_n7, assign106450_e158593_d_n8, assign106450_e158593_d_n9, assign106450_e158593_d_n10, assign106450_e158593_d_n13,) = {
    if (((var_guard2400 != 0.0) && (var_guard2402 != 0.0)) && (var_guard2403 == 0.0)) {
        let assign106450_e158589: f64 = (var_vdsei - var_vdsi);
        let assign106450_e158590: f64 = (var_powratio * assign106450_e158589);
        let assign106450_e158591: f64 = (var_vdsi + assign106450_e158590);
        (assign106450_e158591, ((var_powratio_dn0 * assign106450_e158589) + (var_powratio * var_vdsei_dn0)), ((var_powratio_dn2 * assign106450_e158589) + (var_powratio * var_vdsei_dn2)), (var_powratio_dn4 * assign106450_e158589), (var_vdsi_dn5 + ((var_powratio_dn5 * assign106450_e158589) + (var_powratio * (-var_vdsi_dn5)))), (var_powratio_dn6 * assign106450_e158589), (var_vdsi_dn7 + ((var_powratio_dn7 * assign106450_e158589) + (var_powratio * (-var_vdsi_dn7)))), (var_powratio_dn8 * assign106450_e158589), (var_powratio_dn9 * assign106450_e158589), (var_powratio_dn10 * assign106450_e158589), (var_powratio_dn13 * assign106450_e158589),)
    } else {
        (var_veffpower, var_veffpower_dn0, var_veffpower_dn2, var_veffpower_dn4, var_veffpower_dn5, var_veffpower_dn6, var_veffpower_dn7, var_veffpower_dn8, var_veffpower_dn9, var_veffpower_dn10, var_veffpower_dn13,)
    }
};
        var_veffpower = assign106450_e158593;
        var_veffpower_dn0 = assign106450_e158593_d_n0;
        var_veffpower_dn2 = assign106450_e158593_d_n2;
        var_veffpower_dn4 = assign106450_e158593_d_n4;
        var_veffpower_dn5 = assign106450_e158593_d_n5;
        var_veffpower_dn6 = assign106450_e158593_d_n6;
        var_veffpower_dn7 = assign106450_e158593_d_n7;
        var_veffpower_dn8 = assign106450_e158593_d_n8;
        var_veffpower_dn9 = assign106450_e158593_d_n9;
        var_veffpower_dn10 = assign106450_e158593_d_n10;
        var_veffpower_dn13 = assign106450_e158593_d_n13;

        let (assign106460_e158600, assign106460_e158600_d_n0, assign106460_e158600_d_n2, assign106460_e158600_d_n4, assign106460_e158600_d_n5, assign106460_e158600_d_n6, assign106460_e158600_d_n7, assign106460_e158600_d_n8, assign106460_e158600_d_n9, assign106460_e158600_d_n10, assign106460_e158600_d_n13,) = {
    if ((var_guard2400 != 0.0) && (var_guard2402 == 0.0)) {
        (var_vdsi, 0.0, 0.0, 0.0, var_vdsi_dn5, 0.0, var_vdsi_dn7, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_veffpower, var_veffpower_dn0, var_veffpower_dn2, var_veffpower_dn4, var_veffpower_dn5, var_veffpower_dn6, var_veffpower_dn7, var_veffpower_dn8, var_veffpower_dn9, var_veffpower_dn10, var_veffpower_dn13,)
    }
};
        var_veffpower = assign106460_e158600;
        var_veffpower_dn0 = assign106460_e158600_d_n0;
        var_veffpower_dn2 = assign106460_e158600_d_n2;
        var_veffpower_dn4 = assign106460_e158600_d_n4;
        var_veffpower_dn5 = assign106460_e158600_d_n5;
        var_veffpower_dn6 = assign106460_e158600_d_n6;
        var_veffpower_dn7 = assign106460_e158600_d_n7;
        var_veffpower_dn8 = assign106460_e158600_d_n8;
        var_veffpower_dn9 = assign106460_e158600_d_n9;
        var_veffpower_dn10 = assign106460_e158600_d_n10;
        var_veffpower_dn13 = assign106460_e158600_d_n13;

        let (assign106470_e158606, assign106470_e158606_d_n0, assign106470_e158606_d_n2, assign106470_e158606_d_n4, assign106470_e158606_d_n5, assign106470_e158606_d_n6, assign106470_e158606_d_n7, assign106470_e158606_d_n8, assign106470_e158606_d_n9, assign106470_e158606_d_n10, assign106470_e158606_d_n13,) = {
    if (var_guard2400 != 0.0) {
        let assign106470_e158604: f64 = (var_ids * var_veffpower);
        (assign106470_e158604, ((var_ids_dn0 * var_veffpower) + (var_ids * var_veffpower_dn0)), ((var_ids_dn2 * var_veffpower) + (var_ids * var_veffpower_dn2)), ((var_ids_dn4 * var_veffpower) + (var_ids * var_veffpower_dn4)), ((var_ids_dn5 * var_veffpower) + (var_ids * var_veffpower_dn5)), ((var_ids_dn6 * var_veffpower) + (var_ids * var_veffpower_dn6)), ((var_ids_dn7 * var_veffpower) + (var_ids * var_veffpower_dn7)), ((var_ids_dn8 * var_veffpower) + (var_ids * var_veffpower_dn8)), ((var_ids_dn9 * var_veffpower) + (var_ids * var_veffpower_dn9)), ((var_ids_dn10 * var_veffpower) + (var_ids * var_veffpower_dn10)), ((var_ids_dn13 * var_veffpower) + (var_ids * var_veffpower_dn13)),)
    } else {
        (var_p, var_p_dn0, var_p_dn2, var_p_dn4, var_p_dn5, var_p_dn6, var_p_dn7, var_p_dn8, var_p_dn9, var_p_dn10, var_p_dn13,)
    }
};
        var_p = assign106470_e158606;
        var_p_dn0 = assign106470_e158606_d_n0;
        var_p_dn2 = assign106470_e158606_d_n2;
        var_p_dn4 = assign106470_e158606_d_n4;
        var_p_dn5 = assign106470_e158606_d_n5;
        var_p_dn6 = assign106470_e158606_d_n6;
        var_p_dn7 = assign106470_e158606_d_n7;
        var_p_dn8 = assign106470_e158606_d_n8;
        var_p_dn9 = assign106470_e158606_d_n9;
        var_p_dn10 = assign106470_e158606_d_n10;
        var_p_dn13 = assign106470_e158606_d_n13;

        let assign106480_e158609: f64 = if p.p53 == 1.0 { 1.0 } else { 0.0 };
        var_guard2404 = assign106480_e158609;

        *var_gth_slot = var_gth;
        *var_gth_dn0_slot = var_gth_dn0;
        *var_gth_dn10_slot = var_gth_dn10;
        *var_gth_dn13_slot = var_gth_dn13;
        *var_gth_dn2_slot = var_gth_dn2;
        *var_gth_dn4_slot = var_gth_dn4;
        *var_gth_dn5_slot = var_gth_dn5;
        *var_gth_dn6_slot = var_gth_dn6;
        *var_gth_dn7_slot = var_gth_dn7;
        *var_gth_dn8_slot = var_gth_dn8;
        *var_gth_dn9_slot = var_gth_dn9;
        *var_guard2400_slot = var_guard2400;
        *var_guard2401_slot = var_guard2401;
        *var_guard2402_slot = var_guard2402;
        *var_guard2403_slot = var_guard2403;
        *var_guard2404_slot = var_guard2404;
        *var_ibjt_slot = var_ibjt;
        *var_ibjt_dn0_slot = var_ibjt_dn0;
        *var_ibjt_dn10_slot = var_ibjt_dn10;
        *var_ibjt_dn13_slot = var_ibjt_dn13;
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
        *var_ibjts_dn13_slot = var_ibjts_dn13;
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
        *var_ibreak_dn13_slot = var_ibreak_dn13;
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
        *var_ibreaks_dn13_slot = var_ibreaks_dn13;
        *var_ibreaks_dn2_slot = var_ibreaks_dn2;
        *var_ibreaks_dn4_slot = var_ibreaks_dn4;
        *var_ibreaks_dn5_slot = var_ibreaks_dn5;
        *var_ibreaks_dn6_slot = var_ibreaks_dn6;
        *var_ibreaks_dn7_slot = var_ibreaks_dn7;
        *var_ibreaks_dn8_slot = var_ibreaks_dn8;
        *var_ibreaks_dn9_slot = var_ibreaks_dn9;
        *var_idsibpc_slot = var_idsibpc;
        *var_idsibpc_dn0_slot = var_idsibpc_dn0;
        *var_idsibpc_dn10_slot = var_idsibpc_dn10;
        *var_idsibpc_dn13_slot = var_idsibpc_dn13;
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
        *var_idsibpcs_dn13_slot = var_idsibpcs_dn13;
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
        *var_igidl_dn13_slot = var_igidl_dn13;
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
        *var_igisl_dn13_slot = var_igisl_dn13;
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
        *var_isub_dn13_slot = var_isub_dn13;
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
        *var_isubld_dn13_slot = var_isubld_dn13;
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
        *var_isublds_dn13_slot = var_isublds_dn13;
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
        *var_isubs_dn13_slot = var_isubs_dn13;
        *var_isubs_dn2_slot = var_isubs_dn2;
        *var_isubs_dn4_slot = var_isubs_dn4;
        *var_isubs_dn5_slot = var_isubs_dn5;
        *var_isubs_dn6_slot = var_isubs_dn6;
        *var_isubs_dn7_slot = var_isubs_dn7;
        *var_isubs_dn8_slot = var_isubs_dn8;
        *var_isubs_dn9_slot = var_isubs_dn9;
        *var_p_slot = var_p;
        *var_p_dn0_slot = var_p_dn0;
        *var_p_dn10_slot = var_p_dn10;
        *var_p_dn13_slot = var_p_dn13;
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
        *var_qb_dn13_slot = var_qb_dn13;
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
        *var_qbext_dn13_slot = var_qbext_dn13;
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
        *var_qd_dn13_slot = var_qd_dn13;
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
        *var_qdext_dn13_slot = var_qdext_dn13;
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
        *var_qdrat_dn13_slot = var_qdrat_dn13;
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
        *var_qfd_dn6_slot = var_qfd_dn6;
        *var_qfs_slot = var_qfs;
        *var_qfs_dn2_slot = var_qfs_dn2;
        *var_qfs_dn6_slot = var_qfs_dn6;
        *var_qg_slot = var_qg;
        *var_qg_dn0_slot = var_qg_dn0;
        *var_qg_dn10_slot = var_qg_dn10;
        *var_qg_dn13_slot = var_qg_dn13;
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
        *var_qgext_dn13_slot = var_qgext_dn13;
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
        *var_qs_dn13_slot = var_qs_dn13;
        *var_qs_dn2_slot = var_qs_dn2;
        *var_qs_dn4_slot = var_qs_dn4;
        *var_qs_dn5_slot = var_qs_dn5;
        *var_qs_dn6_slot = var_qs_dn6;
        *var_qs_dn7_slot = var_qs_dn7;
        *var_qs_dn8_slot = var_qs_dn8;
        *var_qs_dn9_slot = var_qs_dn9;
        *var_veffpower_slot = var_veffpower;
        *var_veffpower_dn0_slot = var_veffpower_dn0;
        *var_veffpower_dn10_slot = var_veffpower_dn10;
        *var_veffpower_dn13_slot = var_veffpower_dn13;
        *var_veffpower_dn2_slot = var_veffpower_dn2;
        *var_veffpower_dn4_slot = var_veffpower_dn4;
        *var_veffpower_dn5_slot = var_veffpower_dn5;
        *var_veffpower_dn6_slot = var_veffpower_dn6;
        *var_veffpower_dn7_slot = var_veffpower_dn7;
        *var_veffpower_dn8_slot = var_veffpower_dn8;
        *var_veffpower_dn9_slot = var_veffpower_dn9;
    }

    pub(super) fn stamp_transient_block_376(
        p: &Parameters,
        var_flg_nqs: f64,
        var_guard2400: f64,
        var_guard2404: f64,
        var_ids: f64,
        var_ids_dn0: f64,
        var_ids_dn10: f64,
        var_ids_dn13: f64,
        var_ids_dn2: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_ids_dn9: f64,
        var_mode: f64,
        var_qb_nqs: f64,
        var_qb_nqs_dn12: f64,
        var_qbulk: f64,
        var_qbulk_dn0: f64,
        var_qbulk_dn10: f64,
        var_qbulk_dn13: f64,
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
        var_qdrat_dn13: f64,
        var_qdrat_dn2: f64,
        var_qdrat_dn4: f64,
        var_qdrat_dn5: f64,
        var_qdrat_dn6: f64,
        var_qdrat_dn7: f64,
        var_qdrat_dn8: f64,
        var_qdrat_dn9: f64,
        var_qg_dn5: f64,
        var_qg_dn7: f64,
        var_qi: f64,
        var_qi_dn0: f64,
        var_qi_dn10: f64,
        var_qi_dn13: f64,
        var_qi_dn2: f64,
        var_qi_dn4: f64,
        var_qi_dn5: f64,
        var_qi_dn6: f64,
        var_qi_dn7: f64,
        var_qi_dn8: f64,
        var_qi_dn9: f64,
        var_qi_nqs: f64,
        var_qi_nqs_dn11: f64,
        var_cgdbd_slot: &mut f64,
        var_cgdbd_dn0_slot: &mut f64,
        var_cgdbd_dn10_slot: &mut f64,
        var_cgdbd_dn13_slot: &mut f64,
        var_cgdbd_dn2_slot: &mut f64,
        var_cgdbd_dn4_slot: &mut f64,
        var_cgdbd_dn5_slot: &mut f64,
        var_cgdbd_dn6_slot: &mut f64,
        var_cgdbd_dn7_slot: &mut f64,
        var_cgdbd_dn8_slot: &mut f64,
        var_cgdbd_dn9_slot: &mut f64,
        var_cgsbd_slot: &mut f64,
        var_cgsbd_dn0_slot: &mut f64,
        var_cgsbd_dn10_slot: &mut f64,
        var_cgsbd_dn13_slot: &mut f64,
        var_cgsbd_dn2_slot: &mut f64,
        var_cgsbd_dn4_slot: &mut f64,
        var_cgsbd_dn5_slot: &mut f64,
        var_cgsbd_dn6_slot: &mut f64,
        var_cgsbd_dn7_slot: &mut f64,
        var_cgsbd_dn8_slot: &mut f64,
        var_cgsbd_dn9_slot: &mut f64,
        var_gth_slot: &mut f64,
        var_gth_dn0_slot: &mut f64,
        var_gth_dn10_slot: &mut f64,
        var_gth_dn13_slot: &mut f64,
        var_gth_dn2_slot: &mut f64,
        var_gth_dn4_slot: &mut f64,
        var_gth_dn5_slot: &mut f64,
        var_gth_dn6_slot: &mut f64,
        var_gth_dn7_slot: &mut f64,
        var_gth_dn8_slot: &mut f64,
        var_gth_dn9_slot: &mut f64,
        var_guard2405_slot: &mut f64,
        var_guard2406_slot: &mut f64,
        var_idse_slot: &mut f64,
        var_idse_dn0_slot: &mut f64,
        var_idse_dn10_slot: &mut f64,
        var_idse_dn13_slot: &mut f64,
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
        var_iqb_nqs_dn12_slot: &mut f64,
        var_iqb_nqs_dn13_slot: &mut f64,
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
        var_iqi_nqs_dn13_slot: &mut f64,
        var_iqi_nqs_dn2_slot: &mut f64,
        var_iqi_nqs_dn4_slot: &mut f64,
        var_iqi_nqs_dn5_slot: &mut f64,
        var_iqi_nqs_dn6_slot: &mut f64,
        var_iqi_nqs_dn7_slot: &mut f64,
        var_iqi_nqs_dn8_slot: &mut f64,
        var_iqi_nqs_dn9_slot: &mut f64,
        var_p_slot: &mut f64,
        var_p_dn0_slot: &mut f64,
        var_p_dn10_slot: &mut f64,
        var_p_dn13_slot: &mut f64,
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
        var_qd_nqs_dn13_slot: &mut f64,
        var_qd_nqs_dn2_slot: &mut f64,
        var_qd_nqs_dn4_slot: &mut f64,
        var_qd_nqs_dn5_slot: &mut f64,
        var_qd_nqs_dn6_slot: &mut f64,
        var_qd_nqs_dn7_slot: &mut f64,
        var_qd_nqs_dn8_slot: &mut f64,
        var_qd_nqs_dn9_slot: &mut f64,
        var_qg_nqs_slot: &mut f64,
        var_qg_nqs_dn11_slot: &mut f64,
        var_qg_nqs_dn12_slot: &mut f64,
        var_qs_nqs_slot: &mut f64,
        var_qs_nqs_dn0_slot: &mut f64,
        var_qs_nqs_dn10_slot: &mut f64,
        var_qs_nqs_dn11_slot: &mut f64,
        var_qs_nqs_dn13_slot: &mut f64,
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
        var_t0_dn13_slot: &mut f64,
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
        var_t1_dn13_slot: &mut f64,
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
        var_t2_dn13_slot: &mut f64,
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
        var_tau_dn13_slot: &mut f64,
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
        var_taub_dn13_slot: &mut f64,
        var_taub_dn2_slot: &mut f64,
        var_taub_dn4_slot: &mut f64,
        var_taub_dn5_slot: &mut f64,
        var_taub_dn6_slot: &mut f64,
        var_taub_dn7_slot: &mut f64,
        var_taub_dn8_slot: &mut f64,
        var_taub_dn9_slot: &mut f64,
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
    ) {
        let mut var_cgdbd: f64 = *var_cgdbd_slot;
        let mut var_cgdbd_dn0: f64 = *var_cgdbd_dn0_slot;
        let mut var_cgdbd_dn10: f64 = *var_cgdbd_dn10_slot;
        let mut var_cgdbd_dn13: f64 = *var_cgdbd_dn13_slot;
        let mut var_cgdbd_dn2: f64 = *var_cgdbd_dn2_slot;
        let mut var_cgdbd_dn4: f64 = *var_cgdbd_dn4_slot;
        let mut var_cgdbd_dn5: f64 = *var_cgdbd_dn5_slot;
        let mut var_cgdbd_dn6: f64 = *var_cgdbd_dn6_slot;
        let mut var_cgdbd_dn7: f64 = *var_cgdbd_dn7_slot;
        let mut var_cgdbd_dn8: f64 = *var_cgdbd_dn8_slot;
        let mut var_cgdbd_dn9: f64 = *var_cgdbd_dn9_slot;
        let mut var_cgsbd: f64 = *var_cgsbd_slot;
        let mut var_cgsbd_dn0: f64 = *var_cgsbd_dn0_slot;
        let mut var_cgsbd_dn10: f64 = *var_cgsbd_dn10_slot;
        let mut var_cgsbd_dn13: f64 = *var_cgsbd_dn13_slot;
        let mut var_cgsbd_dn2: f64 = *var_cgsbd_dn2_slot;
        let mut var_cgsbd_dn4: f64 = *var_cgsbd_dn4_slot;
        let mut var_cgsbd_dn5: f64 = *var_cgsbd_dn5_slot;
        let mut var_cgsbd_dn6: f64 = *var_cgsbd_dn6_slot;
        let mut var_cgsbd_dn7: f64 = *var_cgsbd_dn7_slot;
        let mut var_cgsbd_dn8: f64 = *var_cgsbd_dn8_slot;
        let mut var_cgsbd_dn9: f64 = *var_cgsbd_dn9_slot;
        let mut var_gth: f64 = *var_gth_slot;
        let mut var_gth_dn0: f64 = *var_gth_dn0_slot;
        let mut var_gth_dn10: f64 = *var_gth_dn10_slot;
        let mut var_gth_dn13: f64 = *var_gth_dn13_slot;
        let mut var_gth_dn2: f64 = *var_gth_dn2_slot;
        let mut var_gth_dn4: f64 = *var_gth_dn4_slot;
        let mut var_gth_dn5: f64 = *var_gth_dn5_slot;
        let mut var_gth_dn6: f64 = *var_gth_dn6_slot;
        let mut var_gth_dn7: f64 = *var_gth_dn7_slot;
        let mut var_gth_dn8: f64 = *var_gth_dn8_slot;
        let mut var_gth_dn9: f64 = *var_gth_dn9_slot;
        let mut var_guard2405: f64 = *var_guard2405_slot;
        let mut var_guard2406: f64 = *var_guard2406_slot;
        let mut var_idse: f64 = *var_idse_slot;
        let mut var_idse_dn0: f64 = *var_idse_dn0_slot;
        let mut var_idse_dn10: f64 = *var_idse_dn10_slot;
        let mut var_idse_dn13: f64 = *var_idse_dn13_slot;
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
        let mut var_iqb_nqs_dn12: f64 = *var_iqb_nqs_dn12_slot;
        let mut var_iqb_nqs_dn13: f64 = *var_iqb_nqs_dn13_slot;
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
        let mut var_iqi_nqs_dn13: f64 = *var_iqi_nqs_dn13_slot;
        let mut var_iqi_nqs_dn2: f64 = *var_iqi_nqs_dn2_slot;
        let mut var_iqi_nqs_dn4: f64 = *var_iqi_nqs_dn4_slot;
        let mut var_iqi_nqs_dn5: f64 = *var_iqi_nqs_dn5_slot;
        let mut var_iqi_nqs_dn6: f64 = *var_iqi_nqs_dn6_slot;
        let mut var_iqi_nqs_dn7: f64 = *var_iqi_nqs_dn7_slot;
        let mut var_iqi_nqs_dn8: f64 = *var_iqi_nqs_dn8_slot;
        let mut var_iqi_nqs_dn9: f64 = *var_iqi_nqs_dn9_slot;
        let mut var_p: f64 = *var_p_slot;
        let mut var_p_dn0: f64 = *var_p_dn0_slot;
        let mut var_p_dn10: f64 = *var_p_dn10_slot;
        let mut var_p_dn13: f64 = *var_p_dn13_slot;
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
        let mut var_qd_nqs_dn13: f64 = *var_qd_nqs_dn13_slot;
        let mut var_qd_nqs_dn2: f64 = *var_qd_nqs_dn2_slot;
        let mut var_qd_nqs_dn4: f64 = *var_qd_nqs_dn4_slot;
        let mut var_qd_nqs_dn5: f64 = *var_qd_nqs_dn5_slot;
        let mut var_qd_nqs_dn6: f64 = *var_qd_nqs_dn6_slot;
        let mut var_qd_nqs_dn7: f64 = *var_qd_nqs_dn7_slot;
        let mut var_qd_nqs_dn8: f64 = *var_qd_nqs_dn8_slot;
        let mut var_qd_nqs_dn9: f64 = *var_qd_nqs_dn9_slot;
        let mut var_qg_nqs: f64 = *var_qg_nqs_slot;
        let mut var_qg_nqs_dn11: f64 = *var_qg_nqs_dn11_slot;
        let mut var_qg_nqs_dn12: f64 = *var_qg_nqs_dn12_slot;
        let mut var_qs_nqs: f64 = *var_qs_nqs_slot;
        let mut var_qs_nqs_dn0: f64 = *var_qs_nqs_dn0_slot;
        let mut var_qs_nqs_dn10: f64 = *var_qs_nqs_dn10_slot;
        let mut var_qs_nqs_dn11: f64 = *var_qs_nqs_dn11_slot;
        let mut var_qs_nqs_dn13: f64 = *var_qs_nqs_dn13_slot;
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
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
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
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
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
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
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
        let mut var_tau_dn13: f64 = *var_tau_dn13_slot;
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
        let mut var_taub_dn13: f64 = *var_taub_dn13_slot;
        let mut var_taub_dn2: f64 = *var_taub_dn2_slot;
        let mut var_taub_dn4: f64 = *var_taub_dn4_slot;
        let mut var_taub_dn5: f64 = *var_taub_dn5_slot;
        let mut var_taub_dn6: f64 = *var_taub_dn6_slot;
        let mut var_taub_dn7: f64 = *var_taub_dn7_slot;
        let mut var_taub_dn8: f64 = *var_taub_dn8_slot;
        let mut var_taub_dn9: f64 = *var_taub_dn9_slot;
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

        let (assign106490_e158617, assign106490_e158617_d_n0, assign106490_e158617_d_n2, assign106490_e158617_d_n4, assign106490_e158617_d_n5, assign106490_e158617_d_n6, assign106490_e158617_d_n7, assign106490_e158617_d_n8, assign106490_e158617_d_n9, assign106490_e158617_d_n10, assign106490_e158617_d_n13,) = {
    if ((var_guard2400 != 0.0) && (var_guard2404 != 0.0)) {
        let assign106490_e158615: f64 = (p.p433 * var_gth);
        (assign106490_e158615, (p.p433 * var_gth_dn0), (p.p433 * var_gth_dn2), (p.p433 * var_gth_dn4), (p.p433 * var_gth_dn5), (p.p433 * var_gth_dn6), (p.p433 * var_gth_dn7), (p.p433 * var_gth_dn8), (p.p433 * var_gth_dn9), (p.p433 * var_gth_dn10), (p.p433 * var_gth_dn13),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign106490_e158617;
        var_t1_dn0 = assign106490_e158617_d_n0;
        var_t1_dn2 = assign106490_e158617_d_n2;
        var_t1_dn4 = assign106490_e158617_d_n4;
        var_t1_dn5 = assign106490_e158617_d_n5;
        var_t1_dn6 = assign106490_e158617_d_n6;
        var_t1_dn7 = assign106490_e158617_d_n7;
        var_t1_dn8 = assign106490_e158617_d_n8;
        var_t1_dn9 = assign106490_e158617_d_n9;
        var_t1_dn10 = assign106490_e158617_d_n10;
        var_t1_dn13 = assign106490_e158617_d_n13;

        let (assign106500_e158629, assign106500_e158629_d_n0, assign106500_e158629_d_n2, assign106500_e158629_d_n4, assign106500_e158629_d_n5, assign106500_e158629_d_n6, assign106500_e158629_d_n7, assign106500_e158629_d_n8, assign106500_e158629_d_n9, assign106500_e158629_d_n10, assign106500_e158629_d_n13,) = {
    if ((var_guard2400 != 0.0) && (var_guard2404 != 0.0)) {
        let assign106500_e158623: f64 = (var_t1 - var_p);
        let assign106500_e158626: f64 = (p.p337 * var_gth);
        let assign106500_e158627: f64 = (assign106500_e158623 - assign106500_e158626);
        (assign106500_e158627, ((var_t1_dn0 - var_p_dn0) - (p.p337 * var_gth_dn0)), ((var_t1_dn2 - var_p_dn2) - (p.p337 * var_gth_dn2)), ((var_t1_dn4 - var_p_dn4) - (p.p337 * var_gth_dn4)), ((var_t1_dn5 - var_p_dn5) - (p.p337 * var_gth_dn5)), ((var_t1_dn6 - var_p_dn6) - (p.p337 * var_gth_dn6)), ((var_t1_dn7 - var_p_dn7) - (p.p337 * var_gth_dn7)), ((var_t1_dn8 - var_p_dn8) - (p.p337 * var_gth_dn8)), ((var_t1_dn9 - var_p_dn9) - (p.p337 * var_gth_dn9)), ((var_t1_dn10 - var_p_dn10) - (p.p337 * var_gth_dn10)), ((var_t1_dn13 - var_p_dn13) - (p.p337 * var_gth_dn13)),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn13,)
    }
};
        var_tmf1 = assign106500_e158629;
        var_tmf1_dn0 = assign106500_e158629_d_n0;
        var_tmf1_dn2 = assign106500_e158629_d_n2;
        var_tmf1_dn4 = assign106500_e158629_d_n4;
        var_tmf1_dn5 = assign106500_e158629_d_n5;
        var_tmf1_dn6 = assign106500_e158629_d_n6;
        var_tmf1_dn7 = assign106500_e158629_d_n7;
        var_tmf1_dn8 = assign106500_e158629_d_n8;
        var_tmf1_dn9 = assign106500_e158629_d_n9;
        var_tmf1_dn10 = assign106500_e158629_d_n10;
        var_tmf1_dn13 = assign106500_e158629_d_n13;

        let (assign106510_e158641, assign106510_e158641_d_n0, assign106510_e158641_d_n2, assign106510_e158641_d_n4, assign106510_e158641_d_n5, assign106510_e158641_d_n6, assign106510_e158641_d_n7, assign106510_e158641_d_n8, assign106510_e158641_d_n9, assign106510_e158641_d_n10, assign106510_e158641_d_n13,) = {
    if ((var_guard2400 != 0.0) && (var_guard2404 != 0.0)) {
        let assign106510_e158635: f64 = (4.0 * var_t1);
        let assign106510_e158638: f64 = (p.p337 * var_gth);
        let assign106510_e158639: f64 = (assign106510_e158635 * assign106510_e158638);
        (assign106510_e158639, (((4.0 * var_t1_dn0) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * var_gth_dn0))), (((4.0 * var_t1_dn2) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * var_gth_dn2))), (((4.0 * var_t1_dn4) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * var_gth_dn4))), (((4.0 * var_t1_dn5) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * var_gth_dn5))), (((4.0 * var_t1_dn6) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * var_gth_dn6))), (((4.0 * var_t1_dn7) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * var_gth_dn7))), (((4.0 * var_t1_dn8) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * var_gth_dn8))), (((4.0 * var_t1_dn9) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * var_gth_dn9))), (((4.0 * var_t1_dn10) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * var_gth_dn10))), (((4.0 * var_t1_dn13) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * var_gth_dn13))),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign106510_e158641;
        var_tmf2_dn0 = assign106510_e158641_d_n0;
        var_tmf2_dn2 = assign106510_e158641_d_n2;
        var_tmf2_dn4 = assign106510_e158641_d_n4;
        var_tmf2_dn5 = assign106510_e158641_d_n5;
        var_tmf2_dn6 = assign106510_e158641_d_n6;
        var_tmf2_dn7 = assign106510_e158641_d_n7;
        var_tmf2_dn8 = assign106510_e158641_d_n8;
        var_tmf2_dn9 = assign106510_e158641_d_n9;
        var_tmf2_dn10 = assign106510_e158641_d_n10;
        var_tmf2_dn13 = assign106510_e158641_d_n13;

        let (assign106520_e158653, assign106520_e158653_d_n0, assign106520_e158653_d_n2, assign106520_e158653_d_n4, assign106520_e158653_d_n5, assign106520_e158653_d_n6, assign106520_e158653_d_n7, assign106520_e158653_d_n8, assign106520_e158653_d_n9, assign106520_e158653_d_n10, assign106520_e158653_d_n13,) = {
    if ((var_guard2400 != 0.0) && (var_guard2404 != 0.0)) {
        let (assign106520_e158651, assign106520_e158651_d_n0, assign106520_e158651_d_n2, assign106520_e158651_d_n4, assign106520_e158651_d_n5, assign106520_e158651_d_n6, assign106520_e158651_d_n7, assign106520_e158651_d_n8, assign106520_e158651_d_n9, assign106520_e158651_d_n10, assign106520_e158651_d_n13,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
            } else {
                let assign106520_e158650: f64 = (-var_tmf2);
                (assign106520_e158650, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn13),)
            }
        };
        (assign106520_e158651, assign106520_e158651_d_n0, assign106520_e158651_d_n2, assign106520_e158651_d_n4, assign106520_e158651_d_n5, assign106520_e158651_d_n6, assign106520_e158651_d_n7, assign106520_e158651_d_n8, assign106520_e158651_d_n9, assign106520_e158651_d_n10, assign106520_e158651_d_n13,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign106520_e158653;
        var_tmf2_dn0 = assign106520_e158653_d_n0;
        var_tmf2_dn2 = assign106520_e158653_d_n2;
        var_tmf2_dn4 = assign106520_e158653_d_n4;
        var_tmf2_dn5 = assign106520_e158653_d_n5;
        var_tmf2_dn6 = assign106520_e158653_d_n6;
        var_tmf2_dn7 = assign106520_e158653_d_n7;
        var_tmf2_dn8 = assign106520_e158653_d_n8;
        var_tmf2_dn9 = assign106520_e158653_d_n9;
        var_tmf2_dn10 = assign106520_e158653_d_n10;
        var_tmf2_dn13 = assign106520_e158653_d_n13;

        let (assign106530_e158664, assign106530_e158664_d_n0, assign106530_e158664_d_n2, assign106530_e158664_d_n4, assign106530_e158664_d_n5, assign106530_e158664_d_n6, assign106530_e158664_d_n7, assign106530_e158664_d_n8, assign106530_e158664_d_n9, assign106530_e158664_d_n10, assign106530_e158664_d_n13,) = {
    if ((var_guard2400 != 0.0) && (var_guard2404 != 0.0)) {
        let assign106530_e158659: f64 = (var_tmf1 * var_tmf1);
        let assign106530_e158661: f64 = (assign106530_e158659 + var_tmf2);
        let assign106530_e158662: f64 = (assign106530_e158661).sqrt();
        (assign106530_e158662, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign106530_e158662)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign106530_e158662)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign106530_e158662)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign106530_e158662)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign106530_e158662)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign106530_e158662)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign106530_e158662)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign106530_e158662)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign106530_e158662)), ((((var_tmf1_dn13 * var_tmf1) + (var_tmf1 * var_tmf1_dn13)) + var_tmf2_dn13) / (2.0 * assign106530_e158662)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign106530_e158664;
        var_tmf2_dn0 = assign106530_e158664_d_n0;
        var_tmf2_dn2 = assign106530_e158664_d_n2;
        var_tmf2_dn4 = assign106530_e158664_d_n4;
        var_tmf2_dn5 = assign106530_e158664_d_n5;
        var_tmf2_dn6 = assign106530_e158664_d_n6;
        var_tmf2_dn7 = assign106530_e158664_d_n7;
        var_tmf2_dn8 = assign106530_e158664_d_n8;
        var_tmf2_dn9 = assign106530_e158664_d_n9;
        var_tmf2_dn10 = assign106530_e158664_d_n10;
        var_tmf2_dn13 = assign106530_e158664_d_n13;

        let (assign106540_e158676, assign106540_e158676_d_n0, assign106540_e158676_d_n2, assign106540_e158676_d_n4, assign106540_e158676_d_n5, assign106540_e158676_d_n6, assign106540_e158676_d_n7, assign106540_e158676_d_n8, assign106540_e158676_d_n9, assign106540_e158676_d_n10, assign106540_e158676_d_n13,) = {
    if ((var_guard2400 != 0.0) && (var_guard2404 != 0.0)) {
        let assign106540_e158672: f64 = (var_tmf1 / var_tmf2);
        let assign106540_e158673: f64 = (1.0 + assign106540_e158672);
        let assign106540_e158674: f64 = (0.5 * assign106540_e158673);
        (assign106540_e158674, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn13 * var_tmf2) - (var_tmf1 * var_tmf2_dn13)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign106540_e158676;
        var_t0_dn0 = assign106540_e158676_d_n0;
        var_t0_dn2 = assign106540_e158676_d_n2;
        var_t0_dn4 = assign106540_e158676_d_n4;
        var_t0_dn5 = assign106540_e158676_d_n5;
        var_t0_dn6 = assign106540_e158676_d_n6;
        var_t0_dn7 = assign106540_e158676_d_n7;
        var_t0_dn8 = assign106540_e158676_d_n8;
        var_t0_dn9 = assign106540_e158676_d_n9;
        var_t0_dn10 = assign106540_e158676_d_n10;
        var_t0_dn13 = assign106540_e158676_d_n13;

        let (assign106550_e158688, assign106550_e158688_d_n0, assign106550_e158688_d_n2, assign106550_e158688_d_n4, assign106550_e158688_d_n5, assign106550_e158688_d_n6, assign106550_e158688_d_n7, assign106550_e158688_d_n8, assign106550_e158688_d_n9, assign106550_e158688_d_n10, assign106550_e158688_d_n13,) = {
    if ((var_guard2400 != 0.0) && (var_guard2404 != 0.0)) {
        let assign106550_e158684: f64 = (var_tmf1 + var_tmf2);
        let assign106550_e158685: f64 = (0.5 * assign106550_e158684);
        let assign106550_e158686: f64 = (var_t1 - assign106550_e158685);
        (assign106550_e158686, (var_t1_dn0 - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_t1_dn2 - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_t1_dn4 - (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_t1_dn5 - (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_t1_dn6 - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (var_t1_dn7 - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (var_t1_dn8 - (0.5 * (var_tmf1_dn8 + var_tmf2_dn8))), (var_t1_dn9 - (0.5 * (var_tmf1_dn9 + var_tmf2_dn9))), (var_t1_dn10 - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (var_t1_dn13 - (0.5 * (var_tmf1_dn13 + var_tmf2_dn13))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign106550_e158688;
        var_t2_dn0 = assign106550_e158688_d_n0;
        var_t2_dn2 = assign106550_e158688_d_n2;
        var_t2_dn4 = assign106550_e158688_d_n4;
        var_t2_dn5 = assign106550_e158688_d_n5;
        var_t2_dn6 = assign106550_e158688_d_n6;
        var_t2_dn7 = assign106550_e158688_d_n7;
        var_t2_dn8 = assign106550_e158688_d_n8;
        var_t2_dn9 = assign106550_e158688_d_n9;
        var_t2_dn10 = assign106550_e158688_d_n10;
        var_t2_dn13 = assign106550_e158688_d_n13;

        let (assign106560_e158694, assign106560_e158694_d_n0, assign106560_e158694_d_n2, assign106560_e158694_d_n4, assign106560_e158694_d_n5, assign106560_e158694_d_n6, assign106560_e158694_d_n7, assign106560_e158694_d_n8, assign106560_e158694_d_n9, assign106560_e158694_d_n10, assign106560_e158694_d_n13,) = {
    if ((var_guard2400 != 0.0) && (var_guard2404 != 0.0)) {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    } else {
        (var_p, var_p_dn0, var_p_dn2, var_p_dn4, var_p_dn5, var_p_dn6, var_p_dn7, var_p_dn8, var_p_dn9, var_p_dn10, var_p_dn13,)
    }
};
        var_p = assign106560_e158694;
        var_p_dn0 = assign106560_e158694_d_n0;
        var_p_dn2 = assign106560_e158694_d_n2;
        var_p_dn4 = assign106560_e158694_d_n4;
        var_p_dn5 = assign106560_e158694_d_n5;
        var_p_dn6 = assign106560_e158694_d_n6;
        var_p_dn7 = assign106560_e158694_d_n7;
        var_p_dn8 = assign106560_e158694_d_n8;
        var_p_dn9 = assign106560_e158694_d_n9;
        var_p_dn10 = assign106560_e158694_d_n10;
        var_p_dn13 = assign106560_e158694_d_n13;

        let (assign106570_e158699, assign106570_e158699_d_n0, assign106570_e158699_d_n2, assign106570_e158699_d_n4, assign106570_e158699_d_n5, assign106570_e158699_d_n6, assign106570_e158699_d_n7, assign106570_e158699_d_n8, assign106570_e158699_d_n9, assign106570_e158699_d_n10, assign106570_e158699_d_n13,) = {
    if (var_guard2400 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_gth, var_gth_dn0, var_gth_dn2, var_gth_dn4, var_gth_dn5, var_gth_dn6, var_gth_dn7, var_gth_dn8, var_gth_dn9, var_gth_dn10, var_gth_dn13,)
    }
};
        var_gth = assign106570_e158699;
        var_gth_dn0 = assign106570_e158699_d_n0;
        var_gth_dn2 = assign106570_e158699_d_n2;
        var_gth_dn4 = assign106570_e158699_d_n4;
        var_gth_dn5 = assign106570_e158699_d_n5;
        var_gth_dn6 = assign106570_e158699_d_n6;
        var_gth_dn7 = assign106570_e158699_d_n7;
        var_gth_dn8 = assign106570_e158699_d_n8;
        var_gth_dn9 = assign106570_e158699_d_n9;
        var_gth_dn10 = assign106570_e158699_d_n10;
        var_gth_dn13 = assign106570_e158699_d_n13;

        let (assign106580_e158704, assign106580_e158704_d_n0, assign106580_e158704_d_n2, assign106580_e158704_d_n4, assign106580_e158704_d_n5, assign106580_e158704_d_n6, assign106580_e158704_d_n7, assign106580_e158704_d_n8, assign106580_e158704_d_n9, assign106580_e158704_d_n10, assign106580_e158704_d_n13,) = {
    if (var_guard2400 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_p, var_p_dn0, var_p_dn2, var_p_dn4, var_p_dn5, var_p_dn6, var_p_dn7, var_p_dn8, var_p_dn9, var_p_dn10, var_p_dn13,)
    }
};
        var_p = assign106580_e158704;
        var_p_dn0 = assign106580_e158704_d_n0;
        var_p_dn2 = assign106580_e158704_d_n2;
        var_p_dn4 = assign106580_e158704_d_n4;
        var_p_dn5 = assign106580_e158704_d_n5;
        var_p_dn6 = assign106580_e158704_d_n6;
        var_p_dn7 = assign106580_e158704_d_n7;
        var_p_dn8 = assign106580_e158704_d_n8;
        var_p_dn9 = assign106580_e158704_d_n9;
        var_p_dn10 = assign106580_e158704_d_n10;
        var_p_dn13 = assign106580_e158704_d_n13;

        let assign106590_e158707: f64 = if var_tau < 1e-15 { 1.0 } else { 0.0 };
        var_guard2405 = assign106590_e158707;

        let (assign106600_e158713, assign106600_e158713_d_n0, assign106600_e158713_d_n2, assign106600_e158713_d_n4, assign106600_e158713_d_n5, assign106600_e158713_d_n6, assign106600_e158713_d_n7, assign106600_e158713_d_n8, assign106600_e158713_d_n9, assign106600_e158713_d_n10, assign106600_e158713_d_n13,) = {
    if ((var_flg_nqs != 0.0) && (var_guard2405 != 0.0)) {
        (1e-15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tau, var_tau_dn0, var_tau_dn2, var_tau_dn4, var_tau_dn5, var_tau_dn6, var_tau_dn7, var_tau_dn8, var_tau_dn9, var_tau_dn10, var_tau_dn13,)
    }
};
        var_tau = assign106600_e158713;
        var_tau_dn0 = assign106600_e158713_d_n0;
        var_tau_dn2 = assign106600_e158713_d_n2;
        var_tau_dn4 = assign106600_e158713_d_n4;
        var_tau_dn5 = assign106600_e158713_d_n5;
        var_tau_dn6 = assign106600_e158713_d_n6;
        var_tau_dn7 = assign106600_e158713_d_n7;
        var_tau_dn8 = assign106600_e158713_d_n8;
        var_tau_dn9 = assign106600_e158713_d_n9;
        var_tau_dn10 = assign106600_e158713_d_n10;
        var_tau_dn13 = assign106600_e158713_d_n13;

        let assign106610_e158716: f64 = if var_taub < 1e-15 { 1.0 } else { 0.0 };
        var_guard2406 = assign106610_e158716;

        let (assign106620_e158722, assign106620_e158722_d_n0, assign106620_e158722_d_n2, assign106620_e158722_d_n4, assign106620_e158722_d_n5, assign106620_e158722_d_n6, assign106620_e158722_d_n7, assign106620_e158722_d_n8, assign106620_e158722_d_n9, assign106620_e158722_d_n10, assign106620_e158722_d_n13,) = {
    if ((var_flg_nqs != 0.0) && (var_guard2406 != 0.0)) {
        (1e-15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_taub, var_taub_dn0, var_taub_dn2, var_taub_dn4, var_taub_dn5, var_taub_dn6, var_taub_dn7, var_taub_dn8, var_taub_dn9, var_taub_dn10, var_taub_dn13,)
    }
};
        var_taub = assign106620_e158722;
        var_taub_dn0 = assign106620_e158722_d_n0;
        var_taub_dn2 = assign106620_e158722_d_n2;
        var_taub_dn4 = assign106620_e158722_d_n4;
        var_taub_dn5 = assign106620_e158722_d_n5;
        var_taub_dn6 = assign106620_e158722_d_n6;
        var_taub_dn7 = assign106620_e158722_d_n7;
        var_taub_dn8 = assign106620_e158722_d_n8;
        var_taub_dn9 = assign106620_e158722_d_n9;
        var_taub_dn10 = assign106620_e158722_d_n10;
        var_taub_dn13 = assign106620_e158722_d_n13;

        let (assign106630_e158730, assign106630_e158730_d_n0, assign106630_e158730_d_n2, assign106630_e158730_d_n4, assign106630_e158730_d_n5, assign106630_e158730_d_n6, assign106630_e158730_d_n7, assign106630_e158730_d_n8, assign106630_e158730_d_n9, assign106630_e158730_d_n10, assign106630_e158730_d_n11, assign106630_e158730_d_n13,) = {
    if (var_flg_nqs != 0.0) {
        let assign106630_e158726: f64 = (var_qi_nqs - var_qi);
        let assign106630_e158728: f64 = (assign106630_e158726 / var_tau);
        (assign106630_e158728, ((((-var_qi_dn0) * var_tau) - (assign106630_e158726 * var_tau_dn0)) / (var_tau * var_tau)), ((((-var_qi_dn2) * var_tau) - (assign106630_e158726 * var_tau_dn2)) / (var_tau * var_tau)), ((((-var_qi_dn4) * var_tau) - (assign106630_e158726 * var_tau_dn4)) / (var_tau * var_tau)), ((((-var_qi_dn5) * var_tau) - (assign106630_e158726 * var_tau_dn5)) / (var_tau * var_tau)), ((((-var_qi_dn6) * var_tau) - (assign106630_e158726 * var_tau_dn6)) / (var_tau * var_tau)), ((((-var_qi_dn7) * var_tau) - (assign106630_e158726 * var_tau_dn7)) / (var_tau * var_tau)), ((((-var_qi_dn8) * var_tau) - (assign106630_e158726 * var_tau_dn8)) / (var_tau * var_tau)), ((((-var_qi_dn9) * var_tau) - (assign106630_e158726 * var_tau_dn9)) / (var_tau * var_tau)), ((((-var_qi_dn10) * var_tau) - (assign106630_e158726 * var_tau_dn10)) / (var_tau * var_tau)), (var_qi_nqs_dn11 / var_tau), ((((-var_qi_dn13) * var_tau) - (assign106630_e158726 * var_tau_dn13)) / (var_tau * var_tau)),)
    } else {
        (var_iqi_nqs, var_iqi_nqs_dn0, var_iqi_nqs_dn2, var_iqi_nqs_dn4, var_iqi_nqs_dn5, var_iqi_nqs_dn6, var_iqi_nqs_dn7, var_iqi_nqs_dn8, var_iqi_nqs_dn9, var_iqi_nqs_dn10, var_iqi_nqs_dn11, var_iqi_nqs_dn13,)
    }
};
        var_iqi_nqs = assign106630_e158730;
        var_iqi_nqs_dn0 = assign106630_e158730_d_n0;
        var_iqi_nqs_dn2 = assign106630_e158730_d_n2;
        var_iqi_nqs_dn4 = assign106630_e158730_d_n4;
        var_iqi_nqs_dn5 = assign106630_e158730_d_n5;
        var_iqi_nqs_dn6 = assign106630_e158730_d_n6;
        var_iqi_nqs_dn7 = assign106630_e158730_d_n7;
        var_iqi_nqs_dn8 = assign106630_e158730_d_n8;
        var_iqi_nqs_dn9 = assign106630_e158730_d_n9;
        var_iqi_nqs_dn10 = assign106630_e158730_d_n10;
        var_iqi_nqs_dn11 = assign106630_e158730_d_n11;
        var_iqi_nqs_dn13 = assign106630_e158730_d_n13;

        let (assign106640_e158738, assign106640_e158738_d_n0, assign106640_e158738_d_n2, assign106640_e158738_d_n4, assign106640_e158738_d_n5, assign106640_e158738_d_n6, assign106640_e158738_d_n7, assign106640_e158738_d_n8, assign106640_e158738_d_n9, assign106640_e158738_d_n10, assign106640_e158738_d_n12, assign106640_e158738_d_n13,) = {
    if (var_flg_nqs != 0.0) {
        let assign106640_e158734: f64 = (var_qb_nqs - var_qbulk);
        let assign106640_e158736: f64 = (assign106640_e158734 / var_taub);
        (assign106640_e158736, ((((-var_qbulk_dn0) * var_taub) - (assign106640_e158734 * var_taub_dn0)) / (var_taub * var_taub)), ((((-var_qbulk_dn2) * var_taub) - (assign106640_e158734 * var_taub_dn2)) / (var_taub * var_taub)), ((((-var_qbulk_dn4) * var_taub) - (assign106640_e158734 * var_taub_dn4)) / (var_taub * var_taub)), ((((-var_qbulk_dn5) * var_taub) - (assign106640_e158734 * var_taub_dn5)) / (var_taub * var_taub)), ((((-var_qbulk_dn6) * var_taub) - (assign106640_e158734 * var_taub_dn6)) / (var_taub * var_taub)), ((((-var_qbulk_dn7) * var_taub) - (assign106640_e158734 * var_taub_dn7)) / (var_taub * var_taub)), ((((-var_qbulk_dn8) * var_taub) - (assign106640_e158734 * var_taub_dn8)) / (var_taub * var_taub)), ((((-var_qbulk_dn9) * var_taub) - (assign106640_e158734 * var_taub_dn9)) / (var_taub * var_taub)), ((((-var_qbulk_dn10) * var_taub) - (assign106640_e158734 * var_taub_dn10)) / (var_taub * var_taub)), (var_qb_nqs_dn12 / var_taub), ((((-var_qbulk_dn13) * var_taub) - (assign106640_e158734 * var_taub_dn13)) / (var_taub * var_taub)),)
    } else {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn4, var_iqb_nqs_dn5, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn8, var_iqb_nqs_dn9, var_iqb_nqs_dn10, var_iqb_nqs_dn12, var_iqb_nqs_dn13,)
    }
};
        var_iqb_nqs = assign106640_e158738;
        var_iqb_nqs_dn0 = assign106640_e158738_d_n0;
        var_iqb_nqs_dn2 = assign106640_e158738_d_n2;
        var_iqb_nqs_dn4 = assign106640_e158738_d_n4;
        var_iqb_nqs_dn5 = assign106640_e158738_d_n5;
        var_iqb_nqs_dn6 = assign106640_e158738_d_n6;
        var_iqb_nqs_dn7 = assign106640_e158738_d_n7;
        var_iqb_nqs_dn8 = assign106640_e158738_d_n8;
        var_iqb_nqs_dn9 = assign106640_e158738_d_n9;
        var_iqb_nqs_dn10 = assign106640_e158738_d_n10;
        var_iqb_nqs_dn12 = assign106640_e158738_d_n12;
        var_iqb_nqs_dn13 = assign106640_e158738_d_n13;

        let (assign106650_e158744, assign106650_e158744_d_n0, assign106650_e158744_d_n2, assign106650_e158744_d_n4, assign106650_e158744_d_n5, assign106650_e158744_d_n6, assign106650_e158744_d_n7, assign106650_e158744_d_n8, assign106650_e158744_d_n9, assign106650_e158744_d_n10, assign106650_e158744_d_n11, assign106650_e158744_d_n13,) = {
    if (var_flg_nqs != 0.0) {
        let assign106650_e158742: f64 = (var_qi_nqs * var_qdrat);
        (assign106650_e158742, (var_qi_nqs * var_qdrat_dn0), (var_qi_nqs * var_qdrat_dn2), (var_qi_nqs * var_qdrat_dn4), (var_qi_nqs * var_qdrat_dn5), (var_qi_nqs * var_qdrat_dn6), (var_qi_nqs * var_qdrat_dn7), (var_qi_nqs * var_qdrat_dn8), (var_qi_nqs * var_qdrat_dn9), (var_qi_nqs * var_qdrat_dn10), (var_qi_nqs_dn11 * var_qdrat), (var_qi_nqs * var_qdrat_dn13),)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn4, var_qd_nqs_dn5, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn8, var_qd_nqs_dn9, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn13,)
    }
};
        var_qd_nqs = assign106650_e158744;
        var_qd_nqs_dn0 = assign106650_e158744_d_n0;
        var_qd_nqs_dn2 = assign106650_e158744_d_n2;
        var_qd_nqs_dn4 = assign106650_e158744_d_n4;
        var_qd_nqs_dn5 = assign106650_e158744_d_n5;
        var_qd_nqs_dn6 = assign106650_e158744_d_n6;
        var_qd_nqs_dn7 = assign106650_e158744_d_n7;
        var_qd_nqs_dn8 = assign106650_e158744_d_n8;
        var_qd_nqs_dn9 = assign106650_e158744_d_n9;
        var_qd_nqs_dn10 = assign106650_e158744_d_n10;
        var_qd_nqs_dn11 = assign106650_e158744_d_n11;
        var_qd_nqs_dn13 = assign106650_e158744_d_n13;

        let (assign106660_e158751, assign106660_e158751_d_n11, assign106660_e158751_d_n12,) = {
    if (var_flg_nqs != 0.0) {
        let assign106660_e158747: f64 = (-var_qi_nqs);
        let assign106660_e158749: f64 = (assign106660_e158747 - var_qb_nqs);
        (assign106660_e158749, (-var_qi_nqs_dn11), (-var_qb_nqs_dn12),)
    } else {
        (var_qg_nqs, var_qg_nqs_dn11, var_qg_nqs_dn12,)
    }
};
        var_qg_nqs = assign106660_e158751;
        var_qg_nqs_dn11 = assign106660_e158751_d_n11;
        var_qg_nqs_dn12 = assign106660_e158751_d_n12;

        let (assign106670_e158759, assign106670_e158759_d_n0, assign106670_e158759_d_n2, assign106670_e158759_d_n4, assign106670_e158759_d_n5, assign106670_e158759_d_n6, assign106670_e158759_d_n7, assign106670_e158759_d_n8, assign106670_e158759_d_n9, assign106670_e158759_d_n10, assign106670_e158759_d_n11, assign106670_e158759_d_n13,) = {
    if (var_flg_nqs != 0.0) {
        let assign106670_e158756: f64 = (1.0 - var_qdrat);
        let assign106670_e158757: f64 = (var_qi_nqs * assign106670_e158756);
        (assign106670_e158757, (var_qi_nqs * (-var_qdrat_dn0)), (var_qi_nqs * (-var_qdrat_dn2)), (var_qi_nqs * (-var_qdrat_dn4)), (var_qi_nqs * (-var_qdrat_dn5)), (var_qi_nqs * (-var_qdrat_dn6)), (var_qi_nqs * (-var_qdrat_dn7)), (var_qi_nqs * (-var_qdrat_dn8)), (var_qi_nqs * (-var_qdrat_dn9)), (var_qi_nqs * (-var_qdrat_dn10)), (var_qi_nqs_dn11 * assign106670_e158756), (var_qi_nqs * (-var_qdrat_dn13)),)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn4, var_qs_nqs_dn5, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn8, var_qs_nqs_dn9, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn13,)
    }
};
        var_qs_nqs = assign106670_e158759;
        var_qs_nqs_dn0 = assign106670_e158759_d_n0;
        var_qs_nqs_dn2 = assign106670_e158759_d_n2;
        var_qs_nqs_dn4 = assign106670_e158759_d_n4;
        var_qs_nqs_dn5 = assign106670_e158759_d_n5;
        var_qs_nqs_dn6 = assign106670_e158759_d_n6;
        var_qs_nqs_dn7 = assign106670_e158759_d_n7;
        var_qs_nqs_dn8 = assign106670_e158759_d_n8;
        var_qs_nqs_dn9 = assign106670_e158759_d_n9;
        var_qs_nqs_dn10 = assign106670_e158759_d_n10;
        var_qs_nqs_dn11 = assign106670_e158759_d_n11;
        var_qs_nqs_dn13 = assign106670_e158759_d_n13;

        let (assign106680_e158764, assign106680_e158764_d_n0, assign106680_e158764_d_n2, assign106680_e158764_d_n4, assign106680_e158764_d_n5, assign106680_e158764_d_n6, assign106680_e158764_d_n7, assign106680_e158764_d_n8, assign106680_e158764_d_n9, assign106680_e158764_d_n10, assign106680_e158764_d_n11, assign106680_e158764_d_n13,) = {
    if (var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iqi_nqs, var_iqi_nqs_dn0, var_iqi_nqs_dn2, var_iqi_nqs_dn4, var_iqi_nqs_dn5, var_iqi_nqs_dn6, var_iqi_nqs_dn7, var_iqi_nqs_dn8, var_iqi_nqs_dn9, var_iqi_nqs_dn10, var_iqi_nqs_dn11, var_iqi_nqs_dn13,)
    }
};
        var_iqi_nqs = assign106680_e158764;
        var_iqi_nqs_dn0 = assign106680_e158764_d_n0;
        var_iqi_nqs_dn2 = assign106680_e158764_d_n2;
        var_iqi_nqs_dn4 = assign106680_e158764_d_n4;
        var_iqi_nqs_dn5 = assign106680_e158764_d_n5;
        var_iqi_nqs_dn6 = assign106680_e158764_d_n6;
        var_iqi_nqs_dn7 = assign106680_e158764_d_n7;
        var_iqi_nqs_dn8 = assign106680_e158764_d_n8;
        var_iqi_nqs_dn9 = assign106680_e158764_d_n9;
        var_iqi_nqs_dn10 = assign106680_e158764_d_n10;
        var_iqi_nqs_dn11 = assign106680_e158764_d_n11;
        var_iqi_nqs_dn13 = assign106680_e158764_d_n13;

        let (assign106690_e158769, assign106690_e158769_d_n0, assign106690_e158769_d_n2, assign106690_e158769_d_n4, assign106690_e158769_d_n5, assign106690_e158769_d_n6, assign106690_e158769_d_n7, assign106690_e158769_d_n8, assign106690_e158769_d_n9, assign106690_e158769_d_n10, assign106690_e158769_d_n12, assign106690_e158769_d_n13,) = {
    if (var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn4, var_iqb_nqs_dn5, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn8, var_iqb_nqs_dn9, var_iqb_nqs_dn10, var_iqb_nqs_dn12, var_iqb_nqs_dn13,)
    }
};
        var_iqb_nqs = assign106690_e158769;
        var_iqb_nqs_dn0 = assign106690_e158769_d_n0;
        var_iqb_nqs_dn2 = assign106690_e158769_d_n2;
        var_iqb_nqs_dn4 = assign106690_e158769_d_n4;
        var_iqb_nqs_dn5 = assign106690_e158769_d_n5;
        var_iqb_nqs_dn6 = assign106690_e158769_d_n6;
        var_iqb_nqs_dn7 = assign106690_e158769_d_n7;
        var_iqb_nqs_dn8 = assign106690_e158769_d_n8;
        var_iqb_nqs_dn9 = assign106690_e158769_d_n9;
        var_iqb_nqs_dn10 = assign106690_e158769_d_n10;
        var_iqb_nqs_dn12 = assign106690_e158769_d_n12;
        var_iqb_nqs_dn13 = assign106690_e158769_d_n13;

        let (assign106700_e158774, assign106700_e158774_d_n0, assign106700_e158774_d_n2, assign106700_e158774_d_n4, assign106700_e158774_d_n5, assign106700_e158774_d_n6, assign106700_e158774_d_n7, assign106700_e158774_d_n8, assign106700_e158774_d_n9, assign106700_e158774_d_n10, assign106700_e158774_d_n11, assign106700_e158774_d_n13,) = {
    if (var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn4, var_qd_nqs_dn5, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn8, var_qd_nqs_dn9, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn13,)
    }
};
        var_qd_nqs = assign106700_e158774;
        var_qd_nqs_dn0 = assign106700_e158774_d_n0;
        var_qd_nqs_dn2 = assign106700_e158774_d_n2;
        var_qd_nqs_dn4 = assign106700_e158774_d_n4;
        var_qd_nqs_dn5 = assign106700_e158774_d_n5;
        var_qd_nqs_dn6 = assign106700_e158774_d_n6;
        var_qd_nqs_dn7 = assign106700_e158774_d_n7;
        var_qd_nqs_dn8 = assign106700_e158774_d_n8;
        var_qd_nqs_dn9 = assign106700_e158774_d_n9;
        var_qd_nqs_dn10 = assign106700_e158774_d_n10;
        var_qd_nqs_dn11 = assign106700_e158774_d_n11;
        var_qd_nqs_dn13 = assign106700_e158774_d_n13;

        let (assign106710_e158779, assign106710_e158779_d_n11, assign106710_e158779_d_n12,) = {
    if (var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_qg_nqs, var_qg_nqs_dn11, var_qg_nqs_dn12,)
    }
};
        var_qg_nqs = assign106710_e158779;
        var_qg_nqs_dn11 = assign106710_e158779_d_n11;
        var_qg_nqs_dn12 = assign106710_e158779_d_n12;

        let (assign106720_e158784, assign106720_e158784_d_n0, assign106720_e158784_d_n2, assign106720_e158784_d_n4, assign106720_e158784_d_n5, assign106720_e158784_d_n6, assign106720_e158784_d_n7, assign106720_e158784_d_n8, assign106720_e158784_d_n9, assign106720_e158784_d_n10, assign106720_e158784_d_n11, assign106720_e158784_d_n13,) = {
    if (var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn4, var_qs_nqs_dn5, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn8, var_qs_nqs_dn9, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn13,)
    }
};
        var_qs_nqs = assign106720_e158784;
        var_qs_nqs_dn0 = assign106720_e158784_d_n0;
        var_qs_nqs_dn2 = assign106720_e158784_d_n2;
        var_qs_nqs_dn4 = assign106720_e158784_d_n4;
        var_qs_nqs_dn5 = assign106720_e158784_d_n5;
        var_qs_nqs_dn6 = assign106720_e158784_d_n6;
        var_qs_nqs_dn7 = assign106720_e158784_d_n7;
        var_qs_nqs_dn8 = assign106720_e158784_d_n8;
        var_qs_nqs_dn9 = assign106720_e158784_d_n9;
        var_qs_nqs_dn10 = assign106720_e158784_d_n10;
        var_qs_nqs_dn11 = assign106720_e158784_d_n11;
        var_qs_nqs_dn13 = assign106720_e158784_d_n13;

        let assign106730_e158787: f64 = (p.p87 * var_mode);
        let assign106730_e158789: f64 = (assign106730_e158787 * var_ids);
        var_idse = assign106730_e158789;
        var_idse_dn0 = (assign106730_e158787 * var_ids_dn0);
        var_idse_dn2 = (assign106730_e158787 * var_ids_dn2);
        var_idse_dn4 = (assign106730_e158787 * var_ids_dn4);
        var_idse_dn5 = (assign106730_e158787 * var_ids_dn5);
        var_idse_dn6 = (assign106730_e158787 * var_ids_dn6);
        var_idse_dn7 = (assign106730_e158787 * var_ids_dn7);
        var_idse_dn8 = (assign106730_e158787 * var_ids_dn8);
        var_idse_dn9 = (assign106730_e158787 * var_ids_dn9);
        var_idse_dn10 = (assign106730_e158787 * var_ids_dn10);
        var_idse_dn13 = (assign106730_e158787 * var_ids_dn13);

        let assign106890_e158837: f64 = var_qg_dn5;
        var_cgdbd = assign106890_e158837;
        var_cgdbd_dn0 = 0.0;
        var_cgdbd_dn2 = 0.0;
        var_cgdbd_dn4 = 0.0;
        var_cgdbd_dn5 = 0.0;
        var_cgdbd_dn6 = 0.0;
        var_cgdbd_dn7 = 0.0;
        var_cgdbd_dn8 = 0.0;
        var_cgdbd_dn9 = 0.0;
        var_cgdbd_dn10 = 0.0;
        var_cgdbd_dn13 = 0.0;

        let assign106900_e158840: f64 = (p.p87 * var_cgdbd);
        var_cgdbd = assign106900_e158840;
        var_cgdbd_dn0 = (p.p87 * var_cgdbd_dn0);
        var_cgdbd_dn2 = (p.p87 * var_cgdbd_dn2);
        var_cgdbd_dn4 = (p.p87 * var_cgdbd_dn4);
        var_cgdbd_dn5 = (p.p87 * var_cgdbd_dn5);
        var_cgdbd_dn6 = (p.p87 * var_cgdbd_dn6);
        var_cgdbd_dn7 = (p.p87 * var_cgdbd_dn7);
        var_cgdbd_dn8 = (p.p87 * var_cgdbd_dn8);
        var_cgdbd_dn9 = (p.p87 * var_cgdbd_dn9);
        var_cgdbd_dn10 = (p.p87 * var_cgdbd_dn10);
        var_cgdbd_dn13 = (p.p87 * var_cgdbd_dn13);

        let assign106910_e158843: f64 = var_qg_dn7;
        var_cgsbd = assign106910_e158843;
        var_cgsbd_dn0 = 0.0;
        var_cgsbd_dn2 = 0.0;
        var_cgsbd_dn4 = 0.0;
        var_cgsbd_dn5 = 0.0;
        var_cgsbd_dn6 = 0.0;
        var_cgsbd_dn7 = 0.0;
        var_cgsbd_dn8 = 0.0;
        var_cgsbd_dn9 = 0.0;
        var_cgsbd_dn10 = 0.0;
        var_cgsbd_dn13 = 0.0;

        *var_cgdbd_slot = var_cgdbd;
        *var_cgdbd_dn0_slot = var_cgdbd_dn0;
        *var_cgdbd_dn10_slot = var_cgdbd_dn10;
        *var_cgdbd_dn13_slot = var_cgdbd_dn13;
        *var_cgdbd_dn2_slot = var_cgdbd_dn2;
        *var_cgdbd_dn4_slot = var_cgdbd_dn4;
        *var_cgdbd_dn5_slot = var_cgdbd_dn5;
        *var_cgdbd_dn6_slot = var_cgdbd_dn6;
        *var_cgdbd_dn7_slot = var_cgdbd_dn7;
        *var_cgdbd_dn8_slot = var_cgdbd_dn8;
        *var_cgdbd_dn9_slot = var_cgdbd_dn9;
        *var_cgsbd_slot = var_cgsbd;
        *var_cgsbd_dn0_slot = var_cgsbd_dn0;
        *var_cgsbd_dn10_slot = var_cgsbd_dn10;
        *var_cgsbd_dn13_slot = var_cgsbd_dn13;
        *var_cgsbd_dn2_slot = var_cgsbd_dn2;
        *var_cgsbd_dn4_slot = var_cgsbd_dn4;
        *var_cgsbd_dn5_slot = var_cgsbd_dn5;
        *var_cgsbd_dn6_slot = var_cgsbd_dn6;
        *var_cgsbd_dn7_slot = var_cgsbd_dn7;
        *var_cgsbd_dn8_slot = var_cgsbd_dn8;
        *var_cgsbd_dn9_slot = var_cgsbd_dn9;
        *var_gth_slot = var_gth;
        *var_gth_dn0_slot = var_gth_dn0;
        *var_gth_dn10_slot = var_gth_dn10;
        *var_gth_dn13_slot = var_gth_dn13;
        *var_gth_dn2_slot = var_gth_dn2;
        *var_gth_dn4_slot = var_gth_dn4;
        *var_gth_dn5_slot = var_gth_dn5;
        *var_gth_dn6_slot = var_gth_dn6;
        *var_gth_dn7_slot = var_gth_dn7;
        *var_gth_dn8_slot = var_gth_dn8;
        *var_gth_dn9_slot = var_gth_dn9;
        *var_guard2405_slot = var_guard2405;
        *var_guard2406_slot = var_guard2406;
        *var_idse_slot = var_idse;
        *var_idse_dn0_slot = var_idse_dn0;
        *var_idse_dn10_slot = var_idse_dn10;
        *var_idse_dn13_slot = var_idse_dn13;
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
        *var_iqb_nqs_dn12_slot = var_iqb_nqs_dn12;
        *var_iqb_nqs_dn13_slot = var_iqb_nqs_dn13;
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
        *var_iqi_nqs_dn13_slot = var_iqi_nqs_dn13;
        *var_iqi_nqs_dn2_slot = var_iqi_nqs_dn2;
        *var_iqi_nqs_dn4_slot = var_iqi_nqs_dn4;
        *var_iqi_nqs_dn5_slot = var_iqi_nqs_dn5;
        *var_iqi_nqs_dn6_slot = var_iqi_nqs_dn6;
        *var_iqi_nqs_dn7_slot = var_iqi_nqs_dn7;
        *var_iqi_nqs_dn8_slot = var_iqi_nqs_dn8;
        *var_iqi_nqs_dn9_slot = var_iqi_nqs_dn9;
        *var_p_slot = var_p;
        *var_p_dn0_slot = var_p_dn0;
        *var_p_dn10_slot = var_p_dn10;
        *var_p_dn13_slot = var_p_dn13;
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
        *var_qd_nqs_dn13_slot = var_qd_nqs_dn13;
        *var_qd_nqs_dn2_slot = var_qd_nqs_dn2;
        *var_qd_nqs_dn4_slot = var_qd_nqs_dn4;
        *var_qd_nqs_dn5_slot = var_qd_nqs_dn5;
        *var_qd_nqs_dn6_slot = var_qd_nqs_dn6;
        *var_qd_nqs_dn7_slot = var_qd_nqs_dn7;
        *var_qd_nqs_dn8_slot = var_qd_nqs_dn8;
        *var_qd_nqs_dn9_slot = var_qd_nqs_dn9;
        *var_qg_nqs_slot = var_qg_nqs;
        *var_qg_nqs_dn11_slot = var_qg_nqs_dn11;
        *var_qg_nqs_dn12_slot = var_qg_nqs_dn12;
        *var_qs_nqs_slot = var_qs_nqs;
        *var_qs_nqs_dn0_slot = var_qs_nqs_dn0;
        *var_qs_nqs_dn10_slot = var_qs_nqs_dn10;
        *var_qs_nqs_dn11_slot = var_qs_nqs_dn11;
        *var_qs_nqs_dn13_slot = var_qs_nqs_dn13;
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
        *var_t0_dn13_slot = var_t0_dn13;
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
        *var_t1_dn13_slot = var_t1_dn13;
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
        *var_t2_dn13_slot = var_t2_dn13;
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
        *var_tau_dn13_slot = var_tau_dn13;
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
        *var_taub_dn13_slot = var_taub_dn13;
        *var_taub_dn2_slot = var_taub_dn2;
        *var_taub_dn4_slot = var_taub_dn4;
        *var_taub_dn5_slot = var_taub_dn5;
        *var_taub_dn6_slot = var_taub_dn6;
        *var_taub_dn7_slot = var_taub_dn7;
        *var_taub_dn8_slot = var_taub_dn8;
        *var_taub_dn9_slot = var_taub_dn9;
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
    }

    pub(super) fn stamp_transient_block_377(
        p: &Parameters,
        var_cgdbd: f64,
        var_cgdbd_dn0: f64,
        var_cgdbd_dn10: f64,
        var_cgdbd_dn13: f64,
        var_cgdbd_dn2: f64,
        var_cgdbd_dn4: f64,
        var_cgdbd_dn5: f64,
        var_cgdbd_dn6: f64,
        var_cgdbd_dn7: f64,
        var_cgdbd_dn8: f64,
        var_cgdbd_dn9: f64,
        var_mode: f64,
        var_p: f64,
        var_p_dn0: f64,
        var_p_dn10: f64,
        var_p_dn13: f64,
        var_p_dn2: f64,
        var_p_dn4: f64,
        var_p_dn5: f64,
        var_p_dn6: f64,
        var_p_dn7: f64,
        var_p_dn8: f64,
        var_p_dn9: f64,
        var_cgsb_slot: &mut f64,
        var_cgsb_dn0_slot: &mut f64,
        var_cgsb_dn10_slot: &mut f64,
        var_cgsb_dn13_slot: &mut f64,
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
        var_cgsbd_dn13_slot: &mut f64,
        var_cgsbd_dn2_slot: &mut f64,
        var_cgsbd_dn4_slot: &mut f64,
        var_cgsbd_dn5_slot: &mut f64,
        var_cgsbd_dn6_slot: &mut f64,
        var_cgsbd_dn7_slot: &mut f64,
        var_cgsbd_dn8_slot: &mut f64,
        var_cgsbd_dn9_slot: &mut f64,
        var_cqb_slot: &mut f64,
        var_cqi_slot: &mut f64,
        var_guard2407_slot: &mut f64,
        var_guard2409_slot: &mut f64,
        var_guard2410_slot: &mut f64,
        var_guard2413_slot: &mut f64,
        var_itemp_slot: &mut f64,
        var_itemp_dn0_slot: &mut f64,
        var_itemp_dn10_slot: &mut f64,
        var_itemp_dn13_slot: &mut f64,
        var_itemp_dn2_slot: &mut f64,
        var_itemp_dn4_slot: &mut f64,
        var_itemp_dn5_slot: &mut f64,
        var_itemp_dn6_slot: &mut f64,
        var_itemp_dn7_slot: &mut f64,
        var_itemp_dn8_slot: &mut f64,
        var_itemp_dn9_slot: &mut f64,
    ) {
        let mut var_cgsb: f64 = *var_cgsb_slot;
        let mut var_cgsb_dn0: f64 = *var_cgsb_dn0_slot;
        let mut var_cgsb_dn10: f64 = *var_cgsb_dn10_slot;
        let mut var_cgsb_dn13: f64 = *var_cgsb_dn13_slot;
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
        let mut var_cgsbd_dn13: f64 = *var_cgsbd_dn13_slot;
        let mut var_cgsbd_dn2: f64 = *var_cgsbd_dn2_slot;
        let mut var_cgsbd_dn4: f64 = *var_cgsbd_dn4_slot;
        let mut var_cgsbd_dn5: f64 = *var_cgsbd_dn5_slot;
        let mut var_cgsbd_dn6: f64 = *var_cgsbd_dn6_slot;
        let mut var_cgsbd_dn7: f64 = *var_cgsbd_dn7_slot;
        let mut var_cgsbd_dn8: f64 = *var_cgsbd_dn8_slot;
        let mut var_cgsbd_dn9: f64 = *var_cgsbd_dn9_slot;
        let mut var_cqb: f64 = *var_cqb_slot;
        let mut var_cqi: f64 = *var_cqi_slot;
        let mut var_guard2407: f64 = *var_guard2407_slot;
        let mut var_guard2409: f64 = *var_guard2409_slot;
        let mut var_guard2410: f64 = *var_guard2410_slot;
        let mut var_guard2413: f64 = *var_guard2413_slot;
        let mut var_itemp: f64 = *var_itemp_slot;
        let mut var_itemp_dn0: f64 = *var_itemp_dn0_slot;
        let mut var_itemp_dn10: f64 = *var_itemp_dn10_slot;
        let mut var_itemp_dn13: f64 = *var_itemp_dn13_slot;
        let mut var_itemp_dn2: f64 = *var_itemp_dn2_slot;
        let mut var_itemp_dn4: f64 = *var_itemp_dn4_slot;
        let mut var_itemp_dn5: f64 = *var_itemp_dn5_slot;
        let mut var_itemp_dn6: f64 = *var_itemp_dn6_slot;
        let mut var_itemp_dn7: f64 = *var_itemp_dn7_slot;
        let mut var_itemp_dn8: f64 = *var_itemp_dn8_slot;
        let mut var_itemp_dn9: f64 = *var_itemp_dn9_slot;

        let assign106920_e158846: f64 = (p.p87 * var_cgsbd);
        var_cgsbd = assign106920_e158846;
        var_cgsbd_dn0 = (p.p87 * var_cgsbd_dn0);
        var_cgsbd_dn2 = (p.p87 * var_cgsbd_dn2);
        var_cgsbd_dn4 = (p.p87 * var_cgsbd_dn4);
        var_cgsbd_dn5 = (p.p87 * var_cgsbd_dn5);
        var_cgsbd_dn6 = (p.p87 * var_cgsbd_dn6);
        var_cgsbd_dn7 = (p.p87 * var_cgsbd_dn7);
        var_cgsbd_dn8 = (p.p87 * var_cgsbd_dn8);
        var_cgsbd_dn9 = (p.p87 * var_cgsbd_dn9);
        var_cgsbd_dn10 = (p.p87 * var_cgsbd_dn10);
        var_cgsbd_dn13 = (p.p87 * var_cgsbd_dn13);

        let assign107290_e158961: f64 = if var_mode == 1.0 { 1.0 } else { 0.0 };
        var_guard2407 = assign107290_e158961;

        let (assign107320_e158973, assign107320_e158973_d_n0, assign107320_e158973_d_n2, assign107320_e158973_d_n4, assign107320_e158973_d_n5, assign107320_e158973_d_n6, assign107320_e158973_d_n7, assign107320_e158973_d_n8, assign107320_e158973_d_n9, assign107320_e158973_d_n10, assign107320_e158973_d_n13,) = {
    if (var_guard2407 != 0.0) {
        (var_cgsbd, var_cgsbd_dn0, var_cgsbd_dn2, var_cgsbd_dn4, var_cgsbd_dn5, var_cgsbd_dn6, var_cgsbd_dn7, var_cgsbd_dn8, var_cgsbd_dn9, var_cgsbd_dn10, var_cgsbd_dn13,)
    } else {
        (var_cgsb, var_cgsb_dn0, var_cgsb_dn2, var_cgsb_dn4, var_cgsb_dn5, var_cgsb_dn6, var_cgsb_dn7, var_cgsb_dn8, var_cgsb_dn9, var_cgsb_dn10, var_cgsb_dn13,)
    }
};
        var_cgsb = assign107320_e158973;
        var_cgsb_dn0 = assign107320_e158973_d_n0;
        var_cgsb_dn2 = assign107320_e158973_d_n2;
        var_cgsb_dn4 = assign107320_e158973_d_n4;
        var_cgsb_dn5 = assign107320_e158973_d_n5;
        var_cgsb_dn6 = assign107320_e158973_d_n6;
        var_cgsb_dn7 = assign107320_e158973_d_n7;
        var_cgsb_dn8 = assign107320_e158973_d_n8;
        var_cgsb_dn9 = assign107320_e158973_d_n9;
        var_cgsb_dn10 = assign107320_e158973_d_n10;
        var_cgsb_dn13 = assign107320_e158973_d_n13;

        let (assign107420_e159017, assign107420_e159017_d_n0, assign107420_e159017_d_n2, assign107420_e159017_d_n4, assign107420_e159017_d_n5, assign107420_e159017_d_n6, assign107420_e159017_d_n7, assign107420_e159017_d_n8, assign107420_e159017_d_n9, assign107420_e159017_d_n10, assign107420_e159017_d_n13,) = {
    if (var_guard2407 == 0.0) {
        (var_cgdbd, var_cgdbd_dn0, var_cgdbd_dn2, var_cgdbd_dn4, var_cgdbd_dn5, var_cgdbd_dn6, var_cgdbd_dn7, var_cgdbd_dn8, var_cgdbd_dn9, var_cgdbd_dn10, var_cgdbd_dn13,)
    } else {
        (var_cgsb, var_cgsb_dn0, var_cgsb_dn2, var_cgsb_dn4, var_cgsb_dn5, var_cgsb_dn6, var_cgsb_dn7, var_cgsb_dn8, var_cgsb_dn9, var_cgsb_dn10, var_cgsb_dn13,)
    }
};
        var_cgsb = assign107420_e159017;
        var_cgsb_dn0 = assign107420_e159017_d_n0;
        var_cgsb_dn2 = assign107420_e159017_d_n2;
        var_cgsb_dn4 = assign107420_e159017_d_n4;
        var_cgsb_dn5 = assign107420_e159017_d_n5;
        var_cgsb_dn6 = assign107420_e159017_d_n6;
        var_cgsb_dn7 = assign107420_e159017_d_n7;
        var_cgsb_dn8 = assign107420_e159017_d_n8;
        var_cgsb_dn9 = assign107420_e159017_d_n9;
        var_cgsb_dn10 = assign107420_e159017_d_n10;
        var_cgsb_dn13 = assign107420_e159017_d_n13;

        let assign107650_e159080: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        var_guard2409 = assign107650_e159080;

        let assign107660_e159083: f64 = if p.p24 == 1.0 { 1.0 } else { 0.0 };
        var_guard2410 = assign107660_e159083;

        let assign107710_e159112: f64 = if p.p53 > 0.0 { 1.0 } else { 0.0 };
        var_guard2413 = assign107710_e159112;

        let (assign107720_e159116, assign107720_e159116_d_n0, assign107720_e159116_d_n2, assign107720_e159116_d_n4, assign107720_e159116_d_n5, assign107720_e159116_d_n6, assign107720_e159116_d_n7, assign107720_e159116_d_n8, assign107720_e159116_d_n9, assign107720_e159116_d_n10, assign107720_e159116_d_n13,) = {
    if (var_guard2413 != 0.0) {
        (var_p, var_p_dn0, var_p_dn2, var_p_dn4, var_p_dn5, var_p_dn6, var_p_dn7, var_p_dn8, var_p_dn9, var_p_dn10, var_p_dn13,)
    } else {
        (var_itemp, var_itemp_dn0, var_itemp_dn2, var_itemp_dn4, var_itemp_dn5, var_itemp_dn6, var_itemp_dn7, var_itemp_dn8, var_itemp_dn9, var_itemp_dn10, var_itemp_dn13,)
    }
};
        var_itemp = assign107720_e159116;
        var_itemp_dn0 = assign107720_e159116_d_n0;
        var_itemp_dn2 = assign107720_e159116_d_n2;
        var_itemp_dn4 = assign107720_e159116_d_n4;
        var_itemp_dn5 = assign107720_e159116_d_n5;
        var_itemp_dn6 = assign107720_e159116_d_n6;
        var_itemp_dn7 = assign107720_e159116_d_n7;
        var_itemp_dn8 = assign107720_e159116_d_n8;
        var_itemp_dn9 = assign107720_e159116_d_n9;
        var_itemp_dn10 = assign107720_e159116_d_n10;
        var_itemp_dn13 = assign107720_e159116_d_n13;

        let (assign107740_e159125,) = {
    if (p.p28 != 0.0) {
        (1.0,)
    } else {
        (var_cqi,)
    }
};
        var_cqi = assign107740_e159125;

        let (assign107750_e159129,) = {
    if (p.p28 != 0.0) {
        (1.0,)
    } else {
        (var_cqb,)
    }
};
        var_cqb = assign107750_e159129;

        *var_cgsb_slot = var_cgsb;
        *var_cgsb_dn0_slot = var_cgsb_dn0;
        *var_cgsb_dn10_slot = var_cgsb_dn10;
        *var_cgsb_dn13_slot = var_cgsb_dn13;
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
        *var_cgsbd_dn13_slot = var_cgsbd_dn13;
        *var_cgsbd_dn2_slot = var_cgsbd_dn2;
        *var_cgsbd_dn4_slot = var_cgsbd_dn4;
        *var_cgsbd_dn5_slot = var_cgsbd_dn5;
        *var_cgsbd_dn6_slot = var_cgsbd_dn6;
        *var_cgsbd_dn7_slot = var_cgsbd_dn7;
        *var_cgsbd_dn8_slot = var_cgsbd_dn8;
        *var_cgsbd_dn9_slot = var_cgsbd_dn9;
        *var_cqb_slot = var_cqb;
        *var_cqi_slot = var_cqi;
        *var_guard2407_slot = var_guard2407;
        *var_guard2409_slot = var_guard2409;
        *var_guard2410_slot = var_guard2410;
        *var_guard2413_slot = var_guard2413;
        *var_itemp_slot = var_itemp;
        *var_itemp_dn0_slot = var_itemp_dn0;
        *var_itemp_dn10_slot = var_itemp_dn10;
        *var_itemp_dn13_slot = var_itemp_dn13;
        *var_itemp_dn2_slot = var_itemp_dn2;
        *var_itemp_dn4_slot = var_itemp_dn4;
        *var_itemp_dn5_slot = var_itemp_dn5;
        *var_itemp_dn6_slot = var_itemp_dn6;
        *var_itemp_dn7_slot = var_itemp_dn7;
        *var_itemp_dn8_slot = var_itemp_dn8;
        *var_itemp_dn9_slot = var_itemp_dn9;
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

        s.b[1004] = (s.v[452] < 0.0);
        s.store_scalar(1004, if s.b[1004] { 1.0 } else { 0.0 });

        if s.b[1004] {
            s.store_scalar(452, 0.0);
        }

        s.b[1005] = (s.v[452] > 0.0);
        s.store_scalar(1005, if s.b[1005] { 1.0 } else { 0.0 });

        if s.b[1005] {
            s.store_scalar(452, 0.0);
        }

        s.b[1007] = (s.v[451] < 0.0);
        s.store_scalar(1007, if s.b[1007] { 1.0 } else { 0.0 });

        if s.b[1007] {
            s.store_scalar(451, 0.0);
        }

        s.b[1010] = (s.v[453] < 0.0);
        s.store_scalar(1010, if s.b[1010] { 1.0 } else { 0.0 });

        if s.b[1010] {
            s.store_scalar(453, 0.0);
        }

        s.b[1011] = (s.v[453] > 1.0);
        s.store_scalar(1011, if s.b[1011] { 1.0 } else { 0.0 });

        if s.b[1011] {
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

        s.b[1106] = ((s.v[963] < 3.0) && (s.v[963] > 0.0));
        s.store_scalar(1106, if s.b[1106] { 1.0 } else { 0.0 });

        s.b[1109] = (s.v[964] < 5000000000000000.0);
        s.store_scalar(1109, if s.b[1109] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1109]) {
            s.store_scalar(964, 5000000000000000.0);
        }

        s.b[1110] = (s.v[964] > 1e18);
        s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1110]) {
            s.store_scalar(964, 1e18);
        }

        s.b[1113] = (s.v[965] < 1e-8);
        s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1113]) {
            s.store_scalar(965, 1e-8);
        }

        s.b[1114] = (s.v[965] > 1e-6);
        s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1114]) {
            s.store_scalar(965, 1e-6);
        }

        s.b[1117] = (s.v[966] < 1.0);
        s.store_scalar(1117, if s.b[1117] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1117]) {
            s.store_scalar(966, 1.0);
        }

        s.b[1118] = (s.v[966] > 100000.0);
        s.store_scalar(1118, if s.b[1118] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1118]) {
            s.store_scalar(966, 100000.0);
        }

        s.b[1121] = (s.v[967] < 1.0);
        s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1121]) {
            s.store_scalar(967, 1.0);
        }

        s.b[1122] = (s.v[967] > 100000.0);
        s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1122]) {
            s.store_scalar(967, 100000.0);
        }

        s.b[1125] = (s.v[971] < 1.0);
        s.store_scalar(1125, if s.b[1125] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1125]) {
            s.store_scalar(971, 1.0);
        }

        s.b[1126] = (s.v[971] > 100000.0);
        s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1126]) {
            s.store_scalar(971, 100000.0);
        }

        s.b[1129] = (s.v[975] < 0.1);
        s.store_scalar(1129, if s.b[1129] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1129]) {
            s.store_scalar(975, 0.1);
        }

        s.b[1130] = (s.v[975] > 4.0);
        s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1130]) {
            s.store_scalar(975, 4.0);
        }

        s.b[1133] = (s.v[972] < 0.0);
        s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1133]) {
            s.store_scalar(972, 0.0);
        }

        s.b[1134] = (s.v[972] > 5.0);
        s.store_scalar(1134, if s.b[1134] { 1.0 } else { 0.0 });

        if (s.b[1106] && s.b[1134]) {
            s.store_scalar(972, 5.0);
        }

        s.b[1135] = (s.v[963] == 3.0);
        s.store_scalar(1135, if s.b[1135] { 1.0 } else { 0.0 });

        s.b[1138] = (s.v[964] < 5000000000000000.0);
        s.store_scalar(1138, if s.b[1138] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1138]) {
            s.store_scalar(964, 5000000000000000.0);
        }

        s.b[1139] = (s.v[964] > 1e18);
        s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1139]) {
            s.store_scalar(964, 1e18);
        }

        s.b[1142] = (s.v[965] < 1e-8);
        s.store_scalar(1142, if s.b[1142] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1142]) {
            s.store_scalar(965, 1e-8);
        }

        s.b[1143] = (s.v[965] > 1e-6);
        s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1143]) {
            s.store_scalar(965, 1e-6);
        }

        s.b[1146] = (s.v[966] < 1.0);
        s.store_scalar(1146, if s.b[1146] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1146]) {
            s.store_scalar(966, 1.0);
        }

        s.b[1147] = (s.v[966] > 10000000000.0);
        s.store_scalar(1147, if s.b[1147] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1147]) {
            s.store_scalar(966, 10000000000.0);
        }

        s.b[1150] = (s.v[971] < 100.0);
        s.store_scalar(1150, if s.b[1150] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1150]) {
            s.store_scalar(971, 100.0);
        }

        s.b[1151] = (s.v[971] > 2000000000.0);
        s.store_scalar(1151, if s.b[1151] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1151]) {
            s.store_scalar(971, 2000000000.0);
        }

        s.b[1154] = (s.v[972] < 0.0);
        s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1154]) {
            s.store_scalar(972, 0.0);
        }

        s.b[1155] = (s.v[972] > 5.0);
        s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });

        if (((!s.b[1106]) && s.b[1135]) && s.b[1155]) {
            s.store_scalar(972, 5.0);
        }

        s.store_scalar(543, p.p96);

        s.b[1164] = (s.v[543] < p.p95);
        s.store_scalar(1164, if s.b[1164] { 1.0 } else { 0.0 });

        if s.b[1164] {
            s.store_scalar(543, p.p95);
        }

        s.b[1165] = (s.v[543] > 5e-7);
        s.store_scalar(1165, if s.b[1165] { 1.0 } else { 0.0 });

        if s.b[1165] {
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

        s.b[1181] = (s.v[963] != 0.0);
        s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });

        if s.b[1181] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p342));
            s.store_mul_offset_ad_rhs(964, 964, A::div_from_scalar(p.p341, s.ad_value(337)), 1.0);
        }

        s.b[1182] = (s.v[964] < 1e21);
        s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });

        if (s.b[1181] && s.b[1182]) {
            s.store_scalar(964, 1e21);
        }

        if s.b[1181] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p369));
            s.store_scaled_offset_ad(973, A::div_from_scalar(p.p368, s.ad_value(337)), 1.0, s.v[973]);
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p362));
            s.store_scaled_offset_ad(972, A::div_from_scalar(p.p361, s.ad_value(337)), 1.0, p.p360);
        }

        s.b[1183] = (s.v[972] < 0.0);
        s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });

        if (s.b[1181] && s.b[1183]) {
            s.store_scalar(972, 0.0);
        }

        if s.b[1181] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p348));
            s.store_scaled_offset_ad(966, A::div_from_scalar(p.p347, s.ad_value(337)), 1.0, p.p346);
        }

        s.b[1184] = (s.v[966] < 1.0);
        s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1181] && s.b[1184]) {
            s.store_scalar(966, 1.0);
        }

        if s.b[1181] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p351));
            s.store_scaled_offset_ad(968, A::div_from_scalar(p.p350, s.ad_value(337)), 1.0, p.p349);
        }

        s.b[1185] = (s.v[968] < 0.0);
        s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });

        if (s.b[1181] && s.b[1185]) {
            s.store_scalar(968, 0.0);
        }

        if s.b[1181] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p357));
            s.store_scaled_offset_ad(967, A::div_from_scalar(p.p356, s.ad_value(337)), 1.0, p.p354);
        }

        s.b[1186] = (s.v[967] < 0.0);
        s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });

        if (s.b[1181] && s.b[1186]) {
            s.store_scalar(967, 0.0);
        }

        if s.b[1181] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p359));
            s.store_scaled_offset_ad(969, A::div_from_scalar(p.p358, s.ad_value(337)), 1.0, p.p355);
        }

        s.b[1187] = (s.v[969] < 0.0);
        s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });

        if (s.b[1181] && s.b[1187]) {
            s.store_scalar(969, 0.0);
        }

        if s.b[1181] {
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p373));
            s.store_scaled_offset_ad(974, A::div_from_scalar(p.p372, s.ad_value(337)), 1.0, s.v[974]);
            s.store_scalar(337, ((s.v[576]) as f64).powf(p.p375));
            s.store_mul_offset_ad_rhs(975, 975, A::div_from_scalar(p.p374, s.ad_value(337)), 1.0);
        }

        s.b[1188] = (s.v[975] < 0.1);
        s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });

        if (s.b[1181] && s.b[1188]) {
            s.store_scalar(975, 0.1);
        }

        if (!s.b[1181]) {
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

        s.b[1240] = ((s.v[450] * s.v[451]) > 1.0);
        s.store_scalar(1240, if s.b[1240] { 1.0 } else { 0.0 });

        if s.b[1240] {
            s.store_div_from_scalar(450, 1.0, 451);
        }

        s.b[1242] = ((p.p40 == 1.0) && (((p.p19 > 0.0) && (s.v[459] == 0.0)) || ((p.p18 > 0.0) && (s.v[460] == 0.0))));
        s.store_scalar(1242, if s.b[1242] { 1.0 } else { 0.0 });

        if s.b[1242] {
            s.store_scalar(449, 0.0);
        }

        if (!s.b[1242]) {
            s.store_scalar(449, p.p40);
        }

        s.b[1243] = (s.v[449] == 1.0);
        s.store_scalar(1243, if s.b[1243] { 1.0 } else { 0.0 });

        if s.b[1243] {
            s.store_scalar(75, (if (p.p19 > 0.0) { 1.0 } else { 0.0 }));
        }

        if s.b[1243] {
            s.store_scalar(76, (if (p.p18 > 0.0) { 1.0 } else { 0.0 }));
        }

        s.b[1244] = ((p.p17 == 0.0) || (p.p17 == 2.0));
        s.store_scalar(1244, if s.b[1244] { 1.0 } else { 0.0 });

        if ((!s.b[1243]) && s.b[1244]) {
            s.store_scalar(75, 0.0);
            s.store_scalar(76, 0.0);
        }

        if ((!s.b[1243]) && (!s.b[1244])) {
            s.store_scalar(335, (((p.p130 * p.p2) * p.p7) + (((s.v[530] + s.v[538]) * (((p.p67 * s.v[536]) * 1000000.0) + s.v[534])) * (((p.p68 * p.p100) * 1000000.0) + p.p101))));
        }

        if ((!s.b[1243]) && (!s.b[1244])) {
            s.store_scalar(75, (if (s.v[335] > 0.0) { 1.0 } else { 0.0 }));
        }

        if ((!s.b[1243]) && (!s.b[1244])) {
            s.store_scalar(335, (((p.p131 * p.p3) * p.p7) + ((s.v[540] * (((p.p69 * s.v[536]) * 1000000.0) + s.v[534])) * (((p.p70 * p.p100) * 1000000.0) + p.p101))));
        }

        if ((!s.b[1243]) && (!s.b[1244])) {
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

        s.b[1246] = (((p.p235 == 0.0) && (p.p237 == 0.0)) || (p.p236 == 0.0));
        s.store_scalar(1246, if s.b[1246] { 1.0 } else { 0.0 });

        if s.b[1246] {
            s.store_scalar(765, 0.0);
        }

        if (!s.b[1246]) {
            s.store_scalar(765, 1.0);
        }

        s.store_scalar(581, (s.v[580] * s.v[576]));

        s.store_scalar(777, (p.p289 * 1000000.0));

        s.store_scalar(616, (s.v[457] - (s.v[764] * (9.025e-5 + (s.v[764] * 1e-7)))));

        s.store_scalar(617, (8.8541878e-12 * p.p267));

        s.copy_ad(618, 452);

        s.b[1247] = (s.v[471] == 0.0);
        s.store_scalar(1247, if s.b[1247] { 1.0 } else { 0.0 });

        if s.b[1247] {
            s.store_scalar(615, 0.0);
            s.store_scalar(642, 0.0);
        }

        if (!s.b[1247]) {
            s.store_scalar(615, 1.0);
            s.store_scalar(642, ((((1.0 + (1.0 / s.v[576]))) as f64).powf(p.p153) * s.v[471]));
        }

        s.store_scalar(619, (1.0 + (((s.v[576]) as f64).powf(p.p229) * p.p230)));

        s.store_scalar(335, ((1.0 / (p.p118 + (0.5 * p.p0))) + (1.0 / (p.p119 + (0.5 * p.p0)))));

        s.store_scalar(589, (2.0 / s.v[335]));

        s.b[1248] = (((p.p8 > 0.0) && (p.p9 > 0.0)) && ((p.p7 == 1.0) || ((p.p7 > 1.0) && (p.p10 > 0.0))));
        s.store_scalar(1248, if s.b[1248] { 1.0 } else { 0.0 });

        if s.b[1248] {
            s.store_scalar(335, 0.0);
            s.store_scalar(721, 0.0);
        }

        let mut assign10780_loop_guard: usize = 0;
        while {
            let assign10780_cond_e5711: f64 = if (s.b[1248] && (s.v[721] < p.p7)) { 1.0 } else { 0.0 };
            assign10780_cond_e5711 != 0.0
        } {
            assign10780_loop_guard += 1;
            assert!(assign10780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[1248] {
                s.store_add_scaled_inputs3_mixed_iaa(335, 335, 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(721), (p.p10 + p.p0), (p.p8 + (0.5 * p.p0)))), 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(721), (p.p10 + p.p0), (p.p9 + (0.5 * p.p0)))), 1.0);
                s.store_offset(721, 721, 1.0);
            }
        }

        if s.b[1248] {
            s.store_div_from_scalar(588, (2.0 * p.p7), 335);
        }

        if (!s.b[1248]) {
            s.store_scalar(588, 0.0);
        }

        s.store_scalar(773, s.v[528]);

        s.store_scalar(620, s.v[476]);

        s.store_scalar(621, s.v[464]);

        s.store_scalar(622, s.v[463]);

        s.b[1249] = ((p.p32 == 1.0) && s.b[623]);
        s.store_scalar(1249, if s.b[1249] { 1.0 } else { 0.0 });

        if s.b[1249] {
            s.store_scalar(620, (s.v[620] * ((p.p282 * (((s.v[571]) as f64).ln() - ((s.v[622]) as f64).ln())) + 1.0)));
            s.store_scalar(621, ((s.v[621] + s.v[571]) - s.v[622]));
            s.store_scalar(773, ((s.v[773] + s.v[571]) - s.v[622]));
            s.store_scalar(622, s.v[571]);
        }

        s.store_scale(573, 620, ((1.0 + (p.p162 / ((s.v[580]) as f64).powf(p.p163))) * ((1.0 + (p.p164 / ((s.v[576]) as f64).powf(p.p165))) * (1.0 + (p.p167 / ((s.v[581]) as f64).powf(p.p168))))));

        s.b[1251] = (s.v[588] > 0.0);
        s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });

        if s.b[1251] {
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

        s.b[1259] = (s.v[335] < 1000000000000000.0);
        s.store_scalar(1259, if s.b[1259] { 1.0 } else { 0.0 });

        if s.b[1259] {
            s.store_scalar(335, 1000000000000000.0);
        }

        s.store_scale(622, 335, 1000000.0);

        s.b[1261] = (s.v[336] < 1000000000000000.0);
        s.store_scalar(1261, if s.b[1261] { 1.0 } else { 0.0 });

        if s.b[1261] {
            s.store_scalar(336, 1000000000000000.0);
        }

        s.store_scale(584, 336, 1000000.0);

        s.b[1262] = (s.v[588] > 0.0);
        s.store_scalar(1262, if s.b[1262] { 1.0 } else { 0.0 });

        if s.b[1262] {
            s.store_scalar(335, (1.0 / (1.0 + s.v[503])));
            s.store_powf_ad(336, A::div_from_scalar(s.v[502], s.ad_value(588)), s.v[504]);
            s.store_scalar(337, (((s.v[502] / s.v[589])) as f64).powf(s.v[504]));
            s.store_div_scaled_product_offset_denominator(585, s.ad_value(584), A::offset(A::mul(s.ad_value(335), s.ad_value(336)), 1.0), 1.0, A::mul(s.ad_value(335), s.ad_value(337)), 1.0, 1.0);
        }

        if (!s.b[1262]) {
            s.copy_ad(585, 584);
        }

        s.b[1263] = ((s.v[582] > p.p140) || (p.p140 <= 0.0));
        s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });

        if s.b[1263] {
            s.store_add_scaled_inputs(586, 622, ((s.v[582] - p.p140) * 1.0 / (s.v[582])), 585, (p.p140 * 1.0 / (s.v[582])));
        }

        if (!s.b[1263]) {
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

        s.b[1264] = ((s.v[582] <= (2.0 * p.p140)) && (p.p140 > 0.0));
        s.store_scalar(1264, if s.b[1264] { 1.0 } else { 0.0 });

        if s.b[1264] {
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
        if (!s.b[1264]) {
            s.store_scalar(638, 0.0);
        }

        s.store_scalar(639, (((((2.0 * 1.6021918e-19) * s.v[494]) * 1.034943e-10)) as f64).sqrt());

        s.store_scalar(640, (1.0 / (s.v[494] * s.v[494])));

        s.store_scalar(641, ((1.0 + (s.v[542] / ((s.v[576]) as f64).powf(p.p231))) * (1.0 + (p.p238 / ((s.v[581]) as f64).powf(p.p239)))));

        s.store_scaled_ln_scaled_input(158, 586, 1.0 / (1.04e16), (2.0 / 38.68283));

        s.store_scaled_ln_scaled_input(159, 622, 1.0 / (1.04e16), (2.0 / 38.68283));

        s.b[1265] = (p.p51 == 1.0);
        s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });

        if s.b[1265] {
            s.store_scalar(335, (p.p5 + (s.v[163] / (3.0 * p.p4))));
            s.store_scalar(336, (s.v[582] - p.p6));
        }

        s.b[1267] = (p.p130 > 0.0);
        s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });

        if s.b[1267] {
            s.store_scalar(644, (p.p130 * p.p2));
            s.store_scalar(648, (p.p130 * p.p3));
        }

        if (!s.b[1267]) {
            s.store_scalar(644, 0.0);
            s.store_scalar(648, 0.0);
        }

        s.b[1268] = (p.p131 > 0.0);
        s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });

        if s.b[1268] {
            s.store_scalar(648, (p.p131 * p.p3));
        }

        if (!s.b[1268]) {
            s.store_scalar(648, 0.0);
        }

        s.b[1269] = (s.v[449] == 0.0);
        s.store_scalar(1269, if s.b[1269] { 1.0 } else { 0.0 });

        s.b[1270] = ((s.v[530] > 0.0) || (s.v[540] > 0.0));
        s.store_scalar(1270, if s.b[1270] { 1.0 } else { 0.0 });

        if (s.b[1269] && s.b[1270]) {
            s.store_scalar(645, (1.0 + (p.p309 / ((s.v[581]) as f64).powf(p.p310))));
        }

        s.b[1271] = (s.v[538] != 0.0);
        s.store_scalar(1271, if s.b[1271] { 1.0 } else { 0.0 });

        if ((s.b[1269] && s.b[1270]) && s.b[1271]) {
            s.store_scalar(341, (1.0 + (p.p303 / ((s.v[581]) as f64).powf(p.p304))));
            s.store_scalar(340, ((-p.p301) * ((s.v[576]) as f64).powf(p.p302)));
        }

        s.b[1272] = (s.v[340] > 60.0);
        s.store_scalar(1272, if s.b[1272] { 1.0 } else { 0.0 });

        if (((s.b[1269] && s.b[1270]) && s.b[1271]) && s.b[1272]) {
            s.store_scalar(340, 60.0);
        }

        if ((s.b[1269] && s.b[1270]) && s.b[1271]) {
            s.store_exp(340, 340);
            s.store_mul(646, 340, 341);
        }

        if ((s.b[1269] && s.b[1270]) && (!s.b[1271])) {
            s.store_scalar(646, 0.0);
        }

        if (s.b[1269] && (!s.b[1270])) {
            s.store_scalar(645, 0.0);
            s.store_scalar(646, 0.0);
        }

        s.b[1273] = (s.v[532] != 0.0);
        s.store_scalar(1273, if s.b[1273] { 1.0 } else { 0.0 });

        if (s.b[1269] && s.b[1273]) {
            s.store_scalar(336, (1.0 + (p.p307 / ((s.v[581]) as f64).powf(p.p308))));
            s.store_scalar(335, ((-p.p305) * ((s.v[576]) as f64).powf(p.p306)));
        }

        s.b[1274] = (s.v[335] > 60.0);
        s.store_scalar(1274, if s.b[1274] { 1.0 } else { 0.0 });

        if ((s.b[1269] && s.b[1273]) && s.b[1274]) {
            s.store_scalar(335, 60.0);
        }

        if (s.b[1269] && s.b[1273]) {
            s.store_exp(335, 335);
            s.store_scaled_mul(337, 336, 335, s.v[532]);
            s.store_scaled_add_sqrt_square_offset_rhs(647, 337, 337, ((((4.0 * 1e-6) / 100.0) * 1e-6) / 100.0), 0.5);
        }

        if (s.b[1269] && (!s.b[1273])) {
            s.store_scalar(647, 0.0);
        }

        if s.b[1269] {
            s.store_scalar(649, 0.0);
            s.store_scalar(614, 0.0);
            s.store_scalar(786, 0.0);
            s.store_scalar(652, 0.0);
            s.store_scalar(653, 0.0);
            s.store_scalar(654, 0.0);
        }

        if (!s.b[1269]) {
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

        s.b[1275] = (s.v[459] > 0.0);
        s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });

        if s.b[1275] {
            s.store_scalar(650, ((2.0 * 1.034943e-10) / (1.6021918e-19 * s.v[459])));
            s.store_div_scaled_value_offset_denominator(651, s.ad_value(622), (((2.0 * 1.034943e-10) / 1.6021918e-19) * 1.0 / (s.v[459])), s.ad_value(622), s.v[459], 1.0);
        }

        if (!s.b[1275]) {
            s.store_scalar(650, 0.0);
            s.store_scalar(651, 0.0);
        }

        s.b[1280] = (p.p44 == 0.0);
        s.store_scalar(1280, if s.b[1280] { 1.0 } else { 0.0 });

        if s.b[1280] {
            s.store_scalar(335, ((p.p108 * s.v[576]) + p.p109));
        }

        s.b[1281] = (s.v[335] < 0.0);
        s.store_scalar(1281, if s.b[1281] { 1.0 } else { 0.0 });

        if (s.b[1280] && s.b[1281]) {
            s.store_scalar(335, 0.0);
        }

        if s.b[1280] {
            s.store_offset_ad(658, A::div_scaled_value_offset_denominator(s.ad_value(335), p.p107, s.ad_value(335), p.p107, 1.0), 1.0);
        }

        if (!s.b[1280]) {
            s.store_scalar(335, (p.p108 * s.v[576]));
        }

        s.b[1282] = (s.v[335] < 0.0);
        s.store_scalar(1282, if s.b[1282] { 1.0 } else { 0.0 });

        if ((!s.b[1280]) && s.b[1282]) {
            s.store_scalar(335, 0.0);
        }

        if (!s.b[1280]) {
            s.store_offset_ad(658, A::div_scaled_value_offset_denominator(s.ad_value(335), p.p107, s.ad_value(335), p.p107, 1.0), ((p.p109) + (1e-25)));
        }

        s.b[1284] = (s.v[658] < 0.1);
        s.store_scalar(1284, if s.b[1284] { 1.0 } else { 0.0 });

        if s.b[1284] {
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

        s.b[1285] = ((p.p53 == 0.0) || (s.v[541] == 0.0));
        s.store_scalar(1285, if s.b[1285] { 1.0 } else { 0.0 });

        if s.b[1285] {
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

        s.b[1286] = ((s.v[963] != 0.0) && (s.v[963] < 3.0));
        s.store_scalar(1286, if s.b[1286] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1286]) {
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
        s.b[1288] = (s.v[973] < 1000.0);
        s.store_scalar(1288, if s.b[1288] { 1.0 } else { 0.0 });

        if ((s.b[1285] && s.b[1286]) && s.b[1288]) {
            s.store_scalar(973, 1000.0);
        }

        if (s.b[1285] && s.b[1286]) {
            s.store_div_ad_rhs(966, 966, A::powf(s.ad_value(676), p.p381));
            s.store_div_from_scalar_powf_ad(970, s.v[970], s.ad_value(676), p.p382);
        }

        s.b[1289] = (s.v[963] == 3.0);
        s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });

        if ((s.b[1285] && (!s.b[1286])) && s.b[1289]) {
            s.store_sqrt_mul_scaled_lhs(209, 964, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div_scaled_product_by_product(210, s.ad_value(394), s.ad_value(394), 1.0, s.ad_value(964), s.ad_value(964), 1.0);
            s.store_mul_scaled_ln_ad_rhs(961, 155, 2.0, A::div(s.ad_value(964), s.ad_value(394)));
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_product_by_product(s.ad_value(964), s.ad_value(622), 1.0, s.ad_value(394), s.ad_value(394), 1.0));
            s.store_exp_scaled_input(335, 590, p.p380);
            s.store_div(977, 335, 971);
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p.p379), p.p379));
            s.store_div(973, 973, 334);
        }

        s.b[1291] = (s.v[973] < 1000.0);
        s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });

        if (((s.b[1285] && (!s.b[1286])) && s.b[1289]) && s.b[1291]) {
            s.store_scalar(973, 1000.0);
        }

        if ((s.b[1285] && (!s.b[1286])) && s.b[1289]) {
            s.store_div_ad_rhs(966, 966, A::powf(s.ad_value(676), p.p381));
            s.store_offset_scaled(976, 676, p.p365, (((((-1.0)) * (p.p365))) + (p.p364)));
        }

        if ((s.b[1285] && (!s.b[1286])) && (!s.b[1289])) {
            s.store_scalar(961, 0.0);
            s.store_mul_ln_ad_rhs(960, 155, A::div_scaled_value_by_product(s.ad_value(586), s.v[489], s.ad_value(394), s.ad_value(394), 1.0));
            s.store_scalar(977, 0.0);
        }

        if s.b[1285] {
            s.store_mul(680, 638, 155);
            s.store_scale(335, 387, 1.0 / (s.v[764]));
            s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(335), 0.4, 1.8), 1.0, s.ad_value(335), s.ad_value(335), 0.1), A::scale_offset(s.ad_value(335), (-s.v[477]), s.v[477]));
        }

        s.b[1292] = (p.p39 != 2.0);
        s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1292]) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(389), p.p90, 1.0), 1.0, s.ad_value(390), p.p91));
        }

        if (s.b[1285] && (!s.b[1292])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(681, (s.v[641] * s.v[454]), 334, A::add_scaled_inputs(A::scale_offset(s.ad_value(391), p.p90, 1.0), 1.0, s.ad_value(392), p.p91));
        }

        s.b[1294] = (p.p39 != 2.0);
        s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1294]) {
            s.store_add_scaled_inputs_ad_lhs(682, A::scale_offset(s.ad_value(389), p.p324, 1.0), s.v[627], 390, (p.p325 * s.v[627]));
            s.store_add_scaled_ad_lhs(335, A::scale_offset(s.ad_value(389), p.p390, 1.0), 390, p.p391);
            s.store_scale(688, 335, s.v[633]);
            s.store_scale(689, 335, s.v[634]);
        }

        if (s.b[1285] && (!s.b[1294])) {
            s.store_add_scaled_inputs_ad_lhs(682, A::scale_offset(s.ad_value(391), p.p324, 1.0), s.v[627], 392, (p.p325 * s.v[627]));
            s.store_add_scaled_ad_lhs(335, A::scale_offset(s.ad_value(391), p.p390, 1.0), 392, p.p391);
            s.store_scale(688, 335, s.v[633]);
            s.store_scale(689, 335, s.v[634]);
        }

        s.b[1296] = (s.v[682] < 0.0);
        s.store_scalar(1296, if s.b[1296] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1296]) {
            s.store_scalar(682, 0.0);
        }

        s.b[1298] = (s.v[688] < 0.0);
        s.store_scalar(1298, if s.b[1298] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1298]) {
            s.store_scalar(688, 0.0);
        }

        s.b[1300] = (s.v[689] < 0.0);
        s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1300]) {
            s.store_scalar(689, 0.0);
        }

        if (s.b[1285] && (p.p53 != 0.0)) {
            s.store_add_scaled_inputs_ad_lhs(766, A::scale_offset(s.ad_value(389), p.p328, s.v[541]), s.v[675], 390, (p.p329 * s.v[675]));
        }

        s.b[1302] = (s.v[766] < 0.0001);
        s.store_scalar(1302, if s.b[1302] { 1.0 } else { 0.0 });

        if ((s.b[1285] && (p.p53 != 0.0)) && s.b[1302]) {
            s.store_scalar(766, 0.0001);
        }

        if s.b[1285] {
            s.store_add_scaled_ad_lhs(336, A::scale_offset(s.ad_value(389), p.p330, s.v[529]), 390, p.p331);
            s.store_offset(781, 336, (-0.05));
            s.store_scalar(782, 0.0);
        }

        if s.b[1285] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1285] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_scaled_add(336, 781, 782, 0.5);
            s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(336), (-0.05));
            s.store_scalar(782, (4.0 * 0.05));
        }

        if s.b[1285] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if s.b[1285] {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(683, 781, (-0.5), 782, (-0.5), 1.0);
            s.store_mul_scaled_ln_ad_rhs(157, 155, 2.0, A::div(s.ad_value(586), s.ad_value(394)));
            s.store_scalar(335, ((2.0 * 1.034943e-10) / 1.6021918e-19));
            s.store_sqrt_div(684, 335, 586);
            s.store_sqrt_div(685, 335, 621);
        }

        s.b[1303] = (s.v[963] == 0.0);
        s.store_scalar(1303, if s.b[1303] { 1.0 } else { 0.0 });

        if (s.b[1285] && s.b[1303]) {
            s.store_sqrt_mul_scaled_lhs(209, 586, ((2.0 * 1.034943e-10) * 1.6021918e-19), 155);
            s.store_div(335, 394, 586);
            s.store_square(210, 335);
        }

        s.b[1304] = (s.v[963] == 0.0);
        s.store_scalar(1304, if s.b[1304] { 1.0 } else { 0.0 });

        s.b[1305] = (s.v[459] != 0.0);
        s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });

        if ((s.b[1285] && s.b[1304]) && s.b[1305]) {
            s.store_mul_sqrt_ad_rhs(686, 209, A::div_from_scalar(s.v[459], s.ad_value(586)));
        }

        s.b[1306] = (s.v[460] != 0.0);
        s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });

        if ((s.b[1285] && s.b[1304]) && s.b[1306]) {
            s.store_mul_sqrt_ad_rhs(687, 209, A::div_from_scalar(s.v[460], s.ad_value(586)));
        }

        s.b[1307] = (s.v[459] != 0.0);
        s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });

        if ((s.b[1285] && (!s.b[1304])) && s.b[1307]) {
            s.store_mul_sqrt_ad_rhs(686, 209, A::div_from_scalar(s.v[459], s.ad_value(964)));
        }

        s.b[1308] = (s.v[460] != 0.0);
        s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });

        if ((s.b[1285] && (!s.b[1304])) && s.b[1308]) {
            s.store_mul_sqrt_ad_rhs(687, 209, A::div_from_scalar(s.v[460], s.ad_value(964)));
        }

        s.b[1309] = (s.v[449] == 0.0);
        s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });

        s.b[1310] = (s.v[530] > 0.0);
        s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });

        if ((s.b[1285] && s.b[1309]) && s.b[1310]) {
            s.store_scale(336, 645, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));
        }

        s.b[1311] = (p.p39 == 1.0);
        s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });

        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && s.b[1311]) {
            s.store_mul_add_scaled_inputs_rhs(690, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[530]), 1.0, s.ad_value(390), s.v[556]);
            s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && s.b[1311]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && s.b[1311]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && (!s.b[1311])) {
            s.store_mul_add_scaled_inputs_rhs(690, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[530]), 1.0, s.ad_value(392), s.v[556]);
            s.store_offset(781, 690, (((-(0.005 * s.v[530]))) + ((-(0.01 * s.v[530])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[530])) * (0.01 * s.v[530])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && (!s.b[1311])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1310]) && (!s.b[1311])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(690, 781, 0.5, 782, 0.5, (0.005 * s.v[530]));
        }

        if ((s.b[1285] && s.b[1309]) && (!s.b[1310])) {
            s.store_scalar(690, 0.0);
        }

        s.b[1312] = (s.v[540] > 0.0);
        s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });

        if ((s.b[1285] && s.b[1309]) && s.b[1312]) {
            s.store_scale(336, 645, ((((p.p69 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p70 * p.p100) * 1000000.0) + p.p101)));
        }

        s.b[1313] = (p.p39 == 1.0);
        s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });

        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && s.b[1313]) {
            s.store_mul_add_scaled_inputs_rhs(691, 336, A::scale_offset(s.ad_value(389), s.v[555], s.v[540]), 1.0, s.ad_value(390), s.v[556]);
            s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && s.b[1313]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && s.b[1313]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && (!s.b[1313])) {
            s.store_mul_add_scaled_inputs_rhs(691, 336, A::scale_offset(s.ad_value(391), s.v[555], s.v[540]), 1.0, s.ad_value(392), s.v[556]);
            s.store_offset(781, 691, (((-(0.005 * s.v[540]))) + ((-(0.01 * s.v[540])))));
            s.store_scalar(782, ((4.0 * (0.005 * s.v[540])) * (0.01 * s.v[540])));
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && (!s.b[1313])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1285] && s.b[1309]) && s.b[1312]) && (!s.b[1313])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(691, 781, 0.5, 782, 0.5, (0.005 * s.v[540]));
        }

        if ((s.b[1285] && s.b[1309]) && (!s.b[1312])) {
            s.store_scalar(691, 0.0);
        }

        s.b[1314] = (s.v[538] > 0.0);
        s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_scale(338, 646, ((((p.p67 * s.v[536]) * 1000000.0) + s.v[534]) * (((p.p68 * p.p100) * 1000000.0) + p.p101)));
            s.store_scalar(335, (((1.0 - s.v[535]) * p.p63) * 1000000.0));
            s.store_scalar(782, ((((p.p99 * p.p99) + ((4.0 * (0.0001 * 0.01)) * (0.0001 * 0.01)))) as f64).sqrt());
            s.store_scaled_offset_ad(334, A::div_from_scalar(p.p99, s.ad_value(782)), 1.0, 0.5);
            s.store_scaled_offset(336, 782, p.p99, 0.5);
        }

        s.b[1315] = (s.v[336] < 0.0);
        s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });

        if (((s.b[1285] && s.b[1309]) && s.b[1314]) && s.b[1315]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1285] && s.b[1309]) && s.b[1314]) {
            s.store_div_from_scalar(342, (-p.p98), 336);
            s.store_offset_scaled(337, 342, (p.p63 * 1000000.0), ((1.0) + (p.p98)));
            s.store_offset_add_scaled_product(781, s.ad_value(338), (-1.0), s.ad_value(337), s.ad_value(338), 1.0, (-0.01));
            s.store_scale(782, 338, (4.0 * 0.01));
        }

    }
}
