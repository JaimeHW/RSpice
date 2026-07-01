#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_57(
        var_deltemp: f64,
        var_deltemp_dn4: f64,
        var_devtemp: f64,
        var_devtemp_dn4: f64,
        var_guard244: f64,
        var_guard259: f64,
        var_guard300: f64,
        var_guard301: f64,
        var_guard302: f64,
        var_guard304: f64,
        var_prt1_i: f64,
        var_prt_i: f64,
        var_sprt_i: f64,
        var_tnom: f64,
        var_tr0_i: f64,
        var_guard305_slot: &mut f64,
        var_guard305_rv_slot: &mut f64,
        var_guard306_slot: &mut f64,
        var_guard306_rv_slot: &mut f64,
        var_rdstemp0_slot: &mut f64,
        var_rdstemp0_dn4_slot: &mut f64,
        var_rdstemp0_rv_slot: &mut f64,
        var_rdstemp1_slot: &mut f64,
        var_rdstemp1_dn4_slot: &mut f64,
        var_rdstemp1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn13_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
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
        var_t3_dn11_slot: &mut f64,
        var_t3_dn13_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
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
        var_t4_dn11_slot: &mut f64,
        var_t4_dn13_slot: &mut f64,
        var_t4_dn14_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
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
        var_t5_dn11_slot: &mut f64,
        var_t5_dn13_slot: &mut f64,
        var_t5_dn14_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
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
        var_t6_dn11_slot: &mut f64,
        var_t6_dn13_slot: &mut f64,
        var_t6_dn14_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
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
        var_t7_dn11_slot: &mut f64,
        var_t7_dn13_slot: &mut f64,
        var_t7_dn14_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn3_slot: &mut f64,
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
        var_t8_dn11_slot: &mut f64,
        var_t8_dn13_slot: &mut f64,
        var_t8_dn14_slot: &mut f64,
        var_t8_dn2_slot: &mut f64,
        var_t8_dn3_slot: &mut f64,
        var_t8_dn4_slot: &mut f64,
        var_t8_dn5_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn7_slot: &mut f64,
        var_t8_dn8_slot: &mut f64,
        var_t8_dn9_slot: &mut f64,
        var_t8_rv_slot: &mut f64,
    ) {
        let mut var_guard305: f64 = *var_guard305_slot;
        let mut var_guard305_rv: f64 = *var_guard305_rv_slot;
        let mut var_guard306: f64 = *var_guard306_slot;
        let mut var_guard306_rv: f64 = *var_guard306_rv_slot;
        let mut var_rdstemp0: f64 = *var_rdstemp0_slot;
        let mut var_rdstemp0_dn4: f64 = *var_rdstemp0_dn4_slot;
        let mut var_rdstemp0_rv: f64 = *var_rdstemp0_rv_slot;
        let mut var_rdstemp1: f64 = *var_rdstemp1_slot;
        let mut var_rdstemp1_dn4: f64 = *var_rdstemp1_dn4_slot;
        let mut var_rdstemp1_rv: f64 = *var_rdstemp1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
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
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
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
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn13: f64 = *var_t4_dn13_slot;
        let mut var_t4_dn14: f64 = *var_t4_dn14_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
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
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn13: f64 = *var_t5_dn13_slot;
        let mut var_t5_dn14: f64 = *var_t5_dn14_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
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
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn13: f64 = *var_t6_dn13_slot;
        let mut var_t6_dn14: f64 = *var_t6_dn14_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
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
        let mut var_t7_dn11: f64 = *var_t7_dn11_slot;
        let mut var_t7_dn13: f64 = *var_t7_dn13_slot;
        let mut var_t7_dn14: f64 = *var_t7_dn14_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn3: f64 = *var_t7_dn3_slot;
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
        let mut var_t8_dn11: f64 = *var_t8_dn11_slot;
        let mut var_t8_dn13: f64 = *var_t8_dn13_slot;
        let mut var_t8_dn14: f64 = *var_t8_dn14_slot;
        let mut var_t8_dn2: f64 = *var_t8_dn2_slot;
        let mut var_t8_dn3: f64 = *var_t8_dn3_slot;
        let mut var_t8_dn4: f64 = *var_t8_dn4_slot;
        let mut var_t8_dn5: f64 = *var_t8_dn5_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn7: f64 = *var_t8_dn7_slot;
        let mut var_t8_dn8: f64 = *var_t8_dn8_slot;
        let mut var_t8_dn9: f64 = *var_t8_dn9_slot;
        let mut var_t8_rv: f64 = *var_t8_rv_slot;

        let (assign17010_e29473, assign17010_e29473_d_n0, assign17010_e29473_d_n2, assign17010_e29473_d_n3, assign17010_e29473_d_n4, assign17010_e29473_d_n5, assign17010_e29473_d_n6, assign17010_e29473_d_n7, assign17010_e29473_d_n8, assign17010_e29473_d_n9, assign17010_e29473_d_n10, assign17010_e29473_d_n11, assign17010_e29473_d_n13, assign17010_e29473_d_n14,) = {
    if ((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 != 0.0)) {
        let assign17010_e29469: f64 = (210.0 - var_tnom);
        let assign17010_e29470: f64 = (var_prt_i * assign17010_e29469);
        let assign17010_e29471: f64 = (1.0 + assign17010_e29470);
        (assign17010_e29471, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11, var_t4_dn13, var_t4_dn14,)
    }
};
        var_t4 = assign17010_e29473;
        var_t4_dn0 = assign17010_e29473_d_n0;
        var_t4_dn2 = assign17010_e29473_d_n2;
        var_t4_dn3 = assign17010_e29473_d_n3;
        var_t4_dn4 = assign17010_e29473_d_n4;
        var_t4_dn5 = assign17010_e29473_d_n5;
        var_t4_dn6 = assign17010_e29473_d_n6;
        var_t4_dn7 = assign17010_e29473_d_n7;
        var_t4_dn8 = assign17010_e29473_d_n8;
        var_t4_dn9 = assign17010_e29473_d_n9;
        var_t4_dn10 = assign17010_e29473_d_n10;
        var_t4_dn11 = assign17010_e29473_d_n11;
        var_t4_dn13 = assign17010_e29473_d_n13;
        var_t4_dn14 = assign17010_e29473_d_n14;
        var_t4_rv = 0.0;

        let (assign17020_e29503, assign17020_e29503_d_n0, assign17020_e29503_d_n2, assign17020_e29503_d_n3, assign17020_e29503_d_n4, assign17020_e29503_d_n5, assign17020_e29503_d_n6, assign17020_e29503_d_n7, assign17020_e29503_d_n8, assign17020_e29503_d_n9, assign17020_e29503_d_n10, assign17020_e29503_d_n11, assign17020_e29503_d_n13, assign17020_e29503_d_n14,) = {
    if ((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 != 0.0)) {
        let assign17020_e29493: f64 = (210.0 - var_tr0_i);
        let assign17020_e29494: f64 = (var_prt1_i * assign17020_e29493);
        let assign17020_e29495: f64 = (1.0 + assign17020_e29494);
        let assign17020_e29499: f64 = (var_tr0_i - var_tnom);
        let assign17020_e29500: f64 = (var_prt_i * assign17020_e29499);
        let assign17020_e29501: f64 = (assign17020_e29495 + assign17020_e29500);
        (assign17020_e29501, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11, var_t5_dn13, var_t5_dn14,)
    }
};
        var_t5 = assign17020_e29503;
        var_t5_dn0 = assign17020_e29503_d_n0;
        var_t5_dn2 = assign17020_e29503_d_n2;
        var_t5_dn3 = assign17020_e29503_d_n3;
        var_t5_dn4 = assign17020_e29503_d_n4;
        var_t5_dn5 = assign17020_e29503_d_n5;
        var_t5_dn6 = assign17020_e29503_d_n6;
        var_t5_dn7 = assign17020_e29503_d_n7;
        var_t5_dn8 = assign17020_e29503_d_n8;
        var_t5_dn9 = assign17020_e29503_d_n9;
        var_t5_dn10 = assign17020_e29503_d_n10;
        var_t5_dn11 = assign17020_e29503_d_n11;
        var_t5_dn13 = assign17020_e29503_d_n13;
        var_t5_dn14 = assign17020_e29503_d_n14;
        var_t5_rv = 0.0;

        let assign17030_e29506: f64 = if var_prt1_i < var_prt_i { 1.0 } else { 0.0 };
        var_guard305 = assign17030_e29506;
        var_guard305_rv = 0.0;

        let (assign17040_e29566, assign17040_e29566_d_n0, assign17040_e29566_d_n2, assign17040_e29566_d_n3, assign17040_e29566_d_n4, assign17040_e29566_d_n5, assign17040_e29566_d_n6, assign17040_e29566_d_n7, assign17040_e29566_d_n8, assign17040_e29566_d_n9, assign17040_e29566_d_n10, assign17040_e29566_d_n11, assign17040_e29566_d_n13, assign17040_e29566_d_n14,) = {
    if (((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 != 0.0)) && (var_guard305 != 0.0)) {
        let assign17040_e29527: f64 = (var_rdstemp0 + var_rdstemp1);
        let assign17040_e29530: f64 = (var_rdstemp0 - var_rdstemp1);
        let assign17040_e29533: f64 = (var_rdstemp0 - var_rdstemp1);
        let assign17040_e29534: f64 = (assign17040_e29530 * assign17040_e29533);
        let assign17040_e29537: f64 = (0.25 * var_sprt_i);
        let assign17040_e29539: f64 = (assign17040_e29537 * var_sprt_i);
        let assign17040_e29540: f64 = (assign17040_e29534 + assign17040_e29539);
        let assign17040_e29541: f64 = (assign17040_e29540).sqrt();
        let assign17040_e29542: f64 = (assign17040_e29527 + assign17040_e29541);
        let assign17040_e29543: f64 = (0.5 * assign17040_e29542);
        let assign17040_e29547: f64 = var_t3;
        let assign17040_e29550: f64 = var_t3;
        let assign17040_e29553: f64 = var_t3;
        let assign17040_e29554: f64 = (assign17040_e29550 * assign17040_e29553);
        let assign17040_e29557: f64 = (0.25 * var_sprt_i);
        let assign17040_e29559: f64 = (assign17040_e29557 * var_sprt_i);
        let assign17040_e29560: f64 = (assign17040_e29554 + assign17040_e29559);
        let assign17040_e29561: f64 = (assign17040_e29560).sqrt();
        let assign17040_e29562: f64 = (assign17040_e29547 + assign17040_e29561);
        let assign17040_e29563: f64 = (0.5 * assign17040_e29562);
        let assign17040_e29564: f64 = (assign17040_e29543 - assign17040_e29563);
        (assign17040_e29564, (-(0.5 * (var_t3_dn0 + (((var_t3_dn0 * assign17040_e29553) + (assign17040_e29550 * var_t3_dn0)) / (2.0 * assign17040_e29561))))), (-(0.5 * (var_t3_dn2 + (((var_t3_dn2 * assign17040_e29553) + (assign17040_e29550 * var_t3_dn2)) / (2.0 * assign17040_e29561))))), (-(0.5 * (var_t3_dn3 + (((var_t3_dn3 * assign17040_e29553) + (assign17040_e29550 * var_t3_dn3)) / (2.0 * assign17040_e29561))))), ((0.5 * ((var_rdstemp0_dn4 + var_rdstemp1_dn4) + ((((var_rdstemp0_dn4 - var_rdstemp1_dn4) * assign17040_e29533) + (assign17040_e29530 * (var_rdstemp0_dn4 - var_rdstemp1_dn4))) / (2.0 * assign17040_e29541)))) - (0.5 * (var_t3_dn4 + (((var_t3_dn4 * assign17040_e29553) + (assign17040_e29550 * var_t3_dn4)) / (2.0 * assign17040_e29561))))), (-(0.5 * (var_t3_dn5 + (((var_t3_dn5 * assign17040_e29553) + (assign17040_e29550 * var_t3_dn5)) / (2.0 * assign17040_e29561))))), (-(0.5 * (var_t3_dn6 + (((var_t3_dn6 * assign17040_e29553) + (assign17040_e29550 * var_t3_dn6)) / (2.0 * assign17040_e29561))))), (-(0.5 * (var_t3_dn7 + (((var_t3_dn7 * assign17040_e29553) + (assign17040_e29550 * var_t3_dn7)) / (2.0 * assign17040_e29561))))), (-(0.5 * (var_t3_dn8 + (((var_t3_dn8 * assign17040_e29553) + (assign17040_e29550 * var_t3_dn8)) / (2.0 * assign17040_e29561))))), (-(0.5 * (var_t3_dn9 + (((var_t3_dn9 * assign17040_e29553) + (assign17040_e29550 * var_t3_dn9)) / (2.0 * assign17040_e29561))))), (-(0.5 * (var_t3_dn10 + (((var_t3_dn10 * assign17040_e29553) + (assign17040_e29550 * var_t3_dn10)) / (2.0 * assign17040_e29561))))), (-(0.5 * (var_t3_dn11 + (((var_t3_dn11 * assign17040_e29553) + (assign17040_e29550 * var_t3_dn11)) / (2.0 * assign17040_e29561))))), (-(0.5 * (var_t3_dn13 + (((var_t3_dn13 * assign17040_e29553) + (assign17040_e29550 * var_t3_dn13)) / (2.0 * assign17040_e29561))))), (-(0.5 * (var_t3_dn14 + (((var_t3_dn14 * assign17040_e29553) + (assign17040_e29550 * var_t3_dn14)) / (2.0 * assign17040_e29561))))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn13, var_t6_dn14,)
    }
};
        var_t6 = assign17040_e29566;
        var_t6_dn0 = assign17040_e29566_d_n0;
        var_t6_dn2 = assign17040_e29566_d_n2;
        var_t6_dn3 = assign17040_e29566_d_n3;
        var_t6_dn4 = assign17040_e29566_d_n4;
        var_t6_dn5 = assign17040_e29566_d_n5;
        var_t6_dn6 = assign17040_e29566_d_n6;
        var_t6_dn7 = assign17040_e29566_d_n7;
        var_t6_dn8 = assign17040_e29566_d_n8;
        var_t6_dn9 = assign17040_e29566_d_n9;
        var_t6_dn10 = assign17040_e29566_d_n10;
        var_t6_dn11 = assign17040_e29566_d_n11;
        var_t6_dn13 = assign17040_e29566_d_n13;
        var_t6_dn14 = assign17040_e29566_d_n14;
        var_t6_rv = 0.0;

        let (assign17050_e29626, assign17050_e29626_d_n0, assign17050_e29626_d_n2, assign17050_e29626_d_n3, assign17050_e29626_d_n4, assign17050_e29626_d_n5, assign17050_e29626_d_n6, assign17050_e29626_d_n7, assign17050_e29626_d_n8, assign17050_e29626_d_n9, assign17050_e29626_d_n10, assign17050_e29626_d_n11, assign17050_e29626_d_n13, assign17050_e29626_d_n14,) = {
    if (((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 != 0.0)) && (var_guard305 != 0.0)) {
        let assign17050_e29587: f64 = (var_t4 + var_t5);
        let assign17050_e29590: f64 = (var_t4 - var_t5);
        let assign17050_e29593: f64 = (var_t4 - var_t5);
        let assign17050_e29594: f64 = (assign17050_e29590 * assign17050_e29593);
        let assign17050_e29597: f64 = (0.25 * var_sprt_i);
        let assign17050_e29599: f64 = (assign17050_e29597 * var_sprt_i);
        let assign17050_e29600: f64 = (assign17050_e29594 + assign17050_e29599);
        let assign17050_e29601: f64 = (assign17050_e29600).sqrt();
        let assign17050_e29602: f64 = (assign17050_e29587 + assign17050_e29601);
        let assign17050_e29603: f64 = (0.5 * assign17050_e29602);
        let assign17050_e29607: f64 = var_t3;
        let assign17050_e29610: f64 = var_t3;
        let assign17050_e29613: f64 = var_t3;
        let assign17050_e29614: f64 = (assign17050_e29610 * assign17050_e29613);
        let assign17050_e29617: f64 = (0.25 * var_sprt_i);
        let assign17050_e29619: f64 = (assign17050_e29617 * var_sprt_i);
        let assign17050_e29620: f64 = (assign17050_e29614 + assign17050_e29619);
        let assign17050_e29621: f64 = (assign17050_e29620).sqrt();
        let assign17050_e29622: f64 = (assign17050_e29607 + assign17050_e29621);
        let assign17050_e29623: f64 = (0.5 * assign17050_e29622);
        let assign17050_e29624: f64 = (assign17050_e29603 - assign17050_e29623);
        (assign17050_e29624, ((0.5 * ((var_t4_dn0 + var_t5_dn0) + ((((var_t4_dn0 - var_t5_dn0) * assign17050_e29593) + (assign17050_e29590 * (var_t4_dn0 - var_t5_dn0))) / (2.0 * assign17050_e29601)))) - (0.5 * (var_t3_dn0 + (((var_t3_dn0 * assign17050_e29613) + (assign17050_e29610 * var_t3_dn0)) / (2.0 * assign17050_e29621))))), ((0.5 * ((var_t4_dn2 + var_t5_dn2) + ((((var_t4_dn2 - var_t5_dn2) * assign17050_e29593) + (assign17050_e29590 * (var_t4_dn2 - var_t5_dn2))) / (2.0 * assign17050_e29601)))) - (0.5 * (var_t3_dn2 + (((var_t3_dn2 * assign17050_e29613) + (assign17050_e29610 * var_t3_dn2)) / (2.0 * assign17050_e29621))))), ((0.5 * ((var_t4_dn3 + var_t5_dn3) + ((((var_t4_dn3 - var_t5_dn3) * assign17050_e29593) + (assign17050_e29590 * (var_t4_dn3 - var_t5_dn3))) / (2.0 * assign17050_e29601)))) - (0.5 * (var_t3_dn3 + (((var_t3_dn3 * assign17050_e29613) + (assign17050_e29610 * var_t3_dn3)) / (2.0 * assign17050_e29621))))), ((0.5 * ((var_t4_dn4 + var_t5_dn4) + ((((var_t4_dn4 - var_t5_dn4) * assign17050_e29593) + (assign17050_e29590 * (var_t4_dn4 - var_t5_dn4))) / (2.0 * assign17050_e29601)))) - (0.5 * (var_t3_dn4 + (((var_t3_dn4 * assign17050_e29613) + (assign17050_e29610 * var_t3_dn4)) / (2.0 * assign17050_e29621))))), ((0.5 * ((var_t4_dn5 + var_t5_dn5) + ((((var_t4_dn5 - var_t5_dn5) * assign17050_e29593) + (assign17050_e29590 * (var_t4_dn5 - var_t5_dn5))) / (2.0 * assign17050_e29601)))) - (0.5 * (var_t3_dn5 + (((var_t3_dn5 * assign17050_e29613) + (assign17050_e29610 * var_t3_dn5)) / (2.0 * assign17050_e29621))))), ((0.5 * ((var_t4_dn6 + var_t5_dn6) + ((((var_t4_dn6 - var_t5_dn6) * assign17050_e29593) + (assign17050_e29590 * (var_t4_dn6 - var_t5_dn6))) / (2.0 * assign17050_e29601)))) - (0.5 * (var_t3_dn6 + (((var_t3_dn6 * assign17050_e29613) + (assign17050_e29610 * var_t3_dn6)) / (2.0 * assign17050_e29621))))), ((0.5 * ((var_t4_dn7 + var_t5_dn7) + ((((var_t4_dn7 - var_t5_dn7) * assign17050_e29593) + (assign17050_e29590 * (var_t4_dn7 - var_t5_dn7))) / (2.0 * assign17050_e29601)))) - (0.5 * (var_t3_dn7 + (((var_t3_dn7 * assign17050_e29613) + (assign17050_e29610 * var_t3_dn7)) / (2.0 * assign17050_e29621))))), ((0.5 * ((var_t4_dn8 + var_t5_dn8) + ((((var_t4_dn8 - var_t5_dn8) * assign17050_e29593) + (assign17050_e29590 * (var_t4_dn8 - var_t5_dn8))) / (2.0 * assign17050_e29601)))) - (0.5 * (var_t3_dn8 + (((var_t3_dn8 * assign17050_e29613) + (assign17050_e29610 * var_t3_dn8)) / (2.0 * assign17050_e29621))))), ((0.5 * ((var_t4_dn9 + var_t5_dn9) + ((((var_t4_dn9 - var_t5_dn9) * assign17050_e29593) + (assign17050_e29590 * (var_t4_dn9 - var_t5_dn9))) / (2.0 * assign17050_e29601)))) - (0.5 * (var_t3_dn9 + (((var_t3_dn9 * assign17050_e29613) + (assign17050_e29610 * var_t3_dn9)) / (2.0 * assign17050_e29621))))), ((0.5 * ((var_t4_dn10 + var_t5_dn10) + ((((var_t4_dn10 - var_t5_dn10) * assign17050_e29593) + (assign17050_e29590 * (var_t4_dn10 - var_t5_dn10))) / (2.0 * assign17050_e29601)))) - (0.5 * (var_t3_dn10 + (((var_t3_dn10 * assign17050_e29613) + (assign17050_e29610 * var_t3_dn10)) / (2.0 * assign17050_e29621))))), ((0.5 * ((var_t4_dn11 + var_t5_dn11) + ((((var_t4_dn11 - var_t5_dn11) * assign17050_e29593) + (assign17050_e29590 * (var_t4_dn11 - var_t5_dn11))) / (2.0 * assign17050_e29601)))) - (0.5 * (var_t3_dn11 + (((var_t3_dn11 * assign17050_e29613) + (assign17050_e29610 * var_t3_dn11)) / (2.0 * assign17050_e29621))))), ((0.5 * ((var_t4_dn13 + var_t5_dn13) + ((((var_t4_dn13 - var_t5_dn13) * assign17050_e29593) + (assign17050_e29590 * (var_t4_dn13 - var_t5_dn13))) / (2.0 * assign17050_e29601)))) - (0.5 * (var_t3_dn13 + (((var_t3_dn13 * assign17050_e29613) + (assign17050_e29610 * var_t3_dn13)) / (2.0 * assign17050_e29621))))), ((0.5 * ((var_t4_dn14 + var_t5_dn14) + ((((var_t4_dn14 - var_t5_dn14) * assign17050_e29593) + (assign17050_e29590 * (var_t4_dn14 - var_t5_dn14))) / (2.0 * assign17050_e29601)))) - (0.5 * (var_t3_dn14 + (((var_t3_dn14 * assign17050_e29613) + (assign17050_e29610 * var_t3_dn14)) / (2.0 * assign17050_e29621))))),)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn3, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn11, var_t7_dn13, var_t7_dn14,)
    }
};
        var_t7 = assign17050_e29626;
        var_t7_dn0 = assign17050_e29626_d_n0;
        var_t7_dn2 = assign17050_e29626_d_n2;
        var_t7_dn3 = assign17050_e29626_d_n3;
        var_t7_dn4 = assign17050_e29626_d_n4;
        var_t7_dn5 = assign17050_e29626_d_n5;
        var_t7_dn6 = assign17050_e29626_d_n6;
        var_t7_dn7 = assign17050_e29626_d_n7;
        var_t7_dn8 = assign17050_e29626_d_n8;
        var_t7_dn9 = assign17050_e29626_d_n9;
        var_t7_dn10 = assign17050_e29626_d_n10;
        var_t7_dn11 = assign17050_e29626_d_n11;
        var_t7_dn13 = assign17050_e29626_d_n13;
        var_t7_dn14 = assign17050_e29626_d_n14;
        var_t7_rv = 0.0;

        let (assign17060_e29652, assign17060_e29652_d_n0, assign17060_e29652_d_n2, assign17060_e29652_d_n3, assign17060_e29652_d_n4, assign17060_e29652_d_n5, assign17060_e29652_d_n6, assign17060_e29652_d_n7, assign17060_e29652_d_n8, assign17060_e29652_d_n9, assign17060_e29652_d_n10, assign17060_e29652_d_n11, assign17060_e29652_d_n13, assign17060_e29652_d_n14,) = {
    if (((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 != 0.0)) && (var_guard305 != 0.0)) {
        let assign17060_e29648: f64 = (var_devtemp - 210.0);
        let assign17060_e29649: f64 = (var_prt_i * assign17060_e29648);
        let assign17060_e29650: f64 = (var_t7 + assign17060_e29649);
        (assign17060_e29650, var_t7_dn0, var_t7_dn2, var_t7_dn3, (var_t7_dn4 + (var_prt_i * var_devtemp_dn4)), var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn11, var_t7_dn13, var_t7_dn14,)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn3, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8, var_t8_dn9, var_t8_dn10, var_t8_dn11, var_t8_dn13, var_t8_dn14,)
    }
};
        var_t8 = assign17060_e29652;
        var_t8_dn0 = assign17060_e29652_d_n0;
        var_t8_dn2 = assign17060_e29652_d_n2;
        var_t8_dn3 = assign17060_e29652_d_n3;
        var_t8_dn4 = assign17060_e29652_d_n4;
        var_t8_dn5 = assign17060_e29652_d_n5;
        var_t8_dn6 = assign17060_e29652_d_n6;
        var_t8_dn7 = assign17060_e29652_d_n7;
        var_t8_dn8 = assign17060_e29652_d_n8;
        var_t8_dn9 = assign17060_e29652_d_n9;
        var_t8_dn10 = assign17060_e29652_d_n10;
        var_t8_dn11 = assign17060_e29652_d_n11;
        var_t8_dn13 = assign17060_e29652_d_n13;
        var_t8_dn14 = assign17060_e29652_d_n14;
        var_t8_rv = 0.0;

        let (assign17070_e29691, assign17070_e29691_d_n0, assign17070_e29691_d_n2, assign17070_e29691_d_n3, assign17070_e29691_d_n4, assign17070_e29691_d_n5, assign17070_e29691_d_n6, assign17070_e29691_d_n7, assign17070_e29691_d_n8, assign17070_e29691_d_n9, assign17070_e29691_d_n10, assign17070_e29691_d_n11, assign17070_e29691_d_n13, assign17070_e29691_d_n14,) = {
    if (((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 != 0.0)) && (var_guard305 != 0.0)) {
        let assign17070_e29673: f64 = (var_t6 + var_t8);
        let assign17070_e29676: f64 = (var_t6 - var_t8);
        let assign17070_e29679: f64 = (var_t6 - var_t8);
        let assign17070_e29680: f64 = (assign17070_e29676 * assign17070_e29679);
        let assign17070_e29683: f64 = (0.25 * 0.001);
        let assign17070_e29685: f64 = (assign17070_e29683 * 0.001);
        let assign17070_e29686: f64 = (assign17070_e29680 + assign17070_e29685);
        let assign17070_e29687: f64 = (assign17070_e29686).sqrt();
        let assign17070_e29688: f64 = (assign17070_e29673 + assign17070_e29687);
        let assign17070_e29689: f64 = (0.5 * assign17070_e29688);
        (assign17070_e29689, (0.5 * ((var_t6_dn0 + var_t8_dn0) + ((((var_t6_dn0 - var_t8_dn0) * assign17070_e29679) + (assign17070_e29676 * (var_t6_dn0 - var_t8_dn0))) / (2.0 * assign17070_e29687)))), (0.5 * ((var_t6_dn2 + var_t8_dn2) + ((((var_t6_dn2 - var_t8_dn2) * assign17070_e29679) + (assign17070_e29676 * (var_t6_dn2 - var_t8_dn2))) / (2.0 * assign17070_e29687)))), (0.5 * ((var_t6_dn3 + var_t8_dn3) + ((((var_t6_dn3 - var_t8_dn3) * assign17070_e29679) + (assign17070_e29676 * (var_t6_dn3 - var_t8_dn3))) / (2.0 * assign17070_e29687)))), (0.5 * ((var_t6_dn4 + var_t8_dn4) + ((((var_t6_dn4 - var_t8_dn4) * assign17070_e29679) + (assign17070_e29676 * (var_t6_dn4 - var_t8_dn4))) / (2.0 * assign17070_e29687)))), (0.5 * ((var_t6_dn5 + var_t8_dn5) + ((((var_t6_dn5 - var_t8_dn5) * assign17070_e29679) + (assign17070_e29676 * (var_t6_dn5 - var_t8_dn5))) / (2.0 * assign17070_e29687)))), (0.5 * ((var_t6_dn6 + var_t8_dn6) + ((((var_t6_dn6 - var_t8_dn6) * assign17070_e29679) + (assign17070_e29676 * (var_t6_dn6 - var_t8_dn6))) / (2.0 * assign17070_e29687)))), (0.5 * ((var_t6_dn7 + var_t8_dn7) + ((((var_t6_dn7 - var_t8_dn7) * assign17070_e29679) + (assign17070_e29676 * (var_t6_dn7 - var_t8_dn7))) / (2.0 * assign17070_e29687)))), (0.5 * ((var_t6_dn8 + var_t8_dn8) + ((((var_t6_dn8 - var_t8_dn8) * assign17070_e29679) + (assign17070_e29676 * (var_t6_dn8 - var_t8_dn8))) / (2.0 * assign17070_e29687)))), (0.5 * ((var_t6_dn9 + var_t8_dn9) + ((((var_t6_dn9 - var_t8_dn9) * assign17070_e29679) + (assign17070_e29676 * (var_t6_dn9 - var_t8_dn9))) / (2.0 * assign17070_e29687)))), (0.5 * ((var_t6_dn10 + var_t8_dn10) + ((((var_t6_dn10 - var_t8_dn10) * assign17070_e29679) + (assign17070_e29676 * (var_t6_dn10 - var_t8_dn10))) / (2.0 * assign17070_e29687)))), (0.5 * ((var_t6_dn11 + var_t8_dn11) + ((((var_t6_dn11 - var_t8_dn11) * assign17070_e29679) + (assign17070_e29676 * (var_t6_dn11 - var_t8_dn11))) / (2.0 * assign17070_e29687)))), (0.5 * ((var_t6_dn13 + var_t8_dn13) + ((((var_t6_dn13 - var_t8_dn13) * assign17070_e29679) + (assign17070_e29676 * (var_t6_dn13 - var_t8_dn13))) / (2.0 * assign17070_e29687)))), (0.5 * ((var_t6_dn14 + var_t8_dn14) + ((((var_t6_dn14 - var_t8_dn14) * assign17070_e29679) + (assign17070_e29676 * (var_t6_dn14 - var_t8_dn14))) / (2.0 * assign17070_e29687)))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    }
};
        var_t2 = assign17070_e29691;
        var_t2_dn0 = assign17070_e29691_d_n0;
        var_t2_dn2 = assign17070_e29691_d_n2;
        var_t2_dn3 = assign17070_e29691_d_n3;
        var_t2_dn4 = assign17070_e29691_d_n4;
        var_t2_dn5 = assign17070_e29691_d_n5;
        var_t2_dn6 = assign17070_e29691_d_n6;
        var_t2_dn7 = assign17070_e29691_d_n7;
        var_t2_dn8 = assign17070_e29691_d_n8;
        var_t2_dn9 = assign17070_e29691_d_n9;
        var_t2_dn10 = assign17070_e29691_d_n10;
        var_t2_dn11 = assign17070_e29691_d_n11;
        var_t2_dn13 = assign17070_e29691_d_n13;
        var_t2_dn14 = assign17070_e29691_d_n14;
        var_t2_rv = 0.0;

        let (assign17080_e29752, assign17080_e29752_d_n0, assign17080_e29752_d_n2, assign17080_e29752_d_n3, assign17080_e29752_d_n4, assign17080_e29752_d_n5, assign17080_e29752_d_n6, assign17080_e29752_d_n7, assign17080_e29752_d_n8, assign17080_e29752_d_n9, assign17080_e29752_d_n10, assign17080_e29752_d_n11, assign17080_e29752_d_n13, assign17080_e29752_d_n14,) = {
    if (((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 != 0.0)) && (var_guard305 == 0.0)) {
        let assign17080_e29713: f64 = (var_rdstemp0 + var_rdstemp1);
        let assign17080_e29716: f64 = (var_rdstemp0 - var_rdstemp1);
        let assign17080_e29719: f64 = (var_rdstemp0 - var_rdstemp1);
        let assign17080_e29720: f64 = (assign17080_e29716 * assign17080_e29719);
        let assign17080_e29723: f64 = (0.25 * var_sprt_i);
        let assign17080_e29725: f64 = (assign17080_e29723 * var_sprt_i);
        let assign17080_e29726: f64 = (assign17080_e29720 + assign17080_e29725);
        let assign17080_e29727: f64 = (assign17080_e29726).sqrt();
        let assign17080_e29728: f64 = (assign17080_e29713 - assign17080_e29727);
        let assign17080_e29729: f64 = (0.5 * assign17080_e29728);
        let assign17080_e29733: f64 = var_t3;
        let assign17080_e29736: f64 = var_t3;
        let assign17080_e29739: f64 = var_t3;
        let assign17080_e29740: f64 = (assign17080_e29736 * assign17080_e29739);
        let assign17080_e29743: f64 = (0.25 * var_sprt_i);
        let assign17080_e29745: f64 = (assign17080_e29743 * var_sprt_i);
        let assign17080_e29746: f64 = (assign17080_e29740 + assign17080_e29745);
        let assign17080_e29747: f64 = (assign17080_e29746).sqrt();
        let assign17080_e29748: f64 = (assign17080_e29733 - assign17080_e29747);
        let assign17080_e29749: f64 = (0.5 * assign17080_e29748);
        let assign17080_e29750: f64 = (assign17080_e29729 - assign17080_e29749);
        (assign17080_e29750, (-(0.5 * (var_t3_dn0 - (((var_t3_dn0 * assign17080_e29739) + (assign17080_e29736 * var_t3_dn0)) / (2.0 * assign17080_e29747))))), (-(0.5 * (var_t3_dn2 - (((var_t3_dn2 * assign17080_e29739) + (assign17080_e29736 * var_t3_dn2)) / (2.0 * assign17080_e29747))))), (-(0.5 * (var_t3_dn3 - (((var_t3_dn3 * assign17080_e29739) + (assign17080_e29736 * var_t3_dn3)) / (2.0 * assign17080_e29747))))), ((0.5 * ((var_rdstemp0_dn4 + var_rdstemp1_dn4) - ((((var_rdstemp0_dn4 - var_rdstemp1_dn4) * assign17080_e29719) + (assign17080_e29716 * (var_rdstemp0_dn4 - var_rdstemp1_dn4))) / (2.0 * assign17080_e29727)))) - (0.5 * (var_t3_dn4 - (((var_t3_dn4 * assign17080_e29739) + (assign17080_e29736 * var_t3_dn4)) / (2.0 * assign17080_e29747))))), (-(0.5 * (var_t3_dn5 - (((var_t3_dn5 * assign17080_e29739) + (assign17080_e29736 * var_t3_dn5)) / (2.0 * assign17080_e29747))))), (-(0.5 * (var_t3_dn6 - (((var_t3_dn6 * assign17080_e29739) + (assign17080_e29736 * var_t3_dn6)) / (2.0 * assign17080_e29747))))), (-(0.5 * (var_t3_dn7 - (((var_t3_dn7 * assign17080_e29739) + (assign17080_e29736 * var_t3_dn7)) / (2.0 * assign17080_e29747))))), (-(0.5 * (var_t3_dn8 - (((var_t3_dn8 * assign17080_e29739) + (assign17080_e29736 * var_t3_dn8)) / (2.0 * assign17080_e29747))))), (-(0.5 * (var_t3_dn9 - (((var_t3_dn9 * assign17080_e29739) + (assign17080_e29736 * var_t3_dn9)) / (2.0 * assign17080_e29747))))), (-(0.5 * (var_t3_dn10 - (((var_t3_dn10 * assign17080_e29739) + (assign17080_e29736 * var_t3_dn10)) / (2.0 * assign17080_e29747))))), (-(0.5 * (var_t3_dn11 - (((var_t3_dn11 * assign17080_e29739) + (assign17080_e29736 * var_t3_dn11)) / (2.0 * assign17080_e29747))))), (-(0.5 * (var_t3_dn13 - (((var_t3_dn13 * assign17080_e29739) + (assign17080_e29736 * var_t3_dn13)) / (2.0 * assign17080_e29747))))), (-(0.5 * (var_t3_dn14 - (((var_t3_dn14 * assign17080_e29739) + (assign17080_e29736 * var_t3_dn14)) / (2.0 * assign17080_e29747))))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn13, var_t6_dn14,)
    }
};
        var_t6 = assign17080_e29752;
        var_t6_dn0 = assign17080_e29752_d_n0;
        var_t6_dn2 = assign17080_e29752_d_n2;
        var_t6_dn3 = assign17080_e29752_d_n3;
        var_t6_dn4 = assign17080_e29752_d_n4;
        var_t6_dn5 = assign17080_e29752_d_n5;
        var_t6_dn6 = assign17080_e29752_d_n6;
        var_t6_dn7 = assign17080_e29752_d_n7;
        var_t6_dn8 = assign17080_e29752_d_n8;
        var_t6_dn9 = assign17080_e29752_d_n9;
        var_t6_dn10 = assign17080_e29752_d_n10;
        var_t6_dn11 = assign17080_e29752_d_n11;
        var_t6_dn13 = assign17080_e29752_d_n13;
        var_t6_dn14 = assign17080_e29752_d_n14;
        var_t6_rv = 0.0;

        let (assign17090_e29813, assign17090_e29813_d_n0, assign17090_e29813_d_n2, assign17090_e29813_d_n3, assign17090_e29813_d_n4, assign17090_e29813_d_n5, assign17090_e29813_d_n6, assign17090_e29813_d_n7, assign17090_e29813_d_n8, assign17090_e29813_d_n9, assign17090_e29813_d_n10, assign17090_e29813_d_n11, assign17090_e29813_d_n13, assign17090_e29813_d_n14,) = {
    if (((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 != 0.0)) && (var_guard305 == 0.0)) {
        let assign17090_e29774: f64 = (var_t4 + var_t5);
        let assign17090_e29777: f64 = (var_t4 - var_t5);
        let assign17090_e29780: f64 = (var_t4 - var_t5);
        let assign17090_e29781: f64 = (assign17090_e29777 * assign17090_e29780);
        let assign17090_e29784: f64 = (0.25 * var_sprt_i);
        let assign17090_e29786: f64 = (assign17090_e29784 * var_sprt_i);
        let assign17090_e29787: f64 = (assign17090_e29781 + assign17090_e29786);
        let assign17090_e29788: f64 = (assign17090_e29787).sqrt();
        let assign17090_e29789: f64 = (assign17090_e29774 - assign17090_e29788);
        let assign17090_e29790: f64 = (0.5 * assign17090_e29789);
        let assign17090_e29794: f64 = var_t3;
        let assign17090_e29797: f64 = var_t3;
        let assign17090_e29800: f64 = var_t3;
        let assign17090_e29801: f64 = (assign17090_e29797 * assign17090_e29800);
        let assign17090_e29804: f64 = (0.25 * var_sprt_i);
        let assign17090_e29806: f64 = (assign17090_e29804 * var_sprt_i);
        let assign17090_e29807: f64 = (assign17090_e29801 + assign17090_e29806);
        let assign17090_e29808: f64 = (assign17090_e29807).sqrt();
        let assign17090_e29809: f64 = (assign17090_e29794 - assign17090_e29808);
        let assign17090_e29810: f64 = (0.5 * assign17090_e29809);
        let assign17090_e29811: f64 = (assign17090_e29790 - assign17090_e29810);
        (assign17090_e29811, ((0.5 * ((var_t4_dn0 + var_t5_dn0) - ((((var_t4_dn0 - var_t5_dn0) * assign17090_e29780) + (assign17090_e29777 * (var_t4_dn0 - var_t5_dn0))) / (2.0 * assign17090_e29788)))) - (0.5 * (var_t3_dn0 - (((var_t3_dn0 * assign17090_e29800) + (assign17090_e29797 * var_t3_dn0)) / (2.0 * assign17090_e29808))))), ((0.5 * ((var_t4_dn2 + var_t5_dn2) - ((((var_t4_dn2 - var_t5_dn2) * assign17090_e29780) + (assign17090_e29777 * (var_t4_dn2 - var_t5_dn2))) / (2.0 * assign17090_e29788)))) - (0.5 * (var_t3_dn2 - (((var_t3_dn2 * assign17090_e29800) + (assign17090_e29797 * var_t3_dn2)) / (2.0 * assign17090_e29808))))), ((0.5 * ((var_t4_dn3 + var_t5_dn3) - ((((var_t4_dn3 - var_t5_dn3) * assign17090_e29780) + (assign17090_e29777 * (var_t4_dn3 - var_t5_dn3))) / (2.0 * assign17090_e29788)))) - (0.5 * (var_t3_dn3 - (((var_t3_dn3 * assign17090_e29800) + (assign17090_e29797 * var_t3_dn3)) / (2.0 * assign17090_e29808))))), ((0.5 * ((var_t4_dn4 + var_t5_dn4) - ((((var_t4_dn4 - var_t5_dn4) * assign17090_e29780) + (assign17090_e29777 * (var_t4_dn4 - var_t5_dn4))) / (2.0 * assign17090_e29788)))) - (0.5 * (var_t3_dn4 - (((var_t3_dn4 * assign17090_e29800) + (assign17090_e29797 * var_t3_dn4)) / (2.0 * assign17090_e29808))))), ((0.5 * ((var_t4_dn5 + var_t5_dn5) - ((((var_t4_dn5 - var_t5_dn5) * assign17090_e29780) + (assign17090_e29777 * (var_t4_dn5 - var_t5_dn5))) / (2.0 * assign17090_e29788)))) - (0.5 * (var_t3_dn5 - (((var_t3_dn5 * assign17090_e29800) + (assign17090_e29797 * var_t3_dn5)) / (2.0 * assign17090_e29808))))), ((0.5 * ((var_t4_dn6 + var_t5_dn6) - ((((var_t4_dn6 - var_t5_dn6) * assign17090_e29780) + (assign17090_e29777 * (var_t4_dn6 - var_t5_dn6))) / (2.0 * assign17090_e29788)))) - (0.5 * (var_t3_dn6 - (((var_t3_dn6 * assign17090_e29800) + (assign17090_e29797 * var_t3_dn6)) / (2.0 * assign17090_e29808))))), ((0.5 * ((var_t4_dn7 + var_t5_dn7) - ((((var_t4_dn7 - var_t5_dn7) * assign17090_e29780) + (assign17090_e29777 * (var_t4_dn7 - var_t5_dn7))) / (2.0 * assign17090_e29788)))) - (0.5 * (var_t3_dn7 - (((var_t3_dn7 * assign17090_e29800) + (assign17090_e29797 * var_t3_dn7)) / (2.0 * assign17090_e29808))))), ((0.5 * ((var_t4_dn8 + var_t5_dn8) - ((((var_t4_dn8 - var_t5_dn8) * assign17090_e29780) + (assign17090_e29777 * (var_t4_dn8 - var_t5_dn8))) / (2.0 * assign17090_e29788)))) - (0.5 * (var_t3_dn8 - (((var_t3_dn8 * assign17090_e29800) + (assign17090_e29797 * var_t3_dn8)) / (2.0 * assign17090_e29808))))), ((0.5 * ((var_t4_dn9 + var_t5_dn9) - ((((var_t4_dn9 - var_t5_dn9) * assign17090_e29780) + (assign17090_e29777 * (var_t4_dn9 - var_t5_dn9))) / (2.0 * assign17090_e29788)))) - (0.5 * (var_t3_dn9 - (((var_t3_dn9 * assign17090_e29800) + (assign17090_e29797 * var_t3_dn9)) / (2.0 * assign17090_e29808))))), ((0.5 * ((var_t4_dn10 + var_t5_dn10) - ((((var_t4_dn10 - var_t5_dn10) * assign17090_e29780) + (assign17090_e29777 * (var_t4_dn10 - var_t5_dn10))) / (2.0 * assign17090_e29788)))) - (0.5 * (var_t3_dn10 - (((var_t3_dn10 * assign17090_e29800) + (assign17090_e29797 * var_t3_dn10)) / (2.0 * assign17090_e29808))))), ((0.5 * ((var_t4_dn11 + var_t5_dn11) - ((((var_t4_dn11 - var_t5_dn11) * assign17090_e29780) + (assign17090_e29777 * (var_t4_dn11 - var_t5_dn11))) / (2.0 * assign17090_e29788)))) - (0.5 * (var_t3_dn11 - (((var_t3_dn11 * assign17090_e29800) + (assign17090_e29797 * var_t3_dn11)) / (2.0 * assign17090_e29808))))), ((0.5 * ((var_t4_dn13 + var_t5_dn13) - ((((var_t4_dn13 - var_t5_dn13) * assign17090_e29780) + (assign17090_e29777 * (var_t4_dn13 - var_t5_dn13))) / (2.0 * assign17090_e29788)))) - (0.5 * (var_t3_dn13 - (((var_t3_dn13 * assign17090_e29800) + (assign17090_e29797 * var_t3_dn13)) / (2.0 * assign17090_e29808))))), ((0.5 * ((var_t4_dn14 + var_t5_dn14) - ((((var_t4_dn14 - var_t5_dn14) * assign17090_e29780) + (assign17090_e29777 * (var_t4_dn14 - var_t5_dn14))) / (2.0 * assign17090_e29788)))) - (0.5 * (var_t3_dn14 - (((var_t3_dn14 * assign17090_e29800) + (assign17090_e29797 * var_t3_dn14)) / (2.0 * assign17090_e29808))))),)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn3, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn11, var_t7_dn13, var_t7_dn14,)
    }
};
        var_t7 = assign17090_e29813;
        var_t7_dn0 = assign17090_e29813_d_n0;
        var_t7_dn2 = assign17090_e29813_d_n2;
        var_t7_dn3 = assign17090_e29813_d_n3;
        var_t7_dn4 = assign17090_e29813_d_n4;
        var_t7_dn5 = assign17090_e29813_d_n5;
        var_t7_dn6 = assign17090_e29813_d_n6;
        var_t7_dn7 = assign17090_e29813_d_n7;
        var_t7_dn8 = assign17090_e29813_d_n8;
        var_t7_dn9 = assign17090_e29813_d_n9;
        var_t7_dn10 = assign17090_e29813_d_n10;
        var_t7_dn11 = assign17090_e29813_d_n11;
        var_t7_dn13 = assign17090_e29813_d_n13;
        var_t7_dn14 = assign17090_e29813_d_n14;
        var_t7_rv = 0.0;

        let (assign17100_e29840, assign17100_e29840_d_n0, assign17100_e29840_d_n2, assign17100_e29840_d_n3, assign17100_e29840_d_n4, assign17100_e29840_d_n5, assign17100_e29840_d_n6, assign17100_e29840_d_n7, assign17100_e29840_d_n8, assign17100_e29840_d_n9, assign17100_e29840_d_n10, assign17100_e29840_d_n11, assign17100_e29840_d_n13, assign17100_e29840_d_n14,) = {
    if (((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 != 0.0)) && (var_guard305 == 0.0)) {
        let assign17100_e29836: f64 = (var_devtemp - 210.0);
        let assign17100_e29837: f64 = (var_prt_i * assign17100_e29836);
        let assign17100_e29838: f64 = (var_t7 + assign17100_e29837);
        (assign17100_e29838, var_t7_dn0, var_t7_dn2, var_t7_dn3, (var_t7_dn4 + (var_prt_i * var_devtemp_dn4)), var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn11, var_t7_dn13, var_t7_dn14,)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn3, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8, var_t8_dn9, var_t8_dn10, var_t8_dn11, var_t8_dn13, var_t8_dn14,)
    }
};
        var_t8 = assign17100_e29840;
        var_t8_dn0 = assign17100_e29840_d_n0;
        var_t8_dn2 = assign17100_e29840_d_n2;
        var_t8_dn3 = assign17100_e29840_d_n3;
        var_t8_dn4 = assign17100_e29840_d_n4;
        var_t8_dn5 = assign17100_e29840_d_n5;
        var_t8_dn6 = assign17100_e29840_d_n6;
        var_t8_dn7 = assign17100_e29840_d_n7;
        var_t8_dn8 = assign17100_e29840_d_n8;
        var_t8_dn9 = assign17100_e29840_d_n9;
        var_t8_dn10 = assign17100_e29840_d_n10;
        var_t8_dn11 = assign17100_e29840_d_n11;
        var_t8_dn13 = assign17100_e29840_d_n13;
        var_t8_dn14 = assign17100_e29840_d_n14;
        var_t8_rv = 0.0;

        let (assign17110_e29880, assign17110_e29880_d_n0, assign17110_e29880_d_n2, assign17110_e29880_d_n3, assign17110_e29880_d_n4, assign17110_e29880_d_n5, assign17110_e29880_d_n6, assign17110_e29880_d_n7, assign17110_e29880_d_n8, assign17110_e29880_d_n9, assign17110_e29880_d_n10, assign17110_e29880_d_n11, assign17110_e29880_d_n13, assign17110_e29880_d_n14,) = {
    if (((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 != 0.0)) && (var_guard305 == 0.0)) {
        let assign17110_e29862: f64 = (var_t6 + var_t8);
        let assign17110_e29865: f64 = (var_t6 - var_t8);
        let assign17110_e29868: f64 = (var_t6 - var_t8);
        let assign17110_e29869: f64 = (assign17110_e29865 * assign17110_e29868);
        let assign17110_e29872: f64 = (0.25 * 0.001);
        let assign17110_e29874: f64 = (assign17110_e29872 * 0.001);
        let assign17110_e29875: f64 = (assign17110_e29869 + assign17110_e29874);
        let assign17110_e29876: f64 = (assign17110_e29875).sqrt();
        let assign17110_e29877: f64 = (assign17110_e29862 - assign17110_e29876);
        let assign17110_e29878: f64 = (0.5 * assign17110_e29877);
        (assign17110_e29878, (0.5 * ((var_t6_dn0 + var_t8_dn0) - ((((var_t6_dn0 - var_t8_dn0) * assign17110_e29868) + (assign17110_e29865 * (var_t6_dn0 - var_t8_dn0))) / (2.0 * assign17110_e29876)))), (0.5 * ((var_t6_dn2 + var_t8_dn2) - ((((var_t6_dn2 - var_t8_dn2) * assign17110_e29868) + (assign17110_e29865 * (var_t6_dn2 - var_t8_dn2))) / (2.0 * assign17110_e29876)))), (0.5 * ((var_t6_dn3 + var_t8_dn3) - ((((var_t6_dn3 - var_t8_dn3) * assign17110_e29868) + (assign17110_e29865 * (var_t6_dn3 - var_t8_dn3))) / (2.0 * assign17110_e29876)))), (0.5 * ((var_t6_dn4 + var_t8_dn4) - ((((var_t6_dn4 - var_t8_dn4) * assign17110_e29868) + (assign17110_e29865 * (var_t6_dn4 - var_t8_dn4))) / (2.0 * assign17110_e29876)))), (0.5 * ((var_t6_dn5 + var_t8_dn5) - ((((var_t6_dn5 - var_t8_dn5) * assign17110_e29868) + (assign17110_e29865 * (var_t6_dn5 - var_t8_dn5))) / (2.0 * assign17110_e29876)))), (0.5 * ((var_t6_dn6 + var_t8_dn6) - ((((var_t6_dn6 - var_t8_dn6) * assign17110_e29868) + (assign17110_e29865 * (var_t6_dn6 - var_t8_dn6))) / (2.0 * assign17110_e29876)))), (0.5 * ((var_t6_dn7 + var_t8_dn7) - ((((var_t6_dn7 - var_t8_dn7) * assign17110_e29868) + (assign17110_e29865 * (var_t6_dn7 - var_t8_dn7))) / (2.0 * assign17110_e29876)))), (0.5 * ((var_t6_dn8 + var_t8_dn8) - ((((var_t6_dn8 - var_t8_dn8) * assign17110_e29868) + (assign17110_e29865 * (var_t6_dn8 - var_t8_dn8))) / (2.0 * assign17110_e29876)))), (0.5 * ((var_t6_dn9 + var_t8_dn9) - ((((var_t6_dn9 - var_t8_dn9) * assign17110_e29868) + (assign17110_e29865 * (var_t6_dn9 - var_t8_dn9))) / (2.0 * assign17110_e29876)))), (0.5 * ((var_t6_dn10 + var_t8_dn10) - ((((var_t6_dn10 - var_t8_dn10) * assign17110_e29868) + (assign17110_e29865 * (var_t6_dn10 - var_t8_dn10))) / (2.0 * assign17110_e29876)))), (0.5 * ((var_t6_dn11 + var_t8_dn11) - ((((var_t6_dn11 - var_t8_dn11) * assign17110_e29868) + (assign17110_e29865 * (var_t6_dn11 - var_t8_dn11))) / (2.0 * assign17110_e29876)))), (0.5 * ((var_t6_dn13 + var_t8_dn13) - ((((var_t6_dn13 - var_t8_dn13) * assign17110_e29868) + (assign17110_e29865 * (var_t6_dn13 - var_t8_dn13))) / (2.0 * assign17110_e29876)))), (0.5 * ((var_t6_dn14 + var_t8_dn14) - ((((var_t6_dn14 - var_t8_dn14) * assign17110_e29868) + (assign17110_e29865 * (var_t6_dn14 - var_t8_dn14))) / (2.0 * assign17110_e29876)))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    }
};
        var_t2 = assign17110_e29880;
        var_t2_dn0 = assign17110_e29880_d_n0;
        var_t2_dn2 = assign17110_e29880_d_n2;
        var_t2_dn3 = assign17110_e29880_d_n3;
        var_t2_dn4 = assign17110_e29880_d_n4;
        var_t2_dn5 = assign17110_e29880_d_n5;
        var_t2_dn6 = assign17110_e29880_d_n6;
        var_t2_dn7 = assign17110_e29880_d_n7;
        var_t2_dn8 = assign17110_e29880_d_n8;
        var_t2_dn9 = assign17110_e29880_d_n9;
        var_t2_dn10 = assign17110_e29880_d_n10;
        var_t2_dn11 = assign17110_e29880_d_n11;
        var_t2_dn13 = assign17110_e29880_d_n13;
        var_t2_dn14 = assign17110_e29880_d_n14;
        var_t2_rv = 0.0;

        let (assign17120_e29903, assign17120_e29903_d_n4,) = {
    if ((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 == 0.0)) {
        let assign17120_e29900: f64 = (var_prt1_i * var_deltemp);
        let assign17120_e29901: f64 = (1.0 + assign17120_e29900);
        (assign17120_e29901, (var_prt1_i * var_deltemp_dn4),)
    } else {
        (var_rdstemp1, var_rdstemp1_dn4,)
    }
};
        var_rdstemp1 = assign17120_e29903;
        var_rdstemp1_dn4 = assign17120_e29903_d_n4;
        var_rdstemp1_rv = 0.0;

        let (assign17130_e29934, assign17130_e29934_d_n4,) = {
    if ((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 == 0.0)) {
        let assign17130_e29924: f64 = (var_devtemp - var_tr0_i);
        let assign17130_e29925: f64 = (var_prt_i * assign17130_e29924);
        let assign17130_e29926: f64 = (1.0 + assign17130_e29925);
        let assign17130_e29930: f64 = (var_tr0_i - var_tnom);
        let assign17130_e29931: f64 = (var_prt1_i * assign17130_e29930);
        let assign17130_e29932: f64 = (assign17130_e29926 + assign17130_e29931);
        (assign17130_e29932, (var_prt_i * var_devtemp_dn4),)
    } else {
        (var_rdstemp0, var_rdstemp0_dn4,)
    }
};
        var_rdstemp0 = assign17130_e29934;
        var_rdstemp0_dn4 = assign17130_e29934_d_n4;
        var_rdstemp0_rv = 0.0;

        let (assign17140_e29959, assign17140_e29959_d_n0, assign17140_e29959_d_n2, assign17140_e29959_d_n3, assign17140_e29959_d_n4, assign17140_e29959_d_n5, assign17140_e29959_d_n6, assign17140_e29959_d_n7, assign17140_e29959_d_n8, assign17140_e29959_d_n9, assign17140_e29959_d_n10, assign17140_e29959_d_n11, assign17140_e29959_d_n13, assign17140_e29959_d_n14,) = {
    if ((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 == 0.0)) {
        let assign17140_e29953: f64 = (var_prt1_i - var_prt_i);
        let assign17140_e29956: f64 = (var_tr0_i - var_tnom);
        let assign17140_e29957: f64 = (assign17140_e29953 * assign17140_e29956);
        (assign17140_e29957, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn13, var_t3_dn14,)
    }
};
        var_t3 = assign17140_e29959;
        var_t3_dn0 = assign17140_e29959_d_n0;
        var_t3_dn2 = assign17140_e29959_d_n2;
        var_t3_dn3 = assign17140_e29959_d_n3;
        var_t3_dn4 = assign17140_e29959_d_n4;
        var_t3_dn5 = assign17140_e29959_d_n5;
        var_t3_dn6 = assign17140_e29959_d_n6;
        var_t3_dn7 = assign17140_e29959_d_n7;
        var_t3_dn8 = assign17140_e29959_d_n8;
        var_t3_dn9 = assign17140_e29959_d_n9;
        var_t3_dn10 = assign17140_e29959_d_n10;
        var_t3_dn11 = assign17140_e29959_d_n11;
        var_t3_dn13 = assign17140_e29959_d_n13;
        var_t3_dn14 = assign17140_e29959_d_n14;
        var_t3_rv = 0.0;

        let (assign17150_e29984, assign17150_e29984_d_n0, assign17150_e29984_d_n2, assign17150_e29984_d_n3, assign17150_e29984_d_n4, assign17150_e29984_d_n5, assign17150_e29984_d_n6, assign17150_e29984_d_n7, assign17150_e29984_d_n8, assign17150_e29984_d_n9, assign17150_e29984_d_n10, assign17150_e29984_d_n11, assign17150_e29984_d_n13, assign17150_e29984_d_n14,) = {
    if ((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 == 0.0)) {
        let assign17150_e29980: f64 = (210.0 - var_tnom);
        let assign17150_e29981: f64 = (var_prt1_i * assign17150_e29980);
        let assign17150_e29982: f64 = (1.0 + assign17150_e29981);
        (assign17150_e29982, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11, var_t4_dn13, var_t4_dn14,)
    }
};
        var_t4 = assign17150_e29984;
        var_t4_dn0 = assign17150_e29984_d_n0;
        var_t4_dn2 = assign17150_e29984_d_n2;
        var_t4_dn3 = assign17150_e29984_d_n3;
        var_t4_dn4 = assign17150_e29984_d_n4;
        var_t4_dn5 = assign17150_e29984_d_n5;
        var_t4_dn6 = assign17150_e29984_d_n6;
        var_t4_dn7 = assign17150_e29984_d_n7;
        var_t4_dn8 = assign17150_e29984_d_n8;
        var_t4_dn9 = assign17150_e29984_d_n9;
        var_t4_dn10 = assign17150_e29984_d_n10;
        var_t4_dn11 = assign17150_e29984_d_n11;
        var_t4_dn13 = assign17150_e29984_d_n13;
        var_t4_dn14 = assign17150_e29984_d_n14;
        var_t4_rv = 0.0;

        let (assign17160_e30015, assign17160_e30015_d_n0, assign17160_e30015_d_n2, assign17160_e30015_d_n3, assign17160_e30015_d_n4, assign17160_e30015_d_n5, assign17160_e30015_d_n6, assign17160_e30015_d_n7, assign17160_e30015_d_n8, assign17160_e30015_d_n9, assign17160_e30015_d_n10, assign17160_e30015_d_n11, assign17160_e30015_d_n13, assign17160_e30015_d_n14,) = {
    if ((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 == 0.0)) {
        let assign17160_e30005: f64 = (210.0 - var_tr0_i);
        let assign17160_e30006: f64 = (var_prt_i * assign17160_e30005);
        let assign17160_e30007: f64 = (1.0 + assign17160_e30006);
        let assign17160_e30011: f64 = (var_tr0_i - var_tnom);
        let assign17160_e30012: f64 = (var_prt1_i * assign17160_e30011);
        let assign17160_e30013: f64 = (assign17160_e30007 + assign17160_e30012);
        (assign17160_e30013, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11, var_t5_dn13, var_t5_dn14,)
    }
};
        var_t5 = assign17160_e30015;
        var_t5_dn0 = assign17160_e30015_d_n0;
        var_t5_dn2 = assign17160_e30015_d_n2;
        var_t5_dn3 = assign17160_e30015_d_n3;
        var_t5_dn4 = assign17160_e30015_d_n4;
        var_t5_dn5 = assign17160_e30015_d_n5;
        var_t5_dn6 = assign17160_e30015_d_n6;
        var_t5_dn7 = assign17160_e30015_d_n7;
        var_t5_dn8 = assign17160_e30015_d_n8;
        var_t5_dn9 = assign17160_e30015_d_n9;
        var_t5_dn10 = assign17160_e30015_d_n10;
        var_t5_dn11 = assign17160_e30015_d_n11;
        var_t5_dn13 = assign17160_e30015_d_n13;
        var_t5_dn14 = assign17160_e30015_d_n14;
        var_t5_rv = 0.0;

        let assign17170_e30018: f64 = if var_prt1_i < var_prt_i { 1.0 } else { 0.0 };
        var_guard306 = assign17170_e30018;
        var_guard306_rv = 0.0;

        let (assign17180_e30079, assign17180_e30079_d_n0, assign17180_e30079_d_n2, assign17180_e30079_d_n3, assign17180_e30079_d_n4, assign17180_e30079_d_n5, assign17180_e30079_d_n6, assign17180_e30079_d_n7, assign17180_e30079_d_n8, assign17180_e30079_d_n9, assign17180_e30079_d_n10, assign17180_e30079_d_n11, assign17180_e30079_d_n13, assign17180_e30079_d_n14,) = {
    if (((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 == 0.0)) && (var_guard306 != 0.0)) {
        let assign17180_e30040: f64 = (var_rdstemp0 + var_rdstemp1);
        let assign17180_e30043: f64 = (var_rdstemp0 - var_rdstemp1);
        let assign17180_e30046: f64 = (var_rdstemp0 - var_rdstemp1);
        let assign17180_e30047: f64 = (assign17180_e30043 * assign17180_e30046);
        let assign17180_e30050: f64 = (0.25 * var_sprt_i);
        let assign17180_e30052: f64 = (assign17180_e30050 * var_sprt_i);
        let assign17180_e30053: f64 = (assign17180_e30047 + assign17180_e30052);
        let assign17180_e30054: f64 = (assign17180_e30053).sqrt();
        let assign17180_e30055: f64 = (assign17180_e30040 + assign17180_e30054);
        let assign17180_e30056: f64 = (0.5 * assign17180_e30055);
        let assign17180_e30060: f64 = var_t3;
        let assign17180_e30063: f64 = var_t3;
        let assign17180_e30066: f64 = var_t3;
        let assign17180_e30067: f64 = (assign17180_e30063 * assign17180_e30066);
        let assign17180_e30070: f64 = (0.25 * var_sprt_i);
        let assign17180_e30072: f64 = (assign17180_e30070 * var_sprt_i);
        let assign17180_e30073: f64 = (assign17180_e30067 + assign17180_e30072);
        let assign17180_e30074: f64 = (assign17180_e30073).sqrt();
        let assign17180_e30075: f64 = (assign17180_e30060 + assign17180_e30074);
        let assign17180_e30076: f64 = (0.5 * assign17180_e30075);
        let assign17180_e30077: f64 = (assign17180_e30056 - assign17180_e30076);
        (assign17180_e30077, (-(0.5 * (var_t3_dn0 + (((var_t3_dn0 * assign17180_e30066) + (assign17180_e30063 * var_t3_dn0)) / (2.0 * assign17180_e30074))))), (-(0.5 * (var_t3_dn2 + (((var_t3_dn2 * assign17180_e30066) + (assign17180_e30063 * var_t3_dn2)) / (2.0 * assign17180_e30074))))), (-(0.5 * (var_t3_dn3 + (((var_t3_dn3 * assign17180_e30066) + (assign17180_e30063 * var_t3_dn3)) / (2.0 * assign17180_e30074))))), ((0.5 * ((var_rdstemp0_dn4 + var_rdstemp1_dn4) + ((((var_rdstemp0_dn4 - var_rdstemp1_dn4) * assign17180_e30046) + (assign17180_e30043 * (var_rdstemp0_dn4 - var_rdstemp1_dn4))) / (2.0 * assign17180_e30054)))) - (0.5 * (var_t3_dn4 + (((var_t3_dn4 * assign17180_e30066) + (assign17180_e30063 * var_t3_dn4)) / (2.0 * assign17180_e30074))))), (-(0.5 * (var_t3_dn5 + (((var_t3_dn5 * assign17180_e30066) + (assign17180_e30063 * var_t3_dn5)) / (2.0 * assign17180_e30074))))), (-(0.5 * (var_t3_dn6 + (((var_t3_dn6 * assign17180_e30066) + (assign17180_e30063 * var_t3_dn6)) / (2.0 * assign17180_e30074))))), (-(0.5 * (var_t3_dn7 + (((var_t3_dn7 * assign17180_e30066) + (assign17180_e30063 * var_t3_dn7)) / (2.0 * assign17180_e30074))))), (-(0.5 * (var_t3_dn8 + (((var_t3_dn8 * assign17180_e30066) + (assign17180_e30063 * var_t3_dn8)) / (2.0 * assign17180_e30074))))), (-(0.5 * (var_t3_dn9 + (((var_t3_dn9 * assign17180_e30066) + (assign17180_e30063 * var_t3_dn9)) / (2.0 * assign17180_e30074))))), (-(0.5 * (var_t3_dn10 + (((var_t3_dn10 * assign17180_e30066) + (assign17180_e30063 * var_t3_dn10)) / (2.0 * assign17180_e30074))))), (-(0.5 * (var_t3_dn11 + (((var_t3_dn11 * assign17180_e30066) + (assign17180_e30063 * var_t3_dn11)) / (2.0 * assign17180_e30074))))), (-(0.5 * (var_t3_dn13 + (((var_t3_dn13 * assign17180_e30066) + (assign17180_e30063 * var_t3_dn13)) / (2.0 * assign17180_e30074))))), (-(0.5 * (var_t3_dn14 + (((var_t3_dn14 * assign17180_e30066) + (assign17180_e30063 * var_t3_dn14)) / (2.0 * assign17180_e30074))))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn13, var_t6_dn14,)
    }
};
        var_t6 = assign17180_e30079;
        var_t6_dn0 = assign17180_e30079_d_n0;
        var_t6_dn2 = assign17180_e30079_d_n2;
        var_t6_dn3 = assign17180_e30079_d_n3;
        var_t6_dn4 = assign17180_e30079_d_n4;
        var_t6_dn5 = assign17180_e30079_d_n5;
        var_t6_dn6 = assign17180_e30079_d_n6;
        var_t6_dn7 = assign17180_e30079_d_n7;
        var_t6_dn8 = assign17180_e30079_d_n8;
        var_t6_dn9 = assign17180_e30079_d_n9;
        var_t6_dn10 = assign17180_e30079_d_n10;
        var_t6_dn11 = assign17180_e30079_d_n11;
        var_t6_dn13 = assign17180_e30079_d_n13;
        var_t6_dn14 = assign17180_e30079_d_n14;
        var_t6_rv = 0.0;

        *var_guard305_slot = var_guard305;
        *var_guard305_rv_slot = var_guard305_rv;
        *var_guard306_slot = var_guard306;
        *var_guard306_rv_slot = var_guard306_rv;
        *var_rdstemp0_slot = var_rdstemp0;
        *var_rdstemp0_dn4_slot = var_rdstemp0_dn4;
        *var_rdstemp0_rv_slot = var_rdstemp0_rv;
        *var_rdstemp1_slot = var_rdstemp1;
        *var_rdstemp1_dn4_slot = var_rdstemp1_dn4;
        *var_rdstemp1_rv_slot = var_rdstemp1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
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
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn3_slot = var_t3_dn3;
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
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn13_slot = var_t4_dn13;
        *var_t4_dn14_slot = var_t4_dn14;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn3_slot = var_t4_dn3;
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
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn13_slot = var_t5_dn13;
        *var_t5_dn14_slot = var_t5_dn14;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn3_slot = var_t5_dn3;
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
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn13_slot = var_t6_dn13;
        *var_t6_dn14_slot = var_t6_dn14;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn3_slot = var_t6_dn3;
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
        *var_t7_dn11_slot = var_t7_dn11;
        *var_t7_dn13_slot = var_t7_dn13;
        *var_t7_dn14_slot = var_t7_dn14;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn3_slot = var_t7_dn3;
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
        *var_t8_dn11_slot = var_t8_dn11;
        *var_t8_dn13_slot = var_t8_dn13;
        *var_t8_dn14_slot = var_t8_dn14;
        *var_t8_dn2_slot = var_t8_dn2;
        *var_t8_dn3_slot = var_t8_dn3;
        *var_t8_dn4_slot = var_t8_dn4;
        *var_t8_dn5_slot = var_t8_dn5;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn7_slot = var_t8_dn7;
        *var_t8_dn8_slot = var_t8_dn8;
        *var_t8_dn9_slot = var_t8_dn9;
        *var_t8_rv_slot = var_t8_rv;
    }

    pub(super) fn stamp_reactive_block_58(
        var_deltemp: f64,
        var_deltemp_dn4: f64,
        var_devtemp: f64,
        var_devtemp_dn4: f64,
        var_guard244: f64,
        var_guard259: f64,
        var_guard300: f64,
        var_guard301: f64,
        var_guard302: f64,
        var_guard304: f64,
        var_guard306: f64,
        var_prt1_i: f64,
        var_prt_i: f64,
        var_sprt_i: f64,
        var_t3: f64,
        var_t3_dn0: f64,
        var_t3_dn10: f64,
        var_t3_dn11: f64,
        var_t3_dn13: f64,
        var_t3_dn14: f64,
        var_t3_dn2: f64,
        var_t3_dn3: f64,
        var_t3_dn4: f64,
        var_t3_dn5: f64,
        var_t3_dn6: f64,
        var_t3_dn7: f64,
        var_t3_dn8: f64,
        var_t3_dn9: f64,
        var_t4: f64,
        var_t4_dn0: f64,
        var_t4_dn10: f64,
        var_t4_dn11: f64,
        var_t4_dn13: f64,
        var_t4_dn14: f64,
        var_t4_dn2: f64,
        var_t4_dn3: f64,
        var_t4_dn4: f64,
        var_t4_dn5: f64,
        var_t4_dn6: f64,
        var_t4_dn7: f64,
        var_t4_dn8: f64,
        var_t4_dn9: f64,
        var_t5: f64,
        var_t5_dn0: f64,
        var_t5_dn10: f64,
        var_t5_dn11: f64,
        var_t5_dn13: f64,
        var_t5_dn14: f64,
        var_t5_dn2: f64,
        var_t5_dn3: f64,
        var_t5_dn4: f64,
        var_t5_dn5: f64,
        var_t5_dn6: f64,
        var_t5_dn7: f64,
        var_t5_dn8: f64,
        var_t5_dn9: f64,
        var_tnom: f64,
        var_guard307_slot: &mut f64,
        var_guard307_rv_slot: &mut f64,
        var_guard308_slot: &mut f64,
        var_guard308_rv_slot: &mut f64,
        var_guard309_slot: &mut f64,
        var_guard309_rv_slot: &mut f64,
        var_rdstemp_slot: &mut f64,
        var_rdstemp0_slot: &mut f64,
        var_rdstemp0_dn4_slot: &mut f64,
        var_rdstemp0_rv_slot: &mut f64,
        var_rdstemp1_slot: &mut f64,
        var_rdstemp1_dn4_slot: &mut f64,
        var_rdstemp1_rv_slot: &mut f64,
        var_rdstemp_dn0_slot: &mut f64,
        var_rdstemp_dn10_slot: &mut f64,
        var_rdstemp_dn11_slot: &mut f64,
        var_rdstemp_dn13_slot: &mut f64,
        var_rdstemp_dn14_slot: &mut f64,
        var_rdstemp_dn2_slot: &mut f64,
        var_rdstemp_dn3_slot: &mut f64,
        var_rdstemp_dn4_slot: &mut f64,
        var_rdstemp_dn5_slot: &mut f64,
        var_rdstemp_dn6_slot: &mut f64,
        var_rdstemp_dn7_slot: &mut f64,
        var_rdstemp_dn8_slot: &mut f64,
        var_rdstemp_dn9_slot: &mut f64,
        var_rdstemp_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn13_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn13_slot: &mut f64,
        var_t6_dn14_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
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
        var_t7_dn11_slot: &mut f64,
        var_t7_dn13_slot: &mut f64,
        var_t7_dn14_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn3_slot: &mut f64,
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
        var_t8_dn11_slot: &mut f64,
        var_t8_dn13_slot: &mut f64,
        var_t8_dn14_slot: &mut f64,
        var_t8_dn2_slot: &mut f64,
        var_t8_dn3_slot: &mut f64,
        var_t8_dn4_slot: &mut f64,
        var_t8_dn5_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn7_slot: &mut f64,
        var_t8_dn8_slot: &mut f64,
        var_t8_dn9_slot: &mut f64,
        var_t8_rv_slot: &mut f64,
    ) {
        let mut var_guard307: f64 = *var_guard307_slot;
        let mut var_guard307_rv: f64 = *var_guard307_rv_slot;
        let mut var_guard308: f64 = *var_guard308_slot;
        let mut var_guard308_rv: f64 = *var_guard308_rv_slot;
        let mut var_guard309: f64 = *var_guard309_slot;
        let mut var_guard309_rv: f64 = *var_guard309_rv_slot;
        let mut var_rdstemp: f64 = *var_rdstemp_slot;
        let mut var_rdstemp0: f64 = *var_rdstemp0_slot;
        let mut var_rdstemp0_dn4: f64 = *var_rdstemp0_dn4_slot;
        let mut var_rdstemp0_rv: f64 = *var_rdstemp0_rv_slot;
        let mut var_rdstemp1: f64 = *var_rdstemp1_slot;
        let mut var_rdstemp1_dn4: f64 = *var_rdstemp1_dn4_slot;
        let mut var_rdstemp1_rv: f64 = *var_rdstemp1_rv_slot;
        let mut var_rdstemp_dn0: f64 = *var_rdstemp_dn0_slot;
        let mut var_rdstemp_dn10: f64 = *var_rdstemp_dn10_slot;
        let mut var_rdstemp_dn11: f64 = *var_rdstemp_dn11_slot;
        let mut var_rdstemp_dn13: f64 = *var_rdstemp_dn13_slot;
        let mut var_rdstemp_dn14: f64 = *var_rdstemp_dn14_slot;
        let mut var_rdstemp_dn2: f64 = *var_rdstemp_dn2_slot;
        let mut var_rdstemp_dn3: f64 = *var_rdstemp_dn3_slot;
        let mut var_rdstemp_dn4: f64 = *var_rdstemp_dn4_slot;
        let mut var_rdstemp_dn5: f64 = *var_rdstemp_dn5_slot;
        let mut var_rdstemp_dn6: f64 = *var_rdstemp_dn6_slot;
        let mut var_rdstemp_dn7: f64 = *var_rdstemp_dn7_slot;
        let mut var_rdstemp_dn8: f64 = *var_rdstemp_dn8_slot;
        let mut var_rdstemp_dn9: f64 = *var_rdstemp_dn9_slot;
        let mut var_rdstemp_rv: f64 = *var_rdstemp_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn13: f64 = *var_t6_dn13_slot;
        let mut var_t6_dn14: f64 = *var_t6_dn14_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
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
        let mut var_t7_dn11: f64 = *var_t7_dn11_slot;
        let mut var_t7_dn13: f64 = *var_t7_dn13_slot;
        let mut var_t7_dn14: f64 = *var_t7_dn14_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn3: f64 = *var_t7_dn3_slot;
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
        let mut var_t8_dn11: f64 = *var_t8_dn11_slot;
        let mut var_t8_dn13: f64 = *var_t8_dn13_slot;
        let mut var_t8_dn14: f64 = *var_t8_dn14_slot;
        let mut var_t8_dn2: f64 = *var_t8_dn2_slot;
        let mut var_t8_dn3: f64 = *var_t8_dn3_slot;
        let mut var_t8_dn4: f64 = *var_t8_dn4_slot;
        let mut var_t8_dn5: f64 = *var_t8_dn5_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn7: f64 = *var_t8_dn7_slot;
        let mut var_t8_dn8: f64 = *var_t8_dn8_slot;
        let mut var_t8_dn9: f64 = *var_t8_dn9_slot;
        let mut var_t8_rv: f64 = *var_t8_rv_slot;

        let (assign17190_e30140, assign17190_e30140_d_n0, assign17190_e30140_d_n2, assign17190_e30140_d_n3, assign17190_e30140_d_n4, assign17190_e30140_d_n5, assign17190_e30140_d_n6, assign17190_e30140_d_n7, assign17190_e30140_d_n8, assign17190_e30140_d_n9, assign17190_e30140_d_n10, assign17190_e30140_d_n11, assign17190_e30140_d_n13, assign17190_e30140_d_n14,) = {
    if (((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 == 0.0)) && (var_guard306 != 0.0)) {
        let assign17190_e30101: f64 = (var_t4 + var_t5);
        let assign17190_e30104: f64 = (var_t4 - var_t5);
        let assign17190_e30107: f64 = (var_t4 - var_t5);
        let assign17190_e30108: f64 = (assign17190_e30104 * assign17190_e30107);
        let assign17190_e30111: f64 = (0.25 * var_sprt_i);
        let assign17190_e30113: f64 = (assign17190_e30111 * var_sprt_i);
        let assign17190_e30114: f64 = (assign17190_e30108 + assign17190_e30113);
        let assign17190_e30115: f64 = (assign17190_e30114).sqrt();
        let assign17190_e30116: f64 = (assign17190_e30101 + assign17190_e30115);
        let assign17190_e30117: f64 = (0.5 * assign17190_e30116);
        let assign17190_e30121: f64 = var_t3;
        let assign17190_e30124: f64 = var_t3;
        let assign17190_e30127: f64 = var_t3;
        let assign17190_e30128: f64 = (assign17190_e30124 * assign17190_e30127);
        let assign17190_e30131: f64 = (0.25 * var_sprt_i);
        let assign17190_e30133: f64 = (assign17190_e30131 * var_sprt_i);
        let assign17190_e30134: f64 = (assign17190_e30128 + assign17190_e30133);
        let assign17190_e30135: f64 = (assign17190_e30134).sqrt();
        let assign17190_e30136: f64 = (assign17190_e30121 + assign17190_e30135);
        let assign17190_e30137: f64 = (0.5 * assign17190_e30136);
        let assign17190_e30138: f64 = (assign17190_e30117 - assign17190_e30137);
        (assign17190_e30138, ((0.5 * ((var_t4_dn0 + var_t5_dn0) + ((((var_t4_dn0 - var_t5_dn0) * assign17190_e30107) + (assign17190_e30104 * (var_t4_dn0 - var_t5_dn0))) / (2.0 * assign17190_e30115)))) - (0.5 * (var_t3_dn0 + (((var_t3_dn0 * assign17190_e30127) + (assign17190_e30124 * var_t3_dn0)) / (2.0 * assign17190_e30135))))), ((0.5 * ((var_t4_dn2 + var_t5_dn2) + ((((var_t4_dn2 - var_t5_dn2) * assign17190_e30107) + (assign17190_e30104 * (var_t4_dn2 - var_t5_dn2))) / (2.0 * assign17190_e30115)))) - (0.5 * (var_t3_dn2 + (((var_t3_dn2 * assign17190_e30127) + (assign17190_e30124 * var_t3_dn2)) / (2.0 * assign17190_e30135))))), ((0.5 * ((var_t4_dn3 + var_t5_dn3) + ((((var_t4_dn3 - var_t5_dn3) * assign17190_e30107) + (assign17190_e30104 * (var_t4_dn3 - var_t5_dn3))) / (2.0 * assign17190_e30115)))) - (0.5 * (var_t3_dn3 + (((var_t3_dn3 * assign17190_e30127) + (assign17190_e30124 * var_t3_dn3)) / (2.0 * assign17190_e30135))))), ((0.5 * ((var_t4_dn4 + var_t5_dn4) + ((((var_t4_dn4 - var_t5_dn4) * assign17190_e30107) + (assign17190_e30104 * (var_t4_dn4 - var_t5_dn4))) / (2.0 * assign17190_e30115)))) - (0.5 * (var_t3_dn4 + (((var_t3_dn4 * assign17190_e30127) + (assign17190_e30124 * var_t3_dn4)) / (2.0 * assign17190_e30135))))), ((0.5 * ((var_t4_dn5 + var_t5_dn5) + ((((var_t4_dn5 - var_t5_dn5) * assign17190_e30107) + (assign17190_e30104 * (var_t4_dn5 - var_t5_dn5))) / (2.0 * assign17190_e30115)))) - (0.5 * (var_t3_dn5 + (((var_t3_dn5 * assign17190_e30127) + (assign17190_e30124 * var_t3_dn5)) / (2.0 * assign17190_e30135))))), ((0.5 * ((var_t4_dn6 + var_t5_dn6) + ((((var_t4_dn6 - var_t5_dn6) * assign17190_e30107) + (assign17190_e30104 * (var_t4_dn6 - var_t5_dn6))) / (2.0 * assign17190_e30115)))) - (0.5 * (var_t3_dn6 + (((var_t3_dn6 * assign17190_e30127) + (assign17190_e30124 * var_t3_dn6)) / (2.0 * assign17190_e30135))))), ((0.5 * ((var_t4_dn7 + var_t5_dn7) + ((((var_t4_dn7 - var_t5_dn7) * assign17190_e30107) + (assign17190_e30104 * (var_t4_dn7 - var_t5_dn7))) / (2.0 * assign17190_e30115)))) - (0.5 * (var_t3_dn7 + (((var_t3_dn7 * assign17190_e30127) + (assign17190_e30124 * var_t3_dn7)) / (2.0 * assign17190_e30135))))), ((0.5 * ((var_t4_dn8 + var_t5_dn8) + ((((var_t4_dn8 - var_t5_dn8) * assign17190_e30107) + (assign17190_e30104 * (var_t4_dn8 - var_t5_dn8))) / (2.0 * assign17190_e30115)))) - (0.5 * (var_t3_dn8 + (((var_t3_dn8 * assign17190_e30127) + (assign17190_e30124 * var_t3_dn8)) / (2.0 * assign17190_e30135))))), ((0.5 * ((var_t4_dn9 + var_t5_dn9) + ((((var_t4_dn9 - var_t5_dn9) * assign17190_e30107) + (assign17190_e30104 * (var_t4_dn9 - var_t5_dn9))) / (2.0 * assign17190_e30115)))) - (0.5 * (var_t3_dn9 + (((var_t3_dn9 * assign17190_e30127) + (assign17190_e30124 * var_t3_dn9)) / (2.0 * assign17190_e30135))))), ((0.5 * ((var_t4_dn10 + var_t5_dn10) + ((((var_t4_dn10 - var_t5_dn10) * assign17190_e30107) + (assign17190_e30104 * (var_t4_dn10 - var_t5_dn10))) / (2.0 * assign17190_e30115)))) - (0.5 * (var_t3_dn10 + (((var_t3_dn10 * assign17190_e30127) + (assign17190_e30124 * var_t3_dn10)) / (2.0 * assign17190_e30135))))), ((0.5 * ((var_t4_dn11 + var_t5_dn11) + ((((var_t4_dn11 - var_t5_dn11) * assign17190_e30107) + (assign17190_e30104 * (var_t4_dn11 - var_t5_dn11))) / (2.0 * assign17190_e30115)))) - (0.5 * (var_t3_dn11 + (((var_t3_dn11 * assign17190_e30127) + (assign17190_e30124 * var_t3_dn11)) / (2.0 * assign17190_e30135))))), ((0.5 * ((var_t4_dn13 + var_t5_dn13) + ((((var_t4_dn13 - var_t5_dn13) * assign17190_e30107) + (assign17190_e30104 * (var_t4_dn13 - var_t5_dn13))) / (2.0 * assign17190_e30115)))) - (0.5 * (var_t3_dn13 + (((var_t3_dn13 * assign17190_e30127) + (assign17190_e30124 * var_t3_dn13)) / (2.0 * assign17190_e30135))))), ((0.5 * ((var_t4_dn14 + var_t5_dn14) + ((((var_t4_dn14 - var_t5_dn14) * assign17190_e30107) + (assign17190_e30104 * (var_t4_dn14 - var_t5_dn14))) / (2.0 * assign17190_e30115)))) - (0.5 * (var_t3_dn14 + (((var_t3_dn14 * assign17190_e30127) + (assign17190_e30124 * var_t3_dn14)) / (2.0 * assign17190_e30135))))),)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn3, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn11, var_t7_dn13, var_t7_dn14,)
    }
};
        var_t7 = assign17190_e30140;
        var_t7_dn0 = assign17190_e30140_d_n0;
        var_t7_dn2 = assign17190_e30140_d_n2;
        var_t7_dn3 = assign17190_e30140_d_n3;
        var_t7_dn4 = assign17190_e30140_d_n4;
        var_t7_dn5 = assign17190_e30140_d_n5;
        var_t7_dn6 = assign17190_e30140_d_n6;
        var_t7_dn7 = assign17190_e30140_d_n7;
        var_t7_dn8 = assign17190_e30140_d_n8;
        var_t7_dn9 = assign17190_e30140_d_n9;
        var_t7_dn10 = assign17190_e30140_d_n10;
        var_t7_dn11 = assign17190_e30140_d_n11;
        var_t7_dn13 = assign17190_e30140_d_n13;
        var_t7_dn14 = assign17190_e30140_d_n14;
        var_t7_rv = 0.0;

        let (assign17200_e30167, assign17200_e30167_d_n0, assign17200_e30167_d_n2, assign17200_e30167_d_n3, assign17200_e30167_d_n4, assign17200_e30167_d_n5, assign17200_e30167_d_n6, assign17200_e30167_d_n7, assign17200_e30167_d_n8, assign17200_e30167_d_n9, assign17200_e30167_d_n10, assign17200_e30167_d_n11, assign17200_e30167_d_n13, assign17200_e30167_d_n14,) = {
    if (((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 == 0.0)) && (var_guard306 != 0.0)) {
        let assign17200_e30163: f64 = (var_devtemp - 210.0);
        let assign17200_e30164: f64 = (var_prt_i * assign17200_e30163);
        let assign17200_e30165: f64 = (var_t7 + assign17200_e30164);
        (assign17200_e30165, var_t7_dn0, var_t7_dn2, var_t7_dn3, (var_t7_dn4 + (var_prt_i * var_devtemp_dn4)), var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn11, var_t7_dn13, var_t7_dn14,)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn3, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8, var_t8_dn9, var_t8_dn10, var_t8_dn11, var_t8_dn13, var_t8_dn14,)
    }
};
        var_t8 = assign17200_e30167;
        var_t8_dn0 = assign17200_e30167_d_n0;
        var_t8_dn2 = assign17200_e30167_d_n2;
        var_t8_dn3 = assign17200_e30167_d_n3;
        var_t8_dn4 = assign17200_e30167_d_n4;
        var_t8_dn5 = assign17200_e30167_d_n5;
        var_t8_dn6 = assign17200_e30167_d_n6;
        var_t8_dn7 = assign17200_e30167_d_n7;
        var_t8_dn8 = assign17200_e30167_d_n8;
        var_t8_dn9 = assign17200_e30167_d_n9;
        var_t8_dn10 = assign17200_e30167_d_n10;
        var_t8_dn11 = assign17200_e30167_d_n11;
        var_t8_dn13 = assign17200_e30167_d_n13;
        var_t8_dn14 = assign17200_e30167_d_n14;
        var_t8_rv = 0.0;

        let (assign17210_e30207, assign17210_e30207_d_n0, assign17210_e30207_d_n2, assign17210_e30207_d_n3, assign17210_e30207_d_n4, assign17210_e30207_d_n5, assign17210_e30207_d_n6, assign17210_e30207_d_n7, assign17210_e30207_d_n8, assign17210_e30207_d_n9, assign17210_e30207_d_n10, assign17210_e30207_d_n11, assign17210_e30207_d_n13, assign17210_e30207_d_n14,) = {
    if (((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 == 0.0)) && (var_guard306 != 0.0)) {
        let assign17210_e30189: f64 = (var_t6 + var_t8);
        let assign17210_e30192: f64 = (var_t6 - var_t8);
        let assign17210_e30195: f64 = (var_t6 - var_t8);
        let assign17210_e30196: f64 = (assign17210_e30192 * assign17210_e30195);
        let assign17210_e30199: f64 = (0.25 * 0.001);
        let assign17210_e30201: f64 = (assign17210_e30199 * 0.001);
        let assign17210_e30202: f64 = (assign17210_e30196 + assign17210_e30201);
        let assign17210_e30203: f64 = (assign17210_e30202).sqrt();
        let assign17210_e30204: f64 = (assign17210_e30189 + assign17210_e30203);
        let assign17210_e30205: f64 = (0.5 * assign17210_e30204);
        (assign17210_e30205, (0.5 * ((var_t6_dn0 + var_t8_dn0) + ((((var_t6_dn0 - var_t8_dn0) * assign17210_e30195) + (assign17210_e30192 * (var_t6_dn0 - var_t8_dn0))) / (2.0 * assign17210_e30203)))), (0.5 * ((var_t6_dn2 + var_t8_dn2) + ((((var_t6_dn2 - var_t8_dn2) * assign17210_e30195) + (assign17210_e30192 * (var_t6_dn2 - var_t8_dn2))) / (2.0 * assign17210_e30203)))), (0.5 * ((var_t6_dn3 + var_t8_dn3) + ((((var_t6_dn3 - var_t8_dn3) * assign17210_e30195) + (assign17210_e30192 * (var_t6_dn3 - var_t8_dn3))) / (2.0 * assign17210_e30203)))), (0.5 * ((var_t6_dn4 + var_t8_dn4) + ((((var_t6_dn4 - var_t8_dn4) * assign17210_e30195) + (assign17210_e30192 * (var_t6_dn4 - var_t8_dn4))) / (2.0 * assign17210_e30203)))), (0.5 * ((var_t6_dn5 + var_t8_dn5) + ((((var_t6_dn5 - var_t8_dn5) * assign17210_e30195) + (assign17210_e30192 * (var_t6_dn5 - var_t8_dn5))) / (2.0 * assign17210_e30203)))), (0.5 * ((var_t6_dn6 + var_t8_dn6) + ((((var_t6_dn6 - var_t8_dn6) * assign17210_e30195) + (assign17210_e30192 * (var_t6_dn6 - var_t8_dn6))) / (2.0 * assign17210_e30203)))), (0.5 * ((var_t6_dn7 + var_t8_dn7) + ((((var_t6_dn7 - var_t8_dn7) * assign17210_e30195) + (assign17210_e30192 * (var_t6_dn7 - var_t8_dn7))) / (2.0 * assign17210_e30203)))), (0.5 * ((var_t6_dn8 + var_t8_dn8) + ((((var_t6_dn8 - var_t8_dn8) * assign17210_e30195) + (assign17210_e30192 * (var_t6_dn8 - var_t8_dn8))) / (2.0 * assign17210_e30203)))), (0.5 * ((var_t6_dn9 + var_t8_dn9) + ((((var_t6_dn9 - var_t8_dn9) * assign17210_e30195) + (assign17210_e30192 * (var_t6_dn9 - var_t8_dn9))) / (2.0 * assign17210_e30203)))), (0.5 * ((var_t6_dn10 + var_t8_dn10) + ((((var_t6_dn10 - var_t8_dn10) * assign17210_e30195) + (assign17210_e30192 * (var_t6_dn10 - var_t8_dn10))) / (2.0 * assign17210_e30203)))), (0.5 * ((var_t6_dn11 + var_t8_dn11) + ((((var_t6_dn11 - var_t8_dn11) * assign17210_e30195) + (assign17210_e30192 * (var_t6_dn11 - var_t8_dn11))) / (2.0 * assign17210_e30203)))), (0.5 * ((var_t6_dn13 + var_t8_dn13) + ((((var_t6_dn13 - var_t8_dn13) * assign17210_e30195) + (assign17210_e30192 * (var_t6_dn13 - var_t8_dn13))) / (2.0 * assign17210_e30203)))), (0.5 * ((var_t6_dn14 + var_t8_dn14) + ((((var_t6_dn14 - var_t8_dn14) * assign17210_e30195) + (assign17210_e30192 * (var_t6_dn14 - var_t8_dn14))) / (2.0 * assign17210_e30203)))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    }
};
        var_t2 = assign17210_e30207;
        var_t2_dn0 = assign17210_e30207_d_n0;
        var_t2_dn2 = assign17210_e30207_d_n2;
        var_t2_dn3 = assign17210_e30207_d_n3;
        var_t2_dn4 = assign17210_e30207_d_n4;
        var_t2_dn5 = assign17210_e30207_d_n5;
        var_t2_dn6 = assign17210_e30207_d_n6;
        var_t2_dn7 = assign17210_e30207_d_n7;
        var_t2_dn8 = assign17210_e30207_d_n8;
        var_t2_dn9 = assign17210_e30207_d_n9;
        var_t2_dn10 = assign17210_e30207_d_n10;
        var_t2_dn11 = assign17210_e30207_d_n11;
        var_t2_dn13 = assign17210_e30207_d_n13;
        var_t2_dn14 = assign17210_e30207_d_n14;
        var_t2_rv = 0.0;

        let (assign17220_e30269, assign17220_e30269_d_n0, assign17220_e30269_d_n2, assign17220_e30269_d_n3, assign17220_e30269_d_n4, assign17220_e30269_d_n5, assign17220_e30269_d_n6, assign17220_e30269_d_n7, assign17220_e30269_d_n8, assign17220_e30269_d_n9, assign17220_e30269_d_n10, assign17220_e30269_d_n11, assign17220_e30269_d_n13, assign17220_e30269_d_n14,) = {
    if (((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 == 0.0)) && (var_guard306 == 0.0)) {
        let assign17220_e30230: f64 = (var_rdstemp0 + var_rdstemp1);
        let assign17220_e30233: f64 = (var_rdstemp0 - var_rdstemp1);
        let assign17220_e30236: f64 = (var_rdstemp0 - var_rdstemp1);
        let assign17220_e30237: f64 = (assign17220_e30233 * assign17220_e30236);
        let assign17220_e30240: f64 = (0.25 * var_sprt_i);
        let assign17220_e30242: f64 = (assign17220_e30240 * var_sprt_i);
        let assign17220_e30243: f64 = (assign17220_e30237 + assign17220_e30242);
        let assign17220_e30244: f64 = (assign17220_e30243).sqrt();
        let assign17220_e30245: f64 = (assign17220_e30230 - assign17220_e30244);
        let assign17220_e30246: f64 = (0.5 * assign17220_e30245);
        let assign17220_e30250: f64 = var_t3;
        let assign17220_e30253: f64 = var_t3;
        let assign17220_e30256: f64 = var_t3;
        let assign17220_e30257: f64 = (assign17220_e30253 * assign17220_e30256);
        let assign17220_e30260: f64 = (0.25 * var_sprt_i);
        let assign17220_e30262: f64 = (assign17220_e30260 * var_sprt_i);
        let assign17220_e30263: f64 = (assign17220_e30257 + assign17220_e30262);
        let assign17220_e30264: f64 = (assign17220_e30263).sqrt();
        let assign17220_e30265: f64 = (assign17220_e30250 - assign17220_e30264);
        let assign17220_e30266: f64 = (0.5 * assign17220_e30265);
        let assign17220_e30267: f64 = (assign17220_e30246 - assign17220_e30266);
        (assign17220_e30267, (-(0.5 * (var_t3_dn0 - (((var_t3_dn0 * assign17220_e30256) + (assign17220_e30253 * var_t3_dn0)) / (2.0 * assign17220_e30264))))), (-(0.5 * (var_t3_dn2 - (((var_t3_dn2 * assign17220_e30256) + (assign17220_e30253 * var_t3_dn2)) / (2.0 * assign17220_e30264))))), (-(0.5 * (var_t3_dn3 - (((var_t3_dn3 * assign17220_e30256) + (assign17220_e30253 * var_t3_dn3)) / (2.0 * assign17220_e30264))))), ((0.5 * ((var_rdstemp0_dn4 + var_rdstemp1_dn4) - ((((var_rdstemp0_dn4 - var_rdstemp1_dn4) * assign17220_e30236) + (assign17220_e30233 * (var_rdstemp0_dn4 - var_rdstemp1_dn4))) / (2.0 * assign17220_e30244)))) - (0.5 * (var_t3_dn4 - (((var_t3_dn4 * assign17220_e30256) + (assign17220_e30253 * var_t3_dn4)) / (2.0 * assign17220_e30264))))), (-(0.5 * (var_t3_dn5 - (((var_t3_dn5 * assign17220_e30256) + (assign17220_e30253 * var_t3_dn5)) / (2.0 * assign17220_e30264))))), (-(0.5 * (var_t3_dn6 - (((var_t3_dn6 * assign17220_e30256) + (assign17220_e30253 * var_t3_dn6)) / (2.0 * assign17220_e30264))))), (-(0.5 * (var_t3_dn7 - (((var_t3_dn7 * assign17220_e30256) + (assign17220_e30253 * var_t3_dn7)) / (2.0 * assign17220_e30264))))), (-(0.5 * (var_t3_dn8 - (((var_t3_dn8 * assign17220_e30256) + (assign17220_e30253 * var_t3_dn8)) / (2.0 * assign17220_e30264))))), (-(0.5 * (var_t3_dn9 - (((var_t3_dn9 * assign17220_e30256) + (assign17220_e30253 * var_t3_dn9)) / (2.0 * assign17220_e30264))))), (-(0.5 * (var_t3_dn10 - (((var_t3_dn10 * assign17220_e30256) + (assign17220_e30253 * var_t3_dn10)) / (2.0 * assign17220_e30264))))), (-(0.5 * (var_t3_dn11 - (((var_t3_dn11 * assign17220_e30256) + (assign17220_e30253 * var_t3_dn11)) / (2.0 * assign17220_e30264))))), (-(0.5 * (var_t3_dn13 - (((var_t3_dn13 * assign17220_e30256) + (assign17220_e30253 * var_t3_dn13)) / (2.0 * assign17220_e30264))))), (-(0.5 * (var_t3_dn14 - (((var_t3_dn14 * assign17220_e30256) + (assign17220_e30253 * var_t3_dn14)) / (2.0 * assign17220_e30264))))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn13, var_t6_dn14,)
    }
};
        var_t6 = assign17220_e30269;
        var_t6_dn0 = assign17220_e30269_d_n0;
        var_t6_dn2 = assign17220_e30269_d_n2;
        var_t6_dn3 = assign17220_e30269_d_n3;
        var_t6_dn4 = assign17220_e30269_d_n4;
        var_t6_dn5 = assign17220_e30269_d_n5;
        var_t6_dn6 = assign17220_e30269_d_n6;
        var_t6_dn7 = assign17220_e30269_d_n7;
        var_t6_dn8 = assign17220_e30269_d_n8;
        var_t6_dn9 = assign17220_e30269_d_n9;
        var_t6_dn10 = assign17220_e30269_d_n10;
        var_t6_dn11 = assign17220_e30269_d_n11;
        var_t6_dn13 = assign17220_e30269_d_n13;
        var_t6_dn14 = assign17220_e30269_d_n14;
        var_t6_rv = 0.0;

        let (assign17230_e30331, assign17230_e30331_d_n0, assign17230_e30331_d_n2, assign17230_e30331_d_n3, assign17230_e30331_d_n4, assign17230_e30331_d_n5, assign17230_e30331_d_n6, assign17230_e30331_d_n7, assign17230_e30331_d_n8, assign17230_e30331_d_n9, assign17230_e30331_d_n10, assign17230_e30331_d_n11, assign17230_e30331_d_n13, assign17230_e30331_d_n14,) = {
    if (((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 == 0.0)) && (var_guard306 == 0.0)) {
        let assign17230_e30292: f64 = (var_t4 + var_t5);
        let assign17230_e30295: f64 = (var_t4 - var_t5);
        let assign17230_e30298: f64 = (var_t4 - var_t5);
        let assign17230_e30299: f64 = (assign17230_e30295 * assign17230_e30298);
        let assign17230_e30302: f64 = (0.25 * var_sprt_i);
        let assign17230_e30304: f64 = (assign17230_e30302 * var_sprt_i);
        let assign17230_e30305: f64 = (assign17230_e30299 + assign17230_e30304);
        let assign17230_e30306: f64 = (assign17230_e30305).sqrt();
        let assign17230_e30307: f64 = (assign17230_e30292 - assign17230_e30306);
        let assign17230_e30308: f64 = (0.5 * assign17230_e30307);
        let assign17230_e30312: f64 = var_t3;
        let assign17230_e30315: f64 = var_t3;
        let assign17230_e30318: f64 = var_t3;
        let assign17230_e30319: f64 = (assign17230_e30315 * assign17230_e30318);
        let assign17230_e30322: f64 = (0.25 * var_sprt_i);
        let assign17230_e30324: f64 = (assign17230_e30322 * var_sprt_i);
        let assign17230_e30325: f64 = (assign17230_e30319 + assign17230_e30324);
        let assign17230_e30326: f64 = (assign17230_e30325).sqrt();
        let assign17230_e30327: f64 = (assign17230_e30312 - assign17230_e30326);
        let assign17230_e30328: f64 = (0.5 * assign17230_e30327);
        let assign17230_e30329: f64 = (assign17230_e30308 - assign17230_e30328);
        (assign17230_e30329, ((0.5 * ((var_t4_dn0 + var_t5_dn0) - ((((var_t4_dn0 - var_t5_dn0) * assign17230_e30298) + (assign17230_e30295 * (var_t4_dn0 - var_t5_dn0))) / (2.0 * assign17230_e30306)))) - (0.5 * (var_t3_dn0 - (((var_t3_dn0 * assign17230_e30318) + (assign17230_e30315 * var_t3_dn0)) / (2.0 * assign17230_e30326))))), ((0.5 * ((var_t4_dn2 + var_t5_dn2) - ((((var_t4_dn2 - var_t5_dn2) * assign17230_e30298) + (assign17230_e30295 * (var_t4_dn2 - var_t5_dn2))) / (2.0 * assign17230_e30306)))) - (0.5 * (var_t3_dn2 - (((var_t3_dn2 * assign17230_e30318) + (assign17230_e30315 * var_t3_dn2)) / (2.0 * assign17230_e30326))))), ((0.5 * ((var_t4_dn3 + var_t5_dn3) - ((((var_t4_dn3 - var_t5_dn3) * assign17230_e30298) + (assign17230_e30295 * (var_t4_dn3 - var_t5_dn3))) / (2.0 * assign17230_e30306)))) - (0.5 * (var_t3_dn3 - (((var_t3_dn3 * assign17230_e30318) + (assign17230_e30315 * var_t3_dn3)) / (2.0 * assign17230_e30326))))), ((0.5 * ((var_t4_dn4 + var_t5_dn4) - ((((var_t4_dn4 - var_t5_dn4) * assign17230_e30298) + (assign17230_e30295 * (var_t4_dn4 - var_t5_dn4))) / (2.0 * assign17230_e30306)))) - (0.5 * (var_t3_dn4 - (((var_t3_dn4 * assign17230_e30318) + (assign17230_e30315 * var_t3_dn4)) / (2.0 * assign17230_e30326))))), ((0.5 * ((var_t4_dn5 + var_t5_dn5) - ((((var_t4_dn5 - var_t5_dn5) * assign17230_e30298) + (assign17230_e30295 * (var_t4_dn5 - var_t5_dn5))) / (2.0 * assign17230_e30306)))) - (0.5 * (var_t3_dn5 - (((var_t3_dn5 * assign17230_e30318) + (assign17230_e30315 * var_t3_dn5)) / (2.0 * assign17230_e30326))))), ((0.5 * ((var_t4_dn6 + var_t5_dn6) - ((((var_t4_dn6 - var_t5_dn6) * assign17230_e30298) + (assign17230_e30295 * (var_t4_dn6 - var_t5_dn6))) / (2.0 * assign17230_e30306)))) - (0.5 * (var_t3_dn6 - (((var_t3_dn6 * assign17230_e30318) + (assign17230_e30315 * var_t3_dn6)) / (2.0 * assign17230_e30326))))), ((0.5 * ((var_t4_dn7 + var_t5_dn7) - ((((var_t4_dn7 - var_t5_dn7) * assign17230_e30298) + (assign17230_e30295 * (var_t4_dn7 - var_t5_dn7))) / (2.0 * assign17230_e30306)))) - (0.5 * (var_t3_dn7 - (((var_t3_dn7 * assign17230_e30318) + (assign17230_e30315 * var_t3_dn7)) / (2.0 * assign17230_e30326))))), ((0.5 * ((var_t4_dn8 + var_t5_dn8) - ((((var_t4_dn8 - var_t5_dn8) * assign17230_e30298) + (assign17230_e30295 * (var_t4_dn8 - var_t5_dn8))) / (2.0 * assign17230_e30306)))) - (0.5 * (var_t3_dn8 - (((var_t3_dn8 * assign17230_e30318) + (assign17230_e30315 * var_t3_dn8)) / (2.0 * assign17230_e30326))))), ((0.5 * ((var_t4_dn9 + var_t5_dn9) - ((((var_t4_dn9 - var_t5_dn9) * assign17230_e30298) + (assign17230_e30295 * (var_t4_dn9 - var_t5_dn9))) / (2.0 * assign17230_e30306)))) - (0.5 * (var_t3_dn9 - (((var_t3_dn9 * assign17230_e30318) + (assign17230_e30315 * var_t3_dn9)) / (2.0 * assign17230_e30326))))), ((0.5 * ((var_t4_dn10 + var_t5_dn10) - ((((var_t4_dn10 - var_t5_dn10) * assign17230_e30298) + (assign17230_e30295 * (var_t4_dn10 - var_t5_dn10))) / (2.0 * assign17230_e30306)))) - (0.5 * (var_t3_dn10 - (((var_t3_dn10 * assign17230_e30318) + (assign17230_e30315 * var_t3_dn10)) / (2.0 * assign17230_e30326))))), ((0.5 * ((var_t4_dn11 + var_t5_dn11) - ((((var_t4_dn11 - var_t5_dn11) * assign17230_e30298) + (assign17230_e30295 * (var_t4_dn11 - var_t5_dn11))) / (2.0 * assign17230_e30306)))) - (0.5 * (var_t3_dn11 - (((var_t3_dn11 * assign17230_e30318) + (assign17230_e30315 * var_t3_dn11)) / (2.0 * assign17230_e30326))))), ((0.5 * ((var_t4_dn13 + var_t5_dn13) - ((((var_t4_dn13 - var_t5_dn13) * assign17230_e30298) + (assign17230_e30295 * (var_t4_dn13 - var_t5_dn13))) / (2.0 * assign17230_e30306)))) - (0.5 * (var_t3_dn13 - (((var_t3_dn13 * assign17230_e30318) + (assign17230_e30315 * var_t3_dn13)) / (2.0 * assign17230_e30326))))), ((0.5 * ((var_t4_dn14 + var_t5_dn14) - ((((var_t4_dn14 - var_t5_dn14) * assign17230_e30298) + (assign17230_e30295 * (var_t4_dn14 - var_t5_dn14))) / (2.0 * assign17230_e30306)))) - (0.5 * (var_t3_dn14 - (((var_t3_dn14 * assign17230_e30318) + (assign17230_e30315 * var_t3_dn14)) / (2.0 * assign17230_e30326))))),)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn3, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn11, var_t7_dn13, var_t7_dn14,)
    }
};
        var_t7 = assign17230_e30331;
        var_t7_dn0 = assign17230_e30331_d_n0;
        var_t7_dn2 = assign17230_e30331_d_n2;
        var_t7_dn3 = assign17230_e30331_d_n3;
        var_t7_dn4 = assign17230_e30331_d_n4;
        var_t7_dn5 = assign17230_e30331_d_n5;
        var_t7_dn6 = assign17230_e30331_d_n6;
        var_t7_dn7 = assign17230_e30331_d_n7;
        var_t7_dn8 = assign17230_e30331_d_n8;
        var_t7_dn9 = assign17230_e30331_d_n9;
        var_t7_dn10 = assign17230_e30331_d_n10;
        var_t7_dn11 = assign17230_e30331_d_n11;
        var_t7_dn13 = assign17230_e30331_d_n13;
        var_t7_dn14 = assign17230_e30331_d_n14;
        var_t7_rv = 0.0;

        let (assign17240_e30359, assign17240_e30359_d_n0, assign17240_e30359_d_n2, assign17240_e30359_d_n3, assign17240_e30359_d_n4, assign17240_e30359_d_n5, assign17240_e30359_d_n6, assign17240_e30359_d_n7, assign17240_e30359_d_n8, assign17240_e30359_d_n9, assign17240_e30359_d_n10, assign17240_e30359_d_n11, assign17240_e30359_d_n13, assign17240_e30359_d_n14,) = {
    if (((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 == 0.0)) && (var_guard306 == 0.0)) {
        let assign17240_e30355: f64 = (var_devtemp - 210.0);
        let assign17240_e30356: f64 = (var_prt_i * assign17240_e30355);
        let assign17240_e30357: f64 = (var_t7 + assign17240_e30356);
        (assign17240_e30357, var_t7_dn0, var_t7_dn2, var_t7_dn3, (var_t7_dn4 + (var_prt_i * var_devtemp_dn4)), var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn11, var_t7_dn13, var_t7_dn14,)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn3, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8, var_t8_dn9, var_t8_dn10, var_t8_dn11, var_t8_dn13, var_t8_dn14,)
    }
};
        var_t8 = assign17240_e30359;
        var_t8_dn0 = assign17240_e30359_d_n0;
        var_t8_dn2 = assign17240_e30359_d_n2;
        var_t8_dn3 = assign17240_e30359_d_n3;
        var_t8_dn4 = assign17240_e30359_d_n4;
        var_t8_dn5 = assign17240_e30359_d_n5;
        var_t8_dn6 = assign17240_e30359_d_n6;
        var_t8_dn7 = assign17240_e30359_d_n7;
        var_t8_dn8 = assign17240_e30359_d_n8;
        var_t8_dn9 = assign17240_e30359_d_n9;
        var_t8_dn10 = assign17240_e30359_d_n10;
        var_t8_dn11 = assign17240_e30359_d_n11;
        var_t8_dn13 = assign17240_e30359_d_n13;
        var_t8_dn14 = assign17240_e30359_d_n14;
        var_t8_rv = 0.0;

        let (assign17250_e30400, assign17250_e30400_d_n0, assign17250_e30400_d_n2, assign17250_e30400_d_n3, assign17250_e30400_d_n4, assign17250_e30400_d_n5, assign17250_e30400_d_n6, assign17250_e30400_d_n7, assign17250_e30400_d_n8, assign17250_e30400_d_n9, assign17250_e30400_d_n10, assign17250_e30400_d_n11, assign17250_e30400_d_n13, assign17250_e30400_d_n14,) = {
    if (((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 != 0.0)) && (var_guard302 == 0.0)) && (var_guard304 == 0.0)) && (var_guard306 == 0.0)) {
        let assign17250_e30382: f64 = (var_t6 + var_t8);
        let assign17250_e30385: f64 = (var_t6 - var_t8);
        let assign17250_e30388: f64 = (var_t6 - var_t8);
        let assign17250_e30389: f64 = (assign17250_e30385 * assign17250_e30388);
        let assign17250_e30392: f64 = (0.25 * 0.001);
        let assign17250_e30394: f64 = (assign17250_e30392 * 0.001);
        let assign17250_e30395: f64 = (assign17250_e30389 + assign17250_e30394);
        let assign17250_e30396: f64 = (assign17250_e30395).sqrt();
        let assign17250_e30397: f64 = (assign17250_e30382 - assign17250_e30396);
        let assign17250_e30398: f64 = (0.5 * assign17250_e30397);
        (assign17250_e30398, (0.5 * ((var_t6_dn0 + var_t8_dn0) - ((((var_t6_dn0 - var_t8_dn0) * assign17250_e30388) + (assign17250_e30385 * (var_t6_dn0 - var_t8_dn0))) / (2.0 * assign17250_e30396)))), (0.5 * ((var_t6_dn2 + var_t8_dn2) - ((((var_t6_dn2 - var_t8_dn2) * assign17250_e30388) + (assign17250_e30385 * (var_t6_dn2 - var_t8_dn2))) / (2.0 * assign17250_e30396)))), (0.5 * ((var_t6_dn3 + var_t8_dn3) - ((((var_t6_dn3 - var_t8_dn3) * assign17250_e30388) + (assign17250_e30385 * (var_t6_dn3 - var_t8_dn3))) / (2.0 * assign17250_e30396)))), (0.5 * ((var_t6_dn4 + var_t8_dn4) - ((((var_t6_dn4 - var_t8_dn4) * assign17250_e30388) + (assign17250_e30385 * (var_t6_dn4 - var_t8_dn4))) / (2.0 * assign17250_e30396)))), (0.5 * ((var_t6_dn5 + var_t8_dn5) - ((((var_t6_dn5 - var_t8_dn5) * assign17250_e30388) + (assign17250_e30385 * (var_t6_dn5 - var_t8_dn5))) / (2.0 * assign17250_e30396)))), (0.5 * ((var_t6_dn6 + var_t8_dn6) - ((((var_t6_dn6 - var_t8_dn6) * assign17250_e30388) + (assign17250_e30385 * (var_t6_dn6 - var_t8_dn6))) / (2.0 * assign17250_e30396)))), (0.5 * ((var_t6_dn7 + var_t8_dn7) - ((((var_t6_dn7 - var_t8_dn7) * assign17250_e30388) + (assign17250_e30385 * (var_t6_dn7 - var_t8_dn7))) / (2.0 * assign17250_e30396)))), (0.5 * ((var_t6_dn8 + var_t8_dn8) - ((((var_t6_dn8 - var_t8_dn8) * assign17250_e30388) + (assign17250_e30385 * (var_t6_dn8 - var_t8_dn8))) / (2.0 * assign17250_e30396)))), (0.5 * ((var_t6_dn9 + var_t8_dn9) - ((((var_t6_dn9 - var_t8_dn9) * assign17250_e30388) + (assign17250_e30385 * (var_t6_dn9 - var_t8_dn9))) / (2.0 * assign17250_e30396)))), (0.5 * ((var_t6_dn10 + var_t8_dn10) - ((((var_t6_dn10 - var_t8_dn10) * assign17250_e30388) + (assign17250_e30385 * (var_t6_dn10 - var_t8_dn10))) / (2.0 * assign17250_e30396)))), (0.5 * ((var_t6_dn11 + var_t8_dn11) - ((((var_t6_dn11 - var_t8_dn11) * assign17250_e30388) + (assign17250_e30385 * (var_t6_dn11 - var_t8_dn11))) / (2.0 * assign17250_e30396)))), (0.5 * ((var_t6_dn13 + var_t8_dn13) - ((((var_t6_dn13 - var_t8_dn13) * assign17250_e30388) + (assign17250_e30385 * (var_t6_dn13 - var_t8_dn13))) / (2.0 * assign17250_e30396)))), (0.5 * ((var_t6_dn14 + var_t8_dn14) - ((((var_t6_dn14 - var_t8_dn14) * assign17250_e30388) + (assign17250_e30385 * (var_t6_dn14 - var_t8_dn14))) / (2.0 * assign17250_e30396)))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    }
};
        var_t2 = assign17250_e30400;
        var_t2_dn0 = assign17250_e30400_d_n0;
        var_t2_dn2 = assign17250_e30400_d_n2;
        var_t2_dn3 = assign17250_e30400_d_n3;
        var_t2_dn4 = assign17250_e30400_d_n4;
        var_t2_dn5 = assign17250_e30400_d_n5;
        var_t2_dn6 = assign17250_e30400_d_n6;
        var_t2_dn7 = assign17250_e30400_d_n7;
        var_t2_dn8 = assign17250_e30400_d_n8;
        var_t2_dn9 = assign17250_e30400_d_n9;
        var_t2_dn10 = assign17250_e30400_d_n10;
        var_t2_dn11 = assign17250_e30400_d_n11;
        var_t2_dn13 = assign17250_e30400_d_n13;
        var_t2_dn14 = assign17250_e30400_d_n14;
        var_t2_rv = 0.0;

        let assign17260_e30403: f64 = if var_tnom > 210.0 { 1.0 } else { 0.0 };
        var_guard307 = assign17260_e30403;
        var_guard307_rv = 0.0;

        let (assign17270_e30423, assign17270_e30423_d_n4,) = {
    if (((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 == 0.0)) && (var_guard307 != 0.0)) {
        let assign17270_e30420: f64 = (var_prt_i * var_deltemp);
        let assign17270_e30421: f64 = (1.0 + assign17270_e30420);
        (assign17270_e30421, (var_prt_i * var_deltemp_dn4),)
    } else {
        (var_rdstemp0, var_rdstemp0_dn4,)
    }
};
        var_rdstemp0 = assign17270_e30423;
        var_rdstemp0_dn4 = assign17270_e30423_d_n4;
        var_rdstemp0_rv = 0.0;

        let (assign17280_e30451, assign17280_e30451_d_n4,) = {
    if (((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 == 0.0)) && (var_guard307 != 0.0)) {
        let assign17280_e30441: f64 = (var_devtemp - 210.0);
        let assign17280_e30442: f64 = (var_prt1_i * assign17280_e30441);
        let assign17280_e30443: f64 = (1.0 + assign17280_e30442);
        let assign17280_e30447: f64 = (210.0 - var_tnom);
        let assign17280_e30448: f64 = (var_prt_i * assign17280_e30447);
        let assign17280_e30449: f64 = (assign17280_e30443 + assign17280_e30448);
        (assign17280_e30449, (var_prt1_i * var_devtemp_dn4),)
    } else {
        (var_rdstemp1, var_rdstemp1_dn4,)
    }
};
        var_rdstemp1 = assign17280_e30451;
        var_rdstemp1_dn4 = assign17280_e30451_d_n4;
        var_rdstemp1_rv = 0.0;

        let assign17290_e30454: f64 = if var_prt1_i < var_prt_i { 1.0 } else { 0.0 };
        var_guard308 = assign17290_e30454;
        var_guard308_rv = 0.0;

        let (assign17300_e30491, assign17300_e30491_d_n0, assign17300_e30491_d_n2, assign17300_e30491_d_n3, assign17300_e30491_d_n4, assign17300_e30491_d_n5, assign17300_e30491_d_n6, assign17300_e30491_d_n7, assign17300_e30491_d_n8, assign17300_e30491_d_n9, assign17300_e30491_d_n10, assign17300_e30491_d_n11, assign17300_e30491_d_n13, assign17300_e30491_d_n14,) = {
    if ((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 == 0.0)) && (var_guard307 != 0.0)) && (var_guard308 != 0.0)) {
        let assign17300_e30473: f64 = (var_rdstemp0 + var_rdstemp1);
        let assign17300_e30476: f64 = (var_rdstemp0 - var_rdstemp1);
        let assign17300_e30479: f64 = (var_rdstemp0 - var_rdstemp1);
        let assign17300_e30480: f64 = (assign17300_e30476 * assign17300_e30479);
        let assign17300_e30483: f64 = (0.25 * 0.01);
        let assign17300_e30485: f64 = (assign17300_e30483 * 0.01);
        let assign17300_e30486: f64 = (assign17300_e30480 + assign17300_e30485);
        let assign17300_e30487: f64 = (assign17300_e30486).sqrt();
        let assign17300_e30488: f64 = (assign17300_e30473 + assign17300_e30487);
        let assign17300_e30489: f64 = (0.5 * assign17300_e30488);
        (assign17300_e30489, 0.0, 0.0, 0.0, (0.5 * ((var_rdstemp0_dn4 + var_rdstemp1_dn4) + ((((var_rdstemp0_dn4 - var_rdstemp1_dn4) * assign17300_e30479) + (assign17300_e30476 * (var_rdstemp0_dn4 - var_rdstemp1_dn4))) / (2.0 * assign17300_e30487)))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    }
};
        var_t2 = assign17300_e30491;
        var_t2_dn0 = assign17300_e30491_d_n0;
        var_t2_dn2 = assign17300_e30491_d_n2;
        var_t2_dn3 = assign17300_e30491_d_n3;
        var_t2_dn4 = assign17300_e30491_d_n4;
        var_t2_dn5 = assign17300_e30491_d_n5;
        var_t2_dn6 = assign17300_e30491_d_n6;
        var_t2_dn7 = assign17300_e30491_d_n7;
        var_t2_dn8 = assign17300_e30491_d_n8;
        var_t2_dn9 = assign17300_e30491_d_n9;
        var_t2_dn10 = assign17300_e30491_d_n10;
        var_t2_dn11 = assign17300_e30491_d_n11;
        var_t2_dn13 = assign17300_e30491_d_n13;
        var_t2_dn14 = assign17300_e30491_d_n14;
        var_t2_rv = 0.0;

        let (assign17310_e30529, assign17310_e30529_d_n0, assign17310_e30529_d_n2, assign17310_e30529_d_n3, assign17310_e30529_d_n4, assign17310_e30529_d_n5, assign17310_e30529_d_n6, assign17310_e30529_d_n7, assign17310_e30529_d_n8, assign17310_e30529_d_n9, assign17310_e30529_d_n10, assign17310_e30529_d_n11, assign17310_e30529_d_n13, assign17310_e30529_d_n14,) = {
    if ((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 == 0.0)) && (var_guard307 != 0.0)) && (var_guard308 == 0.0)) {
        let assign17310_e30511: f64 = (var_rdstemp0 + var_rdstemp1);
        let assign17310_e30514: f64 = (var_rdstemp0 - var_rdstemp1);
        let assign17310_e30517: f64 = (var_rdstemp0 - var_rdstemp1);
        let assign17310_e30518: f64 = (assign17310_e30514 * assign17310_e30517);
        let assign17310_e30521: f64 = (0.25 * 0.01);
        let assign17310_e30523: f64 = (assign17310_e30521 * 0.01);
        let assign17310_e30524: f64 = (assign17310_e30518 + assign17310_e30523);
        let assign17310_e30525: f64 = (assign17310_e30524).sqrt();
        let assign17310_e30526: f64 = (assign17310_e30511 - assign17310_e30525);
        let assign17310_e30527: f64 = (0.5 * assign17310_e30526);
        (assign17310_e30527, 0.0, 0.0, 0.0, (0.5 * ((var_rdstemp0_dn4 + var_rdstemp1_dn4) - ((((var_rdstemp0_dn4 - var_rdstemp1_dn4) * assign17310_e30517) + (assign17310_e30514 * (var_rdstemp0_dn4 - var_rdstemp1_dn4))) / (2.0 * assign17310_e30525)))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    }
};
        var_t2 = assign17310_e30529;
        var_t2_dn0 = assign17310_e30529_d_n0;
        var_t2_dn2 = assign17310_e30529_d_n2;
        var_t2_dn3 = assign17310_e30529_d_n3;
        var_t2_dn4 = assign17310_e30529_d_n4;
        var_t2_dn5 = assign17310_e30529_d_n5;
        var_t2_dn6 = assign17310_e30529_d_n6;
        var_t2_dn7 = assign17310_e30529_d_n7;
        var_t2_dn8 = assign17310_e30529_d_n8;
        var_t2_dn9 = assign17310_e30529_d_n9;
        var_t2_dn10 = assign17310_e30529_d_n10;
        var_t2_dn11 = assign17310_e30529_d_n11;
        var_t2_dn13 = assign17310_e30529_d_n13;
        var_t2_dn14 = assign17310_e30529_d_n14;
        var_t2_rv = 0.0;

        let (assign17320_e30550, assign17320_e30550_d_n4,) = {
    if (((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 == 0.0)) && (var_guard307 == 0.0)) {
        let assign17320_e30547: f64 = (var_prt1_i * var_deltemp);
        let assign17320_e30548: f64 = (1.0 + assign17320_e30547);
        (assign17320_e30548, (var_prt1_i * var_deltemp_dn4),)
    } else {
        (var_rdstemp1, var_rdstemp1_dn4,)
    }
};
        var_rdstemp1 = assign17320_e30550;
        var_rdstemp1_dn4 = assign17320_e30550_d_n4;
        var_rdstemp1_rv = 0.0;

        let (assign17330_e30579, assign17330_e30579_d_n4,) = {
    if (((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 == 0.0)) && (var_guard307 == 0.0)) {
        let assign17330_e30569: f64 = (var_devtemp - 210.0);
        let assign17330_e30570: f64 = (var_prt_i * assign17330_e30569);
        let assign17330_e30571: f64 = (1.0 + assign17330_e30570);
        let assign17330_e30575: f64 = (210.0 - var_tnom);
        let assign17330_e30576: f64 = (var_prt1_i * assign17330_e30575);
        let assign17330_e30577: f64 = (assign17330_e30571 + assign17330_e30576);
        (assign17330_e30577, (var_prt_i * var_devtemp_dn4),)
    } else {
        (var_rdstemp0, var_rdstemp0_dn4,)
    }
};
        var_rdstemp0 = assign17330_e30579;
        var_rdstemp0_dn4 = assign17330_e30579_d_n4;
        var_rdstemp0_rv = 0.0;

        let assign17340_e30582: f64 = if var_prt1_i < var_prt_i { 1.0 } else { 0.0 };
        var_guard309 = assign17340_e30582;
        var_guard309_rv = 0.0;

        let (assign17350_e30620, assign17350_e30620_d_n0, assign17350_e30620_d_n2, assign17350_e30620_d_n3, assign17350_e30620_d_n4, assign17350_e30620_d_n5, assign17350_e30620_d_n6, assign17350_e30620_d_n7, assign17350_e30620_d_n8, assign17350_e30620_d_n9, assign17350_e30620_d_n10, assign17350_e30620_d_n11, assign17350_e30620_d_n13, assign17350_e30620_d_n14,) = {
    if ((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 == 0.0)) && (var_guard307 == 0.0)) && (var_guard309 != 0.0)) {
        let assign17350_e30602: f64 = (var_rdstemp0 + var_rdstemp1);
        let assign17350_e30605: f64 = (var_rdstemp0 - var_rdstemp1);
        let assign17350_e30608: f64 = (var_rdstemp0 - var_rdstemp1);
        let assign17350_e30609: f64 = (assign17350_e30605 * assign17350_e30608);
        let assign17350_e30612: f64 = (0.25 * 0.01);
        let assign17350_e30614: f64 = (assign17350_e30612 * 0.01);
        let assign17350_e30615: f64 = (assign17350_e30609 + assign17350_e30614);
        let assign17350_e30616: f64 = (assign17350_e30615).sqrt();
        let assign17350_e30617: f64 = (assign17350_e30602 + assign17350_e30616);
        let assign17350_e30618: f64 = (0.5 * assign17350_e30617);
        (assign17350_e30618, 0.0, 0.0, 0.0, (0.5 * ((var_rdstemp0_dn4 + var_rdstemp1_dn4) + ((((var_rdstemp0_dn4 - var_rdstemp1_dn4) * assign17350_e30608) + (assign17350_e30605 * (var_rdstemp0_dn4 - var_rdstemp1_dn4))) / (2.0 * assign17350_e30616)))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    }
};
        var_t2 = assign17350_e30620;
        var_t2_dn0 = assign17350_e30620_d_n0;
        var_t2_dn2 = assign17350_e30620_d_n2;
        var_t2_dn3 = assign17350_e30620_d_n3;
        var_t2_dn4 = assign17350_e30620_d_n4;
        var_t2_dn5 = assign17350_e30620_d_n5;
        var_t2_dn6 = assign17350_e30620_d_n6;
        var_t2_dn7 = assign17350_e30620_d_n7;
        var_t2_dn8 = assign17350_e30620_d_n8;
        var_t2_dn9 = assign17350_e30620_d_n9;
        var_t2_dn10 = assign17350_e30620_d_n10;
        var_t2_dn11 = assign17350_e30620_d_n11;
        var_t2_dn13 = assign17350_e30620_d_n13;
        var_t2_dn14 = assign17350_e30620_d_n14;
        var_t2_rv = 0.0;

        let (assign17360_e30659, assign17360_e30659_d_n0, assign17360_e30659_d_n2, assign17360_e30659_d_n3, assign17360_e30659_d_n4, assign17360_e30659_d_n5, assign17360_e30659_d_n6, assign17360_e30659_d_n7, assign17360_e30659_d_n8, assign17360_e30659_d_n9, assign17360_e30659_d_n10, assign17360_e30659_d_n11, assign17360_e30659_d_n13, assign17360_e30659_d_n14,) = {
    if ((((((var_guard244 == 0.0) && (var_guard259 == 0.0)) && (var_guard300 == 0.0)) && (var_guard301 == 0.0)) && (var_guard307 == 0.0)) && (var_guard309 == 0.0)) {
        let assign17360_e30641: f64 = (var_rdstemp0 + var_rdstemp1);
        let assign17360_e30644: f64 = (var_rdstemp0 - var_rdstemp1);
        let assign17360_e30647: f64 = (var_rdstemp0 - var_rdstemp1);
        let assign17360_e30648: f64 = (assign17360_e30644 * assign17360_e30647);
        let assign17360_e30651: f64 = (0.25 * 0.01);
        let assign17360_e30653: f64 = (assign17360_e30651 * 0.01);
        let assign17360_e30654: f64 = (assign17360_e30648 + assign17360_e30653);
        let assign17360_e30655: f64 = (assign17360_e30654).sqrt();
        let assign17360_e30656: f64 = (assign17360_e30641 - assign17360_e30655);
        let assign17360_e30657: f64 = (0.5 * assign17360_e30656);
        (assign17360_e30657, 0.0, 0.0, 0.0, (0.5 * ((var_rdstemp0_dn4 + var_rdstemp1_dn4) - ((((var_rdstemp0_dn4 - var_rdstemp1_dn4) * assign17360_e30647) + (assign17360_e30644 * (var_rdstemp0_dn4 - var_rdstemp1_dn4))) / (2.0 * assign17360_e30655)))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    }
};
        var_t2 = assign17360_e30659;
        var_t2_dn0 = assign17360_e30659_d_n0;
        var_t2_dn2 = assign17360_e30659_d_n2;
        var_t2_dn3 = assign17360_e30659_d_n3;
        var_t2_dn4 = assign17360_e30659_d_n4;
        var_t2_dn5 = assign17360_e30659_d_n5;
        var_t2_dn6 = assign17360_e30659_d_n6;
        var_t2_dn7 = assign17360_e30659_d_n7;
        var_t2_dn8 = assign17360_e30659_d_n8;
        var_t2_dn9 = assign17360_e30659_d_n9;
        var_t2_dn10 = assign17360_e30659_d_n10;
        var_t2_dn11 = assign17360_e30659_d_n11;
        var_t2_dn13 = assign17360_e30659_d_n13;
        var_t2_dn14 = assign17360_e30659_d_n14;
        var_t2_rv = 0.0;

        let (assign17370_e30714, assign17370_e30714_d_n0, assign17370_e30714_d_n2, assign17370_e30714_d_n3, assign17370_e30714_d_n4, assign17370_e30714_d_n5, assign17370_e30714_d_n6, assign17370_e30714_d_n7, assign17370_e30714_d_n8, assign17370_e30714_d_n9, assign17370_e30714_d_n10, assign17370_e30714_d_n11, assign17370_e30714_d_n13, assign17370_e30714_d_n14,) = {
    if ((var_guard244 == 0.0) && (var_guard259 == 0.0)) {
        let assign17370_e30667: f64 = (var_t2 - 1e-6);
        let assign17370_e30669: f64 = (-10000.0);
        let assign17370_e30671: f64 = (assign17370_e30669 * 0.001);
        let (assign17370_e30712, assign17370_e30712_d_n0, assign17370_e30712_d_n2, assign17370_e30712_d_n3, assign17370_e30712_d_n4, assign17370_e30712_d_n5, assign17370_e30712_d_n6, assign17370_e30712_d_n7, assign17370_e30712_d_n8, assign17370_e30712_d_n9, assign17370_e30712_d_n10, assign17370_e30712_d_n11, assign17370_e30712_d_n13, assign17370_e30712_d_n14,) = {
            if (!(assign17370_e30667 < assign17370_e30671)) {
                let assign17370_e30677: f64 = (var_t2 - 1e-6);
                let assign17370_e30680: f64 = (var_t2 - 1e-6);
                let assign17370_e30683: f64 = (var_t2 - 1e-6);
                let assign17370_e30684: f64 = (assign17370_e30680 * assign17370_e30683);
                let assign17370_e30687: f64 = (4.0 * 0.001);
                let assign17370_e30689: f64 = (assign17370_e30687 * 0.001);
                let assign17370_e30690: f64 = (assign17370_e30684 + assign17370_e30689);
                let assign17370_e30691: f64 = (assign17370_e30690).sqrt();
                let assign17370_e30692: f64 = (assign17370_e30677 + assign17370_e30691);
                let assign17370_e30693: f64 = (0.5 * assign17370_e30692);
                (assign17370_e30693, (0.5 * (var_t2_dn0 + (((var_t2_dn0 * assign17370_e30683) + (assign17370_e30680 * var_t2_dn0)) / (2.0 * assign17370_e30691)))), (0.5 * (var_t2_dn2 + (((var_t2_dn2 * assign17370_e30683) + (assign17370_e30680 * var_t2_dn2)) / (2.0 * assign17370_e30691)))), (0.5 * (var_t2_dn3 + (((var_t2_dn3 * assign17370_e30683) + (assign17370_e30680 * var_t2_dn3)) / (2.0 * assign17370_e30691)))), (0.5 * (var_t2_dn4 + (((var_t2_dn4 * assign17370_e30683) + (assign17370_e30680 * var_t2_dn4)) / (2.0 * assign17370_e30691)))), (0.5 * (var_t2_dn5 + (((var_t2_dn5 * assign17370_e30683) + (assign17370_e30680 * var_t2_dn5)) / (2.0 * assign17370_e30691)))), (0.5 * (var_t2_dn6 + (((var_t2_dn6 * assign17370_e30683) + (assign17370_e30680 * var_t2_dn6)) / (2.0 * assign17370_e30691)))), (0.5 * (var_t2_dn7 + (((var_t2_dn7 * assign17370_e30683) + (assign17370_e30680 * var_t2_dn7)) / (2.0 * assign17370_e30691)))), (0.5 * (var_t2_dn8 + (((var_t2_dn8 * assign17370_e30683) + (assign17370_e30680 * var_t2_dn8)) / (2.0 * assign17370_e30691)))), (0.5 * (var_t2_dn9 + (((var_t2_dn9 * assign17370_e30683) + (assign17370_e30680 * var_t2_dn9)) / (2.0 * assign17370_e30691)))), (0.5 * (var_t2_dn10 + (((var_t2_dn10 * assign17370_e30683) + (assign17370_e30680 * var_t2_dn10)) / (2.0 * assign17370_e30691)))), (0.5 * (var_t2_dn11 + (((var_t2_dn11 * assign17370_e30683) + (assign17370_e30680 * var_t2_dn11)) / (2.0 * assign17370_e30691)))), (0.5 * (var_t2_dn13 + (((var_t2_dn13 * assign17370_e30683) + (assign17370_e30680 * var_t2_dn13)) / (2.0 * assign17370_e30691)))), (0.5 * (var_t2_dn14 + (((var_t2_dn14 * assign17370_e30683) + (assign17370_e30680 * var_t2_dn14)) / (2.0 * assign17370_e30691)))),)
            } else {
                let assign17370_e30696: f64 = (var_t2 - 1e-6);
                let assign17370_e30698: f64 = (-10000.0);
                let assign17370_e30700: f64 = (assign17370_e30698 * 0.001);
                let (assign17370_e30711, assign17370_e30711_d_n0, assign17370_e30711_d_n2, assign17370_e30711_d_n3, assign17370_e30711_d_n4, assign17370_e30711_d_n5, assign17370_e30711_d_n6, assign17370_e30711_d_n7, assign17370_e30711_d_n8, assign17370_e30711_d_n9, assign17370_e30711_d_n10, assign17370_e30711_d_n11, assign17370_e30711_d_n13, assign17370_e30711_d_n14,) = {
                    if (assign17370_e30696 < assign17370_e30700) {
                        let assign17370_e30703: f64 = (-0.001);
                        let assign17370_e30705: f64 = (assign17370_e30703 * 0.001);
                        let assign17370_e30708: f64 = (var_t2 - 1e-6);
                        let assign17370_e30709: f64 = (assign17370_e30705 / assign17370_e30708);
                        (assign17370_e30709, (-((assign17370_e30705 * var_t2_dn0) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * var_t2_dn2) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * var_t2_dn3) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * var_t2_dn4) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * var_t2_dn5) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * var_t2_dn6) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * var_t2_dn7) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * var_t2_dn8) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * var_t2_dn9) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * var_t2_dn10) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * var_t2_dn11) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * var_t2_dn13) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * var_t2_dn14) / (assign17370_e30708 * assign17370_e30708))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign17370_e30711, assign17370_e30711_d_n0, assign17370_e30711_d_n2, assign17370_e30711_d_n3, assign17370_e30711_d_n4, assign17370_e30711_d_n5, assign17370_e30711_d_n6, assign17370_e30711_d_n7, assign17370_e30711_d_n8, assign17370_e30711_d_n9, assign17370_e30711_d_n10, assign17370_e30711_d_n11, assign17370_e30711_d_n13, assign17370_e30711_d_n14,)
            }
        };
        (assign17370_e30712, assign17370_e30712_d_n0, assign17370_e30712_d_n2, assign17370_e30712_d_n3, assign17370_e30712_d_n4, assign17370_e30712_d_n5, assign17370_e30712_d_n6, assign17370_e30712_d_n7, assign17370_e30712_d_n8, assign17370_e30712_d_n9, assign17370_e30712_d_n10, assign17370_e30712_d_n11, assign17370_e30712_d_n13, assign17370_e30712_d_n14,)
    } else {
        (var_rdstemp, var_rdstemp_dn0, var_rdstemp_dn2, var_rdstemp_dn3, var_rdstemp_dn4, var_rdstemp_dn5, var_rdstemp_dn6, var_rdstemp_dn7, var_rdstemp_dn8, var_rdstemp_dn9, var_rdstemp_dn10, var_rdstemp_dn11, var_rdstemp_dn13, var_rdstemp_dn14,)
    }
};
        var_rdstemp = assign17370_e30714;
        var_rdstemp_dn0 = assign17370_e30714_d_n0;
        var_rdstemp_dn2 = assign17370_e30714_d_n2;
        var_rdstemp_dn3 = assign17370_e30714_d_n3;
        var_rdstemp_dn4 = assign17370_e30714_d_n4;
        var_rdstemp_dn5 = assign17370_e30714_d_n5;
        var_rdstemp_dn6 = assign17370_e30714_d_n6;
        var_rdstemp_dn7 = assign17370_e30714_d_n7;
        var_rdstemp_dn8 = assign17370_e30714_d_n8;
        var_rdstemp_dn9 = assign17370_e30714_d_n9;
        var_rdstemp_dn10 = assign17370_e30714_d_n10;
        var_rdstemp_dn11 = assign17370_e30714_d_n11;
        var_rdstemp_dn13 = assign17370_e30714_d_n13;
        var_rdstemp_dn14 = assign17370_e30714_d_n14;
        var_rdstemp_rv = 0.0;

        *var_guard307_slot = var_guard307;
        *var_guard307_rv_slot = var_guard307_rv;
        *var_guard308_slot = var_guard308;
        *var_guard308_rv_slot = var_guard308_rv;
        *var_guard309_slot = var_guard309;
        *var_guard309_rv_slot = var_guard309_rv;
        *var_rdstemp_slot = var_rdstemp;
        *var_rdstemp0_slot = var_rdstemp0;
        *var_rdstemp0_dn4_slot = var_rdstemp0_dn4;
        *var_rdstemp0_rv_slot = var_rdstemp0_rv;
        *var_rdstemp1_slot = var_rdstemp1;
        *var_rdstemp1_dn4_slot = var_rdstemp1_dn4;
        *var_rdstemp1_rv_slot = var_rdstemp1_rv;
        *var_rdstemp_dn0_slot = var_rdstemp_dn0;
        *var_rdstemp_dn10_slot = var_rdstemp_dn10;
        *var_rdstemp_dn11_slot = var_rdstemp_dn11;
        *var_rdstemp_dn13_slot = var_rdstemp_dn13;
        *var_rdstemp_dn14_slot = var_rdstemp_dn14;
        *var_rdstemp_dn2_slot = var_rdstemp_dn2;
        *var_rdstemp_dn3_slot = var_rdstemp_dn3;
        *var_rdstemp_dn4_slot = var_rdstemp_dn4;
        *var_rdstemp_dn5_slot = var_rdstemp_dn5;
        *var_rdstemp_dn6_slot = var_rdstemp_dn6;
        *var_rdstemp_dn7_slot = var_rdstemp_dn7;
        *var_rdstemp_dn8_slot = var_rdstemp_dn8;
        *var_rdstemp_dn9_slot = var_rdstemp_dn9;
        *var_rdstemp_rv_slot = var_rdstemp_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn13_slot = var_t6_dn13;
        *var_t6_dn14_slot = var_t6_dn14;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn3_slot = var_t6_dn3;
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
        *var_t7_dn11_slot = var_t7_dn11;
        *var_t7_dn13_slot = var_t7_dn13;
        *var_t7_dn14_slot = var_t7_dn14;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn3_slot = var_t7_dn3;
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
        *var_t8_dn11_slot = var_t8_dn11;
        *var_t8_dn13_slot = var_t8_dn13;
        *var_t8_dn14_slot = var_t8_dn14;
        *var_t8_dn2_slot = var_t8_dn2;
        *var_t8_dn3_slot = var_t8_dn3;
        *var_t8_dn4_slot = var_t8_dn4;
        *var_t8_dn5_slot = var_t8_dn5;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn7_slot = var_t8_dn7;
        *var_t8_dn8_slot = var_t8_dn8;
        *var_t8_dn9_slot = var_t8_dn9;
        *var_t8_rv_slot = var_t8_rv;
    }

    pub(super) fn stamp_reactive_block_59(
        p: &Parameters,
        var_deltemp: f64,
        var_deltemp_dn4: f64,
        var_devtemp1: f64,
        var_devtemp1_dn4: f64,
        var_guard244: f64,
        var_guard259: f64,
        var_kt1_i: f64,
        var_leff_1: f64,
        var_leff_1_dn0: f64,
        var_leff_1_dn10: f64,
        var_leff_1_dn11: f64,
        var_leff_1_dn13: f64,
        var_leff_1_dn14: f64,
        var_leff_1_dn2: f64,
        var_leff_1_dn3: f64,
        var_leff_1_dn4: f64,
        var_leff_1_dn5: f64,
        var_leff_1_dn6: f64,
        var_leff_1_dn7: f64,
        var_leff_1_dn8: f64,
        var_leff_1_dn9: f64,
        var_tnom: f64,
        var_tratio_m1: f64,
        var_tratio_m1_dn4: f64,
        var_uc1_i: f64,
        var_uc1cv_i: f64,
        var_uc1r_i: f64,
        var_uc_i: f64,
        var_uccv_i: f64,
        var_ucr_i: f64,
        var_dvth_temp_slot: &mut f64,
        var_dvth_temp_dn0_slot: &mut f64,
        var_dvth_temp_dn10_slot: &mut f64,
        var_dvth_temp_dn11_slot: &mut f64,
        var_dvth_temp_dn13_slot: &mut f64,
        var_dvth_temp_dn14_slot: &mut f64,
        var_dvth_temp_dn2_slot: &mut f64,
        var_dvth_temp_dn3_slot: &mut f64,
        var_dvth_temp_dn4_slot: &mut f64,
        var_dvth_temp_dn5_slot: &mut f64,
        var_dvth_temp_dn6_slot: &mut f64,
        var_dvth_temp_dn7_slot: &mut f64,
        var_dvth_temp_dn8_slot: &mut f64,
        var_dvth_temp_dn9_slot: &mut f64,
        var_dvth_temp_rv_slot: &mut f64,
        var_guard310_slot: &mut f64,
        var_guard310_rv_slot: &mut f64,
        var_guard311_slot: &mut f64,
        var_guard311_rv_slot: &mut f64,
        var_guard312_slot: &mut f64,
        var_guard312_rv_slot: &mut f64,
        var_guard313_slot: &mut f64,
        var_guard313_rv_slot: &mut f64,
        var_guard314_slot: &mut f64,
        var_guard314_rv_slot: &mut f64,
        var_guard315_slot: &mut f64,
        var_guard315_rv_slot: &mut f64,
        var_guard316_slot: &mut f64,
        var_guard316_rv_slot: &mut f64,
        var_guard317_slot: &mut f64,
        var_guard317_rv_slot: &mut f64,
        var_guard318_slot: &mut f64,
        var_guard318_rv_slot: &mut f64,
        var_guard319_slot: &mut f64,
        var_guard319_rv_slot: &mut f64,
        var_guard320_slot: &mut f64,
        var_guard320_rv_slot: &mut f64,
        var_guard321_slot: &mut f64,
        var_guard321_rv_slot: &mut f64,
        var_guard322_slot: &mut f64,
        var_guard322_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn13_slot: &mut f64,
        var_t4_dn14_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_uc_t_slot: &mut f64,
        var_uc_t_dn4_slot: &mut f64,
        var_uc_t_rv_slot: &mut f64,
        var_uccv_t_slot: &mut f64,
        var_uccv_t_dn4_slot: &mut f64,
        var_uccv_t_rv_slot: &mut f64,
        var_ucr_t_slot: &mut f64,
        var_ucr_t_dn4_slot: &mut f64,
        var_ucr_t_rv_slot: &mut f64,
        var_vsat1_t_slot: &mut f64,
        var_vsat1_t_dn0_slot: &mut f64,
        var_vsat1_t_dn10_slot: &mut f64,
        var_vsat1_t_dn11_slot: &mut f64,
        var_vsat1_t_dn13_slot: &mut f64,
        var_vsat1_t_dn14_slot: &mut f64,
        var_vsat1_t_dn2_slot: &mut f64,
        var_vsat1_t_dn3_slot: &mut f64,
        var_vsat1_t_dn4_slot: &mut f64,
        var_vsat1_t_dn5_slot: &mut f64,
        var_vsat1_t_dn6_slot: &mut f64,
        var_vsat1_t_dn7_slot: &mut f64,
        var_vsat1_t_dn8_slot: &mut f64,
        var_vsat1_t_dn9_slot: &mut f64,
        var_vsat1_t_rv_slot: &mut f64,
        var_vsat_t_slot: &mut f64,
        var_vsat_t_dn0_slot: &mut f64,
        var_vsat_t_dn10_slot: &mut f64,
        var_vsat_t_dn11_slot: &mut f64,
        var_vsat_t_dn13_slot: &mut f64,
        var_vsat_t_dn14_slot: &mut f64,
        var_vsat_t_dn2_slot: &mut f64,
        var_vsat_t_dn3_slot: &mut f64,
        var_vsat_t_dn4_slot: &mut f64,
        var_vsat_t_dn5_slot: &mut f64,
        var_vsat_t_dn6_slot: &mut f64,
        var_vsat_t_dn7_slot: &mut f64,
        var_vsat_t_dn8_slot: &mut f64,
        var_vsat_t_dn9_slot: &mut f64,
        var_vsat_t_rv_slot: &mut f64,
        var_vsatcv_t_slot: &mut f64,
        var_vsatcv_t_dn0_slot: &mut f64,
        var_vsatcv_t_dn10_slot: &mut f64,
        var_vsatcv_t_dn11_slot: &mut f64,
        var_vsatcv_t_dn13_slot: &mut f64,
        var_vsatcv_t_dn14_slot: &mut f64,
        var_vsatcv_t_dn2_slot: &mut f64,
        var_vsatcv_t_dn3_slot: &mut f64,
        var_vsatcv_t_dn4_slot: &mut f64,
        var_vsatcv_t_dn5_slot: &mut f64,
        var_vsatcv_t_dn6_slot: &mut f64,
        var_vsatcv_t_dn7_slot: &mut f64,
        var_vsatcv_t_dn8_slot: &mut f64,
        var_vsatcv_t_dn9_slot: &mut f64,
        var_vsatcv_t_rv_slot: &mut f64,
    ) {
        let mut var_dvth_temp: f64 = *var_dvth_temp_slot;
        let mut var_dvth_temp_dn0: f64 = *var_dvth_temp_dn0_slot;
        let mut var_dvth_temp_dn10: f64 = *var_dvth_temp_dn10_slot;
        let mut var_dvth_temp_dn11: f64 = *var_dvth_temp_dn11_slot;
        let mut var_dvth_temp_dn13: f64 = *var_dvth_temp_dn13_slot;
        let mut var_dvth_temp_dn14: f64 = *var_dvth_temp_dn14_slot;
        let mut var_dvth_temp_dn2: f64 = *var_dvth_temp_dn2_slot;
        let mut var_dvth_temp_dn3: f64 = *var_dvth_temp_dn3_slot;
        let mut var_dvth_temp_dn4: f64 = *var_dvth_temp_dn4_slot;
        let mut var_dvth_temp_dn5: f64 = *var_dvth_temp_dn5_slot;
        let mut var_dvth_temp_dn6: f64 = *var_dvth_temp_dn6_slot;
        let mut var_dvth_temp_dn7: f64 = *var_dvth_temp_dn7_slot;
        let mut var_dvth_temp_dn8: f64 = *var_dvth_temp_dn8_slot;
        let mut var_dvth_temp_dn9: f64 = *var_dvth_temp_dn9_slot;
        let mut var_dvth_temp_rv: f64 = *var_dvth_temp_rv_slot;
        let mut var_guard310: f64 = *var_guard310_slot;
        let mut var_guard310_rv: f64 = *var_guard310_rv_slot;
        let mut var_guard311: f64 = *var_guard311_slot;
        let mut var_guard311_rv: f64 = *var_guard311_rv_slot;
        let mut var_guard312: f64 = *var_guard312_slot;
        let mut var_guard312_rv: f64 = *var_guard312_rv_slot;
        let mut var_guard313: f64 = *var_guard313_slot;
        let mut var_guard313_rv: f64 = *var_guard313_rv_slot;
        let mut var_guard314: f64 = *var_guard314_slot;
        let mut var_guard314_rv: f64 = *var_guard314_rv_slot;
        let mut var_guard315: f64 = *var_guard315_slot;
        let mut var_guard315_rv: f64 = *var_guard315_rv_slot;
        let mut var_guard316: f64 = *var_guard316_slot;
        let mut var_guard316_rv: f64 = *var_guard316_rv_slot;
        let mut var_guard317: f64 = *var_guard317_slot;
        let mut var_guard317_rv: f64 = *var_guard317_rv_slot;
        let mut var_guard318: f64 = *var_guard318_slot;
        let mut var_guard318_rv: f64 = *var_guard318_rv_slot;
        let mut var_guard319: f64 = *var_guard319_slot;
        let mut var_guard319_rv: f64 = *var_guard319_rv_slot;
        let mut var_guard320: f64 = *var_guard320_slot;
        let mut var_guard320_rv: f64 = *var_guard320_rv_slot;
        let mut var_guard321: f64 = *var_guard321_slot;
        let mut var_guard321_rv: f64 = *var_guard321_rv_slot;
        let mut var_guard322: f64 = *var_guard322_slot;
        let mut var_guard322_rv: f64 = *var_guard322_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn13: f64 = *var_t4_dn13_slot;
        let mut var_t4_dn14: f64 = *var_t4_dn14_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_uc_t: f64 = *var_uc_t_slot;
        let mut var_uc_t_dn4: f64 = *var_uc_t_dn4_slot;
        let mut var_uc_t_rv: f64 = *var_uc_t_rv_slot;
        let mut var_uccv_t: f64 = *var_uccv_t_slot;
        let mut var_uccv_t_dn4: f64 = *var_uccv_t_dn4_slot;
        let mut var_uccv_t_rv: f64 = *var_uccv_t_rv_slot;
        let mut var_ucr_t: f64 = *var_ucr_t_slot;
        let mut var_ucr_t_dn4: f64 = *var_ucr_t_dn4_slot;
        let mut var_ucr_t_rv: f64 = *var_ucr_t_rv_slot;
        let mut var_vsat1_t: f64 = *var_vsat1_t_slot;
        let mut var_vsat1_t_dn0: f64 = *var_vsat1_t_dn0_slot;
        let mut var_vsat1_t_dn10: f64 = *var_vsat1_t_dn10_slot;
        let mut var_vsat1_t_dn11: f64 = *var_vsat1_t_dn11_slot;
        let mut var_vsat1_t_dn13: f64 = *var_vsat1_t_dn13_slot;
        let mut var_vsat1_t_dn14: f64 = *var_vsat1_t_dn14_slot;
        let mut var_vsat1_t_dn2: f64 = *var_vsat1_t_dn2_slot;
        let mut var_vsat1_t_dn3: f64 = *var_vsat1_t_dn3_slot;
        let mut var_vsat1_t_dn4: f64 = *var_vsat1_t_dn4_slot;
        let mut var_vsat1_t_dn5: f64 = *var_vsat1_t_dn5_slot;
        let mut var_vsat1_t_dn6: f64 = *var_vsat1_t_dn6_slot;
        let mut var_vsat1_t_dn7: f64 = *var_vsat1_t_dn7_slot;
        let mut var_vsat1_t_dn8: f64 = *var_vsat1_t_dn8_slot;
        let mut var_vsat1_t_dn9: f64 = *var_vsat1_t_dn9_slot;
        let mut var_vsat1_t_rv: f64 = *var_vsat1_t_rv_slot;
        let mut var_vsat_t: f64 = *var_vsat_t_slot;
        let mut var_vsat_t_dn0: f64 = *var_vsat_t_dn0_slot;
        let mut var_vsat_t_dn10: f64 = *var_vsat_t_dn10_slot;
        let mut var_vsat_t_dn11: f64 = *var_vsat_t_dn11_slot;
        let mut var_vsat_t_dn13: f64 = *var_vsat_t_dn13_slot;
        let mut var_vsat_t_dn14: f64 = *var_vsat_t_dn14_slot;
        let mut var_vsat_t_dn2: f64 = *var_vsat_t_dn2_slot;
        let mut var_vsat_t_dn3: f64 = *var_vsat_t_dn3_slot;
        let mut var_vsat_t_dn4: f64 = *var_vsat_t_dn4_slot;
        let mut var_vsat_t_dn5: f64 = *var_vsat_t_dn5_slot;
        let mut var_vsat_t_dn6: f64 = *var_vsat_t_dn6_slot;
        let mut var_vsat_t_dn7: f64 = *var_vsat_t_dn7_slot;
        let mut var_vsat_t_dn8: f64 = *var_vsat_t_dn8_slot;
        let mut var_vsat_t_dn9: f64 = *var_vsat_t_dn9_slot;
        let mut var_vsat_t_rv: f64 = *var_vsat_t_rv_slot;
        let mut var_vsatcv_t: f64 = *var_vsatcv_t_slot;
        let mut var_vsatcv_t_dn0: f64 = *var_vsatcv_t_dn0_slot;
        let mut var_vsatcv_t_dn10: f64 = *var_vsatcv_t_dn10_slot;
        let mut var_vsatcv_t_dn11: f64 = *var_vsatcv_t_dn11_slot;
        let mut var_vsatcv_t_dn13: f64 = *var_vsatcv_t_dn13_slot;
        let mut var_vsatcv_t_dn14: f64 = *var_vsatcv_t_dn14_slot;
        let mut var_vsatcv_t_dn2: f64 = *var_vsatcv_t_dn2_slot;
        let mut var_vsatcv_t_dn3: f64 = *var_vsatcv_t_dn3_slot;
        let mut var_vsatcv_t_dn4: f64 = *var_vsatcv_t_dn4_slot;
        let mut var_vsatcv_t_dn5: f64 = *var_vsatcv_t_dn5_slot;
        let mut var_vsatcv_t_dn6: f64 = *var_vsatcv_t_dn6_slot;
        let mut var_vsatcv_t_dn7: f64 = *var_vsatcv_t_dn7_slot;
        let mut var_vsatcv_t_dn8: f64 = *var_vsatcv_t_dn8_slot;
        let mut var_vsatcv_t_dn9: f64 = *var_vsatcv_t_dn9_slot;
        let mut var_vsatcv_t_rv: f64 = *var_vsatcv_t_rv_slot;

        let (assign17380_e30741, assign17380_e30741_d_n0, assign17380_e30741_d_n2, assign17380_e30741_d_n3, assign17380_e30741_d_n4, assign17380_e30741_d_n5, assign17380_e30741_d_n6, assign17380_e30741_d_n7, assign17380_e30741_d_n8, assign17380_e30741_d_n9, assign17380_e30741_d_n10, assign17380_e30741_d_n11, assign17380_e30741_d_n13, assign17380_e30741_d_n14,) = {
    if ((var_guard244 == 0.0) && (var_guard259 == 0.0)) {
        let assign17380_e30723: f64 = (var_tnom + 210.0);
        let assign17380_e30726: f64 = (var_tnom - 210.0);
        let assign17380_e30729: f64 = (var_tnom - 210.0);
        let assign17380_e30730: f64 = (assign17380_e30726 * assign17380_e30729);
        let assign17380_e30733: f64 = (0.25 * 0.2);
        let assign17380_e30735: f64 = (assign17380_e30733 * 0.2);
        let assign17380_e30736: f64 = (assign17380_e30730 + assign17380_e30735);
        let assign17380_e30737: f64 = (assign17380_e30736).sqrt();
        let assign17380_e30738: f64 = (assign17380_e30723 - assign17380_e30737);
        let assign17380_e30739: f64 = (0.5 * assign17380_e30738);
        (assign17380_e30739, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11, var_t4_dn13, var_t4_dn14,)
    }
};
        var_t4 = assign17380_e30741;
        var_t4_dn0 = assign17380_e30741_d_n0;
        var_t4_dn2 = assign17380_e30741_d_n2;
        var_t4_dn3 = assign17380_e30741_d_n3;
        var_t4_dn4 = assign17380_e30741_d_n4;
        var_t4_dn5 = assign17380_e30741_d_n5;
        var_t4_dn6 = assign17380_e30741_d_n6;
        var_t4_dn7 = assign17380_e30741_d_n7;
        var_t4_dn8 = assign17380_e30741_d_n8;
        var_t4_dn9 = assign17380_e30741_d_n9;
        var_t4_dn10 = assign17380_e30741_d_n10;
        var_t4_dn11 = assign17380_e30741_d_n11;
        var_t4_dn13 = assign17380_e30741_d_n13;
        var_t4_dn14 = assign17380_e30741_d_n14;
        var_t4_rv = 0.0;

        let (assign17390_e30777, assign17390_e30777_d_n0, assign17390_e30777_d_n2, assign17390_e30777_d_n3, assign17390_e30777_d_n4, assign17390_e30777_d_n5, assign17390_e30777_d_n6, assign17390_e30777_d_n7, assign17390_e30777_d_n8, assign17390_e30777_d_n9, assign17390_e30777_d_n10, assign17390_e30777_d_n11, assign17390_e30777_d_n13, assign17390_e30777_d_n14,) = {
    if ((var_guard244 == 0.0) && (var_guard259 == 0.0)) {
        let assign17390_e30750: f64 = (p.p1720 / var_leff_1);
        let assign17390_e30751: f64 = (var_kt1_i + assign17390_e30750);
        let assign17390_e30753: f64 = (assign17390_e30751 * var_tratio_m1);
        let assign17390_e30759: f64 = (var_devtemp1 - p.p1749);
        let assign17390_e30760: f64 = (p.p1748 * assign17390_e30759);
        let assign17390_e30761: f64 = { let limited_exp_arg = assign17390_e30760; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign17390_e30762: f64 = (1.0 + assign17390_e30761);
        let assign17390_e30763: f64 = (p.p1747 / assign17390_e30762);
        let assign17390_e30764: f64 = (assign17390_e30753 + assign17390_e30763);
        let assign17390_e30770: f64 = (var_t4 - p.p1749);
        let assign17390_e30771: f64 = (p.p1748 * assign17390_e30770);
        let assign17390_e30772: f64 = { let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign17390_e30773: f64 = (1.0 + assign17390_e30772);
        let assign17390_e30774: f64 = (p.p1747 / assign17390_e30773);
        let assign17390_e30775: f64 = (assign17390_e30764 - assign17390_e30774);
        (assign17390_e30775, (((-((p.p1720 * var_leff_1_dn0) / (var_leff_1 * var_leff_1))) * var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * var_t4_dn0))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * var_leff_1_dn2) / (var_leff_1 * var_leff_1))) * var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * var_t4_dn2))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * var_leff_1_dn3) / (var_leff_1 * var_leff_1))) * var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * var_t4_dn3))) / (assign17390_e30773 * assign17390_e30773)))), (((((-((p.p1720 * var_leff_1_dn4) / (var_leff_1 * var_leff_1))) * var_tratio_m1) + (assign17390_e30751 * var_tratio_m1_dn4)) + (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30760; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * var_devtemp1_dn4))) / (assign17390_e30762 * assign17390_e30762)))) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * var_t4_dn4))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * var_leff_1_dn5) / (var_leff_1 * var_leff_1))) * var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * var_t4_dn5))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * var_leff_1_dn6) / (var_leff_1 * var_leff_1))) * var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * var_t4_dn6))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * var_leff_1_dn7) / (var_leff_1 * var_leff_1))) * var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * var_t4_dn7))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * var_leff_1_dn8) / (var_leff_1 * var_leff_1))) * var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * var_t4_dn8))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * var_leff_1_dn9) / (var_leff_1 * var_leff_1))) * var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * var_t4_dn9))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * var_leff_1_dn10) / (var_leff_1 * var_leff_1))) * var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * var_t4_dn10))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * var_leff_1_dn11) / (var_leff_1 * var_leff_1))) * var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * var_t4_dn11))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * var_leff_1_dn13) / (var_leff_1 * var_leff_1))) * var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * var_t4_dn13))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * var_leff_1_dn14) / (var_leff_1 * var_leff_1))) * var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * var_t4_dn14))) / (assign17390_e30773 * assign17390_e30773)))),)
    } else {
        (var_dvth_temp, var_dvth_temp_dn0, var_dvth_temp_dn2, var_dvth_temp_dn3, var_dvth_temp_dn4, var_dvth_temp_dn5, var_dvth_temp_dn6, var_dvth_temp_dn7, var_dvth_temp_dn8, var_dvth_temp_dn9, var_dvth_temp_dn10, var_dvth_temp_dn11, var_dvth_temp_dn13, var_dvth_temp_dn14,)
    }
};
        var_dvth_temp = assign17390_e30777;
        var_dvth_temp_dn0 = assign17390_e30777_d_n0;
        var_dvth_temp_dn2 = assign17390_e30777_d_n2;
        var_dvth_temp_dn3 = assign17390_e30777_d_n3;
        var_dvth_temp_dn4 = assign17390_e30777_d_n4;
        var_dvth_temp_dn5 = assign17390_e30777_d_n5;
        var_dvth_temp_dn6 = assign17390_e30777_d_n6;
        var_dvth_temp_dn7 = assign17390_e30777_d_n7;
        var_dvth_temp_dn8 = assign17390_e30777_d_n8;
        var_dvth_temp_dn9 = assign17390_e30777_d_n9;
        var_dvth_temp_dn10 = assign17390_e30777_d_n10;
        var_dvth_temp_dn11 = assign17390_e30777_d_n11;
        var_dvth_temp_dn13 = assign17390_e30777_d_n13;
        var_dvth_temp_dn14 = assign17390_e30777_d_n14;
        var_dvth_temp_rv = 0.0;

        let assign17400_e30780: f64 = if var_vsat_t < 1000.0 { 1.0 } else { 0.0 };
        var_guard310 = assign17400_e30780;
        var_guard310_rv = 0.0;

        let (assign17410_e30784, assign17410_e30784_d_n0, assign17410_e30784_d_n2, assign17410_e30784_d_n3, assign17410_e30784_d_n4, assign17410_e30784_d_n5, assign17410_e30784_d_n6, assign17410_e30784_d_n7, assign17410_e30784_d_n8, assign17410_e30784_d_n9, assign17410_e30784_d_n10, assign17410_e30784_d_n11, assign17410_e30784_d_n13, assign17410_e30784_d_n14,) = {
    if (var_guard310 != 0.0) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vsat_t, var_vsat_t_dn0, var_vsat_t_dn2, var_vsat_t_dn3, var_vsat_t_dn4, var_vsat_t_dn5, var_vsat_t_dn6, var_vsat_t_dn7, var_vsat_t_dn8, var_vsat_t_dn9, var_vsat_t_dn10, var_vsat_t_dn11, var_vsat_t_dn13, var_vsat_t_dn14,)
    }
};
        var_vsat_t = assign17410_e30784;
        var_vsat_t_dn0 = assign17410_e30784_d_n0;
        var_vsat_t_dn2 = assign17410_e30784_d_n2;
        var_vsat_t_dn3 = assign17410_e30784_d_n3;
        var_vsat_t_dn4 = assign17410_e30784_d_n4;
        var_vsat_t_dn5 = assign17410_e30784_d_n5;
        var_vsat_t_dn6 = assign17410_e30784_d_n6;
        var_vsat_t_dn7 = assign17410_e30784_d_n7;
        var_vsat_t_dn8 = assign17410_e30784_d_n8;
        var_vsat_t_dn9 = assign17410_e30784_d_n9;
        var_vsat_t_dn10 = assign17410_e30784_d_n10;
        var_vsat_t_dn11 = assign17410_e30784_d_n11;
        var_vsat_t_dn13 = assign17410_e30784_d_n13;
        var_vsat_t_dn14 = assign17410_e30784_d_n14;
        var_vsat_t_rv = 0.0;

        let assign17420_e30787: f64 = if var_vsat1_t < 1000.0 { 1.0 } else { 0.0 };
        var_guard311 = assign17420_e30787;
        var_guard311_rv = 0.0;

        let (assign17430_e30791, assign17430_e30791_d_n0, assign17430_e30791_d_n2, assign17430_e30791_d_n3, assign17430_e30791_d_n4, assign17430_e30791_d_n5, assign17430_e30791_d_n6, assign17430_e30791_d_n7, assign17430_e30791_d_n8, assign17430_e30791_d_n9, assign17430_e30791_d_n10, assign17430_e30791_d_n11, assign17430_e30791_d_n13, assign17430_e30791_d_n14,) = {
    if (var_guard311 != 0.0) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vsat1_t, var_vsat1_t_dn0, var_vsat1_t_dn2, var_vsat1_t_dn3, var_vsat1_t_dn4, var_vsat1_t_dn5, var_vsat1_t_dn6, var_vsat1_t_dn7, var_vsat1_t_dn8, var_vsat1_t_dn9, var_vsat1_t_dn10, var_vsat1_t_dn11, var_vsat1_t_dn13, var_vsat1_t_dn14,)
    }
};
        var_vsat1_t = assign17430_e30791;
        var_vsat1_t_dn0 = assign17430_e30791_d_n0;
        var_vsat1_t_dn2 = assign17430_e30791_d_n2;
        var_vsat1_t_dn3 = assign17430_e30791_d_n3;
        var_vsat1_t_dn4 = assign17430_e30791_d_n4;
        var_vsat1_t_dn5 = assign17430_e30791_d_n5;
        var_vsat1_t_dn6 = assign17430_e30791_d_n6;
        var_vsat1_t_dn7 = assign17430_e30791_d_n7;
        var_vsat1_t_dn8 = assign17430_e30791_d_n8;
        var_vsat1_t_dn9 = assign17430_e30791_d_n9;
        var_vsat1_t_dn10 = assign17430_e30791_d_n10;
        var_vsat1_t_dn11 = assign17430_e30791_d_n11;
        var_vsat1_t_dn13 = assign17430_e30791_d_n13;
        var_vsat1_t_dn14 = assign17430_e30791_d_n14;
        var_vsat1_t_rv = 0.0;

        let assign17440_e30794: f64 = if var_vsatcv_t < 1000.0 { 1.0 } else { 0.0 };
        var_guard312 = assign17440_e30794;
        var_guard312_rv = 0.0;

        let (assign17450_e30798, assign17450_e30798_d_n0, assign17450_e30798_d_n2, assign17450_e30798_d_n3, assign17450_e30798_d_n4, assign17450_e30798_d_n5, assign17450_e30798_d_n6, assign17450_e30798_d_n7, assign17450_e30798_d_n8, assign17450_e30798_d_n9, assign17450_e30798_d_n10, assign17450_e30798_d_n11, assign17450_e30798_d_n13, assign17450_e30798_d_n14,) = {
    if (var_guard312 != 0.0) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vsatcv_t, var_vsatcv_t_dn0, var_vsatcv_t_dn2, var_vsatcv_t_dn3, var_vsatcv_t_dn4, var_vsatcv_t_dn5, var_vsatcv_t_dn6, var_vsatcv_t_dn7, var_vsatcv_t_dn8, var_vsatcv_t_dn9, var_vsatcv_t_dn10, var_vsatcv_t_dn11, var_vsatcv_t_dn13, var_vsatcv_t_dn14,)
    }
};
        var_vsatcv_t = assign17450_e30798;
        var_vsatcv_t_dn0 = assign17450_e30798_d_n0;
        var_vsatcv_t_dn2 = assign17450_e30798_d_n2;
        var_vsatcv_t_dn3 = assign17450_e30798_d_n3;
        var_vsatcv_t_dn4 = assign17450_e30798_d_n4;
        var_vsatcv_t_dn5 = assign17450_e30798_d_n5;
        var_vsatcv_t_dn6 = assign17450_e30798_d_n6;
        var_vsatcv_t_dn7 = assign17450_e30798_d_n7;
        var_vsatcv_t_dn8 = assign17450_e30798_d_n8;
        var_vsatcv_t_dn9 = assign17450_e30798_d_n9;
        var_vsatcv_t_dn10 = assign17450_e30798_d_n10;
        var_vsatcv_t_dn11 = assign17450_e30798_d_n11;
        var_vsatcv_t_dn13 = assign17450_e30798_d_n13;
        var_vsatcv_t_dn14 = assign17450_e30798_d_n14;
        var_vsatcv_t_rv = 0.0;

        let assign17460_e30801: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        var_guard313 = assign17460_e30801;
        var_guard313_rv = 0.0;

        let assign17470_e30804: f64 = if p.p75 == 0.0 { 1.0 } else { 0.0 };
        var_guard314 = assign17470_e30804;
        var_guard314_rv = 0.0;

        let assign17480_e30807: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard315 = assign17480_e30807;
        var_guard315_rv = 0.0;

        let (assign17490_e30855, assign17490_e30855_d_n4,) = {
    if (((var_guard313 != 0.0) && (var_guard314 != 0.0)) && (var_guard315 != 0.0)) {
        let assign17490_e30815: f64 = (-var_uc_i);
        let assign17490_e30819: f64 = (var_uc1_i * var_deltemp);
        let assign17490_e30821: f64 = (-var_uc_i);
        let assign17490_e30822: f64 = (assign17490_e30819 - assign17490_e30821);
        let assign17490_e30824: f64 = (assign17490_e30822 - 1e-6);
        let assign17490_e30827: f64 = (var_uc1_i * var_deltemp);
        let assign17490_e30829: f64 = (-var_uc_i);
        let assign17490_e30830: f64 = (assign17490_e30827 - assign17490_e30829);
        let assign17490_e30832: f64 = (assign17490_e30830 - 1e-6);
        let assign17490_e30835: f64 = (var_uc1_i * var_deltemp);
        let assign17490_e30837: f64 = (-var_uc_i);
        let assign17490_e30838: f64 = (assign17490_e30835 - assign17490_e30837);
        let assign17490_e30840: f64 = (assign17490_e30838 - 1e-6);
        let assign17490_e30841: f64 = (assign17490_e30832 * assign17490_e30840);
        let assign17490_e30844: f64 = (-var_uc_i);
        let assign17490_e30845: f64 = (4.0 * assign17490_e30844);
        let assign17490_e30847: f64 = (assign17490_e30845 * 1e-6);
        let assign17490_e30848: f64 = (assign17490_e30841 - assign17490_e30847);
        let assign17490_e30849: f64 = (assign17490_e30848).sqrt();
        let assign17490_e30850: f64 = (assign17490_e30824 + assign17490_e30849);
        let assign17490_e30851: f64 = (0.5 * assign17490_e30850);
        let assign17490_e30852: f64 = (assign17490_e30815 + assign17490_e30851);
        let assign17490_e30853: f64 = (var_uc_i + assign17490_e30852);
        (assign17490_e30853, (0.5 * ((var_uc1_i * var_deltemp_dn4) + ((((var_uc1_i * var_deltemp_dn4) * assign17490_e30840) + (assign17490_e30832 * (var_uc1_i * var_deltemp_dn4))) / (2.0 * assign17490_e30849)))),)
    } else {
        (var_uc_t, var_uc_t_dn4,)
    }
};
        var_uc_t = assign17490_e30855;
        var_uc_t_dn4 = assign17490_e30855_d_n4;
        var_uc_t_rv = 0.0;

        let (assign17500_e30937, assign17500_e30937_d_n4,) = {
    if (((var_guard313 != 0.0) && (var_guard314 != 0.0)) && (var_guard315 == 0.0)) {
        let assign17500_e30866: f64 = (var_uc1_i * var_deltemp);
        let assign17500_e30867: f64 = (1.0 + assign17500_e30866);
        let assign17500_e30869: f64 = (assign17500_e30867 - 1e-6);
        let assign17500_e30871: f64 = (-10000.0);
        let assign17500_e30873: f64 = (assign17500_e30871 * 0.001);
        let (assign17500_e30934, assign17500_e30934_d_n4,) = {
            if (!(assign17500_e30869 < assign17500_e30873)) {
                let assign17500_e30880: f64 = (var_uc1_i * var_deltemp);
                let assign17500_e30881: f64 = (1.0 + assign17500_e30880);
                let assign17500_e30883: f64 = (assign17500_e30881 - 1e-6);
                let assign17500_e30887: f64 = (var_uc1_i * var_deltemp);
                let assign17500_e30888: f64 = (1.0 + assign17500_e30887);
                let assign17500_e30890: f64 = (assign17500_e30888 - 1e-6);
                let assign17500_e30894: f64 = (var_uc1_i * var_deltemp);
                let assign17500_e30895: f64 = (1.0 + assign17500_e30894);
                let assign17500_e30897: f64 = (assign17500_e30895 - 1e-6);
                let assign17500_e30898: f64 = (assign17500_e30890 * assign17500_e30897);
                let assign17500_e30901: f64 = (4.0 * 0.001);
                let assign17500_e30903: f64 = (assign17500_e30901 * 0.001);
                let assign17500_e30904: f64 = (assign17500_e30898 + assign17500_e30903);
                let assign17500_e30905: f64 = (assign17500_e30904).sqrt();
                let assign17500_e30906: f64 = (assign17500_e30883 + assign17500_e30905);
                let assign17500_e30907: f64 = (0.5 * assign17500_e30906);
                (assign17500_e30907, (0.5 * ((var_uc1_i * var_deltemp_dn4) + ((((var_uc1_i * var_deltemp_dn4) * assign17500_e30897) + (assign17500_e30890 * (var_uc1_i * var_deltemp_dn4))) / (2.0 * assign17500_e30905)))),)
            } else {
                let assign17500_e30911: f64 = (var_uc1_i * var_deltemp);
                let assign17500_e30912: f64 = (1.0 + assign17500_e30911);
                let assign17500_e30914: f64 = (assign17500_e30912 - 1e-6);
                let assign17500_e30916: f64 = (-10000.0);
                let assign17500_e30918: f64 = (assign17500_e30916 * 0.001);
                let (assign17500_e30933, assign17500_e30933_d_n4,) = {
                    if (assign17500_e30914 < assign17500_e30918) {
                        let assign17500_e30921: f64 = (-0.001);
                        let assign17500_e30923: f64 = (assign17500_e30921 * 0.001);
                        let assign17500_e30927: f64 = (var_uc1_i * var_deltemp);
                        let assign17500_e30928: f64 = (1.0 + assign17500_e30927);
                        let assign17500_e30930: f64 = (assign17500_e30928 - 1e-6);
                        let assign17500_e30931: f64 = (assign17500_e30923 / assign17500_e30930);
                        (assign17500_e30931, (-((assign17500_e30923 * (var_uc1_i * var_deltemp_dn4)) / (assign17500_e30930 * assign17500_e30930))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17500_e30933, assign17500_e30933_d_n4,)
            }
        };
        let assign17500_e30935: f64 = (var_uc_i * assign17500_e30934);
        (assign17500_e30935, (var_uc_i * assign17500_e30934_d_n4),)
    } else {
        (var_uc_t, var_uc_t_dn4,)
    }
};
        var_uc_t = assign17500_e30937;
        var_uc_t_dn4 = assign17500_e30937_d_n4;
        var_uc_t_rv = 0.0;

        let assign17510_e30940: f64 = if p.p67 == 1.0 { 1.0 } else { 0.0 };
        var_guard316 = assign17510_e30940;
        var_guard316_rv = 0.0;

        let assign17520_e30943: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard317 = assign17520_e30943;
        var_guard317_rv = 0.0;

        let (assign17530_e30993, assign17530_e30993_d_n4,) = {
    if ((((var_guard313 != 0.0) && (var_guard314 != 0.0)) && (var_guard316 != 0.0)) && (var_guard317 != 0.0)) {
        let assign17530_e30953: f64 = (-var_uccv_i);
        let assign17530_e30957: f64 = (var_uc1cv_i * var_deltemp);
        let assign17530_e30959: f64 = (-var_uccv_i);
        let assign17530_e30960: f64 = (assign17530_e30957 - assign17530_e30959);
        let assign17530_e30962: f64 = (assign17530_e30960 - 1e-6);
        let assign17530_e30965: f64 = (var_uc1cv_i * var_deltemp);
        let assign17530_e30967: f64 = (-var_uccv_i);
        let assign17530_e30968: f64 = (assign17530_e30965 - assign17530_e30967);
        let assign17530_e30970: f64 = (assign17530_e30968 - 1e-6);
        let assign17530_e30973: f64 = (var_uc1cv_i * var_deltemp);
        let assign17530_e30975: f64 = (-var_uccv_i);
        let assign17530_e30976: f64 = (assign17530_e30973 - assign17530_e30975);
        let assign17530_e30978: f64 = (assign17530_e30976 - 1e-6);
        let assign17530_e30979: f64 = (assign17530_e30970 * assign17530_e30978);
        let assign17530_e30982: f64 = (-var_uccv_i);
        let assign17530_e30983: f64 = (4.0 * assign17530_e30982);
        let assign17530_e30985: f64 = (assign17530_e30983 * 1e-6);
        let assign17530_e30986: f64 = (assign17530_e30979 - assign17530_e30985);
        let assign17530_e30987: f64 = (assign17530_e30986).sqrt();
        let assign17530_e30988: f64 = (assign17530_e30962 + assign17530_e30987);
        let assign17530_e30989: f64 = (0.5 * assign17530_e30988);
        let assign17530_e30990: f64 = (assign17530_e30953 + assign17530_e30989);
        let assign17530_e30991: f64 = (var_uccv_i + assign17530_e30990);
        (assign17530_e30991, (0.5 * ((var_uc1cv_i * var_deltemp_dn4) + ((((var_uc1cv_i * var_deltemp_dn4) * assign17530_e30978) + (assign17530_e30970 * (var_uc1cv_i * var_deltemp_dn4))) / (2.0 * assign17530_e30987)))),)
    } else {
        (var_uccv_t, var_uccv_t_dn4,)
    }
};
        var_uccv_t = assign17530_e30993;
        var_uccv_t_dn4 = assign17530_e30993_d_n4;
        var_uccv_t_rv = 0.0;

        let (assign17540_e31077, assign17540_e31077_d_n4,) = {
    if ((((var_guard313 != 0.0) && (var_guard314 != 0.0)) && (var_guard316 != 0.0)) && (var_guard317 == 0.0)) {
        let assign17540_e31006: f64 = (var_uc1cv_i * var_deltemp);
        let assign17540_e31007: f64 = (1.0 + assign17540_e31006);
        let assign17540_e31009: f64 = (assign17540_e31007 - 1e-6);
        let assign17540_e31011: f64 = (-10000.0);
        let assign17540_e31013: f64 = (assign17540_e31011 * 0.001);
        let (assign17540_e31074, assign17540_e31074_d_n4,) = {
            if (!(assign17540_e31009 < assign17540_e31013)) {
                let assign17540_e31020: f64 = (var_uc1cv_i * var_deltemp);
                let assign17540_e31021: f64 = (1.0 + assign17540_e31020);
                let assign17540_e31023: f64 = (assign17540_e31021 - 1e-6);
                let assign17540_e31027: f64 = (var_uc1cv_i * var_deltemp);
                let assign17540_e31028: f64 = (1.0 + assign17540_e31027);
                let assign17540_e31030: f64 = (assign17540_e31028 - 1e-6);
                let assign17540_e31034: f64 = (var_uc1cv_i * var_deltemp);
                let assign17540_e31035: f64 = (1.0 + assign17540_e31034);
                let assign17540_e31037: f64 = (assign17540_e31035 - 1e-6);
                let assign17540_e31038: f64 = (assign17540_e31030 * assign17540_e31037);
                let assign17540_e31041: f64 = (4.0 * 0.001);
                let assign17540_e31043: f64 = (assign17540_e31041 * 0.001);
                let assign17540_e31044: f64 = (assign17540_e31038 + assign17540_e31043);
                let assign17540_e31045: f64 = (assign17540_e31044).sqrt();
                let assign17540_e31046: f64 = (assign17540_e31023 + assign17540_e31045);
                let assign17540_e31047: f64 = (0.5 * assign17540_e31046);
                (assign17540_e31047, (0.5 * ((var_uc1cv_i * var_deltemp_dn4) + ((((var_uc1cv_i * var_deltemp_dn4) * assign17540_e31037) + (assign17540_e31030 * (var_uc1cv_i * var_deltemp_dn4))) / (2.0 * assign17540_e31045)))),)
            } else {
                let assign17540_e31051: f64 = (var_uc1cv_i * var_deltemp);
                let assign17540_e31052: f64 = (1.0 + assign17540_e31051);
                let assign17540_e31054: f64 = (assign17540_e31052 - 1e-6);
                let assign17540_e31056: f64 = (-10000.0);
                let assign17540_e31058: f64 = (assign17540_e31056 * 0.001);
                let (assign17540_e31073, assign17540_e31073_d_n4,) = {
                    if (assign17540_e31054 < assign17540_e31058) {
                        let assign17540_e31061: f64 = (-0.001);
                        let assign17540_e31063: f64 = (assign17540_e31061 * 0.001);
                        let assign17540_e31067: f64 = (var_uc1cv_i * var_deltemp);
                        let assign17540_e31068: f64 = (1.0 + assign17540_e31067);
                        let assign17540_e31070: f64 = (assign17540_e31068 - 1e-6);
                        let assign17540_e31071: f64 = (assign17540_e31063 / assign17540_e31070);
                        (assign17540_e31071, (-((assign17540_e31063 * (var_uc1cv_i * var_deltemp_dn4)) / (assign17540_e31070 * assign17540_e31070))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17540_e31073, assign17540_e31073_d_n4,)
            }
        };
        let assign17540_e31075: f64 = (var_uccv_i * assign17540_e31074);
        (assign17540_e31075, (var_uccv_i * assign17540_e31074_d_n4),)
    } else {
        (var_uccv_t, var_uccv_t_dn4,)
    }
};
        var_uccv_t = assign17540_e31077;
        var_uccv_t_dn4 = assign17540_e31077_d_n4;
        var_uccv_t_rv = 0.0;

        let assign17550_e31080: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        var_guard318 = assign17550_e31080;
        var_guard318_rv = 0.0;

        let assign17560_e31083: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard319 = assign17560_e31083;
        var_guard319_rv = 0.0;

        let (assign17570_e31133, assign17570_e31133_d_n4,) = {
    if ((((var_guard313 != 0.0) && (var_guard314 != 0.0)) && (var_guard318 != 0.0)) && (var_guard319 != 0.0)) {
        let assign17570_e31093: f64 = (-var_ucr_i);
        let assign17570_e31097: f64 = (var_uc1r_i * var_deltemp);
        let assign17570_e31099: f64 = (-var_ucr_i);
        let assign17570_e31100: f64 = (assign17570_e31097 - assign17570_e31099);
        let assign17570_e31102: f64 = (assign17570_e31100 - 1e-6);
        let assign17570_e31105: f64 = (var_uc1r_i * var_deltemp);
        let assign17570_e31107: f64 = (-var_ucr_i);
        let assign17570_e31108: f64 = (assign17570_e31105 - assign17570_e31107);
        let assign17570_e31110: f64 = (assign17570_e31108 - 1e-6);
        let assign17570_e31113: f64 = (var_uc1r_i * var_deltemp);
        let assign17570_e31115: f64 = (-var_ucr_i);
        let assign17570_e31116: f64 = (assign17570_e31113 - assign17570_e31115);
        let assign17570_e31118: f64 = (assign17570_e31116 - 1e-6);
        let assign17570_e31119: f64 = (assign17570_e31110 * assign17570_e31118);
        let assign17570_e31122: f64 = (-var_ucr_i);
        let assign17570_e31123: f64 = (4.0 * assign17570_e31122);
        let assign17570_e31125: f64 = (assign17570_e31123 * 1e-6);
        let assign17570_e31126: f64 = (assign17570_e31119 - assign17570_e31125);
        let assign17570_e31127: f64 = (assign17570_e31126).sqrt();
        let assign17570_e31128: f64 = (assign17570_e31102 + assign17570_e31127);
        let assign17570_e31129: f64 = (0.5 * assign17570_e31128);
        let assign17570_e31130: f64 = (assign17570_e31093 + assign17570_e31129);
        let assign17570_e31131: f64 = (var_ucr_i + assign17570_e31130);
        (assign17570_e31131, (0.5 * ((var_uc1r_i * var_deltemp_dn4) + ((((var_uc1r_i * var_deltemp_dn4) * assign17570_e31118) + (assign17570_e31110 * (var_uc1r_i * var_deltemp_dn4))) / (2.0 * assign17570_e31127)))),)
    } else {
        (var_ucr_t, var_ucr_t_dn4,)
    }
};
        var_ucr_t = assign17570_e31133;
        var_ucr_t_dn4 = assign17570_e31133_d_n4;
        var_ucr_t_rv = 0.0;

        let (assign17580_e31217, assign17580_e31217_d_n4,) = {
    if ((((var_guard313 != 0.0) && (var_guard314 != 0.0)) && (var_guard318 != 0.0)) && (var_guard319 == 0.0)) {
        let assign17580_e31146: f64 = (var_uc1r_i * var_deltemp);
        let assign17580_e31147: f64 = (1.0 + assign17580_e31146);
        let assign17580_e31149: f64 = (assign17580_e31147 - 1e-6);
        let assign17580_e31151: f64 = (-10000.0);
        let assign17580_e31153: f64 = (assign17580_e31151 * 0.001);
        let (assign17580_e31214, assign17580_e31214_d_n4,) = {
            if (!(assign17580_e31149 < assign17580_e31153)) {
                let assign17580_e31160: f64 = (var_uc1r_i * var_deltemp);
                let assign17580_e31161: f64 = (1.0 + assign17580_e31160);
                let assign17580_e31163: f64 = (assign17580_e31161 - 1e-6);
                let assign17580_e31167: f64 = (var_uc1r_i * var_deltemp);
                let assign17580_e31168: f64 = (1.0 + assign17580_e31167);
                let assign17580_e31170: f64 = (assign17580_e31168 - 1e-6);
                let assign17580_e31174: f64 = (var_uc1r_i * var_deltemp);
                let assign17580_e31175: f64 = (1.0 + assign17580_e31174);
                let assign17580_e31177: f64 = (assign17580_e31175 - 1e-6);
                let assign17580_e31178: f64 = (assign17580_e31170 * assign17580_e31177);
                let assign17580_e31181: f64 = (4.0 * 0.001);
                let assign17580_e31183: f64 = (assign17580_e31181 * 0.001);
                let assign17580_e31184: f64 = (assign17580_e31178 + assign17580_e31183);
                let assign17580_e31185: f64 = (assign17580_e31184).sqrt();
                let assign17580_e31186: f64 = (assign17580_e31163 + assign17580_e31185);
                let assign17580_e31187: f64 = (0.5 * assign17580_e31186);
                (assign17580_e31187, (0.5 * ((var_uc1r_i * var_deltemp_dn4) + ((((var_uc1r_i * var_deltemp_dn4) * assign17580_e31177) + (assign17580_e31170 * (var_uc1r_i * var_deltemp_dn4))) / (2.0 * assign17580_e31185)))),)
            } else {
                let assign17580_e31191: f64 = (var_uc1r_i * var_deltemp);
                let assign17580_e31192: f64 = (1.0 + assign17580_e31191);
                let assign17580_e31194: f64 = (assign17580_e31192 - 1e-6);
                let assign17580_e31196: f64 = (-10000.0);
                let assign17580_e31198: f64 = (assign17580_e31196 * 0.001);
                let (assign17580_e31213, assign17580_e31213_d_n4,) = {
                    if (assign17580_e31194 < assign17580_e31198) {
                        let assign17580_e31201: f64 = (-0.001);
                        let assign17580_e31203: f64 = (assign17580_e31201 * 0.001);
                        let assign17580_e31207: f64 = (var_uc1r_i * var_deltemp);
                        let assign17580_e31208: f64 = (1.0 + assign17580_e31207);
                        let assign17580_e31210: f64 = (assign17580_e31208 - 1e-6);
                        let assign17580_e31211: f64 = (assign17580_e31203 / assign17580_e31210);
                        (assign17580_e31211, (-((assign17580_e31203 * (var_uc1r_i * var_deltemp_dn4)) / (assign17580_e31210 * assign17580_e31210))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17580_e31213, assign17580_e31213_d_n4,)
            }
        };
        let assign17580_e31215: f64 = (var_ucr_i * assign17580_e31214);
        (assign17580_e31215, (var_ucr_i * assign17580_e31214_d_n4),)
    } else {
        (var_ucr_t, var_ucr_t_dn4,)
    }
};
        var_ucr_t = assign17580_e31217;
        var_ucr_t_dn4 = assign17580_e31217_d_n4;
        var_ucr_t_rv = 0.0;

        let (assign17590_e31228, assign17590_e31228_d_n4,) = {
    if ((var_guard313 != 0.0) && (var_guard314 == 0.0)) {
        let assign17590_e31225: f64 = (var_uc1_i * var_deltemp);
        let assign17590_e31226: f64 = (var_uc_i + assign17590_e31225);
        (assign17590_e31226, (var_uc1_i * var_deltemp_dn4),)
    } else {
        (var_uc_t, var_uc_t_dn4,)
    }
};
        var_uc_t = assign17590_e31228;
        var_uc_t_dn4 = assign17590_e31228_d_n4;
        var_uc_t_rv = 0.0;

        let assign17600_e31231: f64 = if p.p67 == 1.0 { 1.0 } else { 0.0 };
        var_guard320 = assign17600_e31231;
        var_guard320_rv = 0.0;

        let (assign17610_e31244, assign17610_e31244_d_n4,) = {
    if (((var_guard313 != 0.0) && (var_guard314 == 0.0)) && (var_guard320 != 0.0)) {
        let assign17610_e31241: f64 = (var_uc1cv_i * var_deltemp);
        let assign17610_e31242: f64 = (var_uccv_i + assign17610_e31241);
        (assign17610_e31242, (var_uc1cv_i * var_deltemp_dn4),)
    } else {
        (var_uccv_t, var_uccv_t_dn4,)
    }
};
        var_uccv_t = assign17610_e31244;
        var_uccv_t_dn4 = assign17610_e31244_d_n4;
        var_uccv_t_rv = 0.0;

        let assign17620_e31247: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        var_guard321 = assign17620_e31247;
        var_guard321_rv = 0.0;

        let (assign17630_e31260, assign17630_e31260_d_n4,) = {
    if (((var_guard313 != 0.0) && (var_guard314 == 0.0)) && (var_guard321 != 0.0)) {
        let assign17630_e31257: f64 = (var_uc1r_i * var_deltemp);
        let assign17630_e31258: f64 = (var_ucr_i + assign17630_e31257);
        (assign17630_e31258, (var_uc1r_i * var_deltemp_dn4),)
    } else {
        (var_ucr_t, var_ucr_t_dn4,)
    }
};
        var_ucr_t = assign17630_e31260;
        var_ucr_t_dn4 = assign17630_e31260_d_n4;
        var_ucr_t_rv = 0.0;

        let assign17640_e31263: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard322 = assign17640_e31263;
        var_guard322_rv = 0.0;

        *var_dvth_temp_slot = var_dvth_temp;
        *var_dvth_temp_dn0_slot = var_dvth_temp_dn0;
        *var_dvth_temp_dn10_slot = var_dvth_temp_dn10;
        *var_dvth_temp_dn11_slot = var_dvth_temp_dn11;
        *var_dvth_temp_dn13_slot = var_dvth_temp_dn13;
        *var_dvth_temp_dn14_slot = var_dvth_temp_dn14;
        *var_dvth_temp_dn2_slot = var_dvth_temp_dn2;
        *var_dvth_temp_dn3_slot = var_dvth_temp_dn3;
        *var_dvth_temp_dn4_slot = var_dvth_temp_dn4;
        *var_dvth_temp_dn5_slot = var_dvth_temp_dn5;
        *var_dvth_temp_dn6_slot = var_dvth_temp_dn6;
        *var_dvth_temp_dn7_slot = var_dvth_temp_dn7;
        *var_dvth_temp_dn8_slot = var_dvth_temp_dn8;
        *var_dvth_temp_dn9_slot = var_dvth_temp_dn9;
        *var_dvth_temp_rv_slot = var_dvth_temp_rv;
        *var_guard310_slot = var_guard310;
        *var_guard310_rv_slot = var_guard310_rv;
        *var_guard311_slot = var_guard311;
        *var_guard311_rv_slot = var_guard311_rv;
        *var_guard312_slot = var_guard312;
        *var_guard312_rv_slot = var_guard312_rv;
        *var_guard313_slot = var_guard313;
        *var_guard313_rv_slot = var_guard313_rv;
        *var_guard314_slot = var_guard314;
        *var_guard314_rv_slot = var_guard314_rv;
        *var_guard315_slot = var_guard315;
        *var_guard315_rv_slot = var_guard315_rv;
        *var_guard316_slot = var_guard316;
        *var_guard316_rv_slot = var_guard316_rv;
        *var_guard317_slot = var_guard317;
        *var_guard317_rv_slot = var_guard317_rv;
        *var_guard318_slot = var_guard318;
        *var_guard318_rv_slot = var_guard318_rv;
        *var_guard319_slot = var_guard319;
        *var_guard319_rv_slot = var_guard319_rv;
        *var_guard320_slot = var_guard320;
        *var_guard320_rv_slot = var_guard320_rv;
        *var_guard321_slot = var_guard321;
        *var_guard321_rv_slot = var_guard321_rv;
        *var_guard322_slot = var_guard322;
        *var_guard322_rv_slot = var_guard322_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn13_slot = var_t4_dn13;
        *var_t4_dn14_slot = var_t4_dn14;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t4_rv_slot = var_t4_rv;
        *var_uc_t_slot = var_uc_t;
        *var_uc_t_dn4_slot = var_uc_t_dn4;
        *var_uc_t_rv_slot = var_uc_t_rv;
        *var_uccv_t_slot = var_uccv_t;
        *var_uccv_t_dn4_slot = var_uccv_t_dn4;
        *var_uccv_t_rv_slot = var_uccv_t_rv;
        *var_ucr_t_slot = var_ucr_t;
        *var_ucr_t_dn4_slot = var_ucr_t_dn4;
        *var_ucr_t_rv_slot = var_ucr_t_rv;
        *var_vsat1_t_slot = var_vsat1_t;
        *var_vsat1_t_dn0_slot = var_vsat1_t_dn0;
        *var_vsat1_t_dn10_slot = var_vsat1_t_dn10;
        *var_vsat1_t_dn11_slot = var_vsat1_t_dn11;
        *var_vsat1_t_dn13_slot = var_vsat1_t_dn13;
        *var_vsat1_t_dn14_slot = var_vsat1_t_dn14;
        *var_vsat1_t_dn2_slot = var_vsat1_t_dn2;
        *var_vsat1_t_dn3_slot = var_vsat1_t_dn3;
        *var_vsat1_t_dn4_slot = var_vsat1_t_dn4;
        *var_vsat1_t_dn5_slot = var_vsat1_t_dn5;
        *var_vsat1_t_dn6_slot = var_vsat1_t_dn6;
        *var_vsat1_t_dn7_slot = var_vsat1_t_dn7;
        *var_vsat1_t_dn8_slot = var_vsat1_t_dn8;
        *var_vsat1_t_dn9_slot = var_vsat1_t_dn9;
        *var_vsat1_t_rv_slot = var_vsat1_t_rv;
        *var_vsat_t_slot = var_vsat_t;
        *var_vsat_t_dn0_slot = var_vsat_t_dn0;
        *var_vsat_t_dn10_slot = var_vsat_t_dn10;
        *var_vsat_t_dn11_slot = var_vsat_t_dn11;
        *var_vsat_t_dn13_slot = var_vsat_t_dn13;
        *var_vsat_t_dn14_slot = var_vsat_t_dn14;
        *var_vsat_t_dn2_slot = var_vsat_t_dn2;
        *var_vsat_t_dn3_slot = var_vsat_t_dn3;
        *var_vsat_t_dn4_slot = var_vsat_t_dn4;
        *var_vsat_t_dn5_slot = var_vsat_t_dn5;
        *var_vsat_t_dn6_slot = var_vsat_t_dn6;
        *var_vsat_t_dn7_slot = var_vsat_t_dn7;
        *var_vsat_t_dn8_slot = var_vsat_t_dn8;
        *var_vsat_t_dn9_slot = var_vsat_t_dn9;
        *var_vsat_t_rv_slot = var_vsat_t_rv;
        *var_vsatcv_t_slot = var_vsatcv_t;
        *var_vsatcv_t_dn0_slot = var_vsatcv_t_dn0;
        *var_vsatcv_t_dn10_slot = var_vsatcv_t_dn10;
        *var_vsatcv_t_dn11_slot = var_vsatcv_t_dn11;
        *var_vsatcv_t_dn13_slot = var_vsatcv_t_dn13;
        *var_vsatcv_t_dn14_slot = var_vsatcv_t_dn14;
        *var_vsatcv_t_dn2_slot = var_vsatcv_t_dn2;
        *var_vsatcv_t_dn3_slot = var_vsatcv_t_dn3;
        *var_vsatcv_t_dn4_slot = var_vsatcv_t_dn4;
        *var_vsatcv_t_dn5_slot = var_vsatcv_t_dn5;
        *var_vsatcv_t_dn6_slot = var_vsatcv_t_dn6;
        *var_vsatcv_t_dn7_slot = var_vsatcv_t_dn7;
        *var_vsatcv_t_dn8_slot = var_vsatcv_t_dn8;
        *var_vsatcv_t_dn9_slot = var_vsatcv_t_dn9;
        *var_vsatcv_t_rv_slot = var_vsatcv_t_rv;
    }

    pub(super) fn stamp_reactive_block_60(
        p: &Parameters,
        var_deltemp: f64,
        var_deltemp_dn4: f64,
        var_emobt_i: f64,
        var_eta0_i: f64,
        var_eta0_i_dn0: f64,
        var_eta0_i_dn10: f64,
        var_eta0_i_dn11: f64,
        var_eta0_i_dn13: f64,
        var_eta0_i_dn14: f64,
        var_eta0_i_dn2: f64,
        var_eta0_i_dn3: f64,
        var_eta0_i_dn4: f64,
        var_eta0_i_dn5: f64,
        var_eta0_i_dn6: f64,
        var_eta0_i_dn7: f64,
        var_eta0_i_dn8: f64,
        var_eta0_i_dn9: f64,
        var_eta0cv_i: f64,
        var_eta0cv_i_dn0: f64,
        var_eta0cv_i_dn10: f64,
        var_eta0cv_i_dn11: f64,
        var_eta0cv_i_dn13: f64,
        var_eta0cv_i_dn14: f64,
        var_eta0cv_i_dn2: f64,
        var_eta0cv_i_dn3: f64,
        var_eta0cv_i_dn4: f64,
        var_eta0cv_i_dn5: f64,
        var_eta0cv_i_dn6: f64,
        var_eta0cv_i_dn7: f64,
        var_eta0cv_i_dn8: f64,
        var_eta0cv_i_dn9: f64,
        var_eta0r_i: f64,
        var_etamob_i: f64,
        var_guard322: f64,
        var_eta0_t_slot: &mut f64,
        var_eta0_t_dn0_slot: &mut f64,
        var_eta0_t_dn10_slot: &mut f64,
        var_eta0_t_dn11_slot: &mut f64,
        var_eta0_t_dn13_slot: &mut f64,
        var_eta0_t_dn14_slot: &mut f64,
        var_eta0_t_dn2_slot: &mut f64,
        var_eta0_t_dn3_slot: &mut f64,
        var_eta0_t_dn4_slot: &mut f64,
        var_eta0_t_dn5_slot: &mut f64,
        var_eta0_t_dn6_slot: &mut f64,
        var_eta0_t_dn7_slot: &mut f64,
        var_eta0_t_dn8_slot: &mut f64,
        var_eta0_t_dn9_slot: &mut f64,
        var_eta0_t_rv_slot: &mut f64,
        var_eta0cv_t_slot: &mut f64,
        var_eta0cv_t_dn0_slot: &mut f64,
        var_eta0cv_t_dn10_slot: &mut f64,
        var_eta0cv_t_dn11_slot: &mut f64,
        var_eta0cv_t_dn13_slot: &mut f64,
        var_eta0cv_t_dn14_slot: &mut f64,
        var_eta0cv_t_dn2_slot: &mut f64,
        var_eta0cv_t_dn3_slot: &mut f64,
        var_eta0cv_t_dn4_slot: &mut f64,
        var_eta0cv_t_dn5_slot: &mut f64,
        var_eta0cv_t_dn6_slot: &mut f64,
        var_eta0cv_t_dn7_slot: &mut f64,
        var_eta0cv_t_dn8_slot: &mut f64,
        var_eta0cv_t_dn9_slot: &mut f64,
        var_eta0cv_t_rv_slot: &mut f64,
        var_eta0r_t_slot: &mut f64,
        var_eta0r_t_dn4_slot: &mut f64,
        var_eta0r_t_rv_slot: &mut f64,
        var_etamob_t_slot: &mut f64,
        var_etamob_t_dn4_slot: &mut f64,
        var_etamob_t_rv_slot: &mut f64,
        var_guard323_slot: &mut f64,
        var_guard323_rv_slot: &mut f64,
        var_guard324_slot: &mut f64,
        var_guard324_rv_slot: &mut f64,
        var_guard325_slot: &mut f64,
        var_guard325_rv_slot: &mut f64,
        var_guard326_slot: &mut f64,
        var_guard326_rv_slot: &mut f64,
        var_guard327_slot: &mut f64,
        var_guard327_rv_slot: &mut f64,
        var_rsdr_t_slot: &mut f64,
        var_rsdr_t_dn4_slot: &mut f64,
        var_rsdr_t_rv_slot: &mut f64,
    ) {
        let mut var_eta0_t: f64 = *var_eta0_t_slot;
        let mut var_eta0_t_dn0: f64 = *var_eta0_t_dn0_slot;
        let mut var_eta0_t_dn10: f64 = *var_eta0_t_dn10_slot;
        let mut var_eta0_t_dn11: f64 = *var_eta0_t_dn11_slot;
        let mut var_eta0_t_dn13: f64 = *var_eta0_t_dn13_slot;
        let mut var_eta0_t_dn14: f64 = *var_eta0_t_dn14_slot;
        let mut var_eta0_t_dn2: f64 = *var_eta0_t_dn2_slot;
        let mut var_eta0_t_dn3: f64 = *var_eta0_t_dn3_slot;
        let mut var_eta0_t_dn4: f64 = *var_eta0_t_dn4_slot;
        let mut var_eta0_t_dn5: f64 = *var_eta0_t_dn5_slot;
        let mut var_eta0_t_dn6: f64 = *var_eta0_t_dn6_slot;
        let mut var_eta0_t_dn7: f64 = *var_eta0_t_dn7_slot;
        let mut var_eta0_t_dn8: f64 = *var_eta0_t_dn8_slot;
        let mut var_eta0_t_dn9: f64 = *var_eta0_t_dn9_slot;
        let mut var_eta0_t_rv: f64 = *var_eta0_t_rv_slot;
        let mut var_eta0cv_t: f64 = *var_eta0cv_t_slot;
        let mut var_eta0cv_t_dn0: f64 = *var_eta0cv_t_dn0_slot;
        let mut var_eta0cv_t_dn10: f64 = *var_eta0cv_t_dn10_slot;
        let mut var_eta0cv_t_dn11: f64 = *var_eta0cv_t_dn11_slot;
        let mut var_eta0cv_t_dn13: f64 = *var_eta0cv_t_dn13_slot;
        let mut var_eta0cv_t_dn14: f64 = *var_eta0cv_t_dn14_slot;
        let mut var_eta0cv_t_dn2: f64 = *var_eta0cv_t_dn2_slot;
        let mut var_eta0cv_t_dn3: f64 = *var_eta0cv_t_dn3_slot;
        let mut var_eta0cv_t_dn4: f64 = *var_eta0cv_t_dn4_slot;
        let mut var_eta0cv_t_dn5: f64 = *var_eta0cv_t_dn5_slot;
        let mut var_eta0cv_t_dn6: f64 = *var_eta0cv_t_dn6_slot;
        let mut var_eta0cv_t_dn7: f64 = *var_eta0cv_t_dn7_slot;
        let mut var_eta0cv_t_dn8: f64 = *var_eta0cv_t_dn8_slot;
        let mut var_eta0cv_t_dn9: f64 = *var_eta0cv_t_dn9_slot;
        let mut var_eta0cv_t_rv: f64 = *var_eta0cv_t_rv_slot;
        let mut var_eta0r_t: f64 = *var_eta0r_t_slot;
        let mut var_eta0r_t_dn4: f64 = *var_eta0r_t_dn4_slot;
        let mut var_eta0r_t_rv: f64 = *var_eta0r_t_rv_slot;
        let mut var_etamob_t: f64 = *var_etamob_t_slot;
        let mut var_etamob_t_dn4: f64 = *var_etamob_t_dn4_slot;
        let mut var_etamob_t_rv: f64 = *var_etamob_t_rv_slot;
        let mut var_guard323: f64 = *var_guard323_slot;
        let mut var_guard323_rv: f64 = *var_guard323_rv_slot;
        let mut var_guard324: f64 = *var_guard324_slot;
        let mut var_guard324_rv: f64 = *var_guard324_rv_slot;
        let mut var_guard325: f64 = *var_guard325_slot;
        let mut var_guard325_rv: f64 = *var_guard325_rv_slot;
        let mut var_guard326: f64 = *var_guard326_slot;
        let mut var_guard326_rv: f64 = *var_guard326_rv_slot;
        let mut var_guard327: f64 = *var_guard327_slot;
        let mut var_guard327_rv: f64 = *var_guard327_rv_slot;
        let mut var_rsdr_t: f64 = *var_rsdr_t_slot;
        let mut var_rsdr_t_dn4: f64 = *var_rsdr_t_dn4_slot;
        let mut var_rsdr_t_rv: f64 = *var_rsdr_t_rv_slot;

        let (assign17650_e31307, assign17650_e31307_d_n0, assign17650_e31307_d_n2, assign17650_e31307_d_n3, assign17650_e31307_d_n4, assign17650_e31307_d_n5, assign17650_e31307_d_n6, assign17650_e31307_d_n7, assign17650_e31307_d_n8, assign17650_e31307_d_n9, assign17650_e31307_d_n10, assign17650_e31307_d_n11, assign17650_e31307_d_n13, assign17650_e31307_d_n14,) = {
    if (var_guard322 != 0.0) {
        let assign17650_e31267: f64 = (-var_eta0_i);
        let assign17650_e31271: f64 = (p.p164 * var_deltemp);
        let assign17650_e31273: f64 = (-var_eta0_i);
        let assign17650_e31274: f64 = (assign17650_e31271 - assign17650_e31273);
        let assign17650_e31276: f64 = (assign17650_e31274 - 1e-6);
        let assign17650_e31279: f64 = (p.p164 * var_deltemp);
        let assign17650_e31281: f64 = (-var_eta0_i);
        let assign17650_e31282: f64 = (assign17650_e31279 - assign17650_e31281);
        let assign17650_e31284: f64 = (assign17650_e31282 - 1e-6);
        let assign17650_e31287: f64 = (p.p164 * var_deltemp);
        let assign17650_e31289: f64 = (-var_eta0_i);
        let assign17650_e31290: f64 = (assign17650_e31287 - assign17650_e31289);
        let assign17650_e31292: f64 = (assign17650_e31290 - 1e-6);
        let assign17650_e31293: f64 = (assign17650_e31284 * assign17650_e31292);
        let assign17650_e31296: f64 = (-var_eta0_i);
        let assign17650_e31297: f64 = (4.0 * assign17650_e31296);
        let assign17650_e31299: f64 = (assign17650_e31297 * 1e-6);
        let assign17650_e31300: f64 = (assign17650_e31293 - assign17650_e31299);
        let assign17650_e31301: f64 = (assign17650_e31300).sqrt();
        let assign17650_e31302: f64 = (assign17650_e31276 + assign17650_e31301);
        let assign17650_e31303: f64 = (0.5 * assign17650_e31302);
        let assign17650_e31304: f64 = (assign17650_e31267 + assign17650_e31303);
        let assign17650_e31305: f64 = (var_eta0_i + assign17650_e31304);
        (assign17650_e31305, (var_eta0_i_dn0 + ((-var_eta0_i_dn0) + (0.5 * ((-(-var_eta0_i_dn0)) + (((((-(-var_eta0_i_dn0)) * assign17650_e31292) + (assign17650_e31284 * (-(-var_eta0_i_dn0)))) - ((4.0 * (-var_eta0_i_dn0)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (var_eta0_i_dn2 + ((-var_eta0_i_dn2) + (0.5 * ((-(-var_eta0_i_dn2)) + (((((-(-var_eta0_i_dn2)) * assign17650_e31292) + (assign17650_e31284 * (-(-var_eta0_i_dn2)))) - ((4.0 * (-var_eta0_i_dn2)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (var_eta0_i_dn3 + ((-var_eta0_i_dn3) + (0.5 * ((-(-var_eta0_i_dn3)) + (((((-(-var_eta0_i_dn3)) * assign17650_e31292) + (assign17650_e31284 * (-(-var_eta0_i_dn3)))) - ((4.0 * (-var_eta0_i_dn3)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (var_eta0_i_dn4 + ((-var_eta0_i_dn4) + (0.5 * (((p.p164 * var_deltemp_dn4) - (-var_eta0_i_dn4)) + ((((((p.p164 * var_deltemp_dn4) - (-var_eta0_i_dn4)) * assign17650_e31292) + (assign17650_e31284 * ((p.p164 * var_deltemp_dn4) - (-var_eta0_i_dn4)))) - ((4.0 * (-var_eta0_i_dn4)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (var_eta0_i_dn5 + ((-var_eta0_i_dn5) + (0.5 * ((-(-var_eta0_i_dn5)) + (((((-(-var_eta0_i_dn5)) * assign17650_e31292) + (assign17650_e31284 * (-(-var_eta0_i_dn5)))) - ((4.0 * (-var_eta0_i_dn5)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (var_eta0_i_dn6 + ((-var_eta0_i_dn6) + (0.5 * ((-(-var_eta0_i_dn6)) + (((((-(-var_eta0_i_dn6)) * assign17650_e31292) + (assign17650_e31284 * (-(-var_eta0_i_dn6)))) - ((4.0 * (-var_eta0_i_dn6)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (var_eta0_i_dn7 + ((-var_eta0_i_dn7) + (0.5 * ((-(-var_eta0_i_dn7)) + (((((-(-var_eta0_i_dn7)) * assign17650_e31292) + (assign17650_e31284 * (-(-var_eta0_i_dn7)))) - ((4.0 * (-var_eta0_i_dn7)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (var_eta0_i_dn8 + ((-var_eta0_i_dn8) + (0.5 * ((-(-var_eta0_i_dn8)) + (((((-(-var_eta0_i_dn8)) * assign17650_e31292) + (assign17650_e31284 * (-(-var_eta0_i_dn8)))) - ((4.0 * (-var_eta0_i_dn8)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (var_eta0_i_dn9 + ((-var_eta0_i_dn9) + (0.5 * ((-(-var_eta0_i_dn9)) + (((((-(-var_eta0_i_dn9)) * assign17650_e31292) + (assign17650_e31284 * (-(-var_eta0_i_dn9)))) - ((4.0 * (-var_eta0_i_dn9)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (var_eta0_i_dn10 + ((-var_eta0_i_dn10) + (0.5 * ((-(-var_eta0_i_dn10)) + (((((-(-var_eta0_i_dn10)) * assign17650_e31292) + (assign17650_e31284 * (-(-var_eta0_i_dn10)))) - ((4.0 * (-var_eta0_i_dn10)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (var_eta0_i_dn11 + ((-var_eta0_i_dn11) + (0.5 * ((-(-var_eta0_i_dn11)) + (((((-(-var_eta0_i_dn11)) * assign17650_e31292) + (assign17650_e31284 * (-(-var_eta0_i_dn11)))) - ((4.0 * (-var_eta0_i_dn11)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (var_eta0_i_dn13 + ((-var_eta0_i_dn13) + (0.5 * ((-(-var_eta0_i_dn13)) + (((((-(-var_eta0_i_dn13)) * assign17650_e31292) + (assign17650_e31284 * (-(-var_eta0_i_dn13)))) - ((4.0 * (-var_eta0_i_dn13)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (var_eta0_i_dn14 + ((-var_eta0_i_dn14) + (0.5 * ((-(-var_eta0_i_dn14)) + (((((-(-var_eta0_i_dn14)) * assign17650_e31292) + (assign17650_e31284 * (-(-var_eta0_i_dn14)))) - ((4.0 * (-var_eta0_i_dn14)) * 1e-6)) / (2.0 * assign17650_e31301)))))),)
    } else {
        (var_eta0_t, var_eta0_t_dn0, var_eta0_t_dn2, var_eta0_t_dn3, var_eta0_t_dn4, var_eta0_t_dn5, var_eta0_t_dn6, var_eta0_t_dn7, var_eta0_t_dn8, var_eta0_t_dn9, var_eta0_t_dn10, var_eta0_t_dn11, var_eta0_t_dn13, var_eta0_t_dn14,)
    }
};
        var_eta0_t = assign17650_e31307;
        var_eta0_t_dn0 = assign17650_e31307_d_n0;
        var_eta0_t_dn2 = assign17650_e31307_d_n2;
        var_eta0_t_dn3 = assign17650_e31307_d_n3;
        var_eta0_t_dn4 = assign17650_e31307_d_n4;
        var_eta0_t_dn5 = assign17650_e31307_d_n5;
        var_eta0_t_dn6 = assign17650_e31307_d_n6;
        var_eta0_t_dn7 = assign17650_e31307_d_n7;
        var_eta0_t_dn8 = assign17650_e31307_d_n8;
        var_eta0_t_dn9 = assign17650_e31307_d_n9;
        var_eta0_t_dn10 = assign17650_e31307_d_n10;
        var_eta0_t_dn11 = assign17650_e31307_d_n11;
        var_eta0_t_dn13 = assign17650_e31307_d_n13;
        var_eta0_t_dn14 = assign17650_e31307_d_n14;
        var_eta0_t_rv = 0.0;

        let (assign17660_e31385, assign17660_e31385_d_n0, assign17660_e31385_d_n2, assign17660_e31385_d_n3, assign17660_e31385_d_n4, assign17660_e31385_d_n5, assign17660_e31385_d_n6, assign17660_e31385_d_n7, assign17660_e31385_d_n8, assign17660_e31385_d_n9, assign17660_e31385_d_n10, assign17660_e31385_d_n11, assign17660_e31385_d_n13, assign17660_e31385_d_n14,) = {
    if (var_guard322 == 0.0) {
        let assign17660_e31314: f64 = (p.p164 * var_deltemp);
        let assign17660_e31315: f64 = (1.0 + assign17660_e31314);
        let assign17660_e31317: f64 = (assign17660_e31315 - 1e-6);
        let assign17660_e31319: f64 = (-10000.0);
        let assign17660_e31321: f64 = (assign17660_e31319 * 0.001);
        let (assign17660_e31382, assign17660_e31382_d_n4,) = {
            if (!(assign17660_e31317 < assign17660_e31321)) {
                let assign17660_e31328: f64 = (p.p164 * var_deltemp);
                let assign17660_e31329: f64 = (1.0 + assign17660_e31328);
                let assign17660_e31331: f64 = (assign17660_e31329 - 1e-6);
                let assign17660_e31335: f64 = (p.p164 * var_deltemp);
                let assign17660_e31336: f64 = (1.0 + assign17660_e31335);
                let assign17660_e31338: f64 = (assign17660_e31336 - 1e-6);
                let assign17660_e31342: f64 = (p.p164 * var_deltemp);
                let assign17660_e31343: f64 = (1.0 + assign17660_e31342);
                let assign17660_e31345: f64 = (assign17660_e31343 - 1e-6);
                let assign17660_e31346: f64 = (assign17660_e31338 * assign17660_e31345);
                let assign17660_e31349: f64 = (4.0 * 0.001);
                let assign17660_e31351: f64 = (assign17660_e31349 * 0.001);
                let assign17660_e31352: f64 = (assign17660_e31346 + assign17660_e31351);
                let assign17660_e31353: f64 = (assign17660_e31352).sqrt();
                let assign17660_e31354: f64 = (assign17660_e31331 + assign17660_e31353);
                let assign17660_e31355: f64 = (0.5 * assign17660_e31354);
                (assign17660_e31355, (0.5 * ((p.p164 * var_deltemp_dn4) + ((((p.p164 * var_deltemp_dn4) * assign17660_e31345) + (assign17660_e31338 * (p.p164 * var_deltemp_dn4))) / (2.0 * assign17660_e31353)))),)
            } else {
                let assign17660_e31359: f64 = (p.p164 * var_deltemp);
                let assign17660_e31360: f64 = (1.0 + assign17660_e31359);
                let assign17660_e31362: f64 = (assign17660_e31360 - 1e-6);
                let assign17660_e31364: f64 = (-10000.0);
                let assign17660_e31366: f64 = (assign17660_e31364 * 0.001);
                let (assign17660_e31381, assign17660_e31381_d_n4,) = {
                    if (assign17660_e31362 < assign17660_e31366) {
                        let assign17660_e31369: f64 = (-0.001);
                        let assign17660_e31371: f64 = (assign17660_e31369 * 0.001);
                        let assign17660_e31375: f64 = (p.p164 * var_deltemp);
                        let assign17660_e31376: f64 = (1.0 + assign17660_e31375);
                        let assign17660_e31378: f64 = (assign17660_e31376 - 1e-6);
                        let assign17660_e31379: f64 = (assign17660_e31371 / assign17660_e31378);
                        (assign17660_e31379, (-((assign17660_e31371 * (p.p164 * var_deltemp_dn4)) / (assign17660_e31378 * assign17660_e31378))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17660_e31381, assign17660_e31381_d_n4,)
            }
        };
        let assign17660_e31383: f64 = (var_eta0_i * assign17660_e31382);
        (assign17660_e31383, (var_eta0_i_dn0 * assign17660_e31382), (var_eta0_i_dn2 * assign17660_e31382), (var_eta0_i_dn3 * assign17660_e31382), ((var_eta0_i_dn4 * assign17660_e31382) + (var_eta0_i * assign17660_e31382_d_n4)), (var_eta0_i_dn5 * assign17660_e31382), (var_eta0_i_dn6 * assign17660_e31382), (var_eta0_i_dn7 * assign17660_e31382), (var_eta0_i_dn8 * assign17660_e31382), (var_eta0_i_dn9 * assign17660_e31382), (var_eta0_i_dn10 * assign17660_e31382), (var_eta0_i_dn11 * assign17660_e31382), (var_eta0_i_dn13 * assign17660_e31382), (var_eta0_i_dn14 * assign17660_e31382),)
    } else {
        (var_eta0_t, var_eta0_t_dn0, var_eta0_t_dn2, var_eta0_t_dn3, var_eta0_t_dn4, var_eta0_t_dn5, var_eta0_t_dn6, var_eta0_t_dn7, var_eta0_t_dn8, var_eta0_t_dn9, var_eta0_t_dn10, var_eta0_t_dn11, var_eta0_t_dn13, var_eta0_t_dn14,)
    }
};
        var_eta0_t = assign17660_e31385;
        var_eta0_t_dn0 = assign17660_e31385_d_n0;
        var_eta0_t_dn2 = assign17660_e31385_d_n2;
        var_eta0_t_dn3 = assign17660_e31385_d_n3;
        var_eta0_t_dn4 = assign17660_e31385_d_n4;
        var_eta0_t_dn5 = assign17660_e31385_d_n5;
        var_eta0_t_dn6 = assign17660_e31385_d_n6;
        var_eta0_t_dn7 = assign17660_e31385_d_n7;
        var_eta0_t_dn8 = assign17660_e31385_d_n8;
        var_eta0_t_dn9 = assign17660_e31385_d_n9;
        var_eta0_t_dn10 = assign17660_e31385_d_n10;
        var_eta0_t_dn11 = assign17660_e31385_d_n11;
        var_eta0_t_dn13 = assign17660_e31385_d_n13;
        var_eta0_t_dn14 = assign17660_e31385_d_n14;
        var_eta0_t_rv = 0.0;

        let assign17670_e31388: f64 = if p.p67 == 1.0 { 1.0 } else { 0.0 };
        var_guard323 = assign17670_e31388;
        var_guard323_rv = 0.0;

        let assign17680_e31391: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard324 = assign17680_e31391;
        var_guard324_rv = 0.0;

        let (assign17690_e31437, assign17690_e31437_d_n0, assign17690_e31437_d_n2, assign17690_e31437_d_n3, assign17690_e31437_d_n4, assign17690_e31437_d_n5, assign17690_e31437_d_n6, assign17690_e31437_d_n7, assign17690_e31437_d_n8, assign17690_e31437_d_n9, assign17690_e31437_d_n10, assign17690_e31437_d_n11, assign17690_e31437_d_n13, assign17690_e31437_d_n14,) = {
    if ((var_guard323 != 0.0) && (var_guard324 != 0.0)) {
        let assign17690_e31397: f64 = (-var_eta0cv_i);
        let assign17690_e31401: f64 = (p.p165 * var_deltemp);
        let assign17690_e31403: f64 = (-var_eta0cv_i);
        let assign17690_e31404: f64 = (assign17690_e31401 - assign17690_e31403);
        let assign17690_e31406: f64 = (assign17690_e31404 - 1e-6);
        let assign17690_e31409: f64 = (p.p165 * var_deltemp);
        let assign17690_e31411: f64 = (-var_eta0cv_i);
        let assign17690_e31412: f64 = (assign17690_e31409 - assign17690_e31411);
        let assign17690_e31414: f64 = (assign17690_e31412 - 1e-6);
        let assign17690_e31417: f64 = (p.p165 * var_deltemp);
        let assign17690_e31419: f64 = (-var_eta0cv_i);
        let assign17690_e31420: f64 = (assign17690_e31417 - assign17690_e31419);
        let assign17690_e31422: f64 = (assign17690_e31420 - 1e-6);
        let assign17690_e31423: f64 = (assign17690_e31414 * assign17690_e31422);
        let assign17690_e31426: f64 = (-var_eta0cv_i);
        let assign17690_e31427: f64 = (4.0 * assign17690_e31426);
        let assign17690_e31429: f64 = (assign17690_e31427 * 1e-6);
        let assign17690_e31430: f64 = (assign17690_e31423 - assign17690_e31429);
        let assign17690_e31431: f64 = (assign17690_e31430).sqrt();
        let assign17690_e31432: f64 = (assign17690_e31406 + assign17690_e31431);
        let assign17690_e31433: f64 = (0.5 * assign17690_e31432);
        let assign17690_e31434: f64 = (assign17690_e31397 + assign17690_e31433);
        let assign17690_e31435: f64 = (var_eta0cv_i + assign17690_e31434);
        (assign17690_e31435, (var_eta0cv_i_dn0 + ((-var_eta0cv_i_dn0) + (0.5 * ((-(-var_eta0cv_i_dn0)) + (((((-(-var_eta0cv_i_dn0)) * assign17690_e31422) + (assign17690_e31414 * (-(-var_eta0cv_i_dn0)))) - ((4.0 * (-var_eta0cv_i_dn0)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (var_eta0cv_i_dn2 + ((-var_eta0cv_i_dn2) + (0.5 * ((-(-var_eta0cv_i_dn2)) + (((((-(-var_eta0cv_i_dn2)) * assign17690_e31422) + (assign17690_e31414 * (-(-var_eta0cv_i_dn2)))) - ((4.0 * (-var_eta0cv_i_dn2)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (var_eta0cv_i_dn3 + ((-var_eta0cv_i_dn3) + (0.5 * ((-(-var_eta0cv_i_dn3)) + (((((-(-var_eta0cv_i_dn3)) * assign17690_e31422) + (assign17690_e31414 * (-(-var_eta0cv_i_dn3)))) - ((4.0 * (-var_eta0cv_i_dn3)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (var_eta0cv_i_dn4 + ((-var_eta0cv_i_dn4) + (0.5 * (((p.p165 * var_deltemp_dn4) - (-var_eta0cv_i_dn4)) + ((((((p.p165 * var_deltemp_dn4) - (-var_eta0cv_i_dn4)) * assign17690_e31422) + (assign17690_e31414 * ((p.p165 * var_deltemp_dn4) - (-var_eta0cv_i_dn4)))) - ((4.0 * (-var_eta0cv_i_dn4)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (var_eta0cv_i_dn5 + ((-var_eta0cv_i_dn5) + (0.5 * ((-(-var_eta0cv_i_dn5)) + (((((-(-var_eta0cv_i_dn5)) * assign17690_e31422) + (assign17690_e31414 * (-(-var_eta0cv_i_dn5)))) - ((4.0 * (-var_eta0cv_i_dn5)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (var_eta0cv_i_dn6 + ((-var_eta0cv_i_dn6) + (0.5 * ((-(-var_eta0cv_i_dn6)) + (((((-(-var_eta0cv_i_dn6)) * assign17690_e31422) + (assign17690_e31414 * (-(-var_eta0cv_i_dn6)))) - ((4.0 * (-var_eta0cv_i_dn6)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (var_eta0cv_i_dn7 + ((-var_eta0cv_i_dn7) + (0.5 * ((-(-var_eta0cv_i_dn7)) + (((((-(-var_eta0cv_i_dn7)) * assign17690_e31422) + (assign17690_e31414 * (-(-var_eta0cv_i_dn7)))) - ((4.0 * (-var_eta0cv_i_dn7)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (var_eta0cv_i_dn8 + ((-var_eta0cv_i_dn8) + (0.5 * ((-(-var_eta0cv_i_dn8)) + (((((-(-var_eta0cv_i_dn8)) * assign17690_e31422) + (assign17690_e31414 * (-(-var_eta0cv_i_dn8)))) - ((4.0 * (-var_eta0cv_i_dn8)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (var_eta0cv_i_dn9 + ((-var_eta0cv_i_dn9) + (0.5 * ((-(-var_eta0cv_i_dn9)) + (((((-(-var_eta0cv_i_dn9)) * assign17690_e31422) + (assign17690_e31414 * (-(-var_eta0cv_i_dn9)))) - ((4.0 * (-var_eta0cv_i_dn9)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (var_eta0cv_i_dn10 + ((-var_eta0cv_i_dn10) + (0.5 * ((-(-var_eta0cv_i_dn10)) + (((((-(-var_eta0cv_i_dn10)) * assign17690_e31422) + (assign17690_e31414 * (-(-var_eta0cv_i_dn10)))) - ((4.0 * (-var_eta0cv_i_dn10)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (var_eta0cv_i_dn11 + ((-var_eta0cv_i_dn11) + (0.5 * ((-(-var_eta0cv_i_dn11)) + (((((-(-var_eta0cv_i_dn11)) * assign17690_e31422) + (assign17690_e31414 * (-(-var_eta0cv_i_dn11)))) - ((4.0 * (-var_eta0cv_i_dn11)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (var_eta0cv_i_dn13 + ((-var_eta0cv_i_dn13) + (0.5 * ((-(-var_eta0cv_i_dn13)) + (((((-(-var_eta0cv_i_dn13)) * assign17690_e31422) + (assign17690_e31414 * (-(-var_eta0cv_i_dn13)))) - ((4.0 * (-var_eta0cv_i_dn13)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (var_eta0cv_i_dn14 + ((-var_eta0cv_i_dn14) + (0.5 * ((-(-var_eta0cv_i_dn14)) + (((((-(-var_eta0cv_i_dn14)) * assign17690_e31422) + (assign17690_e31414 * (-(-var_eta0cv_i_dn14)))) - ((4.0 * (-var_eta0cv_i_dn14)) * 1e-6)) / (2.0 * assign17690_e31431)))))),)
    } else {
        (var_eta0cv_t, var_eta0cv_t_dn0, var_eta0cv_t_dn2, var_eta0cv_t_dn3, var_eta0cv_t_dn4, var_eta0cv_t_dn5, var_eta0cv_t_dn6, var_eta0cv_t_dn7, var_eta0cv_t_dn8, var_eta0cv_t_dn9, var_eta0cv_t_dn10, var_eta0cv_t_dn11, var_eta0cv_t_dn13, var_eta0cv_t_dn14,)
    }
};
        var_eta0cv_t = assign17690_e31437;
        var_eta0cv_t_dn0 = assign17690_e31437_d_n0;
        var_eta0cv_t_dn2 = assign17690_e31437_d_n2;
        var_eta0cv_t_dn3 = assign17690_e31437_d_n3;
        var_eta0cv_t_dn4 = assign17690_e31437_d_n4;
        var_eta0cv_t_dn5 = assign17690_e31437_d_n5;
        var_eta0cv_t_dn6 = assign17690_e31437_d_n6;
        var_eta0cv_t_dn7 = assign17690_e31437_d_n7;
        var_eta0cv_t_dn8 = assign17690_e31437_d_n8;
        var_eta0cv_t_dn9 = assign17690_e31437_d_n9;
        var_eta0cv_t_dn10 = assign17690_e31437_d_n10;
        var_eta0cv_t_dn11 = assign17690_e31437_d_n11;
        var_eta0cv_t_dn13 = assign17690_e31437_d_n13;
        var_eta0cv_t_dn14 = assign17690_e31437_d_n14;
        var_eta0cv_t_rv = 0.0;

        let (assign17700_e31517, assign17700_e31517_d_n0, assign17700_e31517_d_n2, assign17700_e31517_d_n3, assign17700_e31517_d_n4, assign17700_e31517_d_n5, assign17700_e31517_d_n6, assign17700_e31517_d_n7, assign17700_e31517_d_n8, assign17700_e31517_d_n9, assign17700_e31517_d_n10, assign17700_e31517_d_n11, assign17700_e31517_d_n13, assign17700_e31517_d_n14,) = {
    if ((var_guard323 != 0.0) && (var_guard324 == 0.0)) {
        let assign17700_e31446: f64 = (p.p165 * var_deltemp);
        let assign17700_e31447: f64 = (1.0 + assign17700_e31446);
        let assign17700_e31449: f64 = (assign17700_e31447 - 1e-6);
        let assign17700_e31451: f64 = (-10000.0);
        let assign17700_e31453: f64 = (assign17700_e31451 * 0.001);
        let (assign17700_e31514, assign17700_e31514_d_n4,) = {
            if (!(assign17700_e31449 < assign17700_e31453)) {
                let assign17700_e31460: f64 = (p.p165 * var_deltemp);
                let assign17700_e31461: f64 = (1.0 + assign17700_e31460);
                let assign17700_e31463: f64 = (assign17700_e31461 - 1e-6);
                let assign17700_e31467: f64 = (p.p165 * var_deltemp);
                let assign17700_e31468: f64 = (1.0 + assign17700_e31467);
                let assign17700_e31470: f64 = (assign17700_e31468 - 1e-6);
                let assign17700_e31474: f64 = (p.p165 * var_deltemp);
                let assign17700_e31475: f64 = (1.0 + assign17700_e31474);
                let assign17700_e31477: f64 = (assign17700_e31475 - 1e-6);
                let assign17700_e31478: f64 = (assign17700_e31470 * assign17700_e31477);
                let assign17700_e31481: f64 = (4.0 * 0.001);
                let assign17700_e31483: f64 = (assign17700_e31481 * 0.001);
                let assign17700_e31484: f64 = (assign17700_e31478 + assign17700_e31483);
                let assign17700_e31485: f64 = (assign17700_e31484).sqrt();
                let assign17700_e31486: f64 = (assign17700_e31463 + assign17700_e31485);
                let assign17700_e31487: f64 = (0.5 * assign17700_e31486);
                (assign17700_e31487, (0.5 * ((p.p165 * var_deltemp_dn4) + ((((p.p165 * var_deltemp_dn4) * assign17700_e31477) + (assign17700_e31470 * (p.p165 * var_deltemp_dn4))) / (2.0 * assign17700_e31485)))),)
            } else {
                let assign17700_e31491: f64 = (p.p165 * var_deltemp);
                let assign17700_e31492: f64 = (1.0 + assign17700_e31491);
                let assign17700_e31494: f64 = (assign17700_e31492 - 1e-6);
                let assign17700_e31496: f64 = (-10000.0);
                let assign17700_e31498: f64 = (assign17700_e31496 * 0.001);
                let (assign17700_e31513, assign17700_e31513_d_n4,) = {
                    if (assign17700_e31494 < assign17700_e31498) {
                        let assign17700_e31501: f64 = (-0.001);
                        let assign17700_e31503: f64 = (assign17700_e31501 * 0.001);
                        let assign17700_e31507: f64 = (p.p165 * var_deltemp);
                        let assign17700_e31508: f64 = (1.0 + assign17700_e31507);
                        let assign17700_e31510: f64 = (assign17700_e31508 - 1e-6);
                        let assign17700_e31511: f64 = (assign17700_e31503 / assign17700_e31510);
                        (assign17700_e31511, (-((assign17700_e31503 * (p.p165 * var_deltemp_dn4)) / (assign17700_e31510 * assign17700_e31510))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17700_e31513, assign17700_e31513_d_n4,)
            }
        };
        let assign17700_e31515: f64 = (var_eta0cv_i * assign17700_e31514);
        (assign17700_e31515, (var_eta0cv_i_dn0 * assign17700_e31514), (var_eta0cv_i_dn2 * assign17700_e31514), (var_eta0cv_i_dn3 * assign17700_e31514), ((var_eta0cv_i_dn4 * assign17700_e31514) + (var_eta0cv_i * assign17700_e31514_d_n4)), (var_eta0cv_i_dn5 * assign17700_e31514), (var_eta0cv_i_dn6 * assign17700_e31514), (var_eta0cv_i_dn7 * assign17700_e31514), (var_eta0cv_i_dn8 * assign17700_e31514), (var_eta0cv_i_dn9 * assign17700_e31514), (var_eta0cv_i_dn10 * assign17700_e31514), (var_eta0cv_i_dn11 * assign17700_e31514), (var_eta0cv_i_dn13 * assign17700_e31514), (var_eta0cv_i_dn14 * assign17700_e31514),)
    } else {
        (var_eta0cv_t, var_eta0cv_t_dn0, var_eta0cv_t_dn2, var_eta0cv_t_dn3, var_eta0cv_t_dn4, var_eta0cv_t_dn5, var_eta0cv_t_dn6, var_eta0cv_t_dn7, var_eta0cv_t_dn8, var_eta0cv_t_dn9, var_eta0cv_t_dn10, var_eta0cv_t_dn11, var_eta0cv_t_dn13, var_eta0cv_t_dn14,)
    }
};
        var_eta0cv_t = assign17700_e31517;
        var_eta0cv_t_dn0 = assign17700_e31517_d_n0;
        var_eta0cv_t_dn2 = assign17700_e31517_d_n2;
        var_eta0cv_t_dn3 = assign17700_e31517_d_n3;
        var_eta0cv_t_dn4 = assign17700_e31517_d_n4;
        var_eta0cv_t_dn5 = assign17700_e31517_d_n5;
        var_eta0cv_t_dn6 = assign17700_e31517_d_n6;
        var_eta0cv_t_dn7 = assign17700_e31517_d_n7;
        var_eta0cv_t_dn8 = assign17700_e31517_d_n8;
        var_eta0cv_t_dn9 = assign17700_e31517_d_n9;
        var_eta0cv_t_dn10 = assign17700_e31517_d_n10;
        var_eta0cv_t_dn11 = assign17700_e31517_d_n11;
        var_eta0cv_t_dn13 = assign17700_e31517_d_n13;
        var_eta0cv_t_dn14 = assign17700_e31517_d_n14;
        var_eta0cv_t_rv = 0.0;

        let assign17710_e31520: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard325 = assign17710_e31520;
        var_guard325_rv = 0.0;

        let (assign17720_e31564, assign17720_e31564_d_n4,) = {
    if (var_guard325 != 0.0) {
        let assign17720_e31524: f64 = (-var_eta0r_i);
        let assign17720_e31528: f64 = (p.p166 * var_deltemp);
        let assign17720_e31530: f64 = (-var_eta0r_i);
        let assign17720_e31531: f64 = (assign17720_e31528 - assign17720_e31530);
        let assign17720_e31533: f64 = (assign17720_e31531 - 1e-6);
        let assign17720_e31536: f64 = (p.p166 * var_deltemp);
        let assign17720_e31538: f64 = (-var_eta0r_i);
        let assign17720_e31539: f64 = (assign17720_e31536 - assign17720_e31538);
        let assign17720_e31541: f64 = (assign17720_e31539 - 1e-6);
        let assign17720_e31544: f64 = (p.p166 * var_deltemp);
        let assign17720_e31546: f64 = (-var_eta0r_i);
        let assign17720_e31547: f64 = (assign17720_e31544 - assign17720_e31546);
        let assign17720_e31549: f64 = (assign17720_e31547 - 1e-6);
        let assign17720_e31550: f64 = (assign17720_e31541 * assign17720_e31549);
        let assign17720_e31553: f64 = (-var_eta0r_i);
        let assign17720_e31554: f64 = (4.0 * assign17720_e31553);
        let assign17720_e31556: f64 = (assign17720_e31554 * 1e-6);
        let assign17720_e31557: f64 = (assign17720_e31550 - assign17720_e31556);
        let assign17720_e31558: f64 = (assign17720_e31557).sqrt();
        let assign17720_e31559: f64 = (assign17720_e31533 + assign17720_e31558);
        let assign17720_e31560: f64 = (0.5 * assign17720_e31559);
        let assign17720_e31561: f64 = (assign17720_e31524 + assign17720_e31560);
        let assign17720_e31562: f64 = (var_eta0r_i + assign17720_e31561);
        (assign17720_e31562, (0.5 * ((p.p166 * var_deltemp_dn4) + ((((p.p166 * var_deltemp_dn4) * assign17720_e31549) + (assign17720_e31541 * (p.p166 * var_deltemp_dn4))) / (2.0 * assign17720_e31558)))),)
    } else {
        (var_eta0r_t, var_eta0r_t_dn4,)
    }
};
        var_eta0r_t = assign17720_e31564;
        var_eta0r_t_dn4 = assign17720_e31564_d_n4;
        var_eta0r_t_rv = 0.0;

        let (assign17730_e31642, assign17730_e31642_d_n4,) = {
    if (var_guard325 == 0.0) {
        let assign17730_e31571: f64 = (p.p166 * var_deltemp);
        let assign17730_e31572: f64 = (1.0 + assign17730_e31571);
        let assign17730_e31574: f64 = (assign17730_e31572 - 1e-6);
        let assign17730_e31576: f64 = (-10000.0);
        let assign17730_e31578: f64 = (assign17730_e31576 * 0.001);
        let (assign17730_e31639, assign17730_e31639_d_n4,) = {
            if (!(assign17730_e31574 < assign17730_e31578)) {
                let assign17730_e31585: f64 = (p.p166 * var_deltemp);
                let assign17730_e31586: f64 = (1.0 + assign17730_e31585);
                let assign17730_e31588: f64 = (assign17730_e31586 - 1e-6);
                let assign17730_e31592: f64 = (p.p166 * var_deltemp);
                let assign17730_e31593: f64 = (1.0 + assign17730_e31592);
                let assign17730_e31595: f64 = (assign17730_e31593 - 1e-6);
                let assign17730_e31599: f64 = (p.p166 * var_deltemp);
                let assign17730_e31600: f64 = (1.0 + assign17730_e31599);
                let assign17730_e31602: f64 = (assign17730_e31600 - 1e-6);
                let assign17730_e31603: f64 = (assign17730_e31595 * assign17730_e31602);
                let assign17730_e31606: f64 = (4.0 * 0.001);
                let assign17730_e31608: f64 = (assign17730_e31606 * 0.001);
                let assign17730_e31609: f64 = (assign17730_e31603 + assign17730_e31608);
                let assign17730_e31610: f64 = (assign17730_e31609).sqrt();
                let assign17730_e31611: f64 = (assign17730_e31588 + assign17730_e31610);
                let assign17730_e31612: f64 = (0.5 * assign17730_e31611);
                (assign17730_e31612, (0.5 * ((p.p166 * var_deltemp_dn4) + ((((p.p166 * var_deltemp_dn4) * assign17730_e31602) + (assign17730_e31595 * (p.p166 * var_deltemp_dn4))) / (2.0 * assign17730_e31610)))),)
            } else {
                let assign17730_e31616: f64 = (p.p166 * var_deltemp);
                let assign17730_e31617: f64 = (1.0 + assign17730_e31616);
                let assign17730_e31619: f64 = (assign17730_e31617 - 1e-6);
                let assign17730_e31621: f64 = (-10000.0);
                let assign17730_e31623: f64 = (assign17730_e31621 * 0.001);
                let (assign17730_e31638, assign17730_e31638_d_n4,) = {
                    if (assign17730_e31619 < assign17730_e31623) {
                        let assign17730_e31626: f64 = (-0.001);
                        let assign17730_e31628: f64 = (assign17730_e31626 * 0.001);
                        let assign17730_e31632: f64 = (p.p166 * var_deltemp);
                        let assign17730_e31633: f64 = (1.0 + assign17730_e31632);
                        let assign17730_e31635: f64 = (assign17730_e31633 - 1e-6);
                        let assign17730_e31636: f64 = (assign17730_e31628 / assign17730_e31635);
                        (assign17730_e31636, (-((assign17730_e31628 * (p.p166 * var_deltemp_dn4)) / (assign17730_e31635 * assign17730_e31635))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17730_e31638, assign17730_e31638_d_n4,)
            }
        };
        let assign17730_e31640: f64 = (var_eta0r_i * assign17730_e31639);
        (assign17730_e31640, (var_eta0r_i * assign17730_e31639_d_n4),)
    } else {
        (var_eta0r_t, var_eta0r_t_dn4,)
    }
};
        var_eta0r_t = assign17730_e31642;
        var_eta0r_t_dn4 = assign17730_e31642_d_n4;
        var_eta0r_t_rv = 0.0;

        let assign17740_e31645: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard326 = assign17740_e31645;
        var_guard326_rv = 0.0;

        let (assign17750_e31689, assign17750_e31689_d_n4,) = {
    if (var_guard326 != 0.0) {
        let assign17750_e31649: f64 = (-var_etamob_i);
        let assign17750_e31653: f64 = (var_emobt_i * var_deltemp);
        let assign17750_e31655: f64 = (-var_etamob_i);
        let assign17750_e31656: f64 = (assign17750_e31653 - assign17750_e31655);
        let assign17750_e31658: f64 = (assign17750_e31656 - 1e-6);
        let assign17750_e31661: f64 = (var_emobt_i * var_deltemp);
        let assign17750_e31663: f64 = (-var_etamob_i);
        let assign17750_e31664: f64 = (assign17750_e31661 - assign17750_e31663);
        let assign17750_e31666: f64 = (assign17750_e31664 - 1e-6);
        let assign17750_e31669: f64 = (var_emobt_i * var_deltemp);
        let assign17750_e31671: f64 = (-var_etamob_i);
        let assign17750_e31672: f64 = (assign17750_e31669 - assign17750_e31671);
        let assign17750_e31674: f64 = (assign17750_e31672 - 1e-6);
        let assign17750_e31675: f64 = (assign17750_e31666 * assign17750_e31674);
        let assign17750_e31678: f64 = (-var_etamob_i);
        let assign17750_e31679: f64 = (4.0 * assign17750_e31678);
        let assign17750_e31681: f64 = (assign17750_e31679 * 1e-6);
        let assign17750_e31682: f64 = (assign17750_e31675 - assign17750_e31681);
        let assign17750_e31683: f64 = (assign17750_e31682).sqrt();
        let assign17750_e31684: f64 = (assign17750_e31658 + assign17750_e31683);
        let assign17750_e31685: f64 = (0.5 * assign17750_e31684);
        let assign17750_e31686: f64 = (assign17750_e31649 + assign17750_e31685);
        let assign17750_e31687: f64 = (var_etamob_i + assign17750_e31686);
        (assign17750_e31687, (0.5 * ((var_emobt_i * var_deltemp_dn4) + ((((var_emobt_i * var_deltemp_dn4) * assign17750_e31674) + (assign17750_e31666 * (var_emobt_i * var_deltemp_dn4))) / (2.0 * assign17750_e31683)))),)
    } else {
        (var_etamob_t, var_etamob_t_dn4,)
    }
};
        var_etamob_t = assign17750_e31689;
        var_etamob_t_dn4 = assign17750_e31689_d_n4;
        var_etamob_t_rv = 0.0;

        let (assign17760_e31767, assign17760_e31767_d_n4,) = {
    if (var_guard326 == 0.0) {
        let assign17760_e31696: f64 = (var_emobt_i * var_deltemp);
        let assign17760_e31697: f64 = (1.0 + assign17760_e31696);
        let assign17760_e31699: f64 = (assign17760_e31697 - 1e-6);
        let assign17760_e31701: f64 = (-10000.0);
        let assign17760_e31703: f64 = (assign17760_e31701 * 0.001);
        let (assign17760_e31764, assign17760_e31764_d_n4,) = {
            if (!(assign17760_e31699 < assign17760_e31703)) {
                let assign17760_e31710: f64 = (var_emobt_i * var_deltemp);
                let assign17760_e31711: f64 = (1.0 + assign17760_e31710);
                let assign17760_e31713: f64 = (assign17760_e31711 - 1e-6);
                let assign17760_e31717: f64 = (var_emobt_i * var_deltemp);
                let assign17760_e31718: f64 = (1.0 + assign17760_e31717);
                let assign17760_e31720: f64 = (assign17760_e31718 - 1e-6);
                let assign17760_e31724: f64 = (var_emobt_i * var_deltemp);
                let assign17760_e31725: f64 = (1.0 + assign17760_e31724);
                let assign17760_e31727: f64 = (assign17760_e31725 - 1e-6);
                let assign17760_e31728: f64 = (assign17760_e31720 * assign17760_e31727);
                let assign17760_e31731: f64 = (4.0 * 0.001);
                let assign17760_e31733: f64 = (assign17760_e31731 * 0.001);
                let assign17760_e31734: f64 = (assign17760_e31728 + assign17760_e31733);
                let assign17760_e31735: f64 = (assign17760_e31734).sqrt();
                let assign17760_e31736: f64 = (assign17760_e31713 + assign17760_e31735);
                let assign17760_e31737: f64 = (0.5 * assign17760_e31736);
                (assign17760_e31737, (0.5 * ((var_emobt_i * var_deltemp_dn4) + ((((var_emobt_i * var_deltemp_dn4) * assign17760_e31727) + (assign17760_e31720 * (var_emobt_i * var_deltemp_dn4))) / (2.0 * assign17760_e31735)))),)
            } else {
                let assign17760_e31741: f64 = (var_emobt_i * var_deltemp);
                let assign17760_e31742: f64 = (1.0 + assign17760_e31741);
                let assign17760_e31744: f64 = (assign17760_e31742 - 1e-6);
                let assign17760_e31746: f64 = (-10000.0);
                let assign17760_e31748: f64 = (assign17760_e31746 * 0.001);
                let (assign17760_e31763, assign17760_e31763_d_n4,) = {
                    if (assign17760_e31744 < assign17760_e31748) {
                        let assign17760_e31751: f64 = (-0.001);
                        let assign17760_e31753: f64 = (assign17760_e31751 * 0.001);
                        let assign17760_e31757: f64 = (var_emobt_i * var_deltemp);
                        let assign17760_e31758: f64 = (1.0 + assign17760_e31757);
                        let assign17760_e31760: f64 = (assign17760_e31758 - 1e-6);
                        let assign17760_e31761: f64 = (assign17760_e31753 / assign17760_e31760);
                        (assign17760_e31761, (-((assign17760_e31753 * (var_emobt_i * var_deltemp_dn4)) / (assign17760_e31760 * assign17760_e31760))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17760_e31763, assign17760_e31763_d_n4,)
            }
        };
        let assign17760_e31765: f64 = (var_etamob_i * assign17760_e31764);
        (assign17760_e31765, (var_etamob_i * assign17760_e31764_d_n4),)
    } else {
        (var_etamob_t, var_etamob_t_dn4,)
    }
};
        var_etamob_t = assign17760_e31767;
        var_etamob_t_dn4 = assign17760_e31767_d_n4;
        var_etamob_t_rv = 0.0;

        let assign17770_e31770: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard327 = assign17770_e31770;
        var_guard327_rv = 0.0;

        let (assign17780_e31814, assign17780_e31814_d_n4,) = {
    if (var_guard327 != 0.0) {
        let assign17780_e31774: f64 = (-p.p917);
        let assign17780_e31778: f64 = (p.p923 * var_deltemp);
        let assign17780_e31780: f64 = (-p.p917);
        let assign17780_e31781: f64 = (assign17780_e31778 - assign17780_e31780);
        let assign17780_e31783: f64 = (assign17780_e31781 - 1e-6);
        let assign17780_e31786: f64 = (p.p923 * var_deltemp);
        let assign17780_e31788: f64 = (-p.p917);
        let assign17780_e31789: f64 = (assign17780_e31786 - assign17780_e31788);
        let assign17780_e31791: f64 = (assign17780_e31789 - 1e-6);
        let assign17780_e31794: f64 = (p.p923 * var_deltemp);
        let assign17780_e31796: f64 = (-p.p917);
        let assign17780_e31797: f64 = (assign17780_e31794 - assign17780_e31796);
        let assign17780_e31799: f64 = (assign17780_e31797 - 1e-6);
        let assign17780_e31800: f64 = (assign17780_e31791 * assign17780_e31799);
        let assign17780_e31803: f64 = (-p.p917);
        let assign17780_e31804: f64 = (4.0 * assign17780_e31803);
        let assign17780_e31806: f64 = (assign17780_e31804 * 1e-6);
        let assign17780_e31807: f64 = (assign17780_e31800 - assign17780_e31806);
        let assign17780_e31808: f64 = (assign17780_e31807).sqrt();
        let assign17780_e31809: f64 = (assign17780_e31783 + assign17780_e31808);
        let assign17780_e31810: f64 = (0.5 * assign17780_e31809);
        let assign17780_e31811: f64 = (assign17780_e31774 + assign17780_e31810);
        let assign17780_e31812: f64 = (p.p917 + assign17780_e31811);
        (assign17780_e31812, (0.5 * ((p.p923 * var_deltemp_dn4) + ((((p.p923 * var_deltemp_dn4) * assign17780_e31799) + (assign17780_e31791 * (p.p923 * var_deltemp_dn4))) / (2.0 * assign17780_e31808)))),)
    } else {
        (var_rsdr_t, var_rsdr_t_dn4,)
    }
};
        var_rsdr_t = assign17780_e31814;
        var_rsdr_t_dn4 = assign17780_e31814_d_n4;
        var_rsdr_t_rv = 0.0;

        *var_eta0_t_slot = var_eta0_t;
        *var_eta0_t_dn0_slot = var_eta0_t_dn0;
        *var_eta0_t_dn10_slot = var_eta0_t_dn10;
        *var_eta0_t_dn11_slot = var_eta0_t_dn11;
        *var_eta0_t_dn13_slot = var_eta0_t_dn13;
        *var_eta0_t_dn14_slot = var_eta0_t_dn14;
        *var_eta0_t_dn2_slot = var_eta0_t_dn2;
        *var_eta0_t_dn3_slot = var_eta0_t_dn3;
        *var_eta0_t_dn4_slot = var_eta0_t_dn4;
        *var_eta0_t_dn5_slot = var_eta0_t_dn5;
        *var_eta0_t_dn6_slot = var_eta0_t_dn6;
        *var_eta0_t_dn7_slot = var_eta0_t_dn7;
        *var_eta0_t_dn8_slot = var_eta0_t_dn8;
        *var_eta0_t_dn9_slot = var_eta0_t_dn9;
        *var_eta0_t_rv_slot = var_eta0_t_rv;
        *var_eta0cv_t_slot = var_eta0cv_t;
        *var_eta0cv_t_dn0_slot = var_eta0cv_t_dn0;
        *var_eta0cv_t_dn10_slot = var_eta0cv_t_dn10;
        *var_eta0cv_t_dn11_slot = var_eta0cv_t_dn11;
        *var_eta0cv_t_dn13_slot = var_eta0cv_t_dn13;
        *var_eta0cv_t_dn14_slot = var_eta0cv_t_dn14;
        *var_eta0cv_t_dn2_slot = var_eta0cv_t_dn2;
        *var_eta0cv_t_dn3_slot = var_eta0cv_t_dn3;
        *var_eta0cv_t_dn4_slot = var_eta0cv_t_dn4;
        *var_eta0cv_t_dn5_slot = var_eta0cv_t_dn5;
        *var_eta0cv_t_dn6_slot = var_eta0cv_t_dn6;
        *var_eta0cv_t_dn7_slot = var_eta0cv_t_dn7;
        *var_eta0cv_t_dn8_slot = var_eta0cv_t_dn8;
        *var_eta0cv_t_dn9_slot = var_eta0cv_t_dn9;
        *var_eta0cv_t_rv_slot = var_eta0cv_t_rv;
        *var_eta0r_t_slot = var_eta0r_t;
        *var_eta0r_t_dn4_slot = var_eta0r_t_dn4;
        *var_eta0r_t_rv_slot = var_eta0r_t_rv;
        *var_etamob_t_slot = var_etamob_t;
        *var_etamob_t_dn4_slot = var_etamob_t_dn4;
        *var_etamob_t_rv_slot = var_etamob_t_rv;
        *var_guard323_slot = var_guard323;
        *var_guard323_rv_slot = var_guard323_rv;
        *var_guard324_slot = var_guard324;
        *var_guard324_rv_slot = var_guard324_rv;
        *var_guard325_slot = var_guard325;
        *var_guard325_rv_slot = var_guard325_rv;
        *var_guard326_slot = var_guard326;
        *var_guard326_rv_slot = var_guard326_rv;
        *var_guard327_slot = var_guard327;
        *var_guard327_rv_slot = var_guard327_rv;
        *var_rsdr_t_slot = var_rsdr_t;
        *var_rsdr_t_dn4_slot = var_rsdr_t_dn4;
        *var_rsdr_t_rv_slot = var_rsdr_t_rv;
    }

    pub(super) fn stamp_reactive_block_61(
        p: &Parameters,
        var_deltemp: f64,
        var_deltemp_dn4: f64,
        var_guard327: f64,
        var_ptwg_i: f64,
        var_ptwg_i_dn0: f64,
        var_ptwg_i_dn10: f64,
        var_ptwg_i_dn11: f64,
        var_ptwg_i_dn13: f64,
        var_ptwg_i_dn14: f64,
        var_ptwg_i_dn2: f64,
        var_ptwg_i_dn3: f64,
        var_ptwg_i_dn4: f64,
        var_ptwg_i_dn5: f64,
        var_ptwg_i_dn6: f64,
        var_ptwg_i_dn7: f64,
        var_ptwg_i_dn8: f64,
        var_ptwg_i_dn9: f64,
        var_ptwgt_i: f64,
        var_guard328_slot: &mut f64,
        var_guard328_rv_slot: &mut f64,
        var_guard329_slot: &mut f64,
        var_guard329_rv_slot: &mut f64,
        var_guard330_slot: &mut f64,
        var_guard330_rv_slot: &mut f64,
        var_guard331_slot: &mut f64,
        var_guard331_rv_slot: &mut f64,
        var_guard332_slot: &mut f64,
        var_guard332_rv_slot: &mut f64,
        var_guard333_slot: &mut f64,
        var_guard333_rv_slot: &mut f64,
        var_guard334_slot: &mut f64,
        var_guard334_rv_slot: &mut f64,
        var_guard335_slot: &mut f64,
        var_guard335_rv_slot: &mut f64,
        var_ptwg_t_slot: &mut f64,
        var_ptwg_t_dn0_slot: &mut f64,
        var_ptwg_t_dn10_slot: &mut f64,
        var_ptwg_t_dn11_slot: &mut f64,
        var_ptwg_t_dn13_slot: &mut f64,
        var_ptwg_t_dn14_slot: &mut f64,
        var_ptwg_t_dn2_slot: &mut f64,
        var_ptwg_t_dn3_slot: &mut f64,
        var_ptwg_t_dn4_slot: &mut f64,
        var_ptwg_t_dn5_slot: &mut f64,
        var_ptwg_t_dn6_slot: &mut f64,
        var_ptwg_t_dn7_slot: &mut f64,
        var_ptwg_t_dn8_slot: &mut f64,
        var_ptwg_t_dn9_slot: &mut f64,
        var_ptwg_t_rv_slot: &mut f64,
        var_rddr_t_slot: &mut f64,
        var_rddr_t_dn4_slot: &mut f64,
        var_rddr_t_rv_slot: &mut f64,
        var_rddrr_t_slot: &mut f64,
        var_rddrr_t_dn4_slot: &mut f64,
        var_rddrr_t_rv_slot: &mut f64,
        var_rsdr_t_slot: &mut f64,
        var_rsdr_t_dn4_slot: &mut f64,
        var_rsdr_t_rv_slot: &mut f64,
        var_rsdrr_t_slot: &mut f64,
        var_rsdrr_t_dn4_slot: &mut f64,
        var_rsdrr_t_rv_slot: &mut f64,
    ) {
        let mut var_guard328: f64 = *var_guard328_slot;
        let mut var_guard328_rv: f64 = *var_guard328_rv_slot;
        let mut var_guard329: f64 = *var_guard329_slot;
        let mut var_guard329_rv: f64 = *var_guard329_rv_slot;
        let mut var_guard330: f64 = *var_guard330_slot;
        let mut var_guard330_rv: f64 = *var_guard330_rv_slot;
        let mut var_guard331: f64 = *var_guard331_slot;
        let mut var_guard331_rv: f64 = *var_guard331_rv_slot;
        let mut var_guard332: f64 = *var_guard332_slot;
        let mut var_guard332_rv: f64 = *var_guard332_rv_slot;
        let mut var_guard333: f64 = *var_guard333_slot;
        let mut var_guard333_rv: f64 = *var_guard333_rv_slot;
        let mut var_guard334: f64 = *var_guard334_slot;
        let mut var_guard334_rv: f64 = *var_guard334_rv_slot;
        let mut var_guard335: f64 = *var_guard335_slot;
        let mut var_guard335_rv: f64 = *var_guard335_rv_slot;
        let mut var_ptwg_t: f64 = *var_ptwg_t_slot;
        let mut var_ptwg_t_dn0: f64 = *var_ptwg_t_dn0_slot;
        let mut var_ptwg_t_dn10: f64 = *var_ptwg_t_dn10_slot;
        let mut var_ptwg_t_dn11: f64 = *var_ptwg_t_dn11_slot;
        let mut var_ptwg_t_dn13: f64 = *var_ptwg_t_dn13_slot;
        let mut var_ptwg_t_dn14: f64 = *var_ptwg_t_dn14_slot;
        let mut var_ptwg_t_dn2: f64 = *var_ptwg_t_dn2_slot;
        let mut var_ptwg_t_dn3: f64 = *var_ptwg_t_dn3_slot;
        let mut var_ptwg_t_dn4: f64 = *var_ptwg_t_dn4_slot;
        let mut var_ptwg_t_dn5: f64 = *var_ptwg_t_dn5_slot;
        let mut var_ptwg_t_dn6: f64 = *var_ptwg_t_dn6_slot;
        let mut var_ptwg_t_dn7: f64 = *var_ptwg_t_dn7_slot;
        let mut var_ptwg_t_dn8: f64 = *var_ptwg_t_dn8_slot;
        let mut var_ptwg_t_dn9: f64 = *var_ptwg_t_dn9_slot;
        let mut var_ptwg_t_rv: f64 = *var_ptwg_t_rv_slot;
        let mut var_rddr_t: f64 = *var_rddr_t_slot;
        let mut var_rddr_t_dn4: f64 = *var_rddr_t_dn4_slot;
        let mut var_rddr_t_rv: f64 = *var_rddr_t_rv_slot;
        let mut var_rddrr_t: f64 = *var_rddrr_t_slot;
        let mut var_rddrr_t_dn4: f64 = *var_rddrr_t_dn4_slot;
        let mut var_rddrr_t_rv: f64 = *var_rddrr_t_rv_slot;
        let mut var_rsdr_t: f64 = *var_rsdr_t_slot;
        let mut var_rsdr_t_dn4: f64 = *var_rsdr_t_dn4_slot;
        let mut var_rsdr_t_rv: f64 = *var_rsdr_t_rv_slot;
        let mut var_rsdrr_t: f64 = *var_rsdrr_t_slot;
        let mut var_rsdrr_t_dn4: f64 = *var_rsdrr_t_dn4_slot;
        let mut var_rsdrr_t_rv: f64 = *var_rsdrr_t_rv_slot;

        let (assign17790_e31892, assign17790_e31892_d_n4,) = {
    if (var_guard327 == 0.0) {
        let assign17790_e31821: f64 = (p.p923 * var_deltemp);
        let assign17790_e31822: f64 = (1.0 + assign17790_e31821);
        let assign17790_e31824: f64 = (assign17790_e31822 - 1e-6);
        let assign17790_e31826: f64 = (-10000.0);
        let assign17790_e31828: f64 = (assign17790_e31826 * 0.001);
        let (assign17790_e31889, assign17790_e31889_d_n4,) = {
            if (!(assign17790_e31824 < assign17790_e31828)) {
                let assign17790_e31835: f64 = (p.p923 * var_deltemp);
                let assign17790_e31836: f64 = (1.0 + assign17790_e31835);
                let assign17790_e31838: f64 = (assign17790_e31836 - 1e-6);
                let assign17790_e31842: f64 = (p.p923 * var_deltemp);
                let assign17790_e31843: f64 = (1.0 + assign17790_e31842);
                let assign17790_e31845: f64 = (assign17790_e31843 - 1e-6);
                let assign17790_e31849: f64 = (p.p923 * var_deltemp);
                let assign17790_e31850: f64 = (1.0 + assign17790_e31849);
                let assign17790_e31852: f64 = (assign17790_e31850 - 1e-6);
                let assign17790_e31853: f64 = (assign17790_e31845 * assign17790_e31852);
                let assign17790_e31856: f64 = (4.0 * 0.001);
                let assign17790_e31858: f64 = (assign17790_e31856 * 0.001);
                let assign17790_e31859: f64 = (assign17790_e31853 + assign17790_e31858);
                let assign17790_e31860: f64 = (assign17790_e31859).sqrt();
                let assign17790_e31861: f64 = (assign17790_e31838 + assign17790_e31860);
                let assign17790_e31862: f64 = (0.5 * assign17790_e31861);
                (assign17790_e31862, (0.5 * ((p.p923 * var_deltemp_dn4) + ((((p.p923 * var_deltemp_dn4) * assign17790_e31852) + (assign17790_e31845 * (p.p923 * var_deltemp_dn4))) / (2.0 * assign17790_e31860)))),)
            } else {
                let assign17790_e31866: f64 = (p.p923 * var_deltemp);
                let assign17790_e31867: f64 = (1.0 + assign17790_e31866);
                let assign17790_e31869: f64 = (assign17790_e31867 - 1e-6);
                let assign17790_e31871: f64 = (-10000.0);
                let assign17790_e31873: f64 = (assign17790_e31871 * 0.001);
                let (assign17790_e31888, assign17790_e31888_d_n4,) = {
                    if (assign17790_e31869 < assign17790_e31873) {
                        let assign17790_e31876: f64 = (-0.001);
                        let assign17790_e31878: f64 = (assign17790_e31876 * 0.001);
                        let assign17790_e31882: f64 = (p.p923 * var_deltemp);
                        let assign17790_e31883: f64 = (1.0 + assign17790_e31882);
                        let assign17790_e31885: f64 = (assign17790_e31883 - 1e-6);
                        let assign17790_e31886: f64 = (assign17790_e31878 / assign17790_e31885);
                        (assign17790_e31886, (-((assign17790_e31878 * (p.p923 * var_deltemp_dn4)) / (assign17790_e31885 * assign17790_e31885))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17790_e31888, assign17790_e31888_d_n4,)
            }
        };
        let assign17790_e31890: f64 = (p.p917 * assign17790_e31889);
        (assign17790_e31890, (p.p917 * assign17790_e31889_d_n4),)
    } else {
        (var_rsdr_t, var_rsdr_t_dn4,)
    }
};
        var_rsdr_t = assign17790_e31892;
        var_rsdr_t_dn4 = assign17790_e31892_d_n4;
        var_rsdr_t_rv = 0.0;

        let assign17800_e31895: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        var_guard328 = assign17800_e31895;
        var_guard328_rv = 0.0;

        let assign17810_e31898: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard329 = assign17810_e31898;
        var_guard329_rv = 0.0;

        let (assign17820_e31944, assign17820_e31944_d_n4,) = {
    if ((var_guard328 != 0.0) && (var_guard329 != 0.0)) {
        let assign17820_e31904: f64 = (-p.p918);
        let assign17820_e31908: f64 = (p.p923 * var_deltemp);
        let assign17820_e31910: f64 = (-p.p918);
        let assign17820_e31911: f64 = (assign17820_e31908 - assign17820_e31910);
        let assign17820_e31913: f64 = (assign17820_e31911 - 1e-6);
        let assign17820_e31916: f64 = (p.p923 * var_deltemp);
        let assign17820_e31918: f64 = (-p.p918);
        let assign17820_e31919: f64 = (assign17820_e31916 - assign17820_e31918);
        let assign17820_e31921: f64 = (assign17820_e31919 - 1e-6);
        let assign17820_e31924: f64 = (p.p923 * var_deltemp);
        let assign17820_e31926: f64 = (-p.p918);
        let assign17820_e31927: f64 = (assign17820_e31924 - assign17820_e31926);
        let assign17820_e31929: f64 = (assign17820_e31927 - 1e-6);
        let assign17820_e31930: f64 = (assign17820_e31921 * assign17820_e31929);
        let assign17820_e31933: f64 = (-p.p918);
        let assign17820_e31934: f64 = (4.0 * assign17820_e31933);
        let assign17820_e31936: f64 = (assign17820_e31934 * 1e-6);
        let assign17820_e31937: f64 = (assign17820_e31930 - assign17820_e31936);
        let assign17820_e31938: f64 = (assign17820_e31937).sqrt();
        let assign17820_e31939: f64 = (assign17820_e31913 + assign17820_e31938);
        let assign17820_e31940: f64 = (0.5 * assign17820_e31939);
        let assign17820_e31941: f64 = (assign17820_e31904 + assign17820_e31940);
        let assign17820_e31942: f64 = (p.p918 + assign17820_e31941);
        (assign17820_e31942, (0.5 * ((p.p923 * var_deltemp_dn4) + ((((p.p923 * var_deltemp_dn4) * assign17820_e31929) + (assign17820_e31921 * (p.p923 * var_deltemp_dn4))) / (2.0 * assign17820_e31938)))),)
    } else {
        (var_rsdrr_t, var_rsdrr_t_dn4,)
    }
};
        var_rsdrr_t = assign17820_e31944;
        var_rsdrr_t_dn4 = assign17820_e31944_d_n4;
        var_rsdrr_t_rv = 0.0;

        let (assign17830_e32024, assign17830_e32024_d_n4,) = {
    if ((var_guard328 != 0.0) && (var_guard329 == 0.0)) {
        let assign17830_e31953: f64 = (p.p923 * var_deltemp);
        let assign17830_e31954: f64 = (1.0 + assign17830_e31953);
        let assign17830_e31956: f64 = (assign17830_e31954 - 1e-6);
        let assign17830_e31958: f64 = (-10000.0);
        let assign17830_e31960: f64 = (assign17830_e31958 * 0.001);
        let (assign17830_e32021, assign17830_e32021_d_n4,) = {
            if (!(assign17830_e31956 < assign17830_e31960)) {
                let assign17830_e31967: f64 = (p.p923 * var_deltemp);
                let assign17830_e31968: f64 = (1.0 + assign17830_e31967);
                let assign17830_e31970: f64 = (assign17830_e31968 - 1e-6);
                let assign17830_e31974: f64 = (p.p923 * var_deltemp);
                let assign17830_e31975: f64 = (1.0 + assign17830_e31974);
                let assign17830_e31977: f64 = (assign17830_e31975 - 1e-6);
                let assign17830_e31981: f64 = (p.p923 * var_deltemp);
                let assign17830_e31982: f64 = (1.0 + assign17830_e31981);
                let assign17830_e31984: f64 = (assign17830_e31982 - 1e-6);
                let assign17830_e31985: f64 = (assign17830_e31977 * assign17830_e31984);
                let assign17830_e31988: f64 = (4.0 * 0.001);
                let assign17830_e31990: f64 = (assign17830_e31988 * 0.001);
                let assign17830_e31991: f64 = (assign17830_e31985 + assign17830_e31990);
                let assign17830_e31992: f64 = (assign17830_e31991).sqrt();
                let assign17830_e31993: f64 = (assign17830_e31970 + assign17830_e31992);
                let assign17830_e31994: f64 = (0.5 * assign17830_e31993);
                (assign17830_e31994, (0.5 * ((p.p923 * var_deltemp_dn4) + ((((p.p923 * var_deltemp_dn4) * assign17830_e31984) + (assign17830_e31977 * (p.p923 * var_deltemp_dn4))) / (2.0 * assign17830_e31992)))),)
            } else {
                let assign17830_e31998: f64 = (p.p923 * var_deltemp);
                let assign17830_e31999: f64 = (1.0 + assign17830_e31998);
                let assign17830_e32001: f64 = (assign17830_e31999 - 1e-6);
                let assign17830_e32003: f64 = (-10000.0);
                let assign17830_e32005: f64 = (assign17830_e32003 * 0.001);
                let (assign17830_e32020, assign17830_e32020_d_n4,) = {
                    if (assign17830_e32001 < assign17830_e32005) {
                        let assign17830_e32008: f64 = (-0.001);
                        let assign17830_e32010: f64 = (assign17830_e32008 * 0.001);
                        let assign17830_e32014: f64 = (p.p923 * var_deltemp);
                        let assign17830_e32015: f64 = (1.0 + assign17830_e32014);
                        let assign17830_e32017: f64 = (assign17830_e32015 - 1e-6);
                        let assign17830_e32018: f64 = (assign17830_e32010 / assign17830_e32017);
                        (assign17830_e32018, (-((assign17830_e32010 * (p.p923 * var_deltemp_dn4)) / (assign17830_e32017 * assign17830_e32017))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17830_e32020, assign17830_e32020_d_n4,)
            }
        };
        let assign17830_e32022: f64 = (p.p918 * assign17830_e32021);
        (assign17830_e32022, (p.p918 * assign17830_e32021_d_n4),)
    } else {
        (var_rsdrr_t, var_rsdrr_t_dn4,)
    }
};
        var_rsdrr_t = assign17830_e32024;
        var_rsdrr_t_dn4 = assign17830_e32024_d_n4;
        var_rsdrr_t_rv = 0.0;

        let assign17840_e32027: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard330 = assign17840_e32027;
        var_guard330_rv = 0.0;

        let (assign17850_e32071, assign17850_e32071_d_n4,) = {
    if (var_guard330 != 0.0) {
        let assign17850_e32031: f64 = (-p.p919);
        let assign17850_e32035: f64 = (p.p924 * var_deltemp);
        let assign17850_e32037: f64 = (-p.p919);
        let assign17850_e32038: f64 = (assign17850_e32035 - assign17850_e32037);
        let assign17850_e32040: f64 = (assign17850_e32038 - 1e-6);
        let assign17850_e32043: f64 = (p.p924 * var_deltemp);
        let assign17850_e32045: f64 = (-p.p919);
        let assign17850_e32046: f64 = (assign17850_e32043 - assign17850_e32045);
        let assign17850_e32048: f64 = (assign17850_e32046 - 1e-6);
        let assign17850_e32051: f64 = (p.p924 * var_deltemp);
        let assign17850_e32053: f64 = (-p.p919);
        let assign17850_e32054: f64 = (assign17850_e32051 - assign17850_e32053);
        let assign17850_e32056: f64 = (assign17850_e32054 - 1e-6);
        let assign17850_e32057: f64 = (assign17850_e32048 * assign17850_e32056);
        let assign17850_e32060: f64 = (-p.p919);
        let assign17850_e32061: f64 = (4.0 * assign17850_e32060);
        let assign17850_e32063: f64 = (assign17850_e32061 * 1e-6);
        let assign17850_e32064: f64 = (assign17850_e32057 - assign17850_e32063);
        let assign17850_e32065: f64 = (assign17850_e32064).sqrt();
        let assign17850_e32066: f64 = (assign17850_e32040 + assign17850_e32065);
        let assign17850_e32067: f64 = (0.5 * assign17850_e32066);
        let assign17850_e32068: f64 = (assign17850_e32031 + assign17850_e32067);
        let assign17850_e32069: f64 = (p.p919 + assign17850_e32068);
        (assign17850_e32069, (0.5 * ((p.p924 * var_deltemp_dn4) + ((((p.p924 * var_deltemp_dn4) * assign17850_e32056) + (assign17850_e32048 * (p.p924 * var_deltemp_dn4))) / (2.0 * assign17850_e32065)))),)
    } else {
        (var_rddr_t, var_rddr_t_dn4,)
    }
};
        var_rddr_t = assign17850_e32071;
        var_rddr_t_dn4 = assign17850_e32071_d_n4;
        var_rddr_t_rv = 0.0;

        let (assign17860_e32149, assign17860_e32149_d_n4,) = {
    if (var_guard330 == 0.0) {
        let assign17860_e32078: f64 = (p.p924 * var_deltemp);
        let assign17860_e32079: f64 = (1.0 + assign17860_e32078);
        let assign17860_e32081: f64 = (assign17860_e32079 - 1e-6);
        let assign17860_e32083: f64 = (-10000.0);
        let assign17860_e32085: f64 = (assign17860_e32083 * 0.001);
        let (assign17860_e32146, assign17860_e32146_d_n4,) = {
            if (!(assign17860_e32081 < assign17860_e32085)) {
                let assign17860_e32092: f64 = (p.p924 * var_deltemp);
                let assign17860_e32093: f64 = (1.0 + assign17860_e32092);
                let assign17860_e32095: f64 = (assign17860_e32093 - 1e-6);
                let assign17860_e32099: f64 = (p.p924 * var_deltemp);
                let assign17860_e32100: f64 = (1.0 + assign17860_e32099);
                let assign17860_e32102: f64 = (assign17860_e32100 - 1e-6);
                let assign17860_e32106: f64 = (p.p924 * var_deltemp);
                let assign17860_e32107: f64 = (1.0 + assign17860_e32106);
                let assign17860_e32109: f64 = (assign17860_e32107 - 1e-6);
                let assign17860_e32110: f64 = (assign17860_e32102 * assign17860_e32109);
                let assign17860_e32113: f64 = (4.0 * 0.001);
                let assign17860_e32115: f64 = (assign17860_e32113 * 0.001);
                let assign17860_e32116: f64 = (assign17860_e32110 + assign17860_e32115);
                let assign17860_e32117: f64 = (assign17860_e32116).sqrt();
                let assign17860_e32118: f64 = (assign17860_e32095 + assign17860_e32117);
                let assign17860_e32119: f64 = (0.5 * assign17860_e32118);
                (assign17860_e32119, (0.5 * ((p.p924 * var_deltemp_dn4) + ((((p.p924 * var_deltemp_dn4) * assign17860_e32109) + (assign17860_e32102 * (p.p924 * var_deltemp_dn4))) / (2.0 * assign17860_e32117)))),)
            } else {
                let assign17860_e32123: f64 = (p.p924 * var_deltemp);
                let assign17860_e32124: f64 = (1.0 + assign17860_e32123);
                let assign17860_e32126: f64 = (assign17860_e32124 - 1e-6);
                let assign17860_e32128: f64 = (-10000.0);
                let assign17860_e32130: f64 = (assign17860_e32128 * 0.001);
                let (assign17860_e32145, assign17860_e32145_d_n4,) = {
                    if (assign17860_e32126 < assign17860_e32130) {
                        let assign17860_e32133: f64 = (-0.001);
                        let assign17860_e32135: f64 = (assign17860_e32133 * 0.001);
                        let assign17860_e32139: f64 = (p.p924 * var_deltemp);
                        let assign17860_e32140: f64 = (1.0 + assign17860_e32139);
                        let assign17860_e32142: f64 = (assign17860_e32140 - 1e-6);
                        let assign17860_e32143: f64 = (assign17860_e32135 / assign17860_e32142);
                        (assign17860_e32143, (-((assign17860_e32135 * (p.p924 * var_deltemp_dn4)) / (assign17860_e32142 * assign17860_e32142))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17860_e32145, assign17860_e32145_d_n4,)
            }
        };
        let assign17860_e32147: f64 = (p.p919 * assign17860_e32146);
        (assign17860_e32147, (p.p919 * assign17860_e32146_d_n4),)
    } else {
        (var_rddr_t, var_rddr_t_dn4,)
    }
};
        var_rddr_t = assign17860_e32149;
        var_rddr_t_dn4 = assign17860_e32149_d_n4;
        var_rddr_t_rv = 0.0;

        let assign17870_e32152: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        var_guard331 = assign17870_e32152;
        var_guard331_rv = 0.0;

        let assign17880_e32155: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard332 = assign17880_e32155;
        var_guard332_rv = 0.0;

        let (assign17890_e32201, assign17890_e32201_d_n4,) = {
    if ((var_guard331 != 0.0) && (var_guard332 != 0.0)) {
        let assign17890_e32161: f64 = (-p.p920);
        let assign17890_e32165: f64 = (p.p924 * var_deltemp);
        let assign17890_e32167: f64 = (-p.p920);
        let assign17890_e32168: f64 = (assign17890_e32165 - assign17890_e32167);
        let assign17890_e32170: f64 = (assign17890_e32168 - 1e-6);
        let assign17890_e32173: f64 = (p.p924 * var_deltemp);
        let assign17890_e32175: f64 = (-p.p920);
        let assign17890_e32176: f64 = (assign17890_e32173 - assign17890_e32175);
        let assign17890_e32178: f64 = (assign17890_e32176 - 1e-6);
        let assign17890_e32181: f64 = (p.p924 * var_deltemp);
        let assign17890_e32183: f64 = (-p.p920);
        let assign17890_e32184: f64 = (assign17890_e32181 - assign17890_e32183);
        let assign17890_e32186: f64 = (assign17890_e32184 - 1e-6);
        let assign17890_e32187: f64 = (assign17890_e32178 * assign17890_e32186);
        let assign17890_e32190: f64 = (-p.p920);
        let assign17890_e32191: f64 = (4.0 * assign17890_e32190);
        let assign17890_e32193: f64 = (assign17890_e32191 * 1e-6);
        let assign17890_e32194: f64 = (assign17890_e32187 - assign17890_e32193);
        let assign17890_e32195: f64 = (assign17890_e32194).sqrt();
        let assign17890_e32196: f64 = (assign17890_e32170 + assign17890_e32195);
        let assign17890_e32197: f64 = (0.5 * assign17890_e32196);
        let assign17890_e32198: f64 = (assign17890_e32161 + assign17890_e32197);
        let assign17890_e32199: f64 = (p.p920 + assign17890_e32198);
        (assign17890_e32199, (0.5 * ((p.p924 * var_deltemp_dn4) + ((((p.p924 * var_deltemp_dn4) * assign17890_e32186) + (assign17890_e32178 * (p.p924 * var_deltemp_dn4))) / (2.0 * assign17890_e32195)))),)
    } else {
        (var_rddrr_t, var_rddrr_t_dn4,)
    }
};
        var_rddrr_t = assign17890_e32201;
        var_rddrr_t_dn4 = assign17890_e32201_d_n4;
        var_rddrr_t_rv = 0.0;

        let (assign17900_e32281, assign17900_e32281_d_n4,) = {
    if ((var_guard331 != 0.0) && (var_guard332 == 0.0)) {
        let assign17900_e32210: f64 = (p.p924 * var_deltemp);
        let assign17900_e32211: f64 = (1.0 + assign17900_e32210);
        let assign17900_e32213: f64 = (assign17900_e32211 - 1e-6);
        let assign17900_e32215: f64 = (-10000.0);
        let assign17900_e32217: f64 = (assign17900_e32215 * 0.001);
        let (assign17900_e32278, assign17900_e32278_d_n4,) = {
            if (!(assign17900_e32213 < assign17900_e32217)) {
                let assign17900_e32224: f64 = (p.p924 * var_deltemp);
                let assign17900_e32225: f64 = (1.0 + assign17900_e32224);
                let assign17900_e32227: f64 = (assign17900_e32225 - 1e-6);
                let assign17900_e32231: f64 = (p.p924 * var_deltemp);
                let assign17900_e32232: f64 = (1.0 + assign17900_e32231);
                let assign17900_e32234: f64 = (assign17900_e32232 - 1e-6);
                let assign17900_e32238: f64 = (p.p924 * var_deltemp);
                let assign17900_e32239: f64 = (1.0 + assign17900_e32238);
                let assign17900_e32241: f64 = (assign17900_e32239 - 1e-6);
                let assign17900_e32242: f64 = (assign17900_e32234 * assign17900_e32241);
                let assign17900_e32245: f64 = (4.0 * 0.001);
                let assign17900_e32247: f64 = (assign17900_e32245 * 0.001);
                let assign17900_e32248: f64 = (assign17900_e32242 + assign17900_e32247);
                let assign17900_e32249: f64 = (assign17900_e32248).sqrt();
                let assign17900_e32250: f64 = (assign17900_e32227 + assign17900_e32249);
                let assign17900_e32251: f64 = (0.5 * assign17900_e32250);
                (assign17900_e32251, (0.5 * ((p.p924 * var_deltemp_dn4) + ((((p.p924 * var_deltemp_dn4) * assign17900_e32241) + (assign17900_e32234 * (p.p924 * var_deltemp_dn4))) / (2.0 * assign17900_e32249)))),)
            } else {
                let assign17900_e32255: f64 = (p.p924 * var_deltemp);
                let assign17900_e32256: f64 = (1.0 + assign17900_e32255);
                let assign17900_e32258: f64 = (assign17900_e32256 - 1e-6);
                let assign17900_e32260: f64 = (-10000.0);
                let assign17900_e32262: f64 = (assign17900_e32260 * 0.001);
                let (assign17900_e32277, assign17900_e32277_d_n4,) = {
                    if (assign17900_e32258 < assign17900_e32262) {
                        let assign17900_e32265: f64 = (-0.001);
                        let assign17900_e32267: f64 = (assign17900_e32265 * 0.001);
                        let assign17900_e32271: f64 = (p.p924 * var_deltemp);
                        let assign17900_e32272: f64 = (1.0 + assign17900_e32271);
                        let assign17900_e32274: f64 = (assign17900_e32272 - 1e-6);
                        let assign17900_e32275: f64 = (assign17900_e32267 / assign17900_e32274);
                        (assign17900_e32275, (-((assign17900_e32267 * (p.p924 * var_deltemp_dn4)) / (assign17900_e32274 * assign17900_e32274))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17900_e32277, assign17900_e32277_d_n4,)
            }
        };
        let assign17900_e32279: f64 = (p.p920 * assign17900_e32278);
        (assign17900_e32279, (p.p920 * assign17900_e32278_d_n4),)
    } else {
        (var_rddrr_t, var_rddrr_t_dn4,)
    }
};
        var_rddrr_t = assign17900_e32281;
        var_rddrr_t_dn4 = assign17900_e32281_d_n4;
        var_rddrr_t_rv = 0.0;

        let assign17910_e32284: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard333 = assign17910_e32284;
        var_guard333_rv = 0.0;

        let (assign17920_e32331, assign17920_e32331_d_n0, assign17920_e32331_d_n2, assign17920_e32331_d_n3, assign17920_e32331_d_n4, assign17920_e32331_d_n5, assign17920_e32331_d_n6, assign17920_e32331_d_n7, assign17920_e32331_d_n8, assign17920_e32331_d_n9, assign17920_e32331_d_n10, assign17920_e32331_d_n11, assign17920_e32331_d_n13, assign17920_e32331_d_n14,) = {
    if (var_guard333 != 0.0) {
        let assign17920_e32288: f64 = (-var_ptwg_i);
        let assign17920_e32291: f64 = (-var_ptwgt_i);
        let assign17920_e32293: f64 = (assign17920_e32291 * var_deltemp);
        let assign17920_e32295: f64 = (-var_ptwg_i);
        let assign17920_e32296: f64 = (assign17920_e32293 - assign17920_e32295);
        let assign17920_e32298: f64 = (assign17920_e32296 - 1e-6);
        let assign17920_e32300: f64 = (-var_ptwgt_i);
        let assign17920_e32302: f64 = (assign17920_e32300 * var_deltemp);
        let assign17920_e32304: f64 = (-var_ptwg_i);
        let assign17920_e32305: f64 = (assign17920_e32302 - assign17920_e32304);
        let assign17920_e32307: f64 = (assign17920_e32305 - 1e-6);
        let assign17920_e32309: f64 = (-var_ptwgt_i);
        let assign17920_e32311: f64 = (assign17920_e32309 * var_deltemp);
        let assign17920_e32313: f64 = (-var_ptwg_i);
        let assign17920_e32314: f64 = (assign17920_e32311 - assign17920_e32313);
        let assign17920_e32316: f64 = (assign17920_e32314 - 1e-6);
        let assign17920_e32317: f64 = (assign17920_e32307 * assign17920_e32316);
        let assign17920_e32320: f64 = (-var_ptwg_i);
        let assign17920_e32321: f64 = (4.0 * assign17920_e32320);
        let assign17920_e32323: f64 = (assign17920_e32321 * 1e-6);
        let assign17920_e32324: f64 = (assign17920_e32317 - assign17920_e32323);
        let assign17920_e32325: f64 = (assign17920_e32324).sqrt();
        let assign17920_e32326: f64 = (assign17920_e32298 + assign17920_e32325);
        let assign17920_e32327: f64 = (0.5 * assign17920_e32326);
        let assign17920_e32328: f64 = (assign17920_e32288 + assign17920_e32327);
        let assign17920_e32329: f64 = (var_ptwg_i + assign17920_e32328);
        (assign17920_e32329, (var_ptwg_i_dn0 + ((-var_ptwg_i_dn0) + (0.5 * ((-(-var_ptwg_i_dn0)) + (((((-(-var_ptwg_i_dn0)) * assign17920_e32316) + (assign17920_e32307 * (-(-var_ptwg_i_dn0)))) - ((4.0 * (-var_ptwg_i_dn0)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (var_ptwg_i_dn2 + ((-var_ptwg_i_dn2) + (0.5 * ((-(-var_ptwg_i_dn2)) + (((((-(-var_ptwg_i_dn2)) * assign17920_e32316) + (assign17920_e32307 * (-(-var_ptwg_i_dn2)))) - ((4.0 * (-var_ptwg_i_dn2)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (var_ptwg_i_dn3 + ((-var_ptwg_i_dn3) + (0.5 * ((-(-var_ptwg_i_dn3)) + (((((-(-var_ptwg_i_dn3)) * assign17920_e32316) + (assign17920_e32307 * (-(-var_ptwg_i_dn3)))) - ((4.0 * (-var_ptwg_i_dn3)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (var_ptwg_i_dn4 + ((-var_ptwg_i_dn4) + (0.5 * (((assign17920_e32291 * var_deltemp_dn4) - (-var_ptwg_i_dn4)) + ((((((assign17920_e32300 * var_deltemp_dn4) - (-var_ptwg_i_dn4)) * assign17920_e32316) + (assign17920_e32307 * ((assign17920_e32309 * var_deltemp_dn4) - (-var_ptwg_i_dn4)))) - ((4.0 * (-var_ptwg_i_dn4)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (var_ptwg_i_dn5 + ((-var_ptwg_i_dn5) + (0.5 * ((-(-var_ptwg_i_dn5)) + (((((-(-var_ptwg_i_dn5)) * assign17920_e32316) + (assign17920_e32307 * (-(-var_ptwg_i_dn5)))) - ((4.0 * (-var_ptwg_i_dn5)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (var_ptwg_i_dn6 + ((-var_ptwg_i_dn6) + (0.5 * ((-(-var_ptwg_i_dn6)) + (((((-(-var_ptwg_i_dn6)) * assign17920_e32316) + (assign17920_e32307 * (-(-var_ptwg_i_dn6)))) - ((4.0 * (-var_ptwg_i_dn6)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (var_ptwg_i_dn7 + ((-var_ptwg_i_dn7) + (0.5 * ((-(-var_ptwg_i_dn7)) + (((((-(-var_ptwg_i_dn7)) * assign17920_e32316) + (assign17920_e32307 * (-(-var_ptwg_i_dn7)))) - ((4.0 * (-var_ptwg_i_dn7)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (var_ptwg_i_dn8 + ((-var_ptwg_i_dn8) + (0.5 * ((-(-var_ptwg_i_dn8)) + (((((-(-var_ptwg_i_dn8)) * assign17920_e32316) + (assign17920_e32307 * (-(-var_ptwg_i_dn8)))) - ((4.0 * (-var_ptwg_i_dn8)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (var_ptwg_i_dn9 + ((-var_ptwg_i_dn9) + (0.5 * ((-(-var_ptwg_i_dn9)) + (((((-(-var_ptwg_i_dn9)) * assign17920_e32316) + (assign17920_e32307 * (-(-var_ptwg_i_dn9)))) - ((4.0 * (-var_ptwg_i_dn9)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (var_ptwg_i_dn10 + ((-var_ptwg_i_dn10) + (0.5 * ((-(-var_ptwg_i_dn10)) + (((((-(-var_ptwg_i_dn10)) * assign17920_e32316) + (assign17920_e32307 * (-(-var_ptwg_i_dn10)))) - ((4.0 * (-var_ptwg_i_dn10)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (var_ptwg_i_dn11 + ((-var_ptwg_i_dn11) + (0.5 * ((-(-var_ptwg_i_dn11)) + (((((-(-var_ptwg_i_dn11)) * assign17920_e32316) + (assign17920_e32307 * (-(-var_ptwg_i_dn11)))) - ((4.0 * (-var_ptwg_i_dn11)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (var_ptwg_i_dn13 + ((-var_ptwg_i_dn13) + (0.5 * ((-(-var_ptwg_i_dn13)) + (((((-(-var_ptwg_i_dn13)) * assign17920_e32316) + (assign17920_e32307 * (-(-var_ptwg_i_dn13)))) - ((4.0 * (-var_ptwg_i_dn13)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (var_ptwg_i_dn14 + ((-var_ptwg_i_dn14) + (0.5 * ((-(-var_ptwg_i_dn14)) + (((((-(-var_ptwg_i_dn14)) * assign17920_e32316) + (assign17920_e32307 * (-(-var_ptwg_i_dn14)))) - ((4.0 * (-var_ptwg_i_dn14)) * 1e-6)) / (2.0 * assign17920_e32325)))))),)
    } else {
        (var_ptwg_t, var_ptwg_t_dn0, var_ptwg_t_dn2, var_ptwg_t_dn3, var_ptwg_t_dn4, var_ptwg_t_dn5, var_ptwg_t_dn6, var_ptwg_t_dn7, var_ptwg_t_dn8, var_ptwg_t_dn9, var_ptwg_t_dn10, var_ptwg_t_dn11, var_ptwg_t_dn13, var_ptwg_t_dn14,)
    }
};
        var_ptwg_t = assign17920_e32331;
        var_ptwg_t_dn0 = assign17920_e32331_d_n0;
        var_ptwg_t_dn2 = assign17920_e32331_d_n2;
        var_ptwg_t_dn3 = assign17920_e32331_d_n3;
        var_ptwg_t_dn4 = assign17920_e32331_d_n4;
        var_ptwg_t_dn5 = assign17920_e32331_d_n5;
        var_ptwg_t_dn6 = assign17920_e32331_d_n6;
        var_ptwg_t_dn7 = assign17920_e32331_d_n7;
        var_ptwg_t_dn8 = assign17920_e32331_d_n8;
        var_ptwg_t_dn9 = assign17920_e32331_d_n9;
        var_ptwg_t_dn10 = assign17920_e32331_d_n10;
        var_ptwg_t_dn11 = assign17920_e32331_d_n11;
        var_ptwg_t_dn13 = assign17920_e32331_d_n13;
        var_ptwg_t_dn14 = assign17920_e32331_d_n14;
        var_ptwg_t_rv = 0.0;

        let (assign17930_e32415, assign17930_e32415_d_n0, assign17930_e32415_d_n2, assign17930_e32415_d_n3, assign17930_e32415_d_n4, assign17930_e32415_d_n5, assign17930_e32415_d_n6, assign17930_e32415_d_n7, assign17930_e32415_d_n8, assign17930_e32415_d_n9, assign17930_e32415_d_n10, assign17930_e32415_d_n11, assign17930_e32415_d_n13, assign17930_e32415_d_n14,) = {
    if (var_guard333 == 0.0) {
        let assign17930_e32337: f64 = (-var_ptwgt_i);
        let assign17930_e32339: f64 = (assign17930_e32337 * var_deltemp);
        let assign17930_e32340: f64 = (1.0 + assign17930_e32339);
        let assign17930_e32342: f64 = (assign17930_e32340 - 1e-6);
        let assign17930_e32344: f64 = (-10000.0);
        let assign17930_e32346: f64 = (assign17930_e32344 * 0.001);
        let (assign17930_e32412, assign17930_e32412_d_n4,) = {
            if (!(assign17930_e32342 < assign17930_e32346)) {
                let assign17930_e32352: f64 = (-var_ptwgt_i);
                let assign17930_e32354: f64 = (assign17930_e32352 * var_deltemp);
                let assign17930_e32355: f64 = (1.0 + assign17930_e32354);
                let assign17930_e32357: f64 = (assign17930_e32355 - 1e-6);
                let assign17930_e32360: f64 = (-var_ptwgt_i);
                let assign17930_e32362: f64 = (assign17930_e32360 * var_deltemp);
                let assign17930_e32363: f64 = (1.0 + assign17930_e32362);
                let assign17930_e32365: f64 = (assign17930_e32363 - 1e-6);
                let assign17930_e32368: f64 = (-var_ptwgt_i);
                let assign17930_e32370: f64 = (assign17930_e32368 * var_deltemp);
                let assign17930_e32371: f64 = (1.0 + assign17930_e32370);
                let assign17930_e32373: f64 = (assign17930_e32371 - 1e-6);
                let assign17930_e32374: f64 = (assign17930_e32365 * assign17930_e32373);
                let assign17930_e32377: f64 = (4.0 * 0.001);
                let assign17930_e32379: f64 = (assign17930_e32377 * 0.001);
                let assign17930_e32380: f64 = (assign17930_e32374 + assign17930_e32379);
                let assign17930_e32381: f64 = (assign17930_e32380).sqrt();
                let assign17930_e32382: f64 = (assign17930_e32357 + assign17930_e32381);
                let assign17930_e32383: f64 = (0.5 * assign17930_e32382);
                (assign17930_e32383, (0.5 * ((assign17930_e32352 * var_deltemp_dn4) + ((((assign17930_e32360 * var_deltemp_dn4) * assign17930_e32373) + (assign17930_e32365 * (assign17930_e32368 * var_deltemp_dn4))) / (2.0 * assign17930_e32381)))),)
            } else {
                let assign17930_e32386: f64 = (-var_ptwgt_i);
                let assign17930_e32388: f64 = (assign17930_e32386 * var_deltemp);
                let assign17930_e32389: f64 = (1.0 + assign17930_e32388);
                let assign17930_e32391: f64 = (assign17930_e32389 - 1e-6);
                let assign17930_e32393: f64 = (-10000.0);
                let assign17930_e32395: f64 = (assign17930_e32393 * 0.001);
                let (assign17930_e32411, assign17930_e32411_d_n4,) = {
                    if (assign17930_e32391 < assign17930_e32395) {
                        let assign17930_e32398: f64 = (-0.001);
                        let assign17930_e32400: f64 = (assign17930_e32398 * 0.001);
                        let assign17930_e32403: f64 = (-var_ptwgt_i);
                        let assign17930_e32405: f64 = (assign17930_e32403 * var_deltemp);
                        let assign17930_e32406: f64 = (1.0 + assign17930_e32405);
                        let assign17930_e32408: f64 = (assign17930_e32406 - 1e-6);
                        let assign17930_e32409: f64 = (assign17930_e32400 / assign17930_e32408);
                        (assign17930_e32409, (-((assign17930_e32400 * (assign17930_e32403 * var_deltemp_dn4)) / (assign17930_e32408 * assign17930_e32408))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17930_e32411, assign17930_e32411_d_n4,)
            }
        };
        let assign17930_e32413: f64 = (var_ptwg_i * assign17930_e32412);
        (assign17930_e32413, (var_ptwg_i_dn0 * assign17930_e32412), (var_ptwg_i_dn2 * assign17930_e32412), (var_ptwg_i_dn3 * assign17930_e32412), ((var_ptwg_i_dn4 * assign17930_e32412) + (var_ptwg_i * assign17930_e32412_d_n4)), (var_ptwg_i_dn5 * assign17930_e32412), (var_ptwg_i_dn6 * assign17930_e32412), (var_ptwg_i_dn7 * assign17930_e32412), (var_ptwg_i_dn8 * assign17930_e32412), (var_ptwg_i_dn9 * assign17930_e32412), (var_ptwg_i_dn10 * assign17930_e32412), (var_ptwg_i_dn11 * assign17930_e32412), (var_ptwg_i_dn13 * assign17930_e32412), (var_ptwg_i_dn14 * assign17930_e32412),)
    } else {
        (var_ptwg_t, var_ptwg_t_dn0, var_ptwg_t_dn2, var_ptwg_t_dn3, var_ptwg_t_dn4, var_ptwg_t_dn5, var_ptwg_t_dn6, var_ptwg_t_dn7, var_ptwg_t_dn8, var_ptwg_t_dn9, var_ptwg_t_dn10, var_ptwg_t_dn11, var_ptwg_t_dn13, var_ptwg_t_dn14,)
    }
};
        var_ptwg_t = assign17930_e32415;
        var_ptwg_t_dn0 = assign17930_e32415_d_n0;
        var_ptwg_t_dn2 = assign17930_e32415_d_n2;
        var_ptwg_t_dn3 = assign17930_e32415_d_n3;
        var_ptwg_t_dn4 = assign17930_e32415_d_n4;
        var_ptwg_t_dn5 = assign17930_e32415_d_n5;
        var_ptwg_t_dn6 = assign17930_e32415_d_n6;
        var_ptwg_t_dn7 = assign17930_e32415_d_n7;
        var_ptwg_t_dn8 = assign17930_e32415_d_n8;
        var_ptwg_t_dn9 = assign17930_e32415_d_n9;
        var_ptwg_t_dn10 = assign17930_e32415_d_n10;
        var_ptwg_t_dn11 = assign17930_e32415_d_n11;
        var_ptwg_t_dn13 = assign17930_e32415_d_n13;
        var_ptwg_t_dn14 = assign17930_e32415_d_n14;
        var_ptwg_t_rv = 0.0;

        let assign17940_e32418: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        var_guard334 = assign17940_e32418;
        var_guard334_rv = 0.0;

        let assign17950_e32421: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard335 = assign17950_e32421;
        var_guard335_rv = 0.0;

        *var_guard328_slot = var_guard328;
        *var_guard328_rv_slot = var_guard328_rv;
        *var_guard329_slot = var_guard329;
        *var_guard329_rv_slot = var_guard329_rv;
        *var_guard330_slot = var_guard330;
        *var_guard330_rv_slot = var_guard330_rv;
        *var_guard331_slot = var_guard331;
        *var_guard331_rv_slot = var_guard331_rv;
        *var_guard332_slot = var_guard332;
        *var_guard332_rv_slot = var_guard332_rv;
        *var_guard333_slot = var_guard333;
        *var_guard333_rv_slot = var_guard333_rv;
        *var_guard334_slot = var_guard334;
        *var_guard334_rv_slot = var_guard334_rv;
        *var_guard335_slot = var_guard335;
        *var_guard335_rv_slot = var_guard335_rv;
        *var_ptwg_t_slot = var_ptwg_t;
        *var_ptwg_t_dn0_slot = var_ptwg_t_dn0;
        *var_ptwg_t_dn10_slot = var_ptwg_t_dn10;
        *var_ptwg_t_dn11_slot = var_ptwg_t_dn11;
        *var_ptwg_t_dn13_slot = var_ptwg_t_dn13;
        *var_ptwg_t_dn14_slot = var_ptwg_t_dn14;
        *var_ptwg_t_dn2_slot = var_ptwg_t_dn2;
        *var_ptwg_t_dn3_slot = var_ptwg_t_dn3;
        *var_ptwg_t_dn4_slot = var_ptwg_t_dn4;
        *var_ptwg_t_dn5_slot = var_ptwg_t_dn5;
        *var_ptwg_t_dn6_slot = var_ptwg_t_dn6;
        *var_ptwg_t_dn7_slot = var_ptwg_t_dn7;
        *var_ptwg_t_dn8_slot = var_ptwg_t_dn8;
        *var_ptwg_t_dn9_slot = var_ptwg_t_dn9;
        *var_ptwg_t_rv_slot = var_ptwg_t_rv;
        *var_rddr_t_slot = var_rddr_t;
        *var_rddr_t_dn4_slot = var_rddr_t_dn4;
        *var_rddr_t_rv_slot = var_rddr_t_rv;
        *var_rddrr_t_slot = var_rddrr_t;
        *var_rddrr_t_dn4_slot = var_rddrr_t_dn4;
        *var_rddrr_t_rv_slot = var_rddrr_t_rv;
        *var_rsdr_t_slot = var_rsdr_t;
        *var_rsdr_t_dn4_slot = var_rsdr_t_dn4;
        *var_rsdr_t_rv_slot = var_rsdr_t_rv;
        *var_rsdrr_t_slot = var_rsdrr_t;
        *var_rsdrr_t_dn4_slot = var_rsdrr_t_dn4;
        *var_rsdrr_t_rv_slot = var_rsdrr_t_rv;
    }

    pub(super) fn stamp_reactive_block_62(
        var_a11_i: f64,
        var_a1_i: f64,
        var_a21_i: f64,
        var_a2_i: f64,
        var_aigbacc1_i: f64,
        var_aigbacc_i: f64,
        var_aigbinv1_i: f64,
        var_aigbinv_i: f64,
        var_aigc1_i: f64,
        var_aigc_i: f64,
        var_aigd1_i: f64,
        var_aigd_i: f64,
        var_aigs1_i: f64,
        var_aigs_i: f64,
        var_beta0_i: f64,
        var_deltemp: f64,
        var_deltemp_dn4: f64,
        var_guard334: f64,
        var_guard335: f64,
        var_iit_i: f64,
        var_k01_i: f64,
        var_k0_i: f64,
        var_k0si1_i: f64,
        var_k0si_i: f64,
        var_k0sisat1_i: f64,
        var_k0sisat_i: f64,
        var_k11_i: f64,
        var_k1_i: f64,
        var_k21_i: f64,
        var_k2_i: f64,
        var_k2sat1_i: f64,
        var_k2sat_i: f64,
        var_k2si1_i: f64,
        var_k2si_i: f64,
        var_k2sisat1_i: f64,
        var_k2sisat_i: f64,
        var_ptwgr_i: f64,
        var_ptwgr_i_dn0: f64,
        var_ptwgr_i_dn10: f64,
        var_ptwgr_i_dn11: f64,
        var_ptwgr_i_dn13: f64,
        var_ptwgr_i_dn14: f64,
        var_ptwgr_i_dn2: f64,
        var_ptwgr_i_dn3: f64,
        var_ptwgr_i_dn4: f64,
        var_ptwgr_i_dn5: f64,
        var_ptwgr_i_dn6: f64,
        var_ptwgr_i_dn7: f64,
        var_ptwgr_i_dn8: f64,
        var_ptwgr_i_dn9: f64,
        var_ptwgt_i: f64,
        var_sii0_i: f64,
        var_tii_i: f64,
        var_trat_ln: f64,
        var_trat_ln_dn4: f64,
        var_tratio_m1: f64,
        var_tratio_m1_dn4: f64,
        var_a1_t_slot: &mut f64,
        var_a1_t_dn4_slot: &mut f64,
        var_a1_t_rv_slot: &mut f64,
        var_a2_t_slot: &mut f64,
        var_a2_t_dn4_slot: &mut f64,
        var_a2_t_rv_slot: &mut f64,
        var_aigbacc_t_slot: &mut f64,
        var_aigbacc_t_dn4_slot: &mut f64,
        var_aigbacc_t_rv_slot: &mut f64,
        var_aigbinv_t_slot: &mut f64,
        var_aigbinv_t_dn4_slot: &mut f64,
        var_aigbinv_t_rv_slot: &mut f64,
        var_aigc_t_slot: &mut f64,
        var_aigc_t_dn4_slot: &mut f64,
        var_aigc_t_rv_slot: &mut f64,
        var_aigd_t_slot: &mut f64,
        var_aigd_t_dn4_slot: &mut f64,
        var_aigd_t_rv_slot: &mut f64,
        var_aigs_t_slot: &mut f64,
        var_aigs_t_dn4_slot: &mut f64,
        var_aigs_t_rv_slot: &mut f64,
        var_beta0_t_slot: &mut f64,
        var_beta0_t_dn4_slot: &mut f64,
        var_beta0_t_rv_slot: &mut f64,
        var_k0_t_slot: &mut f64,
        var_k0_t_dn4_slot: &mut f64,
        var_k0_t_rv_slot: &mut f64,
        var_k0si_t_slot: &mut f64,
        var_k0si_t_dn4_slot: &mut f64,
        var_k0si_t_rv_slot: &mut f64,
        var_k0sisat_t_slot: &mut f64,
        var_k0sisat_t_dn4_slot: &mut f64,
        var_k0sisat_t_rv_slot: &mut f64,
        var_k1_t_slot: &mut f64,
        var_k1_t_dn4_slot: &mut f64,
        var_k1_t_rv_slot: &mut f64,
        var_k2_t_slot: &mut f64,
        var_k2_t_dn4_slot: &mut f64,
        var_k2_t_rv_slot: &mut f64,
        var_k2sat_t_slot: &mut f64,
        var_k2sat_t_dn4_slot: &mut f64,
        var_k2sat_t_rv_slot: &mut f64,
        var_k2si_t_slot: &mut f64,
        var_k2si_t_dn4_slot: &mut f64,
        var_k2si_t_rv_slot: &mut f64,
        var_k2sisat_t_slot: &mut f64,
        var_k2sisat_t_dn4_slot: &mut f64,
        var_k2sisat_t_rv_slot: &mut f64,
        var_ptwgr_t_slot: &mut f64,
        var_ptwgr_t_dn0_slot: &mut f64,
        var_ptwgr_t_dn10_slot: &mut f64,
        var_ptwgr_t_dn11_slot: &mut f64,
        var_ptwgr_t_dn13_slot: &mut f64,
        var_ptwgr_t_dn14_slot: &mut f64,
        var_ptwgr_t_dn2_slot: &mut f64,
        var_ptwgr_t_dn3_slot: &mut f64,
        var_ptwgr_t_dn4_slot: &mut f64,
        var_ptwgr_t_dn5_slot: &mut f64,
        var_ptwgr_t_dn6_slot: &mut f64,
        var_ptwgr_t_dn7_slot: &mut f64,
        var_ptwgr_t_dn8_slot: &mut f64,
        var_ptwgr_t_dn9_slot: &mut f64,
        var_ptwgr_t_rv_slot: &mut f64,
        var_sii0_t_slot: &mut f64,
        var_sii0_t_dn4_slot: &mut f64,
        var_sii0_t_rv_slot: &mut f64,
    ) {
        let mut var_a1_t: f64 = *var_a1_t_slot;
        let mut var_a1_t_dn4: f64 = *var_a1_t_dn4_slot;
        let mut var_a1_t_rv: f64 = *var_a1_t_rv_slot;
        let mut var_a2_t: f64 = *var_a2_t_slot;
        let mut var_a2_t_dn4: f64 = *var_a2_t_dn4_slot;
        let mut var_a2_t_rv: f64 = *var_a2_t_rv_slot;
        let mut var_aigbacc_t: f64 = *var_aigbacc_t_slot;
        let mut var_aigbacc_t_dn4: f64 = *var_aigbacc_t_dn4_slot;
        let mut var_aigbacc_t_rv: f64 = *var_aigbacc_t_rv_slot;
        let mut var_aigbinv_t: f64 = *var_aigbinv_t_slot;
        let mut var_aigbinv_t_dn4: f64 = *var_aigbinv_t_dn4_slot;
        let mut var_aigbinv_t_rv: f64 = *var_aigbinv_t_rv_slot;
        let mut var_aigc_t: f64 = *var_aigc_t_slot;
        let mut var_aigc_t_dn4: f64 = *var_aigc_t_dn4_slot;
        let mut var_aigc_t_rv: f64 = *var_aigc_t_rv_slot;
        let mut var_aigd_t: f64 = *var_aigd_t_slot;
        let mut var_aigd_t_dn4: f64 = *var_aigd_t_dn4_slot;
        let mut var_aigd_t_rv: f64 = *var_aigd_t_rv_slot;
        let mut var_aigs_t: f64 = *var_aigs_t_slot;
        let mut var_aigs_t_dn4: f64 = *var_aigs_t_dn4_slot;
        let mut var_aigs_t_rv: f64 = *var_aigs_t_rv_slot;
        let mut var_beta0_t: f64 = *var_beta0_t_slot;
        let mut var_beta0_t_dn4: f64 = *var_beta0_t_dn4_slot;
        let mut var_beta0_t_rv: f64 = *var_beta0_t_rv_slot;
        let mut var_k0_t: f64 = *var_k0_t_slot;
        let mut var_k0_t_dn4: f64 = *var_k0_t_dn4_slot;
        let mut var_k0_t_rv: f64 = *var_k0_t_rv_slot;
        let mut var_k0si_t: f64 = *var_k0si_t_slot;
        let mut var_k0si_t_dn4: f64 = *var_k0si_t_dn4_slot;
        let mut var_k0si_t_rv: f64 = *var_k0si_t_rv_slot;
        let mut var_k0sisat_t: f64 = *var_k0sisat_t_slot;
        let mut var_k0sisat_t_dn4: f64 = *var_k0sisat_t_dn4_slot;
        let mut var_k0sisat_t_rv: f64 = *var_k0sisat_t_rv_slot;
        let mut var_k1_t: f64 = *var_k1_t_slot;
        let mut var_k1_t_dn4: f64 = *var_k1_t_dn4_slot;
        let mut var_k1_t_rv: f64 = *var_k1_t_rv_slot;
        let mut var_k2_t: f64 = *var_k2_t_slot;
        let mut var_k2_t_dn4: f64 = *var_k2_t_dn4_slot;
        let mut var_k2_t_rv: f64 = *var_k2_t_rv_slot;
        let mut var_k2sat_t: f64 = *var_k2sat_t_slot;
        let mut var_k2sat_t_dn4: f64 = *var_k2sat_t_dn4_slot;
        let mut var_k2sat_t_rv: f64 = *var_k2sat_t_rv_slot;
        let mut var_k2si_t: f64 = *var_k2si_t_slot;
        let mut var_k2si_t_dn4: f64 = *var_k2si_t_dn4_slot;
        let mut var_k2si_t_rv: f64 = *var_k2si_t_rv_slot;
        let mut var_k2sisat_t: f64 = *var_k2sisat_t_slot;
        let mut var_k2sisat_t_dn4: f64 = *var_k2sisat_t_dn4_slot;
        let mut var_k2sisat_t_rv: f64 = *var_k2sisat_t_rv_slot;
        let mut var_ptwgr_t: f64 = *var_ptwgr_t_slot;
        let mut var_ptwgr_t_dn0: f64 = *var_ptwgr_t_dn0_slot;
        let mut var_ptwgr_t_dn10: f64 = *var_ptwgr_t_dn10_slot;
        let mut var_ptwgr_t_dn11: f64 = *var_ptwgr_t_dn11_slot;
        let mut var_ptwgr_t_dn13: f64 = *var_ptwgr_t_dn13_slot;
        let mut var_ptwgr_t_dn14: f64 = *var_ptwgr_t_dn14_slot;
        let mut var_ptwgr_t_dn2: f64 = *var_ptwgr_t_dn2_slot;
        let mut var_ptwgr_t_dn3: f64 = *var_ptwgr_t_dn3_slot;
        let mut var_ptwgr_t_dn4: f64 = *var_ptwgr_t_dn4_slot;
        let mut var_ptwgr_t_dn5: f64 = *var_ptwgr_t_dn5_slot;
        let mut var_ptwgr_t_dn6: f64 = *var_ptwgr_t_dn6_slot;
        let mut var_ptwgr_t_dn7: f64 = *var_ptwgr_t_dn7_slot;
        let mut var_ptwgr_t_dn8: f64 = *var_ptwgr_t_dn8_slot;
        let mut var_ptwgr_t_dn9: f64 = *var_ptwgr_t_dn9_slot;
        let mut var_ptwgr_t_rv: f64 = *var_ptwgr_t_rv_slot;
        let mut var_sii0_t: f64 = *var_sii0_t_slot;
        let mut var_sii0_t_dn4: f64 = *var_sii0_t_dn4_slot;
        let mut var_sii0_t_rv: f64 = *var_sii0_t_rv_slot;

        let (assign17960_e32470, assign17960_e32470_d_n0, assign17960_e32470_d_n2, assign17960_e32470_d_n3, assign17960_e32470_d_n4, assign17960_e32470_d_n5, assign17960_e32470_d_n6, assign17960_e32470_d_n7, assign17960_e32470_d_n8, assign17960_e32470_d_n9, assign17960_e32470_d_n10, assign17960_e32470_d_n11, assign17960_e32470_d_n13, assign17960_e32470_d_n14,) = {
    if ((var_guard334 != 0.0) && (var_guard335 != 0.0)) {
        let assign17960_e32427: f64 = (-var_ptwgr_i);
        let assign17960_e32430: f64 = (-var_ptwgt_i);
        let assign17960_e32432: f64 = (assign17960_e32430 * var_deltemp);
        let assign17960_e32434: f64 = (-var_ptwgr_i);
        let assign17960_e32435: f64 = (assign17960_e32432 - assign17960_e32434);
        let assign17960_e32437: f64 = (assign17960_e32435 - 1e-6);
        let assign17960_e32439: f64 = (-var_ptwgt_i);
        let assign17960_e32441: f64 = (assign17960_e32439 * var_deltemp);
        let assign17960_e32443: f64 = (-var_ptwgr_i);
        let assign17960_e32444: f64 = (assign17960_e32441 - assign17960_e32443);
        let assign17960_e32446: f64 = (assign17960_e32444 - 1e-6);
        let assign17960_e32448: f64 = (-var_ptwgt_i);
        let assign17960_e32450: f64 = (assign17960_e32448 * var_deltemp);
        let assign17960_e32452: f64 = (-var_ptwgr_i);
        let assign17960_e32453: f64 = (assign17960_e32450 - assign17960_e32452);
        let assign17960_e32455: f64 = (assign17960_e32453 - 1e-6);
        let assign17960_e32456: f64 = (assign17960_e32446 * assign17960_e32455);
        let assign17960_e32459: f64 = (-var_ptwgr_i);
        let assign17960_e32460: f64 = (4.0 * assign17960_e32459);
        let assign17960_e32462: f64 = (assign17960_e32460 * 1e-6);
        let assign17960_e32463: f64 = (assign17960_e32456 - assign17960_e32462);
        let assign17960_e32464: f64 = (assign17960_e32463).sqrt();
        let assign17960_e32465: f64 = (assign17960_e32437 + assign17960_e32464);
        let assign17960_e32466: f64 = (0.5 * assign17960_e32465);
        let assign17960_e32467: f64 = (assign17960_e32427 + assign17960_e32466);
        let assign17960_e32468: f64 = (var_ptwgr_i + assign17960_e32467);
        (assign17960_e32468, (var_ptwgr_i_dn0 + ((-var_ptwgr_i_dn0) + (0.5 * ((-(-var_ptwgr_i_dn0)) + (((((-(-var_ptwgr_i_dn0)) * assign17960_e32455) + (assign17960_e32446 * (-(-var_ptwgr_i_dn0)))) - ((4.0 * (-var_ptwgr_i_dn0)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (var_ptwgr_i_dn2 + ((-var_ptwgr_i_dn2) + (0.5 * ((-(-var_ptwgr_i_dn2)) + (((((-(-var_ptwgr_i_dn2)) * assign17960_e32455) + (assign17960_e32446 * (-(-var_ptwgr_i_dn2)))) - ((4.0 * (-var_ptwgr_i_dn2)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (var_ptwgr_i_dn3 + ((-var_ptwgr_i_dn3) + (0.5 * ((-(-var_ptwgr_i_dn3)) + (((((-(-var_ptwgr_i_dn3)) * assign17960_e32455) + (assign17960_e32446 * (-(-var_ptwgr_i_dn3)))) - ((4.0 * (-var_ptwgr_i_dn3)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (var_ptwgr_i_dn4 + ((-var_ptwgr_i_dn4) + (0.5 * (((assign17960_e32430 * var_deltemp_dn4) - (-var_ptwgr_i_dn4)) + ((((((assign17960_e32439 * var_deltemp_dn4) - (-var_ptwgr_i_dn4)) * assign17960_e32455) + (assign17960_e32446 * ((assign17960_e32448 * var_deltemp_dn4) - (-var_ptwgr_i_dn4)))) - ((4.0 * (-var_ptwgr_i_dn4)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (var_ptwgr_i_dn5 + ((-var_ptwgr_i_dn5) + (0.5 * ((-(-var_ptwgr_i_dn5)) + (((((-(-var_ptwgr_i_dn5)) * assign17960_e32455) + (assign17960_e32446 * (-(-var_ptwgr_i_dn5)))) - ((4.0 * (-var_ptwgr_i_dn5)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (var_ptwgr_i_dn6 + ((-var_ptwgr_i_dn6) + (0.5 * ((-(-var_ptwgr_i_dn6)) + (((((-(-var_ptwgr_i_dn6)) * assign17960_e32455) + (assign17960_e32446 * (-(-var_ptwgr_i_dn6)))) - ((4.0 * (-var_ptwgr_i_dn6)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (var_ptwgr_i_dn7 + ((-var_ptwgr_i_dn7) + (0.5 * ((-(-var_ptwgr_i_dn7)) + (((((-(-var_ptwgr_i_dn7)) * assign17960_e32455) + (assign17960_e32446 * (-(-var_ptwgr_i_dn7)))) - ((4.0 * (-var_ptwgr_i_dn7)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (var_ptwgr_i_dn8 + ((-var_ptwgr_i_dn8) + (0.5 * ((-(-var_ptwgr_i_dn8)) + (((((-(-var_ptwgr_i_dn8)) * assign17960_e32455) + (assign17960_e32446 * (-(-var_ptwgr_i_dn8)))) - ((4.0 * (-var_ptwgr_i_dn8)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (var_ptwgr_i_dn9 + ((-var_ptwgr_i_dn9) + (0.5 * ((-(-var_ptwgr_i_dn9)) + (((((-(-var_ptwgr_i_dn9)) * assign17960_e32455) + (assign17960_e32446 * (-(-var_ptwgr_i_dn9)))) - ((4.0 * (-var_ptwgr_i_dn9)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (var_ptwgr_i_dn10 + ((-var_ptwgr_i_dn10) + (0.5 * ((-(-var_ptwgr_i_dn10)) + (((((-(-var_ptwgr_i_dn10)) * assign17960_e32455) + (assign17960_e32446 * (-(-var_ptwgr_i_dn10)))) - ((4.0 * (-var_ptwgr_i_dn10)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (var_ptwgr_i_dn11 + ((-var_ptwgr_i_dn11) + (0.5 * ((-(-var_ptwgr_i_dn11)) + (((((-(-var_ptwgr_i_dn11)) * assign17960_e32455) + (assign17960_e32446 * (-(-var_ptwgr_i_dn11)))) - ((4.0 * (-var_ptwgr_i_dn11)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (var_ptwgr_i_dn13 + ((-var_ptwgr_i_dn13) + (0.5 * ((-(-var_ptwgr_i_dn13)) + (((((-(-var_ptwgr_i_dn13)) * assign17960_e32455) + (assign17960_e32446 * (-(-var_ptwgr_i_dn13)))) - ((4.0 * (-var_ptwgr_i_dn13)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (var_ptwgr_i_dn14 + ((-var_ptwgr_i_dn14) + (0.5 * ((-(-var_ptwgr_i_dn14)) + (((((-(-var_ptwgr_i_dn14)) * assign17960_e32455) + (assign17960_e32446 * (-(-var_ptwgr_i_dn14)))) - ((4.0 * (-var_ptwgr_i_dn14)) * 1e-6)) / (2.0 * assign17960_e32464)))))),)
    } else {
        (var_ptwgr_t, var_ptwgr_t_dn0, var_ptwgr_t_dn2, var_ptwgr_t_dn3, var_ptwgr_t_dn4, var_ptwgr_t_dn5, var_ptwgr_t_dn6, var_ptwgr_t_dn7, var_ptwgr_t_dn8, var_ptwgr_t_dn9, var_ptwgr_t_dn10, var_ptwgr_t_dn11, var_ptwgr_t_dn13, var_ptwgr_t_dn14,)
    }
};
        var_ptwgr_t = assign17960_e32470;
        var_ptwgr_t_dn0 = assign17960_e32470_d_n0;
        var_ptwgr_t_dn2 = assign17960_e32470_d_n2;
        var_ptwgr_t_dn3 = assign17960_e32470_d_n3;
        var_ptwgr_t_dn4 = assign17960_e32470_d_n4;
        var_ptwgr_t_dn5 = assign17960_e32470_d_n5;
        var_ptwgr_t_dn6 = assign17960_e32470_d_n6;
        var_ptwgr_t_dn7 = assign17960_e32470_d_n7;
        var_ptwgr_t_dn8 = assign17960_e32470_d_n8;
        var_ptwgr_t_dn9 = assign17960_e32470_d_n9;
        var_ptwgr_t_dn10 = assign17960_e32470_d_n10;
        var_ptwgr_t_dn11 = assign17960_e32470_d_n11;
        var_ptwgr_t_dn13 = assign17960_e32470_d_n13;
        var_ptwgr_t_dn14 = assign17960_e32470_d_n14;
        var_ptwgr_t_rv = 0.0;

        let (assign17970_e32556, assign17970_e32556_d_n0, assign17970_e32556_d_n2, assign17970_e32556_d_n3, assign17970_e32556_d_n4, assign17970_e32556_d_n5, assign17970_e32556_d_n6, assign17970_e32556_d_n7, assign17970_e32556_d_n8, assign17970_e32556_d_n9, assign17970_e32556_d_n10, assign17970_e32556_d_n11, assign17970_e32556_d_n13, assign17970_e32556_d_n14,) = {
    if ((var_guard334 != 0.0) && (var_guard335 == 0.0)) {
        let assign17970_e32478: f64 = (-var_ptwgt_i);
        let assign17970_e32480: f64 = (assign17970_e32478 * var_deltemp);
        let assign17970_e32481: f64 = (1.0 + assign17970_e32480);
        let assign17970_e32483: f64 = (assign17970_e32481 - 1e-6);
        let assign17970_e32485: f64 = (-10000.0);
        let assign17970_e32487: f64 = (assign17970_e32485 * 0.001);
        let (assign17970_e32553, assign17970_e32553_d_n4,) = {
            if (!(assign17970_e32483 < assign17970_e32487)) {
                let assign17970_e32493: f64 = (-var_ptwgt_i);
                let assign17970_e32495: f64 = (assign17970_e32493 * var_deltemp);
                let assign17970_e32496: f64 = (1.0 + assign17970_e32495);
                let assign17970_e32498: f64 = (assign17970_e32496 - 1e-6);
                let assign17970_e32501: f64 = (-var_ptwgt_i);
                let assign17970_e32503: f64 = (assign17970_e32501 * var_deltemp);
                let assign17970_e32504: f64 = (1.0 + assign17970_e32503);
                let assign17970_e32506: f64 = (assign17970_e32504 - 1e-6);
                let assign17970_e32509: f64 = (-var_ptwgt_i);
                let assign17970_e32511: f64 = (assign17970_e32509 * var_deltemp);
                let assign17970_e32512: f64 = (1.0 + assign17970_e32511);
                let assign17970_e32514: f64 = (assign17970_e32512 - 1e-6);
                let assign17970_e32515: f64 = (assign17970_e32506 * assign17970_e32514);
                let assign17970_e32518: f64 = (4.0 * 0.001);
                let assign17970_e32520: f64 = (assign17970_e32518 * 0.001);
                let assign17970_e32521: f64 = (assign17970_e32515 + assign17970_e32520);
                let assign17970_e32522: f64 = (assign17970_e32521).sqrt();
                let assign17970_e32523: f64 = (assign17970_e32498 + assign17970_e32522);
                let assign17970_e32524: f64 = (0.5 * assign17970_e32523);
                (assign17970_e32524, (0.5 * ((assign17970_e32493 * var_deltemp_dn4) + ((((assign17970_e32501 * var_deltemp_dn4) * assign17970_e32514) + (assign17970_e32506 * (assign17970_e32509 * var_deltemp_dn4))) / (2.0 * assign17970_e32522)))),)
            } else {
                let assign17970_e32527: f64 = (-var_ptwgt_i);
                let assign17970_e32529: f64 = (assign17970_e32527 * var_deltemp);
                let assign17970_e32530: f64 = (1.0 + assign17970_e32529);
                let assign17970_e32532: f64 = (assign17970_e32530 - 1e-6);
                let assign17970_e32534: f64 = (-10000.0);
                let assign17970_e32536: f64 = (assign17970_e32534 * 0.001);
                let (assign17970_e32552, assign17970_e32552_d_n4,) = {
                    if (assign17970_e32532 < assign17970_e32536) {
                        let assign17970_e32539: f64 = (-0.001);
                        let assign17970_e32541: f64 = (assign17970_e32539 * 0.001);
                        let assign17970_e32544: f64 = (-var_ptwgt_i);
                        let assign17970_e32546: f64 = (assign17970_e32544 * var_deltemp);
                        let assign17970_e32547: f64 = (1.0 + assign17970_e32546);
                        let assign17970_e32549: f64 = (assign17970_e32547 - 1e-6);
                        let assign17970_e32550: f64 = (assign17970_e32541 / assign17970_e32549);
                        (assign17970_e32550, (-((assign17970_e32541 * (assign17970_e32544 * var_deltemp_dn4)) / (assign17970_e32549 * assign17970_e32549))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17970_e32552, assign17970_e32552_d_n4,)
            }
        };
        let assign17970_e32554: f64 = (var_ptwgr_i * assign17970_e32553);
        (assign17970_e32554, (var_ptwgr_i_dn0 * assign17970_e32553), (var_ptwgr_i_dn2 * assign17970_e32553), (var_ptwgr_i_dn3 * assign17970_e32553), ((var_ptwgr_i_dn4 * assign17970_e32553) + (var_ptwgr_i * assign17970_e32553_d_n4)), (var_ptwgr_i_dn5 * assign17970_e32553), (var_ptwgr_i_dn6 * assign17970_e32553), (var_ptwgr_i_dn7 * assign17970_e32553), (var_ptwgr_i_dn8 * assign17970_e32553), (var_ptwgr_i_dn9 * assign17970_e32553), (var_ptwgr_i_dn10 * assign17970_e32553), (var_ptwgr_i_dn11 * assign17970_e32553), (var_ptwgr_i_dn13 * assign17970_e32553), (var_ptwgr_i_dn14 * assign17970_e32553),)
    } else {
        (var_ptwgr_t, var_ptwgr_t_dn0, var_ptwgr_t_dn2, var_ptwgr_t_dn3, var_ptwgr_t_dn4, var_ptwgr_t_dn5, var_ptwgr_t_dn6, var_ptwgr_t_dn7, var_ptwgr_t_dn8, var_ptwgr_t_dn9, var_ptwgr_t_dn10, var_ptwgr_t_dn11, var_ptwgr_t_dn13, var_ptwgr_t_dn14,)
    }
};
        var_ptwgr_t = assign17970_e32556;
        var_ptwgr_t_dn0 = assign17970_e32556_d_n0;
        var_ptwgr_t_dn2 = assign17970_e32556_d_n2;
        var_ptwgr_t_dn3 = assign17970_e32556_d_n3;
        var_ptwgr_t_dn4 = assign17970_e32556_d_n4;
        var_ptwgr_t_dn5 = assign17970_e32556_d_n5;
        var_ptwgr_t_dn6 = assign17970_e32556_d_n6;
        var_ptwgr_t_dn7 = assign17970_e32556_d_n7;
        var_ptwgr_t_dn8 = assign17970_e32556_d_n8;
        var_ptwgr_t_dn9 = assign17970_e32556_d_n9;
        var_ptwgr_t_dn10 = assign17970_e32556_d_n10;
        var_ptwgr_t_dn11 = assign17970_e32556_d_n11;
        var_ptwgr_t_dn13 = assign17970_e32556_d_n13;
        var_ptwgr_t_dn14 = assign17970_e32556_d_n14;
        var_ptwgr_t_rv = 0.0;

        let assign17980_e32560: f64 = (var_iit_i * var_trat_ln);
        let assign17980_e32561: f64 = (assign17980_e32560).exp();
        let assign17980_e32562: f64 = (var_beta0_i * assign17980_e32561);
        var_beta0_t = assign17980_e32562;
        var_beta0_t_dn4 = (var_beta0_i * (assign17980_e32561 * (var_iit_i * var_trat_ln_dn4)));
        var_beta0_t_rv = 0.0;

        let assign17990_e32567: f64 = (var_tii_i * var_tratio_m1);
        let assign17990_e32568: f64 = (1.0 + assign17990_e32567);
        let assign17990_e32570: f64 = (assign17990_e32568 - 0.01);
        let assign17990_e32572: f64 = (-10000.0);
        let assign17990_e32574: f64 = (assign17990_e32572 * 0.001);
        let (assign17990_e32635, assign17990_e32635_d_n4,) = {
    if (!(assign17990_e32570 < assign17990_e32574)) {
        let assign17990_e32581: f64 = (var_tii_i * var_tratio_m1);
        let assign17990_e32582: f64 = (1.0 + assign17990_e32581);
        let assign17990_e32584: f64 = (assign17990_e32582 - 0.01);
        let assign17990_e32588: f64 = (var_tii_i * var_tratio_m1);
        let assign17990_e32589: f64 = (1.0 + assign17990_e32588);
        let assign17990_e32591: f64 = (assign17990_e32589 - 0.01);
        let assign17990_e32595: f64 = (var_tii_i * var_tratio_m1);
        let assign17990_e32596: f64 = (1.0 + assign17990_e32595);
        let assign17990_e32598: f64 = (assign17990_e32596 - 0.01);
        let assign17990_e32599: f64 = (assign17990_e32591 * assign17990_e32598);
        let assign17990_e32602: f64 = (4.0 * 0.001);
        let assign17990_e32604: f64 = (assign17990_e32602 * 0.001);
        let assign17990_e32605: f64 = (assign17990_e32599 + assign17990_e32604);
        let assign17990_e32606: f64 = (assign17990_e32605).sqrt();
        let assign17990_e32607: f64 = (assign17990_e32584 + assign17990_e32606);
        let assign17990_e32608: f64 = (0.5 * assign17990_e32607);
        (assign17990_e32608, (0.5 * ((var_tii_i * var_tratio_m1_dn4) + ((((var_tii_i * var_tratio_m1_dn4) * assign17990_e32598) + (assign17990_e32591 * (var_tii_i * var_tratio_m1_dn4))) / (2.0 * assign17990_e32606)))),)
    } else {
        let assign17990_e32612: f64 = (var_tii_i * var_tratio_m1);
        let assign17990_e32613: f64 = (1.0 + assign17990_e32612);
        let assign17990_e32615: f64 = (assign17990_e32613 - 0.01);
        let assign17990_e32617: f64 = (-10000.0);
        let assign17990_e32619: f64 = (assign17990_e32617 * 0.001);
        let (assign17990_e32634, assign17990_e32634_d_n4,) = {
            if (assign17990_e32615 < assign17990_e32619) {
                let assign17990_e32622: f64 = (-0.001);
                let assign17990_e32624: f64 = (assign17990_e32622 * 0.001);
                let assign17990_e32628: f64 = (var_tii_i * var_tratio_m1);
                let assign17990_e32629: f64 = (1.0 + assign17990_e32628);
                let assign17990_e32631: f64 = (assign17990_e32629 - 0.01);
                let assign17990_e32632: f64 = (assign17990_e32624 / assign17990_e32631);
                (assign17990_e32632, (-((assign17990_e32624 * (var_tii_i * var_tratio_m1_dn4)) / (assign17990_e32631 * assign17990_e32631))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign17990_e32634, assign17990_e32634_d_n4,)
    }
};
        let assign17990_e32637: f64 = (assign17990_e32635 + 0.01);
        let assign17990_e32638: f64 = (var_sii0_i * assign17990_e32637);
        var_sii0_t = assign17990_e32638;
        var_sii0_t_dn4 = (var_sii0_i * assign17990_e32635_d_n4);
        var_sii0_t_rv = 0.0;

        let assign18000_e32642: f64 = (var_k01_i * var_deltemp);
        let assign18000_e32643: f64 = (var_k0_i + assign18000_e32642);
        var_k0_t = assign18000_e32643;
        var_k0_t_dn4 = (var_k01_i * var_deltemp_dn4);
        var_k0_t_rv = 0.0;

        let assign18010_e32646: f64 = (-var_k0si_i);
        let assign18010_e32650: f64 = (var_k0si1_i * var_deltemp);
        let assign18010_e32652: f64 = (-var_k0si_i);
        let assign18010_e32653: f64 = (assign18010_e32650 - assign18010_e32652);
        let assign18010_e32655: f64 = (assign18010_e32653 - 1e-6);
        let assign18010_e32658: f64 = (var_k0si1_i * var_deltemp);
        let assign18010_e32660: f64 = (-var_k0si_i);
        let assign18010_e32661: f64 = (assign18010_e32658 - assign18010_e32660);
        let assign18010_e32663: f64 = (assign18010_e32661 - 1e-6);
        let assign18010_e32666: f64 = (var_k0si1_i * var_deltemp);
        let assign18010_e32668: f64 = (-var_k0si_i);
        let assign18010_e32669: f64 = (assign18010_e32666 - assign18010_e32668);
        let assign18010_e32671: f64 = (assign18010_e32669 - 1e-6);
        let assign18010_e32672: f64 = (assign18010_e32663 * assign18010_e32671);
        let assign18010_e32675: f64 = (-var_k0si_i);
        let assign18010_e32676: f64 = (4.0 * assign18010_e32675);
        let assign18010_e32678: f64 = (assign18010_e32676 * 1e-6);
        let assign18010_e32679: f64 = (assign18010_e32672 - assign18010_e32678);
        let assign18010_e32680: f64 = (assign18010_e32679).sqrt();
        let assign18010_e32681: f64 = (assign18010_e32655 + assign18010_e32680);
        let assign18010_e32682: f64 = (0.5 * assign18010_e32681);
        let assign18010_e32683: f64 = (assign18010_e32646 + assign18010_e32682);
        let assign18010_e32684: f64 = (var_k0si_i + assign18010_e32683);
        var_k0si_t = assign18010_e32684;
        var_k0si_t_dn4 = (0.5 * ((var_k0si1_i * var_deltemp_dn4) + ((((var_k0si1_i * var_deltemp_dn4) * assign18010_e32671) + (assign18010_e32663 * (var_k0si1_i * var_deltemp_dn4))) / (2.0 * assign18010_e32680))));
        var_k0si_t_rv = 0.0;

        let assign18020_e32687: f64 = (-var_k2si_i);
        let assign18020_e32691: f64 = (var_k2si1_i * var_deltemp);
        let assign18020_e32693: f64 = (-var_k2si_i);
        let assign18020_e32694: f64 = (assign18020_e32691 - assign18020_e32693);
        let assign18020_e32696: f64 = (assign18020_e32694 - 1e-6);
        let assign18020_e32699: f64 = (var_k2si1_i * var_deltemp);
        let assign18020_e32701: f64 = (-var_k2si_i);
        let assign18020_e32702: f64 = (assign18020_e32699 - assign18020_e32701);
        let assign18020_e32704: f64 = (assign18020_e32702 - 1e-6);
        let assign18020_e32707: f64 = (var_k2si1_i * var_deltemp);
        let assign18020_e32709: f64 = (-var_k2si_i);
        let assign18020_e32710: f64 = (assign18020_e32707 - assign18020_e32709);
        let assign18020_e32712: f64 = (assign18020_e32710 - 1e-6);
        let assign18020_e32713: f64 = (assign18020_e32704 * assign18020_e32712);
        let assign18020_e32716: f64 = (-var_k2si_i);
        let assign18020_e32717: f64 = (4.0 * assign18020_e32716);
        let assign18020_e32719: f64 = (assign18020_e32717 * 1e-6);
        let assign18020_e32720: f64 = (assign18020_e32713 - assign18020_e32719);
        let assign18020_e32721: f64 = (assign18020_e32720).sqrt();
        let assign18020_e32722: f64 = (assign18020_e32696 + assign18020_e32721);
        let assign18020_e32723: f64 = (0.5 * assign18020_e32722);
        let assign18020_e32724: f64 = (assign18020_e32687 + assign18020_e32723);
        let assign18020_e32725: f64 = (var_k2si_i + assign18020_e32724);
        var_k2si_t = assign18020_e32725;
        var_k2si_t_dn4 = (0.5 * ((var_k2si1_i * var_deltemp_dn4) + ((((var_k2si1_i * var_deltemp_dn4) * assign18020_e32712) + (assign18020_e32704 * (var_k2si1_i * var_deltemp_dn4))) / (2.0 * assign18020_e32721))));
        var_k2si_t_rv = 0.0;

        let assign18030_e32728: f64 = (-var_k1_i);
        let assign18030_e32732: f64 = (var_k11_i * var_deltemp);
        let assign18030_e32734: f64 = (-var_k1_i);
        let assign18030_e32735: f64 = (assign18030_e32732 - assign18030_e32734);
        let assign18030_e32737: f64 = (assign18030_e32735 - 1e-6);
        let assign18030_e32740: f64 = (var_k11_i * var_deltemp);
        let assign18030_e32742: f64 = (-var_k1_i);
        let assign18030_e32743: f64 = (assign18030_e32740 - assign18030_e32742);
        let assign18030_e32745: f64 = (assign18030_e32743 - 1e-6);
        let assign18030_e32748: f64 = (var_k11_i * var_deltemp);
        let assign18030_e32750: f64 = (-var_k1_i);
        let assign18030_e32751: f64 = (assign18030_e32748 - assign18030_e32750);
        let assign18030_e32753: f64 = (assign18030_e32751 - 1e-6);
        let assign18030_e32754: f64 = (assign18030_e32745 * assign18030_e32753);
        let assign18030_e32757: f64 = (-var_k1_i);
        let assign18030_e32758: f64 = (4.0 * assign18030_e32757);
        let assign18030_e32760: f64 = (assign18030_e32758 * 1e-6);
        let assign18030_e32761: f64 = (assign18030_e32754 - assign18030_e32760);
        let assign18030_e32762: f64 = (assign18030_e32761).sqrt();
        let assign18030_e32763: f64 = (assign18030_e32737 + assign18030_e32762);
        let assign18030_e32764: f64 = (0.5 * assign18030_e32763);
        let assign18030_e32765: f64 = (assign18030_e32728 + assign18030_e32764);
        let assign18030_e32766: f64 = (var_k1_i + assign18030_e32765);
        var_k1_t = assign18030_e32766;
        var_k1_t_dn4 = (0.5 * ((var_k11_i * var_deltemp_dn4) + ((((var_k11_i * var_deltemp_dn4) * assign18030_e32753) + (assign18030_e32745 * (var_k11_i * var_deltemp_dn4))) / (2.0 * assign18030_e32762))));
        var_k1_t_rv = 0.0;

        let assign18040_e32770: f64 = (var_k2sat1_i * var_deltemp);
        let assign18040_e32771: f64 = (var_k2sat_i + assign18040_e32770);
        var_k2sat_t = assign18040_e32771;
        var_k2sat_t_dn4 = (var_k2sat1_i * var_deltemp_dn4);
        var_k2sat_t_rv = 0.0;

        let assign18050_e32775: f64 = (var_a11_i * var_deltemp);
        let assign18050_e32776: f64 = (var_a1_i + assign18050_e32775);
        var_a1_t = assign18050_e32776;
        var_a1_t_dn4 = (var_a11_i * var_deltemp_dn4);
        var_a1_t_rv = 0.0;

        let assign18060_e32780: f64 = (var_a21_i * var_deltemp);
        let assign18060_e32781: f64 = (var_a2_i + assign18060_e32780);
        var_a2_t = assign18060_e32781;
        var_a2_t_dn4 = (var_a21_i * var_deltemp_dn4);
        var_a2_t_rv = 0.0;

        let assign18070_e32784: f64 = (-var_k2_i);
        let assign18070_e32788: f64 = (var_k21_i * var_deltemp);
        let assign18070_e32790: f64 = (-var_k2_i);
        let assign18070_e32791: f64 = (assign18070_e32788 - assign18070_e32790);
        let assign18070_e32793: f64 = (assign18070_e32791 - 1e-6);
        let assign18070_e32796: f64 = (var_k21_i * var_deltemp);
        let assign18070_e32798: f64 = (-var_k2_i);
        let assign18070_e32799: f64 = (assign18070_e32796 - assign18070_e32798);
        let assign18070_e32801: f64 = (assign18070_e32799 - 1e-6);
        let assign18070_e32804: f64 = (var_k21_i * var_deltemp);
        let assign18070_e32806: f64 = (-var_k2_i);
        let assign18070_e32807: f64 = (assign18070_e32804 - assign18070_e32806);
        let assign18070_e32809: f64 = (assign18070_e32807 - 1e-6);
        let assign18070_e32810: f64 = (assign18070_e32801 * assign18070_e32809);
        let assign18070_e32813: f64 = (-var_k2_i);
        let assign18070_e32814: f64 = (4.0 * assign18070_e32813);
        let assign18070_e32816: f64 = (assign18070_e32814 * 1e-6);
        let assign18070_e32817: f64 = (assign18070_e32810 - assign18070_e32816);
        let assign18070_e32818: f64 = (assign18070_e32817).sqrt();
        let assign18070_e32819: f64 = (assign18070_e32793 + assign18070_e32818);
        let assign18070_e32820: f64 = (0.5 * assign18070_e32819);
        let assign18070_e32821: f64 = (assign18070_e32784 + assign18070_e32820);
        let assign18070_e32822: f64 = (var_k2_i + assign18070_e32821);
        var_k2_t = assign18070_e32822;
        var_k2_t_dn4 = (0.5 * ((var_k21_i * var_deltemp_dn4) + ((((var_k21_i * var_deltemp_dn4) * assign18070_e32809) + (assign18070_e32801 * (var_k21_i * var_deltemp_dn4))) / (2.0 * assign18070_e32818))));
        var_k2_t_rv = 0.0;

        let assign18080_e32826: f64 = (var_k0sisat1_i * var_deltemp);
        let assign18080_e32827: f64 = (var_k0sisat_i + assign18080_e32826);
        var_k0sisat_t = assign18080_e32827;
        var_k0sisat_t_dn4 = (var_k0sisat1_i * var_deltemp_dn4);
        var_k0sisat_t_rv = 0.0;

        let assign18090_e32831: f64 = (var_k2sisat1_i * var_deltemp);
        let assign18090_e32832: f64 = (var_k2sisat_i + assign18090_e32831);
        var_k2sisat_t = assign18090_e32832;
        var_k2sisat_t_dn4 = (var_k2sisat1_i * var_deltemp_dn4);
        var_k2sisat_t_rv = 0.0;

        let assign18100_e32835: f64 = (-var_aigbinv_i);
        let assign18100_e32839: f64 = (var_aigbinv1_i * var_deltemp);
        let assign18100_e32841: f64 = (-var_aigbinv_i);
        let assign18100_e32842: f64 = (assign18100_e32839 - assign18100_e32841);
        let assign18100_e32844: f64 = (assign18100_e32842 - 1e-6);
        let assign18100_e32847: f64 = (var_aigbinv1_i * var_deltemp);
        let assign18100_e32849: f64 = (-var_aigbinv_i);
        let assign18100_e32850: f64 = (assign18100_e32847 - assign18100_e32849);
        let assign18100_e32852: f64 = (assign18100_e32850 - 1e-6);
        let assign18100_e32855: f64 = (var_aigbinv1_i * var_deltemp);
        let assign18100_e32857: f64 = (-var_aigbinv_i);
        let assign18100_e32858: f64 = (assign18100_e32855 - assign18100_e32857);
        let assign18100_e32860: f64 = (assign18100_e32858 - 1e-6);
        let assign18100_e32861: f64 = (assign18100_e32852 * assign18100_e32860);
        let assign18100_e32864: f64 = (-var_aigbinv_i);
        let assign18100_e32865: f64 = (4.0 * assign18100_e32864);
        let assign18100_e32867: f64 = (assign18100_e32865 * 1e-6);
        let assign18100_e32868: f64 = (assign18100_e32861 - assign18100_e32867);
        let assign18100_e32869: f64 = (assign18100_e32868).sqrt();
        let assign18100_e32870: f64 = (assign18100_e32844 + assign18100_e32869);
        let assign18100_e32871: f64 = (0.5 * assign18100_e32870);
        let assign18100_e32872: f64 = (assign18100_e32835 + assign18100_e32871);
        let assign18100_e32873: f64 = (var_aigbinv_i + assign18100_e32872);
        var_aigbinv_t = assign18100_e32873;
        var_aigbinv_t_dn4 = (0.5 * ((var_aigbinv1_i * var_deltemp_dn4) + ((((var_aigbinv1_i * var_deltemp_dn4) * assign18100_e32860) + (assign18100_e32852 * (var_aigbinv1_i * var_deltemp_dn4))) / (2.0 * assign18100_e32869))));
        var_aigbinv_t_rv = 0.0;

        let assign18110_e32876: f64 = (-var_aigbacc_i);
        let assign18110_e32880: f64 = (var_aigbacc1_i * var_deltemp);
        let assign18110_e32882: f64 = (-var_aigbacc_i);
        let assign18110_e32883: f64 = (assign18110_e32880 - assign18110_e32882);
        let assign18110_e32885: f64 = (assign18110_e32883 - 1e-6);
        let assign18110_e32888: f64 = (var_aigbacc1_i * var_deltemp);
        let assign18110_e32890: f64 = (-var_aigbacc_i);
        let assign18110_e32891: f64 = (assign18110_e32888 - assign18110_e32890);
        let assign18110_e32893: f64 = (assign18110_e32891 - 1e-6);
        let assign18110_e32896: f64 = (var_aigbacc1_i * var_deltemp);
        let assign18110_e32898: f64 = (-var_aigbacc_i);
        let assign18110_e32899: f64 = (assign18110_e32896 - assign18110_e32898);
        let assign18110_e32901: f64 = (assign18110_e32899 - 1e-6);
        let assign18110_e32902: f64 = (assign18110_e32893 * assign18110_e32901);
        let assign18110_e32905: f64 = (-var_aigbacc_i);
        let assign18110_e32906: f64 = (4.0 * assign18110_e32905);
        let assign18110_e32908: f64 = (assign18110_e32906 * 1e-6);
        let assign18110_e32909: f64 = (assign18110_e32902 - assign18110_e32908);
        let assign18110_e32910: f64 = (assign18110_e32909).sqrt();
        let assign18110_e32911: f64 = (assign18110_e32885 + assign18110_e32910);
        let assign18110_e32912: f64 = (0.5 * assign18110_e32911);
        let assign18110_e32913: f64 = (assign18110_e32876 + assign18110_e32912);
        let assign18110_e32914: f64 = (var_aigbacc_i + assign18110_e32913);
        var_aigbacc_t = assign18110_e32914;
        var_aigbacc_t_dn4 = (0.5 * ((var_aigbacc1_i * var_deltemp_dn4) + ((((var_aigbacc1_i * var_deltemp_dn4) * assign18110_e32901) + (assign18110_e32893 * (var_aigbacc1_i * var_deltemp_dn4))) / (2.0 * assign18110_e32910))));
        var_aigbacc_t_rv = 0.0;

        let assign18120_e32917: f64 = (-var_aigc_i);
        let assign18120_e32921: f64 = (var_aigc1_i * var_deltemp);
        let assign18120_e32923: f64 = (-var_aigc_i);
        let assign18120_e32924: f64 = (assign18120_e32921 - assign18120_e32923);
        let assign18120_e32926: f64 = (assign18120_e32924 - 1e-6);
        let assign18120_e32929: f64 = (var_aigc1_i * var_deltemp);
        let assign18120_e32931: f64 = (-var_aigc_i);
        let assign18120_e32932: f64 = (assign18120_e32929 - assign18120_e32931);
        let assign18120_e32934: f64 = (assign18120_e32932 - 1e-6);
        let assign18120_e32937: f64 = (var_aigc1_i * var_deltemp);
        let assign18120_e32939: f64 = (-var_aigc_i);
        let assign18120_e32940: f64 = (assign18120_e32937 - assign18120_e32939);
        let assign18120_e32942: f64 = (assign18120_e32940 - 1e-6);
        let assign18120_e32943: f64 = (assign18120_e32934 * assign18120_e32942);
        let assign18120_e32946: f64 = (-var_aigc_i);
        let assign18120_e32947: f64 = (4.0 * assign18120_e32946);
        let assign18120_e32949: f64 = (assign18120_e32947 * 1e-6);
        let assign18120_e32950: f64 = (assign18120_e32943 - assign18120_e32949);
        let assign18120_e32951: f64 = (assign18120_e32950).sqrt();
        let assign18120_e32952: f64 = (assign18120_e32926 + assign18120_e32951);
        let assign18120_e32953: f64 = (0.5 * assign18120_e32952);
        let assign18120_e32954: f64 = (assign18120_e32917 + assign18120_e32953);
        let assign18120_e32955: f64 = (var_aigc_i + assign18120_e32954);
        var_aigc_t = assign18120_e32955;
        var_aigc_t_dn4 = (0.5 * ((var_aigc1_i * var_deltemp_dn4) + ((((var_aigc1_i * var_deltemp_dn4) * assign18120_e32942) + (assign18120_e32934 * (var_aigc1_i * var_deltemp_dn4))) / (2.0 * assign18120_e32951))));
        var_aigc_t_rv = 0.0;

        let assign18130_e32958: f64 = (-var_aigs_i);
        let assign18130_e32962: f64 = (var_aigs1_i * var_deltemp);
        let assign18130_e32964: f64 = (-var_aigs_i);
        let assign18130_e32965: f64 = (assign18130_e32962 - assign18130_e32964);
        let assign18130_e32967: f64 = (assign18130_e32965 - 1e-6);
        let assign18130_e32970: f64 = (var_aigs1_i * var_deltemp);
        let assign18130_e32972: f64 = (-var_aigs_i);
        let assign18130_e32973: f64 = (assign18130_e32970 - assign18130_e32972);
        let assign18130_e32975: f64 = (assign18130_e32973 - 1e-6);
        let assign18130_e32978: f64 = (var_aigs1_i * var_deltemp);
        let assign18130_e32980: f64 = (-var_aigs_i);
        let assign18130_e32981: f64 = (assign18130_e32978 - assign18130_e32980);
        let assign18130_e32983: f64 = (assign18130_e32981 - 1e-6);
        let assign18130_e32984: f64 = (assign18130_e32975 * assign18130_e32983);
        let assign18130_e32987: f64 = (-var_aigs_i);
        let assign18130_e32988: f64 = (4.0 * assign18130_e32987);
        let assign18130_e32990: f64 = (assign18130_e32988 * 1e-6);
        let assign18130_e32991: f64 = (assign18130_e32984 - assign18130_e32990);
        let assign18130_e32992: f64 = (assign18130_e32991).sqrt();
        let assign18130_e32993: f64 = (assign18130_e32967 + assign18130_e32992);
        let assign18130_e32994: f64 = (0.5 * assign18130_e32993);
        let assign18130_e32995: f64 = (assign18130_e32958 + assign18130_e32994);
        let assign18130_e32996: f64 = (var_aigs_i + assign18130_e32995);
        var_aigs_t = assign18130_e32996;
        var_aigs_t_dn4 = (0.5 * ((var_aigs1_i * var_deltemp_dn4) + ((((var_aigs1_i * var_deltemp_dn4) * assign18130_e32983) + (assign18130_e32975 * (var_aigs1_i * var_deltemp_dn4))) / (2.0 * assign18130_e32992))));
        var_aigs_t_rv = 0.0;

        let assign18140_e32999: f64 = (-var_aigd_i);
        let assign18140_e33003: f64 = (var_aigd1_i * var_deltemp);
        let assign18140_e33005: f64 = (-var_aigd_i);
        let assign18140_e33006: f64 = (assign18140_e33003 - assign18140_e33005);
        let assign18140_e33008: f64 = (assign18140_e33006 - 1e-6);
        let assign18140_e33011: f64 = (var_aigd1_i * var_deltemp);
        let assign18140_e33013: f64 = (-var_aigd_i);
        let assign18140_e33014: f64 = (assign18140_e33011 - assign18140_e33013);
        let assign18140_e33016: f64 = (assign18140_e33014 - 1e-6);
        let assign18140_e33019: f64 = (var_aigd1_i * var_deltemp);
        let assign18140_e33021: f64 = (-var_aigd_i);
        let assign18140_e33022: f64 = (assign18140_e33019 - assign18140_e33021);
        let assign18140_e33024: f64 = (assign18140_e33022 - 1e-6);
        let assign18140_e33025: f64 = (assign18140_e33016 * assign18140_e33024);
        let assign18140_e33028: f64 = (-var_aigd_i);
        let assign18140_e33029: f64 = (4.0 * assign18140_e33028);
        let assign18140_e33031: f64 = (assign18140_e33029 * 1e-6);
        let assign18140_e33032: f64 = (assign18140_e33025 - assign18140_e33031);
        let assign18140_e33033: f64 = (assign18140_e33032).sqrt();
        let assign18140_e33034: f64 = (assign18140_e33008 + assign18140_e33033);
        let assign18140_e33035: f64 = (0.5 * assign18140_e33034);
        let assign18140_e33036: f64 = (assign18140_e32999 + assign18140_e33035);
        let assign18140_e33037: f64 = (var_aigd_i + assign18140_e33036);
        var_aigd_t = assign18140_e33037;
        var_aigd_t_dn4 = (0.5 * ((var_aigd1_i * var_deltemp_dn4) + ((((var_aigd1_i * var_deltemp_dn4) * assign18140_e33024) + (assign18140_e33016 * (var_aigd1_i * var_deltemp_dn4))) / (2.0 * assign18140_e33033))));
        var_aigd_t_rv = 0.0;

        *var_a1_t_slot = var_a1_t;
        *var_a1_t_dn4_slot = var_a1_t_dn4;
        *var_a1_t_rv_slot = var_a1_t_rv;
        *var_a2_t_slot = var_a2_t;
        *var_a2_t_dn4_slot = var_a2_t_dn4;
        *var_a2_t_rv_slot = var_a2_t_rv;
        *var_aigbacc_t_slot = var_aigbacc_t;
        *var_aigbacc_t_dn4_slot = var_aigbacc_t_dn4;
        *var_aigbacc_t_rv_slot = var_aigbacc_t_rv;
        *var_aigbinv_t_slot = var_aigbinv_t;
        *var_aigbinv_t_dn4_slot = var_aigbinv_t_dn4;
        *var_aigbinv_t_rv_slot = var_aigbinv_t_rv;
        *var_aigc_t_slot = var_aigc_t;
        *var_aigc_t_dn4_slot = var_aigc_t_dn4;
        *var_aigc_t_rv_slot = var_aigc_t_rv;
        *var_aigd_t_slot = var_aigd_t;
        *var_aigd_t_dn4_slot = var_aigd_t_dn4;
        *var_aigd_t_rv_slot = var_aigd_t_rv;
        *var_aigs_t_slot = var_aigs_t;
        *var_aigs_t_dn4_slot = var_aigs_t_dn4;
        *var_aigs_t_rv_slot = var_aigs_t_rv;
        *var_beta0_t_slot = var_beta0_t;
        *var_beta0_t_dn4_slot = var_beta0_t_dn4;
        *var_beta0_t_rv_slot = var_beta0_t_rv;
        *var_k0_t_slot = var_k0_t;
        *var_k0_t_dn4_slot = var_k0_t_dn4;
        *var_k0_t_rv_slot = var_k0_t_rv;
        *var_k0si_t_slot = var_k0si_t;
        *var_k0si_t_dn4_slot = var_k0si_t_dn4;
        *var_k0si_t_rv_slot = var_k0si_t_rv;
        *var_k0sisat_t_slot = var_k0sisat_t;
        *var_k0sisat_t_dn4_slot = var_k0sisat_t_dn4;
        *var_k0sisat_t_rv_slot = var_k0sisat_t_rv;
        *var_k1_t_slot = var_k1_t;
        *var_k1_t_dn4_slot = var_k1_t_dn4;
        *var_k1_t_rv_slot = var_k1_t_rv;
        *var_k2_t_slot = var_k2_t;
        *var_k2_t_dn4_slot = var_k2_t_dn4;
        *var_k2_t_rv_slot = var_k2_t_rv;
        *var_k2sat_t_slot = var_k2sat_t;
        *var_k2sat_t_dn4_slot = var_k2sat_t_dn4;
        *var_k2sat_t_rv_slot = var_k2sat_t_rv;
        *var_k2si_t_slot = var_k2si_t;
        *var_k2si_t_dn4_slot = var_k2si_t_dn4;
        *var_k2si_t_rv_slot = var_k2si_t_rv;
        *var_k2sisat_t_slot = var_k2sisat_t;
        *var_k2sisat_t_dn4_slot = var_k2sisat_t_dn4;
        *var_k2sisat_t_rv_slot = var_k2sisat_t_rv;
        *var_ptwgr_t_slot = var_ptwgr_t;
        *var_ptwgr_t_dn0_slot = var_ptwgr_t_dn0;
        *var_ptwgr_t_dn10_slot = var_ptwgr_t_dn10;
        *var_ptwgr_t_dn11_slot = var_ptwgr_t_dn11;
        *var_ptwgr_t_dn13_slot = var_ptwgr_t_dn13;
        *var_ptwgr_t_dn14_slot = var_ptwgr_t_dn14;
        *var_ptwgr_t_dn2_slot = var_ptwgr_t_dn2;
        *var_ptwgr_t_dn3_slot = var_ptwgr_t_dn3;
        *var_ptwgr_t_dn4_slot = var_ptwgr_t_dn4;
        *var_ptwgr_t_dn5_slot = var_ptwgr_t_dn5;
        *var_ptwgr_t_dn6_slot = var_ptwgr_t_dn6;
        *var_ptwgr_t_dn7_slot = var_ptwgr_t_dn7;
        *var_ptwgr_t_dn8_slot = var_ptwgr_t_dn8;
        *var_ptwgr_t_dn9_slot = var_ptwgr_t_dn9;
        *var_ptwgr_t_rv_slot = var_ptwgr_t_rv;
        *var_sii0_t_slot = var_sii0_t;
        *var_sii0_t_dn4_slot = var_sii0_t_dn4;
        *var_sii0_t_rv_slot = var_sii0_t_rv;
    }

    pub(super) fn stamp_reactive_block_63(
        p: &Parameters,
        var_alpha0_i: f64,
        var_alpha1_i: f64,
        var_alphaii0_i: f64,
        var_alphaii1_i: f64,
        var_bgidl_i: f64,
        var_bgisl_i: f64,
        var_deltemp: f64,
        var_deltemp_dn4: f64,
        var_tgidl_i: f64,
        var_alpha0_t_slot: &mut f64,
        var_alpha0_t_dn4_slot: &mut f64,
        var_alpha0_t_rv_slot: &mut f64,
        var_alpha1_t_slot: &mut f64,
        var_alpha1_t_dn4_slot: &mut f64,
        var_alpha1_t_rv_slot: &mut f64,
        var_alphaii0_t_slot: &mut f64,
        var_alphaii0_t_dn4_slot: &mut f64,
        var_alphaii0_t_rv_slot: &mut f64,
        var_alphaii1_t_slot: &mut f64,
        var_alphaii1_t_dn4_slot: &mut f64,
        var_alphaii1_t_rv_slot: &mut f64,
        var_bgidl_t_slot: &mut f64,
        var_bgidl_t_dn4_slot: &mut f64,
        var_bgidl_t_rv_slot: &mut f64,
        var_bgisl_t_slot: &mut f64,
        var_bgisl_t_dn4_slot: &mut f64,
        var_bgisl_t_rv_slot: &mut f64,
        var_cjd_t_slot: &mut f64,
        var_cjd_t_dn4_slot: &mut f64,
        var_cjd_t_rv_slot: &mut f64,
        var_cjs_t_slot: &mut f64,
        var_cjs_t_dn4_slot: &mut f64,
        var_cjs_t_rv_slot: &mut f64,
        var_cjsws_t_slot: &mut f64,
        var_cjsws_t_dn4_slot: &mut f64,
        var_cjsws_t_rv_slot: &mut f64,
        var_guard336_slot: &mut f64,
        var_guard336_rv_slot: &mut f64,
        var_guard337_slot: &mut f64,
        var_guard337_rv_slot: &mut f64,
        var_guard338_slot: &mut f64,
        var_guard338_rv_slot: &mut f64,
        var_guard339_slot: &mut f64,
        var_guard339_rv_slot: &mut f64,
        var_guard340_slot: &mut f64,
        var_guard340_rv_slot: &mut f64,
    ) {
        let mut var_alpha0_t: f64 = *var_alpha0_t_slot;
        let mut var_alpha0_t_dn4: f64 = *var_alpha0_t_dn4_slot;
        let mut var_alpha0_t_rv: f64 = *var_alpha0_t_rv_slot;
        let mut var_alpha1_t: f64 = *var_alpha1_t_slot;
        let mut var_alpha1_t_dn4: f64 = *var_alpha1_t_dn4_slot;
        let mut var_alpha1_t_rv: f64 = *var_alpha1_t_rv_slot;
        let mut var_alphaii0_t: f64 = *var_alphaii0_t_slot;
        let mut var_alphaii0_t_dn4: f64 = *var_alphaii0_t_dn4_slot;
        let mut var_alphaii0_t_rv: f64 = *var_alphaii0_t_rv_slot;
        let mut var_alphaii1_t: f64 = *var_alphaii1_t_slot;
        let mut var_alphaii1_t_dn4: f64 = *var_alphaii1_t_dn4_slot;
        let mut var_alphaii1_t_rv: f64 = *var_alphaii1_t_rv_slot;
        let mut var_bgidl_t: f64 = *var_bgidl_t_slot;
        let mut var_bgidl_t_dn4: f64 = *var_bgidl_t_dn4_slot;
        let mut var_bgidl_t_rv: f64 = *var_bgidl_t_rv_slot;
        let mut var_bgisl_t: f64 = *var_bgisl_t_slot;
        let mut var_bgisl_t_dn4: f64 = *var_bgisl_t_dn4_slot;
        let mut var_bgisl_t_rv: f64 = *var_bgisl_t_rv_slot;
        let mut var_cjd_t: f64 = *var_cjd_t_slot;
        let mut var_cjd_t_dn4: f64 = *var_cjd_t_dn4_slot;
        let mut var_cjd_t_rv: f64 = *var_cjd_t_rv_slot;
        let mut var_cjs_t: f64 = *var_cjs_t_slot;
        let mut var_cjs_t_dn4: f64 = *var_cjs_t_dn4_slot;
        let mut var_cjs_t_rv: f64 = *var_cjs_t_rv_slot;
        let mut var_cjsws_t: f64 = *var_cjsws_t_slot;
        let mut var_cjsws_t_dn4: f64 = *var_cjsws_t_dn4_slot;
        let mut var_cjsws_t_rv: f64 = *var_cjsws_t_rv_slot;
        let mut var_guard336: f64 = *var_guard336_slot;
        let mut var_guard336_rv: f64 = *var_guard336_rv_slot;
        let mut var_guard337: f64 = *var_guard337_slot;
        let mut var_guard337_rv: f64 = *var_guard337_rv_slot;
        let mut var_guard338: f64 = *var_guard338_slot;
        let mut var_guard338_rv: f64 = *var_guard338_rv_slot;
        let mut var_guard339: f64 = *var_guard339_slot;
        let mut var_guard339_rv: f64 = *var_guard339_rv_slot;
        let mut var_guard340: f64 = *var_guard340_slot;
        let mut var_guard340_rv: f64 = *var_guard340_rv_slot;

        let assign18150_e33042: f64 = (var_tgidl_i * var_deltemp);
        let assign18150_e33043: f64 = (1.0 + assign18150_e33042);
        let assign18150_e33045: f64 = (assign18150_e33043 - 1e-6);
        let assign18150_e33047: f64 = (-10000.0);
        let assign18150_e33049: f64 = (assign18150_e33047 * 0.001);
        let (assign18150_e33110, assign18150_e33110_d_n4,) = {
    if (!(assign18150_e33045 < assign18150_e33049)) {
        let assign18150_e33056: f64 = (var_tgidl_i * var_deltemp);
        let assign18150_e33057: f64 = (1.0 + assign18150_e33056);
        let assign18150_e33059: f64 = (assign18150_e33057 - 1e-6);
        let assign18150_e33063: f64 = (var_tgidl_i * var_deltemp);
        let assign18150_e33064: f64 = (1.0 + assign18150_e33063);
        let assign18150_e33066: f64 = (assign18150_e33064 - 1e-6);
        let assign18150_e33070: f64 = (var_tgidl_i * var_deltemp);
        let assign18150_e33071: f64 = (1.0 + assign18150_e33070);
        let assign18150_e33073: f64 = (assign18150_e33071 - 1e-6);
        let assign18150_e33074: f64 = (assign18150_e33066 * assign18150_e33073);
        let assign18150_e33077: f64 = (4.0 * 0.001);
        let assign18150_e33079: f64 = (assign18150_e33077 * 0.001);
        let assign18150_e33080: f64 = (assign18150_e33074 + assign18150_e33079);
        let assign18150_e33081: f64 = (assign18150_e33080).sqrt();
        let assign18150_e33082: f64 = (assign18150_e33059 + assign18150_e33081);
        let assign18150_e33083: f64 = (0.5 * assign18150_e33082);
        (assign18150_e33083, (0.5 * ((var_tgidl_i * var_deltemp_dn4) + ((((var_tgidl_i * var_deltemp_dn4) * assign18150_e33073) + (assign18150_e33066 * (var_tgidl_i * var_deltemp_dn4))) / (2.0 * assign18150_e33081)))),)
    } else {
        let assign18150_e33087: f64 = (var_tgidl_i * var_deltemp);
        let assign18150_e33088: f64 = (1.0 + assign18150_e33087);
        let assign18150_e33090: f64 = (assign18150_e33088 - 1e-6);
        let assign18150_e33092: f64 = (-10000.0);
        let assign18150_e33094: f64 = (assign18150_e33092 * 0.001);
        let (assign18150_e33109, assign18150_e33109_d_n4,) = {
            if (assign18150_e33090 < assign18150_e33094) {
                let assign18150_e33097: f64 = (-0.001);
                let assign18150_e33099: f64 = (assign18150_e33097 * 0.001);
                let assign18150_e33103: f64 = (var_tgidl_i * var_deltemp);
                let assign18150_e33104: f64 = (1.0 + assign18150_e33103);
                let assign18150_e33106: f64 = (assign18150_e33104 - 1e-6);
                let assign18150_e33107: f64 = (assign18150_e33099 / assign18150_e33106);
                (assign18150_e33107, (-((assign18150_e33099 * (var_tgidl_i * var_deltemp_dn4)) / (assign18150_e33106 * assign18150_e33106))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign18150_e33109, assign18150_e33109_d_n4,)
    }
};
        let assign18150_e33111: f64 = (var_bgidl_i * assign18150_e33110);
        var_bgidl_t = assign18150_e33111;
        var_bgidl_t_dn4 = (var_bgidl_i * assign18150_e33110_d_n4);
        var_bgidl_t_rv = 0.0;

        let assign18160_e33116: f64 = (var_tgidl_i * var_deltemp);
        let assign18160_e33117: f64 = (1.0 + assign18160_e33116);
        let assign18160_e33119: f64 = (assign18160_e33117 - 1e-6);
        let assign18160_e33121: f64 = (-10000.0);
        let assign18160_e33123: f64 = (assign18160_e33121 * 0.001);
        let (assign18160_e33184, assign18160_e33184_d_n4,) = {
    if (!(assign18160_e33119 < assign18160_e33123)) {
        let assign18160_e33130: f64 = (var_tgidl_i * var_deltemp);
        let assign18160_e33131: f64 = (1.0 + assign18160_e33130);
        let assign18160_e33133: f64 = (assign18160_e33131 - 1e-6);
        let assign18160_e33137: f64 = (var_tgidl_i * var_deltemp);
        let assign18160_e33138: f64 = (1.0 + assign18160_e33137);
        let assign18160_e33140: f64 = (assign18160_e33138 - 1e-6);
        let assign18160_e33144: f64 = (var_tgidl_i * var_deltemp);
        let assign18160_e33145: f64 = (1.0 + assign18160_e33144);
        let assign18160_e33147: f64 = (assign18160_e33145 - 1e-6);
        let assign18160_e33148: f64 = (assign18160_e33140 * assign18160_e33147);
        let assign18160_e33151: f64 = (4.0 * 0.001);
        let assign18160_e33153: f64 = (assign18160_e33151 * 0.001);
        let assign18160_e33154: f64 = (assign18160_e33148 + assign18160_e33153);
        let assign18160_e33155: f64 = (assign18160_e33154).sqrt();
        let assign18160_e33156: f64 = (assign18160_e33133 + assign18160_e33155);
        let assign18160_e33157: f64 = (0.5 * assign18160_e33156);
        (assign18160_e33157, (0.5 * ((var_tgidl_i * var_deltemp_dn4) + ((((var_tgidl_i * var_deltemp_dn4) * assign18160_e33147) + (assign18160_e33140 * (var_tgidl_i * var_deltemp_dn4))) / (2.0 * assign18160_e33155)))),)
    } else {
        let assign18160_e33161: f64 = (var_tgidl_i * var_deltemp);
        let assign18160_e33162: f64 = (1.0 + assign18160_e33161);
        let assign18160_e33164: f64 = (assign18160_e33162 - 1e-6);
        let assign18160_e33166: f64 = (-10000.0);
        let assign18160_e33168: f64 = (assign18160_e33166 * 0.001);
        let (assign18160_e33183, assign18160_e33183_d_n4,) = {
            if (assign18160_e33164 < assign18160_e33168) {
                let assign18160_e33171: f64 = (-0.001);
                let assign18160_e33173: f64 = (assign18160_e33171 * 0.001);
                let assign18160_e33177: f64 = (var_tgidl_i * var_deltemp);
                let assign18160_e33178: f64 = (1.0 + assign18160_e33177);
                let assign18160_e33180: f64 = (assign18160_e33178 - 1e-6);
                let assign18160_e33181: f64 = (assign18160_e33173 / assign18160_e33180);
                (assign18160_e33181, (-((assign18160_e33173 * (var_tgidl_i * var_deltemp_dn4)) / (assign18160_e33180 * assign18160_e33180))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign18160_e33183, assign18160_e33183_d_n4,)
    }
};
        let assign18160_e33185: f64 = (var_bgisl_i * assign18160_e33184);
        var_bgisl_t = assign18160_e33185;
        var_bgisl_t_dn4 = (var_bgisl_i * assign18160_e33184_d_n4);
        var_bgisl_t_rv = 0.0;

        let assign18170_e33188: f64 = (-var_alpha0_i);
        let assign18170_e33192: f64 = (p.p1437 * var_deltemp);
        let assign18170_e33194: f64 = (-var_alpha0_i);
        let assign18170_e33195: f64 = (assign18170_e33192 - assign18170_e33194);
        let assign18170_e33197: f64 = (assign18170_e33195 - 1e-6);
        let assign18170_e33200: f64 = (p.p1437 * var_deltemp);
        let assign18170_e33202: f64 = (-var_alpha0_i);
        let assign18170_e33203: f64 = (assign18170_e33200 - assign18170_e33202);
        let assign18170_e33205: f64 = (assign18170_e33203 - 1e-6);
        let assign18170_e33208: f64 = (p.p1437 * var_deltemp);
        let assign18170_e33210: f64 = (-var_alpha0_i);
        let assign18170_e33211: f64 = (assign18170_e33208 - assign18170_e33210);
        let assign18170_e33213: f64 = (assign18170_e33211 - 1e-6);
        let assign18170_e33214: f64 = (assign18170_e33205 * assign18170_e33213);
        let assign18170_e33217: f64 = (-var_alpha0_i);
        let assign18170_e33218: f64 = (4.0 * assign18170_e33217);
        let assign18170_e33220: f64 = (assign18170_e33218 * 1e-6);
        let assign18170_e33221: f64 = (assign18170_e33214 - assign18170_e33220);
        let assign18170_e33222: f64 = (assign18170_e33221).sqrt();
        let assign18170_e33223: f64 = (assign18170_e33197 + assign18170_e33222);
        let assign18170_e33224: f64 = (0.5 * assign18170_e33223);
        let assign18170_e33225: f64 = (assign18170_e33188 + assign18170_e33224);
        let assign18170_e33226: f64 = (var_alpha0_i + assign18170_e33225);
        var_alpha0_t = assign18170_e33226;
        var_alpha0_t_dn4 = (0.5 * ((p.p1437 * var_deltemp_dn4) + ((((p.p1437 * var_deltemp_dn4) * assign18170_e33213) + (assign18170_e33205 * (p.p1437 * var_deltemp_dn4))) / (2.0 * assign18170_e33222))));
        var_alpha0_t_rv = 0.0;

        let assign18180_e33229: f64 = (-var_alpha1_i);
        let assign18180_e33233: f64 = (p.p1438 * var_deltemp);
        let assign18180_e33235: f64 = (-var_alpha1_i);
        let assign18180_e33236: f64 = (assign18180_e33233 - assign18180_e33235);
        let assign18180_e33238: f64 = (assign18180_e33236 - 1e-6);
        let assign18180_e33241: f64 = (p.p1438 * var_deltemp);
        let assign18180_e33243: f64 = (-var_alpha1_i);
        let assign18180_e33244: f64 = (assign18180_e33241 - assign18180_e33243);
        let assign18180_e33246: f64 = (assign18180_e33244 - 1e-6);
        let assign18180_e33249: f64 = (p.p1438 * var_deltemp);
        let assign18180_e33251: f64 = (-var_alpha1_i);
        let assign18180_e33252: f64 = (assign18180_e33249 - assign18180_e33251);
        let assign18180_e33254: f64 = (assign18180_e33252 - 1e-6);
        let assign18180_e33255: f64 = (assign18180_e33246 * assign18180_e33254);
        let assign18180_e33258: f64 = (-var_alpha1_i);
        let assign18180_e33259: f64 = (4.0 * assign18180_e33258);
        let assign18180_e33261: f64 = (assign18180_e33259 * 1e-6);
        let assign18180_e33262: f64 = (assign18180_e33255 - assign18180_e33261);
        let assign18180_e33263: f64 = (assign18180_e33262).sqrt();
        let assign18180_e33264: f64 = (assign18180_e33238 + assign18180_e33263);
        let assign18180_e33265: f64 = (0.5 * assign18180_e33264);
        let assign18180_e33266: f64 = (assign18180_e33229 + assign18180_e33265);
        let assign18180_e33267: f64 = (var_alpha1_i + assign18180_e33266);
        var_alpha1_t = assign18180_e33267;
        var_alpha1_t_dn4 = (0.5 * ((p.p1438 * var_deltemp_dn4) + ((((p.p1438 * var_deltemp_dn4) * assign18180_e33254) + (assign18180_e33246 * (p.p1438 * var_deltemp_dn4))) / (2.0 * assign18180_e33263))));
        var_alpha1_t_rv = 0.0;

        let assign18190_e33270: f64 = (-var_alphaii0_i);
        let assign18190_e33274: f64 = (p.p1439 * var_deltemp);
        let assign18190_e33276: f64 = (-var_alphaii0_i);
        let assign18190_e33277: f64 = (assign18190_e33274 - assign18190_e33276);
        let assign18190_e33279: f64 = (assign18190_e33277 - 1e-25);
        let assign18190_e33282: f64 = (p.p1439 * var_deltemp);
        let assign18190_e33284: f64 = (-var_alphaii0_i);
        let assign18190_e33285: f64 = (assign18190_e33282 - assign18190_e33284);
        let assign18190_e33287: f64 = (assign18190_e33285 - 1e-25);
        let assign18190_e33290: f64 = (p.p1439 * var_deltemp);
        let assign18190_e33292: f64 = (-var_alphaii0_i);
        let assign18190_e33293: f64 = (assign18190_e33290 - assign18190_e33292);
        let assign18190_e33295: f64 = (assign18190_e33293 - 1e-25);
        let assign18190_e33296: f64 = (assign18190_e33287 * assign18190_e33295);
        let assign18190_e33299: f64 = (-var_alphaii0_i);
        let assign18190_e33300: f64 = (4.0 * assign18190_e33299);
        let assign18190_e33302: f64 = (assign18190_e33300 * 1e-25);
        let assign18190_e33303: f64 = (assign18190_e33296 - assign18190_e33302);
        let assign18190_e33304: f64 = (assign18190_e33303).sqrt();
        let assign18190_e33305: f64 = (assign18190_e33279 + assign18190_e33304);
        let assign18190_e33306: f64 = (0.5 * assign18190_e33305);
        let assign18190_e33307: f64 = (assign18190_e33270 + assign18190_e33306);
        let assign18190_e33308: f64 = (var_alphaii0_i + assign18190_e33307);
        var_alphaii0_t = assign18190_e33308;
        var_alphaii0_t_dn4 = (0.5 * ((p.p1439 * var_deltemp_dn4) + ((((p.p1439 * var_deltemp_dn4) * assign18190_e33295) + (assign18190_e33287 * (p.p1439 * var_deltemp_dn4))) / (2.0 * assign18190_e33304))));
        var_alphaii0_t_rv = 0.0;

        let assign18200_e33311: f64 = (-var_alphaii1_i);
        let assign18200_e33315: f64 = (p.p1440 * var_deltemp);
        let assign18200_e33317: f64 = (-var_alphaii1_i);
        let assign18200_e33318: f64 = (assign18200_e33315 - assign18200_e33317);
        let assign18200_e33320: f64 = (assign18200_e33318 - 1e-20);
        let assign18200_e33323: f64 = (p.p1440 * var_deltemp);
        let assign18200_e33325: f64 = (-var_alphaii1_i);
        let assign18200_e33326: f64 = (assign18200_e33323 - assign18200_e33325);
        let assign18200_e33328: f64 = (assign18200_e33326 - 1e-20);
        let assign18200_e33331: f64 = (p.p1440 * var_deltemp);
        let assign18200_e33333: f64 = (-var_alphaii1_i);
        let assign18200_e33334: f64 = (assign18200_e33331 - assign18200_e33333);
        let assign18200_e33336: f64 = (assign18200_e33334 - 1e-20);
        let assign18200_e33337: f64 = (assign18200_e33328 * assign18200_e33336);
        let assign18200_e33340: f64 = (-var_alphaii1_i);
        let assign18200_e33341: f64 = (4.0 * assign18200_e33340);
        let assign18200_e33343: f64 = (assign18200_e33341 * 1e-20);
        let assign18200_e33344: f64 = (assign18200_e33337 - assign18200_e33343);
        let assign18200_e33345: f64 = (assign18200_e33344).sqrt();
        let assign18200_e33346: f64 = (assign18200_e33320 + assign18200_e33345);
        let assign18200_e33347: f64 = (0.5 * assign18200_e33346);
        let assign18200_e33348: f64 = (assign18200_e33311 + assign18200_e33347);
        let assign18200_e33349: f64 = (var_alphaii1_i + assign18200_e33348);
        var_alphaii1_t = assign18200_e33349;
        var_alphaii1_t_dn4 = (0.5 * ((p.p1440 * var_deltemp_dn4) + ((((p.p1440 * var_deltemp_dn4) * assign18200_e33336) + (assign18200_e33328 * (p.p1440 * var_deltemp_dn4))) / (2.0 * assign18200_e33345))));
        var_alphaii1_t_rv = 0.0;

        let assign18230_e33359: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        var_guard336 = assign18230_e33359;
        var_guard336_rv = 0.0;

        let assign18240_e33362: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard337 = assign18240_e33362;
        var_guard337_rv = 0.0;

        let (assign18250_e33408, assign18250_e33408_d_n4,) = {
    if ((var_guard336 != 0.0) && (var_guard337 != 0.0)) {
        let assign18250_e33368: f64 = (-p.p1584);
        let assign18250_e33372: f64 = (p.p1721 * var_deltemp);
        let assign18250_e33374: f64 = (-p.p1584);
        let assign18250_e33375: f64 = (assign18250_e33372 - assign18250_e33374);
        let assign18250_e33377: f64 = (assign18250_e33375 - 1e-6);
        let assign18250_e33380: f64 = (p.p1721 * var_deltemp);
        let assign18250_e33382: f64 = (-p.p1584);
        let assign18250_e33383: f64 = (assign18250_e33380 - assign18250_e33382);
        let assign18250_e33385: f64 = (assign18250_e33383 - 1e-6);
        let assign18250_e33388: f64 = (p.p1721 * var_deltemp);
        let assign18250_e33390: f64 = (-p.p1584);
        let assign18250_e33391: f64 = (assign18250_e33388 - assign18250_e33390);
        let assign18250_e33393: f64 = (assign18250_e33391 - 1e-6);
        let assign18250_e33394: f64 = (assign18250_e33385 * assign18250_e33393);
        let assign18250_e33397: f64 = (-p.p1584);
        let assign18250_e33398: f64 = (4.0 * assign18250_e33397);
        let assign18250_e33400: f64 = (assign18250_e33398 * 1e-6);
        let assign18250_e33401: f64 = (assign18250_e33394 - assign18250_e33400);
        let assign18250_e33402: f64 = (assign18250_e33401).sqrt();
        let assign18250_e33403: f64 = (assign18250_e33377 + assign18250_e33402);
        let assign18250_e33404: f64 = (0.5 * assign18250_e33403);
        let assign18250_e33405: f64 = (assign18250_e33368 + assign18250_e33404);
        let assign18250_e33406: f64 = (p.p1584 + assign18250_e33405);
        (assign18250_e33406, (0.5 * ((p.p1721 * var_deltemp_dn4) + ((((p.p1721 * var_deltemp_dn4) * assign18250_e33393) + (assign18250_e33385 * (p.p1721 * var_deltemp_dn4))) / (2.0 * assign18250_e33402)))),)
    } else {
        (var_cjs_t, var_cjs_t_dn4,)
    }
};
        var_cjs_t = assign18250_e33408;
        var_cjs_t_dn4 = assign18250_e33408_d_n4;
        var_cjs_t_rv = 0.0;

        let (assign18260_e33488, assign18260_e33488_d_n4,) = {
    if ((var_guard336 != 0.0) && (var_guard337 == 0.0)) {
        let assign18260_e33417: f64 = (p.p1721 * var_deltemp);
        let assign18260_e33418: f64 = (1.0 + assign18260_e33417);
        let assign18260_e33420: f64 = (assign18260_e33418 - 1e-6);
        let assign18260_e33422: f64 = (-10000.0);
        let assign18260_e33424: f64 = (assign18260_e33422 * 0.001);
        let (assign18260_e33485, assign18260_e33485_d_n4,) = {
            if (!(assign18260_e33420 < assign18260_e33424)) {
                let assign18260_e33431: f64 = (p.p1721 * var_deltemp);
                let assign18260_e33432: f64 = (1.0 + assign18260_e33431);
                let assign18260_e33434: f64 = (assign18260_e33432 - 1e-6);
                let assign18260_e33438: f64 = (p.p1721 * var_deltemp);
                let assign18260_e33439: f64 = (1.0 + assign18260_e33438);
                let assign18260_e33441: f64 = (assign18260_e33439 - 1e-6);
                let assign18260_e33445: f64 = (p.p1721 * var_deltemp);
                let assign18260_e33446: f64 = (1.0 + assign18260_e33445);
                let assign18260_e33448: f64 = (assign18260_e33446 - 1e-6);
                let assign18260_e33449: f64 = (assign18260_e33441 * assign18260_e33448);
                let assign18260_e33452: f64 = (4.0 * 0.001);
                let assign18260_e33454: f64 = (assign18260_e33452 * 0.001);
                let assign18260_e33455: f64 = (assign18260_e33449 + assign18260_e33454);
                let assign18260_e33456: f64 = (assign18260_e33455).sqrt();
                let assign18260_e33457: f64 = (assign18260_e33434 + assign18260_e33456);
                let assign18260_e33458: f64 = (0.5 * assign18260_e33457);
                (assign18260_e33458, (0.5 * ((p.p1721 * var_deltemp_dn4) + ((((p.p1721 * var_deltemp_dn4) * assign18260_e33448) + (assign18260_e33441 * (p.p1721 * var_deltemp_dn4))) / (2.0 * assign18260_e33456)))),)
            } else {
                let assign18260_e33462: f64 = (p.p1721 * var_deltemp);
                let assign18260_e33463: f64 = (1.0 + assign18260_e33462);
                let assign18260_e33465: f64 = (assign18260_e33463 - 1e-6);
                let assign18260_e33467: f64 = (-10000.0);
                let assign18260_e33469: f64 = (assign18260_e33467 * 0.001);
                let (assign18260_e33484, assign18260_e33484_d_n4,) = {
                    if (assign18260_e33465 < assign18260_e33469) {
                        let assign18260_e33472: f64 = (-0.001);
                        let assign18260_e33474: f64 = (assign18260_e33472 * 0.001);
                        let assign18260_e33478: f64 = (p.p1721 * var_deltemp);
                        let assign18260_e33479: f64 = (1.0 + assign18260_e33478);
                        let assign18260_e33481: f64 = (assign18260_e33479 - 1e-6);
                        let assign18260_e33482: f64 = (assign18260_e33474 / assign18260_e33481);
                        (assign18260_e33482, (-((assign18260_e33474 * (p.p1721 * var_deltemp_dn4)) / (assign18260_e33481 * assign18260_e33481))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18260_e33484, assign18260_e33484_d_n4,)
            }
        };
        let assign18260_e33486: f64 = (p.p1584 * assign18260_e33485);
        (assign18260_e33486, (p.p1584 * assign18260_e33485_d_n4),)
    } else {
        (var_cjs_t, var_cjs_t_dn4,)
    }
};
        var_cjs_t = assign18260_e33488;
        var_cjs_t_dn4 = assign18260_e33488_d_n4;
        var_cjs_t_rv = 0.0;

        let assign18270_e33491: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard338 = assign18270_e33491;
        var_guard338_rv = 0.0;

        let (assign18280_e33537, assign18280_e33537_d_n4,) = {
    if ((var_guard336 != 0.0) && (var_guard338 != 0.0)) {
        let assign18280_e33497: f64 = (-p.p1585);
        let assign18280_e33501: f64 = (p.p1721 * var_deltemp);
        let assign18280_e33503: f64 = (-p.p1585);
        let assign18280_e33504: f64 = (assign18280_e33501 - assign18280_e33503);
        let assign18280_e33506: f64 = (assign18280_e33504 - 1e-6);
        let assign18280_e33509: f64 = (p.p1721 * var_deltemp);
        let assign18280_e33511: f64 = (-p.p1585);
        let assign18280_e33512: f64 = (assign18280_e33509 - assign18280_e33511);
        let assign18280_e33514: f64 = (assign18280_e33512 - 1e-6);
        let assign18280_e33517: f64 = (p.p1721 * var_deltemp);
        let assign18280_e33519: f64 = (-p.p1585);
        let assign18280_e33520: f64 = (assign18280_e33517 - assign18280_e33519);
        let assign18280_e33522: f64 = (assign18280_e33520 - 1e-6);
        let assign18280_e33523: f64 = (assign18280_e33514 * assign18280_e33522);
        let assign18280_e33526: f64 = (-p.p1585);
        let assign18280_e33527: f64 = (4.0 * assign18280_e33526);
        let assign18280_e33529: f64 = (assign18280_e33527 * 1e-6);
        let assign18280_e33530: f64 = (assign18280_e33523 - assign18280_e33529);
        let assign18280_e33531: f64 = (assign18280_e33530).sqrt();
        let assign18280_e33532: f64 = (assign18280_e33506 + assign18280_e33531);
        let assign18280_e33533: f64 = (0.5 * assign18280_e33532);
        let assign18280_e33534: f64 = (assign18280_e33497 + assign18280_e33533);
        let assign18280_e33535: f64 = (p.p1585 + assign18280_e33534);
        (assign18280_e33535, (0.5 * ((p.p1721 * var_deltemp_dn4) + ((((p.p1721 * var_deltemp_dn4) * assign18280_e33522) + (assign18280_e33514 * (p.p1721 * var_deltemp_dn4))) / (2.0 * assign18280_e33531)))),)
    } else {
        (var_cjd_t, var_cjd_t_dn4,)
    }
};
        var_cjd_t = assign18280_e33537;
        var_cjd_t_dn4 = assign18280_e33537_d_n4;
        var_cjd_t_rv = 0.0;

        let (assign18290_e33617, assign18290_e33617_d_n4,) = {
    if ((var_guard336 != 0.0) && (var_guard338 == 0.0)) {
        let assign18290_e33546: f64 = (p.p1721 * var_deltemp);
        let assign18290_e33547: f64 = (1.0 + assign18290_e33546);
        let assign18290_e33549: f64 = (assign18290_e33547 - 1e-6);
        let assign18290_e33551: f64 = (-10000.0);
        let assign18290_e33553: f64 = (assign18290_e33551 * 0.001);
        let (assign18290_e33614, assign18290_e33614_d_n4,) = {
            if (!(assign18290_e33549 < assign18290_e33553)) {
                let assign18290_e33560: f64 = (p.p1721 * var_deltemp);
                let assign18290_e33561: f64 = (1.0 + assign18290_e33560);
                let assign18290_e33563: f64 = (assign18290_e33561 - 1e-6);
                let assign18290_e33567: f64 = (p.p1721 * var_deltemp);
                let assign18290_e33568: f64 = (1.0 + assign18290_e33567);
                let assign18290_e33570: f64 = (assign18290_e33568 - 1e-6);
                let assign18290_e33574: f64 = (p.p1721 * var_deltemp);
                let assign18290_e33575: f64 = (1.0 + assign18290_e33574);
                let assign18290_e33577: f64 = (assign18290_e33575 - 1e-6);
                let assign18290_e33578: f64 = (assign18290_e33570 * assign18290_e33577);
                let assign18290_e33581: f64 = (4.0 * 0.001);
                let assign18290_e33583: f64 = (assign18290_e33581 * 0.001);
                let assign18290_e33584: f64 = (assign18290_e33578 + assign18290_e33583);
                let assign18290_e33585: f64 = (assign18290_e33584).sqrt();
                let assign18290_e33586: f64 = (assign18290_e33563 + assign18290_e33585);
                let assign18290_e33587: f64 = (0.5 * assign18290_e33586);
                (assign18290_e33587, (0.5 * ((p.p1721 * var_deltemp_dn4) + ((((p.p1721 * var_deltemp_dn4) * assign18290_e33577) + (assign18290_e33570 * (p.p1721 * var_deltemp_dn4))) / (2.0 * assign18290_e33585)))),)
            } else {
                let assign18290_e33591: f64 = (p.p1721 * var_deltemp);
                let assign18290_e33592: f64 = (1.0 + assign18290_e33591);
                let assign18290_e33594: f64 = (assign18290_e33592 - 1e-6);
                let assign18290_e33596: f64 = (-10000.0);
                let assign18290_e33598: f64 = (assign18290_e33596 * 0.001);
                let (assign18290_e33613, assign18290_e33613_d_n4,) = {
                    if (assign18290_e33594 < assign18290_e33598) {
                        let assign18290_e33601: f64 = (-0.001);
                        let assign18290_e33603: f64 = (assign18290_e33601 * 0.001);
                        let assign18290_e33607: f64 = (p.p1721 * var_deltemp);
                        let assign18290_e33608: f64 = (1.0 + assign18290_e33607);
                        let assign18290_e33610: f64 = (assign18290_e33608 - 1e-6);
                        let assign18290_e33611: f64 = (assign18290_e33603 / assign18290_e33610);
                        (assign18290_e33611, (-((assign18290_e33603 * (p.p1721 * var_deltemp_dn4)) / (assign18290_e33610 * assign18290_e33610))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18290_e33613, assign18290_e33613_d_n4,)
            }
        };
        let assign18290_e33615: f64 = (p.p1585 * assign18290_e33614);
        (assign18290_e33615, (p.p1585 * assign18290_e33614_d_n4),)
    } else {
        (var_cjd_t, var_cjd_t_dn4,)
    }
};
        var_cjd_t = assign18290_e33617;
        var_cjd_t_dn4 = assign18290_e33617_d_n4;
        var_cjd_t_rv = 0.0;

        let assign18300_e33620: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard339 = assign18300_e33620;
        var_guard339_rv = 0.0;

        let (assign18310_e33666, assign18310_e33666_d_n4,) = {
    if ((var_guard336 != 0.0) && (var_guard339 != 0.0)) {
        let assign18310_e33626: f64 = (-p.p1586);
        let assign18310_e33630: f64 = (p.p1722 * var_deltemp);
        let assign18310_e33632: f64 = (-p.p1586);
        let assign18310_e33633: f64 = (assign18310_e33630 - assign18310_e33632);
        let assign18310_e33635: f64 = (assign18310_e33633 - 1e-6);
        let assign18310_e33638: f64 = (p.p1722 * var_deltemp);
        let assign18310_e33640: f64 = (-p.p1586);
        let assign18310_e33641: f64 = (assign18310_e33638 - assign18310_e33640);
        let assign18310_e33643: f64 = (assign18310_e33641 - 1e-6);
        let assign18310_e33646: f64 = (p.p1722 * var_deltemp);
        let assign18310_e33648: f64 = (-p.p1586);
        let assign18310_e33649: f64 = (assign18310_e33646 - assign18310_e33648);
        let assign18310_e33651: f64 = (assign18310_e33649 - 1e-6);
        let assign18310_e33652: f64 = (assign18310_e33643 * assign18310_e33651);
        let assign18310_e33655: f64 = (-p.p1586);
        let assign18310_e33656: f64 = (4.0 * assign18310_e33655);
        let assign18310_e33658: f64 = (assign18310_e33656 * 1e-6);
        let assign18310_e33659: f64 = (assign18310_e33652 - assign18310_e33658);
        let assign18310_e33660: f64 = (assign18310_e33659).sqrt();
        let assign18310_e33661: f64 = (assign18310_e33635 + assign18310_e33660);
        let assign18310_e33662: f64 = (0.5 * assign18310_e33661);
        let assign18310_e33663: f64 = (assign18310_e33626 + assign18310_e33662);
        let assign18310_e33664: f64 = (p.p1586 + assign18310_e33663);
        (assign18310_e33664, (0.5 * ((p.p1722 * var_deltemp_dn4) + ((((p.p1722 * var_deltemp_dn4) * assign18310_e33651) + (assign18310_e33643 * (p.p1722 * var_deltemp_dn4))) / (2.0 * assign18310_e33660)))),)
    } else {
        (var_cjsws_t, var_cjsws_t_dn4,)
    }
};
        var_cjsws_t = assign18310_e33666;
        var_cjsws_t_dn4 = assign18310_e33666_d_n4;
        var_cjsws_t_rv = 0.0;

        let (assign18320_e33746, assign18320_e33746_d_n4,) = {
    if ((var_guard336 != 0.0) && (var_guard339 == 0.0)) {
        let assign18320_e33675: f64 = (p.p1722 * var_deltemp);
        let assign18320_e33676: f64 = (1.0 + assign18320_e33675);
        let assign18320_e33678: f64 = (assign18320_e33676 - 1e-6);
        let assign18320_e33680: f64 = (-10000.0);
        let assign18320_e33682: f64 = (assign18320_e33680 * 0.001);
        let (assign18320_e33743, assign18320_e33743_d_n4,) = {
            if (!(assign18320_e33678 < assign18320_e33682)) {
                let assign18320_e33689: f64 = (p.p1722 * var_deltemp);
                let assign18320_e33690: f64 = (1.0 + assign18320_e33689);
                let assign18320_e33692: f64 = (assign18320_e33690 - 1e-6);
                let assign18320_e33696: f64 = (p.p1722 * var_deltemp);
                let assign18320_e33697: f64 = (1.0 + assign18320_e33696);
                let assign18320_e33699: f64 = (assign18320_e33697 - 1e-6);
                let assign18320_e33703: f64 = (p.p1722 * var_deltemp);
                let assign18320_e33704: f64 = (1.0 + assign18320_e33703);
                let assign18320_e33706: f64 = (assign18320_e33704 - 1e-6);
                let assign18320_e33707: f64 = (assign18320_e33699 * assign18320_e33706);
                let assign18320_e33710: f64 = (4.0 * 0.001);
                let assign18320_e33712: f64 = (assign18320_e33710 * 0.001);
                let assign18320_e33713: f64 = (assign18320_e33707 + assign18320_e33712);
                let assign18320_e33714: f64 = (assign18320_e33713).sqrt();
                let assign18320_e33715: f64 = (assign18320_e33692 + assign18320_e33714);
                let assign18320_e33716: f64 = (0.5 * assign18320_e33715);
                (assign18320_e33716, (0.5 * ((p.p1722 * var_deltemp_dn4) + ((((p.p1722 * var_deltemp_dn4) * assign18320_e33706) + (assign18320_e33699 * (p.p1722 * var_deltemp_dn4))) / (2.0 * assign18320_e33714)))),)
            } else {
                let assign18320_e33720: f64 = (p.p1722 * var_deltemp);
                let assign18320_e33721: f64 = (1.0 + assign18320_e33720);
                let assign18320_e33723: f64 = (assign18320_e33721 - 1e-6);
                let assign18320_e33725: f64 = (-10000.0);
                let assign18320_e33727: f64 = (assign18320_e33725 * 0.001);
                let (assign18320_e33742, assign18320_e33742_d_n4,) = {
                    if (assign18320_e33723 < assign18320_e33727) {
                        let assign18320_e33730: f64 = (-0.001);
                        let assign18320_e33732: f64 = (assign18320_e33730 * 0.001);
                        let assign18320_e33736: f64 = (p.p1722 * var_deltemp);
                        let assign18320_e33737: f64 = (1.0 + assign18320_e33736);
                        let assign18320_e33739: f64 = (assign18320_e33737 - 1e-6);
                        let assign18320_e33740: f64 = (assign18320_e33732 / assign18320_e33739);
                        (assign18320_e33740, (-((assign18320_e33732 * (p.p1722 * var_deltemp_dn4)) / (assign18320_e33739 * assign18320_e33739))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18320_e33742, assign18320_e33742_d_n4,)
            }
        };
        let assign18320_e33744: f64 = (p.p1586 * assign18320_e33743);
        (assign18320_e33744, (p.p1586 * assign18320_e33743_d_n4),)
    } else {
        (var_cjsws_t, var_cjsws_t_dn4,)
    }
};
        var_cjsws_t = assign18320_e33746;
        var_cjsws_t_dn4 = assign18320_e33746_d_n4;
        var_cjsws_t_rv = 0.0;

        let assign18330_e33749: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard340 = assign18330_e33749;
        var_guard340_rv = 0.0;

        *var_alpha0_t_slot = var_alpha0_t;
        *var_alpha0_t_dn4_slot = var_alpha0_t_dn4;
        *var_alpha0_t_rv_slot = var_alpha0_t_rv;
        *var_alpha1_t_slot = var_alpha1_t;
        *var_alpha1_t_dn4_slot = var_alpha1_t_dn4;
        *var_alpha1_t_rv_slot = var_alpha1_t_rv;
        *var_alphaii0_t_slot = var_alphaii0_t;
        *var_alphaii0_t_dn4_slot = var_alphaii0_t_dn4;
        *var_alphaii0_t_rv_slot = var_alphaii0_t_rv;
        *var_alphaii1_t_slot = var_alphaii1_t;
        *var_alphaii1_t_dn4_slot = var_alphaii1_t_dn4;
        *var_alphaii1_t_rv_slot = var_alphaii1_t_rv;
        *var_bgidl_t_slot = var_bgidl_t;
        *var_bgidl_t_dn4_slot = var_bgidl_t_dn4;
        *var_bgidl_t_rv_slot = var_bgidl_t_rv;
        *var_bgisl_t_slot = var_bgisl_t;
        *var_bgisl_t_dn4_slot = var_bgisl_t_dn4;
        *var_bgisl_t_rv_slot = var_bgisl_t_rv;
        *var_cjd_t_slot = var_cjd_t;
        *var_cjd_t_dn4_slot = var_cjd_t_dn4;
        *var_cjd_t_rv_slot = var_cjd_t_rv;
        *var_cjs_t_slot = var_cjs_t;
        *var_cjs_t_dn4_slot = var_cjs_t_dn4;
        *var_cjs_t_rv_slot = var_cjs_t_rv;
        *var_cjsws_t_slot = var_cjsws_t;
        *var_cjsws_t_dn4_slot = var_cjsws_t_dn4;
        *var_cjsws_t_rv_slot = var_cjsws_t_rv;
        *var_guard336_slot = var_guard336;
        *var_guard336_rv_slot = var_guard336_rv;
        *var_guard337_slot = var_guard337;
        *var_guard337_rv_slot = var_guard337_rv;
        *var_guard338_slot = var_guard338;
        *var_guard338_rv_slot = var_guard338_rv;
        *var_guard339_slot = var_guard339;
        *var_guard339_rv_slot = var_guard339_rv;
        *var_guard340_slot = var_guard340;
        *var_guard340_rv_slot = var_guard340_rv;
    }

    pub(super) fn stamp_reactive_block_64(
        p: &Parameters,
        var_deltemp: f64,
        var_deltemp_dn4: f64,
        var_guard336: f64,
        var_guard340: f64,
        var_cjswd_t_slot: &mut f64,
        var_cjswd_t_dn4_slot: &mut f64,
        var_cjswd_t_rv_slot: &mut f64,
        var_cjswgd_t_slot: &mut f64,
        var_cjswgd_t_dn4_slot: &mut f64,
        var_cjswgd_t_rv_slot: &mut f64,
        var_cjswgs_t_slot: &mut f64,
        var_cjswgs_t_dn4_slot: &mut f64,
        var_cjswgs_t_rv_slot: &mut f64,
        var_guard341_slot: &mut f64,
        var_guard341_rv_slot: &mut f64,
        var_guard342_slot: &mut f64,
        var_guard342_rv_slot: &mut f64,
        var_pbd_t_slot: &mut f64,
        var_pbd_t_dn4_slot: &mut f64,
        var_pbd_t_rv_slot: &mut f64,
        var_pbs_t_slot: &mut f64,
        var_pbs_t_dn4_slot: &mut f64,
        var_pbs_t_rv_slot: &mut f64,
        var_pbsws_t_slot: &mut f64,
        var_pbsws_t_dn4_slot: &mut f64,
        var_pbsws_t_rv_slot: &mut f64,
    ) {
        let mut var_cjswd_t: f64 = *var_cjswd_t_slot;
        let mut var_cjswd_t_dn4: f64 = *var_cjswd_t_dn4_slot;
        let mut var_cjswd_t_rv: f64 = *var_cjswd_t_rv_slot;
        let mut var_cjswgd_t: f64 = *var_cjswgd_t_slot;
        let mut var_cjswgd_t_dn4: f64 = *var_cjswgd_t_dn4_slot;
        let mut var_cjswgd_t_rv: f64 = *var_cjswgd_t_rv_slot;
        let mut var_cjswgs_t: f64 = *var_cjswgs_t_slot;
        let mut var_cjswgs_t_dn4: f64 = *var_cjswgs_t_dn4_slot;
        let mut var_cjswgs_t_rv: f64 = *var_cjswgs_t_rv_slot;
        let mut var_guard341: f64 = *var_guard341_slot;
        let mut var_guard341_rv: f64 = *var_guard341_rv_slot;
        let mut var_guard342: f64 = *var_guard342_slot;
        let mut var_guard342_rv: f64 = *var_guard342_rv_slot;
        let mut var_pbd_t: f64 = *var_pbd_t_slot;
        let mut var_pbd_t_dn4: f64 = *var_pbd_t_dn4_slot;
        let mut var_pbd_t_rv: f64 = *var_pbd_t_rv_slot;
        let mut var_pbs_t: f64 = *var_pbs_t_slot;
        let mut var_pbs_t_dn4: f64 = *var_pbs_t_dn4_slot;
        let mut var_pbs_t_rv: f64 = *var_pbs_t_rv_slot;
        let mut var_pbsws_t: f64 = *var_pbsws_t_slot;
        let mut var_pbsws_t_dn4: f64 = *var_pbsws_t_dn4_slot;
        let mut var_pbsws_t_rv: f64 = *var_pbsws_t_rv_slot;

        let (assign18340_e33795, assign18340_e33795_d_n4,) = {
    if ((var_guard336 != 0.0) && (var_guard340 != 0.0)) {
        let assign18340_e33755: f64 = (-p.p1587);
        let assign18340_e33759: f64 = (p.p1722 * var_deltemp);
        let assign18340_e33761: f64 = (-p.p1587);
        let assign18340_e33762: f64 = (assign18340_e33759 - assign18340_e33761);
        let assign18340_e33764: f64 = (assign18340_e33762 - 1e-6);
        let assign18340_e33767: f64 = (p.p1722 * var_deltemp);
        let assign18340_e33769: f64 = (-p.p1587);
        let assign18340_e33770: f64 = (assign18340_e33767 - assign18340_e33769);
        let assign18340_e33772: f64 = (assign18340_e33770 - 1e-6);
        let assign18340_e33775: f64 = (p.p1722 * var_deltemp);
        let assign18340_e33777: f64 = (-p.p1587);
        let assign18340_e33778: f64 = (assign18340_e33775 - assign18340_e33777);
        let assign18340_e33780: f64 = (assign18340_e33778 - 1e-6);
        let assign18340_e33781: f64 = (assign18340_e33772 * assign18340_e33780);
        let assign18340_e33784: f64 = (-p.p1587);
        let assign18340_e33785: f64 = (4.0 * assign18340_e33784);
        let assign18340_e33787: f64 = (assign18340_e33785 * 1e-6);
        let assign18340_e33788: f64 = (assign18340_e33781 - assign18340_e33787);
        let assign18340_e33789: f64 = (assign18340_e33788).sqrt();
        let assign18340_e33790: f64 = (assign18340_e33764 + assign18340_e33789);
        let assign18340_e33791: f64 = (0.5 * assign18340_e33790);
        let assign18340_e33792: f64 = (assign18340_e33755 + assign18340_e33791);
        let assign18340_e33793: f64 = (p.p1587 + assign18340_e33792);
        (assign18340_e33793, (0.5 * ((p.p1722 * var_deltemp_dn4) + ((((p.p1722 * var_deltemp_dn4) * assign18340_e33780) + (assign18340_e33772 * (p.p1722 * var_deltemp_dn4))) / (2.0 * assign18340_e33789)))),)
    } else {
        (var_cjswd_t, var_cjswd_t_dn4,)
    }
};
        var_cjswd_t = assign18340_e33795;
        var_cjswd_t_dn4 = assign18340_e33795_d_n4;
        var_cjswd_t_rv = 0.0;

        let (assign18350_e33875, assign18350_e33875_d_n4,) = {
    if ((var_guard336 != 0.0) && (var_guard340 == 0.0)) {
        let assign18350_e33804: f64 = (p.p1722 * var_deltemp);
        let assign18350_e33805: f64 = (1.0 + assign18350_e33804);
        let assign18350_e33807: f64 = (assign18350_e33805 - 1e-6);
        let assign18350_e33809: f64 = (-10000.0);
        let assign18350_e33811: f64 = (assign18350_e33809 * 0.001);
        let (assign18350_e33872, assign18350_e33872_d_n4,) = {
            if (!(assign18350_e33807 < assign18350_e33811)) {
                let assign18350_e33818: f64 = (p.p1722 * var_deltemp);
                let assign18350_e33819: f64 = (1.0 + assign18350_e33818);
                let assign18350_e33821: f64 = (assign18350_e33819 - 1e-6);
                let assign18350_e33825: f64 = (p.p1722 * var_deltemp);
                let assign18350_e33826: f64 = (1.0 + assign18350_e33825);
                let assign18350_e33828: f64 = (assign18350_e33826 - 1e-6);
                let assign18350_e33832: f64 = (p.p1722 * var_deltemp);
                let assign18350_e33833: f64 = (1.0 + assign18350_e33832);
                let assign18350_e33835: f64 = (assign18350_e33833 - 1e-6);
                let assign18350_e33836: f64 = (assign18350_e33828 * assign18350_e33835);
                let assign18350_e33839: f64 = (4.0 * 0.001);
                let assign18350_e33841: f64 = (assign18350_e33839 * 0.001);
                let assign18350_e33842: f64 = (assign18350_e33836 + assign18350_e33841);
                let assign18350_e33843: f64 = (assign18350_e33842).sqrt();
                let assign18350_e33844: f64 = (assign18350_e33821 + assign18350_e33843);
                let assign18350_e33845: f64 = (0.5 * assign18350_e33844);
                (assign18350_e33845, (0.5 * ((p.p1722 * var_deltemp_dn4) + ((((p.p1722 * var_deltemp_dn4) * assign18350_e33835) + (assign18350_e33828 * (p.p1722 * var_deltemp_dn4))) / (2.0 * assign18350_e33843)))),)
            } else {
                let assign18350_e33849: f64 = (p.p1722 * var_deltemp);
                let assign18350_e33850: f64 = (1.0 + assign18350_e33849);
                let assign18350_e33852: f64 = (assign18350_e33850 - 1e-6);
                let assign18350_e33854: f64 = (-10000.0);
                let assign18350_e33856: f64 = (assign18350_e33854 * 0.001);
                let (assign18350_e33871, assign18350_e33871_d_n4,) = {
                    if (assign18350_e33852 < assign18350_e33856) {
                        let assign18350_e33859: f64 = (-0.001);
                        let assign18350_e33861: f64 = (assign18350_e33859 * 0.001);
                        let assign18350_e33865: f64 = (p.p1722 * var_deltemp);
                        let assign18350_e33866: f64 = (1.0 + assign18350_e33865);
                        let assign18350_e33868: f64 = (assign18350_e33866 - 1e-6);
                        let assign18350_e33869: f64 = (assign18350_e33861 / assign18350_e33868);
                        (assign18350_e33869, (-((assign18350_e33861 * (p.p1722 * var_deltemp_dn4)) / (assign18350_e33868 * assign18350_e33868))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18350_e33871, assign18350_e33871_d_n4,)
            }
        };
        let assign18350_e33873: f64 = (p.p1587 * assign18350_e33872);
        (assign18350_e33873, (p.p1587 * assign18350_e33872_d_n4),)
    } else {
        (var_cjswd_t, var_cjswd_t_dn4,)
    }
};
        var_cjswd_t = assign18350_e33875;
        var_cjswd_t_dn4 = assign18350_e33875_d_n4;
        var_cjswd_t_rv = 0.0;

        let assign18360_e33878: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard341 = assign18360_e33878;
        var_guard341_rv = 0.0;

        let (assign18370_e33924, assign18370_e33924_d_n4,) = {
    if ((var_guard336 != 0.0) && (var_guard341 != 0.0)) {
        let assign18370_e33884: f64 = (-p.p1588);
        let assign18370_e33888: f64 = (p.p1723 * var_deltemp);
        let assign18370_e33890: f64 = (-p.p1588);
        let assign18370_e33891: f64 = (assign18370_e33888 - assign18370_e33890);
        let assign18370_e33893: f64 = (assign18370_e33891 - 1e-6);
        let assign18370_e33896: f64 = (p.p1723 * var_deltemp);
        let assign18370_e33898: f64 = (-p.p1588);
        let assign18370_e33899: f64 = (assign18370_e33896 - assign18370_e33898);
        let assign18370_e33901: f64 = (assign18370_e33899 - 1e-6);
        let assign18370_e33904: f64 = (p.p1723 * var_deltemp);
        let assign18370_e33906: f64 = (-p.p1588);
        let assign18370_e33907: f64 = (assign18370_e33904 - assign18370_e33906);
        let assign18370_e33909: f64 = (assign18370_e33907 - 1e-6);
        let assign18370_e33910: f64 = (assign18370_e33901 * assign18370_e33909);
        let assign18370_e33913: f64 = (-p.p1588);
        let assign18370_e33914: f64 = (4.0 * assign18370_e33913);
        let assign18370_e33916: f64 = (assign18370_e33914 * 1e-6);
        let assign18370_e33917: f64 = (assign18370_e33910 - assign18370_e33916);
        let assign18370_e33918: f64 = (assign18370_e33917).sqrt();
        let assign18370_e33919: f64 = (assign18370_e33893 + assign18370_e33918);
        let assign18370_e33920: f64 = (0.5 * assign18370_e33919);
        let assign18370_e33921: f64 = (assign18370_e33884 + assign18370_e33920);
        let assign18370_e33922: f64 = (p.p1588 + assign18370_e33921);
        (assign18370_e33922, (0.5 * ((p.p1723 * var_deltemp_dn4) + ((((p.p1723 * var_deltemp_dn4) * assign18370_e33909) + (assign18370_e33901 * (p.p1723 * var_deltemp_dn4))) / (2.0 * assign18370_e33918)))),)
    } else {
        (var_cjswgs_t, var_cjswgs_t_dn4,)
    }
};
        var_cjswgs_t = assign18370_e33924;
        var_cjswgs_t_dn4 = assign18370_e33924_d_n4;
        var_cjswgs_t_rv = 0.0;

        let (assign18380_e34004, assign18380_e34004_d_n4,) = {
    if ((var_guard336 != 0.0) && (var_guard341 == 0.0)) {
        let assign18380_e33933: f64 = (p.p1723 * var_deltemp);
        let assign18380_e33934: f64 = (1.0 + assign18380_e33933);
        let assign18380_e33936: f64 = (assign18380_e33934 - 1e-6);
        let assign18380_e33938: f64 = (-10000.0);
        let assign18380_e33940: f64 = (assign18380_e33938 * 0.001);
        let (assign18380_e34001, assign18380_e34001_d_n4,) = {
            if (!(assign18380_e33936 < assign18380_e33940)) {
                let assign18380_e33947: f64 = (p.p1723 * var_deltemp);
                let assign18380_e33948: f64 = (1.0 + assign18380_e33947);
                let assign18380_e33950: f64 = (assign18380_e33948 - 1e-6);
                let assign18380_e33954: f64 = (p.p1723 * var_deltemp);
                let assign18380_e33955: f64 = (1.0 + assign18380_e33954);
                let assign18380_e33957: f64 = (assign18380_e33955 - 1e-6);
                let assign18380_e33961: f64 = (p.p1723 * var_deltemp);
                let assign18380_e33962: f64 = (1.0 + assign18380_e33961);
                let assign18380_e33964: f64 = (assign18380_e33962 - 1e-6);
                let assign18380_e33965: f64 = (assign18380_e33957 * assign18380_e33964);
                let assign18380_e33968: f64 = (4.0 * 0.001);
                let assign18380_e33970: f64 = (assign18380_e33968 * 0.001);
                let assign18380_e33971: f64 = (assign18380_e33965 + assign18380_e33970);
                let assign18380_e33972: f64 = (assign18380_e33971).sqrt();
                let assign18380_e33973: f64 = (assign18380_e33950 + assign18380_e33972);
                let assign18380_e33974: f64 = (0.5 * assign18380_e33973);
                (assign18380_e33974, (0.5 * ((p.p1723 * var_deltemp_dn4) + ((((p.p1723 * var_deltemp_dn4) * assign18380_e33964) + (assign18380_e33957 * (p.p1723 * var_deltemp_dn4))) / (2.0 * assign18380_e33972)))),)
            } else {
                let assign18380_e33978: f64 = (p.p1723 * var_deltemp);
                let assign18380_e33979: f64 = (1.0 + assign18380_e33978);
                let assign18380_e33981: f64 = (assign18380_e33979 - 1e-6);
                let assign18380_e33983: f64 = (-10000.0);
                let assign18380_e33985: f64 = (assign18380_e33983 * 0.001);
                let (assign18380_e34000, assign18380_e34000_d_n4,) = {
                    if (assign18380_e33981 < assign18380_e33985) {
                        let assign18380_e33988: f64 = (-0.001);
                        let assign18380_e33990: f64 = (assign18380_e33988 * 0.001);
                        let assign18380_e33994: f64 = (p.p1723 * var_deltemp);
                        let assign18380_e33995: f64 = (1.0 + assign18380_e33994);
                        let assign18380_e33997: f64 = (assign18380_e33995 - 1e-6);
                        let assign18380_e33998: f64 = (assign18380_e33990 / assign18380_e33997);
                        (assign18380_e33998, (-((assign18380_e33990 * (p.p1723 * var_deltemp_dn4)) / (assign18380_e33997 * assign18380_e33997))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18380_e34000, assign18380_e34000_d_n4,)
            }
        };
        let assign18380_e34002: f64 = (p.p1588 * assign18380_e34001);
        (assign18380_e34002, (p.p1588 * assign18380_e34001_d_n4),)
    } else {
        (var_cjswgs_t, var_cjswgs_t_dn4,)
    }
};
        var_cjswgs_t = assign18380_e34004;
        var_cjswgs_t_dn4 = assign18380_e34004_d_n4;
        var_cjswgs_t_rv = 0.0;

        let assign18390_e34007: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard342 = assign18390_e34007;
        var_guard342_rv = 0.0;

        let (assign18400_e34053, assign18400_e34053_d_n4,) = {
    if ((var_guard336 != 0.0) && (var_guard342 != 0.0)) {
        let assign18400_e34013: f64 = (-p.p1589);
        let assign18400_e34017: f64 = (p.p1723 * var_deltemp);
        let assign18400_e34019: f64 = (-p.p1589);
        let assign18400_e34020: f64 = (assign18400_e34017 - assign18400_e34019);
        let assign18400_e34022: f64 = (assign18400_e34020 - 1e-6);
        let assign18400_e34025: f64 = (p.p1723 * var_deltemp);
        let assign18400_e34027: f64 = (-p.p1589);
        let assign18400_e34028: f64 = (assign18400_e34025 - assign18400_e34027);
        let assign18400_e34030: f64 = (assign18400_e34028 - 1e-6);
        let assign18400_e34033: f64 = (p.p1723 * var_deltemp);
        let assign18400_e34035: f64 = (-p.p1589);
        let assign18400_e34036: f64 = (assign18400_e34033 - assign18400_e34035);
        let assign18400_e34038: f64 = (assign18400_e34036 - 1e-6);
        let assign18400_e34039: f64 = (assign18400_e34030 * assign18400_e34038);
        let assign18400_e34042: f64 = (-p.p1589);
        let assign18400_e34043: f64 = (4.0 * assign18400_e34042);
        let assign18400_e34045: f64 = (assign18400_e34043 * 1e-6);
        let assign18400_e34046: f64 = (assign18400_e34039 - assign18400_e34045);
        let assign18400_e34047: f64 = (assign18400_e34046).sqrt();
        let assign18400_e34048: f64 = (assign18400_e34022 + assign18400_e34047);
        let assign18400_e34049: f64 = (0.5 * assign18400_e34048);
        let assign18400_e34050: f64 = (assign18400_e34013 + assign18400_e34049);
        let assign18400_e34051: f64 = (p.p1589 + assign18400_e34050);
        (assign18400_e34051, (0.5 * ((p.p1723 * var_deltemp_dn4) + ((((p.p1723 * var_deltemp_dn4) * assign18400_e34038) + (assign18400_e34030 * (p.p1723 * var_deltemp_dn4))) / (2.0 * assign18400_e34047)))),)
    } else {
        (var_cjswgd_t, var_cjswgd_t_dn4,)
    }
};
        var_cjswgd_t = assign18400_e34053;
        var_cjswgd_t_dn4 = assign18400_e34053_d_n4;
        var_cjswgd_t_rv = 0.0;

        let (assign18410_e34133, assign18410_e34133_d_n4,) = {
    if ((var_guard336 != 0.0) && (var_guard342 == 0.0)) {
        let assign18410_e34062: f64 = (p.p1723 * var_deltemp);
        let assign18410_e34063: f64 = (1.0 + assign18410_e34062);
        let assign18410_e34065: f64 = (assign18410_e34063 - 1e-6);
        let assign18410_e34067: f64 = (-10000.0);
        let assign18410_e34069: f64 = (assign18410_e34067 * 0.001);
        let (assign18410_e34130, assign18410_e34130_d_n4,) = {
            if (!(assign18410_e34065 < assign18410_e34069)) {
                let assign18410_e34076: f64 = (p.p1723 * var_deltemp);
                let assign18410_e34077: f64 = (1.0 + assign18410_e34076);
                let assign18410_e34079: f64 = (assign18410_e34077 - 1e-6);
                let assign18410_e34083: f64 = (p.p1723 * var_deltemp);
                let assign18410_e34084: f64 = (1.0 + assign18410_e34083);
                let assign18410_e34086: f64 = (assign18410_e34084 - 1e-6);
                let assign18410_e34090: f64 = (p.p1723 * var_deltemp);
                let assign18410_e34091: f64 = (1.0 + assign18410_e34090);
                let assign18410_e34093: f64 = (assign18410_e34091 - 1e-6);
                let assign18410_e34094: f64 = (assign18410_e34086 * assign18410_e34093);
                let assign18410_e34097: f64 = (4.0 * 0.001);
                let assign18410_e34099: f64 = (assign18410_e34097 * 0.001);
                let assign18410_e34100: f64 = (assign18410_e34094 + assign18410_e34099);
                let assign18410_e34101: f64 = (assign18410_e34100).sqrt();
                let assign18410_e34102: f64 = (assign18410_e34079 + assign18410_e34101);
                let assign18410_e34103: f64 = (0.5 * assign18410_e34102);
                (assign18410_e34103, (0.5 * ((p.p1723 * var_deltemp_dn4) + ((((p.p1723 * var_deltemp_dn4) * assign18410_e34093) + (assign18410_e34086 * (p.p1723 * var_deltemp_dn4))) / (2.0 * assign18410_e34101)))),)
            } else {
                let assign18410_e34107: f64 = (p.p1723 * var_deltemp);
                let assign18410_e34108: f64 = (1.0 + assign18410_e34107);
                let assign18410_e34110: f64 = (assign18410_e34108 - 1e-6);
                let assign18410_e34112: f64 = (-10000.0);
                let assign18410_e34114: f64 = (assign18410_e34112 * 0.001);
                let (assign18410_e34129, assign18410_e34129_d_n4,) = {
                    if (assign18410_e34110 < assign18410_e34114) {
                        let assign18410_e34117: f64 = (-0.001);
                        let assign18410_e34119: f64 = (assign18410_e34117 * 0.001);
                        let assign18410_e34123: f64 = (p.p1723 * var_deltemp);
                        let assign18410_e34124: f64 = (1.0 + assign18410_e34123);
                        let assign18410_e34126: f64 = (assign18410_e34124 - 1e-6);
                        let assign18410_e34127: f64 = (assign18410_e34119 / assign18410_e34126);
                        (assign18410_e34127, (-((assign18410_e34119 * (p.p1723 * var_deltemp_dn4)) / (assign18410_e34126 * assign18410_e34126))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18410_e34129, assign18410_e34129_d_n4,)
            }
        };
        let assign18410_e34131: f64 = (p.p1589 * assign18410_e34130);
        (assign18410_e34131, (p.p1589 * assign18410_e34130_d_n4),)
    } else {
        (var_cjswgd_t, var_cjswgd_t_dn4,)
    }
};
        var_cjswgd_t = assign18410_e34133;
        var_cjswgd_t_dn4 = assign18410_e34133_d_n4;
        var_cjswgd_t_rv = 0.0;

        let (assign18420_e34210, assign18420_e34210_d_n4,) = {
    if (var_guard336 != 0.0) {
        let assign18420_e34138: f64 = (p.p1724 * var_deltemp);
        let assign18420_e34139: f64 = (p.p1590 - assign18420_e34138);
        let assign18420_e34141: f64 = (assign18420_e34139 - 0.01);
        let assign18420_e34143: f64 = (-10000.0);
        let assign18420_e34145: f64 = (assign18420_e34143 * 0.001);
        let (assign18420_e34206, assign18420_e34206_d_n4,) = {
            if (!(assign18420_e34141 < assign18420_e34145)) {
                let assign18420_e34152: f64 = (p.p1724 * var_deltemp);
                let assign18420_e34153: f64 = (p.p1590 - assign18420_e34152);
                let assign18420_e34155: f64 = (assign18420_e34153 - 0.01);
                let assign18420_e34159: f64 = (p.p1724 * var_deltemp);
                let assign18420_e34160: f64 = (p.p1590 - assign18420_e34159);
                let assign18420_e34162: f64 = (assign18420_e34160 - 0.01);
                let assign18420_e34166: f64 = (p.p1724 * var_deltemp);
                let assign18420_e34167: f64 = (p.p1590 - assign18420_e34166);
                let assign18420_e34169: f64 = (assign18420_e34167 - 0.01);
                let assign18420_e34170: f64 = (assign18420_e34162 * assign18420_e34169);
                let assign18420_e34173: f64 = (4.0 * 0.001);
                let assign18420_e34175: f64 = (assign18420_e34173 * 0.001);
                let assign18420_e34176: f64 = (assign18420_e34170 + assign18420_e34175);
                let assign18420_e34177: f64 = (assign18420_e34176).sqrt();
                let assign18420_e34178: f64 = (assign18420_e34155 + assign18420_e34177);
                let assign18420_e34179: f64 = (0.5 * assign18420_e34178);
                (assign18420_e34179, (0.5 * ((-(p.p1724 * var_deltemp_dn4)) + ((((-(p.p1724 * var_deltemp_dn4)) * assign18420_e34169) + (assign18420_e34162 * (-(p.p1724 * var_deltemp_dn4)))) / (2.0 * assign18420_e34177)))),)
            } else {
                let assign18420_e34183: f64 = (p.p1724 * var_deltemp);
                let assign18420_e34184: f64 = (p.p1590 - assign18420_e34183);
                let assign18420_e34186: f64 = (assign18420_e34184 - 0.01);
                let assign18420_e34188: f64 = (-10000.0);
                let assign18420_e34190: f64 = (assign18420_e34188 * 0.001);
                let (assign18420_e34205, assign18420_e34205_d_n4,) = {
                    if (assign18420_e34186 < assign18420_e34190) {
                        let assign18420_e34193: f64 = (-0.001);
                        let assign18420_e34195: f64 = (assign18420_e34193 * 0.001);
                        let assign18420_e34199: f64 = (p.p1724 * var_deltemp);
                        let assign18420_e34200: f64 = (p.p1590 - assign18420_e34199);
                        let assign18420_e34202: f64 = (assign18420_e34200 - 0.01);
                        let assign18420_e34203: f64 = (assign18420_e34195 / assign18420_e34202);
                        (assign18420_e34203, (-((assign18420_e34195 * (-(p.p1724 * var_deltemp_dn4))) / (assign18420_e34202 * assign18420_e34202))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18420_e34205, assign18420_e34205_d_n4,)
            }
        };
        let assign18420_e34208: f64 = (assign18420_e34206 + 0.01);
        (assign18420_e34208, assign18420_e34206_d_n4,)
    } else {
        (var_pbs_t, var_pbs_t_dn4,)
    }
};
        var_pbs_t = assign18420_e34210;
        var_pbs_t_dn4 = assign18420_e34210_d_n4;
        var_pbs_t_rv = 0.0;

        let (assign18430_e34287, assign18430_e34287_d_n4,) = {
    if (var_guard336 != 0.0) {
        let assign18430_e34215: f64 = (p.p1724 * var_deltemp);
        let assign18430_e34216: f64 = (p.p1591 - assign18430_e34215);
        let assign18430_e34218: f64 = (assign18430_e34216 - 0.01);
        let assign18430_e34220: f64 = (-10000.0);
        let assign18430_e34222: f64 = (assign18430_e34220 * 0.001);
        let (assign18430_e34283, assign18430_e34283_d_n4,) = {
            if (!(assign18430_e34218 < assign18430_e34222)) {
                let assign18430_e34229: f64 = (p.p1724 * var_deltemp);
                let assign18430_e34230: f64 = (p.p1591 - assign18430_e34229);
                let assign18430_e34232: f64 = (assign18430_e34230 - 0.01);
                let assign18430_e34236: f64 = (p.p1724 * var_deltemp);
                let assign18430_e34237: f64 = (p.p1591 - assign18430_e34236);
                let assign18430_e34239: f64 = (assign18430_e34237 - 0.01);
                let assign18430_e34243: f64 = (p.p1724 * var_deltemp);
                let assign18430_e34244: f64 = (p.p1591 - assign18430_e34243);
                let assign18430_e34246: f64 = (assign18430_e34244 - 0.01);
                let assign18430_e34247: f64 = (assign18430_e34239 * assign18430_e34246);
                let assign18430_e34250: f64 = (4.0 * 0.001);
                let assign18430_e34252: f64 = (assign18430_e34250 * 0.001);
                let assign18430_e34253: f64 = (assign18430_e34247 + assign18430_e34252);
                let assign18430_e34254: f64 = (assign18430_e34253).sqrt();
                let assign18430_e34255: f64 = (assign18430_e34232 + assign18430_e34254);
                let assign18430_e34256: f64 = (0.5 * assign18430_e34255);
                (assign18430_e34256, (0.5 * ((-(p.p1724 * var_deltemp_dn4)) + ((((-(p.p1724 * var_deltemp_dn4)) * assign18430_e34246) + (assign18430_e34239 * (-(p.p1724 * var_deltemp_dn4)))) / (2.0 * assign18430_e34254)))),)
            } else {
                let assign18430_e34260: f64 = (p.p1724 * var_deltemp);
                let assign18430_e34261: f64 = (p.p1591 - assign18430_e34260);
                let assign18430_e34263: f64 = (assign18430_e34261 - 0.01);
                let assign18430_e34265: f64 = (-10000.0);
                let assign18430_e34267: f64 = (assign18430_e34265 * 0.001);
                let (assign18430_e34282, assign18430_e34282_d_n4,) = {
                    if (assign18430_e34263 < assign18430_e34267) {
                        let assign18430_e34270: f64 = (-0.001);
                        let assign18430_e34272: f64 = (assign18430_e34270 * 0.001);
                        let assign18430_e34276: f64 = (p.p1724 * var_deltemp);
                        let assign18430_e34277: f64 = (p.p1591 - assign18430_e34276);
                        let assign18430_e34279: f64 = (assign18430_e34277 - 0.01);
                        let assign18430_e34280: f64 = (assign18430_e34272 / assign18430_e34279);
                        (assign18430_e34280, (-((assign18430_e34272 * (-(p.p1724 * var_deltemp_dn4))) / (assign18430_e34279 * assign18430_e34279))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18430_e34282, assign18430_e34282_d_n4,)
            }
        };
        let assign18430_e34285: f64 = (assign18430_e34283 + 0.01);
        (assign18430_e34285, assign18430_e34283_d_n4,)
    } else {
        (var_pbd_t, var_pbd_t_dn4,)
    }
};
        var_pbd_t = assign18430_e34287;
        var_pbd_t_dn4 = assign18430_e34287_d_n4;
        var_pbd_t_rv = 0.0;

        let (assign18440_e34364, assign18440_e34364_d_n4,) = {
    if (var_guard336 != 0.0) {
        let assign18440_e34292: f64 = (p.p1725 * var_deltemp);
        let assign18440_e34293: f64 = (p.p1592 - assign18440_e34292);
        let assign18440_e34295: f64 = (assign18440_e34293 - 0.01);
        let assign18440_e34297: f64 = (-10000.0);
        let assign18440_e34299: f64 = (assign18440_e34297 * 0.001);
        let (assign18440_e34360, assign18440_e34360_d_n4,) = {
            if (!(assign18440_e34295 < assign18440_e34299)) {
                let assign18440_e34306: f64 = (p.p1725 * var_deltemp);
                let assign18440_e34307: f64 = (p.p1592 - assign18440_e34306);
                let assign18440_e34309: f64 = (assign18440_e34307 - 0.01);
                let assign18440_e34313: f64 = (p.p1725 * var_deltemp);
                let assign18440_e34314: f64 = (p.p1592 - assign18440_e34313);
                let assign18440_e34316: f64 = (assign18440_e34314 - 0.01);
                let assign18440_e34320: f64 = (p.p1725 * var_deltemp);
                let assign18440_e34321: f64 = (p.p1592 - assign18440_e34320);
                let assign18440_e34323: f64 = (assign18440_e34321 - 0.01);
                let assign18440_e34324: f64 = (assign18440_e34316 * assign18440_e34323);
                let assign18440_e34327: f64 = (4.0 * 0.001);
                let assign18440_e34329: f64 = (assign18440_e34327 * 0.001);
                let assign18440_e34330: f64 = (assign18440_e34324 + assign18440_e34329);
                let assign18440_e34331: f64 = (assign18440_e34330).sqrt();
                let assign18440_e34332: f64 = (assign18440_e34309 + assign18440_e34331);
                let assign18440_e34333: f64 = (0.5 * assign18440_e34332);
                (assign18440_e34333, (0.5 * ((-(p.p1725 * var_deltemp_dn4)) + ((((-(p.p1725 * var_deltemp_dn4)) * assign18440_e34323) + (assign18440_e34316 * (-(p.p1725 * var_deltemp_dn4)))) / (2.0 * assign18440_e34331)))),)
            } else {
                let assign18440_e34337: f64 = (p.p1725 * var_deltemp);
                let assign18440_e34338: f64 = (p.p1592 - assign18440_e34337);
                let assign18440_e34340: f64 = (assign18440_e34338 - 0.01);
                let assign18440_e34342: f64 = (-10000.0);
                let assign18440_e34344: f64 = (assign18440_e34342 * 0.001);
                let (assign18440_e34359, assign18440_e34359_d_n4,) = {
                    if (assign18440_e34340 < assign18440_e34344) {
                        let assign18440_e34347: f64 = (-0.001);
                        let assign18440_e34349: f64 = (assign18440_e34347 * 0.001);
                        let assign18440_e34353: f64 = (p.p1725 * var_deltemp);
                        let assign18440_e34354: f64 = (p.p1592 - assign18440_e34353);
                        let assign18440_e34356: f64 = (assign18440_e34354 - 0.01);
                        let assign18440_e34357: f64 = (assign18440_e34349 / assign18440_e34356);
                        (assign18440_e34357, (-((assign18440_e34349 * (-(p.p1725 * var_deltemp_dn4))) / (assign18440_e34356 * assign18440_e34356))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18440_e34359, assign18440_e34359_d_n4,)
            }
        };
        let assign18440_e34362: f64 = (assign18440_e34360 + 0.01);
        (assign18440_e34362, assign18440_e34360_d_n4,)
    } else {
        (var_pbsws_t, var_pbsws_t_dn4,)
    }
};
        var_pbsws_t = assign18440_e34364;
        var_pbsws_t_dn4 = assign18440_e34364_d_n4;
        var_pbsws_t_rv = 0.0;

        *var_cjswd_t_slot = var_cjswd_t;
        *var_cjswd_t_dn4_slot = var_cjswd_t_dn4;
        *var_cjswd_t_rv_slot = var_cjswd_t_rv;
        *var_cjswgd_t_slot = var_cjswgd_t;
        *var_cjswgd_t_dn4_slot = var_cjswgd_t_dn4;
        *var_cjswgd_t_rv_slot = var_cjswgd_t_rv;
        *var_cjswgs_t_slot = var_cjswgs_t;
        *var_cjswgs_t_dn4_slot = var_cjswgs_t_dn4;
        *var_cjswgs_t_rv_slot = var_cjswgs_t_rv;
        *var_guard341_slot = var_guard341;
        *var_guard341_rv_slot = var_guard341_rv;
        *var_guard342_slot = var_guard342;
        *var_guard342_rv_slot = var_guard342_rv;
        *var_pbd_t_slot = var_pbd_t;
        *var_pbd_t_dn4_slot = var_pbd_t_dn4;
        *var_pbd_t_rv_slot = var_pbd_t_rv;
        *var_pbs_t_slot = var_pbs_t;
        *var_pbs_t_dn4_slot = var_pbs_t_dn4;
        *var_pbs_t_rv_slot = var_pbs_t_rv;
        *var_pbsws_t_slot = var_pbsws_t;
        *var_pbsws_t_dn4_slot = var_pbsws_t_dn4;
        *var_pbsws_t_rv_slot = var_pbsws_t_rv;
    }

    pub(super) fn stamp_reactive_block_65(
        p: &Parameters,
        var_deltemp: f64,
        var_deltemp_dn4: f64,
        var_eg: f64,
        var_eg0: f64,
        var_eg_dn4: f64,
        var_guard336: f64,
        var_trat_ln: f64,
        var_trat_ln_dn4: f64,
        var_tratio_m1: f64,
        var_tratio_m1_dn4: f64,
        var_vtm: f64,
        var_vtm0: f64,
        var_vtm_dn4: f64,
        var_weff0: f64,
        var_jsd_t_slot: &mut f64,
        var_jsd_t_dn0_slot: &mut f64,
        var_jsd_t_dn10_slot: &mut f64,
        var_jsd_t_dn11_slot: &mut f64,
        var_jsd_t_dn13_slot: &mut f64,
        var_jsd_t_dn14_slot: &mut f64,
        var_jsd_t_dn2_slot: &mut f64,
        var_jsd_t_dn3_slot: &mut f64,
        var_jsd_t_dn4_slot: &mut f64,
        var_jsd_t_dn5_slot: &mut f64,
        var_jsd_t_dn6_slot: &mut f64,
        var_jsd_t_dn7_slot: &mut f64,
        var_jsd_t_dn8_slot: &mut f64,
        var_jsd_t_dn9_slot: &mut f64,
        var_jsd_t_rv_slot: &mut f64,
        var_jss_t_slot: &mut f64,
        var_jss_t_dn0_slot: &mut f64,
        var_jss_t_dn10_slot: &mut f64,
        var_jss_t_dn11_slot: &mut f64,
        var_jss_t_dn13_slot: &mut f64,
        var_jss_t_dn14_slot: &mut f64,
        var_jss_t_dn2_slot: &mut f64,
        var_jss_t_dn3_slot: &mut f64,
        var_jss_t_dn4_slot: &mut f64,
        var_jss_t_dn5_slot: &mut f64,
        var_jss_t_dn6_slot: &mut f64,
        var_jss_t_dn7_slot: &mut f64,
        var_jss_t_dn8_slot: &mut f64,
        var_jss_t_dn9_slot: &mut f64,
        var_jss_t_rv_slot: &mut f64,
        var_jswd_t_slot: &mut f64,
        var_jswd_t_dn0_slot: &mut f64,
        var_jswd_t_dn10_slot: &mut f64,
        var_jswd_t_dn11_slot: &mut f64,
        var_jswd_t_dn13_slot: &mut f64,
        var_jswd_t_dn14_slot: &mut f64,
        var_jswd_t_dn2_slot: &mut f64,
        var_jswd_t_dn3_slot: &mut f64,
        var_jswd_t_dn4_slot: &mut f64,
        var_jswd_t_dn5_slot: &mut f64,
        var_jswd_t_dn6_slot: &mut f64,
        var_jswd_t_dn7_slot: &mut f64,
        var_jswd_t_dn8_slot: &mut f64,
        var_jswd_t_dn9_slot: &mut f64,
        var_jswd_t_rv_slot: &mut f64,
        var_jswgd_t_slot: &mut f64,
        var_jswgd_t_dn0_slot: &mut f64,
        var_jswgd_t_dn10_slot: &mut f64,
        var_jswgd_t_dn11_slot: &mut f64,
        var_jswgd_t_dn13_slot: &mut f64,
        var_jswgd_t_dn14_slot: &mut f64,
        var_jswgd_t_dn2_slot: &mut f64,
        var_jswgd_t_dn3_slot: &mut f64,
        var_jswgd_t_dn4_slot: &mut f64,
        var_jswgd_t_dn5_slot: &mut f64,
        var_jswgd_t_dn6_slot: &mut f64,
        var_jswgd_t_dn7_slot: &mut f64,
        var_jswgd_t_dn8_slot: &mut f64,
        var_jswgd_t_dn9_slot: &mut f64,
        var_jswgd_t_rv_slot: &mut f64,
        var_jswgs_t_slot: &mut f64,
        var_jswgs_t_dn0_slot: &mut f64,
        var_jswgs_t_dn10_slot: &mut f64,
        var_jswgs_t_dn11_slot: &mut f64,
        var_jswgs_t_dn13_slot: &mut f64,
        var_jswgs_t_dn14_slot: &mut f64,
        var_jswgs_t_dn2_slot: &mut f64,
        var_jswgs_t_dn3_slot: &mut f64,
        var_jswgs_t_dn4_slot: &mut f64,
        var_jswgs_t_dn5_slot: &mut f64,
        var_jswgs_t_dn6_slot: &mut f64,
        var_jswgs_t_dn7_slot: &mut f64,
        var_jswgs_t_dn8_slot: &mut f64,
        var_jswgs_t_dn9_slot: &mut f64,
        var_jswgs_t_rv_slot: &mut f64,
        var_jsws_t_slot: &mut f64,
        var_jsws_t_dn0_slot: &mut f64,
        var_jsws_t_dn10_slot: &mut f64,
        var_jsws_t_dn11_slot: &mut f64,
        var_jsws_t_dn13_slot: &mut f64,
        var_jsws_t_dn14_slot: &mut f64,
        var_jsws_t_dn2_slot: &mut f64,
        var_jsws_t_dn3_slot: &mut f64,
        var_jsws_t_dn4_slot: &mut f64,
        var_jsws_t_dn5_slot: &mut f64,
        var_jsws_t_dn6_slot: &mut f64,
        var_jsws_t_dn7_slot: &mut f64,
        var_jsws_t_dn8_slot: &mut f64,
        var_jsws_t_dn9_slot: &mut f64,
        var_jsws_t_rv_slot: &mut f64,
        var_jtsd_t_slot: &mut f64,
        var_jtsd_t_dn4_slot: &mut f64,
        var_jtsd_t_rv_slot: &mut f64,
        var_jtss_t_slot: &mut f64,
        var_jtss_t_dn4_slot: &mut f64,
        var_jtss_t_rv_slot: &mut f64,
        var_jtsswd_t_slot: &mut f64,
        var_jtsswd_t_dn4_slot: &mut f64,
        var_jtsswd_t_rv_slot: &mut f64,
        var_jtsswgd_t_slot: &mut f64,
        var_jtsswgd_t_dn4_slot: &mut f64,
        var_jtsswgd_t_rv_slot: &mut f64,
        var_jtsswgs_t_slot: &mut f64,
        var_jtsswgs_t_dn4_slot: &mut f64,
        var_jtsswgs_t_rv_slot: &mut f64,
        var_jtssws_t_slot: &mut f64,
        var_jtssws_t_dn4_slot: &mut f64,
        var_jtssws_t_rv_slot: &mut f64,
        var_pbswd_t_slot: &mut f64,
        var_pbswd_t_dn4_slot: &mut f64,
        var_pbswd_t_rv_slot: &mut f64,
        var_pbswgd_t_slot: &mut f64,
        var_pbswgd_t_dn4_slot: &mut f64,
        var_pbswgd_t_rv_slot: &mut f64,
        var_pbswgs_t_slot: &mut f64,
        var_pbswgs_t_dn4_slot: &mut f64,
        var_pbswgs_t_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn13_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
    ) {
        let mut var_jsd_t: f64 = *var_jsd_t_slot;
        let mut var_jsd_t_dn0: f64 = *var_jsd_t_dn0_slot;
        let mut var_jsd_t_dn10: f64 = *var_jsd_t_dn10_slot;
        let mut var_jsd_t_dn11: f64 = *var_jsd_t_dn11_slot;
        let mut var_jsd_t_dn13: f64 = *var_jsd_t_dn13_slot;
        let mut var_jsd_t_dn14: f64 = *var_jsd_t_dn14_slot;
        let mut var_jsd_t_dn2: f64 = *var_jsd_t_dn2_slot;
        let mut var_jsd_t_dn3: f64 = *var_jsd_t_dn3_slot;
        let mut var_jsd_t_dn4: f64 = *var_jsd_t_dn4_slot;
        let mut var_jsd_t_dn5: f64 = *var_jsd_t_dn5_slot;
        let mut var_jsd_t_dn6: f64 = *var_jsd_t_dn6_slot;
        let mut var_jsd_t_dn7: f64 = *var_jsd_t_dn7_slot;
        let mut var_jsd_t_dn8: f64 = *var_jsd_t_dn8_slot;
        let mut var_jsd_t_dn9: f64 = *var_jsd_t_dn9_slot;
        let mut var_jsd_t_rv: f64 = *var_jsd_t_rv_slot;
        let mut var_jss_t: f64 = *var_jss_t_slot;
        let mut var_jss_t_dn0: f64 = *var_jss_t_dn0_slot;
        let mut var_jss_t_dn10: f64 = *var_jss_t_dn10_slot;
        let mut var_jss_t_dn11: f64 = *var_jss_t_dn11_slot;
        let mut var_jss_t_dn13: f64 = *var_jss_t_dn13_slot;
        let mut var_jss_t_dn14: f64 = *var_jss_t_dn14_slot;
        let mut var_jss_t_dn2: f64 = *var_jss_t_dn2_slot;
        let mut var_jss_t_dn3: f64 = *var_jss_t_dn3_slot;
        let mut var_jss_t_dn4: f64 = *var_jss_t_dn4_slot;
        let mut var_jss_t_dn5: f64 = *var_jss_t_dn5_slot;
        let mut var_jss_t_dn6: f64 = *var_jss_t_dn6_slot;
        let mut var_jss_t_dn7: f64 = *var_jss_t_dn7_slot;
        let mut var_jss_t_dn8: f64 = *var_jss_t_dn8_slot;
        let mut var_jss_t_dn9: f64 = *var_jss_t_dn9_slot;
        let mut var_jss_t_rv: f64 = *var_jss_t_rv_slot;
        let mut var_jswd_t: f64 = *var_jswd_t_slot;
        let mut var_jswd_t_dn0: f64 = *var_jswd_t_dn0_slot;
        let mut var_jswd_t_dn10: f64 = *var_jswd_t_dn10_slot;
        let mut var_jswd_t_dn11: f64 = *var_jswd_t_dn11_slot;
        let mut var_jswd_t_dn13: f64 = *var_jswd_t_dn13_slot;
        let mut var_jswd_t_dn14: f64 = *var_jswd_t_dn14_slot;
        let mut var_jswd_t_dn2: f64 = *var_jswd_t_dn2_slot;
        let mut var_jswd_t_dn3: f64 = *var_jswd_t_dn3_slot;
        let mut var_jswd_t_dn4: f64 = *var_jswd_t_dn4_slot;
        let mut var_jswd_t_dn5: f64 = *var_jswd_t_dn5_slot;
        let mut var_jswd_t_dn6: f64 = *var_jswd_t_dn6_slot;
        let mut var_jswd_t_dn7: f64 = *var_jswd_t_dn7_slot;
        let mut var_jswd_t_dn8: f64 = *var_jswd_t_dn8_slot;
        let mut var_jswd_t_dn9: f64 = *var_jswd_t_dn9_slot;
        let mut var_jswd_t_rv: f64 = *var_jswd_t_rv_slot;
        let mut var_jswgd_t: f64 = *var_jswgd_t_slot;
        let mut var_jswgd_t_dn0: f64 = *var_jswgd_t_dn0_slot;
        let mut var_jswgd_t_dn10: f64 = *var_jswgd_t_dn10_slot;
        let mut var_jswgd_t_dn11: f64 = *var_jswgd_t_dn11_slot;
        let mut var_jswgd_t_dn13: f64 = *var_jswgd_t_dn13_slot;
        let mut var_jswgd_t_dn14: f64 = *var_jswgd_t_dn14_slot;
        let mut var_jswgd_t_dn2: f64 = *var_jswgd_t_dn2_slot;
        let mut var_jswgd_t_dn3: f64 = *var_jswgd_t_dn3_slot;
        let mut var_jswgd_t_dn4: f64 = *var_jswgd_t_dn4_slot;
        let mut var_jswgd_t_dn5: f64 = *var_jswgd_t_dn5_slot;
        let mut var_jswgd_t_dn6: f64 = *var_jswgd_t_dn6_slot;
        let mut var_jswgd_t_dn7: f64 = *var_jswgd_t_dn7_slot;
        let mut var_jswgd_t_dn8: f64 = *var_jswgd_t_dn8_slot;
        let mut var_jswgd_t_dn9: f64 = *var_jswgd_t_dn9_slot;
        let mut var_jswgd_t_rv: f64 = *var_jswgd_t_rv_slot;
        let mut var_jswgs_t: f64 = *var_jswgs_t_slot;
        let mut var_jswgs_t_dn0: f64 = *var_jswgs_t_dn0_slot;
        let mut var_jswgs_t_dn10: f64 = *var_jswgs_t_dn10_slot;
        let mut var_jswgs_t_dn11: f64 = *var_jswgs_t_dn11_slot;
        let mut var_jswgs_t_dn13: f64 = *var_jswgs_t_dn13_slot;
        let mut var_jswgs_t_dn14: f64 = *var_jswgs_t_dn14_slot;
        let mut var_jswgs_t_dn2: f64 = *var_jswgs_t_dn2_slot;
        let mut var_jswgs_t_dn3: f64 = *var_jswgs_t_dn3_slot;
        let mut var_jswgs_t_dn4: f64 = *var_jswgs_t_dn4_slot;
        let mut var_jswgs_t_dn5: f64 = *var_jswgs_t_dn5_slot;
        let mut var_jswgs_t_dn6: f64 = *var_jswgs_t_dn6_slot;
        let mut var_jswgs_t_dn7: f64 = *var_jswgs_t_dn7_slot;
        let mut var_jswgs_t_dn8: f64 = *var_jswgs_t_dn8_slot;
        let mut var_jswgs_t_dn9: f64 = *var_jswgs_t_dn9_slot;
        let mut var_jswgs_t_rv: f64 = *var_jswgs_t_rv_slot;
        let mut var_jsws_t: f64 = *var_jsws_t_slot;
        let mut var_jsws_t_dn0: f64 = *var_jsws_t_dn0_slot;
        let mut var_jsws_t_dn10: f64 = *var_jsws_t_dn10_slot;
        let mut var_jsws_t_dn11: f64 = *var_jsws_t_dn11_slot;
        let mut var_jsws_t_dn13: f64 = *var_jsws_t_dn13_slot;
        let mut var_jsws_t_dn14: f64 = *var_jsws_t_dn14_slot;
        let mut var_jsws_t_dn2: f64 = *var_jsws_t_dn2_slot;
        let mut var_jsws_t_dn3: f64 = *var_jsws_t_dn3_slot;
        let mut var_jsws_t_dn4: f64 = *var_jsws_t_dn4_slot;
        let mut var_jsws_t_dn5: f64 = *var_jsws_t_dn5_slot;
        let mut var_jsws_t_dn6: f64 = *var_jsws_t_dn6_slot;
        let mut var_jsws_t_dn7: f64 = *var_jsws_t_dn7_slot;
        let mut var_jsws_t_dn8: f64 = *var_jsws_t_dn8_slot;
        let mut var_jsws_t_dn9: f64 = *var_jsws_t_dn9_slot;
        let mut var_jsws_t_rv: f64 = *var_jsws_t_rv_slot;
        let mut var_jtsd_t: f64 = *var_jtsd_t_slot;
        let mut var_jtsd_t_dn4: f64 = *var_jtsd_t_dn4_slot;
        let mut var_jtsd_t_rv: f64 = *var_jtsd_t_rv_slot;
        let mut var_jtss_t: f64 = *var_jtss_t_slot;
        let mut var_jtss_t_dn4: f64 = *var_jtss_t_dn4_slot;
        let mut var_jtss_t_rv: f64 = *var_jtss_t_rv_slot;
        let mut var_jtsswd_t: f64 = *var_jtsswd_t_slot;
        let mut var_jtsswd_t_dn4: f64 = *var_jtsswd_t_dn4_slot;
        let mut var_jtsswd_t_rv: f64 = *var_jtsswd_t_rv_slot;
        let mut var_jtsswgd_t: f64 = *var_jtsswgd_t_slot;
        let mut var_jtsswgd_t_dn4: f64 = *var_jtsswgd_t_dn4_slot;
        let mut var_jtsswgd_t_rv: f64 = *var_jtsswgd_t_rv_slot;
        let mut var_jtsswgs_t: f64 = *var_jtsswgs_t_slot;
        let mut var_jtsswgs_t_dn4: f64 = *var_jtsswgs_t_dn4_slot;
        let mut var_jtsswgs_t_rv: f64 = *var_jtsswgs_t_rv_slot;
        let mut var_jtssws_t: f64 = *var_jtssws_t_slot;
        let mut var_jtssws_t_dn4: f64 = *var_jtssws_t_dn4_slot;
        let mut var_jtssws_t_rv: f64 = *var_jtssws_t_rv_slot;
        let mut var_pbswd_t: f64 = *var_pbswd_t_slot;
        let mut var_pbswd_t_dn4: f64 = *var_pbswd_t_dn4_slot;
        let mut var_pbswd_t_rv: f64 = *var_pbswd_t_rv_slot;
        let mut var_pbswgd_t: f64 = *var_pbswgd_t_slot;
        let mut var_pbswgd_t_dn4: f64 = *var_pbswgd_t_dn4_slot;
        let mut var_pbswgd_t_rv: f64 = *var_pbswgd_t_rv_slot;
        let mut var_pbswgs_t: f64 = *var_pbswgs_t_slot;
        let mut var_pbswgs_t_dn4: f64 = *var_pbswgs_t_dn4_slot;
        let mut var_pbswgs_t_rv: f64 = *var_pbswgs_t_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;

        let (assign18450_e34441, assign18450_e34441_d_n4,) = {
    if (var_guard336 != 0.0) {
        let assign18450_e34369: f64 = (p.p1725 * var_deltemp);
        let assign18450_e34370: f64 = (p.p1593 - assign18450_e34369);
        let assign18450_e34372: f64 = (assign18450_e34370 - 0.01);
        let assign18450_e34374: f64 = (-10000.0);
        let assign18450_e34376: f64 = (assign18450_e34374 * 0.001);
        let (assign18450_e34437, assign18450_e34437_d_n4,) = {
            if (!(assign18450_e34372 < assign18450_e34376)) {
                let assign18450_e34383: f64 = (p.p1725 * var_deltemp);
                let assign18450_e34384: f64 = (p.p1593 - assign18450_e34383);
                let assign18450_e34386: f64 = (assign18450_e34384 - 0.01);
                let assign18450_e34390: f64 = (p.p1725 * var_deltemp);
                let assign18450_e34391: f64 = (p.p1593 - assign18450_e34390);
                let assign18450_e34393: f64 = (assign18450_e34391 - 0.01);
                let assign18450_e34397: f64 = (p.p1725 * var_deltemp);
                let assign18450_e34398: f64 = (p.p1593 - assign18450_e34397);
                let assign18450_e34400: f64 = (assign18450_e34398 - 0.01);
                let assign18450_e34401: f64 = (assign18450_e34393 * assign18450_e34400);
                let assign18450_e34404: f64 = (4.0 * 0.001);
                let assign18450_e34406: f64 = (assign18450_e34404 * 0.001);
                let assign18450_e34407: f64 = (assign18450_e34401 + assign18450_e34406);
                let assign18450_e34408: f64 = (assign18450_e34407).sqrt();
                let assign18450_e34409: f64 = (assign18450_e34386 + assign18450_e34408);
                let assign18450_e34410: f64 = (0.5 * assign18450_e34409);
                (assign18450_e34410, (0.5 * ((-(p.p1725 * var_deltemp_dn4)) + ((((-(p.p1725 * var_deltemp_dn4)) * assign18450_e34400) + (assign18450_e34393 * (-(p.p1725 * var_deltemp_dn4)))) / (2.0 * assign18450_e34408)))),)
            } else {
                let assign18450_e34414: f64 = (p.p1725 * var_deltemp);
                let assign18450_e34415: f64 = (p.p1593 - assign18450_e34414);
                let assign18450_e34417: f64 = (assign18450_e34415 - 0.01);
                let assign18450_e34419: f64 = (-10000.0);
                let assign18450_e34421: f64 = (assign18450_e34419 * 0.001);
                let (assign18450_e34436, assign18450_e34436_d_n4,) = {
                    if (assign18450_e34417 < assign18450_e34421) {
                        let assign18450_e34424: f64 = (-0.001);
                        let assign18450_e34426: f64 = (assign18450_e34424 * 0.001);
                        let assign18450_e34430: f64 = (p.p1725 * var_deltemp);
                        let assign18450_e34431: f64 = (p.p1593 - assign18450_e34430);
                        let assign18450_e34433: f64 = (assign18450_e34431 - 0.01);
                        let assign18450_e34434: f64 = (assign18450_e34426 / assign18450_e34433);
                        (assign18450_e34434, (-((assign18450_e34426 * (-(p.p1725 * var_deltemp_dn4))) / (assign18450_e34433 * assign18450_e34433))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18450_e34436, assign18450_e34436_d_n4,)
            }
        };
        let assign18450_e34439: f64 = (assign18450_e34437 + 0.01);
        (assign18450_e34439, assign18450_e34437_d_n4,)
    } else {
        (var_pbswd_t, var_pbswd_t_dn4,)
    }
};
        var_pbswd_t = assign18450_e34441;
        var_pbswd_t_dn4 = assign18450_e34441_d_n4;
        var_pbswd_t_rv = 0.0;

        let (assign18460_e34518, assign18460_e34518_d_n4,) = {
    if (var_guard336 != 0.0) {
        let assign18460_e34446: f64 = (p.p1726 * var_deltemp);
        let assign18460_e34447: f64 = (p.p1594 - assign18460_e34446);
        let assign18460_e34449: f64 = (assign18460_e34447 - 0.01);
        let assign18460_e34451: f64 = (-10000.0);
        let assign18460_e34453: f64 = (assign18460_e34451 * 0.001);
        let (assign18460_e34514, assign18460_e34514_d_n4,) = {
            if (!(assign18460_e34449 < assign18460_e34453)) {
                let assign18460_e34460: f64 = (p.p1726 * var_deltemp);
                let assign18460_e34461: f64 = (p.p1594 - assign18460_e34460);
                let assign18460_e34463: f64 = (assign18460_e34461 - 0.01);
                let assign18460_e34467: f64 = (p.p1726 * var_deltemp);
                let assign18460_e34468: f64 = (p.p1594 - assign18460_e34467);
                let assign18460_e34470: f64 = (assign18460_e34468 - 0.01);
                let assign18460_e34474: f64 = (p.p1726 * var_deltemp);
                let assign18460_e34475: f64 = (p.p1594 - assign18460_e34474);
                let assign18460_e34477: f64 = (assign18460_e34475 - 0.01);
                let assign18460_e34478: f64 = (assign18460_e34470 * assign18460_e34477);
                let assign18460_e34481: f64 = (4.0 * 0.001);
                let assign18460_e34483: f64 = (assign18460_e34481 * 0.001);
                let assign18460_e34484: f64 = (assign18460_e34478 + assign18460_e34483);
                let assign18460_e34485: f64 = (assign18460_e34484).sqrt();
                let assign18460_e34486: f64 = (assign18460_e34463 + assign18460_e34485);
                let assign18460_e34487: f64 = (0.5 * assign18460_e34486);
                (assign18460_e34487, (0.5 * ((-(p.p1726 * var_deltemp_dn4)) + ((((-(p.p1726 * var_deltemp_dn4)) * assign18460_e34477) + (assign18460_e34470 * (-(p.p1726 * var_deltemp_dn4)))) / (2.0 * assign18460_e34485)))),)
            } else {
                let assign18460_e34491: f64 = (p.p1726 * var_deltemp);
                let assign18460_e34492: f64 = (p.p1594 - assign18460_e34491);
                let assign18460_e34494: f64 = (assign18460_e34492 - 0.01);
                let assign18460_e34496: f64 = (-10000.0);
                let assign18460_e34498: f64 = (assign18460_e34496 * 0.001);
                let (assign18460_e34513, assign18460_e34513_d_n4,) = {
                    if (assign18460_e34494 < assign18460_e34498) {
                        let assign18460_e34501: f64 = (-0.001);
                        let assign18460_e34503: f64 = (assign18460_e34501 * 0.001);
                        let assign18460_e34507: f64 = (p.p1726 * var_deltemp);
                        let assign18460_e34508: f64 = (p.p1594 - assign18460_e34507);
                        let assign18460_e34510: f64 = (assign18460_e34508 - 0.01);
                        let assign18460_e34511: f64 = (assign18460_e34503 / assign18460_e34510);
                        (assign18460_e34511, (-((assign18460_e34503 * (-(p.p1726 * var_deltemp_dn4))) / (assign18460_e34510 * assign18460_e34510))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18460_e34513, assign18460_e34513_d_n4,)
            }
        };
        let assign18460_e34516: f64 = (assign18460_e34514 + 0.01);
        (assign18460_e34516, assign18460_e34514_d_n4,)
    } else {
        (var_pbswgs_t, var_pbswgs_t_dn4,)
    }
};
        var_pbswgs_t = assign18460_e34518;
        var_pbswgs_t_dn4 = assign18460_e34518_d_n4;
        var_pbswgs_t_rv = 0.0;

        let (assign18470_e34595, assign18470_e34595_d_n4,) = {
    if (var_guard336 != 0.0) {
        let assign18470_e34523: f64 = (p.p1726 * var_deltemp);
        let assign18470_e34524: f64 = (p.p1595 - assign18470_e34523);
        let assign18470_e34526: f64 = (assign18470_e34524 - 0.01);
        let assign18470_e34528: f64 = (-10000.0);
        let assign18470_e34530: f64 = (assign18470_e34528 * 0.001);
        let (assign18470_e34591, assign18470_e34591_d_n4,) = {
            if (!(assign18470_e34526 < assign18470_e34530)) {
                let assign18470_e34537: f64 = (p.p1726 * var_deltemp);
                let assign18470_e34538: f64 = (p.p1595 - assign18470_e34537);
                let assign18470_e34540: f64 = (assign18470_e34538 - 0.01);
                let assign18470_e34544: f64 = (p.p1726 * var_deltemp);
                let assign18470_e34545: f64 = (p.p1595 - assign18470_e34544);
                let assign18470_e34547: f64 = (assign18470_e34545 - 0.01);
                let assign18470_e34551: f64 = (p.p1726 * var_deltemp);
                let assign18470_e34552: f64 = (p.p1595 - assign18470_e34551);
                let assign18470_e34554: f64 = (assign18470_e34552 - 0.01);
                let assign18470_e34555: f64 = (assign18470_e34547 * assign18470_e34554);
                let assign18470_e34558: f64 = (4.0 * 0.001);
                let assign18470_e34560: f64 = (assign18470_e34558 * 0.001);
                let assign18470_e34561: f64 = (assign18470_e34555 + assign18470_e34560);
                let assign18470_e34562: f64 = (assign18470_e34561).sqrt();
                let assign18470_e34563: f64 = (assign18470_e34540 + assign18470_e34562);
                let assign18470_e34564: f64 = (0.5 * assign18470_e34563);
                (assign18470_e34564, (0.5 * ((-(p.p1726 * var_deltemp_dn4)) + ((((-(p.p1726 * var_deltemp_dn4)) * assign18470_e34554) + (assign18470_e34547 * (-(p.p1726 * var_deltemp_dn4)))) / (2.0 * assign18470_e34562)))),)
            } else {
                let assign18470_e34568: f64 = (p.p1726 * var_deltemp);
                let assign18470_e34569: f64 = (p.p1595 - assign18470_e34568);
                let assign18470_e34571: f64 = (assign18470_e34569 - 0.01);
                let assign18470_e34573: f64 = (-10000.0);
                let assign18470_e34575: f64 = (assign18470_e34573 * 0.001);
                let (assign18470_e34590, assign18470_e34590_d_n4,) = {
                    if (assign18470_e34571 < assign18470_e34575) {
                        let assign18470_e34578: f64 = (-0.001);
                        let assign18470_e34580: f64 = (assign18470_e34578 * 0.001);
                        let assign18470_e34584: f64 = (p.p1726 * var_deltemp);
                        let assign18470_e34585: f64 = (p.p1595 - assign18470_e34584);
                        let assign18470_e34587: f64 = (assign18470_e34585 - 0.01);
                        let assign18470_e34588: f64 = (assign18470_e34580 / assign18470_e34587);
                        (assign18470_e34588, (-((assign18470_e34580 * (-(p.p1726 * var_deltemp_dn4))) / (assign18470_e34587 * assign18470_e34587))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18470_e34590, assign18470_e34590_d_n4,)
            }
        };
        let assign18470_e34593: f64 = (assign18470_e34591 + 0.01);
        (assign18470_e34593, assign18470_e34591_d_n4,)
    } else {
        (var_pbswgd_t, var_pbswgd_t_dn4,)
    }
};
        var_pbswgd_t = assign18470_e34595;
        var_pbswgd_t_dn4 = assign18470_e34595_d_n4;
        var_pbswgd_t_rv = 0.0;

        let (assign18480_e34605, assign18480_e34605_d_n0, assign18480_e34605_d_n2, assign18480_e34605_d_n3, assign18480_e34605_d_n4, assign18480_e34605_d_n5, assign18480_e34605_d_n6, assign18480_e34605_d_n7, assign18480_e34605_d_n8, assign18480_e34605_d_n9, assign18480_e34605_d_n10, assign18480_e34605_d_n11, assign18480_e34605_d_n13, assign18480_e34605_d_n14,) = {
    if (var_guard336 != 0.0) {
        let assign18480_e34599: f64 = (var_eg0 / var_vtm0);
        let assign18480_e34602: f64 = (var_eg / var_vtm);
        let assign18480_e34603: f64 = (assign18480_e34599 - assign18480_e34602);
        (assign18480_e34603, 0.0, 0.0, 0.0, (-(((var_eg_dn4 * var_vtm) - (var_eg * var_vtm_dn4)) / (var_vtm * var_vtm))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn13, var_t0_dn14,)
    }
};
        var_t0 = assign18480_e34605;
        var_t0_dn0 = assign18480_e34605_d_n0;
        var_t0_dn2 = assign18480_e34605_d_n2;
        var_t0_dn3 = assign18480_e34605_d_n3;
        var_t0_dn4 = assign18480_e34605_d_n4;
        var_t0_dn5 = assign18480_e34605_d_n5;
        var_t0_dn6 = assign18480_e34605_d_n6;
        var_t0_dn7 = assign18480_e34605_d_n7;
        var_t0_dn8 = assign18480_e34605_d_n8;
        var_t0_dn9 = assign18480_e34605_d_n9;
        var_t0_dn10 = assign18480_e34605_d_n10;
        var_t0_dn11 = assign18480_e34605_d_n11;
        var_t0_dn13 = assign18480_e34605_d_n13;
        var_t0_dn14 = assign18480_e34605_d_n14;
        var_t0_rv = 0.0;

        let (assign18490_e34616, assign18490_e34616_d_n0, assign18490_e34616_d_n2, assign18490_e34616_d_n3, assign18490_e34616_d_n4, assign18490_e34616_d_n5, assign18490_e34616_d_n6, assign18490_e34616_d_n7, assign18490_e34616_d_n8, assign18490_e34616_d_n9, assign18490_e34616_d_n10, assign18490_e34616_d_n11, assign18490_e34616_d_n13, assign18490_e34616_d_n14,) = {
    if (var_guard336 != 0.0) {
        let assign18490_e34610: f64 = (p.p1727 * var_trat_ln);
        let assign18490_e34611: f64 = (var_t0 + assign18490_e34610);
        let assign18490_e34613: f64 = (assign18490_e34611 / p.p1620);
        let assign18490_e34614: f64 = { let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign18490_e34614, ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn0 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn2 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn3 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_t0_dn4 + (p.p1727 * var_trat_ln_dn4)) / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn5 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn6 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn7 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn8 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn9 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn10 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn11 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn13 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn14 / p.p1620)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn13, var_t3_dn14,)
    }
};
        var_t3 = assign18490_e34616;
        var_t3_dn0 = assign18490_e34616_d_n0;
        var_t3_dn2 = assign18490_e34616_d_n2;
        var_t3_dn3 = assign18490_e34616_d_n3;
        var_t3_dn4 = assign18490_e34616_d_n4;
        var_t3_dn5 = assign18490_e34616_d_n5;
        var_t3_dn6 = assign18490_e34616_d_n6;
        var_t3_dn7 = assign18490_e34616_d_n7;
        var_t3_dn8 = assign18490_e34616_d_n8;
        var_t3_dn9 = assign18490_e34616_d_n9;
        var_t3_dn10 = assign18490_e34616_d_n10;
        var_t3_dn11 = assign18490_e34616_d_n11;
        var_t3_dn13 = assign18490_e34616_d_n13;
        var_t3_dn14 = assign18490_e34616_d_n14;
        var_t3_rv = 0.0;

        let (assign18500_e34622, assign18500_e34622_d_n0, assign18500_e34622_d_n2, assign18500_e34622_d_n3, assign18500_e34622_d_n4, assign18500_e34622_d_n5, assign18500_e34622_d_n6, assign18500_e34622_d_n7, assign18500_e34622_d_n8, assign18500_e34622_d_n9, assign18500_e34622_d_n10, assign18500_e34622_d_n11, assign18500_e34622_d_n13, assign18500_e34622_d_n14,) = {
    if (var_guard336 != 0.0) {
        let assign18500_e34620: f64 = (p.p1614 * var_t3);
        (assign18500_e34620, (p.p1614 * var_t3_dn0), (p.p1614 * var_t3_dn2), (p.p1614 * var_t3_dn3), (p.p1614 * var_t3_dn4), (p.p1614 * var_t3_dn5), (p.p1614 * var_t3_dn6), (p.p1614 * var_t3_dn7), (p.p1614 * var_t3_dn8), (p.p1614 * var_t3_dn9), (p.p1614 * var_t3_dn10), (p.p1614 * var_t3_dn11), (p.p1614 * var_t3_dn13), (p.p1614 * var_t3_dn14),)
    } else {
        (var_jss_t, var_jss_t_dn0, var_jss_t_dn2, var_jss_t_dn3, var_jss_t_dn4, var_jss_t_dn5, var_jss_t_dn6, var_jss_t_dn7, var_jss_t_dn8, var_jss_t_dn9, var_jss_t_dn10, var_jss_t_dn11, var_jss_t_dn13, var_jss_t_dn14,)
    }
};
        var_jss_t = assign18500_e34622;
        var_jss_t_dn0 = assign18500_e34622_d_n0;
        var_jss_t_dn2 = assign18500_e34622_d_n2;
        var_jss_t_dn3 = assign18500_e34622_d_n3;
        var_jss_t_dn4 = assign18500_e34622_d_n4;
        var_jss_t_dn5 = assign18500_e34622_d_n5;
        var_jss_t_dn6 = assign18500_e34622_d_n6;
        var_jss_t_dn7 = assign18500_e34622_d_n7;
        var_jss_t_dn8 = assign18500_e34622_d_n8;
        var_jss_t_dn9 = assign18500_e34622_d_n9;
        var_jss_t_dn10 = assign18500_e34622_d_n10;
        var_jss_t_dn11 = assign18500_e34622_d_n11;
        var_jss_t_dn13 = assign18500_e34622_d_n13;
        var_jss_t_dn14 = assign18500_e34622_d_n14;
        var_jss_t_rv = 0.0;

        let (assign18510_e34628, assign18510_e34628_d_n0, assign18510_e34628_d_n2, assign18510_e34628_d_n3, assign18510_e34628_d_n4, assign18510_e34628_d_n5, assign18510_e34628_d_n6, assign18510_e34628_d_n7, assign18510_e34628_d_n8, assign18510_e34628_d_n9, assign18510_e34628_d_n10, assign18510_e34628_d_n11, assign18510_e34628_d_n13, assign18510_e34628_d_n14,) = {
    if (var_guard336 != 0.0) {
        let assign18510_e34626: f64 = (p.p1616 * var_t3);
        (assign18510_e34626, (p.p1616 * var_t3_dn0), (p.p1616 * var_t3_dn2), (p.p1616 * var_t3_dn3), (p.p1616 * var_t3_dn4), (p.p1616 * var_t3_dn5), (p.p1616 * var_t3_dn6), (p.p1616 * var_t3_dn7), (p.p1616 * var_t3_dn8), (p.p1616 * var_t3_dn9), (p.p1616 * var_t3_dn10), (p.p1616 * var_t3_dn11), (p.p1616 * var_t3_dn13), (p.p1616 * var_t3_dn14),)
    } else {
        (var_jsws_t, var_jsws_t_dn0, var_jsws_t_dn2, var_jsws_t_dn3, var_jsws_t_dn4, var_jsws_t_dn5, var_jsws_t_dn6, var_jsws_t_dn7, var_jsws_t_dn8, var_jsws_t_dn9, var_jsws_t_dn10, var_jsws_t_dn11, var_jsws_t_dn13, var_jsws_t_dn14,)
    }
};
        var_jsws_t = assign18510_e34628;
        var_jsws_t_dn0 = assign18510_e34628_d_n0;
        var_jsws_t_dn2 = assign18510_e34628_d_n2;
        var_jsws_t_dn3 = assign18510_e34628_d_n3;
        var_jsws_t_dn4 = assign18510_e34628_d_n4;
        var_jsws_t_dn5 = assign18510_e34628_d_n5;
        var_jsws_t_dn6 = assign18510_e34628_d_n6;
        var_jsws_t_dn7 = assign18510_e34628_d_n7;
        var_jsws_t_dn8 = assign18510_e34628_d_n8;
        var_jsws_t_dn9 = assign18510_e34628_d_n9;
        var_jsws_t_dn10 = assign18510_e34628_d_n10;
        var_jsws_t_dn11 = assign18510_e34628_d_n11;
        var_jsws_t_dn13 = assign18510_e34628_d_n13;
        var_jsws_t_dn14 = assign18510_e34628_d_n14;
        var_jsws_t_rv = 0.0;

        let (assign18520_e34634, assign18520_e34634_d_n0, assign18520_e34634_d_n2, assign18520_e34634_d_n3, assign18520_e34634_d_n4, assign18520_e34634_d_n5, assign18520_e34634_d_n6, assign18520_e34634_d_n7, assign18520_e34634_d_n8, assign18520_e34634_d_n9, assign18520_e34634_d_n10, assign18520_e34634_d_n11, assign18520_e34634_d_n13, assign18520_e34634_d_n14,) = {
    if (var_guard336 != 0.0) {
        let assign18520_e34632: f64 = (p.p1618 * var_t3);
        (assign18520_e34632, (p.p1618 * var_t3_dn0), (p.p1618 * var_t3_dn2), (p.p1618 * var_t3_dn3), (p.p1618 * var_t3_dn4), (p.p1618 * var_t3_dn5), (p.p1618 * var_t3_dn6), (p.p1618 * var_t3_dn7), (p.p1618 * var_t3_dn8), (p.p1618 * var_t3_dn9), (p.p1618 * var_t3_dn10), (p.p1618 * var_t3_dn11), (p.p1618 * var_t3_dn13), (p.p1618 * var_t3_dn14),)
    } else {
        (var_jswgs_t, var_jswgs_t_dn0, var_jswgs_t_dn2, var_jswgs_t_dn3, var_jswgs_t_dn4, var_jswgs_t_dn5, var_jswgs_t_dn6, var_jswgs_t_dn7, var_jswgs_t_dn8, var_jswgs_t_dn9, var_jswgs_t_dn10, var_jswgs_t_dn11, var_jswgs_t_dn13, var_jswgs_t_dn14,)
    }
};
        var_jswgs_t = assign18520_e34634;
        var_jswgs_t_dn0 = assign18520_e34634_d_n0;
        var_jswgs_t_dn2 = assign18520_e34634_d_n2;
        var_jswgs_t_dn3 = assign18520_e34634_d_n3;
        var_jswgs_t_dn4 = assign18520_e34634_d_n4;
        var_jswgs_t_dn5 = assign18520_e34634_d_n5;
        var_jswgs_t_dn6 = assign18520_e34634_d_n6;
        var_jswgs_t_dn7 = assign18520_e34634_d_n7;
        var_jswgs_t_dn8 = assign18520_e34634_d_n8;
        var_jswgs_t_dn9 = assign18520_e34634_d_n9;
        var_jswgs_t_dn10 = assign18520_e34634_d_n10;
        var_jswgs_t_dn11 = assign18520_e34634_d_n11;
        var_jswgs_t_dn13 = assign18520_e34634_d_n13;
        var_jswgs_t_dn14 = assign18520_e34634_d_n14;
        var_jswgs_t_rv = 0.0;

        let (assign18530_e34645, assign18530_e34645_d_n0, assign18530_e34645_d_n2, assign18530_e34645_d_n3, assign18530_e34645_d_n4, assign18530_e34645_d_n5, assign18530_e34645_d_n6, assign18530_e34645_d_n7, assign18530_e34645_d_n8, assign18530_e34645_d_n9, assign18530_e34645_d_n10, assign18530_e34645_d_n11, assign18530_e34645_d_n13, assign18530_e34645_d_n14,) = {
    if (var_guard336 != 0.0) {
        let assign18530_e34639: f64 = (p.p1728 * var_trat_ln);
        let assign18530_e34640: f64 = (var_t0 + assign18530_e34639);
        let assign18530_e34642: f64 = (assign18530_e34640 / p.p1621);
        let assign18530_e34643: f64 = { let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign18530_e34643, ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn0 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn2 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn3 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((var_t0_dn4 + (p.p1728 * var_trat_ln_dn4)) / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn5 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn6 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn7 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn8 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn9 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn10 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn11 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn13 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn14 / p.p1621)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn13, var_t3_dn14,)
    }
};
        var_t3 = assign18530_e34645;
        var_t3_dn0 = assign18530_e34645_d_n0;
        var_t3_dn2 = assign18530_e34645_d_n2;
        var_t3_dn3 = assign18530_e34645_d_n3;
        var_t3_dn4 = assign18530_e34645_d_n4;
        var_t3_dn5 = assign18530_e34645_d_n5;
        var_t3_dn6 = assign18530_e34645_d_n6;
        var_t3_dn7 = assign18530_e34645_d_n7;
        var_t3_dn8 = assign18530_e34645_d_n8;
        var_t3_dn9 = assign18530_e34645_d_n9;
        var_t3_dn10 = assign18530_e34645_d_n10;
        var_t3_dn11 = assign18530_e34645_d_n11;
        var_t3_dn13 = assign18530_e34645_d_n13;
        var_t3_dn14 = assign18530_e34645_d_n14;
        var_t3_rv = 0.0;

        let (assign18540_e34651, assign18540_e34651_d_n0, assign18540_e34651_d_n2, assign18540_e34651_d_n3, assign18540_e34651_d_n4, assign18540_e34651_d_n5, assign18540_e34651_d_n6, assign18540_e34651_d_n7, assign18540_e34651_d_n8, assign18540_e34651_d_n9, assign18540_e34651_d_n10, assign18540_e34651_d_n11, assign18540_e34651_d_n13, assign18540_e34651_d_n14,) = {
    if (var_guard336 != 0.0) {
        let assign18540_e34649: f64 = (p.p1615 * var_t3);
        (assign18540_e34649, (p.p1615 * var_t3_dn0), (p.p1615 * var_t3_dn2), (p.p1615 * var_t3_dn3), (p.p1615 * var_t3_dn4), (p.p1615 * var_t3_dn5), (p.p1615 * var_t3_dn6), (p.p1615 * var_t3_dn7), (p.p1615 * var_t3_dn8), (p.p1615 * var_t3_dn9), (p.p1615 * var_t3_dn10), (p.p1615 * var_t3_dn11), (p.p1615 * var_t3_dn13), (p.p1615 * var_t3_dn14),)
    } else {
        (var_jsd_t, var_jsd_t_dn0, var_jsd_t_dn2, var_jsd_t_dn3, var_jsd_t_dn4, var_jsd_t_dn5, var_jsd_t_dn6, var_jsd_t_dn7, var_jsd_t_dn8, var_jsd_t_dn9, var_jsd_t_dn10, var_jsd_t_dn11, var_jsd_t_dn13, var_jsd_t_dn14,)
    }
};
        var_jsd_t = assign18540_e34651;
        var_jsd_t_dn0 = assign18540_e34651_d_n0;
        var_jsd_t_dn2 = assign18540_e34651_d_n2;
        var_jsd_t_dn3 = assign18540_e34651_d_n3;
        var_jsd_t_dn4 = assign18540_e34651_d_n4;
        var_jsd_t_dn5 = assign18540_e34651_d_n5;
        var_jsd_t_dn6 = assign18540_e34651_d_n6;
        var_jsd_t_dn7 = assign18540_e34651_d_n7;
        var_jsd_t_dn8 = assign18540_e34651_d_n8;
        var_jsd_t_dn9 = assign18540_e34651_d_n9;
        var_jsd_t_dn10 = assign18540_e34651_d_n10;
        var_jsd_t_dn11 = assign18540_e34651_d_n11;
        var_jsd_t_dn13 = assign18540_e34651_d_n13;
        var_jsd_t_dn14 = assign18540_e34651_d_n14;
        var_jsd_t_rv = 0.0;

        let (assign18550_e34657, assign18550_e34657_d_n0, assign18550_e34657_d_n2, assign18550_e34657_d_n3, assign18550_e34657_d_n4, assign18550_e34657_d_n5, assign18550_e34657_d_n6, assign18550_e34657_d_n7, assign18550_e34657_d_n8, assign18550_e34657_d_n9, assign18550_e34657_d_n10, assign18550_e34657_d_n11, assign18550_e34657_d_n13, assign18550_e34657_d_n14,) = {
    if (var_guard336 != 0.0) {
        let assign18550_e34655: f64 = (p.p1617 * var_t3);
        (assign18550_e34655, (p.p1617 * var_t3_dn0), (p.p1617 * var_t3_dn2), (p.p1617 * var_t3_dn3), (p.p1617 * var_t3_dn4), (p.p1617 * var_t3_dn5), (p.p1617 * var_t3_dn6), (p.p1617 * var_t3_dn7), (p.p1617 * var_t3_dn8), (p.p1617 * var_t3_dn9), (p.p1617 * var_t3_dn10), (p.p1617 * var_t3_dn11), (p.p1617 * var_t3_dn13), (p.p1617 * var_t3_dn14),)
    } else {
        (var_jswd_t, var_jswd_t_dn0, var_jswd_t_dn2, var_jswd_t_dn3, var_jswd_t_dn4, var_jswd_t_dn5, var_jswd_t_dn6, var_jswd_t_dn7, var_jswd_t_dn8, var_jswd_t_dn9, var_jswd_t_dn10, var_jswd_t_dn11, var_jswd_t_dn13, var_jswd_t_dn14,)
    }
};
        var_jswd_t = assign18550_e34657;
        var_jswd_t_dn0 = assign18550_e34657_d_n0;
        var_jswd_t_dn2 = assign18550_e34657_d_n2;
        var_jswd_t_dn3 = assign18550_e34657_d_n3;
        var_jswd_t_dn4 = assign18550_e34657_d_n4;
        var_jswd_t_dn5 = assign18550_e34657_d_n5;
        var_jswd_t_dn6 = assign18550_e34657_d_n6;
        var_jswd_t_dn7 = assign18550_e34657_d_n7;
        var_jswd_t_dn8 = assign18550_e34657_d_n8;
        var_jswd_t_dn9 = assign18550_e34657_d_n9;
        var_jswd_t_dn10 = assign18550_e34657_d_n10;
        var_jswd_t_dn11 = assign18550_e34657_d_n11;
        var_jswd_t_dn13 = assign18550_e34657_d_n13;
        var_jswd_t_dn14 = assign18550_e34657_d_n14;
        var_jswd_t_rv = 0.0;

        let (assign18560_e34663, assign18560_e34663_d_n0, assign18560_e34663_d_n2, assign18560_e34663_d_n3, assign18560_e34663_d_n4, assign18560_e34663_d_n5, assign18560_e34663_d_n6, assign18560_e34663_d_n7, assign18560_e34663_d_n8, assign18560_e34663_d_n9, assign18560_e34663_d_n10, assign18560_e34663_d_n11, assign18560_e34663_d_n13, assign18560_e34663_d_n14,) = {
    if (var_guard336 != 0.0) {
        let assign18560_e34661: f64 = (p.p1619 * var_t3);
        (assign18560_e34661, (p.p1619 * var_t3_dn0), (p.p1619 * var_t3_dn2), (p.p1619 * var_t3_dn3), (p.p1619 * var_t3_dn4), (p.p1619 * var_t3_dn5), (p.p1619 * var_t3_dn6), (p.p1619 * var_t3_dn7), (p.p1619 * var_t3_dn8), (p.p1619 * var_t3_dn9), (p.p1619 * var_t3_dn10), (p.p1619 * var_t3_dn11), (p.p1619 * var_t3_dn13), (p.p1619 * var_t3_dn14),)
    } else {
        (var_jswgd_t, var_jswgd_t_dn0, var_jswgd_t_dn2, var_jswgd_t_dn3, var_jswgd_t_dn4, var_jswgd_t_dn5, var_jswgd_t_dn6, var_jswgd_t_dn7, var_jswgd_t_dn8, var_jswgd_t_dn9, var_jswgd_t_dn10, var_jswgd_t_dn11, var_jswgd_t_dn13, var_jswgd_t_dn14,)
    }
};
        var_jswgd_t = assign18560_e34663;
        var_jswgd_t_dn0 = assign18560_e34663_d_n0;
        var_jswgd_t_dn2 = assign18560_e34663_d_n2;
        var_jswgd_t_dn3 = assign18560_e34663_d_n3;
        var_jswgd_t_dn4 = assign18560_e34663_d_n4;
        var_jswgd_t_dn5 = assign18560_e34663_d_n5;
        var_jswgd_t_dn6 = assign18560_e34663_d_n6;
        var_jswgd_t_dn7 = assign18560_e34663_d_n7;
        var_jswgd_t_dn8 = assign18560_e34663_d_n8;
        var_jswgd_t_dn9 = assign18560_e34663_d_n9;
        var_jswgd_t_dn10 = assign18560_e34663_d_n10;
        var_jswgd_t_dn11 = assign18560_e34663_d_n11;
        var_jswgd_t_dn13 = assign18560_e34663_d_n13;
        var_jswgd_t_dn14 = assign18560_e34663_d_n14;
        var_jswgd_t_rv = 0.0;

        let (assign18570_e34676, assign18570_e34676_d_n4,) = {
    if (var_guard336 != 0.0) {
        let assign18570_e34668: f64 = (var_eg0 * p.p1729);
        let assign18570_e34670: f64 = (assign18570_e34668 * var_tratio_m1);
        let assign18570_e34672: f64 = (assign18570_e34670 / var_vtm);
        let assign18570_e34673: f64 = { let limited_exp_arg = assign18570_e34672; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18570_e34674: f64 = (p.p1630 * assign18570_e34673);
        (assign18570_e34674, (p.p1630 * ({ let limited_exp_arg = assign18570_e34672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign18570_e34668 * var_tratio_m1_dn4) * var_vtm) - (assign18570_e34670 * var_vtm_dn4)) / (var_vtm * var_vtm)))),)
    } else {
        (var_jtss_t, var_jtss_t_dn4,)
    }
};
        var_jtss_t = assign18570_e34676;
        var_jtss_t_dn4 = assign18570_e34676_d_n4;
        var_jtss_t_rv = 0.0;

        let (assign18580_e34689, assign18580_e34689_d_n4,) = {
    if (var_guard336 != 0.0) {
        let assign18580_e34681: f64 = (var_eg0 * p.p1730);
        let assign18580_e34683: f64 = (assign18580_e34681 * var_tratio_m1);
        let assign18580_e34685: f64 = (assign18580_e34683 / var_vtm);
        let assign18580_e34686: f64 = { let limited_exp_arg = assign18580_e34685; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18580_e34687: f64 = (p.p1631 * assign18580_e34686);
        (assign18580_e34687, (p.p1631 * ({ let limited_exp_arg = assign18580_e34685; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign18580_e34681 * var_tratio_m1_dn4) * var_vtm) - (assign18580_e34683 * var_vtm_dn4)) / (var_vtm * var_vtm)))),)
    } else {
        (var_jtsd_t, var_jtsd_t_dn4,)
    }
};
        var_jtsd_t = assign18580_e34689;
        var_jtsd_t_dn4 = assign18580_e34689_d_n4;
        var_jtsd_t_rv = 0.0;

        let (assign18590_e34702, assign18590_e34702_d_n4,) = {
    if (var_guard336 != 0.0) {
        let assign18590_e34694: f64 = (var_eg0 * p.p1731);
        let assign18590_e34696: f64 = (assign18590_e34694 * var_tratio_m1);
        let assign18590_e34698: f64 = (assign18590_e34696 / var_vtm);
        let assign18590_e34699: f64 = { let limited_exp_arg = assign18590_e34698; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18590_e34700: f64 = (p.p1632 * assign18590_e34699);
        (assign18590_e34700, (p.p1632 * ({ let limited_exp_arg = assign18590_e34698; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign18590_e34694 * var_tratio_m1_dn4) * var_vtm) - (assign18590_e34696 * var_vtm_dn4)) / (var_vtm * var_vtm)))),)
    } else {
        (var_jtssws_t, var_jtssws_t_dn4,)
    }
};
        var_jtssws_t = assign18590_e34702;
        var_jtssws_t_dn4 = assign18590_e34702_d_n4;
        var_jtssws_t_rv = 0.0;

        let (assign18600_e34715, assign18600_e34715_d_n4,) = {
    if (var_guard336 != 0.0) {
        let assign18600_e34707: f64 = (var_eg0 * p.p1732);
        let assign18600_e34709: f64 = (assign18600_e34707 * var_tratio_m1);
        let assign18600_e34711: f64 = (assign18600_e34709 / var_vtm);
        let assign18600_e34712: f64 = { let limited_exp_arg = assign18600_e34711; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18600_e34713: f64 = (p.p1633 * assign18600_e34712);
        (assign18600_e34713, (p.p1633 * ({ let limited_exp_arg = assign18600_e34711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign18600_e34707 * var_tratio_m1_dn4) * var_vtm) - (assign18600_e34709 * var_vtm_dn4)) / (var_vtm * var_vtm)))),)
    } else {
        (var_jtsswd_t, var_jtsswd_t_dn4,)
    }
};
        var_jtsswd_t = assign18600_e34715;
        var_jtsswd_t_dn4 = assign18600_e34715_d_n4;
        var_jtsswd_t_rv = 0.0;

        let (assign18610_e34735, assign18610_e34735_d_n4,) = {
    if (var_guard336 != 0.0) {
        let assign18610_e34720: f64 = (p.p1636 / var_weff0);
        let assign18610_e34721: f64 = (assign18610_e34720).sqrt();
        let assign18610_e34723: f64 = (assign18610_e34721 + 1.0);
        let assign18610_e34724: f64 = (p.p1634 * assign18610_e34723);
        let assign18610_e34727: f64 = (var_eg0 * p.p1733);
        let assign18610_e34729: f64 = (assign18610_e34727 * var_tratio_m1);
        let assign18610_e34731: f64 = (assign18610_e34729 / var_vtm);
        let assign18610_e34732: f64 = { let limited_exp_arg = assign18610_e34731; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18610_e34733: f64 = (assign18610_e34724 * assign18610_e34732);
        (assign18610_e34733, (assign18610_e34724 * ({ let limited_exp_arg = assign18610_e34731; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign18610_e34727 * var_tratio_m1_dn4) * var_vtm) - (assign18610_e34729 * var_vtm_dn4)) / (var_vtm * var_vtm)))),)
    } else {
        (var_jtsswgs_t, var_jtsswgs_t_dn4,)
    }
};
        var_jtsswgs_t = assign18610_e34735;
        var_jtsswgs_t_dn4 = assign18610_e34735_d_n4;
        var_jtsswgs_t_rv = 0.0;

        let (assign18620_e34755, assign18620_e34755_d_n4,) = {
    if (var_guard336 != 0.0) {
        let assign18620_e34740: f64 = (p.p1636 / var_weff0);
        let assign18620_e34741: f64 = (assign18620_e34740).sqrt();
        let assign18620_e34743: f64 = (assign18620_e34741 + 1.0);
        let assign18620_e34744: f64 = (p.p1635 * assign18620_e34743);
        let assign18620_e34747: f64 = (var_eg0 * p.p1734);
        let assign18620_e34749: f64 = (assign18620_e34747 * var_tratio_m1);
        let assign18620_e34751: f64 = (assign18620_e34749 / var_vtm);
        let assign18620_e34752: f64 = { let limited_exp_arg = assign18620_e34751; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18620_e34753: f64 = (assign18620_e34744 * assign18620_e34752);
        (assign18620_e34753, (assign18620_e34744 * ({ let limited_exp_arg = assign18620_e34751; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign18620_e34747 * var_tratio_m1_dn4) * var_vtm) - (assign18620_e34749 * var_vtm_dn4)) / (var_vtm * var_vtm)))),)
    } else {
        (var_jtsswgd_t, var_jtsswgd_t_dn4,)
    }
};
        var_jtsswgd_t = assign18620_e34755;
        var_jtsswgd_t_dn4 = assign18620_e34755_d_n4;
        var_jtsswgd_t_rv = 0.0;

        *var_jsd_t_slot = var_jsd_t;
        *var_jsd_t_dn0_slot = var_jsd_t_dn0;
        *var_jsd_t_dn10_slot = var_jsd_t_dn10;
        *var_jsd_t_dn11_slot = var_jsd_t_dn11;
        *var_jsd_t_dn13_slot = var_jsd_t_dn13;
        *var_jsd_t_dn14_slot = var_jsd_t_dn14;
        *var_jsd_t_dn2_slot = var_jsd_t_dn2;
        *var_jsd_t_dn3_slot = var_jsd_t_dn3;
        *var_jsd_t_dn4_slot = var_jsd_t_dn4;
        *var_jsd_t_dn5_slot = var_jsd_t_dn5;
        *var_jsd_t_dn6_slot = var_jsd_t_dn6;
        *var_jsd_t_dn7_slot = var_jsd_t_dn7;
        *var_jsd_t_dn8_slot = var_jsd_t_dn8;
        *var_jsd_t_dn9_slot = var_jsd_t_dn9;
        *var_jsd_t_rv_slot = var_jsd_t_rv;
        *var_jss_t_slot = var_jss_t;
        *var_jss_t_dn0_slot = var_jss_t_dn0;
        *var_jss_t_dn10_slot = var_jss_t_dn10;
        *var_jss_t_dn11_slot = var_jss_t_dn11;
        *var_jss_t_dn13_slot = var_jss_t_dn13;
        *var_jss_t_dn14_slot = var_jss_t_dn14;
        *var_jss_t_dn2_slot = var_jss_t_dn2;
        *var_jss_t_dn3_slot = var_jss_t_dn3;
        *var_jss_t_dn4_slot = var_jss_t_dn4;
        *var_jss_t_dn5_slot = var_jss_t_dn5;
        *var_jss_t_dn6_slot = var_jss_t_dn6;
        *var_jss_t_dn7_slot = var_jss_t_dn7;
        *var_jss_t_dn8_slot = var_jss_t_dn8;
        *var_jss_t_dn9_slot = var_jss_t_dn9;
        *var_jss_t_rv_slot = var_jss_t_rv;
        *var_jswd_t_slot = var_jswd_t;
        *var_jswd_t_dn0_slot = var_jswd_t_dn0;
        *var_jswd_t_dn10_slot = var_jswd_t_dn10;
        *var_jswd_t_dn11_slot = var_jswd_t_dn11;
        *var_jswd_t_dn13_slot = var_jswd_t_dn13;
        *var_jswd_t_dn14_slot = var_jswd_t_dn14;
        *var_jswd_t_dn2_slot = var_jswd_t_dn2;
        *var_jswd_t_dn3_slot = var_jswd_t_dn3;
        *var_jswd_t_dn4_slot = var_jswd_t_dn4;
        *var_jswd_t_dn5_slot = var_jswd_t_dn5;
        *var_jswd_t_dn6_slot = var_jswd_t_dn6;
        *var_jswd_t_dn7_slot = var_jswd_t_dn7;
        *var_jswd_t_dn8_slot = var_jswd_t_dn8;
        *var_jswd_t_dn9_slot = var_jswd_t_dn9;
        *var_jswd_t_rv_slot = var_jswd_t_rv;
        *var_jswgd_t_slot = var_jswgd_t;
        *var_jswgd_t_dn0_slot = var_jswgd_t_dn0;
        *var_jswgd_t_dn10_slot = var_jswgd_t_dn10;
        *var_jswgd_t_dn11_slot = var_jswgd_t_dn11;
        *var_jswgd_t_dn13_slot = var_jswgd_t_dn13;
        *var_jswgd_t_dn14_slot = var_jswgd_t_dn14;
        *var_jswgd_t_dn2_slot = var_jswgd_t_dn2;
        *var_jswgd_t_dn3_slot = var_jswgd_t_dn3;
        *var_jswgd_t_dn4_slot = var_jswgd_t_dn4;
        *var_jswgd_t_dn5_slot = var_jswgd_t_dn5;
        *var_jswgd_t_dn6_slot = var_jswgd_t_dn6;
        *var_jswgd_t_dn7_slot = var_jswgd_t_dn7;
        *var_jswgd_t_dn8_slot = var_jswgd_t_dn8;
        *var_jswgd_t_dn9_slot = var_jswgd_t_dn9;
        *var_jswgd_t_rv_slot = var_jswgd_t_rv;
        *var_jswgs_t_slot = var_jswgs_t;
        *var_jswgs_t_dn0_slot = var_jswgs_t_dn0;
        *var_jswgs_t_dn10_slot = var_jswgs_t_dn10;
        *var_jswgs_t_dn11_slot = var_jswgs_t_dn11;
        *var_jswgs_t_dn13_slot = var_jswgs_t_dn13;
        *var_jswgs_t_dn14_slot = var_jswgs_t_dn14;
        *var_jswgs_t_dn2_slot = var_jswgs_t_dn2;
        *var_jswgs_t_dn3_slot = var_jswgs_t_dn3;
        *var_jswgs_t_dn4_slot = var_jswgs_t_dn4;
        *var_jswgs_t_dn5_slot = var_jswgs_t_dn5;
        *var_jswgs_t_dn6_slot = var_jswgs_t_dn6;
        *var_jswgs_t_dn7_slot = var_jswgs_t_dn7;
        *var_jswgs_t_dn8_slot = var_jswgs_t_dn8;
        *var_jswgs_t_dn9_slot = var_jswgs_t_dn9;
        *var_jswgs_t_rv_slot = var_jswgs_t_rv;
        *var_jsws_t_slot = var_jsws_t;
        *var_jsws_t_dn0_slot = var_jsws_t_dn0;
        *var_jsws_t_dn10_slot = var_jsws_t_dn10;
        *var_jsws_t_dn11_slot = var_jsws_t_dn11;
        *var_jsws_t_dn13_slot = var_jsws_t_dn13;
        *var_jsws_t_dn14_slot = var_jsws_t_dn14;
        *var_jsws_t_dn2_slot = var_jsws_t_dn2;
        *var_jsws_t_dn3_slot = var_jsws_t_dn3;
        *var_jsws_t_dn4_slot = var_jsws_t_dn4;
        *var_jsws_t_dn5_slot = var_jsws_t_dn5;
        *var_jsws_t_dn6_slot = var_jsws_t_dn6;
        *var_jsws_t_dn7_slot = var_jsws_t_dn7;
        *var_jsws_t_dn8_slot = var_jsws_t_dn8;
        *var_jsws_t_dn9_slot = var_jsws_t_dn9;
        *var_jsws_t_rv_slot = var_jsws_t_rv;
        *var_jtsd_t_slot = var_jtsd_t;
        *var_jtsd_t_dn4_slot = var_jtsd_t_dn4;
        *var_jtsd_t_rv_slot = var_jtsd_t_rv;
        *var_jtss_t_slot = var_jtss_t;
        *var_jtss_t_dn4_slot = var_jtss_t_dn4;
        *var_jtss_t_rv_slot = var_jtss_t_rv;
        *var_jtsswd_t_slot = var_jtsswd_t;
        *var_jtsswd_t_dn4_slot = var_jtsswd_t_dn4;
        *var_jtsswd_t_rv_slot = var_jtsswd_t_rv;
        *var_jtsswgd_t_slot = var_jtsswgd_t;
        *var_jtsswgd_t_dn4_slot = var_jtsswgd_t_dn4;
        *var_jtsswgd_t_rv_slot = var_jtsswgd_t_rv;
        *var_jtsswgs_t_slot = var_jtsswgs_t;
        *var_jtsswgs_t_dn4_slot = var_jtsswgs_t_dn4;
        *var_jtsswgs_t_rv_slot = var_jtsswgs_t_rv;
        *var_jtssws_t_slot = var_jtssws_t;
        *var_jtssws_t_dn4_slot = var_jtssws_t_dn4;
        *var_jtssws_t_rv_slot = var_jtssws_t_rv;
        *var_pbswd_t_slot = var_pbswd_t;
        *var_pbswd_t_dn4_slot = var_pbswd_t_dn4;
        *var_pbswd_t_rv_slot = var_pbswd_t_rv;
        *var_pbswgd_t_slot = var_pbswgd_t;
        *var_pbswgd_t_dn4_slot = var_pbswgd_t_dn4;
        *var_pbswgd_t_rv_slot = var_pbswgd_t_rv;
        *var_pbswgs_t_slot = var_pbswgs_t;
        *var_pbswgs_t_dn4_slot = var_pbswgs_t_dn4;
        *var_pbswgs_t_rv_slot = var_pbswgs_t_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn13_slot = var_t0_dn13;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
    }

    pub(super) fn stamp_reactive_block_66(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard336: f64,
        var_tratio_m1: f64,
        var_tratio_m1_dn4: f64,
        var_guard343_slot: &mut f64,
        var_guard343_rv_slot: &mut f64,
        var_guard344_slot: &mut f64,
        var_guard344_rv_slot: &mut f64,
        var_guard345_slot: &mut f64,
        var_guard345_rv_slot: &mut f64,
        var_njts_t_slot: &mut f64,
        var_njts_t_dn4_slot: &mut f64,
        var_njts_t_rv_slot: &mut f64,
        var_njtsd_t_slot: &mut f64,
        var_njtsd_t_dn4_slot: &mut f64,
        var_njtsd_t_rv_slot: &mut f64,
        var_njtssw_t_slot: &mut f64,
        var_njtssw_t_dn4_slot: &mut f64,
        var_njtssw_t_rv_slot: &mut f64,
        var_njtsswd_t_slot: &mut f64,
        var_njtsswd_t_dn4_slot: &mut f64,
        var_njtsswd_t_rv_slot: &mut f64,
        var_njtsswg_t_slot: &mut f64,
        var_njtsswg_t_dn4_slot: &mut f64,
        var_njtsswg_t_rv_slot: &mut f64,
        var_njtsswgd_t_slot: &mut f64,
        var_njtsswgd_t_dn4_slot: &mut f64,
        var_njtsswgd_t_rv_slot: &mut f64,
    ) {
        let mut var_guard343: f64 = *var_guard343_slot;
        let mut var_guard343_rv: f64 = *var_guard343_rv_slot;
        let mut var_guard344: f64 = *var_guard344_slot;
        let mut var_guard344_rv: f64 = *var_guard344_rv_slot;
        let mut var_guard345: f64 = *var_guard345_slot;
        let mut var_guard345_rv: f64 = *var_guard345_rv_slot;
        let mut var_njts_t: f64 = *var_njts_t_slot;
        let mut var_njts_t_dn4: f64 = *var_njts_t_dn4_slot;
        let mut var_njts_t_rv: f64 = *var_njts_t_rv_slot;
        let mut var_njtsd_t: f64 = *var_njtsd_t_slot;
        let mut var_njtsd_t_dn4: f64 = *var_njtsd_t_dn4_slot;
        let mut var_njtsd_t_rv: f64 = *var_njtsd_t_rv_slot;
        let mut var_njtssw_t: f64 = *var_njtssw_t_slot;
        let mut var_njtssw_t_dn4: f64 = *var_njtssw_t_dn4_slot;
        let mut var_njtssw_t_rv: f64 = *var_njtssw_t_rv_slot;
        let mut var_njtsswd_t: f64 = *var_njtsswd_t_slot;
        let mut var_njtsswd_t_dn4: f64 = *var_njtsswd_t_dn4_slot;
        let mut var_njtsswd_t_rv: f64 = *var_njtsswd_t_rv_slot;
        let mut var_njtsswg_t: f64 = *var_njtsswg_t_slot;
        let mut var_njtsswg_t_dn4: f64 = *var_njtsswg_t_dn4_slot;
        let mut var_njtsswg_t_rv: f64 = *var_njtsswg_t_rv_slot;
        let mut var_njtsswgd_t: f64 = *var_njtsswgd_t_slot;
        let mut var_njtsswgd_t_dn4: f64 = *var_njtsswgd_t_dn4_slot;
        let mut var_njtsswgd_t_rv: f64 = *var_njtsswgd_t_rv_slot;

        let (assign18630_e34844, assign18630_e34844_d_n4,) = {
    if (var_guard336 != 0.0) {
        let assign18630_e34761: f64 = (p.p1735 * var_tratio_m1);
        let assign18630_e34762: f64 = (1.0 + assign18630_e34761);
        let assign18630_e34763: f64 = (p.p1637 * assign18630_e34762);
        let assign18630_e34765: f64 = (assign18630_e34763 - 0.01);
        let assign18630_e34767: f64 = (-10000.0);
        let assign18630_e34769: f64 = (assign18630_e34767 * 0.001);
        let (assign18630_e34840, assign18630_e34840_d_n4,) = {
            if (!(assign18630_e34765 < assign18630_e34769)) {
                let assign18630_e34777: f64 = (p.p1735 * var_tratio_m1);
                let assign18630_e34778: f64 = (1.0 + assign18630_e34777);
                let assign18630_e34779: f64 = (p.p1637 * assign18630_e34778);
                let assign18630_e34781: f64 = (assign18630_e34779 - 0.01);
                let assign18630_e34786: f64 = (p.p1735 * var_tratio_m1);
                let assign18630_e34787: f64 = (1.0 + assign18630_e34786);
                let assign18630_e34788: f64 = (p.p1637 * assign18630_e34787);
                let assign18630_e34790: f64 = (assign18630_e34788 - 0.01);
                let assign18630_e34795: f64 = (p.p1735 * var_tratio_m1);
                let assign18630_e34796: f64 = (1.0 + assign18630_e34795);
                let assign18630_e34797: f64 = (p.p1637 * assign18630_e34796);
                let assign18630_e34799: f64 = (assign18630_e34797 - 0.01);
                let assign18630_e34800: f64 = (assign18630_e34790 * assign18630_e34799);
                let assign18630_e34803: f64 = (4.0 * 0.001);
                let assign18630_e34805: f64 = (assign18630_e34803 * 0.001);
                let assign18630_e34806: f64 = (assign18630_e34800 + assign18630_e34805);
                let assign18630_e34807: f64 = (assign18630_e34806).sqrt();
                let assign18630_e34808: f64 = (assign18630_e34781 + assign18630_e34807);
                let assign18630_e34809: f64 = (0.5 * assign18630_e34808);
                (assign18630_e34809, (0.5 * ((p.p1637 * (p.p1735 * var_tratio_m1_dn4)) + ((((p.p1637 * (p.p1735 * var_tratio_m1_dn4)) * assign18630_e34799) + (assign18630_e34790 * (p.p1637 * (p.p1735 * var_tratio_m1_dn4)))) / (2.0 * assign18630_e34807)))),)
            } else {
                let assign18630_e34814: f64 = (p.p1735 * var_tratio_m1);
                let assign18630_e34815: f64 = (1.0 + assign18630_e34814);
                let assign18630_e34816: f64 = (p.p1637 * assign18630_e34815);
                let assign18630_e34818: f64 = (assign18630_e34816 - 0.01);
                let assign18630_e34820: f64 = (-10000.0);
                let assign18630_e34822: f64 = (assign18630_e34820 * 0.001);
                let (assign18630_e34839, assign18630_e34839_d_n4,) = {
                    if (assign18630_e34818 < assign18630_e34822) {
                        let assign18630_e34825: f64 = (-0.001);
                        let assign18630_e34827: f64 = (assign18630_e34825 * 0.001);
                        let assign18630_e34832: f64 = (p.p1735 * var_tratio_m1);
                        let assign18630_e34833: f64 = (1.0 + assign18630_e34832);
                        let assign18630_e34834: f64 = (p.p1637 * assign18630_e34833);
                        let assign18630_e34836: f64 = (assign18630_e34834 - 0.01);
                        let assign18630_e34837: f64 = (assign18630_e34827 / assign18630_e34836);
                        (assign18630_e34837, (-((assign18630_e34827 * (p.p1637 * (p.p1735 * var_tratio_m1_dn4))) / (assign18630_e34836 * assign18630_e34836))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18630_e34839, assign18630_e34839_d_n4,)
            }
        };
        let assign18630_e34842: f64 = (assign18630_e34840 + 0.01);
        (assign18630_e34842, assign18630_e34840_d_n4,)
    } else {
        (var_njts_t, var_njts_t_dn4,)
    }
};
        var_njts_t = assign18630_e34844;
        var_njts_t_dn4 = assign18630_e34844_d_n4;
        var_njts_t_rv = 0.0;

        let (assign18640_e34933, assign18640_e34933_d_n4,) = {
    if (var_guard336 != 0.0) {
        let assign18640_e34850: f64 = (p.p1736 * var_tratio_m1);
        let assign18640_e34851: f64 = (1.0 + assign18640_e34850);
        let assign18640_e34852: f64 = (p.p1638 * assign18640_e34851);
        let assign18640_e34854: f64 = (assign18640_e34852 - 0.01);
        let assign18640_e34856: f64 = (-10000.0);
        let assign18640_e34858: f64 = (assign18640_e34856 * 0.001);
        let (assign18640_e34929, assign18640_e34929_d_n4,) = {
            if (!(assign18640_e34854 < assign18640_e34858)) {
                let assign18640_e34866: f64 = (p.p1736 * var_tratio_m1);
                let assign18640_e34867: f64 = (1.0 + assign18640_e34866);
                let assign18640_e34868: f64 = (p.p1638 * assign18640_e34867);
                let assign18640_e34870: f64 = (assign18640_e34868 - 0.01);
                let assign18640_e34875: f64 = (p.p1736 * var_tratio_m1);
                let assign18640_e34876: f64 = (1.0 + assign18640_e34875);
                let assign18640_e34877: f64 = (p.p1638 * assign18640_e34876);
                let assign18640_e34879: f64 = (assign18640_e34877 - 0.01);
                let assign18640_e34884: f64 = (p.p1736 * var_tratio_m1);
                let assign18640_e34885: f64 = (1.0 + assign18640_e34884);
                let assign18640_e34886: f64 = (p.p1638 * assign18640_e34885);
                let assign18640_e34888: f64 = (assign18640_e34886 - 0.01);
                let assign18640_e34889: f64 = (assign18640_e34879 * assign18640_e34888);
                let assign18640_e34892: f64 = (4.0 * 0.001);
                let assign18640_e34894: f64 = (assign18640_e34892 * 0.001);
                let assign18640_e34895: f64 = (assign18640_e34889 + assign18640_e34894);
                let assign18640_e34896: f64 = (assign18640_e34895).sqrt();
                let assign18640_e34897: f64 = (assign18640_e34870 + assign18640_e34896);
                let assign18640_e34898: f64 = (0.5 * assign18640_e34897);
                (assign18640_e34898, (0.5 * ((p.p1638 * (p.p1736 * var_tratio_m1_dn4)) + ((((p.p1638 * (p.p1736 * var_tratio_m1_dn4)) * assign18640_e34888) + (assign18640_e34879 * (p.p1638 * (p.p1736 * var_tratio_m1_dn4)))) / (2.0 * assign18640_e34896)))),)
            } else {
                let assign18640_e34903: f64 = (p.p1736 * var_tratio_m1);
                let assign18640_e34904: f64 = (1.0 + assign18640_e34903);
                let assign18640_e34905: f64 = (p.p1638 * assign18640_e34904);
                let assign18640_e34907: f64 = (assign18640_e34905 - 0.01);
                let assign18640_e34909: f64 = (-10000.0);
                let assign18640_e34911: f64 = (assign18640_e34909 * 0.001);
                let (assign18640_e34928, assign18640_e34928_d_n4,) = {
                    if (assign18640_e34907 < assign18640_e34911) {
                        let assign18640_e34914: f64 = (-0.001);
                        let assign18640_e34916: f64 = (assign18640_e34914 * 0.001);
                        let assign18640_e34921: f64 = (p.p1736 * var_tratio_m1);
                        let assign18640_e34922: f64 = (1.0 + assign18640_e34921);
                        let assign18640_e34923: f64 = (p.p1638 * assign18640_e34922);
                        let assign18640_e34925: f64 = (assign18640_e34923 - 0.01);
                        let assign18640_e34926: f64 = (assign18640_e34916 / assign18640_e34925);
                        (assign18640_e34926, (-((assign18640_e34916 * (p.p1638 * (p.p1736 * var_tratio_m1_dn4))) / (assign18640_e34925 * assign18640_e34925))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18640_e34928, assign18640_e34928_d_n4,)
            }
        };
        let assign18640_e34931: f64 = (assign18640_e34929 + 0.01);
        (assign18640_e34931, assign18640_e34929_d_n4,)
    } else {
        (var_njtsd_t, var_njtsd_t_dn4,)
    }
};
        var_njtsd_t = assign18640_e34933;
        var_njtsd_t_dn4 = assign18640_e34933_d_n4;
        var_njtsd_t_rv = 0.0;

        let (assign18650_e35022, assign18650_e35022_d_n4,) = {
    if (var_guard336 != 0.0) {
        let assign18650_e34939: f64 = (p.p1737 * var_tratio_m1);
        let assign18650_e34940: f64 = (1.0 + assign18650_e34939);
        let assign18650_e34941: f64 = (p.p1639 * assign18650_e34940);
        let assign18650_e34943: f64 = (assign18650_e34941 - 0.01);
        let assign18650_e34945: f64 = (-10000.0);
        let assign18650_e34947: f64 = (assign18650_e34945 * 0.001);
        let (assign18650_e35018, assign18650_e35018_d_n4,) = {
            if (!(assign18650_e34943 < assign18650_e34947)) {
                let assign18650_e34955: f64 = (p.p1737 * var_tratio_m1);
                let assign18650_e34956: f64 = (1.0 + assign18650_e34955);
                let assign18650_e34957: f64 = (p.p1639 * assign18650_e34956);
                let assign18650_e34959: f64 = (assign18650_e34957 - 0.01);
                let assign18650_e34964: f64 = (p.p1737 * var_tratio_m1);
                let assign18650_e34965: f64 = (1.0 + assign18650_e34964);
                let assign18650_e34966: f64 = (p.p1639 * assign18650_e34965);
                let assign18650_e34968: f64 = (assign18650_e34966 - 0.01);
                let assign18650_e34973: f64 = (p.p1737 * var_tratio_m1);
                let assign18650_e34974: f64 = (1.0 + assign18650_e34973);
                let assign18650_e34975: f64 = (p.p1639 * assign18650_e34974);
                let assign18650_e34977: f64 = (assign18650_e34975 - 0.01);
                let assign18650_e34978: f64 = (assign18650_e34968 * assign18650_e34977);
                let assign18650_e34981: f64 = (4.0 * 0.001);
                let assign18650_e34983: f64 = (assign18650_e34981 * 0.001);
                let assign18650_e34984: f64 = (assign18650_e34978 + assign18650_e34983);
                let assign18650_e34985: f64 = (assign18650_e34984).sqrt();
                let assign18650_e34986: f64 = (assign18650_e34959 + assign18650_e34985);
                let assign18650_e34987: f64 = (0.5 * assign18650_e34986);
                (assign18650_e34987, (0.5 * ((p.p1639 * (p.p1737 * var_tratio_m1_dn4)) + ((((p.p1639 * (p.p1737 * var_tratio_m1_dn4)) * assign18650_e34977) + (assign18650_e34968 * (p.p1639 * (p.p1737 * var_tratio_m1_dn4)))) / (2.0 * assign18650_e34985)))),)
            } else {
                let assign18650_e34992: f64 = (p.p1737 * var_tratio_m1);
                let assign18650_e34993: f64 = (1.0 + assign18650_e34992);
                let assign18650_e34994: f64 = (p.p1639 * assign18650_e34993);
                let assign18650_e34996: f64 = (assign18650_e34994 - 0.01);
                let assign18650_e34998: f64 = (-10000.0);
                let assign18650_e35000: f64 = (assign18650_e34998 * 0.001);
                let (assign18650_e35017, assign18650_e35017_d_n4,) = {
                    if (assign18650_e34996 < assign18650_e35000) {
                        let assign18650_e35003: f64 = (-0.001);
                        let assign18650_e35005: f64 = (assign18650_e35003 * 0.001);
                        let assign18650_e35010: f64 = (p.p1737 * var_tratio_m1);
                        let assign18650_e35011: f64 = (1.0 + assign18650_e35010);
                        let assign18650_e35012: f64 = (p.p1639 * assign18650_e35011);
                        let assign18650_e35014: f64 = (assign18650_e35012 - 0.01);
                        let assign18650_e35015: f64 = (assign18650_e35005 / assign18650_e35014);
                        (assign18650_e35015, (-((assign18650_e35005 * (p.p1639 * (p.p1737 * var_tratio_m1_dn4))) / (assign18650_e35014 * assign18650_e35014))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18650_e35017, assign18650_e35017_d_n4,)
            }
        };
        let assign18650_e35020: f64 = (assign18650_e35018 + 0.01);
        (assign18650_e35020, assign18650_e35018_d_n4,)
    } else {
        (var_njtssw_t, var_njtssw_t_dn4,)
    }
};
        var_njtssw_t = assign18650_e35022;
        var_njtssw_t_dn4 = assign18650_e35022_d_n4;
        var_njtssw_t_rv = 0.0;

        let (assign18660_e35111, assign18660_e35111_d_n4,) = {
    if (var_guard336 != 0.0) {
        let assign18660_e35028: f64 = (p.p1738 * var_tratio_m1);
        let assign18660_e35029: f64 = (1.0 + assign18660_e35028);
        let assign18660_e35030: f64 = (p.p1640 * assign18660_e35029);
        let assign18660_e35032: f64 = (assign18660_e35030 - 0.01);
        let assign18660_e35034: f64 = (-10000.0);
        let assign18660_e35036: f64 = (assign18660_e35034 * 0.001);
        let (assign18660_e35107, assign18660_e35107_d_n4,) = {
            if (!(assign18660_e35032 < assign18660_e35036)) {
                let assign18660_e35044: f64 = (p.p1738 * var_tratio_m1);
                let assign18660_e35045: f64 = (1.0 + assign18660_e35044);
                let assign18660_e35046: f64 = (p.p1640 * assign18660_e35045);
                let assign18660_e35048: f64 = (assign18660_e35046 - 0.01);
                let assign18660_e35053: f64 = (p.p1738 * var_tratio_m1);
                let assign18660_e35054: f64 = (1.0 + assign18660_e35053);
                let assign18660_e35055: f64 = (p.p1640 * assign18660_e35054);
                let assign18660_e35057: f64 = (assign18660_e35055 - 0.01);
                let assign18660_e35062: f64 = (p.p1738 * var_tratio_m1);
                let assign18660_e35063: f64 = (1.0 + assign18660_e35062);
                let assign18660_e35064: f64 = (p.p1640 * assign18660_e35063);
                let assign18660_e35066: f64 = (assign18660_e35064 - 0.01);
                let assign18660_e35067: f64 = (assign18660_e35057 * assign18660_e35066);
                let assign18660_e35070: f64 = (4.0 * 0.001);
                let assign18660_e35072: f64 = (assign18660_e35070 * 0.001);
                let assign18660_e35073: f64 = (assign18660_e35067 + assign18660_e35072);
                let assign18660_e35074: f64 = (assign18660_e35073).sqrt();
                let assign18660_e35075: f64 = (assign18660_e35048 + assign18660_e35074);
                let assign18660_e35076: f64 = (0.5 * assign18660_e35075);
                (assign18660_e35076, (0.5 * ((p.p1640 * (p.p1738 * var_tratio_m1_dn4)) + ((((p.p1640 * (p.p1738 * var_tratio_m1_dn4)) * assign18660_e35066) + (assign18660_e35057 * (p.p1640 * (p.p1738 * var_tratio_m1_dn4)))) / (2.0 * assign18660_e35074)))),)
            } else {
                let assign18660_e35081: f64 = (p.p1738 * var_tratio_m1);
                let assign18660_e35082: f64 = (1.0 + assign18660_e35081);
                let assign18660_e35083: f64 = (p.p1640 * assign18660_e35082);
                let assign18660_e35085: f64 = (assign18660_e35083 - 0.01);
                let assign18660_e35087: f64 = (-10000.0);
                let assign18660_e35089: f64 = (assign18660_e35087 * 0.001);
                let (assign18660_e35106, assign18660_e35106_d_n4,) = {
                    if (assign18660_e35085 < assign18660_e35089) {
                        let assign18660_e35092: f64 = (-0.001);
                        let assign18660_e35094: f64 = (assign18660_e35092 * 0.001);
                        let assign18660_e35099: f64 = (p.p1738 * var_tratio_m1);
                        let assign18660_e35100: f64 = (1.0 + assign18660_e35099);
                        let assign18660_e35101: f64 = (p.p1640 * assign18660_e35100);
                        let assign18660_e35103: f64 = (assign18660_e35101 - 0.01);
                        let assign18660_e35104: f64 = (assign18660_e35094 / assign18660_e35103);
                        (assign18660_e35104, (-((assign18660_e35094 * (p.p1640 * (p.p1738 * var_tratio_m1_dn4))) / (assign18660_e35103 * assign18660_e35103))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18660_e35106, assign18660_e35106_d_n4,)
            }
        };
        let assign18660_e35109: f64 = (assign18660_e35107 + 0.01);
        (assign18660_e35109, assign18660_e35107_d_n4,)
    } else {
        (var_njtsswd_t, var_njtsswd_t_dn4,)
    }
};
        var_njtsswd_t = assign18660_e35111;
        var_njtsswd_t_dn4 = assign18660_e35111_d_n4;
        var_njtsswd_t_rv = 0.0;

        let (assign18670_e35200, assign18670_e35200_d_n4,) = {
    if (var_guard336 != 0.0) {
        let assign18670_e35117: f64 = (p.p1739 * var_tratio_m1);
        let assign18670_e35118: f64 = (1.0 + assign18670_e35117);
        let assign18670_e35119: f64 = (p.p1641 * assign18670_e35118);
        let assign18670_e35121: f64 = (assign18670_e35119 - 0.01);
        let assign18670_e35123: f64 = (-10000.0);
        let assign18670_e35125: f64 = (assign18670_e35123 * 0.001);
        let (assign18670_e35196, assign18670_e35196_d_n4,) = {
            if (!(assign18670_e35121 < assign18670_e35125)) {
                let assign18670_e35133: f64 = (p.p1739 * var_tratio_m1);
                let assign18670_e35134: f64 = (1.0 + assign18670_e35133);
                let assign18670_e35135: f64 = (p.p1641 * assign18670_e35134);
                let assign18670_e35137: f64 = (assign18670_e35135 - 0.01);
                let assign18670_e35142: f64 = (p.p1739 * var_tratio_m1);
                let assign18670_e35143: f64 = (1.0 + assign18670_e35142);
                let assign18670_e35144: f64 = (p.p1641 * assign18670_e35143);
                let assign18670_e35146: f64 = (assign18670_e35144 - 0.01);
                let assign18670_e35151: f64 = (p.p1739 * var_tratio_m1);
                let assign18670_e35152: f64 = (1.0 + assign18670_e35151);
                let assign18670_e35153: f64 = (p.p1641 * assign18670_e35152);
                let assign18670_e35155: f64 = (assign18670_e35153 - 0.01);
                let assign18670_e35156: f64 = (assign18670_e35146 * assign18670_e35155);
                let assign18670_e35159: f64 = (4.0 * 0.001);
                let assign18670_e35161: f64 = (assign18670_e35159 * 0.001);
                let assign18670_e35162: f64 = (assign18670_e35156 + assign18670_e35161);
                let assign18670_e35163: f64 = (assign18670_e35162).sqrt();
                let assign18670_e35164: f64 = (assign18670_e35137 + assign18670_e35163);
                let assign18670_e35165: f64 = (0.5 * assign18670_e35164);
                (assign18670_e35165, (0.5 * ((p.p1641 * (p.p1739 * var_tratio_m1_dn4)) + ((((p.p1641 * (p.p1739 * var_tratio_m1_dn4)) * assign18670_e35155) + (assign18670_e35146 * (p.p1641 * (p.p1739 * var_tratio_m1_dn4)))) / (2.0 * assign18670_e35163)))),)
            } else {
                let assign18670_e35170: f64 = (p.p1739 * var_tratio_m1);
                let assign18670_e35171: f64 = (1.0 + assign18670_e35170);
                let assign18670_e35172: f64 = (p.p1641 * assign18670_e35171);
                let assign18670_e35174: f64 = (assign18670_e35172 - 0.01);
                let assign18670_e35176: f64 = (-10000.0);
                let assign18670_e35178: f64 = (assign18670_e35176 * 0.001);
                let (assign18670_e35195, assign18670_e35195_d_n4,) = {
                    if (assign18670_e35174 < assign18670_e35178) {
                        let assign18670_e35181: f64 = (-0.001);
                        let assign18670_e35183: f64 = (assign18670_e35181 * 0.001);
                        let assign18670_e35188: f64 = (p.p1739 * var_tratio_m1);
                        let assign18670_e35189: f64 = (1.0 + assign18670_e35188);
                        let assign18670_e35190: f64 = (p.p1641 * assign18670_e35189);
                        let assign18670_e35192: f64 = (assign18670_e35190 - 0.01);
                        let assign18670_e35193: f64 = (assign18670_e35183 / assign18670_e35192);
                        (assign18670_e35193, (-((assign18670_e35183 * (p.p1641 * (p.p1739 * var_tratio_m1_dn4))) / (assign18670_e35192 * assign18670_e35192))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18670_e35195, assign18670_e35195_d_n4,)
            }
        };
        let assign18670_e35198: f64 = (assign18670_e35196 + 0.01);
        (assign18670_e35198, assign18670_e35196_d_n4,)
    } else {
        (var_njtsswg_t, var_njtsswg_t_dn4,)
    }
};
        var_njtsswg_t = assign18670_e35200;
        var_njtsswg_t_dn4 = assign18670_e35200_d_n4;
        var_njtsswg_t_rv = 0.0;

        let (assign18680_e35289, assign18680_e35289_d_n4,) = {
    if (var_guard336 != 0.0) {
        let assign18680_e35206: f64 = (p.p1740 * var_tratio_m1);
        let assign18680_e35207: f64 = (1.0 + assign18680_e35206);
        let assign18680_e35208: f64 = (p.p1642 * assign18680_e35207);
        let assign18680_e35210: f64 = (assign18680_e35208 - 0.01);
        let assign18680_e35212: f64 = (-10000.0);
        let assign18680_e35214: f64 = (assign18680_e35212 * 0.001);
        let (assign18680_e35285, assign18680_e35285_d_n4,) = {
            if (!(assign18680_e35210 < assign18680_e35214)) {
                let assign18680_e35222: f64 = (p.p1740 * var_tratio_m1);
                let assign18680_e35223: f64 = (1.0 + assign18680_e35222);
                let assign18680_e35224: f64 = (p.p1642 * assign18680_e35223);
                let assign18680_e35226: f64 = (assign18680_e35224 - 0.01);
                let assign18680_e35231: f64 = (p.p1740 * var_tratio_m1);
                let assign18680_e35232: f64 = (1.0 + assign18680_e35231);
                let assign18680_e35233: f64 = (p.p1642 * assign18680_e35232);
                let assign18680_e35235: f64 = (assign18680_e35233 - 0.01);
                let assign18680_e35240: f64 = (p.p1740 * var_tratio_m1);
                let assign18680_e35241: f64 = (1.0 + assign18680_e35240);
                let assign18680_e35242: f64 = (p.p1642 * assign18680_e35241);
                let assign18680_e35244: f64 = (assign18680_e35242 - 0.01);
                let assign18680_e35245: f64 = (assign18680_e35235 * assign18680_e35244);
                let assign18680_e35248: f64 = (4.0 * 0.001);
                let assign18680_e35250: f64 = (assign18680_e35248 * 0.001);
                let assign18680_e35251: f64 = (assign18680_e35245 + assign18680_e35250);
                let assign18680_e35252: f64 = (assign18680_e35251).sqrt();
                let assign18680_e35253: f64 = (assign18680_e35226 + assign18680_e35252);
                let assign18680_e35254: f64 = (0.5 * assign18680_e35253);
                (assign18680_e35254, (0.5 * ((p.p1642 * (p.p1740 * var_tratio_m1_dn4)) + ((((p.p1642 * (p.p1740 * var_tratio_m1_dn4)) * assign18680_e35244) + (assign18680_e35235 * (p.p1642 * (p.p1740 * var_tratio_m1_dn4)))) / (2.0 * assign18680_e35252)))),)
            } else {
                let assign18680_e35259: f64 = (p.p1740 * var_tratio_m1);
                let assign18680_e35260: f64 = (1.0 + assign18680_e35259);
                let assign18680_e35261: f64 = (p.p1642 * assign18680_e35260);
                let assign18680_e35263: f64 = (assign18680_e35261 - 0.01);
                let assign18680_e35265: f64 = (-10000.0);
                let assign18680_e35267: f64 = (assign18680_e35265 * 0.001);
                let (assign18680_e35284, assign18680_e35284_d_n4,) = {
                    if (assign18680_e35263 < assign18680_e35267) {
                        let assign18680_e35270: f64 = (-0.001);
                        let assign18680_e35272: f64 = (assign18680_e35270 * 0.001);
                        let assign18680_e35277: f64 = (p.p1740 * var_tratio_m1);
                        let assign18680_e35278: f64 = (1.0 + assign18680_e35277);
                        let assign18680_e35279: f64 = (p.p1642 * assign18680_e35278);
                        let assign18680_e35281: f64 = (assign18680_e35279 - 0.01);
                        let assign18680_e35282: f64 = (assign18680_e35272 / assign18680_e35281);
                        (assign18680_e35282, (-((assign18680_e35272 * (p.p1642 * (p.p1740 * var_tratio_m1_dn4))) / (assign18680_e35281 * assign18680_e35281))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18680_e35284, assign18680_e35284_d_n4,)
            }
        };
        let assign18680_e35287: f64 = (assign18680_e35285 + 0.01);
        (assign18680_e35287, assign18680_e35285_d_n4,)
    } else {
        (var_njtsswgd_t, var_njtsswgd_t_dn4,)
    }
};
        var_njtsswgd_t = assign18680_e35289;
        var_njtsswgd_t_dn4 = assign18680_e35289_d_n4;
        var_njtsswgd_t_rv = 0.0;

        let assign18690_e35292: f64 = if (!param_given[1106]) { 1.0 } else { 0.0 };
        var_guard343 = assign18690_e35292;
        var_guard343_rv = 0.0;

        let assign18700_e35295: f64 = if p.p145 > 0.0 { 1.0 } else { 0.0 };
        var_guard344 = assign18700_e35295;
        var_guard344_rv = 0.0;

        let assign18710_e35298: f64 = if p.p80 == 0.0 { 1.0 } else { 0.0 };
        var_guard345 = assign18710_e35298;
        var_guard345_rv = 0.0;

        *var_guard343_slot = var_guard343;
        *var_guard343_rv_slot = var_guard343_rv;
        *var_guard344_slot = var_guard344;
        *var_guard344_rv_slot = var_guard344_rv;
        *var_guard345_slot = var_guard345;
        *var_guard345_rv_slot = var_guard345_rv;
        *var_njts_t_slot = var_njts_t;
        *var_njts_t_dn4_slot = var_njts_t_dn4;
        *var_njts_t_rv_slot = var_njts_t_rv;
        *var_njtsd_t_slot = var_njtsd_t;
        *var_njtsd_t_dn4_slot = var_njtsd_t_dn4;
        *var_njtsd_t_rv_slot = var_njtsd_t_rv;
        *var_njtssw_t_slot = var_njtssw_t;
        *var_njtssw_t_dn4_slot = var_njtssw_t_dn4;
        *var_njtssw_t_rv_slot = var_njtssw_t_rv;
        *var_njtsswd_t_slot = var_njtsswd_t;
        *var_njtsswd_t_dn4_slot = var_njtsswd_t_dn4;
        *var_njtsswd_t_rv_slot = var_njtsswd_t_rv;
        *var_njtsswg_t_slot = var_njtsswg_t;
        *var_njtsswg_t_dn4_slot = var_njtsswg_t_dn4;
        *var_njtsswg_t_rv_slot = var_njtsswg_t_rv;
        *var_njtsswgd_t_slot = var_njtsswgd_t;
        *var_njtsswgd_t_dn4_slot = var_njtsswgd_t_dn4;
        *var_njtsswgd_t_rv_slot = var_njtsswgd_t_rv;
    }

    pub(super) fn stamp_reactive_block_67(
        p: &Parameters,
        var_devsign: f64,
        var_eg: f64,
        var_eg_dn4: f64,
        var_guard343: f64,
        var_guard344: f64,
        var_guard345: f64,
        var_ni: f64,
        var_ni_dn0: f64,
        var_ni_dn10: f64,
        var_ni_dn11: f64,
        var_ni_dn13: f64,
        var_ni_dn14: f64,
        var_ni_dn2: f64,
        var_ni_dn3: f64,
        var_ni_dn4: f64,
        var_ni_dn5: f64,
        var_ni_dn6: f64,
        var_ni_dn7: f64,
        var_ni_dn8: f64,
        var_ni_dn9: f64,
        var_vtm: f64,
        var_vtm_dn4: f64,
        var_vfbsd_v_slot: &mut f64,
        var_vfbsd_v_dn0_slot: &mut f64,
        var_vfbsd_v_dn10_slot: &mut f64,
        var_vfbsd_v_dn11_slot: &mut f64,
        var_vfbsd_v_dn13_slot: &mut f64,
        var_vfbsd_v_dn14_slot: &mut f64,
        var_vfbsd_v_dn2_slot: &mut f64,
        var_vfbsd_v_dn3_slot: &mut f64,
        var_vfbsd_v_dn4_slot: &mut f64,
        var_vfbsd_v_dn5_slot: &mut f64,
        var_vfbsd_v_dn6_slot: &mut f64,
        var_vfbsd_v_dn7_slot: &mut f64,
        var_vfbsd_v_dn8_slot: &mut f64,
        var_vfbsd_v_dn9_slot: &mut f64,
        var_vfbsd_v_rv_slot: &mut f64,
    ) {
        let mut var_vfbsd_v: f64 = *var_vfbsd_v_slot;
        let mut var_vfbsd_v_dn0: f64 = *var_vfbsd_v_dn0_slot;
        let mut var_vfbsd_v_dn10: f64 = *var_vfbsd_v_dn10_slot;
        let mut var_vfbsd_v_dn11: f64 = *var_vfbsd_v_dn11_slot;
        let mut var_vfbsd_v_dn13: f64 = *var_vfbsd_v_dn13_slot;
        let mut var_vfbsd_v_dn14: f64 = *var_vfbsd_v_dn14_slot;
        let mut var_vfbsd_v_dn2: f64 = *var_vfbsd_v_dn2_slot;
        let mut var_vfbsd_v_dn3: f64 = *var_vfbsd_v_dn3_slot;
        let mut var_vfbsd_v_dn4: f64 = *var_vfbsd_v_dn4_slot;
        let mut var_vfbsd_v_dn5: f64 = *var_vfbsd_v_dn5_slot;
        let mut var_vfbsd_v_dn6: f64 = *var_vfbsd_v_dn6_slot;
        let mut var_vfbsd_v_dn7: f64 = *var_vfbsd_v_dn7_slot;
        let mut var_vfbsd_v_dn8: f64 = *var_vfbsd_v_dn8_slot;
        let mut var_vfbsd_v_dn9: f64 = *var_vfbsd_v_dn9_slot;
        let mut var_vfbsd_v_rv: f64 = *var_vfbsd_v_rv_slot;

        let (assign18720_e35690, assign18720_e35690_d_n0, assign18720_e35690_d_n2, assign18720_e35690_d_n3, assign18720_e35690_d_n4, assign18720_e35690_d_n5, assign18720_e35690_d_n6, assign18720_e35690_d_n7, assign18720_e35690_d_n8, assign18720_e35690_d_n9, assign18720_e35690_d_n10, assign18720_e35690_d_n11, assign18720_e35690_d_n13, assign18720_e35690_d_n14,) = {
    if (((var_guard343 != 0.0) && (var_guard344 != 0.0)) && (var_guard345 != 0.0)) {
        let assign18720_e35307: f64 = (0.5 * var_eg);
        let assign18720_e35311: f64 = (p.p145 / var_ni);
        let (assign18720_e35328, assign18720_e35328_d_n0, assign18720_e35328_d_n2, assign18720_e35328_d_n3, assign18720_e35328_d_n4, assign18720_e35328_d_n5, assign18720_e35328_d_n6, assign18720_e35328_d_n7, assign18720_e35328_d_n8, assign18720_e35328_d_n9, assign18720_e35328_d_n10, assign18720_e35328_d_n11, assign18720_e35328_d_n13, assign18720_e35328_d_n14,) = {
            if (!(assign18720_e35311 > 1e-38)) {
                let assign18720_e35316: f64 = (-87.498233534);
                (assign18720_e35316, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign18720_e35319: f64 = (p.p145 / var_ni);
                let (assign18720_e35327, assign18720_e35327_d_n0, assign18720_e35327_d_n2, assign18720_e35327_d_n3, assign18720_e35327_d_n4, assign18720_e35327_d_n5, assign18720_e35327_d_n6, assign18720_e35327_d_n7, assign18720_e35327_d_n8, assign18720_e35327_d_n9, assign18720_e35327_d_n10, assign18720_e35327_d_n11, assign18720_e35327_d_n13, assign18720_e35327_d_n14,) = {
                    if (assign18720_e35319 > 1e-38) {
                        let assign18720_e35324: f64 = (p.p145 / var_ni);
                        let assign18720_e35325: f64 = (assign18720_e35324).ln();
                        (assign18720_e35325, ((-((p.p145 * var_ni_dn0) / (var_ni * var_ni))) / assign18720_e35324), ((-((p.p145 * var_ni_dn2) / (var_ni * var_ni))) / assign18720_e35324), ((-((p.p145 * var_ni_dn3) / (var_ni * var_ni))) / assign18720_e35324), ((-((p.p145 * var_ni_dn4) / (var_ni * var_ni))) / assign18720_e35324), ((-((p.p145 * var_ni_dn5) / (var_ni * var_ni))) / assign18720_e35324), ((-((p.p145 * var_ni_dn6) / (var_ni * var_ni))) / assign18720_e35324), ((-((p.p145 * var_ni_dn7) / (var_ni * var_ni))) / assign18720_e35324), ((-((p.p145 * var_ni_dn8) / (var_ni * var_ni))) / assign18720_e35324), ((-((p.p145 * var_ni_dn9) / (var_ni * var_ni))) / assign18720_e35324), ((-((p.p145 * var_ni_dn10) / (var_ni * var_ni))) / assign18720_e35324), ((-((p.p145 * var_ni_dn11) / (var_ni * var_ni))) / assign18720_e35324), ((-((p.p145 * var_ni_dn13) / (var_ni * var_ni))) / assign18720_e35324), ((-((p.p145 * var_ni_dn14) / (var_ni * var_ni))) / assign18720_e35324),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18720_e35327, assign18720_e35327_d_n0, assign18720_e35327_d_n2, assign18720_e35327_d_n3, assign18720_e35327_d_n4, assign18720_e35327_d_n5, assign18720_e35327_d_n6, assign18720_e35327_d_n7, assign18720_e35327_d_n8, assign18720_e35327_d_n9, assign18720_e35327_d_n10, assign18720_e35327_d_n11, assign18720_e35327_d_n13, assign18720_e35327_d_n14,)
            }
        };
        let assign18720_e35329: f64 = (var_vtm * assign18720_e35328);
        let assign18720_e35330: f64 = (assign18720_e35307 - assign18720_e35329);
        let assign18720_e35332: f64 = (-10000.0);
        let assign18720_e35334: f64 = (assign18720_e35332 * 0.0001);
        let (assign18720_e35490, assign18720_e35490_d_n0, assign18720_e35490_d_n2, assign18720_e35490_d_n3, assign18720_e35490_d_n4, assign18720_e35490_d_n5, assign18720_e35490_d_n6, assign18720_e35490_d_n7, assign18720_e35490_d_n8, assign18720_e35490_d_n9, assign18720_e35490_d_n10, assign18720_e35490_d_n11, assign18720_e35490_d_n13, assign18720_e35490_d_n14,) = {
            if (!(assign18720_e35330 < assign18720_e35334)) {
                let assign18720_e35340: f64 = (0.5 * var_eg);
                let assign18720_e35344: f64 = (p.p145 / var_ni);
                let (assign18720_e35361, assign18720_e35361_d_n0, assign18720_e35361_d_n2, assign18720_e35361_d_n3, assign18720_e35361_d_n4, assign18720_e35361_d_n5, assign18720_e35361_d_n6, assign18720_e35361_d_n7, assign18720_e35361_d_n8, assign18720_e35361_d_n9, assign18720_e35361_d_n10, assign18720_e35361_d_n11, assign18720_e35361_d_n13, assign18720_e35361_d_n14,) = {
                    if (!(assign18720_e35344 > 1e-38)) {
                        let assign18720_e35349: f64 = (-87.498233534);
                        (assign18720_e35349, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18720_e35352: f64 = (p.p145 / var_ni);
                        let (assign18720_e35360, assign18720_e35360_d_n0, assign18720_e35360_d_n2, assign18720_e35360_d_n3, assign18720_e35360_d_n4, assign18720_e35360_d_n5, assign18720_e35360_d_n6, assign18720_e35360_d_n7, assign18720_e35360_d_n8, assign18720_e35360_d_n9, assign18720_e35360_d_n10, assign18720_e35360_d_n11, assign18720_e35360_d_n13, assign18720_e35360_d_n14,) = {
                            if (assign18720_e35352 > 1e-38) {
                                let assign18720_e35357: f64 = (p.p145 / var_ni);
                                let assign18720_e35358: f64 = (assign18720_e35357).ln();
                                (assign18720_e35358, ((-((p.p145 * var_ni_dn0) / (var_ni * var_ni))) / assign18720_e35357), ((-((p.p145 * var_ni_dn2) / (var_ni * var_ni))) / assign18720_e35357), ((-((p.p145 * var_ni_dn3) / (var_ni * var_ni))) / assign18720_e35357), ((-((p.p145 * var_ni_dn4) / (var_ni * var_ni))) / assign18720_e35357), ((-((p.p145 * var_ni_dn5) / (var_ni * var_ni))) / assign18720_e35357), ((-((p.p145 * var_ni_dn6) / (var_ni * var_ni))) / assign18720_e35357), ((-((p.p145 * var_ni_dn7) / (var_ni * var_ni))) / assign18720_e35357), ((-((p.p145 * var_ni_dn8) / (var_ni * var_ni))) / assign18720_e35357), ((-((p.p145 * var_ni_dn9) / (var_ni * var_ni))) / assign18720_e35357), ((-((p.p145 * var_ni_dn10) / (var_ni * var_ni))) / assign18720_e35357), ((-((p.p145 * var_ni_dn11) / (var_ni * var_ni))) / assign18720_e35357), ((-((p.p145 * var_ni_dn13) / (var_ni * var_ni))) / assign18720_e35357), ((-((p.p145 * var_ni_dn14) / (var_ni * var_ni))) / assign18720_e35357),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18720_e35360, assign18720_e35360_d_n0, assign18720_e35360_d_n2, assign18720_e35360_d_n3, assign18720_e35360_d_n4, assign18720_e35360_d_n5, assign18720_e35360_d_n6, assign18720_e35360_d_n7, assign18720_e35360_d_n8, assign18720_e35360_d_n9, assign18720_e35360_d_n10, assign18720_e35360_d_n11, assign18720_e35360_d_n13, assign18720_e35360_d_n14,)
                    }
                };
                let assign18720_e35362: f64 = (var_vtm * assign18720_e35361);
                let assign18720_e35363: f64 = (assign18720_e35340 - assign18720_e35362);
                let assign18720_e35366: f64 = (0.5 * var_eg);
                let assign18720_e35370: f64 = (p.p145 / var_ni);
                let (assign18720_e35387, assign18720_e35387_d_n0, assign18720_e35387_d_n2, assign18720_e35387_d_n3, assign18720_e35387_d_n4, assign18720_e35387_d_n5, assign18720_e35387_d_n6, assign18720_e35387_d_n7, assign18720_e35387_d_n8, assign18720_e35387_d_n9, assign18720_e35387_d_n10, assign18720_e35387_d_n11, assign18720_e35387_d_n13, assign18720_e35387_d_n14,) = {
                    if (!(assign18720_e35370 > 1e-38)) {
                        let assign18720_e35375: f64 = (-87.498233534);
                        (assign18720_e35375, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18720_e35378: f64 = (p.p145 / var_ni);
                        let (assign18720_e35386, assign18720_e35386_d_n0, assign18720_e35386_d_n2, assign18720_e35386_d_n3, assign18720_e35386_d_n4, assign18720_e35386_d_n5, assign18720_e35386_d_n6, assign18720_e35386_d_n7, assign18720_e35386_d_n8, assign18720_e35386_d_n9, assign18720_e35386_d_n10, assign18720_e35386_d_n11, assign18720_e35386_d_n13, assign18720_e35386_d_n14,) = {
                            if (assign18720_e35378 > 1e-38) {
                                let assign18720_e35383: f64 = (p.p145 / var_ni);
                                let assign18720_e35384: f64 = (assign18720_e35383).ln();
                                (assign18720_e35384, ((-((p.p145 * var_ni_dn0) / (var_ni * var_ni))) / assign18720_e35383), ((-((p.p145 * var_ni_dn2) / (var_ni * var_ni))) / assign18720_e35383), ((-((p.p145 * var_ni_dn3) / (var_ni * var_ni))) / assign18720_e35383), ((-((p.p145 * var_ni_dn4) / (var_ni * var_ni))) / assign18720_e35383), ((-((p.p145 * var_ni_dn5) / (var_ni * var_ni))) / assign18720_e35383), ((-((p.p145 * var_ni_dn6) / (var_ni * var_ni))) / assign18720_e35383), ((-((p.p145 * var_ni_dn7) / (var_ni * var_ni))) / assign18720_e35383), ((-((p.p145 * var_ni_dn8) / (var_ni * var_ni))) / assign18720_e35383), ((-((p.p145 * var_ni_dn9) / (var_ni * var_ni))) / assign18720_e35383), ((-((p.p145 * var_ni_dn10) / (var_ni * var_ni))) / assign18720_e35383), ((-((p.p145 * var_ni_dn11) / (var_ni * var_ni))) / assign18720_e35383), ((-((p.p145 * var_ni_dn13) / (var_ni * var_ni))) / assign18720_e35383), ((-((p.p145 * var_ni_dn14) / (var_ni * var_ni))) / assign18720_e35383),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18720_e35386, assign18720_e35386_d_n0, assign18720_e35386_d_n2, assign18720_e35386_d_n3, assign18720_e35386_d_n4, assign18720_e35386_d_n5, assign18720_e35386_d_n6, assign18720_e35386_d_n7, assign18720_e35386_d_n8, assign18720_e35386_d_n9, assign18720_e35386_d_n10, assign18720_e35386_d_n11, assign18720_e35386_d_n13, assign18720_e35386_d_n14,)
                    }
                };
                let assign18720_e35388: f64 = (var_vtm * assign18720_e35387);
                let assign18720_e35389: f64 = (assign18720_e35366 - assign18720_e35388);
                let assign18720_e35392: f64 = (0.5 * var_eg);
                let assign18720_e35396: f64 = (p.p145 / var_ni);
                let (assign18720_e35413, assign18720_e35413_d_n0, assign18720_e35413_d_n2, assign18720_e35413_d_n3, assign18720_e35413_d_n4, assign18720_e35413_d_n5, assign18720_e35413_d_n6, assign18720_e35413_d_n7, assign18720_e35413_d_n8, assign18720_e35413_d_n9, assign18720_e35413_d_n10, assign18720_e35413_d_n11, assign18720_e35413_d_n13, assign18720_e35413_d_n14,) = {
                    if (!(assign18720_e35396 > 1e-38)) {
                        let assign18720_e35401: f64 = (-87.498233534);
                        (assign18720_e35401, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18720_e35404: f64 = (p.p145 / var_ni);
                        let (assign18720_e35412, assign18720_e35412_d_n0, assign18720_e35412_d_n2, assign18720_e35412_d_n3, assign18720_e35412_d_n4, assign18720_e35412_d_n5, assign18720_e35412_d_n6, assign18720_e35412_d_n7, assign18720_e35412_d_n8, assign18720_e35412_d_n9, assign18720_e35412_d_n10, assign18720_e35412_d_n11, assign18720_e35412_d_n13, assign18720_e35412_d_n14,) = {
                            if (assign18720_e35404 > 1e-38) {
                                let assign18720_e35409: f64 = (p.p145 / var_ni);
                                let assign18720_e35410: f64 = (assign18720_e35409).ln();
                                (assign18720_e35410, ((-((p.p145 * var_ni_dn0) / (var_ni * var_ni))) / assign18720_e35409), ((-((p.p145 * var_ni_dn2) / (var_ni * var_ni))) / assign18720_e35409), ((-((p.p145 * var_ni_dn3) / (var_ni * var_ni))) / assign18720_e35409), ((-((p.p145 * var_ni_dn4) / (var_ni * var_ni))) / assign18720_e35409), ((-((p.p145 * var_ni_dn5) / (var_ni * var_ni))) / assign18720_e35409), ((-((p.p145 * var_ni_dn6) / (var_ni * var_ni))) / assign18720_e35409), ((-((p.p145 * var_ni_dn7) / (var_ni * var_ni))) / assign18720_e35409), ((-((p.p145 * var_ni_dn8) / (var_ni * var_ni))) / assign18720_e35409), ((-((p.p145 * var_ni_dn9) / (var_ni * var_ni))) / assign18720_e35409), ((-((p.p145 * var_ni_dn10) / (var_ni * var_ni))) / assign18720_e35409), ((-((p.p145 * var_ni_dn11) / (var_ni * var_ni))) / assign18720_e35409), ((-((p.p145 * var_ni_dn13) / (var_ni * var_ni))) / assign18720_e35409), ((-((p.p145 * var_ni_dn14) / (var_ni * var_ni))) / assign18720_e35409),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18720_e35412, assign18720_e35412_d_n0, assign18720_e35412_d_n2, assign18720_e35412_d_n3, assign18720_e35412_d_n4, assign18720_e35412_d_n5, assign18720_e35412_d_n6, assign18720_e35412_d_n7, assign18720_e35412_d_n8, assign18720_e35412_d_n9, assign18720_e35412_d_n10, assign18720_e35412_d_n11, assign18720_e35412_d_n13, assign18720_e35412_d_n14,)
                    }
                };
                let assign18720_e35414: f64 = (var_vtm * assign18720_e35413);
                let assign18720_e35415: f64 = (assign18720_e35392 - assign18720_e35414);
                let assign18720_e35416: f64 = (assign18720_e35389 * assign18720_e35415);
                let assign18720_e35419: f64 = (4.0 * 0.0001);
                let assign18720_e35421: f64 = (assign18720_e35419 * 0.0001);
                let assign18720_e35422: f64 = (assign18720_e35416 + assign18720_e35421);
                let assign18720_e35423: f64 = (assign18720_e35422).sqrt();
                let assign18720_e35424: f64 = (assign18720_e35363 + assign18720_e35423);
                let assign18720_e35425: f64 = (0.5 * assign18720_e35424);
                (assign18720_e35425, (0.5 * ((-(var_vtm * assign18720_e35361_d_n0)) + ((((-(var_vtm * assign18720_e35387_d_n0)) * assign18720_e35415) + (assign18720_e35389 * (-(var_vtm * assign18720_e35413_d_n0)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(var_vtm * assign18720_e35361_d_n2)) + ((((-(var_vtm * assign18720_e35387_d_n2)) * assign18720_e35415) + (assign18720_e35389 * (-(var_vtm * assign18720_e35413_d_n2)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(var_vtm * assign18720_e35361_d_n3)) + ((((-(var_vtm * assign18720_e35387_d_n3)) * assign18720_e35415) + (assign18720_e35389 * (-(var_vtm * assign18720_e35413_d_n3)))) / (2.0 * assign18720_e35423)))), (0.5 * (((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18720_e35361) + (var_vtm * assign18720_e35361_d_n4))) + (((((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18720_e35387) + (var_vtm * assign18720_e35387_d_n4))) * assign18720_e35415) + (assign18720_e35389 * ((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18720_e35413) + (var_vtm * assign18720_e35413_d_n4))))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(var_vtm * assign18720_e35361_d_n5)) + ((((-(var_vtm * assign18720_e35387_d_n5)) * assign18720_e35415) + (assign18720_e35389 * (-(var_vtm * assign18720_e35413_d_n5)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(var_vtm * assign18720_e35361_d_n6)) + ((((-(var_vtm * assign18720_e35387_d_n6)) * assign18720_e35415) + (assign18720_e35389 * (-(var_vtm * assign18720_e35413_d_n6)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(var_vtm * assign18720_e35361_d_n7)) + ((((-(var_vtm * assign18720_e35387_d_n7)) * assign18720_e35415) + (assign18720_e35389 * (-(var_vtm * assign18720_e35413_d_n7)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(var_vtm * assign18720_e35361_d_n8)) + ((((-(var_vtm * assign18720_e35387_d_n8)) * assign18720_e35415) + (assign18720_e35389 * (-(var_vtm * assign18720_e35413_d_n8)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(var_vtm * assign18720_e35361_d_n9)) + ((((-(var_vtm * assign18720_e35387_d_n9)) * assign18720_e35415) + (assign18720_e35389 * (-(var_vtm * assign18720_e35413_d_n9)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(var_vtm * assign18720_e35361_d_n10)) + ((((-(var_vtm * assign18720_e35387_d_n10)) * assign18720_e35415) + (assign18720_e35389 * (-(var_vtm * assign18720_e35413_d_n10)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(var_vtm * assign18720_e35361_d_n11)) + ((((-(var_vtm * assign18720_e35387_d_n11)) * assign18720_e35415) + (assign18720_e35389 * (-(var_vtm * assign18720_e35413_d_n11)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(var_vtm * assign18720_e35361_d_n13)) + ((((-(var_vtm * assign18720_e35387_d_n13)) * assign18720_e35415) + (assign18720_e35389 * (-(var_vtm * assign18720_e35413_d_n13)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(var_vtm * assign18720_e35361_d_n14)) + ((((-(var_vtm * assign18720_e35387_d_n14)) * assign18720_e35415) + (assign18720_e35389 * (-(var_vtm * assign18720_e35413_d_n14)))) / (2.0 * assign18720_e35423)))),)
            } else {
                let assign18720_e35428: f64 = (0.5 * var_eg);
                let assign18720_e35432: f64 = (p.p145 / var_ni);
                let (assign18720_e35449, assign18720_e35449_d_n0, assign18720_e35449_d_n2, assign18720_e35449_d_n3, assign18720_e35449_d_n4, assign18720_e35449_d_n5, assign18720_e35449_d_n6, assign18720_e35449_d_n7, assign18720_e35449_d_n8, assign18720_e35449_d_n9, assign18720_e35449_d_n10, assign18720_e35449_d_n11, assign18720_e35449_d_n13, assign18720_e35449_d_n14,) = {
                    if (!(assign18720_e35432 > 1e-38)) {
                        let assign18720_e35437: f64 = (-87.498233534);
                        (assign18720_e35437, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18720_e35440: f64 = (p.p145 / var_ni);
                        let (assign18720_e35448, assign18720_e35448_d_n0, assign18720_e35448_d_n2, assign18720_e35448_d_n3, assign18720_e35448_d_n4, assign18720_e35448_d_n5, assign18720_e35448_d_n6, assign18720_e35448_d_n7, assign18720_e35448_d_n8, assign18720_e35448_d_n9, assign18720_e35448_d_n10, assign18720_e35448_d_n11, assign18720_e35448_d_n13, assign18720_e35448_d_n14,) = {
                            if (assign18720_e35440 > 1e-38) {
                                let assign18720_e35445: f64 = (p.p145 / var_ni);
                                let assign18720_e35446: f64 = (assign18720_e35445).ln();
                                (assign18720_e35446, ((-((p.p145 * var_ni_dn0) / (var_ni * var_ni))) / assign18720_e35445), ((-((p.p145 * var_ni_dn2) / (var_ni * var_ni))) / assign18720_e35445), ((-((p.p145 * var_ni_dn3) / (var_ni * var_ni))) / assign18720_e35445), ((-((p.p145 * var_ni_dn4) / (var_ni * var_ni))) / assign18720_e35445), ((-((p.p145 * var_ni_dn5) / (var_ni * var_ni))) / assign18720_e35445), ((-((p.p145 * var_ni_dn6) / (var_ni * var_ni))) / assign18720_e35445), ((-((p.p145 * var_ni_dn7) / (var_ni * var_ni))) / assign18720_e35445), ((-((p.p145 * var_ni_dn8) / (var_ni * var_ni))) / assign18720_e35445), ((-((p.p145 * var_ni_dn9) / (var_ni * var_ni))) / assign18720_e35445), ((-((p.p145 * var_ni_dn10) / (var_ni * var_ni))) / assign18720_e35445), ((-((p.p145 * var_ni_dn11) / (var_ni * var_ni))) / assign18720_e35445), ((-((p.p145 * var_ni_dn13) / (var_ni * var_ni))) / assign18720_e35445), ((-((p.p145 * var_ni_dn14) / (var_ni * var_ni))) / assign18720_e35445),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18720_e35448, assign18720_e35448_d_n0, assign18720_e35448_d_n2, assign18720_e35448_d_n3, assign18720_e35448_d_n4, assign18720_e35448_d_n5, assign18720_e35448_d_n6, assign18720_e35448_d_n7, assign18720_e35448_d_n8, assign18720_e35448_d_n9, assign18720_e35448_d_n10, assign18720_e35448_d_n11, assign18720_e35448_d_n13, assign18720_e35448_d_n14,)
                    }
                };
                let assign18720_e35450: f64 = (var_vtm * assign18720_e35449);
                let assign18720_e35451: f64 = (assign18720_e35428 - assign18720_e35450);
                let assign18720_e35453: f64 = (-10000.0);
                let assign18720_e35455: f64 = (assign18720_e35453 * 0.0001);
                let (assign18720_e35489, assign18720_e35489_d_n0, assign18720_e35489_d_n2, assign18720_e35489_d_n3, assign18720_e35489_d_n4, assign18720_e35489_d_n5, assign18720_e35489_d_n6, assign18720_e35489_d_n7, assign18720_e35489_d_n8, assign18720_e35489_d_n9, assign18720_e35489_d_n10, assign18720_e35489_d_n11, assign18720_e35489_d_n13, assign18720_e35489_d_n14,) = {
                    if (assign18720_e35451 < assign18720_e35455) {
                        let assign18720_e35458: f64 = (-0.0001);
                        let assign18720_e35460: f64 = (assign18720_e35458 * 0.0001);
                        let assign18720_e35463: f64 = (0.5 * var_eg);
                        let assign18720_e35467: f64 = (p.p145 / var_ni);
                        let (assign18720_e35484, assign18720_e35484_d_n0, assign18720_e35484_d_n2, assign18720_e35484_d_n3, assign18720_e35484_d_n4, assign18720_e35484_d_n5, assign18720_e35484_d_n6, assign18720_e35484_d_n7, assign18720_e35484_d_n8, assign18720_e35484_d_n9, assign18720_e35484_d_n10, assign18720_e35484_d_n11, assign18720_e35484_d_n13, assign18720_e35484_d_n14,) = {
                            if (!(assign18720_e35467 > 1e-38)) {
                                let assign18720_e35472: f64 = (-87.498233534);
                                (assign18720_e35472, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            } else {
                                let assign18720_e35475: f64 = (p.p145 / var_ni);
                                let (assign18720_e35483, assign18720_e35483_d_n0, assign18720_e35483_d_n2, assign18720_e35483_d_n3, assign18720_e35483_d_n4, assign18720_e35483_d_n5, assign18720_e35483_d_n6, assign18720_e35483_d_n7, assign18720_e35483_d_n8, assign18720_e35483_d_n9, assign18720_e35483_d_n10, assign18720_e35483_d_n11, assign18720_e35483_d_n13, assign18720_e35483_d_n14,) = {
                                    if (assign18720_e35475 > 1e-38) {
                                        let assign18720_e35480: f64 = (p.p145 / var_ni);
                                        let assign18720_e35481: f64 = (assign18720_e35480).ln();
                                        (assign18720_e35481, ((-((p.p145 * var_ni_dn0) / (var_ni * var_ni))) / assign18720_e35480), ((-((p.p145 * var_ni_dn2) / (var_ni * var_ni))) / assign18720_e35480), ((-((p.p145 * var_ni_dn3) / (var_ni * var_ni))) / assign18720_e35480), ((-((p.p145 * var_ni_dn4) / (var_ni * var_ni))) / assign18720_e35480), ((-((p.p145 * var_ni_dn5) / (var_ni * var_ni))) / assign18720_e35480), ((-((p.p145 * var_ni_dn6) / (var_ni * var_ni))) / assign18720_e35480), ((-((p.p145 * var_ni_dn7) / (var_ni * var_ni))) / assign18720_e35480), ((-((p.p145 * var_ni_dn8) / (var_ni * var_ni))) / assign18720_e35480), ((-((p.p145 * var_ni_dn9) / (var_ni * var_ni))) / assign18720_e35480), ((-((p.p145 * var_ni_dn10) / (var_ni * var_ni))) / assign18720_e35480), ((-((p.p145 * var_ni_dn11) / (var_ni * var_ni))) / assign18720_e35480), ((-((p.p145 * var_ni_dn13) / (var_ni * var_ni))) / assign18720_e35480), ((-((p.p145 * var_ni_dn14) / (var_ni * var_ni))) / assign18720_e35480),)
                                    } else {
                                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                                    }
                                };
                                (assign18720_e35483, assign18720_e35483_d_n0, assign18720_e35483_d_n2, assign18720_e35483_d_n3, assign18720_e35483_d_n4, assign18720_e35483_d_n5, assign18720_e35483_d_n6, assign18720_e35483_d_n7, assign18720_e35483_d_n8, assign18720_e35483_d_n9, assign18720_e35483_d_n10, assign18720_e35483_d_n11, assign18720_e35483_d_n13, assign18720_e35483_d_n14,)
                            }
                        };
                        let assign18720_e35485: f64 = (var_vtm * assign18720_e35484);
                        let assign18720_e35486: f64 = (assign18720_e35463 - assign18720_e35485);
                        let assign18720_e35487: f64 = (assign18720_e35460 / assign18720_e35486);
                        (assign18720_e35487, (-((assign18720_e35460 * (-(var_vtm * assign18720_e35484_d_n0))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(var_vtm * assign18720_e35484_d_n2))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(var_vtm * assign18720_e35484_d_n3))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * ((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18720_e35484) + (var_vtm * assign18720_e35484_d_n4)))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(var_vtm * assign18720_e35484_d_n5))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(var_vtm * assign18720_e35484_d_n6))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(var_vtm * assign18720_e35484_d_n7))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(var_vtm * assign18720_e35484_d_n8))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(var_vtm * assign18720_e35484_d_n9))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(var_vtm * assign18720_e35484_d_n10))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(var_vtm * assign18720_e35484_d_n11))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(var_vtm * assign18720_e35484_d_n13))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(var_vtm * assign18720_e35484_d_n14))) / (assign18720_e35486 * assign18720_e35486))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18720_e35489, assign18720_e35489_d_n0, assign18720_e35489_d_n2, assign18720_e35489_d_n3, assign18720_e35489_d_n4, assign18720_e35489_d_n5, assign18720_e35489_d_n6, assign18720_e35489_d_n7, assign18720_e35489_d_n8, assign18720_e35489_d_n9, assign18720_e35489_d_n10, assign18720_e35489_d_n11, assign18720_e35489_d_n13, assign18720_e35489_d_n14,)
            }
        };
        let assign18720_e35493: f64 = (0.5 * var_eg);
        let assign18720_e35497: f64 = (0.5 * var_eg);
        let assign18720_e35500: f64 = (0.5 * var_eg);
        let assign18720_e35504: f64 = (p.p97 / var_ni);
        let (assign18720_e35521, assign18720_e35521_d_n0, assign18720_e35521_d_n2, assign18720_e35521_d_n3, assign18720_e35521_d_n4, assign18720_e35521_d_n5, assign18720_e35521_d_n6, assign18720_e35521_d_n7, assign18720_e35521_d_n8, assign18720_e35521_d_n9, assign18720_e35521_d_n10, assign18720_e35521_d_n11, assign18720_e35521_d_n13, assign18720_e35521_d_n14,) = {
            if (!(assign18720_e35504 > 1e-38)) {
                let assign18720_e35509: f64 = (-87.498233534);
                (assign18720_e35509, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign18720_e35512: f64 = (p.p97 / var_ni);
                let (assign18720_e35520, assign18720_e35520_d_n0, assign18720_e35520_d_n2, assign18720_e35520_d_n3, assign18720_e35520_d_n4, assign18720_e35520_d_n5, assign18720_e35520_d_n6, assign18720_e35520_d_n7, assign18720_e35520_d_n8, assign18720_e35520_d_n9, assign18720_e35520_d_n10, assign18720_e35520_d_n11, assign18720_e35520_d_n13, assign18720_e35520_d_n14,) = {
                    if (assign18720_e35512 > 1e-38) {
                        let assign18720_e35517: f64 = (p.p97 / var_ni);
                        let assign18720_e35518: f64 = (assign18720_e35517).ln();
                        (assign18720_e35518, ((-((p.p97 * var_ni_dn0) / (var_ni * var_ni))) / assign18720_e35517), ((-((p.p97 * var_ni_dn2) / (var_ni * var_ni))) / assign18720_e35517), ((-((p.p97 * var_ni_dn3) / (var_ni * var_ni))) / assign18720_e35517), ((-((p.p97 * var_ni_dn4) / (var_ni * var_ni))) / assign18720_e35517), ((-((p.p97 * var_ni_dn5) / (var_ni * var_ni))) / assign18720_e35517), ((-((p.p97 * var_ni_dn6) / (var_ni * var_ni))) / assign18720_e35517), ((-((p.p97 * var_ni_dn7) / (var_ni * var_ni))) / assign18720_e35517), ((-((p.p97 * var_ni_dn8) / (var_ni * var_ni))) / assign18720_e35517), ((-((p.p97 * var_ni_dn9) / (var_ni * var_ni))) / assign18720_e35517), ((-((p.p97 * var_ni_dn10) / (var_ni * var_ni))) / assign18720_e35517), ((-((p.p97 * var_ni_dn11) / (var_ni * var_ni))) / assign18720_e35517), ((-((p.p97 * var_ni_dn13) / (var_ni * var_ni))) / assign18720_e35517), ((-((p.p97 * var_ni_dn14) / (var_ni * var_ni))) / assign18720_e35517),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18720_e35520, assign18720_e35520_d_n0, assign18720_e35520_d_n2, assign18720_e35520_d_n3, assign18720_e35520_d_n4, assign18720_e35520_d_n5, assign18720_e35520_d_n6, assign18720_e35520_d_n7, assign18720_e35520_d_n8, assign18720_e35520_d_n9, assign18720_e35520_d_n10, assign18720_e35520_d_n11, assign18720_e35520_d_n13, assign18720_e35520_d_n14,)
            }
        };
        let assign18720_e35522: f64 = (var_vtm * assign18720_e35521);
        let assign18720_e35523: f64 = (assign18720_e35500 - assign18720_e35522);
        let assign18720_e35525: f64 = (-10000.0);
        let assign18720_e35527: f64 = (assign18720_e35525 * 0.0001);
        let (assign18720_e35683, assign18720_e35683_d_n0, assign18720_e35683_d_n2, assign18720_e35683_d_n3, assign18720_e35683_d_n4, assign18720_e35683_d_n5, assign18720_e35683_d_n6, assign18720_e35683_d_n7, assign18720_e35683_d_n8, assign18720_e35683_d_n9, assign18720_e35683_d_n10, assign18720_e35683_d_n11, assign18720_e35683_d_n13, assign18720_e35683_d_n14,) = {
            if (!(assign18720_e35523 < assign18720_e35527)) {
                let assign18720_e35533: f64 = (0.5 * var_eg);
                let assign18720_e35537: f64 = (p.p97 / var_ni);
                let (assign18720_e35554, assign18720_e35554_d_n0, assign18720_e35554_d_n2, assign18720_e35554_d_n3, assign18720_e35554_d_n4, assign18720_e35554_d_n5, assign18720_e35554_d_n6, assign18720_e35554_d_n7, assign18720_e35554_d_n8, assign18720_e35554_d_n9, assign18720_e35554_d_n10, assign18720_e35554_d_n11, assign18720_e35554_d_n13, assign18720_e35554_d_n14,) = {
                    if (!(assign18720_e35537 > 1e-38)) {
                        let assign18720_e35542: f64 = (-87.498233534);
                        (assign18720_e35542, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18720_e35545: f64 = (p.p97 / var_ni);
                        let (assign18720_e35553, assign18720_e35553_d_n0, assign18720_e35553_d_n2, assign18720_e35553_d_n3, assign18720_e35553_d_n4, assign18720_e35553_d_n5, assign18720_e35553_d_n6, assign18720_e35553_d_n7, assign18720_e35553_d_n8, assign18720_e35553_d_n9, assign18720_e35553_d_n10, assign18720_e35553_d_n11, assign18720_e35553_d_n13, assign18720_e35553_d_n14,) = {
                            if (assign18720_e35545 > 1e-38) {
                                let assign18720_e35550: f64 = (p.p97 / var_ni);
                                let assign18720_e35551: f64 = (assign18720_e35550).ln();
                                (assign18720_e35551, ((-((p.p97 * var_ni_dn0) / (var_ni * var_ni))) / assign18720_e35550), ((-((p.p97 * var_ni_dn2) / (var_ni * var_ni))) / assign18720_e35550), ((-((p.p97 * var_ni_dn3) / (var_ni * var_ni))) / assign18720_e35550), ((-((p.p97 * var_ni_dn4) / (var_ni * var_ni))) / assign18720_e35550), ((-((p.p97 * var_ni_dn5) / (var_ni * var_ni))) / assign18720_e35550), ((-((p.p97 * var_ni_dn6) / (var_ni * var_ni))) / assign18720_e35550), ((-((p.p97 * var_ni_dn7) / (var_ni * var_ni))) / assign18720_e35550), ((-((p.p97 * var_ni_dn8) / (var_ni * var_ni))) / assign18720_e35550), ((-((p.p97 * var_ni_dn9) / (var_ni * var_ni))) / assign18720_e35550), ((-((p.p97 * var_ni_dn10) / (var_ni * var_ni))) / assign18720_e35550), ((-((p.p97 * var_ni_dn11) / (var_ni * var_ni))) / assign18720_e35550), ((-((p.p97 * var_ni_dn13) / (var_ni * var_ni))) / assign18720_e35550), ((-((p.p97 * var_ni_dn14) / (var_ni * var_ni))) / assign18720_e35550),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18720_e35553, assign18720_e35553_d_n0, assign18720_e35553_d_n2, assign18720_e35553_d_n3, assign18720_e35553_d_n4, assign18720_e35553_d_n5, assign18720_e35553_d_n6, assign18720_e35553_d_n7, assign18720_e35553_d_n8, assign18720_e35553_d_n9, assign18720_e35553_d_n10, assign18720_e35553_d_n11, assign18720_e35553_d_n13, assign18720_e35553_d_n14,)
                    }
                };
                let assign18720_e35555: f64 = (var_vtm * assign18720_e35554);
                let assign18720_e35556: f64 = (assign18720_e35533 - assign18720_e35555);
                let assign18720_e35559: f64 = (0.5 * var_eg);
                let assign18720_e35563: f64 = (p.p97 / var_ni);
                let (assign18720_e35580, assign18720_e35580_d_n0, assign18720_e35580_d_n2, assign18720_e35580_d_n3, assign18720_e35580_d_n4, assign18720_e35580_d_n5, assign18720_e35580_d_n6, assign18720_e35580_d_n7, assign18720_e35580_d_n8, assign18720_e35580_d_n9, assign18720_e35580_d_n10, assign18720_e35580_d_n11, assign18720_e35580_d_n13, assign18720_e35580_d_n14,) = {
                    if (!(assign18720_e35563 > 1e-38)) {
                        let assign18720_e35568: f64 = (-87.498233534);
                        (assign18720_e35568, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18720_e35571: f64 = (p.p97 / var_ni);
                        let (assign18720_e35579, assign18720_e35579_d_n0, assign18720_e35579_d_n2, assign18720_e35579_d_n3, assign18720_e35579_d_n4, assign18720_e35579_d_n5, assign18720_e35579_d_n6, assign18720_e35579_d_n7, assign18720_e35579_d_n8, assign18720_e35579_d_n9, assign18720_e35579_d_n10, assign18720_e35579_d_n11, assign18720_e35579_d_n13, assign18720_e35579_d_n14,) = {
                            if (assign18720_e35571 > 1e-38) {
                                let assign18720_e35576: f64 = (p.p97 / var_ni);
                                let assign18720_e35577: f64 = (assign18720_e35576).ln();
                                (assign18720_e35577, ((-((p.p97 * var_ni_dn0) / (var_ni * var_ni))) / assign18720_e35576), ((-((p.p97 * var_ni_dn2) / (var_ni * var_ni))) / assign18720_e35576), ((-((p.p97 * var_ni_dn3) / (var_ni * var_ni))) / assign18720_e35576), ((-((p.p97 * var_ni_dn4) / (var_ni * var_ni))) / assign18720_e35576), ((-((p.p97 * var_ni_dn5) / (var_ni * var_ni))) / assign18720_e35576), ((-((p.p97 * var_ni_dn6) / (var_ni * var_ni))) / assign18720_e35576), ((-((p.p97 * var_ni_dn7) / (var_ni * var_ni))) / assign18720_e35576), ((-((p.p97 * var_ni_dn8) / (var_ni * var_ni))) / assign18720_e35576), ((-((p.p97 * var_ni_dn9) / (var_ni * var_ni))) / assign18720_e35576), ((-((p.p97 * var_ni_dn10) / (var_ni * var_ni))) / assign18720_e35576), ((-((p.p97 * var_ni_dn11) / (var_ni * var_ni))) / assign18720_e35576), ((-((p.p97 * var_ni_dn13) / (var_ni * var_ni))) / assign18720_e35576), ((-((p.p97 * var_ni_dn14) / (var_ni * var_ni))) / assign18720_e35576),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18720_e35579, assign18720_e35579_d_n0, assign18720_e35579_d_n2, assign18720_e35579_d_n3, assign18720_e35579_d_n4, assign18720_e35579_d_n5, assign18720_e35579_d_n6, assign18720_e35579_d_n7, assign18720_e35579_d_n8, assign18720_e35579_d_n9, assign18720_e35579_d_n10, assign18720_e35579_d_n11, assign18720_e35579_d_n13, assign18720_e35579_d_n14,)
                    }
                };
                let assign18720_e35581: f64 = (var_vtm * assign18720_e35580);
                let assign18720_e35582: f64 = (assign18720_e35559 - assign18720_e35581);
                let assign18720_e35585: f64 = (0.5 * var_eg);
                let assign18720_e35589: f64 = (p.p97 / var_ni);
                let (assign18720_e35606, assign18720_e35606_d_n0, assign18720_e35606_d_n2, assign18720_e35606_d_n3, assign18720_e35606_d_n4, assign18720_e35606_d_n5, assign18720_e35606_d_n6, assign18720_e35606_d_n7, assign18720_e35606_d_n8, assign18720_e35606_d_n9, assign18720_e35606_d_n10, assign18720_e35606_d_n11, assign18720_e35606_d_n13, assign18720_e35606_d_n14,) = {
                    if (!(assign18720_e35589 > 1e-38)) {
                        let assign18720_e35594: f64 = (-87.498233534);
                        (assign18720_e35594, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18720_e35597: f64 = (p.p97 / var_ni);
                        let (assign18720_e35605, assign18720_e35605_d_n0, assign18720_e35605_d_n2, assign18720_e35605_d_n3, assign18720_e35605_d_n4, assign18720_e35605_d_n5, assign18720_e35605_d_n6, assign18720_e35605_d_n7, assign18720_e35605_d_n8, assign18720_e35605_d_n9, assign18720_e35605_d_n10, assign18720_e35605_d_n11, assign18720_e35605_d_n13, assign18720_e35605_d_n14,) = {
                            if (assign18720_e35597 > 1e-38) {
                                let assign18720_e35602: f64 = (p.p97 / var_ni);
                                let assign18720_e35603: f64 = (assign18720_e35602).ln();
                                (assign18720_e35603, ((-((p.p97 * var_ni_dn0) / (var_ni * var_ni))) / assign18720_e35602), ((-((p.p97 * var_ni_dn2) / (var_ni * var_ni))) / assign18720_e35602), ((-((p.p97 * var_ni_dn3) / (var_ni * var_ni))) / assign18720_e35602), ((-((p.p97 * var_ni_dn4) / (var_ni * var_ni))) / assign18720_e35602), ((-((p.p97 * var_ni_dn5) / (var_ni * var_ni))) / assign18720_e35602), ((-((p.p97 * var_ni_dn6) / (var_ni * var_ni))) / assign18720_e35602), ((-((p.p97 * var_ni_dn7) / (var_ni * var_ni))) / assign18720_e35602), ((-((p.p97 * var_ni_dn8) / (var_ni * var_ni))) / assign18720_e35602), ((-((p.p97 * var_ni_dn9) / (var_ni * var_ni))) / assign18720_e35602), ((-((p.p97 * var_ni_dn10) / (var_ni * var_ni))) / assign18720_e35602), ((-((p.p97 * var_ni_dn11) / (var_ni * var_ni))) / assign18720_e35602), ((-((p.p97 * var_ni_dn13) / (var_ni * var_ni))) / assign18720_e35602), ((-((p.p97 * var_ni_dn14) / (var_ni * var_ni))) / assign18720_e35602),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18720_e35605, assign18720_e35605_d_n0, assign18720_e35605_d_n2, assign18720_e35605_d_n3, assign18720_e35605_d_n4, assign18720_e35605_d_n5, assign18720_e35605_d_n6, assign18720_e35605_d_n7, assign18720_e35605_d_n8, assign18720_e35605_d_n9, assign18720_e35605_d_n10, assign18720_e35605_d_n11, assign18720_e35605_d_n13, assign18720_e35605_d_n14,)
                    }
                };
                let assign18720_e35607: f64 = (var_vtm * assign18720_e35606);
                let assign18720_e35608: f64 = (assign18720_e35585 - assign18720_e35607);
                let assign18720_e35609: f64 = (assign18720_e35582 * assign18720_e35608);
                let assign18720_e35612: f64 = (4.0 * 0.0001);
                let assign18720_e35614: f64 = (assign18720_e35612 * 0.0001);
                let assign18720_e35615: f64 = (assign18720_e35609 + assign18720_e35614);
                let assign18720_e35616: f64 = (assign18720_e35615).sqrt();
                let assign18720_e35617: f64 = (assign18720_e35556 + assign18720_e35616);
                let assign18720_e35618: f64 = (0.5 * assign18720_e35617);
                (assign18720_e35618, (0.5 * ((-(var_vtm * assign18720_e35554_d_n0)) + ((((-(var_vtm * assign18720_e35580_d_n0)) * assign18720_e35608) + (assign18720_e35582 * (-(var_vtm * assign18720_e35606_d_n0)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(var_vtm * assign18720_e35554_d_n2)) + ((((-(var_vtm * assign18720_e35580_d_n2)) * assign18720_e35608) + (assign18720_e35582 * (-(var_vtm * assign18720_e35606_d_n2)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(var_vtm * assign18720_e35554_d_n3)) + ((((-(var_vtm * assign18720_e35580_d_n3)) * assign18720_e35608) + (assign18720_e35582 * (-(var_vtm * assign18720_e35606_d_n3)))) / (2.0 * assign18720_e35616)))), (0.5 * (((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18720_e35554) + (var_vtm * assign18720_e35554_d_n4))) + (((((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18720_e35580) + (var_vtm * assign18720_e35580_d_n4))) * assign18720_e35608) + (assign18720_e35582 * ((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18720_e35606) + (var_vtm * assign18720_e35606_d_n4))))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(var_vtm * assign18720_e35554_d_n5)) + ((((-(var_vtm * assign18720_e35580_d_n5)) * assign18720_e35608) + (assign18720_e35582 * (-(var_vtm * assign18720_e35606_d_n5)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(var_vtm * assign18720_e35554_d_n6)) + ((((-(var_vtm * assign18720_e35580_d_n6)) * assign18720_e35608) + (assign18720_e35582 * (-(var_vtm * assign18720_e35606_d_n6)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(var_vtm * assign18720_e35554_d_n7)) + ((((-(var_vtm * assign18720_e35580_d_n7)) * assign18720_e35608) + (assign18720_e35582 * (-(var_vtm * assign18720_e35606_d_n7)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(var_vtm * assign18720_e35554_d_n8)) + ((((-(var_vtm * assign18720_e35580_d_n8)) * assign18720_e35608) + (assign18720_e35582 * (-(var_vtm * assign18720_e35606_d_n8)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(var_vtm * assign18720_e35554_d_n9)) + ((((-(var_vtm * assign18720_e35580_d_n9)) * assign18720_e35608) + (assign18720_e35582 * (-(var_vtm * assign18720_e35606_d_n9)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(var_vtm * assign18720_e35554_d_n10)) + ((((-(var_vtm * assign18720_e35580_d_n10)) * assign18720_e35608) + (assign18720_e35582 * (-(var_vtm * assign18720_e35606_d_n10)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(var_vtm * assign18720_e35554_d_n11)) + ((((-(var_vtm * assign18720_e35580_d_n11)) * assign18720_e35608) + (assign18720_e35582 * (-(var_vtm * assign18720_e35606_d_n11)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(var_vtm * assign18720_e35554_d_n13)) + ((((-(var_vtm * assign18720_e35580_d_n13)) * assign18720_e35608) + (assign18720_e35582 * (-(var_vtm * assign18720_e35606_d_n13)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(var_vtm * assign18720_e35554_d_n14)) + ((((-(var_vtm * assign18720_e35580_d_n14)) * assign18720_e35608) + (assign18720_e35582 * (-(var_vtm * assign18720_e35606_d_n14)))) / (2.0 * assign18720_e35616)))),)
            } else {
                let assign18720_e35621: f64 = (0.5 * var_eg);
                let assign18720_e35625: f64 = (p.p97 / var_ni);
                let (assign18720_e35642, assign18720_e35642_d_n0, assign18720_e35642_d_n2, assign18720_e35642_d_n3, assign18720_e35642_d_n4, assign18720_e35642_d_n5, assign18720_e35642_d_n6, assign18720_e35642_d_n7, assign18720_e35642_d_n8, assign18720_e35642_d_n9, assign18720_e35642_d_n10, assign18720_e35642_d_n11, assign18720_e35642_d_n13, assign18720_e35642_d_n14,) = {
                    if (!(assign18720_e35625 > 1e-38)) {
                        let assign18720_e35630: f64 = (-87.498233534);
                        (assign18720_e35630, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18720_e35633: f64 = (p.p97 / var_ni);
                        let (assign18720_e35641, assign18720_e35641_d_n0, assign18720_e35641_d_n2, assign18720_e35641_d_n3, assign18720_e35641_d_n4, assign18720_e35641_d_n5, assign18720_e35641_d_n6, assign18720_e35641_d_n7, assign18720_e35641_d_n8, assign18720_e35641_d_n9, assign18720_e35641_d_n10, assign18720_e35641_d_n11, assign18720_e35641_d_n13, assign18720_e35641_d_n14,) = {
                            if (assign18720_e35633 > 1e-38) {
                                let assign18720_e35638: f64 = (p.p97 / var_ni);
                                let assign18720_e35639: f64 = (assign18720_e35638).ln();
                                (assign18720_e35639, ((-((p.p97 * var_ni_dn0) / (var_ni * var_ni))) / assign18720_e35638), ((-((p.p97 * var_ni_dn2) / (var_ni * var_ni))) / assign18720_e35638), ((-((p.p97 * var_ni_dn3) / (var_ni * var_ni))) / assign18720_e35638), ((-((p.p97 * var_ni_dn4) / (var_ni * var_ni))) / assign18720_e35638), ((-((p.p97 * var_ni_dn5) / (var_ni * var_ni))) / assign18720_e35638), ((-((p.p97 * var_ni_dn6) / (var_ni * var_ni))) / assign18720_e35638), ((-((p.p97 * var_ni_dn7) / (var_ni * var_ni))) / assign18720_e35638), ((-((p.p97 * var_ni_dn8) / (var_ni * var_ni))) / assign18720_e35638), ((-((p.p97 * var_ni_dn9) / (var_ni * var_ni))) / assign18720_e35638), ((-((p.p97 * var_ni_dn10) / (var_ni * var_ni))) / assign18720_e35638), ((-((p.p97 * var_ni_dn11) / (var_ni * var_ni))) / assign18720_e35638), ((-((p.p97 * var_ni_dn13) / (var_ni * var_ni))) / assign18720_e35638), ((-((p.p97 * var_ni_dn14) / (var_ni * var_ni))) / assign18720_e35638),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18720_e35641, assign18720_e35641_d_n0, assign18720_e35641_d_n2, assign18720_e35641_d_n3, assign18720_e35641_d_n4, assign18720_e35641_d_n5, assign18720_e35641_d_n6, assign18720_e35641_d_n7, assign18720_e35641_d_n8, assign18720_e35641_d_n9, assign18720_e35641_d_n10, assign18720_e35641_d_n11, assign18720_e35641_d_n13, assign18720_e35641_d_n14,)
                    }
                };
                let assign18720_e35643: f64 = (var_vtm * assign18720_e35642);
                let assign18720_e35644: f64 = (assign18720_e35621 - assign18720_e35643);
                let assign18720_e35646: f64 = (-10000.0);
                let assign18720_e35648: f64 = (assign18720_e35646 * 0.0001);
                let (assign18720_e35682, assign18720_e35682_d_n0, assign18720_e35682_d_n2, assign18720_e35682_d_n3, assign18720_e35682_d_n4, assign18720_e35682_d_n5, assign18720_e35682_d_n6, assign18720_e35682_d_n7, assign18720_e35682_d_n8, assign18720_e35682_d_n9, assign18720_e35682_d_n10, assign18720_e35682_d_n11, assign18720_e35682_d_n13, assign18720_e35682_d_n14,) = {
                    if (assign18720_e35644 < assign18720_e35648) {
                        let assign18720_e35651: f64 = (-0.0001);
                        let assign18720_e35653: f64 = (assign18720_e35651 * 0.0001);
                        let assign18720_e35656: f64 = (0.5 * var_eg);
                        let assign18720_e35660: f64 = (p.p97 / var_ni);
                        let (assign18720_e35677, assign18720_e35677_d_n0, assign18720_e35677_d_n2, assign18720_e35677_d_n3, assign18720_e35677_d_n4, assign18720_e35677_d_n5, assign18720_e35677_d_n6, assign18720_e35677_d_n7, assign18720_e35677_d_n8, assign18720_e35677_d_n9, assign18720_e35677_d_n10, assign18720_e35677_d_n11, assign18720_e35677_d_n13, assign18720_e35677_d_n14,) = {
                            if (!(assign18720_e35660 > 1e-38)) {
                                let assign18720_e35665: f64 = (-87.498233534);
                                (assign18720_e35665, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            } else {
                                let assign18720_e35668: f64 = (p.p97 / var_ni);
                                let (assign18720_e35676, assign18720_e35676_d_n0, assign18720_e35676_d_n2, assign18720_e35676_d_n3, assign18720_e35676_d_n4, assign18720_e35676_d_n5, assign18720_e35676_d_n6, assign18720_e35676_d_n7, assign18720_e35676_d_n8, assign18720_e35676_d_n9, assign18720_e35676_d_n10, assign18720_e35676_d_n11, assign18720_e35676_d_n13, assign18720_e35676_d_n14,) = {
                                    if (assign18720_e35668 > 1e-38) {
                                        let assign18720_e35673: f64 = (p.p97 / var_ni);
                                        let assign18720_e35674: f64 = (assign18720_e35673).ln();
                                        (assign18720_e35674, ((-((p.p97 * var_ni_dn0) / (var_ni * var_ni))) / assign18720_e35673), ((-((p.p97 * var_ni_dn2) / (var_ni * var_ni))) / assign18720_e35673), ((-((p.p97 * var_ni_dn3) / (var_ni * var_ni))) / assign18720_e35673), ((-((p.p97 * var_ni_dn4) / (var_ni * var_ni))) / assign18720_e35673), ((-((p.p97 * var_ni_dn5) / (var_ni * var_ni))) / assign18720_e35673), ((-((p.p97 * var_ni_dn6) / (var_ni * var_ni))) / assign18720_e35673), ((-((p.p97 * var_ni_dn7) / (var_ni * var_ni))) / assign18720_e35673), ((-((p.p97 * var_ni_dn8) / (var_ni * var_ni))) / assign18720_e35673), ((-((p.p97 * var_ni_dn9) / (var_ni * var_ni))) / assign18720_e35673), ((-((p.p97 * var_ni_dn10) / (var_ni * var_ni))) / assign18720_e35673), ((-((p.p97 * var_ni_dn11) / (var_ni * var_ni))) / assign18720_e35673), ((-((p.p97 * var_ni_dn13) / (var_ni * var_ni))) / assign18720_e35673), ((-((p.p97 * var_ni_dn14) / (var_ni * var_ni))) / assign18720_e35673),)
                                    } else {
                                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                                    }
                                };
                                (assign18720_e35676, assign18720_e35676_d_n0, assign18720_e35676_d_n2, assign18720_e35676_d_n3, assign18720_e35676_d_n4, assign18720_e35676_d_n5, assign18720_e35676_d_n6, assign18720_e35676_d_n7, assign18720_e35676_d_n8, assign18720_e35676_d_n9, assign18720_e35676_d_n10, assign18720_e35676_d_n11, assign18720_e35676_d_n13, assign18720_e35676_d_n14,)
                            }
                        };
                        let assign18720_e35678: f64 = (var_vtm * assign18720_e35677);
                        let assign18720_e35679: f64 = (assign18720_e35656 - assign18720_e35678);
                        let assign18720_e35680: f64 = (assign18720_e35653 / assign18720_e35679);
                        (assign18720_e35680, (-((assign18720_e35653 * (-(var_vtm * assign18720_e35677_d_n0))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(var_vtm * assign18720_e35677_d_n2))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(var_vtm * assign18720_e35677_d_n3))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * ((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18720_e35677) + (var_vtm * assign18720_e35677_d_n4)))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(var_vtm * assign18720_e35677_d_n5))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(var_vtm * assign18720_e35677_d_n6))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(var_vtm * assign18720_e35677_d_n7))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(var_vtm * assign18720_e35677_d_n8))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(var_vtm * assign18720_e35677_d_n9))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(var_vtm * assign18720_e35677_d_n10))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(var_vtm * assign18720_e35677_d_n11))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(var_vtm * assign18720_e35677_d_n13))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(var_vtm * assign18720_e35677_d_n14))) / (assign18720_e35679 * assign18720_e35679))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18720_e35682, assign18720_e35682_d_n0, assign18720_e35682_d_n2, assign18720_e35682_d_n3, assign18720_e35682_d_n4, assign18720_e35682_d_n5, assign18720_e35682_d_n6, assign18720_e35682_d_n7, assign18720_e35682_d_n8, assign18720_e35682_d_n9, assign18720_e35682_d_n10, assign18720_e35682_d_n11, assign18720_e35682_d_n13, assign18720_e35682_d_n14,)
            }
        };
        let assign18720_e35684: f64 = (assign18720_e35497 - assign18720_e35683);
        let assign18720_e35685: f64 = (var_devsign * assign18720_e35684);
        let assign18720_e35686: f64 = (assign18720_e35493 - assign18720_e35685);
        let assign18720_e35687: f64 = (assign18720_e35490 - assign18720_e35686);
        let assign18720_e35688: f64 = (var_devsign * assign18720_e35687);
        (assign18720_e35688, (var_devsign * (assign18720_e35490_d_n0 - (-(var_devsign * (-assign18720_e35683_d_n0))))), (var_devsign * (assign18720_e35490_d_n2 - (-(var_devsign * (-assign18720_e35683_d_n2))))), (var_devsign * (assign18720_e35490_d_n3 - (-(var_devsign * (-assign18720_e35683_d_n3))))), (var_devsign * (assign18720_e35490_d_n4 - ((0.5 * var_eg_dn4) - (var_devsign * ((0.5 * var_eg_dn4) - assign18720_e35683_d_n4))))), (var_devsign * (assign18720_e35490_d_n5 - (-(var_devsign * (-assign18720_e35683_d_n5))))), (var_devsign * (assign18720_e35490_d_n6 - (-(var_devsign * (-assign18720_e35683_d_n6))))), (var_devsign * (assign18720_e35490_d_n7 - (-(var_devsign * (-assign18720_e35683_d_n7))))), (var_devsign * (assign18720_e35490_d_n8 - (-(var_devsign * (-assign18720_e35683_d_n8))))), (var_devsign * (assign18720_e35490_d_n9 - (-(var_devsign * (-assign18720_e35683_d_n9))))), (var_devsign * (assign18720_e35490_d_n10 - (-(var_devsign * (-assign18720_e35683_d_n10))))), (var_devsign * (assign18720_e35490_d_n11 - (-(var_devsign * (-assign18720_e35683_d_n11))))), (var_devsign * (assign18720_e35490_d_n13 - (-(var_devsign * (-assign18720_e35683_d_n13))))), (var_devsign * (assign18720_e35490_d_n14 - (-(var_devsign * (-assign18720_e35683_d_n14))))),)
    } else {
        (var_vfbsd_v, var_vfbsd_v_dn0, var_vfbsd_v_dn2, var_vfbsd_v_dn3, var_vfbsd_v_dn4, var_vfbsd_v_dn5, var_vfbsd_v_dn6, var_vfbsd_v_dn7, var_vfbsd_v_dn8, var_vfbsd_v_dn9, var_vfbsd_v_dn10, var_vfbsd_v_dn11, var_vfbsd_v_dn13, var_vfbsd_v_dn14,)
    }
};
        var_vfbsd_v = assign18720_e35690;
        var_vfbsd_v_dn0 = assign18720_e35690_d_n0;
        var_vfbsd_v_dn2 = assign18720_e35690_d_n2;
        var_vfbsd_v_dn3 = assign18720_e35690_d_n3;
        var_vfbsd_v_dn4 = assign18720_e35690_d_n4;
        var_vfbsd_v_dn5 = assign18720_e35690_d_n5;
        var_vfbsd_v_dn6 = assign18720_e35690_d_n6;
        var_vfbsd_v_dn7 = assign18720_e35690_d_n7;
        var_vfbsd_v_dn8 = assign18720_e35690_d_n8;
        var_vfbsd_v_dn9 = assign18720_e35690_d_n9;
        var_vfbsd_v_dn10 = assign18720_e35690_d_n10;
        var_vfbsd_v_dn11 = assign18720_e35690_d_n11;
        var_vfbsd_v_dn13 = assign18720_e35690_d_n13;
        var_vfbsd_v_dn14 = assign18720_e35690_d_n14;
        var_vfbsd_v_rv = 0.0;

        *var_vfbsd_v_slot = var_vfbsd_v;
        *var_vfbsd_v_dn0_slot = var_vfbsd_v_dn0;
        *var_vfbsd_v_dn10_slot = var_vfbsd_v_dn10;
        *var_vfbsd_v_dn11_slot = var_vfbsd_v_dn11;
        *var_vfbsd_v_dn13_slot = var_vfbsd_v_dn13;
        *var_vfbsd_v_dn14_slot = var_vfbsd_v_dn14;
        *var_vfbsd_v_dn2_slot = var_vfbsd_v_dn2;
        *var_vfbsd_v_dn3_slot = var_vfbsd_v_dn3;
        *var_vfbsd_v_dn4_slot = var_vfbsd_v_dn4;
        *var_vfbsd_v_dn5_slot = var_vfbsd_v_dn5;
        *var_vfbsd_v_dn6_slot = var_vfbsd_v_dn6;
        *var_vfbsd_v_dn7_slot = var_vfbsd_v_dn7;
        *var_vfbsd_v_dn8_slot = var_vfbsd_v_dn8;
        *var_vfbsd_v_dn9_slot = var_vfbsd_v_dn9;
        *var_vfbsd_v_rv_slot = var_vfbsd_v_rv;
    }

    pub(super) fn stamp_reactive_block_68(
        p: &Parameters,
        var_devsign: f64,
        var_eg: f64,
        var_eg_dn4: f64,
        var_guard343: f64,
        var_guard344: f64,
        var_guard345: f64,
        var_niln: f64,
        var_niln_dn0: f64,
        var_niln_dn10: f64,
        var_niln_dn11: f64,
        var_niln_dn13: f64,
        var_niln_dn14: f64,
        var_niln_dn2: f64,
        var_niln_dn3: f64,
        var_niln_dn4: f64,
        var_niln_dn5: f64,
        var_niln_dn6: f64,
        var_niln_dn7: f64,
        var_niln_dn8: f64,
        var_niln_dn9: f64,
        var_vtm: f64,
        var_vtm_dn4: f64,
        var_guard346_slot: &mut f64,
        var_guard346_rv_slot: &mut f64,
        var_vfbsd_v_slot: &mut f64,
        var_vfbsd_v_dn0_slot: &mut f64,
        var_vfbsd_v_dn10_slot: &mut f64,
        var_vfbsd_v_dn11_slot: &mut f64,
        var_vfbsd_v_dn13_slot: &mut f64,
        var_vfbsd_v_dn14_slot: &mut f64,
        var_vfbsd_v_dn2_slot: &mut f64,
        var_vfbsd_v_dn3_slot: &mut f64,
        var_vfbsd_v_dn4_slot: &mut f64,
        var_vfbsd_v_dn5_slot: &mut f64,
        var_vfbsd_v_dn6_slot: &mut f64,
        var_vfbsd_v_dn7_slot: &mut f64,
        var_vfbsd_v_dn8_slot: &mut f64,
        var_vfbsd_v_dn9_slot: &mut f64,
        var_vfbsd_v_rv_slot: &mut f64,
    ) {
        let mut var_guard346: f64 = *var_guard346_slot;
        let mut var_guard346_rv: f64 = *var_guard346_rv_slot;
        let mut var_vfbsd_v: f64 = *var_vfbsd_v_slot;
        let mut var_vfbsd_v_dn0: f64 = *var_vfbsd_v_dn0_slot;
        let mut var_vfbsd_v_dn10: f64 = *var_vfbsd_v_dn10_slot;
        let mut var_vfbsd_v_dn11: f64 = *var_vfbsd_v_dn11_slot;
        let mut var_vfbsd_v_dn13: f64 = *var_vfbsd_v_dn13_slot;
        let mut var_vfbsd_v_dn14: f64 = *var_vfbsd_v_dn14_slot;
        let mut var_vfbsd_v_dn2: f64 = *var_vfbsd_v_dn2_slot;
        let mut var_vfbsd_v_dn3: f64 = *var_vfbsd_v_dn3_slot;
        let mut var_vfbsd_v_dn4: f64 = *var_vfbsd_v_dn4_slot;
        let mut var_vfbsd_v_dn5: f64 = *var_vfbsd_v_dn5_slot;
        let mut var_vfbsd_v_dn6: f64 = *var_vfbsd_v_dn6_slot;
        let mut var_vfbsd_v_dn7: f64 = *var_vfbsd_v_dn7_slot;
        let mut var_vfbsd_v_dn8: f64 = *var_vfbsd_v_dn8_slot;
        let mut var_vfbsd_v_dn9: f64 = *var_vfbsd_v_dn9_slot;
        let mut var_vfbsd_v_rv: f64 = *var_vfbsd_v_rv_slot;

        let (assign18730_e36035, assign18730_e36035_d_n0, assign18730_e36035_d_n2, assign18730_e36035_d_n3, assign18730_e36035_d_n4, assign18730_e36035_d_n5, assign18730_e36035_d_n6, assign18730_e36035_d_n7, assign18730_e36035_d_n8, assign18730_e36035_d_n9, assign18730_e36035_d_n10, assign18730_e36035_d_n11, assign18730_e36035_d_n13, assign18730_e36035_d_n14,) = {
    if (((var_guard343 != 0.0) && (var_guard344 != 0.0)) && (var_guard345 == 0.0)) {
        let assign18730_e35700: f64 = (0.5 * var_eg);
        let (assign18730_e35715,) = {
            if (!(p.p145 > 1e-38)) {
                let assign18730_e35707: f64 = (-87.498233534);
                (assign18730_e35707,)
            } else {
                let (assign18730_e35714,) = {
                    if (p.p145 > 1e-38) {
                        let assign18730_e35712: f64 = (p.p145).ln();
                        (assign18730_e35712,)
                    } else {
                        (0.0,)
                    }
                };
                (assign18730_e35714,)
            }
        };
        let assign18730_e35717: f64 = (assign18730_e35715 - var_niln);
        let assign18730_e35718: f64 = (var_vtm * assign18730_e35717);
        let assign18730_e35719: f64 = (assign18730_e35700 - assign18730_e35718);
        let assign18730_e35721: f64 = (-10000.0);
        let assign18730_e35723: f64 = (assign18730_e35721 * 0.0001);
        let (assign18730_e35859, assign18730_e35859_d_n0, assign18730_e35859_d_n2, assign18730_e35859_d_n3, assign18730_e35859_d_n4, assign18730_e35859_d_n5, assign18730_e35859_d_n6, assign18730_e35859_d_n7, assign18730_e35859_d_n8, assign18730_e35859_d_n9, assign18730_e35859_d_n10, assign18730_e35859_d_n11, assign18730_e35859_d_n13, assign18730_e35859_d_n14,) = {
            if (!(assign18730_e35719 < assign18730_e35723)) {
                let assign18730_e35729: f64 = (0.5 * var_eg);
                let (assign18730_e35744,) = {
                    if (!(p.p145 > 1e-38)) {
                        let assign18730_e35736: f64 = (-87.498233534);
                        (assign18730_e35736,)
                    } else {
                        let (assign18730_e35743,) = {
                            if (p.p145 > 1e-38) {
                                let assign18730_e35741: f64 = (p.p145).ln();
                                (assign18730_e35741,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18730_e35743,)
                    }
                };
                let assign18730_e35746: f64 = (assign18730_e35744 - var_niln);
                let assign18730_e35747: f64 = (var_vtm * assign18730_e35746);
                let assign18730_e35748: f64 = (assign18730_e35729 - assign18730_e35747);
                let assign18730_e35751: f64 = (0.5 * var_eg);
                let (assign18730_e35766,) = {
                    if (!(p.p145 > 1e-38)) {
                        let assign18730_e35758: f64 = (-87.498233534);
                        (assign18730_e35758,)
                    } else {
                        let (assign18730_e35765,) = {
                            if (p.p145 > 1e-38) {
                                let assign18730_e35763: f64 = (p.p145).ln();
                                (assign18730_e35763,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18730_e35765,)
                    }
                };
                let assign18730_e35768: f64 = (assign18730_e35766 - var_niln);
                let assign18730_e35769: f64 = (var_vtm * assign18730_e35768);
                let assign18730_e35770: f64 = (assign18730_e35751 - assign18730_e35769);
                let assign18730_e35773: f64 = (0.5 * var_eg);
                let (assign18730_e35788,) = {
                    if (!(p.p145 > 1e-38)) {
                        let assign18730_e35780: f64 = (-87.498233534);
                        (assign18730_e35780,)
                    } else {
                        let (assign18730_e35787,) = {
                            if (p.p145 > 1e-38) {
                                let assign18730_e35785: f64 = (p.p145).ln();
                                (assign18730_e35785,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18730_e35787,)
                    }
                };
                let assign18730_e35790: f64 = (assign18730_e35788 - var_niln);
                let assign18730_e35791: f64 = (var_vtm * assign18730_e35790);
                let assign18730_e35792: f64 = (assign18730_e35773 - assign18730_e35791);
                let assign18730_e35793: f64 = (assign18730_e35770 * assign18730_e35792);
                let assign18730_e35796: f64 = (4.0 * 0.0001);
                let assign18730_e35798: f64 = (assign18730_e35796 * 0.0001);
                let assign18730_e35799: f64 = (assign18730_e35793 + assign18730_e35798);
                let assign18730_e35800: f64 = (assign18730_e35799).sqrt();
                let assign18730_e35801: f64 = (assign18730_e35748 + assign18730_e35800);
                let assign18730_e35802: f64 = (0.5 * assign18730_e35801);
                (assign18730_e35802, (0.5 * ((-(var_vtm * (-var_niln_dn0))) + ((((-(var_vtm * (-var_niln_dn0))) * assign18730_e35792) + (assign18730_e35770 * (-(var_vtm * (-var_niln_dn0))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(var_vtm * (-var_niln_dn2))) + ((((-(var_vtm * (-var_niln_dn2))) * assign18730_e35792) + (assign18730_e35770 * (-(var_vtm * (-var_niln_dn2))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(var_vtm * (-var_niln_dn3))) + ((((-(var_vtm * (-var_niln_dn3))) * assign18730_e35792) + (assign18730_e35770 * (-(var_vtm * (-var_niln_dn3))))) / (2.0 * assign18730_e35800)))), (0.5 * (((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18730_e35746) + (var_vtm * (-var_niln_dn4)))) + (((((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18730_e35768) + (var_vtm * (-var_niln_dn4)))) * assign18730_e35792) + (assign18730_e35770 * ((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18730_e35790) + (var_vtm * (-var_niln_dn4)))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(var_vtm * (-var_niln_dn5))) + ((((-(var_vtm * (-var_niln_dn5))) * assign18730_e35792) + (assign18730_e35770 * (-(var_vtm * (-var_niln_dn5))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(var_vtm * (-var_niln_dn6))) + ((((-(var_vtm * (-var_niln_dn6))) * assign18730_e35792) + (assign18730_e35770 * (-(var_vtm * (-var_niln_dn6))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(var_vtm * (-var_niln_dn7))) + ((((-(var_vtm * (-var_niln_dn7))) * assign18730_e35792) + (assign18730_e35770 * (-(var_vtm * (-var_niln_dn7))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(var_vtm * (-var_niln_dn8))) + ((((-(var_vtm * (-var_niln_dn8))) * assign18730_e35792) + (assign18730_e35770 * (-(var_vtm * (-var_niln_dn8))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(var_vtm * (-var_niln_dn9))) + ((((-(var_vtm * (-var_niln_dn9))) * assign18730_e35792) + (assign18730_e35770 * (-(var_vtm * (-var_niln_dn9))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(var_vtm * (-var_niln_dn10))) + ((((-(var_vtm * (-var_niln_dn10))) * assign18730_e35792) + (assign18730_e35770 * (-(var_vtm * (-var_niln_dn10))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(var_vtm * (-var_niln_dn11))) + ((((-(var_vtm * (-var_niln_dn11))) * assign18730_e35792) + (assign18730_e35770 * (-(var_vtm * (-var_niln_dn11))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(var_vtm * (-var_niln_dn13))) + ((((-(var_vtm * (-var_niln_dn13))) * assign18730_e35792) + (assign18730_e35770 * (-(var_vtm * (-var_niln_dn13))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(var_vtm * (-var_niln_dn14))) + ((((-(var_vtm * (-var_niln_dn14))) * assign18730_e35792) + (assign18730_e35770 * (-(var_vtm * (-var_niln_dn14))))) / (2.0 * assign18730_e35800)))),)
            } else {
                let assign18730_e35805: f64 = (0.5 * var_eg);
                let (assign18730_e35820,) = {
                    if (!(p.p145 > 1e-38)) {
                        let assign18730_e35812: f64 = (-87.498233534);
                        (assign18730_e35812,)
                    } else {
                        let (assign18730_e35819,) = {
                            if (p.p145 > 1e-38) {
                                let assign18730_e35817: f64 = (p.p145).ln();
                                (assign18730_e35817,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18730_e35819,)
                    }
                };
                let assign18730_e35822: f64 = (assign18730_e35820 - var_niln);
                let assign18730_e35823: f64 = (var_vtm * assign18730_e35822);
                let assign18730_e35824: f64 = (assign18730_e35805 - assign18730_e35823);
                let assign18730_e35826: f64 = (-10000.0);
                let assign18730_e35828: f64 = (assign18730_e35826 * 0.0001);
                let (assign18730_e35858, assign18730_e35858_d_n0, assign18730_e35858_d_n2, assign18730_e35858_d_n3, assign18730_e35858_d_n4, assign18730_e35858_d_n5, assign18730_e35858_d_n6, assign18730_e35858_d_n7, assign18730_e35858_d_n8, assign18730_e35858_d_n9, assign18730_e35858_d_n10, assign18730_e35858_d_n11, assign18730_e35858_d_n13, assign18730_e35858_d_n14,) = {
                    if (assign18730_e35824 < assign18730_e35828) {
                        let assign18730_e35831: f64 = (-0.0001);
                        let assign18730_e35833: f64 = (assign18730_e35831 * 0.0001);
                        let assign18730_e35836: f64 = (0.5 * var_eg);
                        let (assign18730_e35851,) = {
                            if (!(p.p145 > 1e-38)) {
                                let assign18730_e35843: f64 = (-87.498233534);
                                (assign18730_e35843,)
                            } else {
                                let (assign18730_e35850,) = {
                                    if (p.p145 > 1e-38) {
                                        let assign18730_e35848: f64 = (p.p145).ln();
                                        (assign18730_e35848,)
                                    } else {
                                        (0.0,)
                                    }
                                };
                                (assign18730_e35850,)
                            }
                        };
                        let assign18730_e35853: f64 = (assign18730_e35851 - var_niln);
                        let assign18730_e35854: f64 = (var_vtm * assign18730_e35853);
                        let assign18730_e35855: f64 = (assign18730_e35836 - assign18730_e35854);
                        let assign18730_e35856: f64 = (assign18730_e35833 / assign18730_e35855);
                        (assign18730_e35856, (-((assign18730_e35833 * (-(var_vtm * (-var_niln_dn0)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(var_vtm * (-var_niln_dn2)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(var_vtm * (-var_niln_dn3)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * ((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18730_e35853) + (var_vtm * (-var_niln_dn4))))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(var_vtm * (-var_niln_dn5)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(var_vtm * (-var_niln_dn6)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(var_vtm * (-var_niln_dn7)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(var_vtm * (-var_niln_dn8)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(var_vtm * (-var_niln_dn9)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(var_vtm * (-var_niln_dn10)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(var_vtm * (-var_niln_dn11)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(var_vtm * (-var_niln_dn13)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(var_vtm * (-var_niln_dn14)))) / (assign18730_e35855 * assign18730_e35855))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18730_e35858, assign18730_e35858_d_n0, assign18730_e35858_d_n2, assign18730_e35858_d_n3, assign18730_e35858_d_n4, assign18730_e35858_d_n5, assign18730_e35858_d_n6, assign18730_e35858_d_n7, assign18730_e35858_d_n8, assign18730_e35858_d_n9, assign18730_e35858_d_n10, assign18730_e35858_d_n11, assign18730_e35858_d_n13, assign18730_e35858_d_n14,)
            }
        };
        let assign18730_e35862: f64 = (0.5 * var_eg);
        let assign18730_e35866: f64 = (0.5 * var_eg);
        let assign18730_e35869: f64 = (0.5 * var_eg);
        let (assign18730_e35884,) = {
            if (!(p.p97 > 1e-38)) {
                let assign18730_e35876: f64 = (-87.498233534);
                (assign18730_e35876,)
            } else {
                let (assign18730_e35883,) = {
                    if (p.p97 > 1e-38) {
                        let assign18730_e35881: f64 = (p.p97).ln();
                        (assign18730_e35881,)
                    } else {
                        (0.0,)
                    }
                };
                (assign18730_e35883,)
            }
        };
        let assign18730_e35886: f64 = (assign18730_e35884 - var_niln);
        let assign18730_e35887: f64 = (var_vtm * assign18730_e35886);
        let assign18730_e35888: f64 = (assign18730_e35869 - assign18730_e35887);
        let assign18730_e35890: f64 = (-10000.0);
        let assign18730_e35892: f64 = (assign18730_e35890 * 0.0001);
        let (assign18730_e36028, assign18730_e36028_d_n0, assign18730_e36028_d_n2, assign18730_e36028_d_n3, assign18730_e36028_d_n4, assign18730_e36028_d_n5, assign18730_e36028_d_n6, assign18730_e36028_d_n7, assign18730_e36028_d_n8, assign18730_e36028_d_n9, assign18730_e36028_d_n10, assign18730_e36028_d_n11, assign18730_e36028_d_n13, assign18730_e36028_d_n14,) = {
            if (!(assign18730_e35888 < assign18730_e35892)) {
                let assign18730_e35898: f64 = (0.5 * var_eg);
                let (assign18730_e35913,) = {
                    if (!(p.p97 > 1e-38)) {
                        let assign18730_e35905: f64 = (-87.498233534);
                        (assign18730_e35905,)
                    } else {
                        let (assign18730_e35912,) = {
                            if (p.p97 > 1e-38) {
                                let assign18730_e35910: f64 = (p.p97).ln();
                                (assign18730_e35910,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18730_e35912,)
                    }
                };
                let assign18730_e35915: f64 = (assign18730_e35913 - var_niln);
                let assign18730_e35916: f64 = (var_vtm * assign18730_e35915);
                let assign18730_e35917: f64 = (assign18730_e35898 - assign18730_e35916);
                let assign18730_e35920: f64 = (0.5 * var_eg);
                let (assign18730_e35935,) = {
                    if (!(p.p97 > 1e-38)) {
                        let assign18730_e35927: f64 = (-87.498233534);
                        (assign18730_e35927,)
                    } else {
                        let (assign18730_e35934,) = {
                            if (p.p97 > 1e-38) {
                                let assign18730_e35932: f64 = (p.p97).ln();
                                (assign18730_e35932,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18730_e35934,)
                    }
                };
                let assign18730_e35937: f64 = (assign18730_e35935 - var_niln);
                let assign18730_e35938: f64 = (var_vtm * assign18730_e35937);
                let assign18730_e35939: f64 = (assign18730_e35920 - assign18730_e35938);
                let assign18730_e35942: f64 = (0.5 * var_eg);
                let (assign18730_e35957,) = {
                    if (!(p.p97 > 1e-38)) {
                        let assign18730_e35949: f64 = (-87.498233534);
                        (assign18730_e35949,)
                    } else {
                        let (assign18730_e35956,) = {
                            if (p.p97 > 1e-38) {
                                let assign18730_e35954: f64 = (p.p97).ln();
                                (assign18730_e35954,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18730_e35956,)
                    }
                };
                let assign18730_e35959: f64 = (assign18730_e35957 - var_niln);
                let assign18730_e35960: f64 = (var_vtm * assign18730_e35959);
                let assign18730_e35961: f64 = (assign18730_e35942 - assign18730_e35960);
                let assign18730_e35962: f64 = (assign18730_e35939 * assign18730_e35961);
                let assign18730_e35965: f64 = (4.0 * 0.0001);
                let assign18730_e35967: f64 = (assign18730_e35965 * 0.0001);
                let assign18730_e35968: f64 = (assign18730_e35962 + assign18730_e35967);
                let assign18730_e35969: f64 = (assign18730_e35968).sqrt();
                let assign18730_e35970: f64 = (assign18730_e35917 + assign18730_e35969);
                let assign18730_e35971: f64 = (0.5 * assign18730_e35970);
                (assign18730_e35971, (0.5 * ((-(var_vtm * (-var_niln_dn0))) + ((((-(var_vtm * (-var_niln_dn0))) * assign18730_e35961) + (assign18730_e35939 * (-(var_vtm * (-var_niln_dn0))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(var_vtm * (-var_niln_dn2))) + ((((-(var_vtm * (-var_niln_dn2))) * assign18730_e35961) + (assign18730_e35939 * (-(var_vtm * (-var_niln_dn2))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(var_vtm * (-var_niln_dn3))) + ((((-(var_vtm * (-var_niln_dn3))) * assign18730_e35961) + (assign18730_e35939 * (-(var_vtm * (-var_niln_dn3))))) / (2.0 * assign18730_e35969)))), (0.5 * (((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18730_e35915) + (var_vtm * (-var_niln_dn4)))) + (((((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18730_e35937) + (var_vtm * (-var_niln_dn4)))) * assign18730_e35961) + (assign18730_e35939 * ((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18730_e35959) + (var_vtm * (-var_niln_dn4)))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(var_vtm * (-var_niln_dn5))) + ((((-(var_vtm * (-var_niln_dn5))) * assign18730_e35961) + (assign18730_e35939 * (-(var_vtm * (-var_niln_dn5))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(var_vtm * (-var_niln_dn6))) + ((((-(var_vtm * (-var_niln_dn6))) * assign18730_e35961) + (assign18730_e35939 * (-(var_vtm * (-var_niln_dn6))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(var_vtm * (-var_niln_dn7))) + ((((-(var_vtm * (-var_niln_dn7))) * assign18730_e35961) + (assign18730_e35939 * (-(var_vtm * (-var_niln_dn7))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(var_vtm * (-var_niln_dn8))) + ((((-(var_vtm * (-var_niln_dn8))) * assign18730_e35961) + (assign18730_e35939 * (-(var_vtm * (-var_niln_dn8))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(var_vtm * (-var_niln_dn9))) + ((((-(var_vtm * (-var_niln_dn9))) * assign18730_e35961) + (assign18730_e35939 * (-(var_vtm * (-var_niln_dn9))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(var_vtm * (-var_niln_dn10))) + ((((-(var_vtm * (-var_niln_dn10))) * assign18730_e35961) + (assign18730_e35939 * (-(var_vtm * (-var_niln_dn10))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(var_vtm * (-var_niln_dn11))) + ((((-(var_vtm * (-var_niln_dn11))) * assign18730_e35961) + (assign18730_e35939 * (-(var_vtm * (-var_niln_dn11))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(var_vtm * (-var_niln_dn13))) + ((((-(var_vtm * (-var_niln_dn13))) * assign18730_e35961) + (assign18730_e35939 * (-(var_vtm * (-var_niln_dn13))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(var_vtm * (-var_niln_dn14))) + ((((-(var_vtm * (-var_niln_dn14))) * assign18730_e35961) + (assign18730_e35939 * (-(var_vtm * (-var_niln_dn14))))) / (2.0 * assign18730_e35969)))),)
            } else {
                let assign18730_e35974: f64 = (0.5 * var_eg);
                let (assign18730_e35989,) = {
                    if (!(p.p97 > 1e-38)) {
                        let assign18730_e35981: f64 = (-87.498233534);
                        (assign18730_e35981,)
                    } else {
                        let (assign18730_e35988,) = {
                            if (p.p97 > 1e-38) {
                                let assign18730_e35986: f64 = (p.p97).ln();
                                (assign18730_e35986,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18730_e35988,)
                    }
                };
                let assign18730_e35991: f64 = (assign18730_e35989 - var_niln);
                let assign18730_e35992: f64 = (var_vtm * assign18730_e35991);
                let assign18730_e35993: f64 = (assign18730_e35974 - assign18730_e35992);
                let assign18730_e35995: f64 = (-10000.0);
                let assign18730_e35997: f64 = (assign18730_e35995 * 0.0001);
                let (assign18730_e36027, assign18730_e36027_d_n0, assign18730_e36027_d_n2, assign18730_e36027_d_n3, assign18730_e36027_d_n4, assign18730_e36027_d_n5, assign18730_e36027_d_n6, assign18730_e36027_d_n7, assign18730_e36027_d_n8, assign18730_e36027_d_n9, assign18730_e36027_d_n10, assign18730_e36027_d_n11, assign18730_e36027_d_n13, assign18730_e36027_d_n14,) = {
                    if (assign18730_e35993 < assign18730_e35997) {
                        let assign18730_e36000: f64 = (-0.0001);
                        let assign18730_e36002: f64 = (assign18730_e36000 * 0.0001);
                        let assign18730_e36005: f64 = (0.5 * var_eg);
                        let (assign18730_e36020,) = {
                            if (!(p.p97 > 1e-38)) {
                                let assign18730_e36012: f64 = (-87.498233534);
                                (assign18730_e36012,)
                            } else {
                                let (assign18730_e36019,) = {
                                    if (p.p97 > 1e-38) {
                                        let assign18730_e36017: f64 = (p.p97).ln();
                                        (assign18730_e36017,)
                                    } else {
                                        (0.0,)
                                    }
                                };
                                (assign18730_e36019,)
                            }
                        };
                        let assign18730_e36022: f64 = (assign18730_e36020 - var_niln);
                        let assign18730_e36023: f64 = (var_vtm * assign18730_e36022);
                        let assign18730_e36024: f64 = (assign18730_e36005 - assign18730_e36023);
                        let assign18730_e36025: f64 = (assign18730_e36002 / assign18730_e36024);
                        (assign18730_e36025, (-((assign18730_e36002 * (-(var_vtm * (-var_niln_dn0)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(var_vtm * (-var_niln_dn2)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(var_vtm * (-var_niln_dn3)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * ((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18730_e36022) + (var_vtm * (-var_niln_dn4))))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(var_vtm * (-var_niln_dn5)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(var_vtm * (-var_niln_dn6)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(var_vtm * (-var_niln_dn7)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(var_vtm * (-var_niln_dn8)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(var_vtm * (-var_niln_dn9)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(var_vtm * (-var_niln_dn10)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(var_vtm * (-var_niln_dn11)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(var_vtm * (-var_niln_dn13)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(var_vtm * (-var_niln_dn14)))) / (assign18730_e36024 * assign18730_e36024))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18730_e36027, assign18730_e36027_d_n0, assign18730_e36027_d_n2, assign18730_e36027_d_n3, assign18730_e36027_d_n4, assign18730_e36027_d_n5, assign18730_e36027_d_n6, assign18730_e36027_d_n7, assign18730_e36027_d_n8, assign18730_e36027_d_n9, assign18730_e36027_d_n10, assign18730_e36027_d_n11, assign18730_e36027_d_n13, assign18730_e36027_d_n14,)
            }
        };
        let assign18730_e36029: f64 = (assign18730_e35866 - assign18730_e36028);
        let assign18730_e36030: f64 = (var_devsign * assign18730_e36029);
        let assign18730_e36031: f64 = (assign18730_e35862 - assign18730_e36030);
        let assign18730_e36032: f64 = (assign18730_e35859 - assign18730_e36031);
        let assign18730_e36033: f64 = (var_devsign * assign18730_e36032);
        (assign18730_e36033, (var_devsign * (assign18730_e35859_d_n0 - (-(var_devsign * (-assign18730_e36028_d_n0))))), (var_devsign * (assign18730_e35859_d_n2 - (-(var_devsign * (-assign18730_e36028_d_n2))))), (var_devsign * (assign18730_e35859_d_n3 - (-(var_devsign * (-assign18730_e36028_d_n3))))), (var_devsign * (assign18730_e35859_d_n4 - ((0.5 * var_eg_dn4) - (var_devsign * ((0.5 * var_eg_dn4) - assign18730_e36028_d_n4))))), (var_devsign * (assign18730_e35859_d_n5 - (-(var_devsign * (-assign18730_e36028_d_n5))))), (var_devsign * (assign18730_e35859_d_n6 - (-(var_devsign * (-assign18730_e36028_d_n6))))), (var_devsign * (assign18730_e35859_d_n7 - (-(var_devsign * (-assign18730_e36028_d_n7))))), (var_devsign * (assign18730_e35859_d_n8 - (-(var_devsign * (-assign18730_e36028_d_n8))))), (var_devsign * (assign18730_e35859_d_n9 - (-(var_devsign * (-assign18730_e36028_d_n9))))), (var_devsign * (assign18730_e35859_d_n10 - (-(var_devsign * (-assign18730_e36028_d_n10))))), (var_devsign * (assign18730_e35859_d_n11 - (-(var_devsign * (-assign18730_e36028_d_n11))))), (var_devsign * (assign18730_e35859_d_n13 - (-(var_devsign * (-assign18730_e36028_d_n13))))), (var_devsign * (assign18730_e35859_d_n14 - (-(var_devsign * (-assign18730_e36028_d_n14))))),)
    } else {
        (var_vfbsd_v, var_vfbsd_v_dn0, var_vfbsd_v_dn2, var_vfbsd_v_dn3, var_vfbsd_v_dn4, var_vfbsd_v_dn5, var_vfbsd_v_dn6, var_vfbsd_v_dn7, var_vfbsd_v_dn8, var_vfbsd_v_dn9, var_vfbsd_v_dn10, var_vfbsd_v_dn11, var_vfbsd_v_dn13, var_vfbsd_v_dn14,)
    }
};
        var_vfbsd_v = assign18730_e36035;
        var_vfbsd_v_dn0 = assign18730_e36035_d_n0;
        var_vfbsd_v_dn2 = assign18730_e36035_d_n2;
        var_vfbsd_v_dn3 = assign18730_e36035_d_n3;
        var_vfbsd_v_dn4 = assign18730_e36035_d_n4;
        var_vfbsd_v_dn5 = assign18730_e36035_d_n5;
        var_vfbsd_v_dn6 = assign18730_e36035_d_n6;
        var_vfbsd_v_dn7 = assign18730_e36035_d_n7;
        var_vfbsd_v_dn8 = assign18730_e36035_d_n8;
        var_vfbsd_v_dn9 = assign18730_e36035_d_n9;
        var_vfbsd_v_dn10 = assign18730_e36035_d_n10;
        var_vfbsd_v_dn11 = assign18730_e36035_d_n11;
        var_vfbsd_v_dn13 = assign18730_e36035_d_n13;
        var_vfbsd_v_dn14 = assign18730_e36035_d_n14;
        var_vfbsd_v_rv = 0.0;

        let assign18740_e36038: f64 = if p.p80 == 0.0 { 1.0 } else { 0.0 };
        var_guard346 = assign18740_e36038;
        var_guard346_rv = 0.0;

        *var_guard346_slot = var_guard346;
        *var_guard346_rv_slot = var_guard346_rv;
        *var_vfbsd_v_slot = var_vfbsd_v;
        *var_vfbsd_v_dn0_slot = var_vfbsd_v_dn0;
        *var_vfbsd_v_dn10_slot = var_vfbsd_v_dn10;
        *var_vfbsd_v_dn11_slot = var_vfbsd_v_dn11;
        *var_vfbsd_v_dn13_slot = var_vfbsd_v_dn13;
        *var_vfbsd_v_dn14_slot = var_vfbsd_v_dn14;
        *var_vfbsd_v_dn2_slot = var_vfbsd_v_dn2;
        *var_vfbsd_v_dn3_slot = var_vfbsd_v_dn3;
        *var_vfbsd_v_dn4_slot = var_vfbsd_v_dn4;
        *var_vfbsd_v_dn5_slot = var_vfbsd_v_dn5;
        *var_vfbsd_v_dn6_slot = var_vfbsd_v_dn6;
        *var_vfbsd_v_dn7_slot = var_vfbsd_v_dn7;
        *var_vfbsd_v_dn8_slot = var_vfbsd_v_dn8;
        *var_vfbsd_v_dn9_slot = var_vfbsd_v_dn9;
        *var_vfbsd_v_rv_slot = var_vfbsd_v_rv;
    }

    pub(super) fn stamp_reactive_block_69(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_devsign: f64,
        var_eg: f64,
        var_eg_dn4: f64,
        var_guard343: f64,
        var_guard344: f64,
        var_guard346: f64,
        var_nbody_i: f64,
        var_ni: f64,
        var_ni_dn0: f64,
        var_ni_dn10: f64,
        var_ni_dn11: f64,
        var_ni_dn13: f64,
        var_ni_dn14: f64,
        var_ni_dn2: f64,
        var_ni_dn3: f64,
        var_ni_dn4: f64,
        var_ni_dn5: f64,
        var_ni_dn6: f64,
        var_ni_dn7: f64,
        var_ni_dn8: f64,
        var_ni_dn9: f64,
        var_niln: f64,
        var_niln_dn0: f64,
        var_niln_dn10: f64,
        var_niln_dn11: f64,
        var_niln_dn13: f64,
        var_niln_dn14: f64,
        var_niln_dn2: f64,
        var_niln_dn3: f64,
        var_niln_dn4: f64,
        var_niln_dn5: f64,
        var_niln_dn6: f64,
        var_niln_dn7: f64,
        var_niln_dn8: f64,
        var_niln_dn9: f64,
        var_phig_i: f64,
        var_phig_i_dn0: f64,
        var_phig_i_dn10: f64,
        var_phig_i_dn11: f64,
        var_phig_i_dn13: f64,
        var_phig_i_dn14: f64,
        var_phig_i_dn2: f64,
        var_phig_i_dn3: f64,
        var_phig_i_dn4: f64,
        var_phig_i_dn5: f64,
        var_phig_i_dn6: f64,
        var_phig_i_dn7: f64,
        var_phig_i_dn8: f64,
        var_phig_i_dn9: f64,
        var_vtm: f64,
        var_vtm_dn4: f64,
        var_guard347_slot: &mut f64,
        var_guard347_rv_slot: &mut f64,
        var_guard348_slot: &mut f64,
        var_guard348_rv_slot: &mut f64,
        var_phib_slot: &mut f64,
        var_phib_dn0_slot: &mut f64,
        var_phib_dn10_slot: &mut f64,
        var_phib_dn11_slot: &mut f64,
        var_phib_dn13_slot: &mut f64,
        var_phib_dn14_slot: &mut f64,
        var_phib_dn2_slot: &mut f64,
        var_phib_dn3_slot: &mut f64,
        var_phib_dn4_slot: &mut f64,
        var_phib_dn5_slot: &mut f64,
        var_phib_dn6_slot: &mut f64,
        var_phib_dn7_slot: &mut f64,
        var_phib_dn8_slot: &mut f64,
        var_phib_dn9_slot: &mut f64,
        var_phib_rv_slot: &mut f64,
        var_vfbsd_v_slot: &mut f64,
        var_vfbsd_v_dn0_slot: &mut f64,
        var_vfbsd_v_dn10_slot: &mut f64,
        var_vfbsd_v_dn11_slot: &mut f64,
        var_vfbsd_v_dn13_slot: &mut f64,
        var_vfbsd_v_dn14_slot: &mut f64,
        var_vfbsd_v_dn2_slot: &mut f64,
        var_vfbsd_v_dn3_slot: &mut f64,
        var_vfbsd_v_dn4_slot: &mut f64,
        var_vfbsd_v_dn5_slot: &mut f64,
        var_vfbsd_v_dn6_slot: &mut f64,
        var_vfbsd_v_dn7_slot: &mut f64,
        var_vfbsd_v_dn8_slot: &mut f64,
        var_vfbsd_v_dn9_slot: &mut f64,
        var_vfbsd_v_rv_slot: &mut f64,
        var_vfbsdcv_v_slot: &mut f64,
        var_vfbsdcv_v_dn0_slot: &mut f64,
        var_vfbsdcv_v_dn10_slot: &mut f64,
        var_vfbsdcv_v_dn11_slot: &mut f64,
        var_vfbsdcv_v_dn13_slot: &mut f64,
        var_vfbsdcv_v_dn14_slot: &mut f64,
        var_vfbsdcv_v_dn2_slot: &mut f64,
        var_vfbsdcv_v_dn3_slot: &mut f64,
        var_vfbsdcv_v_dn4_slot: &mut f64,
        var_vfbsdcv_v_dn5_slot: &mut f64,
        var_vfbsdcv_v_dn6_slot: &mut f64,
        var_vfbsdcv_v_dn7_slot: &mut f64,
        var_vfbsdcv_v_dn8_slot: &mut f64,
        var_vfbsdcv_v_dn9_slot: &mut f64,
        var_vfbsdcv_v_rv_slot: &mut f64,
    ) {
        let mut var_guard347: f64 = *var_guard347_slot;
        let mut var_guard347_rv: f64 = *var_guard347_rv_slot;
        let mut var_guard348: f64 = *var_guard348_slot;
        let mut var_guard348_rv: f64 = *var_guard348_rv_slot;
        let mut var_phib: f64 = *var_phib_slot;
        let mut var_phib_dn0: f64 = *var_phib_dn0_slot;
        let mut var_phib_dn10: f64 = *var_phib_dn10_slot;
        let mut var_phib_dn11: f64 = *var_phib_dn11_slot;
        let mut var_phib_dn13: f64 = *var_phib_dn13_slot;
        let mut var_phib_dn14: f64 = *var_phib_dn14_slot;
        let mut var_phib_dn2: f64 = *var_phib_dn2_slot;
        let mut var_phib_dn3: f64 = *var_phib_dn3_slot;
        let mut var_phib_dn4: f64 = *var_phib_dn4_slot;
        let mut var_phib_dn5: f64 = *var_phib_dn5_slot;
        let mut var_phib_dn6: f64 = *var_phib_dn6_slot;
        let mut var_phib_dn7: f64 = *var_phib_dn7_slot;
        let mut var_phib_dn8: f64 = *var_phib_dn8_slot;
        let mut var_phib_dn9: f64 = *var_phib_dn9_slot;
        let mut var_phib_rv: f64 = *var_phib_rv_slot;
        let mut var_vfbsd_v: f64 = *var_vfbsd_v_slot;
        let mut var_vfbsd_v_dn0: f64 = *var_vfbsd_v_dn0_slot;
        let mut var_vfbsd_v_dn10: f64 = *var_vfbsd_v_dn10_slot;
        let mut var_vfbsd_v_dn11: f64 = *var_vfbsd_v_dn11_slot;
        let mut var_vfbsd_v_dn13: f64 = *var_vfbsd_v_dn13_slot;
        let mut var_vfbsd_v_dn14: f64 = *var_vfbsd_v_dn14_slot;
        let mut var_vfbsd_v_dn2: f64 = *var_vfbsd_v_dn2_slot;
        let mut var_vfbsd_v_dn3: f64 = *var_vfbsd_v_dn3_slot;
        let mut var_vfbsd_v_dn4: f64 = *var_vfbsd_v_dn4_slot;
        let mut var_vfbsd_v_dn5: f64 = *var_vfbsd_v_dn5_slot;
        let mut var_vfbsd_v_dn6: f64 = *var_vfbsd_v_dn6_slot;
        let mut var_vfbsd_v_dn7: f64 = *var_vfbsd_v_dn7_slot;
        let mut var_vfbsd_v_dn8: f64 = *var_vfbsd_v_dn8_slot;
        let mut var_vfbsd_v_dn9: f64 = *var_vfbsd_v_dn9_slot;
        let mut var_vfbsd_v_rv: f64 = *var_vfbsd_v_rv_slot;
        let mut var_vfbsdcv_v: f64 = *var_vfbsdcv_v_slot;
        let mut var_vfbsdcv_v_dn0: f64 = *var_vfbsdcv_v_dn0_slot;
        let mut var_vfbsdcv_v_dn10: f64 = *var_vfbsdcv_v_dn10_slot;
        let mut var_vfbsdcv_v_dn11: f64 = *var_vfbsdcv_v_dn11_slot;
        let mut var_vfbsdcv_v_dn13: f64 = *var_vfbsdcv_v_dn13_slot;
        let mut var_vfbsdcv_v_dn14: f64 = *var_vfbsdcv_v_dn14_slot;
        let mut var_vfbsdcv_v_dn2: f64 = *var_vfbsdcv_v_dn2_slot;
        let mut var_vfbsdcv_v_dn3: f64 = *var_vfbsdcv_v_dn3_slot;
        let mut var_vfbsdcv_v_dn4: f64 = *var_vfbsdcv_v_dn4_slot;
        let mut var_vfbsdcv_v_dn5: f64 = *var_vfbsdcv_v_dn5_slot;
        let mut var_vfbsdcv_v_dn6: f64 = *var_vfbsdcv_v_dn6_slot;
        let mut var_vfbsdcv_v_dn7: f64 = *var_vfbsdcv_v_dn7_slot;
        let mut var_vfbsdcv_v_dn8: f64 = *var_vfbsdcv_v_dn8_slot;
        let mut var_vfbsdcv_v_dn9: f64 = *var_vfbsdcv_v_dn9_slot;
        let mut var_vfbsdcv_v_rv: f64 = *var_vfbsdcv_v_rv_slot;

        let (assign18750_e36248, assign18750_e36248_d_n0, assign18750_e36248_d_n2, assign18750_e36248_d_n3, assign18750_e36248_d_n4, assign18750_e36248_d_n5, assign18750_e36248_d_n6, assign18750_e36248_d_n7, assign18750_e36248_d_n8, assign18750_e36248_d_n9, assign18750_e36248_d_n10, assign18750_e36248_d_n11, assign18750_e36248_d_n13, assign18750_e36248_d_n14,) = {
    if (((var_guard343 != 0.0) && (var_guard344 == 0.0)) && (var_guard346 != 0.0)) {
        let assign18750_e36050: f64 = (0.5 * var_eg);
        let assign18750_e36051: f64 = (p.p104 + assign18750_e36050);
        let assign18750_e36055: f64 = (0.5 * var_eg);
        let assign18750_e36058: f64 = (0.5 * var_eg);
        let assign18750_e36062: f64 = (p.p97 / var_ni);
        let (assign18750_e36079, assign18750_e36079_d_n0, assign18750_e36079_d_n2, assign18750_e36079_d_n3, assign18750_e36079_d_n4, assign18750_e36079_d_n5, assign18750_e36079_d_n6, assign18750_e36079_d_n7, assign18750_e36079_d_n8, assign18750_e36079_d_n9, assign18750_e36079_d_n10, assign18750_e36079_d_n11, assign18750_e36079_d_n13, assign18750_e36079_d_n14,) = {
            if (!(assign18750_e36062 > 1e-38)) {
                let assign18750_e36067: f64 = (-87.498233534);
                (assign18750_e36067, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign18750_e36070: f64 = (p.p97 / var_ni);
                let (assign18750_e36078, assign18750_e36078_d_n0, assign18750_e36078_d_n2, assign18750_e36078_d_n3, assign18750_e36078_d_n4, assign18750_e36078_d_n5, assign18750_e36078_d_n6, assign18750_e36078_d_n7, assign18750_e36078_d_n8, assign18750_e36078_d_n9, assign18750_e36078_d_n10, assign18750_e36078_d_n11, assign18750_e36078_d_n13, assign18750_e36078_d_n14,) = {
                    if (assign18750_e36070 > 1e-38) {
                        let assign18750_e36075: f64 = (p.p97 / var_ni);
                        let assign18750_e36076: f64 = (assign18750_e36075).ln();
                        (assign18750_e36076, ((-((p.p97 * var_ni_dn0) / (var_ni * var_ni))) / assign18750_e36075), ((-((p.p97 * var_ni_dn2) / (var_ni * var_ni))) / assign18750_e36075), ((-((p.p97 * var_ni_dn3) / (var_ni * var_ni))) / assign18750_e36075), ((-((p.p97 * var_ni_dn4) / (var_ni * var_ni))) / assign18750_e36075), ((-((p.p97 * var_ni_dn5) / (var_ni * var_ni))) / assign18750_e36075), ((-((p.p97 * var_ni_dn6) / (var_ni * var_ni))) / assign18750_e36075), ((-((p.p97 * var_ni_dn7) / (var_ni * var_ni))) / assign18750_e36075), ((-((p.p97 * var_ni_dn8) / (var_ni * var_ni))) / assign18750_e36075), ((-((p.p97 * var_ni_dn9) / (var_ni * var_ni))) / assign18750_e36075), ((-((p.p97 * var_ni_dn10) / (var_ni * var_ni))) / assign18750_e36075), ((-((p.p97 * var_ni_dn11) / (var_ni * var_ni))) / assign18750_e36075), ((-((p.p97 * var_ni_dn13) / (var_ni * var_ni))) / assign18750_e36075), ((-((p.p97 * var_ni_dn14) / (var_ni * var_ni))) / assign18750_e36075),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18750_e36078, assign18750_e36078_d_n0, assign18750_e36078_d_n2, assign18750_e36078_d_n3, assign18750_e36078_d_n4, assign18750_e36078_d_n5, assign18750_e36078_d_n6, assign18750_e36078_d_n7, assign18750_e36078_d_n8, assign18750_e36078_d_n9, assign18750_e36078_d_n10, assign18750_e36078_d_n11, assign18750_e36078_d_n13, assign18750_e36078_d_n14,)
            }
        };
        let assign18750_e36080: f64 = (var_vtm * assign18750_e36079);
        let assign18750_e36081: f64 = (assign18750_e36058 - assign18750_e36080);
        let assign18750_e36083: f64 = (-10000.0);
        let assign18750_e36085: f64 = (assign18750_e36083 * 0.0001);
        let (assign18750_e36241, assign18750_e36241_d_n0, assign18750_e36241_d_n2, assign18750_e36241_d_n3, assign18750_e36241_d_n4, assign18750_e36241_d_n5, assign18750_e36241_d_n6, assign18750_e36241_d_n7, assign18750_e36241_d_n8, assign18750_e36241_d_n9, assign18750_e36241_d_n10, assign18750_e36241_d_n11, assign18750_e36241_d_n13, assign18750_e36241_d_n14,) = {
            if (!(assign18750_e36081 < assign18750_e36085)) {
                let assign18750_e36091: f64 = (0.5 * var_eg);
                let assign18750_e36095: f64 = (p.p97 / var_ni);
                let (assign18750_e36112, assign18750_e36112_d_n0, assign18750_e36112_d_n2, assign18750_e36112_d_n3, assign18750_e36112_d_n4, assign18750_e36112_d_n5, assign18750_e36112_d_n6, assign18750_e36112_d_n7, assign18750_e36112_d_n8, assign18750_e36112_d_n9, assign18750_e36112_d_n10, assign18750_e36112_d_n11, assign18750_e36112_d_n13, assign18750_e36112_d_n14,) = {
                    if (!(assign18750_e36095 > 1e-38)) {
                        let assign18750_e36100: f64 = (-87.498233534);
                        (assign18750_e36100, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18750_e36103: f64 = (p.p97 / var_ni);
                        let (assign18750_e36111, assign18750_e36111_d_n0, assign18750_e36111_d_n2, assign18750_e36111_d_n3, assign18750_e36111_d_n4, assign18750_e36111_d_n5, assign18750_e36111_d_n6, assign18750_e36111_d_n7, assign18750_e36111_d_n8, assign18750_e36111_d_n9, assign18750_e36111_d_n10, assign18750_e36111_d_n11, assign18750_e36111_d_n13, assign18750_e36111_d_n14,) = {
                            if (assign18750_e36103 > 1e-38) {
                                let assign18750_e36108: f64 = (p.p97 / var_ni);
                                let assign18750_e36109: f64 = (assign18750_e36108).ln();
                                (assign18750_e36109, ((-((p.p97 * var_ni_dn0) / (var_ni * var_ni))) / assign18750_e36108), ((-((p.p97 * var_ni_dn2) / (var_ni * var_ni))) / assign18750_e36108), ((-((p.p97 * var_ni_dn3) / (var_ni * var_ni))) / assign18750_e36108), ((-((p.p97 * var_ni_dn4) / (var_ni * var_ni))) / assign18750_e36108), ((-((p.p97 * var_ni_dn5) / (var_ni * var_ni))) / assign18750_e36108), ((-((p.p97 * var_ni_dn6) / (var_ni * var_ni))) / assign18750_e36108), ((-((p.p97 * var_ni_dn7) / (var_ni * var_ni))) / assign18750_e36108), ((-((p.p97 * var_ni_dn8) / (var_ni * var_ni))) / assign18750_e36108), ((-((p.p97 * var_ni_dn9) / (var_ni * var_ni))) / assign18750_e36108), ((-((p.p97 * var_ni_dn10) / (var_ni * var_ni))) / assign18750_e36108), ((-((p.p97 * var_ni_dn11) / (var_ni * var_ni))) / assign18750_e36108), ((-((p.p97 * var_ni_dn13) / (var_ni * var_ni))) / assign18750_e36108), ((-((p.p97 * var_ni_dn14) / (var_ni * var_ni))) / assign18750_e36108),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18750_e36111, assign18750_e36111_d_n0, assign18750_e36111_d_n2, assign18750_e36111_d_n3, assign18750_e36111_d_n4, assign18750_e36111_d_n5, assign18750_e36111_d_n6, assign18750_e36111_d_n7, assign18750_e36111_d_n8, assign18750_e36111_d_n9, assign18750_e36111_d_n10, assign18750_e36111_d_n11, assign18750_e36111_d_n13, assign18750_e36111_d_n14,)
                    }
                };
                let assign18750_e36113: f64 = (var_vtm * assign18750_e36112);
                let assign18750_e36114: f64 = (assign18750_e36091 - assign18750_e36113);
                let assign18750_e36117: f64 = (0.5 * var_eg);
                let assign18750_e36121: f64 = (p.p97 / var_ni);
                let (assign18750_e36138, assign18750_e36138_d_n0, assign18750_e36138_d_n2, assign18750_e36138_d_n3, assign18750_e36138_d_n4, assign18750_e36138_d_n5, assign18750_e36138_d_n6, assign18750_e36138_d_n7, assign18750_e36138_d_n8, assign18750_e36138_d_n9, assign18750_e36138_d_n10, assign18750_e36138_d_n11, assign18750_e36138_d_n13, assign18750_e36138_d_n14,) = {
                    if (!(assign18750_e36121 > 1e-38)) {
                        let assign18750_e36126: f64 = (-87.498233534);
                        (assign18750_e36126, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18750_e36129: f64 = (p.p97 / var_ni);
                        let (assign18750_e36137, assign18750_e36137_d_n0, assign18750_e36137_d_n2, assign18750_e36137_d_n3, assign18750_e36137_d_n4, assign18750_e36137_d_n5, assign18750_e36137_d_n6, assign18750_e36137_d_n7, assign18750_e36137_d_n8, assign18750_e36137_d_n9, assign18750_e36137_d_n10, assign18750_e36137_d_n11, assign18750_e36137_d_n13, assign18750_e36137_d_n14,) = {
                            if (assign18750_e36129 > 1e-38) {
                                let assign18750_e36134: f64 = (p.p97 / var_ni);
                                let assign18750_e36135: f64 = (assign18750_e36134).ln();
                                (assign18750_e36135, ((-((p.p97 * var_ni_dn0) / (var_ni * var_ni))) / assign18750_e36134), ((-((p.p97 * var_ni_dn2) / (var_ni * var_ni))) / assign18750_e36134), ((-((p.p97 * var_ni_dn3) / (var_ni * var_ni))) / assign18750_e36134), ((-((p.p97 * var_ni_dn4) / (var_ni * var_ni))) / assign18750_e36134), ((-((p.p97 * var_ni_dn5) / (var_ni * var_ni))) / assign18750_e36134), ((-((p.p97 * var_ni_dn6) / (var_ni * var_ni))) / assign18750_e36134), ((-((p.p97 * var_ni_dn7) / (var_ni * var_ni))) / assign18750_e36134), ((-((p.p97 * var_ni_dn8) / (var_ni * var_ni))) / assign18750_e36134), ((-((p.p97 * var_ni_dn9) / (var_ni * var_ni))) / assign18750_e36134), ((-((p.p97 * var_ni_dn10) / (var_ni * var_ni))) / assign18750_e36134), ((-((p.p97 * var_ni_dn11) / (var_ni * var_ni))) / assign18750_e36134), ((-((p.p97 * var_ni_dn13) / (var_ni * var_ni))) / assign18750_e36134), ((-((p.p97 * var_ni_dn14) / (var_ni * var_ni))) / assign18750_e36134),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18750_e36137, assign18750_e36137_d_n0, assign18750_e36137_d_n2, assign18750_e36137_d_n3, assign18750_e36137_d_n4, assign18750_e36137_d_n5, assign18750_e36137_d_n6, assign18750_e36137_d_n7, assign18750_e36137_d_n8, assign18750_e36137_d_n9, assign18750_e36137_d_n10, assign18750_e36137_d_n11, assign18750_e36137_d_n13, assign18750_e36137_d_n14,)
                    }
                };
                let assign18750_e36139: f64 = (var_vtm * assign18750_e36138);
                let assign18750_e36140: f64 = (assign18750_e36117 - assign18750_e36139);
                let assign18750_e36143: f64 = (0.5 * var_eg);
                let assign18750_e36147: f64 = (p.p97 / var_ni);
                let (assign18750_e36164, assign18750_e36164_d_n0, assign18750_e36164_d_n2, assign18750_e36164_d_n3, assign18750_e36164_d_n4, assign18750_e36164_d_n5, assign18750_e36164_d_n6, assign18750_e36164_d_n7, assign18750_e36164_d_n8, assign18750_e36164_d_n9, assign18750_e36164_d_n10, assign18750_e36164_d_n11, assign18750_e36164_d_n13, assign18750_e36164_d_n14,) = {
                    if (!(assign18750_e36147 > 1e-38)) {
                        let assign18750_e36152: f64 = (-87.498233534);
                        (assign18750_e36152, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18750_e36155: f64 = (p.p97 / var_ni);
                        let (assign18750_e36163, assign18750_e36163_d_n0, assign18750_e36163_d_n2, assign18750_e36163_d_n3, assign18750_e36163_d_n4, assign18750_e36163_d_n5, assign18750_e36163_d_n6, assign18750_e36163_d_n7, assign18750_e36163_d_n8, assign18750_e36163_d_n9, assign18750_e36163_d_n10, assign18750_e36163_d_n11, assign18750_e36163_d_n13, assign18750_e36163_d_n14,) = {
                            if (assign18750_e36155 > 1e-38) {
                                let assign18750_e36160: f64 = (p.p97 / var_ni);
                                let assign18750_e36161: f64 = (assign18750_e36160).ln();
                                (assign18750_e36161, ((-((p.p97 * var_ni_dn0) / (var_ni * var_ni))) / assign18750_e36160), ((-((p.p97 * var_ni_dn2) / (var_ni * var_ni))) / assign18750_e36160), ((-((p.p97 * var_ni_dn3) / (var_ni * var_ni))) / assign18750_e36160), ((-((p.p97 * var_ni_dn4) / (var_ni * var_ni))) / assign18750_e36160), ((-((p.p97 * var_ni_dn5) / (var_ni * var_ni))) / assign18750_e36160), ((-((p.p97 * var_ni_dn6) / (var_ni * var_ni))) / assign18750_e36160), ((-((p.p97 * var_ni_dn7) / (var_ni * var_ni))) / assign18750_e36160), ((-((p.p97 * var_ni_dn8) / (var_ni * var_ni))) / assign18750_e36160), ((-((p.p97 * var_ni_dn9) / (var_ni * var_ni))) / assign18750_e36160), ((-((p.p97 * var_ni_dn10) / (var_ni * var_ni))) / assign18750_e36160), ((-((p.p97 * var_ni_dn11) / (var_ni * var_ni))) / assign18750_e36160), ((-((p.p97 * var_ni_dn13) / (var_ni * var_ni))) / assign18750_e36160), ((-((p.p97 * var_ni_dn14) / (var_ni * var_ni))) / assign18750_e36160),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18750_e36163, assign18750_e36163_d_n0, assign18750_e36163_d_n2, assign18750_e36163_d_n3, assign18750_e36163_d_n4, assign18750_e36163_d_n5, assign18750_e36163_d_n6, assign18750_e36163_d_n7, assign18750_e36163_d_n8, assign18750_e36163_d_n9, assign18750_e36163_d_n10, assign18750_e36163_d_n11, assign18750_e36163_d_n13, assign18750_e36163_d_n14,)
                    }
                };
                let assign18750_e36165: f64 = (var_vtm * assign18750_e36164);
                let assign18750_e36166: f64 = (assign18750_e36143 - assign18750_e36165);
                let assign18750_e36167: f64 = (assign18750_e36140 * assign18750_e36166);
                let assign18750_e36170: f64 = (4.0 * 0.0001);
                let assign18750_e36172: f64 = (assign18750_e36170 * 0.0001);
                let assign18750_e36173: f64 = (assign18750_e36167 + assign18750_e36172);
                let assign18750_e36174: f64 = (assign18750_e36173).sqrt();
                let assign18750_e36175: f64 = (assign18750_e36114 + assign18750_e36174);
                let assign18750_e36176: f64 = (0.5 * assign18750_e36175);
                (assign18750_e36176, (0.5 * ((-(var_vtm * assign18750_e36112_d_n0)) + ((((-(var_vtm * assign18750_e36138_d_n0)) * assign18750_e36166) + (assign18750_e36140 * (-(var_vtm * assign18750_e36164_d_n0)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(var_vtm * assign18750_e36112_d_n2)) + ((((-(var_vtm * assign18750_e36138_d_n2)) * assign18750_e36166) + (assign18750_e36140 * (-(var_vtm * assign18750_e36164_d_n2)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(var_vtm * assign18750_e36112_d_n3)) + ((((-(var_vtm * assign18750_e36138_d_n3)) * assign18750_e36166) + (assign18750_e36140 * (-(var_vtm * assign18750_e36164_d_n3)))) / (2.0 * assign18750_e36174)))), (0.5 * (((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18750_e36112) + (var_vtm * assign18750_e36112_d_n4))) + (((((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18750_e36138) + (var_vtm * assign18750_e36138_d_n4))) * assign18750_e36166) + (assign18750_e36140 * ((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18750_e36164) + (var_vtm * assign18750_e36164_d_n4))))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(var_vtm * assign18750_e36112_d_n5)) + ((((-(var_vtm * assign18750_e36138_d_n5)) * assign18750_e36166) + (assign18750_e36140 * (-(var_vtm * assign18750_e36164_d_n5)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(var_vtm * assign18750_e36112_d_n6)) + ((((-(var_vtm * assign18750_e36138_d_n6)) * assign18750_e36166) + (assign18750_e36140 * (-(var_vtm * assign18750_e36164_d_n6)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(var_vtm * assign18750_e36112_d_n7)) + ((((-(var_vtm * assign18750_e36138_d_n7)) * assign18750_e36166) + (assign18750_e36140 * (-(var_vtm * assign18750_e36164_d_n7)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(var_vtm * assign18750_e36112_d_n8)) + ((((-(var_vtm * assign18750_e36138_d_n8)) * assign18750_e36166) + (assign18750_e36140 * (-(var_vtm * assign18750_e36164_d_n8)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(var_vtm * assign18750_e36112_d_n9)) + ((((-(var_vtm * assign18750_e36138_d_n9)) * assign18750_e36166) + (assign18750_e36140 * (-(var_vtm * assign18750_e36164_d_n9)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(var_vtm * assign18750_e36112_d_n10)) + ((((-(var_vtm * assign18750_e36138_d_n10)) * assign18750_e36166) + (assign18750_e36140 * (-(var_vtm * assign18750_e36164_d_n10)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(var_vtm * assign18750_e36112_d_n11)) + ((((-(var_vtm * assign18750_e36138_d_n11)) * assign18750_e36166) + (assign18750_e36140 * (-(var_vtm * assign18750_e36164_d_n11)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(var_vtm * assign18750_e36112_d_n13)) + ((((-(var_vtm * assign18750_e36138_d_n13)) * assign18750_e36166) + (assign18750_e36140 * (-(var_vtm * assign18750_e36164_d_n13)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(var_vtm * assign18750_e36112_d_n14)) + ((((-(var_vtm * assign18750_e36138_d_n14)) * assign18750_e36166) + (assign18750_e36140 * (-(var_vtm * assign18750_e36164_d_n14)))) / (2.0 * assign18750_e36174)))),)
            } else {
                let assign18750_e36179: f64 = (0.5 * var_eg);
                let assign18750_e36183: f64 = (p.p97 / var_ni);
                let (assign18750_e36200, assign18750_e36200_d_n0, assign18750_e36200_d_n2, assign18750_e36200_d_n3, assign18750_e36200_d_n4, assign18750_e36200_d_n5, assign18750_e36200_d_n6, assign18750_e36200_d_n7, assign18750_e36200_d_n8, assign18750_e36200_d_n9, assign18750_e36200_d_n10, assign18750_e36200_d_n11, assign18750_e36200_d_n13, assign18750_e36200_d_n14,) = {
                    if (!(assign18750_e36183 > 1e-38)) {
                        let assign18750_e36188: f64 = (-87.498233534);
                        (assign18750_e36188, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18750_e36191: f64 = (p.p97 / var_ni);
                        let (assign18750_e36199, assign18750_e36199_d_n0, assign18750_e36199_d_n2, assign18750_e36199_d_n3, assign18750_e36199_d_n4, assign18750_e36199_d_n5, assign18750_e36199_d_n6, assign18750_e36199_d_n7, assign18750_e36199_d_n8, assign18750_e36199_d_n9, assign18750_e36199_d_n10, assign18750_e36199_d_n11, assign18750_e36199_d_n13, assign18750_e36199_d_n14,) = {
                            if (assign18750_e36191 > 1e-38) {
                                let assign18750_e36196: f64 = (p.p97 / var_ni);
                                let assign18750_e36197: f64 = (assign18750_e36196).ln();
                                (assign18750_e36197, ((-((p.p97 * var_ni_dn0) / (var_ni * var_ni))) / assign18750_e36196), ((-((p.p97 * var_ni_dn2) / (var_ni * var_ni))) / assign18750_e36196), ((-((p.p97 * var_ni_dn3) / (var_ni * var_ni))) / assign18750_e36196), ((-((p.p97 * var_ni_dn4) / (var_ni * var_ni))) / assign18750_e36196), ((-((p.p97 * var_ni_dn5) / (var_ni * var_ni))) / assign18750_e36196), ((-((p.p97 * var_ni_dn6) / (var_ni * var_ni))) / assign18750_e36196), ((-((p.p97 * var_ni_dn7) / (var_ni * var_ni))) / assign18750_e36196), ((-((p.p97 * var_ni_dn8) / (var_ni * var_ni))) / assign18750_e36196), ((-((p.p97 * var_ni_dn9) / (var_ni * var_ni))) / assign18750_e36196), ((-((p.p97 * var_ni_dn10) / (var_ni * var_ni))) / assign18750_e36196), ((-((p.p97 * var_ni_dn11) / (var_ni * var_ni))) / assign18750_e36196), ((-((p.p97 * var_ni_dn13) / (var_ni * var_ni))) / assign18750_e36196), ((-((p.p97 * var_ni_dn14) / (var_ni * var_ni))) / assign18750_e36196),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18750_e36199, assign18750_e36199_d_n0, assign18750_e36199_d_n2, assign18750_e36199_d_n3, assign18750_e36199_d_n4, assign18750_e36199_d_n5, assign18750_e36199_d_n6, assign18750_e36199_d_n7, assign18750_e36199_d_n8, assign18750_e36199_d_n9, assign18750_e36199_d_n10, assign18750_e36199_d_n11, assign18750_e36199_d_n13, assign18750_e36199_d_n14,)
                    }
                };
                let assign18750_e36201: f64 = (var_vtm * assign18750_e36200);
                let assign18750_e36202: f64 = (assign18750_e36179 - assign18750_e36201);
                let assign18750_e36204: f64 = (-10000.0);
                let assign18750_e36206: f64 = (assign18750_e36204 * 0.0001);
                let (assign18750_e36240, assign18750_e36240_d_n0, assign18750_e36240_d_n2, assign18750_e36240_d_n3, assign18750_e36240_d_n4, assign18750_e36240_d_n5, assign18750_e36240_d_n6, assign18750_e36240_d_n7, assign18750_e36240_d_n8, assign18750_e36240_d_n9, assign18750_e36240_d_n10, assign18750_e36240_d_n11, assign18750_e36240_d_n13, assign18750_e36240_d_n14,) = {
                    if (assign18750_e36202 < assign18750_e36206) {
                        let assign18750_e36209: f64 = (-0.0001);
                        let assign18750_e36211: f64 = (assign18750_e36209 * 0.0001);
                        let assign18750_e36214: f64 = (0.5 * var_eg);
                        let assign18750_e36218: f64 = (p.p97 / var_ni);
                        let (assign18750_e36235, assign18750_e36235_d_n0, assign18750_e36235_d_n2, assign18750_e36235_d_n3, assign18750_e36235_d_n4, assign18750_e36235_d_n5, assign18750_e36235_d_n6, assign18750_e36235_d_n7, assign18750_e36235_d_n8, assign18750_e36235_d_n9, assign18750_e36235_d_n10, assign18750_e36235_d_n11, assign18750_e36235_d_n13, assign18750_e36235_d_n14,) = {
                            if (!(assign18750_e36218 > 1e-38)) {
                                let assign18750_e36223: f64 = (-87.498233534);
                                (assign18750_e36223, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            } else {
                                let assign18750_e36226: f64 = (p.p97 / var_ni);
                                let (assign18750_e36234, assign18750_e36234_d_n0, assign18750_e36234_d_n2, assign18750_e36234_d_n3, assign18750_e36234_d_n4, assign18750_e36234_d_n5, assign18750_e36234_d_n6, assign18750_e36234_d_n7, assign18750_e36234_d_n8, assign18750_e36234_d_n9, assign18750_e36234_d_n10, assign18750_e36234_d_n11, assign18750_e36234_d_n13, assign18750_e36234_d_n14,) = {
                                    if (assign18750_e36226 > 1e-38) {
                                        let assign18750_e36231: f64 = (p.p97 / var_ni);
                                        let assign18750_e36232: f64 = (assign18750_e36231).ln();
                                        (assign18750_e36232, ((-((p.p97 * var_ni_dn0) / (var_ni * var_ni))) / assign18750_e36231), ((-((p.p97 * var_ni_dn2) / (var_ni * var_ni))) / assign18750_e36231), ((-((p.p97 * var_ni_dn3) / (var_ni * var_ni))) / assign18750_e36231), ((-((p.p97 * var_ni_dn4) / (var_ni * var_ni))) / assign18750_e36231), ((-((p.p97 * var_ni_dn5) / (var_ni * var_ni))) / assign18750_e36231), ((-((p.p97 * var_ni_dn6) / (var_ni * var_ni))) / assign18750_e36231), ((-((p.p97 * var_ni_dn7) / (var_ni * var_ni))) / assign18750_e36231), ((-((p.p97 * var_ni_dn8) / (var_ni * var_ni))) / assign18750_e36231), ((-((p.p97 * var_ni_dn9) / (var_ni * var_ni))) / assign18750_e36231), ((-((p.p97 * var_ni_dn10) / (var_ni * var_ni))) / assign18750_e36231), ((-((p.p97 * var_ni_dn11) / (var_ni * var_ni))) / assign18750_e36231), ((-((p.p97 * var_ni_dn13) / (var_ni * var_ni))) / assign18750_e36231), ((-((p.p97 * var_ni_dn14) / (var_ni * var_ni))) / assign18750_e36231),)
                                    } else {
                                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                                    }
                                };
                                (assign18750_e36234, assign18750_e36234_d_n0, assign18750_e36234_d_n2, assign18750_e36234_d_n3, assign18750_e36234_d_n4, assign18750_e36234_d_n5, assign18750_e36234_d_n6, assign18750_e36234_d_n7, assign18750_e36234_d_n8, assign18750_e36234_d_n9, assign18750_e36234_d_n10, assign18750_e36234_d_n11, assign18750_e36234_d_n13, assign18750_e36234_d_n14,)
                            }
                        };
                        let assign18750_e36236: f64 = (var_vtm * assign18750_e36235);
                        let assign18750_e36237: f64 = (assign18750_e36214 - assign18750_e36236);
                        let assign18750_e36238: f64 = (assign18750_e36211 / assign18750_e36237);
                        (assign18750_e36238, (-((assign18750_e36211 * (-(var_vtm * assign18750_e36235_d_n0))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(var_vtm * assign18750_e36235_d_n2))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(var_vtm * assign18750_e36235_d_n3))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * ((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18750_e36235) + (var_vtm * assign18750_e36235_d_n4)))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(var_vtm * assign18750_e36235_d_n5))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(var_vtm * assign18750_e36235_d_n6))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(var_vtm * assign18750_e36235_d_n7))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(var_vtm * assign18750_e36235_d_n8))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(var_vtm * assign18750_e36235_d_n9))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(var_vtm * assign18750_e36235_d_n10))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(var_vtm * assign18750_e36235_d_n11))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(var_vtm * assign18750_e36235_d_n13))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(var_vtm * assign18750_e36235_d_n14))) / (assign18750_e36237 * assign18750_e36237))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18750_e36240, assign18750_e36240_d_n0, assign18750_e36240_d_n2, assign18750_e36240_d_n3, assign18750_e36240_d_n4, assign18750_e36240_d_n5, assign18750_e36240_d_n6, assign18750_e36240_d_n7, assign18750_e36240_d_n8, assign18750_e36240_d_n9, assign18750_e36240_d_n10, assign18750_e36240_d_n11, assign18750_e36240_d_n13, assign18750_e36240_d_n14,)
            }
        };
        let assign18750_e36242: f64 = (assign18750_e36055 - assign18750_e36241);
        let assign18750_e36243: f64 = (var_devsign * assign18750_e36242);
        let assign18750_e36244: f64 = (assign18750_e36051 - assign18750_e36243);
        let assign18750_e36245: f64 = (var_phig_i - assign18750_e36244);
        let assign18750_e36246: f64 = (var_devsign * assign18750_e36245);
        (assign18750_e36246, (var_devsign * (var_phig_i_dn0 - (-(var_devsign * (-assign18750_e36241_d_n0))))), (var_devsign * (var_phig_i_dn2 - (-(var_devsign * (-assign18750_e36241_d_n2))))), (var_devsign * (var_phig_i_dn3 - (-(var_devsign * (-assign18750_e36241_d_n3))))), (var_devsign * (var_phig_i_dn4 - ((0.5 * var_eg_dn4) - (var_devsign * ((0.5 * var_eg_dn4) - assign18750_e36241_d_n4))))), (var_devsign * (var_phig_i_dn5 - (-(var_devsign * (-assign18750_e36241_d_n5))))), (var_devsign * (var_phig_i_dn6 - (-(var_devsign * (-assign18750_e36241_d_n6))))), (var_devsign * (var_phig_i_dn7 - (-(var_devsign * (-assign18750_e36241_d_n7))))), (var_devsign * (var_phig_i_dn8 - (-(var_devsign * (-assign18750_e36241_d_n8))))), (var_devsign * (var_phig_i_dn9 - (-(var_devsign * (-assign18750_e36241_d_n9))))), (var_devsign * (var_phig_i_dn10 - (-(var_devsign * (-assign18750_e36241_d_n10))))), (var_devsign * (var_phig_i_dn11 - (-(var_devsign * (-assign18750_e36241_d_n11))))), (var_devsign * (var_phig_i_dn13 - (-(var_devsign * (-assign18750_e36241_d_n13))))), (var_devsign * (var_phig_i_dn14 - (-(var_devsign * (-assign18750_e36241_d_n14))))),)
    } else {
        (var_vfbsd_v, var_vfbsd_v_dn0, var_vfbsd_v_dn2, var_vfbsd_v_dn3, var_vfbsd_v_dn4, var_vfbsd_v_dn5, var_vfbsd_v_dn6, var_vfbsd_v_dn7, var_vfbsd_v_dn8, var_vfbsd_v_dn9, var_vfbsd_v_dn10, var_vfbsd_v_dn11, var_vfbsd_v_dn13, var_vfbsd_v_dn14,)
    }
};
        var_vfbsd_v = assign18750_e36248;
        var_vfbsd_v_dn0 = assign18750_e36248_d_n0;
        var_vfbsd_v_dn2 = assign18750_e36248_d_n2;
        var_vfbsd_v_dn3 = assign18750_e36248_d_n3;
        var_vfbsd_v_dn4 = assign18750_e36248_d_n4;
        var_vfbsd_v_dn5 = assign18750_e36248_d_n5;
        var_vfbsd_v_dn6 = assign18750_e36248_d_n6;
        var_vfbsd_v_dn7 = assign18750_e36248_d_n7;
        var_vfbsd_v_dn8 = assign18750_e36248_d_n8;
        var_vfbsd_v_dn9 = assign18750_e36248_d_n9;
        var_vfbsd_v_dn10 = assign18750_e36248_d_n10;
        var_vfbsd_v_dn11 = assign18750_e36248_d_n11;
        var_vfbsd_v_dn13 = assign18750_e36248_d_n13;
        var_vfbsd_v_dn14 = assign18750_e36248_d_n14;
        var_vfbsd_v_rv = 0.0;

        let (assign18760_e36435, assign18760_e36435_d_n0, assign18760_e36435_d_n2, assign18760_e36435_d_n3, assign18760_e36435_d_n4, assign18760_e36435_d_n5, assign18760_e36435_d_n6, assign18760_e36435_d_n7, assign18760_e36435_d_n8, assign18760_e36435_d_n9, assign18760_e36435_d_n10, assign18760_e36435_d_n11, assign18760_e36435_d_n13, assign18760_e36435_d_n14,) = {
    if (((var_guard343 != 0.0) && (var_guard344 == 0.0)) && (var_guard346 == 0.0)) {
        let assign18760_e36261: f64 = (0.5 * var_eg);
        let assign18760_e36262: f64 = (p.p104 + assign18760_e36261);
        let assign18760_e36266: f64 = (0.5 * var_eg);
        let assign18760_e36269: f64 = (0.5 * var_eg);
        let (assign18760_e36284,) = {
            if (!(p.p97 > 1e-38)) {
                let assign18760_e36276: f64 = (-87.498233534);
                (assign18760_e36276,)
            } else {
                let (assign18760_e36283,) = {
                    if (p.p97 > 1e-38) {
                        let assign18760_e36281: f64 = (p.p97).ln();
                        (assign18760_e36281,)
                    } else {
                        (0.0,)
                    }
                };
                (assign18760_e36283,)
            }
        };
        let assign18760_e36286: f64 = (assign18760_e36284 - var_niln);
        let assign18760_e36287: f64 = (var_vtm * assign18760_e36286);
        let assign18760_e36288: f64 = (assign18760_e36269 - assign18760_e36287);
        let assign18760_e36290: f64 = (-10000.0);
        let assign18760_e36292: f64 = (assign18760_e36290 * 0.0001);
        let (assign18760_e36428, assign18760_e36428_d_n0, assign18760_e36428_d_n2, assign18760_e36428_d_n3, assign18760_e36428_d_n4, assign18760_e36428_d_n5, assign18760_e36428_d_n6, assign18760_e36428_d_n7, assign18760_e36428_d_n8, assign18760_e36428_d_n9, assign18760_e36428_d_n10, assign18760_e36428_d_n11, assign18760_e36428_d_n13, assign18760_e36428_d_n14,) = {
            if (!(assign18760_e36288 < assign18760_e36292)) {
                let assign18760_e36298: f64 = (0.5 * var_eg);
                let (assign18760_e36313,) = {
                    if (!(p.p97 > 1e-38)) {
                        let assign18760_e36305: f64 = (-87.498233534);
                        (assign18760_e36305,)
                    } else {
                        let (assign18760_e36312,) = {
                            if (p.p97 > 1e-38) {
                                let assign18760_e36310: f64 = (p.p97).ln();
                                (assign18760_e36310,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18760_e36312,)
                    }
                };
                let assign18760_e36315: f64 = (assign18760_e36313 - var_niln);
                let assign18760_e36316: f64 = (var_vtm * assign18760_e36315);
                let assign18760_e36317: f64 = (assign18760_e36298 - assign18760_e36316);
                let assign18760_e36320: f64 = (0.5 * var_eg);
                let (assign18760_e36335,) = {
                    if (!(p.p97 > 1e-38)) {
                        let assign18760_e36327: f64 = (-87.498233534);
                        (assign18760_e36327,)
                    } else {
                        let (assign18760_e36334,) = {
                            if (p.p97 > 1e-38) {
                                let assign18760_e36332: f64 = (p.p97).ln();
                                (assign18760_e36332,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18760_e36334,)
                    }
                };
                let assign18760_e36337: f64 = (assign18760_e36335 - var_niln);
                let assign18760_e36338: f64 = (var_vtm * assign18760_e36337);
                let assign18760_e36339: f64 = (assign18760_e36320 - assign18760_e36338);
                let assign18760_e36342: f64 = (0.5 * var_eg);
                let (assign18760_e36357,) = {
                    if (!(p.p97 > 1e-38)) {
                        let assign18760_e36349: f64 = (-87.498233534);
                        (assign18760_e36349,)
                    } else {
                        let (assign18760_e36356,) = {
                            if (p.p97 > 1e-38) {
                                let assign18760_e36354: f64 = (p.p97).ln();
                                (assign18760_e36354,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18760_e36356,)
                    }
                };
                let assign18760_e36359: f64 = (assign18760_e36357 - var_niln);
                let assign18760_e36360: f64 = (var_vtm * assign18760_e36359);
                let assign18760_e36361: f64 = (assign18760_e36342 - assign18760_e36360);
                let assign18760_e36362: f64 = (assign18760_e36339 * assign18760_e36361);
                let assign18760_e36365: f64 = (4.0 * 0.0001);
                let assign18760_e36367: f64 = (assign18760_e36365 * 0.0001);
                let assign18760_e36368: f64 = (assign18760_e36362 + assign18760_e36367);
                let assign18760_e36369: f64 = (assign18760_e36368).sqrt();
                let assign18760_e36370: f64 = (assign18760_e36317 + assign18760_e36369);
                let assign18760_e36371: f64 = (0.5 * assign18760_e36370);
                (assign18760_e36371, (0.5 * ((-(var_vtm * (-var_niln_dn0))) + ((((-(var_vtm * (-var_niln_dn0))) * assign18760_e36361) + (assign18760_e36339 * (-(var_vtm * (-var_niln_dn0))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(var_vtm * (-var_niln_dn2))) + ((((-(var_vtm * (-var_niln_dn2))) * assign18760_e36361) + (assign18760_e36339 * (-(var_vtm * (-var_niln_dn2))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(var_vtm * (-var_niln_dn3))) + ((((-(var_vtm * (-var_niln_dn3))) * assign18760_e36361) + (assign18760_e36339 * (-(var_vtm * (-var_niln_dn3))))) / (2.0 * assign18760_e36369)))), (0.5 * (((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18760_e36315) + (var_vtm * (-var_niln_dn4)))) + (((((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18760_e36337) + (var_vtm * (-var_niln_dn4)))) * assign18760_e36361) + (assign18760_e36339 * ((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18760_e36359) + (var_vtm * (-var_niln_dn4)))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(var_vtm * (-var_niln_dn5))) + ((((-(var_vtm * (-var_niln_dn5))) * assign18760_e36361) + (assign18760_e36339 * (-(var_vtm * (-var_niln_dn5))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(var_vtm * (-var_niln_dn6))) + ((((-(var_vtm * (-var_niln_dn6))) * assign18760_e36361) + (assign18760_e36339 * (-(var_vtm * (-var_niln_dn6))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(var_vtm * (-var_niln_dn7))) + ((((-(var_vtm * (-var_niln_dn7))) * assign18760_e36361) + (assign18760_e36339 * (-(var_vtm * (-var_niln_dn7))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(var_vtm * (-var_niln_dn8))) + ((((-(var_vtm * (-var_niln_dn8))) * assign18760_e36361) + (assign18760_e36339 * (-(var_vtm * (-var_niln_dn8))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(var_vtm * (-var_niln_dn9))) + ((((-(var_vtm * (-var_niln_dn9))) * assign18760_e36361) + (assign18760_e36339 * (-(var_vtm * (-var_niln_dn9))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(var_vtm * (-var_niln_dn10))) + ((((-(var_vtm * (-var_niln_dn10))) * assign18760_e36361) + (assign18760_e36339 * (-(var_vtm * (-var_niln_dn10))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(var_vtm * (-var_niln_dn11))) + ((((-(var_vtm * (-var_niln_dn11))) * assign18760_e36361) + (assign18760_e36339 * (-(var_vtm * (-var_niln_dn11))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(var_vtm * (-var_niln_dn13))) + ((((-(var_vtm * (-var_niln_dn13))) * assign18760_e36361) + (assign18760_e36339 * (-(var_vtm * (-var_niln_dn13))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(var_vtm * (-var_niln_dn14))) + ((((-(var_vtm * (-var_niln_dn14))) * assign18760_e36361) + (assign18760_e36339 * (-(var_vtm * (-var_niln_dn14))))) / (2.0 * assign18760_e36369)))),)
            } else {
                let assign18760_e36374: f64 = (0.5 * var_eg);
                let (assign18760_e36389,) = {
                    if (!(p.p97 > 1e-38)) {
                        let assign18760_e36381: f64 = (-87.498233534);
                        (assign18760_e36381,)
                    } else {
                        let (assign18760_e36388,) = {
                            if (p.p97 > 1e-38) {
                                let assign18760_e36386: f64 = (p.p97).ln();
                                (assign18760_e36386,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18760_e36388,)
                    }
                };
                let assign18760_e36391: f64 = (assign18760_e36389 - var_niln);
                let assign18760_e36392: f64 = (var_vtm * assign18760_e36391);
                let assign18760_e36393: f64 = (assign18760_e36374 - assign18760_e36392);
                let assign18760_e36395: f64 = (-10000.0);
                let assign18760_e36397: f64 = (assign18760_e36395 * 0.0001);
                let (assign18760_e36427, assign18760_e36427_d_n0, assign18760_e36427_d_n2, assign18760_e36427_d_n3, assign18760_e36427_d_n4, assign18760_e36427_d_n5, assign18760_e36427_d_n6, assign18760_e36427_d_n7, assign18760_e36427_d_n8, assign18760_e36427_d_n9, assign18760_e36427_d_n10, assign18760_e36427_d_n11, assign18760_e36427_d_n13, assign18760_e36427_d_n14,) = {
                    if (assign18760_e36393 < assign18760_e36397) {
                        let assign18760_e36400: f64 = (-0.0001);
                        let assign18760_e36402: f64 = (assign18760_e36400 * 0.0001);
                        let assign18760_e36405: f64 = (0.5 * var_eg);
                        let (assign18760_e36420,) = {
                            if (!(p.p97 > 1e-38)) {
                                let assign18760_e36412: f64 = (-87.498233534);
                                (assign18760_e36412,)
                            } else {
                                let (assign18760_e36419,) = {
                                    if (p.p97 > 1e-38) {
                                        let assign18760_e36417: f64 = (p.p97).ln();
                                        (assign18760_e36417,)
                                    } else {
                                        (0.0,)
                                    }
                                };
                                (assign18760_e36419,)
                            }
                        };
                        let assign18760_e36422: f64 = (assign18760_e36420 - var_niln);
                        let assign18760_e36423: f64 = (var_vtm * assign18760_e36422);
                        let assign18760_e36424: f64 = (assign18760_e36405 - assign18760_e36423);
                        let assign18760_e36425: f64 = (assign18760_e36402 / assign18760_e36424);
                        (assign18760_e36425, (-((assign18760_e36402 * (-(var_vtm * (-var_niln_dn0)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(var_vtm * (-var_niln_dn2)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(var_vtm * (-var_niln_dn3)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * ((0.5 * var_eg_dn4) - ((var_vtm_dn4 * assign18760_e36422) + (var_vtm * (-var_niln_dn4))))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(var_vtm * (-var_niln_dn5)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(var_vtm * (-var_niln_dn6)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(var_vtm * (-var_niln_dn7)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(var_vtm * (-var_niln_dn8)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(var_vtm * (-var_niln_dn9)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(var_vtm * (-var_niln_dn10)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(var_vtm * (-var_niln_dn11)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(var_vtm * (-var_niln_dn13)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(var_vtm * (-var_niln_dn14)))) / (assign18760_e36424 * assign18760_e36424))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18760_e36427, assign18760_e36427_d_n0, assign18760_e36427_d_n2, assign18760_e36427_d_n3, assign18760_e36427_d_n4, assign18760_e36427_d_n5, assign18760_e36427_d_n6, assign18760_e36427_d_n7, assign18760_e36427_d_n8, assign18760_e36427_d_n9, assign18760_e36427_d_n10, assign18760_e36427_d_n11, assign18760_e36427_d_n13, assign18760_e36427_d_n14,)
            }
        };
        let assign18760_e36429: f64 = (assign18760_e36266 - assign18760_e36428);
        let assign18760_e36430: f64 = (var_devsign * assign18760_e36429);
        let assign18760_e36431: f64 = (assign18760_e36262 - assign18760_e36430);
        let assign18760_e36432: f64 = (var_phig_i - assign18760_e36431);
        let assign18760_e36433: f64 = (var_devsign * assign18760_e36432);
        (assign18760_e36433, (var_devsign * (var_phig_i_dn0 - (-(var_devsign * (-assign18760_e36428_d_n0))))), (var_devsign * (var_phig_i_dn2 - (-(var_devsign * (-assign18760_e36428_d_n2))))), (var_devsign * (var_phig_i_dn3 - (-(var_devsign * (-assign18760_e36428_d_n3))))), (var_devsign * (var_phig_i_dn4 - ((0.5 * var_eg_dn4) - (var_devsign * ((0.5 * var_eg_dn4) - assign18760_e36428_d_n4))))), (var_devsign * (var_phig_i_dn5 - (-(var_devsign * (-assign18760_e36428_d_n5))))), (var_devsign * (var_phig_i_dn6 - (-(var_devsign * (-assign18760_e36428_d_n6))))), (var_devsign * (var_phig_i_dn7 - (-(var_devsign * (-assign18760_e36428_d_n7))))), (var_devsign * (var_phig_i_dn8 - (-(var_devsign * (-assign18760_e36428_d_n8))))), (var_devsign * (var_phig_i_dn9 - (-(var_devsign * (-assign18760_e36428_d_n9))))), (var_devsign * (var_phig_i_dn10 - (-(var_devsign * (-assign18760_e36428_d_n10))))), (var_devsign * (var_phig_i_dn11 - (-(var_devsign * (-assign18760_e36428_d_n11))))), (var_devsign * (var_phig_i_dn13 - (-(var_devsign * (-assign18760_e36428_d_n13))))), (var_devsign * (var_phig_i_dn14 - (-(var_devsign * (-assign18760_e36428_d_n14))))),)
    } else {
        (var_vfbsd_v, var_vfbsd_v_dn0, var_vfbsd_v_dn2, var_vfbsd_v_dn3, var_vfbsd_v_dn4, var_vfbsd_v_dn5, var_vfbsd_v_dn6, var_vfbsd_v_dn7, var_vfbsd_v_dn8, var_vfbsd_v_dn9, var_vfbsd_v_dn10, var_vfbsd_v_dn11, var_vfbsd_v_dn13, var_vfbsd_v_dn14,)
    }
};
        var_vfbsd_v = assign18760_e36435;
        var_vfbsd_v_dn0 = assign18760_e36435_d_n0;
        var_vfbsd_v_dn2 = assign18760_e36435_d_n2;
        var_vfbsd_v_dn3 = assign18760_e36435_d_n3;
        var_vfbsd_v_dn4 = assign18760_e36435_d_n4;
        var_vfbsd_v_dn5 = assign18760_e36435_d_n5;
        var_vfbsd_v_dn6 = assign18760_e36435_d_n6;
        var_vfbsd_v_dn7 = assign18760_e36435_d_n7;
        var_vfbsd_v_dn8 = assign18760_e36435_d_n8;
        var_vfbsd_v_dn9 = assign18760_e36435_d_n9;
        var_vfbsd_v_dn10 = assign18760_e36435_d_n10;
        var_vfbsd_v_dn11 = assign18760_e36435_d_n11;
        var_vfbsd_v_dn13 = assign18760_e36435_d_n13;
        var_vfbsd_v_dn14 = assign18760_e36435_d_n14;
        var_vfbsd_v_rv = 0.0;

        let (assign18770_e36440, assign18770_e36440_d_n0, assign18770_e36440_d_n2, assign18770_e36440_d_n3, assign18770_e36440_d_n4, assign18770_e36440_d_n5, assign18770_e36440_d_n6, assign18770_e36440_d_n7, assign18770_e36440_d_n8, assign18770_e36440_d_n9, assign18770_e36440_d_n10, assign18770_e36440_d_n11, assign18770_e36440_d_n13, assign18770_e36440_d_n14,) = {
    if (var_guard343 == 0.0) {
        (p.p1106, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfbsd_v, var_vfbsd_v_dn0, var_vfbsd_v_dn2, var_vfbsd_v_dn3, var_vfbsd_v_dn4, var_vfbsd_v_dn5, var_vfbsd_v_dn6, var_vfbsd_v_dn7, var_vfbsd_v_dn8, var_vfbsd_v_dn9, var_vfbsd_v_dn10, var_vfbsd_v_dn11, var_vfbsd_v_dn13, var_vfbsd_v_dn14,)
    }
};
        var_vfbsd_v = assign18770_e36440;
        var_vfbsd_v_dn0 = assign18770_e36440_d_n0;
        var_vfbsd_v_dn2 = assign18770_e36440_d_n2;
        var_vfbsd_v_dn3 = assign18770_e36440_d_n3;
        var_vfbsd_v_dn4 = assign18770_e36440_d_n4;
        var_vfbsd_v_dn5 = assign18770_e36440_d_n5;
        var_vfbsd_v_dn6 = assign18770_e36440_d_n6;
        var_vfbsd_v_dn7 = assign18770_e36440_d_n7;
        var_vfbsd_v_dn8 = assign18770_e36440_d_n8;
        var_vfbsd_v_dn9 = assign18770_e36440_d_n9;
        var_vfbsd_v_dn10 = assign18770_e36440_d_n10;
        var_vfbsd_v_dn11 = assign18770_e36440_d_n11;
        var_vfbsd_v_dn13 = assign18770_e36440_d_n13;
        var_vfbsd_v_dn14 = assign18770_e36440_d_n14;
        var_vfbsd_v_rv = 0.0;

        let assign18780_e36443: f64 = if (!param_given[1107]) { 1.0 } else { 0.0 };
        var_guard347 = assign18780_e36443;
        var_guard347_rv = 0.0;

        let (assign18790_e36447, assign18790_e36447_d_n0, assign18790_e36447_d_n2, assign18790_e36447_d_n3, assign18790_e36447_d_n4, assign18790_e36447_d_n5, assign18790_e36447_d_n6, assign18790_e36447_d_n7, assign18790_e36447_d_n8, assign18790_e36447_d_n9, assign18790_e36447_d_n10, assign18790_e36447_d_n11, assign18790_e36447_d_n13, assign18790_e36447_d_n14,) = {
    if (var_guard347 != 0.0) {
        (var_vfbsd_v, var_vfbsd_v_dn0, var_vfbsd_v_dn2, var_vfbsd_v_dn3, var_vfbsd_v_dn4, var_vfbsd_v_dn5, var_vfbsd_v_dn6, var_vfbsd_v_dn7, var_vfbsd_v_dn8, var_vfbsd_v_dn9, var_vfbsd_v_dn10, var_vfbsd_v_dn11, var_vfbsd_v_dn13, var_vfbsd_v_dn14,)
    } else {
        (var_vfbsdcv_v, var_vfbsdcv_v_dn0, var_vfbsdcv_v_dn2, var_vfbsdcv_v_dn3, var_vfbsdcv_v_dn4, var_vfbsdcv_v_dn5, var_vfbsdcv_v_dn6, var_vfbsdcv_v_dn7, var_vfbsdcv_v_dn8, var_vfbsdcv_v_dn9, var_vfbsdcv_v_dn10, var_vfbsdcv_v_dn11, var_vfbsdcv_v_dn13, var_vfbsdcv_v_dn14,)
    }
};
        var_vfbsdcv_v = assign18790_e36447;
        var_vfbsdcv_v_dn0 = assign18790_e36447_d_n0;
        var_vfbsdcv_v_dn2 = assign18790_e36447_d_n2;
        var_vfbsdcv_v_dn3 = assign18790_e36447_d_n3;
        var_vfbsdcv_v_dn4 = assign18790_e36447_d_n4;
        var_vfbsdcv_v_dn5 = assign18790_e36447_d_n5;
        var_vfbsdcv_v_dn6 = assign18790_e36447_d_n6;
        var_vfbsdcv_v_dn7 = assign18790_e36447_d_n7;
        var_vfbsdcv_v_dn8 = assign18790_e36447_d_n8;
        var_vfbsdcv_v_dn9 = assign18790_e36447_d_n9;
        var_vfbsdcv_v_dn10 = assign18790_e36447_d_n10;
        var_vfbsdcv_v_dn11 = assign18790_e36447_d_n11;
        var_vfbsdcv_v_dn13 = assign18790_e36447_d_n13;
        var_vfbsdcv_v_dn14 = assign18790_e36447_d_n14;
        var_vfbsdcv_v_rv = 0.0;

        let (assign18800_e36452, assign18800_e36452_d_n0, assign18800_e36452_d_n2, assign18800_e36452_d_n3, assign18800_e36452_d_n4, assign18800_e36452_d_n5, assign18800_e36452_d_n6, assign18800_e36452_d_n7, assign18800_e36452_d_n8, assign18800_e36452_d_n9, assign18800_e36452_d_n10, assign18800_e36452_d_n11, assign18800_e36452_d_n13, assign18800_e36452_d_n14,) = {
    if (var_guard347 == 0.0) {
        (p.p1107, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfbsdcv_v, var_vfbsdcv_v_dn0, var_vfbsdcv_v_dn2, var_vfbsdcv_v_dn3, var_vfbsdcv_v_dn4, var_vfbsdcv_v_dn5, var_vfbsdcv_v_dn6, var_vfbsdcv_v_dn7, var_vfbsdcv_v_dn8, var_vfbsdcv_v_dn9, var_vfbsdcv_v_dn10, var_vfbsdcv_v_dn11, var_vfbsdcv_v_dn13, var_vfbsdcv_v_dn14,)
    }
};
        var_vfbsdcv_v = assign18800_e36452;
        var_vfbsdcv_v_dn0 = assign18800_e36452_d_n0;
        var_vfbsdcv_v_dn2 = assign18800_e36452_d_n2;
        var_vfbsdcv_v_dn3 = assign18800_e36452_d_n3;
        var_vfbsdcv_v_dn4 = assign18800_e36452_d_n4;
        var_vfbsdcv_v_dn5 = assign18800_e36452_d_n5;
        var_vfbsdcv_v_dn6 = assign18800_e36452_d_n6;
        var_vfbsdcv_v_dn7 = assign18800_e36452_d_n7;
        var_vfbsdcv_v_dn8 = assign18800_e36452_d_n8;
        var_vfbsdcv_v_dn9 = assign18800_e36452_d_n9;
        var_vfbsdcv_v_dn10 = assign18800_e36452_d_n10;
        var_vfbsdcv_v_dn11 = assign18800_e36452_d_n11;
        var_vfbsdcv_v_dn13 = assign18800_e36452_d_n13;
        var_vfbsdcv_v_dn14 = assign18800_e36452_d_n14;
        var_vfbsdcv_v_rv = 0.0;

        let assign18810_e36455: f64 = if p.p80 == 0.0 { 1.0 } else { 0.0 };
        var_guard348 = assign18810_e36455;
        var_guard348_rv = 0.0;

        let (assign18820_e36480, assign18820_e36480_d_n0, assign18820_e36480_d_n2, assign18820_e36480_d_n3, assign18820_e36480_d_n4, assign18820_e36480_d_n5, assign18820_e36480_d_n6, assign18820_e36480_d_n7, assign18820_e36480_d_n8, assign18820_e36480_d_n9, assign18820_e36480_d_n10, assign18820_e36480_d_n11, assign18820_e36480_d_n13, assign18820_e36480_d_n14,) = {
    if (var_guard348 != 0.0) {
        let assign18820_e36460: f64 = (var_nbody_i / var_ni);
        let (assign18820_e36477, assign18820_e36477_d_n0, assign18820_e36477_d_n2, assign18820_e36477_d_n3, assign18820_e36477_d_n4, assign18820_e36477_d_n5, assign18820_e36477_d_n6, assign18820_e36477_d_n7, assign18820_e36477_d_n8, assign18820_e36477_d_n9, assign18820_e36477_d_n10, assign18820_e36477_d_n11, assign18820_e36477_d_n13, assign18820_e36477_d_n14,) = {
            if (!(assign18820_e36460 > 1e-38)) {
                let assign18820_e36465: f64 = (-87.498233534);
                (assign18820_e36465, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign18820_e36468: f64 = (var_nbody_i / var_ni);
                let (assign18820_e36476, assign18820_e36476_d_n0, assign18820_e36476_d_n2, assign18820_e36476_d_n3, assign18820_e36476_d_n4, assign18820_e36476_d_n5, assign18820_e36476_d_n6, assign18820_e36476_d_n7, assign18820_e36476_d_n8, assign18820_e36476_d_n9, assign18820_e36476_d_n10, assign18820_e36476_d_n11, assign18820_e36476_d_n13, assign18820_e36476_d_n14,) = {
                    if (assign18820_e36468 > 1e-38) {
                        let assign18820_e36473: f64 = (var_nbody_i / var_ni);
                        let assign18820_e36474: f64 = (assign18820_e36473).ln();
                        (assign18820_e36474, ((-((var_nbody_i * var_ni_dn0) / (var_ni * var_ni))) / assign18820_e36473), ((-((var_nbody_i * var_ni_dn2) / (var_ni * var_ni))) / assign18820_e36473), ((-((var_nbody_i * var_ni_dn3) / (var_ni * var_ni))) / assign18820_e36473), ((-((var_nbody_i * var_ni_dn4) / (var_ni * var_ni))) / assign18820_e36473), ((-((var_nbody_i * var_ni_dn5) / (var_ni * var_ni))) / assign18820_e36473), ((-((var_nbody_i * var_ni_dn6) / (var_ni * var_ni))) / assign18820_e36473), ((-((var_nbody_i * var_ni_dn7) / (var_ni * var_ni))) / assign18820_e36473), ((-((var_nbody_i * var_ni_dn8) / (var_ni * var_ni))) / assign18820_e36473), ((-((var_nbody_i * var_ni_dn9) / (var_ni * var_ni))) / assign18820_e36473), ((-((var_nbody_i * var_ni_dn10) / (var_ni * var_ni))) / assign18820_e36473), ((-((var_nbody_i * var_ni_dn11) / (var_ni * var_ni))) / assign18820_e36473), ((-((var_nbody_i * var_ni_dn13) / (var_ni * var_ni))) / assign18820_e36473), ((-((var_nbody_i * var_ni_dn14) / (var_ni * var_ni))) / assign18820_e36473),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18820_e36476, assign18820_e36476_d_n0, assign18820_e36476_d_n2, assign18820_e36476_d_n3, assign18820_e36476_d_n4, assign18820_e36476_d_n5, assign18820_e36476_d_n6, assign18820_e36476_d_n7, assign18820_e36476_d_n8, assign18820_e36476_d_n9, assign18820_e36476_d_n10, assign18820_e36476_d_n11, assign18820_e36476_d_n13, assign18820_e36476_d_n14,)
            }
        };
        let assign18820_e36478: f64 = (var_vtm * assign18820_e36477);
        (assign18820_e36478, (var_vtm * assign18820_e36477_d_n0), (var_vtm * assign18820_e36477_d_n2), (var_vtm * assign18820_e36477_d_n3), ((var_vtm_dn4 * assign18820_e36477) + (var_vtm * assign18820_e36477_d_n4)), (var_vtm * assign18820_e36477_d_n5), (var_vtm * assign18820_e36477_d_n6), (var_vtm * assign18820_e36477_d_n7), (var_vtm * assign18820_e36477_d_n8), (var_vtm * assign18820_e36477_d_n9), (var_vtm * assign18820_e36477_d_n10), (var_vtm * assign18820_e36477_d_n11), (var_vtm * assign18820_e36477_d_n13), (var_vtm * assign18820_e36477_d_n14),)
    } else {
        (var_phib, var_phib_dn0, var_phib_dn2, var_phib_dn3, var_phib_dn4, var_phib_dn5, var_phib_dn6, var_phib_dn7, var_phib_dn8, var_phib_dn9, var_phib_dn10, var_phib_dn11, var_phib_dn13, var_phib_dn14,)
    }
};
        var_phib = assign18820_e36480;
        var_phib_dn0 = assign18820_e36480_d_n0;
        var_phib_dn2 = assign18820_e36480_d_n2;
        var_phib_dn3 = assign18820_e36480_d_n3;
        var_phib_dn4 = assign18820_e36480_d_n4;
        var_phib_dn5 = assign18820_e36480_d_n5;
        var_phib_dn6 = assign18820_e36480_d_n6;
        var_phib_dn7 = assign18820_e36480_d_n7;
        var_phib_dn8 = assign18820_e36480_d_n8;
        var_phib_dn9 = assign18820_e36480_d_n9;
        var_phib_dn10 = assign18820_e36480_d_n10;
        var_phib_dn11 = assign18820_e36480_d_n11;
        var_phib_dn13 = assign18820_e36480_d_n13;
        var_phib_dn14 = assign18820_e36480_d_n14;
        var_phib_rv = 0.0;

        *var_guard347_slot = var_guard347;
        *var_guard347_rv_slot = var_guard347_rv;
        *var_guard348_slot = var_guard348;
        *var_guard348_rv_slot = var_guard348_rv;
        *var_phib_slot = var_phib;
        *var_phib_dn0_slot = var_phib_dn0;
        *var_phib_dn10_slot = var_phib_dn10;
        *var_phib_dn11_slot = var_phib_dn11;
        *var_phib_dn13_slot = var_phib_dn13;
        *var_phib_dn14_slot = var_phib_dn14;
        *var_phib_dn2_slot = var_phib_dn2;
        *var_phib_dn3_slot = var_phib_dn3;
        *var_phib_dn4_slot = var_phib_dn4;
        *var_phib_dn5_slot = var_phib_dn5;
        *var_phib_dn6_slot = var_phib_dn6;
        *var_phib_dn7_slot = var_phib_dn7;
        *var_phib_dn8_slot = var_phib_dn8;
        *var_phib_dn9_slot = var_phib_dn9;
        *var_phib_rv_slot = var_phib_rv;
        *var_vfbsd_v_slot = var_vfbsd_v;
        *var_vfbsd_v_dn0_slot = var_vfbsd_v_dn0;
        *var_vfbsd_v_dn10_slot = var_vfbsd_v_dn10;
        *var_vfbsd_v_dn11_slot = var_vfbsd_v_dn11;
        *var_vfbsd_v_dn13_slot = var_vfbsd_v_dn13;
        *var_vfbsd_v_dn14_slot = var_vfbsd_v_dn14;
        *var_vfbsd_v_dn2_slot = var_vfbsd_v_dn2;
        *var_vfbsd_v_dn3_slot = var_vfbsd_v_dn3;
        *var_vfbsd_v_dn4_slot = var_vfbsd_v_dn4;
        *var_vfbsd_v_dn5_slot = var_vfbsd_v_dn5;
        *var_vfbsd_v_dn6_slot = var_vfbsd_v_dn6;
        *var_vfbsd_v_dn7_slot = var_vfbsd_v_dn7;
        *var_vfbsd_v_dn8_slot = var_vfbsd_v_dn8;
        *var_vfbsd_v_dn9_slot = var_vfbsd_v_dn9;
        *var_vfbsd_v_rv_slot = var_vfbsd_v_rv;
        *var_vfbsdcv_v_slot = var_vfbsdcv_v;
        *var_vfbsdcv_v_dn0_slot = var_vfbsdcv_v_dn0;
        *var_vfbsdcv_v_dn10_slot = var_vfbsdcv_v_dn10;
        *var_vfbsdcv_v_dn11_slot = var_vfbsdcv_v_dn11;
        *var_vfbsdcv_v_dn13_slot = var_vfbsdcv_v_dn13;
        *var_vfbsdcv_v_dn14_slot = var_vfbsdcv_v_dn14;
        *var_vfbsdcv_v_dn2_slot = var_vfbsdcv_v_dn2;
        *var_vfbsdcv_v_dn3_slot = var_vfbsdcv_v_dn3;
        *var_vfbsdcv_v_dn4_slot = var_vfbsdcv_v_dn4;
        *var_vfbsdcv_v_dn5_slot = var_vfbsdcv_v_dn5;
        *var_vfbsdcv_v_dn6_slot = var_vfbsdcv_v_dn6;
        *var_vfbsdcv_v_dn7_slot = var_vfbsdcv_v_dn7;
        *var_vfbsdcv_v_dn8_slot = var_vfbsdcv_v_dn8;
        *var_vfbsdcv_v_dn9_slot = var_vfbsdcv_v_dn9;
        *var_vfbsdcv_v_rv_slot = var_vfbsdcv_v_rv;
    }

    pub(super) fn stamp_reactive_block_70(
        p: &Parameters,
        var_devsign: f64,
        var_eg: f64,
        var_eg_dn4: f64,
        var_etamob_t: f64,
        var_etamob_t_dn4: f64,
        var_guard348: f64,
        var_jss_t: f64,
        var_jss_t_dn0: f64,
        var_jss_t_dn10: f64,
        var_jss_t_dn11: f64,
        var_jss_t_dn13: f64,
        var_jss_t_dn14: f64,
        var_jss_t_dn2: f64,
        var_jss_t_dn3: f64,
        var_jss_t_dn4: f64,
        var_jss_t_dn5: f64,
        var_jss_t_dn6: f64,
        var_jss_t_dn7: f64,
        var_jss_t_dn8: f64,
        var_jss_t_dn9: f64,
        var_jswgs_t: f64,
        var_jswgs_t_dn0: f64,
        var_jswgs_t_dn10: f64,
        var_jswgs_t_dn11: f64,
        var_jswgs_t_dn13: f64,
        var_jswgs_t_dn14: f64,
        var_jswgs_t_dn2: f64,
        var_jswgs_t_dn3: f64,
        var_jswgs_t_dn4: f64,
        var_jswgs_t_dn5: f64,
        var_jswgs_t_dn6: f64,
        var_jswgs_t_dn7: f64,
        var_jswgs_t_dn8: f64,
        var_jswgs_t_dn9: f64,
        var_jsws_t: f64,
        var_jsws_t_dn0: f64,
        var_jsws_t_dn10: f64,
        var_jsws_t_dn11: f64,
        var_jsws_t_dn13: f64,
        var_jsws_t_dn14: f64,
        var_jsws_t_dn2: f64,
        var_jsws_t_dn3: f64,
        var_jsws_t_dn4: f64,
        var_jsws_t_dn5: f64,
        var_jsws_t_dn6: f64,
        var_jsws_t_dn7: f64,
        var_jsws_t_dn8: f64,
        var_jsws_t_dn9: f64,
        var_nbody_i: f64,
        var_nfintotal: f64,
        var_ni: f64,
        var_ni_dn0: f64,
        var_ni_dn10: f64,
        var_ni_dn11: f64,
        var_ni_dn13: f64,
        var_ni_dn14: f64,
        var_ni_dn2: f64,
        var_ni_dn3: f64,
        var_ni_dn4: f64,
        var_ni_dn5: f64,
        var_ni_dn6: f64,
        var_ni_dn7: f64,
        var_ni_dn8: f64,
        var_ni_dn9: f64,
        var_niln: f64,
        var_niln_dn0: f64,
        var_niln_dn10: f64,
        var_niln_dn11: f64,
        var_niln_dn13: f64,
        var_niln_dn14: f64,
        var_niln_dn2: f64,
        var_niln_dn3: f64,
        var_niln_dn4: f64,
        var_niln_dn5: f64,
        var_niln_dn6: f64,
        var_niln_dn7: f64,
        var_niln_dn8: f64,
        var_niln_dn9: f64,
        var_phig_i: f64,
        var_phig_i_dn0: f64,
        var_phig_i_dn10: f64,
        var_phig_i_dn11: f64,
        var_phig_i_dn13: f64,
        var_phig_i_dn14: f64,
        var_phig_i_dn2: f64,
        var_phig_i_dn3: f64,
        var_phig_i_dn4: f64,
        var_phig_i_dn5: f64,
        var_phig_i_dn6: f64,
        var_phig_i_dn7: f64,
        var_phig_i_dn8: f64,
        var_phig_i_dn9: f64,
        var_vtm: f64,
        var_vtm_dn4: f64,
        var_deltaphi_slot: &mut f64,
        var_deltaphi_dn0_slot: &mut f64,
        var_deltaphi_dn10_slot: &mut f64,
        var_deltaphi_dn11_slot: &mut f64,
        var_deltaphi_dn13_slot: &mut f64,
        var_deltaphi_dn14_slot: &mut f64,
        var_deltaphi_dn2_slot: &mut f64,
        var_deltaphi_dn3_slot: &mut f64,
        var_deltaphi_dn4_slot: &mut f64,
        var_deltaphi_dn5_slot: &mut f64,
        var_deltaphi_dn6_slot: &mut f64,
        var_deltaphi_dn7_slot: &mut f64,
        var_deltaphi_dn8_slot: &mut f64,
        var_deltaphi_dn9_slot: &mut f64,
        var_deltaphi_rv_slot: &mut f64,
        var_eta_mu_slot: &mut f64,
        var_eta_mu_cv_slot: &mut f64,
        var_eta_mu_cv_rv_slot: &mut f64,
        var_eta_mu_dn4_slot: &mut f64,
        var_eta_mu_rv_slot: &mut f64,
        var_guard349_slot: &mut f64,
        var_guard349_rv_slot: &mut f64,
        var_guard350_slot: &mut f64,
        var_guard350_rv_slot: &mut f64,
        var_guard351_slot: &mut f64,
        var_guard351_rv_slot: &mut f64,
        var_isbs_slot: &mut f64,
        var_isbs_dn0_slot: &mut f64,
        var_isbs_dn10_slot: &mut f64,
        var_isbs_dn11_slot: &mut f64,
        var_isbs_dn13_slot: &mut f64,
        var_isbs_dn14_slot: &mut f64,
        var_isbs_dn2_slot: &mut f64,
        var_isbs_dn3_slot: &mut f64,
        var_isbs_dn4_slot: &mut f64,
        var_isbs_dn5_slot: &mut f64,
        var_isbs_dn6_slot: &mut f64,
        var_isbs_dn7_slot: &mut f64,
        var_isbs_dn8_slot: &mut f64,
        var_isbs_dn9_slot: &mut f64,
        var_isbs_rv_slot: &mut f64,
        var_nvtms_slot: &mut f64,
        var_nvtms_dn4_slot: &mut f64,
        var_nvtms_rv_slot: &mut f64,
        var_phib_slot: &mut f64,
        var_phib_dn0_slot: &mut f64,
        var_phib_dn10_slot: &mut f64,
        var_phib_dn11_slot: &mut f64,
        var_phib_dn13_slot: &mut f64,
        var_phib_dn14_slot: &mut f64,
        var_phib_dn2_slot: &mut f64,
        var_phib_dn3_slot: &mut f64,
        var_phib_dn4_slot: &mut f64,
        var_phib_dn5_slot: &mut f64,
        var_phib_dn6_slot: &mut f64,
        var_phib_dn7_slot: &mut f64,
        var_phib_dn8_slot: &mut f64,
        var_phib_dn9_slot: &mut f64,
        var_phib_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn13_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_tb_slot: &mut f64,
        var_tb_dn0_slot: &mut f64,
        var_tb_dn10_slot: &mut f64,
        var_tb_dn11_slot: &mut f64,
        var_tb_dn13_slot: &mut f64,
        var_tb_dn14_slot: &mut f64,
        var_tb_dn2_slot: &mut f64,
        var_tb_dn3_slot: &mut f64,
        var_tb_dn4_slot: &mut f64,
        var_tb_dn5_slot: &mut f64,
        var_tb_dn6_slot: &mut f64,
        var_tb_dn7_slot: &mut f64,
        var_tb_dn8_slot: &mut f64,
        var_tb_dn9_slot: &mut f64,
        var_tb_rv_slot: &mut f64,
        var_vbi_slot: &mut f64,
        var_vbi_dn0_slot: &mut f64,
        var_vbi_dn10_slot: &mut f64,
        var_vbi_dn11_slot: &mut f64,
        var_vbi_dn13_slot: &mut f64,
        var_vbi_dn14_slot: &mut f64,
        var_vbi_dn2_slot: &mut f64,
        var_vbi_dn3_slot: &mut f64,
        var_vbi_dn4_slot: &mut f64,
        var_vbi_dn5_slot: &mut f64,
        var_vbi_dn6_slot: &mut f64,
        var_vbi_dn7_slot: &mut f64,
        var_vbi_dn8_slot: &mut f64,
        var_vbi_dn9_slot: &mut f64,
        var_vbi_rv_slot: &mut f64,
        var_vjsmfwd_slot: &mut f64,
        var_vjsmfwd_dn0_slot: &mut f64,
        var_vjsmfwd_dn10_slot: &mut f64,
        var_vjsmfwd_dn11_slot: &mut f64,
        var_vjsmfwd_dn13_slot: &mut f64,
        var_vjsmfwd_dn14_slot: &mut f64,
        var_vjsmfwd_dn2_slot: &mut f64,
        var_vjsmfwd_dn3_slot: &mut f64,
        var_vjsmfwd_dn4_slot: &mut f64,
        var_vjsmfwd_dn5_slot: &mut f64,
        var_vjsmfwd_dn6_slot: &mut f64,
        var_vjsmfwd_dn7_slot: &mut f64,
        var_vjsmfwd_dn8_slot: &mut f64,
        var_vjsmfwd_dn9_slot: &mut f64,
        var_vjsmfwd_rv_slot: &mut f64,
        var_xexpbvs_slot: &mut f64,
        var_xexpbvs_dn4_slot: &mut f64,
        var_xexpbvs_rv_slot: &mut f64,
    ) {
        let mut var_deltaphi: f64 = *var_deltaphi_slot;
        let mut var_deltaphi_dn0: f64 = *var_deltaphi_dn0_slot;
        let mut var_deltaphi_dn10: f64 = *var_deltaphi_dn10_slot;
        let mut var_deltaphi_dn11: f64 = *var_deltaphi_dn11_slot;
        let mut var_deltaphi_dn13: f64 = *var_deltaphi_dn13_slot;
        let mut var_deltaphi_dn14: f64 = *var_deltaphi_dn14_slot;
        let mut var_deltaphi_dn2: f64 = *var_deltaphi_dn2_slot;
        let mut var_deltaphi_dn3: f64 = *var_deltaphi_dn3_slot;
        let mut var_deltaphi_dn4: f64 = *var_deltaphi_dn4_slot;
        let mut var_deltaphi_dn5: f64 = *var_deltaphi_dn5_slot;
        let mut var_deltaphi_dn6: f64 = *var_deltaphi_dn6_slot;
        let mut var_deltaphi_dn7: f64 = *var_deltaphi_dn7_slot;
        let mut var_deltaphi_dn8: f64 = *var_deltaphi_dn8_slot;
        let mut var_deltaphi_dn9: f64 = *var_deltaphi_dn9_slot;
        let mut var_deltaphi_rv: f64 = *var_deltaphi_rv_slot;
        let mut var_eta_mu: f64 = *var_eta_mu_slot;
        let mut var_eta_mu_cv: f64 = *var_eta_mu_cv_slot;
        let mut var_eta_mu_cv_rv: f64 = *var_eta_mu_cv_rv_slot;
        let mut var_eta_mu_dn4: f64 = *var_eta_mu_dn4_slot;
        let mut var_eta_mu_rv: f64 = *var_eta_mu_rv_slot;
        let mut var_guard349: f64 = *var_guard349_slot;
        let mut var_guard349_rv: f64 = *var_guard349_rv_slot;
        let mut var_guard350: f64 = *var_guard350_slot;
        let mut var_guard350_rv: f64 = *var_guard350_rv_slot;
        let mut var_guard351: f64 = *var_guard351_slot;
        let mut var_guard351_rv: f64 = *var_guard351_rv_slot;
        let mut var_isbs: f64 = *var_isbs_slot;
        let mut var_isbs_dn0: f64 = *var_isbs_dn0_slot;
        let mut var_isbs_dn10: f64 = *var_isbs_dn10_slot;
        let mut var_isbs_dn11: f64 = *var_isbs_dn11_slot;
        let mut var_isbs_dn13: f64 = *var_isbs_dn13_slot;
        let mut var_isbs_dn14: f64 = *var_isbs_dn14_slot;
        let mut var_isbs_dn2: f64 = *var_isbs_dn2_slot;
        let mut var_isbs_dn3: f64 = *var_isbs_dn3_slot;
        let mut var_isbs_dn4: f64 = *var_isbs_dn4_slot;
        let mut var_isbs_dn5: f64 = *var_isbs_dn5_slot;
        let mut var_isbs_dn6: f64 = *var_isbs_dn6_slot;
        let mut var_isbs_dn7: f64 = *var_isbs_dn7_slot;
        let mut var_isbs_dn8: f64 = *var_isbs_dn8_slot;
        let mut var_isbs_dn9: f64 = *var_isbs_dn9_slot;
        let mut var_isbs_rv: f64 = *var_isbs_rv_slot;
        let mut var_nvtms: f64 = *var_nvtms_slot;
        let mut var_nvtms_dn4: f64 = *var_nvtms_dn4_slot;
        let mut var_nvtms_rv: f64 = *var_nvtms_rv_slot;
        let mut var_phib: f64 = *var_phib_slot;
        let mut var_phib_dn0: f64 = *var_phib_dn0_slot;
        let mut var_phib_dn10: f64 = *var_phib_dn10_slot;
        let mut var_phib_dn11: f64 = *var_phib_dn11_slot;
        let mut var_phib_dn13: f64 = *var_phib_dn13_slot;
        let mut var_phib_dn14: f64 = *var_phib_dn14_slot;
        let mut var_phib_dn2: f64 = *var_phib_dn2_slot;
        let mut var_phib_dn3: f64 = *var_phib_dn3_slot;
        let mut var_phib_dn4: f64 = *var_phib_dn4_slot;
        let mut var_phib_dn5: f64 = *var_phib_dn5_slot;
        let mut var_phib_dn6: f64 = *var_phib_dn6_slot;
        let mut var_phib_dn7: f64 = *var_phib_dn7_slot;
        let mut var_phib_dn8: f64 = *var_phib_dn8_slot;
        let mut var_phib_dn9: f64 = *var_phib_dn9_slot;
        let mut var_phib_rv: f64 = *var_phib_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_tb: f64 = *var_tb_slot;
        let mut var_tb_dn0: f64 = *var_tb_dn0_slot;
        let mut var_tb_dn10: f64 = *var_tb_dn10_slot;
        let mut var_tb_dn11: f64 = *var_tb_dn11_slot;
        let mut var_tb_dn13: f64 = *var_tb_dn13_slot;
        let mut var_tb_dn14: f64 = *var_tb_dn14_slot;
        let mut var_tb_dn2: f64 = *var_tb_dn2_slot;
        let mut var_tb_dn3: f64 = *var_tb_dn3_slot;
        let mut var_tb_dn4: f64 = *var_tb_dn4_slot;
        let mut var_tb_dn5: f64 = *var_tb_dn5_slot;
        let mut var_tb_dn6: f64 = *var_tb_dn6_slot;
        let mut var_tb_dn7: f64 = *var_tb_dn7_slot;
        let mut var_tb_dn8: f64 = *var_tb_dn8_slot;
        let mut var_tb_dn9: f64 = *var_tb_dn9_slot;
        let mut var_tb_rv: f64 = *var_tb_rv_slot;
        let mut var_vbi: f64 = *var_vbi_slot;
        let mut var_vbi_dn0: f64 = *var_vbi_dn0_slot;
        let mut var_vbi_dn10: f64 = *var_vbi_dn10_slot;
        let mut var_vbi_dn11: f64 = *var_vbi_dn11_slot;
        let mut var_vbi_dn13: f64 = *var_vbi_dn13_slot;
        let mut var_vbi_dn14: f64 = *var_vbi_dn14_slot;
        let mut var_vbi_dn2: f64 = *var_vbi_dn2_slot;
        let mut var_vbi_dn3: f64 = *var_vbi_dn3_slot;
        let mut var_vbi_dn4: f64 = *var_vbi_dn4_slot;
        let mut var_vbi_dn5: f64 = *var_vbi_dn5_slot;
        let mut var_vbi_dn6: f64 = *var_vbi_dn6_slot;
        let mut var_vbi_dn7: f64 = *var_vbi_dn7_slot;
        let mut var_vbi_dn8: f64 = *var_vbi_dn8_slot;
        let mut var_vbi_dn9: f64 = *var_vbi_dn9_slot;
        let mut var_vbi_rv: f64 = *var_vbi_rv_slot;
        let mut var_vjsmfwd: f64 = *var_vjsmfwd_slot;
        let mut var_vjsmfwd_dn0: f64 = *var_vjsmfwd_dn0_slot;
        let mut var_vjsmfwd_dn10: f64 = *var_vjsmfwd_dn10_slot;
        let mut var_vjsmfwd_dn11: f64 = *var_vjsmfwd_dn11_slot;
        let mut var_vjsmfwd_dn13: f64 = *var_vjsmfwd_dn13_slot;
        let mut var_vjsmfwd_dn14: f64 = *var_vjsmfwd_dn14_slot;
        let mut var_vjsmfwd_dn2: f64 = *var_vjsmfwd_dn2_slot;
        let mut var_vjsmfwd_dn3: f64 = *var_vjsmfwd_dn3_slot;
        let mut var_vjsmfwd_dn4: f64 = *var_vjsmfwd_dn4_slot;
        let mut var_vjsmfwd_dn5: f64 = *var_vjsmfwd_dn5_slot;
        let mut var_vjsmfwd_dn6: f64 = *var_vjsmfwd_dn6_slot;
        let mut var_vjsmfwd_dn7: f64 = *var_vjsmfwd_dn7_slot;
        let mut var_vjsmfwd_dn8: f64 = *var_vjsmfwd_dn8_slot;
        let mut var_vjsmfwd_dn9: f64 = *var_vjsmfwd_dn9_slot;
        let mut var_vjsmfwd_rv: f64 = *var_vjsmfwd_rv_slot;
        let mut var_xexpbvs: f64 = *var_xexpbvs_slot;
        let mut var_xexpbvs_dn4: f64 = *var_xexpbvs_dn4_slot;
        let mut var_xexpbvs_rv: f64 = *var_xexpbvs_rv_slot;

        let (assign18830_e36503, assign18830_e36503_d_n0, assign18830_e36503_d_n2, assign18830_e36503_d_n3, assign18830_e36503_d_n4, assign18830_e36503_d_n5, assign18830_e36503_d_n6, assign18830_e36503_d_n7, assign18830_e36503_d_n8, assign18830_e36503_d_n9, assign18830_e36503_d_n10, assign18830_e36503_d_n11, assign18830_e36503_d_n13, assign18830_e36503_d_n14,) = {
    if (var_guard348 != 0.0) {
        let assign18830_e36485: f64 = var_phib;
        let assign18830_e36488: f64 = var_phib;
        let assign18830_e36491: f64 = var_phib;
        let assign18830_e36492: f64 = (assign18830_e36488 * assign18830_e36491);
        let assign18830_e36495: f64 = (0.25 * 1e-10);
        let assign18830_e36497: f64 = (assign18830_e36495 * 1e-10);
        let assign18830_e36498: f64 = (assign18830_e36492 + assign18830_e36497);
        let assign18830_e36499: f64 = (assign18830_e36498).sqrt();
        let assign18830_e36500: f64 = (assign18830_e36485 + assign18830_e36499);
        let assign18830_e36501: f64 = (0.5 * assign18830_e36500);
        (assign18830_e36501, (0.5 * (var_phib_dn0 + (((var_phib_dn0 * assign18830_e36491) + (assign18830_e36488 * var_phib_dn0)) / (2.0 * assign18830_e36499)))), (0.5 * (var_phib_dn2 + (((var_phib_dn2 * assign18830_e36491) + (assign18830_e36488 * var_phib_dn2)) / (2.0 * assign18830_e36499)))), (0.5 * (var_phib_dn3 + (((var_phib_dn3 * assign18830_e36491) + (assign18830_e36488 * var_phib_dn3)) / (2.0 * assign18830_e36499)))), (0.5 * (var_phib_dn4 + (((var_phib_dn4 * assign18830_e36491) + (assign18830_e36488 * var_phib_dn4)) / (2.0 * assign18830_e36499)))), (0.5 * (var_phib_dn5 + (((var_phib_dn5 * assign18830_e36491) + (assign18830_e36488 * var_phib_dn5)) / (2.0 * assign18830_e36499)))), (0.5 * (var_phib_dn6 + (((var_phib_dn6 * assign18830_e36491) + (assign18830_e36488 * var_phib_dn6)) / (2.0 * assign18830_e36499)))), (0.5 * (var_phib_dn7 + (((var_phib_dn7 * assign18830_e36491) + (assign18830_e36488 * var_phib_dn7)) / (2.0 * assign18830_e36499)))), (0.5 * (var_phib_dn8 + (((var_phib_dn8 * assign18830_e36491) + (assign18830_e36488 * var_phib_dn8)) / (2.0 * assign18830_e36499)))), (0.5 * (var_phib_dn9 + (((var_phib_dn9 * assign18830_e36491) + (assign18830_e36488 * var_phib_dn9)) / (2.0 * assign18830_e36499)))), (0.5 * (var_phib_dn10 + (((var_phib_dn10 * assign18830_e36491) + (assign18830_e36488 * var_phib_dn10)) / (2.0 * assign18830_e36499)))), (0.5 * (var_phib_dn11 + (((var_phib_dn11 * assign18830_e36491) + (assign18830_e36488 * var_phib_dn11)) / (2.0 * assign18830_e36499)))), (0.5 * (var_phib_dn13 + (((var_phib_dn13 * assign18830_e36491) + (assign18830_e36488 * var_phib_dn13)) / (2.0 * assign18830_e36499)))), (0.5 * (var_phib_dn14 + (((var_phib_dn14 * assign18830_e36491) + (assign18830_e36488 * var_phib_dn14)) / (2.0 * assign18830_e36499)))),)
    } else {
        (var_phib, var_phib_dn0, var_phib_dn2, var_phib_dn3, var_phib_dn4, var_phib_dn5, var_phib_dn6, var_phib_dn7, var_phib_dn8, var_phib_dn9, var_phib_dn10, var_phib_dn11, var_phib_dn13, var_phib_dn14,)
    }
};
        var_phib = assign18830_e36503;
        var_phib_dn0 = assign18830_e36503_d_n0;
        var_phib_dn2 = assign18830_e36503_d_n2;
        var_phib_dn3 = assign18830_e36503_d_n3;
        var_phib_dn4 = assign18830_e36503_d_n4;
        var_phib_dn5 = assign18830_e36503_d_n5;
        var_phib_dn6 = assign18830_e36503_d_n6;
        var_phib_dn7 = assign18830_e36503_d_n7;
        var_phib_dn8 = assign18830_e36503_d_n8;
        var_phib_dn9 = assign18830_e36503_d_n9;
        var_phib_dn10 = assign18830_e36503_d_n10;
        var_phib_dn11 = assign18830_e36503_d_n11;
        var_phib_dn13 = assign18830_e36503_d_n13;
        var_phib_dn14 = assign18830_e36503_d_n14;
        var_phib_rv = 0.0;

        let (assign18840_e36540, assign18840_e36540_d_n0, assign18840_e36540_d_n2, assign18840_e36540_d_n3, assign18840_e36540_d_n4, assign18840_e36540_d_n5, assign18840_e36540_d_n6, assign18840_e36540_d_n7, assign18840_e36540_d_n8, assign18840_e36540_d_n9, assign18840_e36540_d_n10, assign18840_e36540_d_n11, assign18840_e36540_d_n13, assign18840_e36540_d_n14,) = {
    if (var_guard348 != 0.0) {
        let assign18840_e36508: f64 = (var_nbody_i * p.p97);
        let assign18840_e36511: f64 = (var_ni * var_ni);
        let assign18840_e36512: f64 = (assign18840_e36508 / assign18840_e36511);
        let (assign18840_e36537, assign18840_e36537_d_n0, assign18840_e36537_d_n2, assign18840_e36537_d_n3, assign18840_e36537_d_n4, assign18840_e36537_d_n5, assign18840_e36537_d_n6, assign18840_e36537_d_n7, assign18840_e36537_d_n8, assign18840_e36537_d_n9, assign18840_e36537_d_n10, assign18840_e36537_d_n11, assign18840_e36537_d_n13, assign18840_e36537_d_n14,) = {
            if (!(assign18840_e36512 > 1e-38)) {
                let assign18840_e36517: f64 = (-87.498233534);
                (assign18840_e36517, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign18840_e36520: f64 = (var_nbody_i * p.p97);
                let assign18840_e36523: f64 = (var_ni * var_ni);
                let assign18840_e36524: f64 = (assign18840_e36520 / assign18840_e36523);
                let (assign18840_e36536, assign18840_e36536_d_n0, assign18840_e36536_d_n2, assign18840_e36536_d_n3, assign18840_e36536_d_n4, assign18840_e36536_d_n5, assign18840_e36536_d_n6, assign18840_e36536_d_n7, assign18840_e36536_d_n8, assign18840_e36536_d_n9, assign18840_e36536_d_n10, assign18840_e36536_d_n11, assign18840_e36536_d_n13, assign18840_e36536_d_n14,) = {
                    if (assign18840_e36524 > 1e-38) {
                        let assign18840_e36529: f64 = (var_nbody_i * p.p97);
                        let assign18840_e36532: f64 = (var_ni * var_ni);
                        let assign18840_e36533: f64 = (assign18840_e36529 / assign18840_e36532);
                        let assign18840_e36534: f64 = (assign18840_e36533).ln();
                        (assign18840_e36534, ((-((assign18840_e36529 * ((var_ni_dn0 * var_ni) + (var_ni * var_ni_dn0))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((var_ni_dn2 * var_ni) + (var_ni * var_ni_dn2))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((var_ni_dn3 * var_ni) + (var_ni * var_ni_dn3))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((var_ni_dn4 * var_ni) + (var_ni * var_ni_dn4))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((var_ni_dn5 * var_ni) + (var_ni * var_ni_dn5))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((var_ni_dn6 * var_ni) + (var_ni * var_ni_dn6))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((var_ni_dn7 * var_ni) + (var_ni * var_ni_dn7))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((var_ni_dn8 * var_ni) + (var_ni * var_ni_dn8))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((var_ni_dn9 * var_ni) + (var_ni * var_ni_dn9))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((var_ni_dn10 * var_ni) + (var_ni * var_ni_dn10))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((var_ni_dn11 * var_ni) + (var_ni * var_ni_dn11))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((var_ni_dn13 * var_ni) + (var_ni * var_ni_dn13))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((var_ni_dn14 * var_ni) + (var_ni * var_ni_dn14))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18840_e36536, assign18840_e36536_d_n0, assign18840_e36536_d_n2, assign18840_e36536_d_n3, assign18840_e36536_d_n4, assign18840_e36536_d_n5, assign18840_e36536_d_n6, assign18840_e36536_d_n7, assign18840_e36536_d_n8, assign18840_e36536_d_n9, assign18840_e36536_d_n10, assign18840_e36536_d_n11, assign18840_e36536_d_n13, assign18840_e36536_d_n14,)
            }
        };
        let assign18840_e36538: f64 = (var_vtm * assign18840_e36537);
        (assign18840_e36538, (var_vtm * assign18840_e36537_d_n0), (var_vtm * assign18840_e36537_d_n2), (var_vtm * assign18840_e36537_d_n3), ((var_vtm_dn4 * assign18840_e36537) + (var_vtm * assign18840_e36537_d_n4)), (var_vtm * assign18840_e36537_d_n5), (var_vtm * assign18840_e36537_d_n6), (var_vtm * assign18840_e36537_d_n7), (var_vtm * assign18840_e36537_d_n8), (var_vtm * assign18840_e36537_d_n9), (var_vtm * assign18840_e36537_d_n10), (var_vtm * assign18840_e36537_d_n11), (var_vtm * assign18840_e36537_d_n13), (var_vtm * assign18840_e36537_d_n14),)
    } else {
        (var_vbi, var_vbi_dn0, var_vbi_dn2, var_vbi_dn3, var_vbi_dn4, var_vbi_dn5, var_vbi_dn6, var_vbi_dn7, var_vbi_dn8, var_vbi_dn9, var_vbi_dn10, var_vbi_dn11, var_vbi_dn13, var_vbi_dn14,)
    }
};
        var_vbi = assign18840_e36540;
        var_vbi_dn0 = assign18840_e36540_d_n0;
        var_vbi_dn2 = assign18840_e36540_d_n2;
        var_vbi_dn3 = assign18840_e36540_d_n3;
        var_vbi_dn4 = assign18840_e36540_d_n4;
        var_vbi_dn5 = assign18840_e36540_d_n5;
        var_vbi_dn6 = assign18840_e36540_d_n6;
        var_vbi_dn7 = assign18840_e36540_d_n7;
        var_vbi_dn8 = assign18840_e36540_d_n8;
        var_vbi_dn9 = assign18840_e36540_d_n9;
        var_vbi_dn10 = assign18840_e36540_d_n10;
        var_vbi_dn11 = assign18840_e36540_d_n11;
        var_vbi_dn13 = assign18840_e36540_d_n13;
        var_vbi_dn14 = assign18840_e36540_d_n14;
        var_vbi_rv = 0.0;

        let (assign18850_e36562, assign18850_e36562_d_n0, assign18850_e36562_d_n2, assign18850_e36562_d_n3, assign18850_e36562_d_n4, assign18850_e36562_d_n5, assign18850_e36562_d_n6, assign18850_e36562_d_n7, assign18850_e36562_d_n8, assign18850_e36562_d_n9, assign18850_e36562_d_n10, assign18850_e36562_d_n11, assign18850_e36562_d_n13, assign18850_e36562_d_n14,) = {
    if (var_guard348 == 0.0) {
        let (assign18850_e36557,) = {
            if (!(var_nbody_i > 1e-38)) {
                let assign18850_e36549: f64 = (-87.498233534);
                (assign18850_e36549,)
            } else {
                let (assign18850_e36556,) = {
                    if (var_nbody_i > 1e-38) {
                        let assign18850_e36554: f64 = (var_nbody_i).ln();
                        (assign18850_e36554,)
                    } else {
                        (0.0,)
                    }
                };
                (assign18850_e36556,)
            }
        };
        let assign18850_e36559: f64 = (assign18850_e36557 - var_niln);
        let assign18850_e36560: f64 = (var_vtm * assign18850_e36559);
        (assign18850_e36560, (var_vtm * (-var_niln_dn0)), (var_vtm * (-var_niln_dn2)), (var_vtm * (-var_niln_dn3)), ((var_vtm_dn4 * assign18850_e36559) + (var_vtm * (-var_niln_dn4))), (var_vtm * (-var_niln_dn5)), (var_vtm * (-var_niln_dn6)), (var_vtm * (-var_niln_dn7)), (var_vtm * (-var_niln_dn8)), (var_vtm * (-var_niln_dn9)), (var_vtm * (-var_niln_dn10)), (var_vtm * (-var_niln_dn11)), (var_vtm * (-var_niln_dn13)), (var_vtm * (-var_niln_dn14)),)
    } else {
        (var_phib, var_phib_dn0, var_phib_dn2, var_phib_dn3, var_phib_dn4, var_phib_dn5, var_phib_dn6, var_phib_dn7, var_phib_dn8, var_phib_dn9, var_phib_dn10, var_phib_dn11, var_phib_dn13, var_phib_dn14,)
    }
};
        var_phib = assign18850_e36562;
        var_phib_dn0 = assign18850_e36562_d_n0;
        var_phib_dn2 = assign18850_e36562_d_n2;
        var_phib_dn3 = assign18850_e36562_d_n3;
        var_phib_dn4 = assign18850_e36562_d_n4;
        var_phib_dn5 = assign18850_e36562_d_n5;
        var_phib_dn6 = assign18850_e36562_d_n6;
        var_phib_dn7 = assign18850_e36562_d_n7;
        var_phib_dn8 = assign18850_e36562_d_n8;
        var_phib_dn9 = assign18850_e36562_d_n9;
        var_phib_dn10 = assign18850_e36562_d_n10;
        var_phib_dn11 = assign18850_e36562_d_n11;
        var_phib_dn13 = assign18850_e36562_d_n13;
        var_phib_dn14 = assign18850_e36562_d_n14;
        var_phib_rv = 0.0;

        let (assign18860_e36586, assign18860_e36586_d_n0, assign18860_e36586_d_n2, assign18860_e36586_d_n3, assign18860_e36586_d_n4, assign18860_e36586_d_n5, assign18860_e36586_d_n6, assign18860_e36586_d_n7, assign18860_e36586_d_n8, assign18860_e36586_d_n9, assign18860_e36586_d_n10, assign18860_e36586_d_n11, assign18860_e36586_d_n13, assign18860_e36586_d_n14,) = {
    if (var_guard348 == 0.0) {
        let assign18860_e36568: f64 = var_phib;
        let assign18860_e36571: f64 = var_phib;
        let assign18860_e36574: f64 = var_phib;
        let assign18860_e36575: f64 = (assign18860_e36571 * assign18860_e36574);
        let assign18860_e36578: f64 = (0.25 * 1e-10);
        let assign18860_e36580: f64 = (assign18860_e36578 * 1e-10);
        let assign18860_e36581: f64 = (assign18860_e36575 + assign18860_e36580);
        let assign18860_e36582: f64 = (assign18860_e36581).sqrt();
        let assign18860_e36583: f64 = (assign18860_e36568 + assign18860_e36582);
        let assign18860_e36584: f64 = (0.5 * assign18860_e36583);
        (assign18860_e36584, (0.5 * (var_phib_dn0 + (((var_phib_dn0 * assign18860_e36574) + (assign18860_e36571 * var_phib_dn0)) / (2.0 * assign18860_e36582)))), (0.5 * (var_phib_dn2 + (((var_phib_dn2 * assign18860_e36574) + (assign18860_e36571 * var_phib_dn2)) / (2.0 * assign18860_e36582)))), (0.5 * (var_phib_dn3 + (((var_phib_dn3 * assign18860_e36574) + (assign18860_e36571 * var_phib_dn3)) / (2.0 * assign18860_e36582)))), (0.5 * (var_phib_dn4 + (((var_phib_dn4 * assign18860_e36574) + (assign18860_e36571 * var_phib_dn4)) / (2.0 * assign18860_e36582)))), (0.5 * (var_phib_dn5 + (((var_phib_dn5 * assign18860_e36574) + (assign18860_e36571 * var_phib_dn5)) / (2.0 * assign18860_e36582)))), (0.5 * (var_phib_dn6 + (((var_phib_dn6 * assign18860_e36574) + (assign18860_e36571 * var_phib_dn6)) / (2.0 * assign18860_e36582)))), (0.5 * (var_phib_dn7 + (((var_phib_dn7 * assign18860_e36574) + (assign18860_e36571 * var_phib_dn7)) / (2.0 * assign18860_e36582)))), (0.5 * (var_phib_dn8 + (((var_phib_dn8 * assign18860_e36574) + (assign18860_e36571 * var_phib_dn8)) / (2.0 * assign18860_e36582)))), (0.5 * (var_phib_dn9 + (((var_phib_dn9 * assign18860_e36574) + (assign18860_e36571 * var_phib_dn9)) / (2.0 * assign18860_e36582)))), (0.5 * (var_phib_dn10 + (((var_phib_dn10 * assign18860_e36574) + (assign18860_e36571 * var_phib_dn10)) / (2.0 * assign18860_e36582)))), (0.5 * (var_phib_dn11 + (((var_phib_dn11 * assign18860_e36574) + (assign18860_e36571 * var_phib_dn11)) / (2.0 * assign18860_e36582)))), (0.5 * (var_phib_dn13 + (((var_phib_dn13 * assign18860_e36574) + (assign18860_e36571 * var_phib_dn13)) / (2.0 * assign18860_e36582)))), (0.5 * (var_phib_dn14 + (((var_phib_dn14 * assign18860_e36574) + (assign18860_e36571 * var_phib_dn14)) / (2.0 * assign18860_e36582)))),)
    } else {
        (var_phib, var_phib_dn0, var_phib_dn2, var_phib_dn3, var_phib_dn4, var_phib_dn5, var_phib_dn6, var_phib_dn7, var_phib_dn8, var_phib_dn9, var_phib_dn10, var_phib_dn11, var_phib_dn13, var_phib_dn14,)
    }
};
        var_phib = assign18860_e36586;
        var_phib_dn0 = assign18860_e36586_d_n0;
        var_phib_dn2 = assign18860_e36586_d_n2;
        var_phib_dn3 = assign18860_e36586_d_n3;
        var_phib_dn4 = assign18860_e36586_d_n4;
        var_phib_dn5 = assign18860_e36586_d_n5;
        var_phib_dn6 = assign18860_e36586_d_n6;
        var_phib_dn7 = assign18860_e36586_d_n7;
        var_phib_dn8 = assign18860_e36586_d_n8;
        var_phib_dn9 = assign18860_e36586_d_n9;
        var_phib_dn10 = assign18860_e36586_d_n10;
        var_phib_dn11 = assign18860_e36586_d_n11;
        var_phib_dn13 = assign18860_e36586_d_n13;
        var_phib_dn14 = assign18860_e36586_d_n14;
        var_phib_rv = 0.0;

        let (assign18870_e36616, assign18870_e36616_d_n0, assign18870_e36616_d_n2, assign18870_e36616_d_n3, assign18870_e36616_d_n4, assign18870_e36616_d_n5, assign18870_e36616_d_n6, assign18870_e36616_d_n7, assign18870_e36616_d_n8, assign18870_e36616_d_n9, assign18870_e36616_d_n10, assign18870_e36616_d_n11, assign18870_e36616_d_n13, assign18870_e36616_d_n14,) = {
    if (var_guard348 == 0.0) {
        let assign18870_e36592: f64 = (var_nbody_i * p.p97);
        let (assign18870_e36609,) = {
            if (!(assign18870_e36592 > 1e-38)) {
                let assign18870_e36597: f64 = (-87.498233534);
                (assign18870_e36597,)
            } else {
                let assign18870_e36600: f64 = (var_nbody_i * p.p97);
                let (assign18870_e36608,) = {
                    if (assign18870_e36600 > 1e-38) {
                        let assign18870_e36605: f64 = (var_nbody_i * p.p97);
                        let assign18870_e36606: f64 = (assign18870_e36605).ln();
                        (assign18870_e36606,)
                    } else {
                        (0.0,)
                    }
                };
                (assign18870_e36608,)
            }
        };
        let assign18870_e36612: f64 = (2.0 * var_niln);
        let assign18870_e36613: f64 = (assign18870_e36609 - assign18870_e36612);
        let assign18870_e36614: f64 = (var_vtm * assign18870_e36613);
        (assign18870_e36614, (var_vtm * (-(2.0 * var_niln_dn0))), (var_vtm * (-(2.0 * var_niln_dn2))), (var_vtm * (-(2.0 * var_niln_dn3))), ((var_vtm_dn4 * assign18870_e36613) + (var_vtm * (-(2.0 * var_niln_dn4)))), (var_vtm * (-(2.0 * var_niln_dn5))), (var_vtm * (-(2.0 * var_niln_dn6))), (var_vtm * (-(2.0 * var_niln_dn7))), (var_vtm * (-(2.0 * var_niln_dn8))), (var_vtm * (-(2.0 * var_niln_dn9))), (var_vtm * (-(2.0 * var_niln_dn10))), (var_vtm * (-(2.0 * var_niln_dn11))), (var_vtm * (-(2.0 * var_niln_dn13))), (var_vtm * (-(2.0 * var_niln_dn14))),)
    } else {
        (var_vbi, var_vbi_dn0, var_vbi_dn2, var_vbi_dn3, var_vbi_dn4, var_vbi_dn5, var_vbi_dn6, var_vbi_dn7, var_vbi_dn8, var_vbi_dn9, var_vbi_dn10, var_vbi_dn11, var_vbi_dn13, var_vbi_dn14,)
    }
};
        var_vbi = assign18870_e36616;
        var_vbi_dn0 = assign18870_e36616_d_n0;
        var_vbi_dn2 = assign18870_e36616_d_n2;
        var_vbi_dn3 = assign18870_e36616_d_n3;
        var_vbi_dn4 = assign18870_e36616_d_n4;
        var_vbi_dn5 = assign18870_e36616_d_n5;
        var_vbi_dn6 = assign18870_e36616_d_n6;
        var_vbi_dn7 = assign18870_e36616_d_n7;
        var_vbi_dn8 = assign18870_e36616_d_n8;
        var_vbi_dn9 = assign18870_e36616_d_n9;
        var_vbi_dn10 = assign18870_e36616_d_n10;
        var_vbi_dn11 = assign18870_e36616_d_n11;
        var_vbi_dn13 = assign18870_e36616_d_n13;
        var_vbi_dn14 = assign18870_e36616_d_n14;
        var_vbi_rv = 0.0;

        let (assign18880_e36625, assign18880_e36625_d_n4,) = {
    if (p.p60 == 1.0) {
        (0.0, 0.0,)
    } else {
        (var_eg, var_eg_dn4,)
    }
};
        let assign18880_e36626: f64 = (p.p104 + assign18880_e36625);
        let assign18880_e36627: f64 = (var_phig_i - assign18880_e36626);
        let assign18880_e36628: f64 = (var_devsign * assign18880_e36627);
        var_deltaphi = assign18880_e36628;
        var_deltaphi_dn0 = (var_devsign * var_phig_i_dn0);
        var_deltaphi_dn2 = (var_devsign * var_phig_i_dn2);
        var_deltaphi_dn3 = (var_devsign * var_phig_i_dn3);
        var_deltaphi_dn4 = (var_devsign * (var_phig_i_dn4 - assign18880_e36625_d_n4));
        var_deltaphi_dn5 = (var_devsign * var_phig_i_dn5);
        var_deltaphi_dn6 = (var_devsign * var_phig_i_dn6);
        var_deltaphi_dn7 = (var_devsign * var_phig_i_dn7);
        var_deltaphi_dn8 = (var_devsign * var_phig_i_dn8);
        var_deltaphi_dn9 = (var_devsign * var_phig_i_dn9);
        var_deltaphi_dn10 = (var_devsign * var_phig_i_dn10);
        var_deltaphi_dn11 = (var_devsign * var_phig_i_dn11);
        var_deltaphi_dn13 = (var_devsign * var_phig_i_dn13);
        var_deltaphi_dn14 = (var_devsign * var_phig_i_dn14);
        var_deltaphi_rv = 0.0;

        let assign18890_e36631: f64 = (0.5 * var_etamob_t);
        var_eta_mu = assign18890_e36631;
        var_eta_mu_dn4 = (0.5 * var_etamob_t_dn4);
        var_eta_mu_rv = 0.0;

        var_eta_mu_cv = 0.5;
        var_eta_mu_cv_rv = 0.0;

        let assign18910_e36635: f64 = if p.p60 != 1.0 { 1.0 } else { 0.0 };
        var_guard349 = assign18910_e36635;
        var_guard349_rv = 0.0;

        let (assign18920_e36641, assign18920_e36641_d_n4,) = {
    if (var_guard349 != 0.0) {
        let assign18920_e36639: f64 = (0.333333333 * var_etamob_t);
        (assign18920_e36639, (0.333333333 * var_etamob_t_dn4),)
    } else {
        (var_eta_mu, var_eta_mu_dn4,)
    }
};
        var_eta_mu = assign18920_e36641;
        var_eta_mu_dn4 = assign18920_e36641_d_n4;
        var_eta_mu_rv = 0.0;

        let (assign18930_e36645,) = {
    if (var_guard349 != 0.0) {
        (0.333333333,)
    } else {
        (var_eta_mu_cv,)
    }
};
        var_eta_mu_cv = assign18930_e36645;
        var_eta_mu_cv_rv = 0.0;

        let assign18940_e36648: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        var_guard350 = assign18940_e36648;
        var_guard350_rv = 0.0;

        let (assign18950_e36664, assign18950_e36664_d_n0, assign18950_e36664_d_n2, assign18950_e36664_d_n3, assign18950_e36664_d_n4, assign18950_e36664_d_n5, assign18950_e36664_d_n6, assign18950_e36664_d_n7, assign18950_e36664_d_n8, assign18950_e36664_d_n9, assign18950_e36664_d_n10, assign18950_e36664_d_n11, assign18950_e36664_d_n13, assign18950_e36664_d_n14,) = {
    if (var_guard350 != 0.0) {
        let assign18950_e36652: f64 = (p.p11 * var_jss_t);
        let assign18950_e36655: f64 = (p.p13 * var_jsws_t);
        let assign18950_e36656: f64 = (assign18950_e36652 + assign18950_e36655);
        let assign18950_e36659: f64 = (p.p3 * var_nfintotal);
        let assign18950_e36661: f64 = (assign18950_e36659 * var_jswgs_t);
        let assign18950_e36662: f64 = (assign18950_e36656 + assign18950_e36661);
        (assign18950_e36662, (((p.p11 * var_jss_t_dn0) + (p.p13 * var_jsws_t_dn0)) + (assign18950_e36659 * var_jswgs_t_dn0)), (((p.p11 * var_jss_t_dn2) + (p.p13 * var_jsws_t_dn2)) + (assign18950_e36659 * var_jswgs_t_dn2)), (((p.p11 * var_jss_t_dn3) + (p.p13 * var_jsws_t_dn3)) + (assign18950_e36659 * var_jswgs_t_dn3)), (((p.p11 * var_jss_t_dn4) + (p.p13 * var_jsws_t_dn4)) + (assign18950_e36659 * var_jswgs_t_dn4)), (((p.p11 * var_jss_t_dn5) + (p.p13 * var_jsws_t_dn5)) + (assign18950_e36659 * var_jswgs_t_dn5)), (((p.p11 * var_jss_t_dn6) + (p.p13 * var_jsws_t_dn6)) + (assign18950_e36659 * var_jswgs_t_dn6)), (((p.p11 * var_jss_t_dn7) + (p.p13 * var_jsws_t_dn7)) + (assign18950_e36659 * var_jswgs_t_dn7)), (((p.p11 * var_jss_t_dn8) + (p.p13 * var_jsws_t_dn8)) + (assign18950_e36659 * var_jswgs_t_dn8)), (((p.p11 * var_jss_t_dn9) + (p.p13 * var_jsws_t_dn9)) + (assign18950_e36659 * var_jswgs_t_dn9)), (((p.p11 * var_jss_t_dn10) + (p.p13 * var_jsws_t_dn10)) + (assign18950_e36659 * var_jswgs_t_dn10)), (((p.p11 * var_jss_t_dn11) + (p.p13 * var_jsws_t_dn11)) + (assign18950_e36659 * var_jswgs_t_dn11)), (((p.p11 * var_jss_t_dn13) + (p.p13 * var_jsws_t_dn13)) + (assign18950_e36659 * var_jswgs_t_dn13)), (((p.p11 * var_jss_t_dn14) + (p.p13 * var_jsws_t_dn14)) + (assign18950_e36659 * var_jswgs_t_dn14)),)
    } else {
        (var_isbs, var_isbs_dn0, var_isbs_dn2, var_isbs_dn3, var_isbs_dn4, var_isbs_dn5, var_isbs_dn6, var_isbs_dn7, var_isbs_dn8, var_isbs_dn9, var_isbs_dn10, var_isbs_dn11, var_isbs_dn13, var_isbs_dn14,)
    }
};
        var_isbs = assign18950_e36664;
        var_isbs_dn0 = assign18950_e36664_d_n0;
        var_isbs_dn2 = assign18950_e36664_d_n2;
        var_isbs_dn3 = assign18950_e36664_d_n3;
        var_isbs_dn4 = assign18950_e36664_d_n4;
        var_isbs_dn5 = assign18950_e36664_d_n5;
        var_isbs_dn6 = assign18950_e36664_d_n6;
        var_isbs_dn7 = assign18950_e36664_d_n7;
        var_isbs_dn8 = assign18950_e36664_d_n8;
        var_isbs_dn9 = assign18950_e36664_d_n9;
        var_isbs_dn10 = assign18950_e36664_d_n10;
        var_isbs_dn11 = assign18950_e36664_d_n11;
        var_isbs_dn13 = assign18950_e36664_d_n13;
        var_isbs_dn14 = assign18950_e36664_d_n14;
        var_isbs_rv = 0.0;

        let assign18960_e36667: f64 = if var_isbs > 0.0 { 1.0 } else { 0.0 };
        var_guard351 = assign18960_e36667;
        var_guard351_rv = 0.0;

        let (assign18970_e36675, assign18970_e36675_d_n4,) = {
    if ((var_guard350 != 0.0) && (var_guard351 != 0.0)) {
        let assign18970_e36673: f64 = (var_vtm * p.p1620);
        (assign18970_e36673, (var_vtm_dn4 * p.p1620),)
    } else {
        (var_nvtms, var_nvtms_dn4,)
    }
};
        var_nvtms = assign18970_e36675;
        var_nvtms_dn4 = assign18970_e36675_d_n4;
        var_nvtms_rv = 0.0;

        let (assign18980_e36687, assign18980_e36687_d_n4,) = {
    if ((var_guard350 != 0.0) && (var_guard351 != 0.0)) {
        let assign18980_e36680: f64 = (-p.p1626);
        let assign18980_e36682: f64 = (assign18980_e36680 / var_nvtms);
        let assign18980_e36683: f64 = { let limited_exp_arg = assign18980_e36682; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18980_e36685: f64 = (assign18980_e36683 * p.p1628);
        (assign18980_e36685, (({ let limited_exp_arg = assign18980_e36682; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-((assign18980_e36680 * var_nvtms_dn4) / (var_nvtms * var_nvtms)))) * p.p1628),)
    } else {
        (var_xexpbvs, var_xexpbvs_dn4,)
    }
};
        var_xexpbvs = assign18980_e36687;
        var_xexpbvs_dn4 = assign18980_e36687_d_n4;
        var_xexpbvs_rv = 0.0;

        let (assign18990_e36697, assign18990_e36697_d_n0, assign18990_e36697_d_n2, assign18990_e36697_d_n3, assign18990_e36697_d_n4, assign18990_e36697_d_n5, assign18990_e36697_d_n6, assign18990_e36697_d_n7, assign18990_e36697_d_n8, assign18990_e36697_d_n9, assign18990_e36697_d_n10, assign18990_e36697_d_n11, assign18990_e36697_d_n13, assign18990_e36697_d_n14,) = {
    if ((var_guard350 != 0.0) && (var_guard351 != 0.0)) {
        let assign18990_e36693: f64 = (p.p1622 / var_isbs);
        let assign18990_e36695: f64 = (assign18990_e36693).max(10.0);
        (assign18990_e36695, if assign18990_e36693 >= 10.0 { (-((p.p1622 * var_isbs_dn0) / (var_isbs * var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * var_isbs_dn2) / (var_isbs * var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * var_isbs_dn3) / (var_isbs * var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * var_isbs_dn4) / (var_isbs * var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * var_isbs_dn5) / (var_isbs * var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * var_isbs_dn6) / (var_isbs * var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * var_isbs_dn7) / (var_isbs * var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * var_isbs_dn8) / (var_isbs * var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * var_isbs_dn9) / (var_isbs * var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * var_isbs_dn10) / (var_isbs * var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * var_isbs_dn11) / (var_isbs * var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * var_isbs_dn13) / (var_isbs * var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * var_isbs_dn14) / (var_isbs * var_isbs))) } else { 0.0 },)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    }
};
        var_t2 = assign18990_e36697;
        var_t2_dn0 = assign18990_e36697_d_n0;
        var_t2_dn2 = assign18990_e36697_d_n2;
        var_t2_dn3 = assign18990_e36697_d_n3;
        var_t2_dn4 = assign18990_e36697_d_n4;
        var_t2_dn5 = assign18990_e36697_d_n5;
        var_t2_dn6 = assign18990_e36697_d_n6;
        var_t2_dn7 = assign18990_e36697_d_n7;
        var_t2_dn8 = assign18990_e36697_d_n8;
        var_t2_dn9 = assign18990_e36697_d_n9;
        var_t2_dn10 = assign18990_e36697_d_n10;
        var_t2_dn11 = assign18990_e36697_d_n11;
        var_t2_dn13 = assign18990_e36697_d_n13;
        var_t2_dn14 = assign18990_e36697_d_n14;
        var_t2_rv = 0.0;

        let (assign19000_e36707, assign19000_e36707_d_n0, assign19000_e36707_d_n2, assign19000_e36707_d_n3, assign19000_e36707_d_n4, assign19000_e36707_d_n5, assign19000_e36707_d_n6, assign19000_e36707_d_n7, assign19000_e36707_d_n8, assign19000_e36707_d_n9, assign19000_e36707_d_n10, assign19000_e36707_d_n11, assign19000_e36707_d_n13, assign19000_e36707_d_n14,) = {
    if ((var_guard350 != 0.0) && (var_guard351 != 0.0)) {
        let assign19000_e36703: f64 = (1.0 + var_t2);
        let assign19000_e36705: f64 = (assign19000_e36703 - var_xexpbvs);
        (assign19000_e36705, var_t2_dn0, var_t2_dn2, var_t2_dn3, (var_t2_dn4 - var_xexpbvs_dn4), var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    } else {
        (var_tb, var_tb_dn0, var_tb_dn2, var_tb_dn3, var_tb_dn4, var_tb_dn5, var_tb_dn6, var_tb_dn7, var_tb_dn8, var_tb_dn9, var_tb_dn10, var_tb_dn11, var_tb_dn13, var_tb_dn14,)
    }
};
        var_tb = assign19000_e36707;
        var_tb_dn0 = assign19000_e36707_d_n0;
        var_tb_dn2 = assign19000_e36707_d_n2;
        var_tb_dn3 = assign19000_e36707_d_n3;
        var_tb_dn4 = assign19000_e36707_d_n4;
        var_tb_dn5 = assign19000_e36707_d_n5;
        var_tb_dn6 = assign19000_e36707_d_n6;
        var_tb_dn7 = assign19000_e36707_d_n7;
        var_tb_dn8 = assign19000_e36707_d_n8;
        var_tb_dn9 = assign19000_e36707_d_n9;
        var_tb_dn10 = assign19000_e36707_d_n10;
        var_tb_dn11 = assign19000_e36707_d_n11;
        var_tb_dn13 = assign19000_e36707_d_n13;
        var_tb_dn14 = assign19000_e36707_d_n14;
        var_tb_rv = 0.0;

        let (assign19010_e36761, assign19010_e36761_d_n0, assign19010_e36761_d_n2, assign19010_e36761_d_n3, assign19010_e36761_d_n4, assign19010_e36761_d_n5, assign19010_e36761_d_n6, assign19010_e36761_d_n7, assign19010_e36761_d_n8, assign19010_e36761_d_n9, assign19010_e36761_d_n10, assign19010_e36761_d_n11, assign19010_e36761_d_n13, assign19010_e36761_d_n14,) = {
    if ((var_guard350 != 0.0) && (var_guard351 != 0.0)) {
        let assign19010_e36716: f64 = (var_tb * var_tb);
        let assign19010_e36719: f64 = (4.0 * var_xexpbvs);
        let assign19010_e36720: f64 = (assign19010_e36716 + assign19010_e36719);
        let assign19010_e36721: f64 = (assign19010_e36720).sqrt();
        let assign19010_e36722: f64 = (var_tb + assign19010_e36721);
        let assign19010_e36723: f64 = (0.5 * assign19010_e36722);
        let (assign19010_e36758, assign19010_e36758_d_n0, assign19010_e36758_d_n2, assign19010_e36758_d_n3, assign19010_e36758_d_n4, assign19010_e36758_d_n5, assign19010_e36758_d_n6, assign19010_e36758_d_n7, assign19010_e36758_d_n8, assign19010_e36758_d_n9, assign19010_e36758_d_n10, assign19010_e36758_d_n11, assign19010_e36758_d_n13, assign19010_e36758_d_n14,) = {
            if (!(assign19010_e36723 > 1e-38)) {
                let assign19010_e36728: f64 = (-87.498233534);
                (assign19010_e36728, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign19010_e36733: f64 = (var_tb * var_tb);
                let assign19010_e36736: f64 = (4.0 * var_xexpbvs);
                let assign19010_e36737: f64 = (assign19010_e36733 + assign19010_e36736);
                let assign19010_e36738: f64 = (assign19010_e36737).sqrt();
                let assign19010_e36739: f64 = (var_tb + assign19010_e36738);
                let assign19010_e36740: f64 = (0.5 * assign19010_e36739);
                let (assign19010_e36757, assign19010_e36757_d_n0, assign19010_e36757_d_n2, assign19010_e36757_d_n3, assign19010_e36757_d_n4, assign19010_e36757_d_n5, assign19010_e36757_d_n6, assign19010_e36757_d_n7, assign19010_e36757_d_n8, assign19010_e36757_d_n9, assign19010_e36757_d_n10, assign19010_e36757_d_n11, assign19010_e36757_d_n13, assign19010_e36757_d_n14,) = {
                    if (assign19010_e36740 > 1e-38) {
                        let assign19010_e36747: f64 = (var_tb * var_tb);
                        let assign19010_e36750: f64 = (4.0 * var_xexpbvs);
                        let assign19010_e36751: f64 = (assign19010_e36747 + assign19010_e36750);
                        let assign19010_e36752: f64 = (assign19010_e36751).sqrt();
                        let assign19010_e36753: f64 = (var_tb + assign19010_e36752);
                        let assign19010_e36754: f64 = (0.5 * assign19010_e36753);
                        let assign19010_e36755: f64 = (assign19010_e36754).ln();
                        (assign19010_e36755, ((0.5 * (var_tb_dn0 + (((var_tb_dn0 * var_tb) + (var_tb * var_tb_dn0)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (var_tb_dn2 + (((var_tb_dn2 * var_tb) + (var_tb * var_tb_dn2)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (var_tb_dn3 + (((var_tb_dn3 * var_tb) + (var_tb * var_tb_dn3)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (var_tb_dn4 + ((((var_tb_dn4 * var_tb) + (var_tb * var_tb_dn4)) + (4.0 * var_xexpbvs_dn4)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (var_tb_dn5 + (((var_tb_dn5 * var_tb) + (var_tb * var_tb_dn5)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (var_tb_dn6 + (((var_tb_dn6 * var_tb) + (var_tb * var_tb_dn6)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (var_tb_dn7 + (((var_tb_dn7 * var_tb) + (var_tb * var_tb_dn7)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (var_tb_dn8 + (((var_tb_dn8 * var_tb) + (var_tb * var_tb_dn8)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (var_tb_dn9 + (((var_tb_dn9 * var_tb) + (var_tb * var_tb_dn9)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (var_tb_dn10 + (((var_tb_dn10 * var_tb) + (var_tb * var_tb_dn10)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (var_tb_dn11 + (((var_tb_dn11 * var_tb) + (var_tb * var_tb_dn11)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (var_tb_dn13 + (((var_tb_dn13 * var_tb) + (var_tb * var_tb_dn13)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (var_tb_dn14 + (((var_tb_dn14 * var_tb) + (var_tb * var_tb_dn14)) / (2.0 * assign19010_e36752)))) / assign19010_e36754),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign19010_e36757, assign19010_e36757_d_n0, assign19010_e36757_d_n2, assign19010_e36757_d_n3, assign19010_e36757_d_n4, assign19010_e36757_d_n5, assign19010_e36757_d_n6, assign19010_e36757_d_n7, assign19010_e36757_d_n8, assign19010_e36757_d_n9, assign19010_e36757_d_n10, assign19010_e36757_d_n11, assign19010_e36757_d_n13, assign19010_e36757_d_n14,)
            }
        };
        let assign19010_e36759: f64 = (var_nvtms * assign19010_e36758);
        (assign19010_e36759, (var_nvtms * assign19010_e36758_d_n0), (var_nvtms * assign19010_e36758_d_n2), (var_nvtms * assign19010_e36758_d_n3), ((var_nvtms_dn4 * assign19010_e36758) + (var_nvtms * assign19010_e36758_d_n4)), (var_nvtms * assign19010_e36758_d_n5), (var_nvtms * assign19010_e36758_d_n6), (var_nvtms * assign19010_e36758_d_n7), (var_nvtms * assign19010_e36758_d_n8), (var_nvtms * assign19010_e36758_d_n9), (var_nvtms * assign19010_e36758_d_n10), (var_nvtms * assign19010_e36758_d_n11), (var_nvtms * assign19010_e36758_d_n13), (var_nvtms * assign19010_e36758_d_n14),)
    } else {
        (var_vjsmfwd, var_vjsmfwd_dn0, var_vjsmfwd_dn2, var_vjsmfwd_dn3, var_vjsmfwd_dn4, var_vjsmfwd_dn5, var_vjsmfwd_dn6, var_vjsmfwd_dn7, var_vjsmfwd_dn8, var_vjsmfwd_dn9, var_vjsmfwd_dn10, var_vjsmfwd_dn11, var_vjsmfwd_dn13, var_vjsmfwd_dn14,)
    }
};
        var_vjsmfwd = assign19010_e36761;
        var_vjsmfwd_dn0 = assign19010_e36761_d_n0;
        var_vjsmfwd_dn2 = assign19010_e36761_d_n2;
        var_vjsmfwd_dn3 = assign19010_e36761_d_n3;
        var_vjsmfwd_dn4 = assign19010_e36761_d_n4;
        var_vjsmfwd_dn5 = assign19010_e36761_d_n5;
        var_vjsmfwd_dn6 = assign19010_e36761_d_n6;
        var_vjsmfwd_dn7 = assign19010_e36761_d_n7;
        var_vjsmfwd_dn8 = assign19010_e36761_d_n8;
        var_vjsmfwd_dn9 = assign19010_e36761_d_n9;
        var_vjsmfwd_dn10 = assign19010_e36761_d_n10;
        var_vjsmfwd_dn11 = assign19010_e36761_d_n11;
        var_vjsmfwd_dn13 = assign19010_e36761_d_n13;
        var_vjsmfwd_dn14 = assign19010_e36761_d_n14;
        var_vjsmfwd_rv = 0.0;

        let (assign19020_e36770, assign19020_e36770_d_n0, assign19020_e36770_d_n2, assign19020_e36770_d_n3, assign19020_e36770_d_n4, assign19020_e36770_d_n5, assign19020_e36770_d_n6, assign19020_e36770_d_n7, assign19020_e36770_d_n8, assign19020_e36770_d_n9, assign19020_e36770_d_n10, assign19020_e36770_d_n11, assign19020_e36770_d_n13, assign19020_e36770_d_n14,) = {
    if ((var_guard350 != 0.0) && (var_guard351 != 0.0)) {
        let assign19020_e36767: f64 = (var_vjsmfwd / var_nvtms);
        let assign19020_e36768: f64 = { let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign19020_e36768, ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjsmfwd_dn0 / var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjsmfwd_dn2 / var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjsmfwd_dn3 / var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((var_vjsmfwd_dn4 * var_nvtms) - (var_vjsmfwd * var_nvtms_dn4)) / (var_nvtms * var_nvtms))), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjsmfwd_dn5 / var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjsmfwd_dn6 / var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjsmfwd_dn7 / var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjsmfwd_dn8 / var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjsmfwd_dn9 / var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjsmfwd_dn10 / var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjsmfwd_dn11 / var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjsmfwd_dn13 / var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjsmfwd_dn14 / var_nvtms)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn13, var_t0_dn14,)
    }
};
        var_t0 = assign19020_e36770;
        var_t0_dn0 = assign19020_e36770_d_n0;
        var_t0_dn2 = assign19020_e36770_d_n2;
        var_t0_dn3 = assign19020_e36770_d_n3;
        var_t0_dn4 = assign19020_e36770_d_n4;
        var_t0_dn5 = assign19020_e36770_d_n5;
        var_t0_dn6 = assign19020_e36770_d_n6;
        var_t0_dn7 = assign19020_e36770_d_n7;
        var_t0_dn8 = assign19020_e36770_d_n8;
        var_t0_dn9 = assign19020_e36770_d_n9;
        var_t0_dn10 = assign19020_e36770_d_n10;
        var_t0_dn11 = assign19020_e36770_d_n11;
        var_t0_dn13 = assign19020_e36770_d_n13;
        var_t0_dn14 = assign19020_e36770_d_n14;
        var_t0_rv = 0.0;

        *var_deltaphi_slot = var_deltaphi;
        *var_deltaphi_dn0_slot = var_deltaphi_dn0;
        *var_deltaphi_dn10_slot = var_deltaphi_dn10;
        *var_deltaphi_dn11_slot = var_deltaphi_dn11;
        *var_deltaphi_dn13_slot = var_deltaphi_dn13;
        *var_deltaphi_dn14_slot = var_deltaphi_dn14;
        *var_deltaphi_dn2_slot = var_deltaphi_dn2;
        *var_deltaphi_dn3_slot = var_deltaphi_dn3;
        *var_deltaphi_dn4_slot = var_deltaphi_dn4;
        *var_deltaphi_dn5_slot = var_deltaphi_dn5;
        *var_deltaphi_dn6_slot = var_deltaphi_dn6;
        *var_deltaphi_dn7_slot = var_deltaphi_dn7;
        *var_deltaphi_dn8_slot = var_deltaphi_dn8;
        *var_deltaphi_dn9_slot = var_deltaphi_dn9;
        *var_deltaphi_rv_slot = var_deltaphi_rv;
        *var_eta_mu_slot = var_eta_mu;
        *var_eta_mu_cv_slot = var_eta_mu_cv;
        *var_eta_mu_cv_rv_slot = var_eta_mu_cv_rv;
        *var_eta_mu_dn4_slot = var_eta_mu_dn4;
        *var_eta_mu_rv_slot = var_eta_mu_rv;
        *var_guard349_slot = var_guard349;
        *var_guard349_rv_slot = var_guard349_rv;
        *var_guard350_slot = var_guard350;
        *var_guard350_rv_slot = var_guard350_rv;
        *var_guard351_slot = var_guard351;
        *var_guard351_rv_slot = var_guard351_rv;
        *var_isbs_slot = var_isbs;
        *var_isbs_dn0_slot = var_isbs_dn0;
        *var_isbs_dn10_slot = var_isbs_dn10;
        *var_isbs_dn11_slot = var_isbs_dn11;
        *var_isbs_dn13_slot = var_isbs_dn13;
        *var_isbs_dn14_slot = var_isbs_dn14;
        *var_isbs_dn2_slot = var_isbs_dn2;
        *var_isbs_dn3_slot = var_isbs_dn3;
        *var_isbs_dn4_slot = var_isbs_dn4;
        *var_isbs_dn5_slot = var_isbs_dn5;
        *var_isbs_dn6_slot = var_isbs_dn6;
        *var_isbs_dn7_slot = var_isbs_dn7;
        *var_isbs_dn8_slot = var_isbs_dn8;
        *var_isbs_dn9_slot = var_isbs_dn9;
        *var_isbs_rv_slot = var_isbs_rv;
        *var_nvtms_slot = var_nvtms;
        *var_nvtms_dn4_slot = var_nvtms_dn4;
        *var_nvtms_rv_slot = var_nvtms_rv;
        *var_phib_slot = var_phib;
        *var_phib_dn0_slot = var_phib_dn0;
        *var_phib_dn10_slot = var_phib_dn10;
        *var_phib_dn11_slot = var_phib_dn11;
        *var_phib_dn13_slot = var_phib_dn13;
        *var_phib_dn14_slot = var_phib_dn14;
        *var_phib_dn2_slot = var_phib_dn2;
        *var_phib_dn3_slot = var_phib_dn3;
        *var_phib_dn4_slot = var_phib_dn4;
        *var_phib_dn5_slot = var_phib_dn5;
        *var_phib_dn6_slot = var_phib_dn6;
        *var_phib_dn7_slot = var_phib_dn7;
        *var_phib_dn8_slot = var_phib_dn8;
        *var_phib_dn9_slot = var_phib_dn9;
        *var_phib_rv_slot = var_phib_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn13_slot = var_t0_dn13;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_tb_slot = var_tb;
        *var_tb_dn0_slot = var_tb_dn0;
        *var_tb_dn10_slot = var_tb_dn10;
        *var_tb_dn11_slot = var_tb_dn11;
        *var_tb_dn13_slot = var_tb_dn13;
        *var_tb_dn14_slot = var_tb_dn14;
        *var_tb_dn2_slot = var_tb_dn2;
        *var_tb_dn3_slot = var_tb_dn3;
        *var_tb_dn4_slot = var_tb_dn4;
        *var_tb_dn5_slot = var_tb_dn5;
        *var_tb_dn6_slot = var_tb_dn6;
        *var_tb_dn7_slot = var_tb_dn7;
        *var_tb_dn8_slot = var_tb_dn8;
        *var_tb_dn9_slot = var_tb_dn9;
        *var_tb_rv_slot = var_tb_rv;
        *var_vbi_slot = var_vbi;
        *var_vbi_dn0_slot = var_vbi_dn0;
        *var_vbi_dn10_slot = var_vbi_dn10;
        *var_vbi_dn11_slot = var_vbi_dn11;
        *var_vbi_dn13_slot = var_vbi_dn13;
        *var_vbi_dn14_slot = var_vbi_dn14;
        *var_vbi_dn2_slot = var_vbi_dn2;
        *var_vbi_dn3_slot = var_vbi_dn3;
        *var_vbi_dn4_slot = var_vbi_dn4;
        *var_vbi_dn5_slot = var_vbi_dn5;
        *var_vbi_dn6_slot = var_vbi_dn6;
        *var_vbi_dn7_slot = var_vbi_dn7;
        *var_vbi_dn8_slot = var_vbi_dn8;
        *var_vbi_dn9_slot = var_vbi_dn9;
        *var_vbi_rv_slot = var_vbi_rv;
        *var_vjsmfwd_slot = var_vjsmfwd;
        *var_vjsmfwd_dn0_slot = var_vjsmfwd_dn0;
        *var_vjsmfwd_dn10_slot = var_vjsmfwd_dn10;
        *var_vjsmfwd_dn11_slot = var_vjsmfwd_dn11;
        *var_vjsmfwd_dn13_slot = var_vjsmfwd_dn13;
        *var_vjsmfwd_dn14_slot = var_vjsmfwd_dn14;
        *var_vjsmfwd_dn2_slot = var_vjsmfwd_dn2;
        *var_vjsmfwd_dn3_slot = var_vjsmfwd_dn3;
        *var_vjsmfwd_dn4_slot = var_vjsmfwd_dn4;
        *var_vjsmfwd_dn5_slot = var_vjsmfwd_dn5;
        *var_vjsmfwd_dn6_slot = var_vjsmfwd_dn6;
        *var_vjsmfwd_dn7_slot = var_vjsmfwd_dn7;
        *var_vjsmfwd_dn8_slot = var_vjsmfwd_dn8;
        *var_vjsmfwd_dn9_slot = var_vjsmfwd_dn9;
        *var_vjsmfwd_rv_slot = var_vjsmfwd_rv;
        *var_xexpbvs_slot = var_xexpbvs;
        *var_xexpbvs_dn4_slot = var_xexpbvs_dn4;
        *var_xexpbvs_rv_slot = var_xexpbvs_rv;
    }

    pub(super) fn stamp_reactive_block_71(
        p: &Parameters,
        var_guard350: f64,
        var_guard351: f64,
        var_isbs: f64,
        var_isbs_dn0: f64,
        var_isbs_dn10: f64,
        var_isbs_dn11: f64,
        var_isbs_dn13: f64,
        var_isbs_dn14: f64,
        var_isbs_dn2: f64,
        var_isbs_dn3: f64,
        var_isbs_dn4: f64,
        var_isbs_dn5: f64,
        var_isbs_dn6: f64,
        var_isbs_dn7: f64,
        var_isbs_dn8: f64,
        var_isbs_dn9: f64,
        var_jsd_t: f64,
        var_jsd_t_dn0: f64,
        var_jsd_t_dn10: f64,
        var_jsd_t_dn11: f64,
        var_jsd_t_dn13: f64,
        var_jsd_t_dn14: f64,
        var_jsd_t_dn2: f64,
        var_jsd_t_dn3: f64,
        var_jsd_t_dn4: f64,
        var_jsd_t_dn5: f64,
        var_jsd_t_dn6: f64,
        var_jsd_t_dn7: f64,
        var_jsd_t_dn8: f64,
        var_jsd_t_dn9: f64,
        var_jswd_t: f64,
        var_jswd_t_dn0: f64,
        var_jswd_t_dn10: f64,
        var_jswd_t_dn11: f64,
        var_jswd_t_dn13: f64,
        var_jswd_t_dn14: f64,
        var_jswd_t_dn2: f64,
        var_jswd_t_dn3: f64,
        var_jswd_t_dn4: f64,
        var_jswd_t_dn5: f64,
        var_jswd_t_dn6: f64,
        var_jswd_t_dn7: f64,
        var_jswd_t_dn8: f64,
        var_jswd_t_dn9: f64,
        var_jswgd_t: f64,
        var_jswgd_t_dn0: f64,
        var_jswgd_t_dn10: f64,
        var_jswgd_t_dn11: f64,
        var_jswgd_t_dn13: f64,
        var_jswgd_t_dn14: f64,
        var_jswgd_t_dn2: f64,
        var_jswgd_t_dn3: f64,
        var_jswgd_t_dn4: f64,
        var_jswgd_t_dn5: f64,
        var_jswgd_t_dn6: f64,
        var_jswgd_t_dn7: f64,
        var_jswgd_t_dn8: f64,
        var_jswgd_t_dn9: f64,
        var_nfintotal: f64,
        var_nvtms: f64,
        var_nvtms_dn4: f64,
        var_vtm: f64,
        var_vtm_dn4: f64,
        var_guard352_slot: &mut f64,
        var_guard352_rv_slot: &mut f64,
        var_isbd_slot: &mut f64,
        var_isbd_dn0_slot: &mut f64,
        var_isbd_dn10_slot: &mut f64,
        var_isbd_dn11_slot: &mut f64,
        var_isbd_dn13_slot: &mut f64,
        var_isbd_dn14_slot: &mut f64,
        var_isbd_dn2_slot: &mut f64,
        var_isbd_dn3_slot: &mut f64,
        var_isbd_dn4_slot: &mut f64,
        var_isbd_dn5_slot: &mut f64,
        var_isbd_dn6_slot: &mut f64,
        var_isbd_dn7_slot: &mut f64,
        var_isbd_dn8_slot: &mut f64,
        var_isbd_dn9_slot: &mut f64,
        var_isbd_rv_slot: &mut f64,
        var_ivjsmrev_slot: &mut f64,
        var_ivjsmrev_dn0_slot: &mut f64,
        var_ivjsmrev_dn10_slot: &mut f64,
        var_ivjsmrev_dn11_slot: &mut f64,
        var_ivjsmrev_dn13_slot: &mut f64,
        var_ivjsmrev_dn14_slot: &mut f64,
        var_ivjsmrev_dn2_slot: &mut f64,
        var_ivjsmrev_dn3_slot: &mut f64,
        var_ivjsmrev_dn4_slot: &mut f64,
        var_ivjsmrev_dn5_slot: &mut f64,
        var_ivjsmrev_dn6_slot: &mut f64,
        var_ivjsmrev_dn7_slot: &mut f64,
        var_ivjsmrev_dn8_slot: &mut f64,
        var_ivjsmrev_dn9_slot: &mut f64,
        var_ivjsmrev_rv_slot: &mut f64,
        var_nvtmd_slot: &mut f64,
        var_nvtmd_dn4_slot: &mut f64,
        var_nvtmd_rv_slot: &mut f64,
        var_sslprev_slot: &mut f64,
        var_sslprev_dn0_slot: &mut f64,
        var_sslprev_dn10_slot: &mut f64,
        var_sslprev_dn11_slot: &mut f64,
        var_sslprev_dn13_slot: &mut f64,
        var_sslprev_dn14_slot: &mut f64,
        var_sslprev_dn2_slot: &mut f64,
        var_sslprev_dn3_slot: &mut f64,
        var_sslprev_dn4_slot: &mut f64,
        var_sslprev_dn5_slot: &mut f64,
        var_sslprev_dn6_slot: &mut f64,
        var_sslprev_dn7_slot: &mut f64,
        var_sslprev_dn8_slot: &mut f64,
        var_sslprev_dn9_slot: &mut f64,
        var_sslprev_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
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
        var_t1_dn11_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
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
        var_t2_dn11_slot: &mut f64,
        var_t2_dn13_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_tb_slot: &mut f64,
        var_tb_dn0_slot: &mut f64,
        var_tb_dn10_slot: &mut f64,
        var_tb_dn11_slot: &mut f64,
        var_tb_dn13_slot: &mut f64,
        var_tb_dn14_slot: &mut f64,
        var_tb_dn2_slot: &mut f64,
        var_tb_dn3_slot: &mut f64,
        var_tb_dn4_slot: &mut f64,
        var_tb_dn5_slot: &mut f64,
        var_tb_dn6_slot: &mut f64,
        var_tb_dn7_slot: &mut f64,
        var_tb_dn8_slot: &mut f64,
        var_tb_dn9_slot: &mut f64,
        var_tb_rv_slot: &mut f64,
        var_vjdmfwd_slot: &mut f64,
        var_vjdmfwd_dn0_slot: &mut f64,
        var_vjdmfwd_dn10_slot: &mut f64,
        var_vjdmfwd_dn11_slot: &mut f64,
        var_vjdmfwd_dn13_slot: &mut f64,
        var_vjdmfwd_dn14_slot: &mut f64,
        var_vjdmfwd_dn2_slot: &mut f64,
        var_vjdmfwd_dn3_slot: &mut f64,
        var_vjdmfwd_dn4_slot: &mut f64,
        var_vjdmfwd_dn5_slot: &mut f64,
        var_vjdmfwd_dn6_slot: &mut f64,
        var_vjdmfwd_dn7_slot: &mut f64,
        var_vjdmfwd_dn8_slot: &mut f64,
        var_vjdmfwd_dn9_slot: &mut f64,
        var_vjdmfwd_rv_slot: &mut f64,
        var_vjdmrev_slot: &mut f64,
        var_vjdmrev_dn0_slot: &mut f64,
        var_vjdmrev_dn10_slot: &mut f64,
        var_vjdmrev_dn11_slot: &mut f64,
        var_vjdmrev_dn13_slot: &mut f64,
        var_vjdmrev_dn14_slot: &mut f64,
        var_vjdmrev_dn2_slot: &mut f64,
        var_vjdmrev_dn3_slot: &mut f64,
        var_vjdmrev_dn4_slot: &mut f64,
        var_vjdmrev_dn5_slot: &mut f64,
        var_vjdmrev_dn6_slot: &mut f64,
        var_vjdmrev_dn7_slot: &mut f64,
        var_vjdmrev_dn8_slot: &mut f64,
        var_vjdmrev_dn9_slot: &mut f64,
        var_vjdmrev_rv_slot: &mut f64,
        var_vjsmrev_slot: &mut f64,
        var_vjsmrev_dn0_slot: &mut f64,
        var_vjsmrev_dn10_slot: &mut f64,
        var_vjsmrev_dn11_slot: &mut f64,
        var_vjsmrev_dn13_slot: &mut f64,
        var_vjsmrev_dn14_slot: &mut f64,
        var_vjsmrev_dn2_slot: &mut f64,
        var_vjsmrev_dn3_slot: &mut f64,
        var_vjsmrev_dn4_slot: &mut f64,
        var_vjsmrev_dn5_slot: &mut f64,
        var_vjsmrev_dn6_slot: &mut f64,
        var_vjsmrev_dn7_slot: &mut f64,
        var_vjsmrev_dn8_slot: &mut f64,
        var_vjsmrev_dn9_slot: &mut f64,
        var_vjsmrev_rv_slot: &mut f64,
        var_xexpbvd_slot: &mut f64,
        var_xexpbvd_dn4_slot: &mut f64,
        var_xexpbvd_rv_slot: &mut f64,
    ) {
        let mut var_guard352: f64 = *var_guard352_slot;
        let mut var_guard352_rv: f64 = *var_guard352_rv_slot;
        let mut var_isbd: f64 = *var_isbd_slot;
        let mut var_isbd_dn0: f64 = *var_isbd_dn0_slot;
        let mut var_isbd_dn10: f64 = *var_isbd_dn10_slot;
        let mut var_isbd_dn11: f64 = *var_isbd_dn11_slot;
        let mut var_isbd_dn13: f64 = *var_isbd_dn13_slot;
        let mut var_isbd_dn14: f64 = *var_isbd_dn14_slot;
        let mut var_isbd_dn2: f64 = *var_isbd_dn2_slot;
        let mut var_isbd_dn3: f64 = *var_isbd_dn3_slot;
        let mut var_isbd_dn4: f64 = *var_isbd_dn4_slot;
        let mut var_isbd_dn5: f64 = *var_isbd_dn5_slot;
        let mut var_isbd_dn6: f64 = *var_isbd_dn6_slot;
        let mut var_isbd_dn7: f64 = *var_isbd_dn7_slot;
        let mut var_isbd_dn8: f64 = *var_isbd_dn8_slot;
        let mut var_isbd_dn9: f64 = *var_isbd_dn9_slot;
        let mut var_isbd_rv: f64 = *var_isbd_rv_slot;
        let mut var_ivjsmrev: f64 = *var_ivjsmrev_slot;
        let mut var_ivjsmrev_dn0: f64 = *var_ivjsmrev_dn0_slot;
        let mut var_ivjsmrev_dn10: f64 = *var_ivjsmrev_dn10_slot;
        let mut var_ivjsmrev_dn11: f64 = *var_ivjsmrev_dn11_slot;
        let mut var_ivjsmrev_dn13: f64 = *var_ivjsmrev_dn13_slot;
        let mut var_ivjsmrev_dn14: f64 = *var_ivjsmrev_dn14_slot;
        let mut var_ivjsmrev_dn2: f64 = *var_ivjsmrev_dn2_slot;
        let mut var_ivjsmrev_dn3: f64 = *var_ivjsmrev_dn3_slot;
        let mut var_ivjsmrev_dn4: f64 = *var_ivjsmrev_dn4_slot;
        let mut var_ivjsmrev_dn5: f64 = *var_ivjsmrev_dn5_slot;
        let mut var_ivjsmrev_dn6: f64 = *var_ivjsmrev_dn6_slot;
        let mut var_ivjsmrev_dn7: f64 = *var_ivjsmrev_dn7_slot;
        let mut var_ivjsmrev_dn8: f64 = *var_ivjsmrev_dn8_slot;
        let mut var_ivjsmrev_dn9: f64 = *var_ivjsmrev_dn9_slot;
        let mut var_ivjsmrev_rv: f64 = *var_ivjsmrev_rv_slot;
        let mut var_nvtmd: f64 = *var_nvtmd_slot;
        let mut var_nvtmd_dn4: f64 = *var_nvtmd_dn4_slot;
        let mut var_nvtmd_rv: f64 = *var_nvtmd_rv_slot;
        let mut var_sslprev: f64 = *var_sslprev_slot;
        let mut var_sslprev_dn0: f64 = *var_sslprev_dn0_slot;
        let mut var_sslprev_dn10: f64 = *var_sslprev_dn10_slot;
        let mut var_sslprev_dn11: f64 = *var_sslprev_dn11_slot;
        let mut var_sslprev_dn13: f64 = *var_sslprev_dn13_slot;
        let mut var_sslprev_dn14: f64 = *var_sslprev_dn14_slot;
        let mut var_sslprev_dn2: f64 = *var_sslprev_dn2_slot;
        let mut var_sslprev_dn3: f64 = *var_sslprev_dn3_slot;
        let mut var_sslprev_dn4: f64 = *var_sslprev_dn4_slot;
        let mut var_sslprev_dn5: f64 = *var_sslprev_dn5_slot;
        let mut var_sslprev_dn6: f64 = *var_sslprev_dn6_slot;
        let mut var_sslprev_dn7: f64 = *var_sslprev_dn7_slot;
        let mut var_sslprev_dn8: f64 = *var_sslprev_dn8_slot;
        let mut var_sslprev_dn9: f64 = *var_sslprev_dn9_slot;
        let mut var_sslprev_rv: f64 = *var_sslprev_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
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
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
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
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_tb: f64 = *var_tb_slot;
        let mut var_tb_dn0: f64 = *var_tb_dn0_slot;
        let mut var_tb_dn10: f64 = *var_tb_dn10_slot;
        let mut var_tb_dn11: f64 = *var_tb_dn11_slot;
        let mut var_tb_dn13: f64 = *var_tb_dn13_slot;
        let mut var_tb_dn14: f64 = *var_tb_dn14_slot;
        let mut var_tb_dn2: f64 = *var_tb_dn2_slot;
        let mut var_tb_dn3: f64 = *var_tb_dn3_slot;
        let mut var_tb_dn4: f64 = *var_tb_dn4_slot;
        let mut var_tb_dn5: f64 = *var_tb_dn5_slot;
        let mut var_tb_dn6: f64 = *var_tb_dn6_slot;
        let mut var_tb_dn7: f64 = *var_tb_dn7_slot;
        let mut var_tb_dn8: f64 = *var_tb_dn8_slot;
        let mut var_tb_dn9: f64 = *var_tb_dn9_slot;
        let mut var_tb_rv: f64 = *var_tb_rv_slot;
        let mut var_vjdmfwd: f64 = *var_vjdmfwd_slot;
        let mut var_vjdmfwd_dn0: f64 = *var_vjdmfwd_dn0_slot;
        let mut var_vjdmfwd_dn10: f64 = *var_vjdmfwd_dn10_slot;
        let mut var_vjdmfwd_dn11: f64 = *var_vjdmfwd_dn11_slot;
        let mut var_vjdmfwd_dn13: f64 = *var_vjdmfwd_dn13_slot;
        let mut var_vjdmfwd_dn14: f64 = *var_vjdmfwd_dn14_slot;
        let mut var_vjdmfwd_dn2: f64 = *var_vjdmfwd_dn2_slot;
        let mut var_vjdmfwd_dn3: f64 = *var_vjdmfwd_dn3_slot;
        let mut var_vjdmfwd_dn4: f64 = *var_vjdmfwd_dn4_slot;
        let mut var_vjdmfwd_dn5: f64 = *var_vjdmfwd_dn5_slot;
        let mut var_vjdmfwd_dn6: f64 = *var_vjdmfwd_dn6_slot;
        let mut var_vjdmfwd_dn7: f64 = *var_vjdmfwd_dn7_slot;
        let mut var_vjdmfwd_dn8: f64 = *var_vjdmfwd_dn8_slot;
        let mut var_vjdmfwd_dn9: f64 = *var_vjdmfwd_dn9_slot;
        let mut var_vjdmfwd_rv: f64 = *var_vjdmfwd_rv_slot;
        let mut var_vjdmrev: f64 = *var_vjdmrev_slot;
        let mut var_vjdmrev_dn0: f64 = *var_vjdmrev_dn0_slot;
        let mut var_vjdmrev_dn10: f64 = *var_vjdmrev_dn10_slot;
        let mut var_vjdmrev_dn11: f64 = *var_vjdmrev_dn11_slot;
        let mut var_vjdmrev_dn13: f64 = *var_vjdmrev_dn13_slot;
        let mut var_vjdmrev_dn14: f64 = *var_vjdmrev_dn14_slot;
        let mut var_vjdmrev_dn2: f64 = *var_vjdmrev_dn2_slot;
        let mut var_vjdmrev_dn3: f64 = *var_vjdmrev_dn3_slot;
        let mut var_vjdmrev_dn4: f64 = *var_vjdmrev_dn4_slot;
        let mut var_vjdmrev_dn5: f64 = *var_vjdmrev_dn5_slot;
        let mut var_vjdmrev_dn6: f64 = *var_vjdmrev_dn6_slot;
        let mut var_vjdmrev_dn7: f64 = *var_vjdmrev_dn7_slot;
        let mut var_vjdmrev_dn8: f64 = *var_vjdmrev_dn8_slot;
        let mut var_vjdmrev_dn9: f64 = *var_vjdmrev_dn9_slot;
        let mut var_vjdmrev_rv: f64 = *var_vjdmrev_rv_slot;
        let mut var_vjsmrev: f64 = *var_vjsmrev_slot;
        let mut var_vjsmrev_dn0: f64 = *var_vjsmrev_dn0_slot;
        let mut var_vjsmrev_dn10: f64 = *var_vjsmrev_dn10_slot;
        let mut var_vjsmrev_dn11: f64 = *var_vjsmrev_dn11_slot;
        let mut var_vjsmrev_dn13: f64 = *var_vjsmrev_dn13_slot;
        let mut var_vjsmrev_dn14: f64 = *var_vjsmrev_dn14_slot;
        let mut var_vjsmrev_dn2: f64 = *var_vjsmrev_dn2_slot;
        let mut var_vjsmrev_dn3: f64 = *var_vjsmrev_dn3_slot;
        let mut var_vjsmrev_dn4: f64 = *var_vjsmrev_dn4_slot;
        let mut var_vjsmrev_dn5: f64 = *var_vjsmrev_dn5_slot;
        let mut var_vjsmrev_dn6: f64 = *var_vjsmrev_dn6_slot;
        let mut var_vjsmrev_dn7: f64 = *var_vjsmrev_dn7_slot;
        let mut var_vjsmrev_dn8: f64 = *var_vjsmrev_dn8_slot;
        let mut var_vjsmrev_dn9: f64 = *var_vjsmrev_dn9_slot;
        let mut var_vjsmrev_rv: f64 = *var_vjsmrev_rv_slot;
        let mut var_xexpbvd: f64 = *var_xexpbvd_slot;
        let mut var_xexpbvd_dn4: f64 = *var_xexpbvd_dn4_slot;
        let mut var_xexpbvd_rv: f64 = *var_xexpbvd_rv_slot;

        let (assign19050_e36867, assign19050_e36867_d_n0, assign19050_e36867_d_n2, assign19050_e36867_d_n3, assign19050_e36867_d_n4, assign19050_e36867_d_n5, assign19050_e36867_d_n6, assign19050_e36867_d_n7, assign19050_e36867_d_n8, assign19050_e36867_d_n9, assign19050_e36867_d_n10, assign19050_e36867_d_n11, assign19050_e36867_d_n13, assign19050_e36867_d_n14,) = {
    if ((var_guard350 != 0.0) && (var_guard351 != 0.0)) {
        let assign19050_e36806: f64 = (p.p1624 / var_isbs);
        let assign19050_e36808: f64 = (assign19050_e36806 - 10.0);
        let assign19050_e36810: f64 = (-10000.0);
        let assign19050_e36812: f64 = (assign19050_e36810 * 0.001);
        let (assign19050_e36863, assign19050_e36863_d_n0, assign19050_e36863_d_n2, assign19050_e36863_d_n3, assign19050_e36863_d_n4, assign19050_e36863_d_n5, assign19050_e36863_d_n6, assign19050_e36863_d_n7, assign19050_e36863_d_n8, assign19050_e36863_d_n9, assign19050_e36863_d_n10, assign19050_e36863_d_n11, assign19050_e36863_d_n13, assign19050_e36863_d_n14,) = {
            if (!(assign19050_e36808 < assign19050_e36812)) {
                let assign19050_e36818: f64 = (p.p1624 / var_isbs);
                let assign19050_e36820: f64 = (assign19050_e36818 - 10.0);
                let assign19050_e36823: f64 = (p.p1624 / var_isbs);
                let assign19050_e36825: f64 = (assign19050_e36823 - 10.0);
                let assign19050_e36828: f64 = (p.p1624 / var_isbs);
                let assign19050_e36830: f64 = (assign19050_e36828 - 10.0);
                let assign19050_e36831: f64 = (assign19050_e36825 * assign19050_e36830);
                let assign19050_e36834: f64 = (4.0 * 0.001);
                let assign19050_e36836: f64 = (assign19050_e36834 * 0.001);
                let assign19050_e36837: f64 = (assign19050_e36831 + assign19050_e36836);
                let assign19050_e36838: f64 = (assign19050_e36837).sqrt();
                let assign19050_e36839: f64 = (assign19050_e36820 + assign19050_e36838);
                let assign19050_e36840: f64 = (0.5 * assign19050_e36839);
                (assign19050_e36840, (0.5 * ((-((p.p1624 * var_isbs_dn0) / (var_isbs * var_isbs))) + ((((-((p.p1624 * var_isbs_dn0) / (var_isbs * var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * var_isbs_dn0) / (var_isbs * var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * var_isbs_dn2) / (var_isbs * var_isbs))) + ((((-((p.p1624 * var_isbs_dn2) / (var_isbs * var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * var_isbs_dn2) / (var_isbs * var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * var_isbs_dn3) / (var_isbs * var_isbs))) + ((((-((p.p1624 * var_isbs_dn3) / (var_isbs * var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * var_isbs_dn3) / (var_isbs * var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * var_isbs_dn4) / (var_isbs * var_isbs))) + ((((-((p.p1624 * var_isbs_dn4) / (var_isbs * var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * var_isbs_dn4) / (var_isbs * var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * var_isbs_dn5) / (var_isbs * var_isbs))) + ((((-((p.p1624 * var_isbs_dn5) / (var_isbs * var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * var_isbs_dn5) / (var_isbs * var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * var_isbs_dn6) / (var_isbs * var_isbs))) + ((((-((p.p1624 * var_isbs_dn6) / (var_isbs * var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * var_isbs_dn6) / (var_isbs * var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * var_isbs_dn7) / (var_isbs * var_isbs))) + ((((-((p.p1624 * var_isbs_dn7) / (var_isbs * var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * var_isbs_dn7) / (var_isbs * var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * var_isbs_dn8) / (var_isbs * var_isbs))) + ((((-((p.p1624 * var_isbs_dn8) / (var_isbs * var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * var_isbs_dn8) / (var_isbs * var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * var_isbs_dn9) / (var_isbs * var_isbs))) + ((((-((p.p1624 * var_isbs_dn9) / (var_isbs * var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * var_isbs_dn9) / (var_isbs * var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * var_isbs_dn10) / (var_isbs * var_isbs))) + ((((-((p.p1624 * var_isbs_dn10) / (var_isbs * var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * var_isbs_dn10) / (var_isbs * var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * var_isbs_dn11) / (var_isbs * var_isbs))) + ((((-((p.p1624 * var_isbs_dn11) / (var_isbs * var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * var_isbs_dn11) / (var_isbs * var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * var_isbs_dn13) / (var_isbs * var_isbs))) + ((((-((p.p1624 * var_isbs_dn13) / (var_isbs * var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * var_isbs_dn13) / (var_isbs * var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * var_isbs_dn14) / (var_isbs * var_isbs))) + ((((-((p.p1624 * var_isbs_dn14) / (var_isbs * var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * var_isbs_dn14) / (var_isbs * var_isbs))))) / (2.0 * assign19050_e36838)))),)
            } else {
                let assign19050_e36843: f64 = (p.p1624 / var_isbs);
                let assign19050_e36845: f64 = (assign19050_e36843 - 10.0);
                let assign19050_e36847: f64 = (-10000.0);
                let assign19050_e36849: f64 = (assign19050_e36847 * 0.001);
                let (assign19050_e36862, assign19050_e36862_d_n0, assign19050_e36862_d_n2, assign19050_e36862_d_n3, assign19050_e36862_d_n4, assign19050_e36862_d_n5, assign19050_e36862_d_n6, assign19050_e36862_d_n7, assign19050_e36862_d_n8, assign19050_e36862_d_n9, assign19050_e36862_d_n10, assign19050_e36862_d_n11, assign19050_e36862_d_n13, assign19050_e36862_d_n14,) = {
                    if (assign19050_e36845 < assign19050_e36849) {
                        let assign19050_e36852: f64 = (-0.001);
                        let assign19050_e36854: f64 = (assign19050_e36852 * 0.001);
                        let assign19050_e36857: f64 = (p.p1624 / var_isbs);
                        let assign19050_e36859: f64 = (assign19050_e36857 - 10.0);
                        let assign19050_e36860: f64 = (assign19050_e36854 / assign19050_e36859);
                        (assign19050_e36860, (-((assign19050_e36854 * (-((p.p1624 * var_isbs_dn0) / (var_isbs * var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * var_isbs_dn2) / (var_isbs * var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * var_isbs_dn3) / (var_isbs * var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * var_isbs_dn4) / (var_isbs * var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * var_isbs_dn5) / (var_isbs * var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * var_isbs_dn6) / (var_isbs * var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * var_isbs_dn7) / (var_isbs * var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * var_isbs_dn8) / (var_isbs * var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * var_isbs_dn9) / (var_isbs * var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * var_isbs_dn10) / (var_isbs * var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * var_isbs_dn11) / (var_isbs * var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * var_isbs_dn13) / (var_isbs * var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * var_isbs_dn14) / (var_isbs * var_isbs)))) / (assign19050_e36859 * assign19050_e36859))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign19050_e36862, assign19050_e36862_d_n0, assign19050_e36862_d_n2, assign19050_e36862_d_n3, assign19050_e36862_d_n4, assign19050_e36862_d_n5, assign19050_e36862_d_n6, assign19050_e36862_d_n7, assign19050_e36862_d_n8, assign19050_e36862_d_n9, assign19050_e36862_d_n10, assign19050_e36862_d_n11, assign19050_e36862_d_n13, assign19050_e36862_d_n14,)
            }
        };
        let assign19050_e36865: f64 = (assign19050_e36863 + 10.0);
        (assign19050_e36865, assign19050_e36863_d_n0, assign19050_e36863_d_n2, assign19050_e36863_d_n3, assign19050_e36863_d_n4, assign19050_e36863_d_n5, assign19050_e36863_d_n6, assign19050_e36863_d_n7, assign19050_e36863_d_n8, assign19050_e36863_d_n9, assign19050_e36863_d_n10, assign19050_e36863_d_n11, assign19050_e36863_d_n13, assign19050_e36863_d_n14,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    }
};
        var_t2 = assign19050_e36867;
        var_t2_dn0 = assign19050_e36867_d_n0;
        var_t2_dn2 = assign19050_e36867_d_n2;
        var_t2_dn3 = assign19050_e36867_d_n3;
        var_t2_dn4 = assign19050_e36867_d_n4;
        var_t2_dn5 = assign19050_e36867_d_n5;
        var_t2_dn6 = assign19050_e36867_d_n6;
        var_t2_dn7 = assign19050_e36867_d_n7;
        var_t2_dn8 = assign19050_e36867_d_n8;
        var_t2_dn9 = assign19050_e36867_d_n9;
        var_t2_dn10 = assign19050_e36867_d_n10;
        var_t2_dn11 = assign19050_e36867_d_n11;
        var_t2_dn13 = assign19050_e36867_d_n13;
        var_t2_dn14 = assign19050_e36867_d_n14;
        var_t2_rv = 0.0;

        let (assign19060_e36903, assign19060_e36903_d_n0, assign19060_e36903_d_n2, assign19060_e36903_d_n3, assign19060_e36903_d_n4, assign19060_e36903_d_n5, assign19060_e36903_d_n6, assign19060_e36903_d_n7, assign19060_e36903_d_n8, assign19060_e36903_d_n9, assign19060_e36903_d_n10, assign19060_e36903_d_n11, assign19060_e36903_d_n13, assign19060_e36903_d_n14,) = {
    if ((var_guard350 != 0.0) && (var_guard351 != 0.0)) {
        let assign19060_e36872: f64 = (-p.p1626);
        let assign19060_e36876: f64 = (var_t2 - 1.0);
        let assign19060_e36878: f64 = (assign19060_e36876 / p.p1628);
        let (assign19060_e36899, assign19060_e36899_d_n0, assign19060_e36899_d_n2, assign19060_e36899_d_n3, assign19060_e36899_d_n4, assign19060_e36899_d_n5, assign19060_e36899_d_n6, assign19060_e36899_d_n7, assign19060_e36899_d_n8, assign19060_e36899_d_n9, assign19060_e36899_d_n10, assign19060_e36899_d_n11, assign19060_e36899_d_n13, assign19060_e36899_d_n14,) = {
            if (!(assign19060_e36878 > 1e-38)) {
                let assign19060_e36883: f64 = (-87.498233534);
                (assign19060_e36883, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign19060_e36886: f64 = (var_t2 - 1.0);
                let assign19060_e36888: f64 = (assign19060_e36886 / p.p1628);
                let (assign19060_e36898, assign19060_e36898_d_n0, assign19060_e36898_d_n2, assign19060_e36898_d_n3, assign19060_e36898_d_n4, assign19060_e36898_d_n5, assign19060_e36898_d_n6, assign19060_e36898_d_n7, assign19060_e36898_d_n8, assign19060_e36898_d_n9, assign19060_e36898_d_n10, assign19060_e36898_d_n11, assign19060_e36898_d_n13, assign19060_e36898_d_n14,) = {
                    if (assign19060_e36888 > 1e-38) {
                        let assign19060_e36893: f64 = (var_t2 - 1.0);
                        let assign19060_e36895: f64 = (assign19060_e36893 / p.p1628);
                        let assign19060_e36896: f64 = (assign19060_e36895).ln();
                        (assign19060_e36896, ((var_t2_dn0 / p.p1628) / assign19060_e36895), ((var_t2_dn2 / p.p1628) / assign19060_e36895), ((var_t2_dn3 / p.p1628) / assign19060_e36895), ((var_t2_dn4 / p.p1628) / assign19060_e36895), ((var_t2_dn5 / p.p1628) / assign19060_e36895), ((var_t2_dn6 / p.p1628) / assign19060_e36895), ((var_t2_dn7 / p.p1628) / assign19060_e36895), ((var_t2_dn8 / p.p1628) / assign19060_e36895), ((var_t2_dn9 / p.p1628) / assign19060_e36895), ((var_t2_dn10 / p.p1628) / assign19060_e36895), ((var_t2_dn11 / p.p1628) / assign19060_e36895), ((var_t2_dn13 / p.p1628) / assign19060_e36895), ((var_t2_dn14 / p.p1628) / assign19060_e36895),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign19060_e36898, assign19060_e36898_d_n0, assign19060_e36898_d_n2, assign19060_e36898_d_n3, assign19060_e36898_d_n4, assign19060_e36898_d_n5, assign19060_e36898_d_n6, assign19060_e36898_d_n7, assign19060_e36898_d_n8, assign19060_e36898_d_n9, assign19060_e36898_d_n10, assign19060_e36898_d_n11, assign19060_e36898_d_n13, assign19060_e36898_d_n14,)
            }
        };
        let assign19060_e36900: f64 = (var_nvtms * assign19060_e36899);
        let assign19060_e36901: f64 = (assign19060_e36872 - assign19060_e36900);
        (assign19060_e36901, (-(var_nvtms * assign19060_e36899_d_n0)), (-(var_nvtms * assign19060_e36899_d_n2)), (-(var_nvtms * assign19060_e36899_d_n3)), (-((var_nvtms_dn4 * assign19060_e36899) + (var_nvtms * assign19060_e36899_d_n4))), (-(var_nvtms * assign19060_e36899_d_n5)), (-(var_nvtms * assign19060_e36899_d_n6)), (-(var_nvtms * assign19060_e36899_d_n7)), (-(var_nvtms * assign19060_e36899_d_n8)), (-(var_nvtms * assign19060_e36899_d_n9)), (-(var_nvtms * assign19060_e36899_d_n10)), (-(var_nvtms * assign19060_e36899_d_n11)), (-(var_nvtms * assign19060_e36899_d_n13)), (-(var_nvtms * assign19060_e36899_d_n14)),)
    } else {
        (var_vjsmrev, var_vjsmrev_dn0, var_vjsmrev_dn2, var_vjsmrev_dn3, var_vjsmrev_dn4, var_vjsmrev_dn5, var_vjsmrev_dn6, var_vjsmrev_dn7, var_vjsmrev_dn8, var_vjsmrev_dn9, var_vjsmrev_dn10, var_vjsmrev_dn11, var_vjsmrev_dn13, var_vjsmrev_dn14,)
    }
};
        var_vjsmrev = assign19060_e36903;
        var_vjsmrev_dn0 = assign19060_e36903_d_n0;
        var_vjsmrev_dn2 = assign19060_e36903_d_n2;
        var_vjsmrev_dn3 = assign19060_e36903_d_n3;
        var_vjsmrev_dn4 = assign19060_e36903_d_n4;
        var_vjsmrev_dn5 = assign19060_e36903_d_n5;
        var_vjsmrev_dn6 = assign19060_e36903_d_n6;
        var_vjsmrev_dn7 = assign19060_e36903_d_n7;
        var_vjsmrev_dn8 = assign19060_e36903_d_n8;
        var_vjsmrev_dn9 = assign19060_e36903_d_n9;
        var_vjsmrev_dn10 = assign19060_e36903_d_n10;
        var_vjsmrev_dn11 = assign19060_e36903_d_n11;
        var_vjsmrev_dn13 = assign19060_e36903_d_n13;
        var_vjsmrev_dn14 = assign19060_e36903_d_n14;
        var_vjsmrev_rv = 0.0;

        let (assign19070_e36917, assign19070_e36917_d_n0, assign19070_e36917_d_n2, assign19070_e36917_d_n3, assign19070_e36917_d_n4, assign19070_e36917_d_n5, assign19070_e36917_d_n6, assign19070_e36917_d_n7, assign19070_e36917_d_n8, assign19070_e36917_d_n9, assign19070_e36917_d_n10, assign19070_e36917_d_n11, assign19070_e36917_d_n13, assign19070_e36917_d_n14,) = {
    if ((var_guard350 != 0.0) && (var_guard351 != 0.0)) {
        let assign19070_e36910: f64 = (p.p1626 + var_vjsmrev);
        let assign19070_e36911: f64 = (-assign19070_e36910);
        let assign19070_e36913: f64 = (assign19070_e36911 / var_nvtms);
        let assign19070_e36914: f64 = { let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign19070_e36915: f64 = (p.p1628 * assign19070_e36914);
        (assign19070_e36915, (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjsmrev_dn0) / var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjsmrev_dn2) / var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjsmrev_dn3) / var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((-var_vjsmrev_dn4) * var_nvtms) - (assign19070_e36911 * var_nvtms_dn4)) / (var_nvtms * var_nvtms)))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjsmrev_dn5) / var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjsmrev_dn6) / var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjsmrev_dn7) / var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjsmrev_dn8) / var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjsmrev_dn9) / var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjsmrev_dn10) / var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjsmrev_dn11) / var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjsmrev_dn13) / var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjsmrev_dn14) / var_nvtms))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn13, var_t1_dn14,)
    }
};
        var_t1 = assign19070_e36917;
        var_t1_dn0 = assign19070_e36917_d_n0;
        var_t1_dn2 = assign19070_e36917_d_n2;
        var_t1_dn3 = assign19070_e36917_d_n3;
        var_t1_dn4 = assign19070_e36917_d_n4;
        var_t1_dn5 = assign19070_e36917_d_n5;
        var_t1_dn6 = assign19070_e36917_d_n6;
        var_t1_dn7 = assign19070_e36917_d_n7;
        var_t1_dn8 = assign19070_e36917_d_n8;
        var_t1_dn9 = assign19070_e36917_d_n9;
        var_t1_dn10 = assign19070_e36917_d_n10;
        var_t1_dn11 = assign19070_e36917_d_n11;
        var_t1_dn13 = assign19070_e36917_d_n13;
        var_t1_dn14 = assign19070_e36917_d_n14;
        var_t1_rv = 0.0;

        let (assign19080_e36927, assign19080_e36927_d_n0, assign19080_e36927_d_n2, assign19080_e36927_d_n3, assign19080_e36927_d_n4, assign19080_e36927_d_n5, assign19080_e36927_d_n6, assign19080_e36927_d_n7, assign19080_e36927_d_n8, assign19080_e36927_d_n9, assign19080_e36927_d_n10, assign19080_e36927_d_n11, assign19080_e36927_d_n13, assign19080_e36927_d_n14,) = {
    if ((var_guard350 != 0.0) && (var_guard351 != 0.0)) {
        let assign19080_e36924: f64 = (1.0 + var_t1);
        let assign19080_e36925: f64 = (var_isbs * assign19080_e36924);
        (assign19080_e36925, ((var_isbs_dn0 * assign19080_e36924) + (var_isbs * var_t1_dn0)), ((var_isbs_dn2 * assign19080_e36924) + (var_isbs * var_t1_dn2)), ((var_isbs_dn3 * assign19080_e36924) + (var_isbs * var_t1_dn3)), ((var_isbs_dn4 * assign19080_e36924) + (var_isbs * var_t1_dn4)), ((var_isbs_dn5 * assign19080_e36924) + (var_isbs * var_t1_dn5)), ((var_isbs_dn6 * assign19080_e36924) + (var_isbs * var_t1_dn6)), ((var_isbs_dn7 * assign19080_e36924) + (var_isbs * var_t1_dn7)), ((var_isbs_dn8 * assign19080_e36924) + (var_isbs * var_t1_dn8)), ((var_isbs_dn9 * assign19080_e36924) + (var_isbs * var_t1_dn9)), ((var_isbs_dn10 * assign19080_e36924) + (var_isbs * var_t1_dn10)), ((var_isbs_dn11 * assign19080_e36924) + (var_isbs * var_t1_dn11)), ((var_isbs_dn13 * assign19080_e36924) + (var_isbs * var_t1_dn13)), ((var_isbs_dn14 * assign19080_e36924) + (var_isbs * var_t1_dn14)),)
    } else {
        (var_ivjsmrev, var_ivjsmrev_dn0, var_ivjsmrev_dn2, var_ivjsmrev_dn3, var_ivjsmrev_dn4, var_ivjsmrev_dn5, var_ivjsmrev_dn6, var_ivjsmrev_dn7, var_ivjsmrev_dn8, var_ivjsmrev_dn9, var_ivjsmrev_dn10, var_ivjsmrev_dn11, var_ivjsmrev_dn13, var_ivjsmrev_dn14,)
    }
};
        var_ivjsmrev = assign19080_e36927;
        var_ivjsmrev_dn0 = assign19080_e36927_d_n0;
        var_ivjsmrev_dn2 = assign19080_e36927_d_n2;
        var_ivjsmrev_dn3 = assign19080_e36927_d_n3;
        var_ivjsmrev_dn4 = assign19080_e36927_d_n4;
        var_ivjsmrev_dn5 = assign19080_e36927_d_n5;
        var_ivjsmrev_dn6 = assign19080_e36927_d_n6;
        var_ivjsmrev_dn7 = assign19080_e36927_d_n7;
        var_ivjsmrev_dn8 = assign19080_e36927_d_n8;
        var_ivjsmrev_dn9 = assign19080_e36927_d_n9;
        var_ivjsmrev_dn10 = assign19080_e36927_d_n10;
        var_ivjsmrev_dn11 = assign19080_e36927_d_n11;
        var_ivjsmrev_dn13 = assign19080_e36927_d_n13;
        var_ivjsmrev_dn14 = assign19080_e36927_d_n14;
        var_ivjsmrev_rv = 0.0;

        let (assign19090_e36938, assign19090_e36938_d_n0, assign19090_e36938_d_n2, assign19090_e36938_d_n3, assign19090_e36938_d_n4, assign19090_e36938_d_n5, assign19090_e36938_d_n6, assign19090_e36938_d_n7, assign19090_e36938_d_n8, assign19090_e36938_d_n9, assign19090_e36938_d_n10, assign19090_e36938_d_n11, assign19090_e36938_d_n13, assign19090_e36938_d_n14,) = {
    if ((var_guard350 != 0.0) && (var_guard351 != 0.0)) {
        let assign19090_e36932: f64 = (-var_isbs);
        let assign19090_e36934: f64 = (assign19090_e36932 * var_t1);
        let assign19090_e36936: f64 = (assign19090_e36934 / var_nvtms);
        (assign19090_e36936, ((((-var_isbs_dn0) * var_t1) + (assign19090_e36932 * var_t1_dn0)) / var_nvtms), ((((-var_isbs_dn2) * var_t1) + (assign19090_e36932 * var_t1_dn2)) / var_nvtms), ((((-var_isbs_dn3) * var_t1) + (assign19090_e36932 * var_t1_dn3)) / var_nvtms), ((((((-var_isbs_dn4) * var_t1) + (assign19090_e36932 * var_t1_dn4)) * var_nvtms) - (assign19090_e36934 * var_nvtms_dn4)) / (var_nvtms * var_nvtms)), ((((-var_isbs_dn5) * var_t1) + (assign19090_e36932 * var_t1_dn5)) / var_nvtms), ((((-var_isbs_dn6) * var_t1) + (assign19090_e36932 * var_t1_dn6)) / var_nvtms), ((((-var_isbs_dn7) * var_t1) + (assign19090_e36932 * var_t1_dn7)) / var_nvtms), ((((-var_isbs_dn8) * var_t1) + (assign19090_e36932 * var_t1_dn8)) / var_nvtms), ((((-var_isbs_dn9) * var_t1) + (assign19090_e36932 * var_t1_dn9)) / var_nvtms), ((((-var_isbs_dn10) * var_t1) + (assign19090_e36932 * var_t1_dn10)) / var_nvtms), ((((-var_isbs_dn11) * var_t1) + (assign19090_e36932 * var_t1_dn11)) / var_nvtms), ((((-var_isbs_dn13) * var_t1) + (assign19090_e36932 * var_t1_dn13)) / var_nvtms), ((((-var_isbs_dn14) * var_t1) + (assign19090_e36932 * var_t1_dn14)) / var_nvtms),)
    } else {
        (var_sslprev, var_sslprev_dn0, var_sslprev_dn2, var_sslprev_dn3, var_sslprev_dn4, var_sslprev_dn5, var_sslprev_dn6, var_sslprev_dn7, var_sslprev_dn8, var_sslprev_dn9, var_sslprev_dn10, var_sslprev_dn11, var_sslprev_dn13, var_sslprev_dn14,)
    }
};
        var_sslprev = assign19090_e36938;
        var_sslprev_dn0 = assign19090_e36938_d_n0;
        var_sslprev_dn2 = assign19090_e36938_d_n2;
        var_sslprev_dn3 = assign19090_e36938_d_n3;
        var_sslprev_dn4 = assign19090_e36938_d_n4;
        var_sslprev_dn5 = assign19090_e36938_d_n5;
        var_sslprev_dn6 = assign19090_e36938_d_n6;
        var_sslprev_dn7 = assign19090_e36938_d_n7;
        var_sslprev_dn8 = assign19090_e36938_d_n8;
        var_sslprev_dn9 = assign19090_e36938_d_n9;
        var_sslprev_dn10 = assign19090_e36938_d_n10;
        var_sslprev_dn11 = assign19090_e36938_d_n11;
        var_sslprev_dn13 = assign19090_e36938_d_n13;
        var_sslprev_dn14 = assign19090_e36938_d_n14;
        var_sslprev_rv = 0.0;

        let (assign19100_e36954, assign19100_e36954_d_n0, assign19100_e36954_d_n2, assign19100_e36954_d_n3, assign19100_e36954_d_n4, assign19100_e36954_d_n5, assign19100_e36954_d_n6, assign19100_e36954_d_n7, assign19100_e36954_d_n8, assign19100_e36954_d_n9, assign19100_e36954_d_n10, assign19100_e36954_d_n11, assign19100_e36954_d_n13, assign19100_e36954_d_n14,) = {
    if (var_guard350 != 0.0) {
        let assign19100_e36942: f64 = (p.p12 * var_jsd_t);
        let assign19100_e36945: f64 = (p.p14 * var_jswd_t);
        let assign19100_e36946: f64 = (assign19100_e36942 + assign19100_e36945);
        let assign19100_e36949: f64 = (p.p3 * var_nfintotal);
        let assign19100_e36951: f64 = (assign19100_e36949 * var_jswgd_t);
        let assign19100_e36952: f64 = (assign19100_e36946 + assign19100_e36951);
        (assign19100_e36952, (((p.p12 * var_jsd_t_dn0) + (p.p14 * var_jswd_t_dn0)) + (assign19100_e36949 * var_jswgd_t_dn0)), (((p.p12 * var_jsd_t_dn2) + (p.p14 * var_jswd_t_dn2)) + (assign19100_e36949 * var_jswgd_t_dn2)), (((p.p12 * var_jsd_t_dn3) + (p.p14 * var_jswd_t_dn3)) + (assign19100_e36949 * var_jswgd_t_dn3)), (((p.p12 * var_jsd_t_dn4) + (p.p14 * var_jswd_t_dn4)) + (assign19100_e36949 * var_jswgd_t_dn4)), (((p.p12 * var_jsd_t_dn5) + (p.p14 * var_jswd_t_dn5)) + (assign19100_e36949 * var_jswgd_t_dn5)), (((p.p12 * var_jsd_t_dn6) + (p.p14 * var_jswd_t_dn6)) + (assign19100_e36949 * var_jswgd_t_dn6)), (((p.p12 * var_jsd_t_dn7) + (p.p14 * var_jswd_t_dn7)) + (assign19100_e36949 * var_jswgd_t_dn7)), (((p.p12 * var_jsd_t_dn8) + (p.p14 * var_jswd_t_dn8)) + (assign19100_e36949 * var_jswgd_t_dn8)), (((p.p12 * var_jsd_t_dn9) + (p.p14 * var_jswd_t_dn9)) + (assign19100_e36949 * var_jswgd_t_dn9)), (((p.p12 * var_jsd_t_dn10) + (p.p14 * var_jswd_t_dn10)) + (assign19100_e36949 * var_jswgd_t_dn10)), (((p.p12 * var_jsd_t_dn11) + (p.p14 * var_jswd_t_dn11)) + (assign19100_e36949 * var_jswgd_t_dn11)), (((p.p12 * var_jsd_t_dn13) + (p.p14 * var_jswd_t_dn13)) + (assign19100_e36949 * var_jswgd_t_dn13)), (((p.p12 * var_jsd_t_dn14) + (p.p14 * var_jswd_t_dn14)) + (assign19100_e36949 * var_jswgd_t_dn14)),)
    } else {
        (var_isbd, var_isbd_dn0, var_isbd_dn2, var_isbd_dn3, var_isbd_dn4, var_isbd_dn5, var_isbd_dn6, var_isbd_dn7, var_isbd_dn8, var_isbd_dn9, var_isbd_dn10, var_isbd_dn11, var_isbd_dn13, var_isbd_dn14,)
    }
};
        var_isbd = assign19100_e36954;
        var_isbd_dn0 = assign19100_e36954_d_n0;
        var_isbd_dn2 = assign19100_e36954_d_n2;
        var_isbd_dn3 = assign19100_e36954_d_n3;
        var_isbd_dn4 = assign19100_e36954_d_n4;
        var_isbd_dn5 = assign19100_e36954_d_n5;
        var_isbd_dn6 = assign19100_e36954_d_n6;
        var_isbd_dn7 = assign19100_e36954_d_n7;
        var_isbd_dn8 = assign19100_e36954_d_n8;
        var_isbd_dn9 = assign19100_e36954_d_n9;
        var_isbd_dn10 = assign19100_e36954_d_n10;
        var_isbd_dn11 = assign19100_e36954_d_n11;
        var_isbd_dn13 = assign19100_e36954_d_n13;
        var_isbd_dn14 = assign19100_e36954_d_n14;
        var_isbd_rv = 0.0;

        let assign19110_e36957: f64 = if var_isbd > 0.0 { 1.0 } else { 0.0 };
        var_guard352 = assign19110_e36957;
        var_guard352_rv = 0.0;

        let (assign19120_e36965, assign19120_e36965_d_n4,) = {
    if ((var_guard350 != 0.0) && (var_guard352 != 0.0)) {
        let assign19120_e36963: f64 = (var_vtm * p.p1621);
        (assign19120_e36963, (var_vtm_dn4 * p.p1621),)
    } else {
        (var_nvtmd, var_nvtmd_dn4,)
    }
};
        var_nvtmd = assign19120_e36965;
        var_nvtmd_dn4 = assign19120_e36965_d_n4;
        var_nvtmd_rv = 0.0;

        let (assign19130_e36977, assign19130_e36977_d_n4,) = {
    if ((var_guard350 != 0.0) && (var_guard352 != 0.0)) {
        let assign19130_e36970: f64 = (-p.p1627);
        let assign19130_e36972: f64 = (assign19130_e36970 / var_nvtmd);
        let assign19130_e36973: f64 = { let limited_exp_arg = assign19130_e36972; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign19130_e36975: f64 = (assign19130_e36973 * p.p1629);
        (assign19130_e36975, (({ let limited_exp_arg = assign19130_e36972; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-((assign19130_e36970 * var_nvtmd_dn4) / (var_nvtmd * var_nvtmd)))) * p.p1629),)
    } else {
        (var_xexpbvd, var_xexpbvd_dn4,)
    }
};
        var_xexpbvd = assign19130_e36977;
        var_xexpbvd_dn4 = assign19130_e36977_d_n4;
        var_xexpbvd_rv = 0.0;

        let (assign19140_e36987, assign19140_e36987_d_n0, assign19140_e36987_d_n2, assign19140_e36987_d_n3, assign19140_e36987_d_n4, assign19140_e36987_d_n5, assign19140_e36987_d_n6, assign19140_e36987_d_n7, assign19140_e36987_d_n8, assign19140_e36987_d_n9, assign19140_e36987_d_n10, assign19140_e36987_d_n11, assign19140_e36987_d_n13, assign19140_e36987_d_n14,) = {
    if ((var_guard350 != 0.0) && (var_guard352 != 0.0)) {
        let assign19140_e36983: f64 = (p.p1623 / var_isbd);
        let assign19140_e36985: f64 = (assign19140_e36983).max(10.0);
        (assign19140_e36985, if assign19140_e36983 >= 10.0 { (-((p.p1623 * var_isbd_dn0) / (var_isbd * var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * var_isbd_dn2) / (var_isbd * var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * var_isbd_dn3) / (var_isbd * var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * var_isbd_dn4) / (var_isbd * var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * var_isbd_dn5) / (var_isbd * var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * var_isbd_dn6) / (var_isbd * var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * var_isbd_dn7) / (var_isbd * var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * var_isbd_dn8) / (var_isbd * var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * var_isbd_dn9) / (var_isbd * var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * var_isbd_dn10) / (var_isbd * var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * var_isbd_dn11) / (var_isbd * var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * var_isbd_dn13) / (var_isbd * var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * var_isbd_dn14) / (var_isbd * var_isbd))) } else { 0.0 },)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    }
};
        var_t2 = assign19140_e36987;
        var_t2_dn0 = assign19140_e36987_d_n0;
        var_t2_dn2 = assign19140_e36987_d_n2;
        var_t2_dn3 = assign19140_e36987_d_n3;
        var_t2_dn4 = assign19140_e36987_d_n4;
        var_t2_dn5 = assign19140_e36987_d_n5;
        var_t2_dn6 = assign19140_e36987_d_n6;
        var_t2_dn7 = assign19140_e36987_d_n7;
        var_t2_dn8 = assign19140_e36987_d_n8;
        var_t2_dn9 = assign19140_e36987_d_n9;
        var_t2_dn10 = assign19140_e36987_d_n10;
        var_t2_dn11 = assign19140_e36987_d_n11;
        var_t2_dn13 = assign19140_e36987_d_n13;
        var_t2_dn14 = assign19140_e36987_d_n14;
        var_t2_rv = 0.0;

        let (assign19150_e36997, assign19150_e36997_d_n0, assign19150_e36997_d_n2, assign19150_e36997_d_n3, assign19150_e36997_d_n4, assign19150_e36997_d_n5, assign19150_e36997_d_n6, assign19150_e36997_d_n7, assign19150_e36997_d_n8, assign19150_e36997_d_n9, assign19150_e36997_d_n10, assign19150_e36997_d_n11, assign19150_e36997_d_n13, assign19150_e36997_d_n14,) = {
    if ((var_guard350 != 0.0) && (var_guard352 != 0.0)) {
        let assign19150_e36993: f64 = (1.0 + var_t2);
        let assign19150_e36995: f64 = (assign19150_e36993 - var_xexpbvd);
        (assign19150_e36995, var_t2_dn0, var_t2_dn2, var_t2_dn3, (var_t2_dn4 - var_xexpbvd_dn4), var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    } else {
        (var_tb, var_tb_dn0, var_tb_dn2, var_tb_dn3, var_tb_dn4, var_tb_dn5, var_tb_dn6, var_tb_dn7, var_tb_dn8, var_tb_dn9, var_tb_dn10, var_tb_dn11, var_tb_dn13, var_tb_dn14,)
    }
};
        var_tb = assign19150_e36997;
        var_tb_dn0 = assign19150_e36997_d_n0;
        var_tb_dn2 = assign19150_e36997_d_n2;
        var_tb_dn3 = assign19150_e36997_d_n3;
        var_tb_dn4 = assign19150_e36997_d_n4;
        var_tb_dn5 = assign19150_e36997_d_n5;
        var_tb_dn6 = assign19150_e36997_d_n6;
        var_tb_dn7 = assign19150_e36997_d_n7;
        var_tb_dn8 = assign19150_e36997_d_n8;
        var_tb_dn9 = assign19150_e36997_d_n9;
        var_tb_dn10 = assign19150_e36997_d_n10;
        var_tb_dn11 = assign19150_e36997_d_n11;
        var_tb_dn13 = assign19150_e36997_d_n13;
        var_tb_dn14 = assign19150_e36997_d_n14;
        var_tb_rv = 0.0;

        let (assign19160_e37051, assign19160_e37051_d_n0, assign19160_e37051_d_n2, assign19160_e37051_d_n3, assign19160_e37051_d_n4, assign19160_e37051_d_n5, assign19160_e37051_d_n6, assign19160_e37051_d_n7, assign19160_e37051_d_n8, assign19160_e37051_d_n9, assign19160_e37051_d_n10, assign19160_e37051_d_n11, assign19160_e37051_d_n13, assign19160_e37051_d_n14,) = {
    if ((var_guard350 != 0.0) && (var_guard352 != 0.0)) {
        let assign19160_e37006: f64 = (var_tb * var_tb);
        let assign19160_e37009: f64 = (4.0 * var_xexpbvd);
        let assign19160_e37010: f64 = (assign19160_e37006 + assign19160_e37009);
        let assign19160_e37011: f64 = (assign19160_e37010).sqrt();
        let assign19160_e37012: f64 = (var_tb + assign19160_e37011);
        let assign19160_e37013: f64 = (0.5 * assign19160_e37012);
        let (assign19160_e37048, assign19160_e37048_d_n0, assign19160_e37048_d_n2, assign19160_e37048_d_n3, assign19160_e37048_d_n4, assign19160_e37048_d_n5, assign19160_e37048_d_n6, assign19160_e37048_d_n7, assign19160_e37048_d_n8, assign19160_e37048_d_n9, assign19160_e37048_d_n10, assign19160_e37048_d_n11, assign19160_e37048_d_n13, assign19160_e37048_d_n14,) = {
            if (!(assign19160_e37013 > 1e-38)) {
                let assign19160_e37018: f64 = (-87.498233534);
                (assign19160_e37018, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign19160_e37023: f64 = (var_tb * var_tb);
                let assign19160_e37026: f64 = (4.0 * var_xexpbvd);
                let assign19160_e37027: f64 = (assign19160_e37023 + assign19160_e37026);
                let assign19160_e37028: f64 = (assign19160_e37027).sqrt();
                let assign19160_e37029: f64 = (var_tb + assign19160_e37028);
                let assign19160_e37030: f64 = (0.5 * assign19160_e37029);
                let (assign19160_e37047, assign19160_e37047_d_n0, assign19160_e37047_d_n2, assign19160_e37047_d_n3, assign19160_e37047_d_n4, assign19160_e37047_d_n5, assign19160_e37047_d_n6, assign19160_e37047_d_n7, assign19160_e37047_d_n8, assign19160_e37047_d_n9, assign19160_e37047_d_n10, assign19160_e37047_d_n11, assign19160_e37047_d_n13, assign19160_e37047_d_n14,) = {
                    if (assign19160_e37030 > 1e-38) {
                        let assign19160_e37037: f64 = (var_tb * var_tb);
                        let assign19160_e37040: f64 = (4.0 * var_xexpbvd);
                        let assign19160_e37041: f64 = (assign19160_e37037 + assign19160_e37040);
                        let assign19160_e37042: f64 = (assign19160_e37041).sqrt();
                        let assign19160_e37043: f64 = (var_tb + assign19160_e37042);
                        let assign19160_e37044: f64 = (0.5 * assign19160_e37043);
                        let assign19160_e37045: f64 = (assign19160_e37044).ln();
                        (assign19160_e37045, ((0.5 * (var_tb_dn0 + (((var_tb_dn0 * var_tb) + (var_tb * var_tb_dn0)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (var_tb_dn2 + (((var_tb_dn2 * var_tb) + (var_tb * var_tb_dn2)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (var_tb_dn3 + (((var_tb_dn3 * var_tb) + (var_tb * var_tb_dn3)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (var_tb_dn4 + ((((var_tb_dn4 * var_tb) + (var_tb * var_tb_dn4)) + (4.0 * var_xexpbvd_dn4)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (var_tb_dn5 + (((var_tb_dn5 * var_tb) + (var_tb * var_tb_dn5)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (var_tb_dn6 + (((var_tb_dn6 * var_tb) + (var_tb * var_tb_dn6)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (var_tb_dn7 + (((var_tb_dn7 * var_tb) + (var_tb * var_tb_dn7)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (var_tb_dn8 + (((var_tb_dn8 * var_tb) + (var_tb * var_tb_dn8)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (var_tb_dn9 + (((var_tb_dn9 * var_tb) + (var_tb * var_tb_dn9)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (var_tb_dn10 + (((var_tb_dn10 * var_tb) + (var_tb * var_tb_dn10)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (var_tb_dn11 + (((var_tb_dn11 * var_tb) + (var_tb * var_tb_dn11)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (var_tb_dn13 + (((var_tb_dn13 * var_tb) + (var_tb * var_tb_dn13)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (var_tb_dn14 + (((var_tb_dn14 * var_tb) + (var_tb * var_tb_dn14)) / (2.0 * assign19160_e37042)))) / assign19160_e37044),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign19160_e37047, assign19160_e37047_d_n0, assign19160_e37047_d_n2, assign19160_e37047_d_n3, assign19160_e37047_d_n4, assign19160_e37047_d_n5, assign19160_e37047_d_n6, assign19160_e37047_d_n7, assign19160_e37047_d_n8, assign19160_e37047_d_n9, assign19160_e37047_d_n10, assign19160_e37047_d_n11, assign19160_e37047_d_n13, assign19160_e37047_d_n14,)
            }
        };
        let assign19160_e37049: f64 = (var_nvtmd * assign19160_e37048);
        (assign19160_e37049, (var_nvtmd * assign19160_e37048_d_n0), (var_nvtmd * assign19160_e37048_d_n2), (var_nvtmd * assign19160_e37048_d_n3), ((var_nvtmd_dn4 * assign19160_e37048) + (var_nvtmd * assign19160_e37048_d_n4)), (var_nvtmd * assign19160_e37048_d_n5), (var_nvtmd * assign19160_e37048_d_n6), (var_nvtmd * assign19160_e37048_d_n7), (var_nvtmd * assign19160_e37048_d_n8), (var_nvtmd * assign19160_e37048_d_n9), (var_nvtmd * assign19160_e37048_d_n10), (var_nvtmd * assign19160_e37048_d_n11), (var_nvtmd * assign19160_e37048_d_n13), (var_nvtmd * assign19160_e37048_d_n14),)
    } else {
        (var_vjdmfwd, var_vjdmfwd_dn0, var_vjdmfwd_dn2, var_vjdmfwd_dn3, var_vjdmfwd_dn4, var_vjdmfwd_dn5, var_vjdmfwd_dn6, var_vjdmfwd_dn7, var_vjdmfwd_dn8, var_vjdmfwd_dn9, var_vjdmfwd_dn10, var_vjdmfwd_dn11, var_vjdmfwd_dn13, var_vjdmfwd_dn14,)
    }
};
        var_vjdmfwd = assign19160_e37051;
        var_vjdmfwd_dn0 = assign19160_e37051_d_n0;
        var_vjdmfwd_dn2 = assign19160_e37051_d_n2;
        var_vjdmfwd_dn3 = assign19160_e37051_d_n3;
        var_vjdmfwd_dn4 = assign19160_e37051_d_n4;
        var_vjdmfwd_dn5 = assign19160_e37051_d_n5;
        var_vjdmfwd_dn6 = assign19160_e37051_d_n6;
        var_vjdmfwd_dn7 = assign19160_e37051_d_n7;
        var_vjdmfwd_dn8 = assign19160_e37051_d_n8;
        var_vjdmfwd_dn9 = assign19160_e37051_d_n9;
        var_vjdmfwd_dn10 = assign19160_e37051_d_n10;
        var_vjdmfwd_dn11 = assign19160_e37051_d_n11;
        var_vjdmfwd_dn13 = assign19160_e37051_d_n13;
        var_vjdmfwd_dn14 = assign19160_e37051_d_n14;
        var_vjdmfwd_rv = 0.0;

        let (assign19170_e37060, assign19170_e37060_d_n0, assign19170_e37060_d_n2, assign19170_e37060_d_n3, assign19170_e37060_d_n4, assign19170_e37060_d_n5, assign19170_e37060_d_n6, assign19170_e37060_d_n7, assign19170_e37060_d_n8, assign19170_e37060_d_n9, assign19170_e37060_d_n10, assign19170_e37060_d_n11, assign19170_e37060_d_n13, assign19170_e37060_d_n14,) = {
    if ((var_guard350 != 0.0) && (var_guard352 != 0.0)) {
        let assign19170_e37057: f64 = (var_vjdmfwd / var_nvtmd);
        let assign19170_e37058: f64 = { let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign19170_e37058, ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjdmfwd_dn0 / var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjdmfwd_dn2 / var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjdmfwd_dn3 / var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((var_vjdmfwd_dn4 * var_nvtmd) - (var_vjdmfwd * var_nvtmd_dn4)) / (var_nvtmd * var_nvtmd))), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjdmfwd_dn5 / var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjdmfwd_dn6 / var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjdmfwd_dn7 / var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjdmfwd_dn8 / var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjdmfwd_dn9 / var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjdmfwd_dn10 / var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjdmfwd_dn11 / var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjdmfwd_dn13 / var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_vjdmfwd_dn14 / var_nvtmd)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn13, var_t0_dn14,)
    }
};
        var_t0 = assign19170_e37060;
        var_t0_dn0 = assign19170_e37060_d_n0;
        var_t0_dn2 = assign19170_e37060_d_n2;
        var_t0_dn3 = assign19170_e37060_d_n3;
        var_t0_dn4 = assign19170_e37060_d_n4;
        var_t0_dn5 = assign19170_e37060_d_n5;
        var_t0_dn6 = assign19170_e37060_d_n6;
        var_t0_dn7 = assign19170_e37060_d_n7;
        var_t0_dn8 = assign19170_e37060_d_n8;
        var_t0_dn9 = assign19170_e37060_d_n9;
        var_t0_dn10 = assign19170_e37060_d_n10;
        var_t0_dn11 = assign19170_e37060_d_n11;
        var_t0_dn13 = assign19170_e37060_d_n13;
        var_t0_dn14 = assign19170_e37060_d_n14;
        var_t0_rv = 0.0;

        let (assign19200_e37157, assign19200_e37157_d_n0, assign19200_e37157_d_n2, assign19200_e37157_d_n3, assign19200_e37157_d_n4, assign19200_e37157_d_n5, assign19200_e37157_d_n6, assign19200_e37157_d_n7, assign19200_e37157_d_n8, assign19200_e37157_d_n9, assign19200_e37157_d_n10, assign19200_e37157_d_n11, assign19200_e37157_d_n13, assign19200_e37157_d_n14,) = {
    if ((var_guard350 != 0.0) && (var_guard352 != 0.0)) {
        let assign19200_e37096: f64 = (p.p1625 / var_isbd);
        let assign19200_e37098: f64 = (assign19200_e37096 - 10.0);
        let assign19200_e37100: f64 = (-10000.0);
        let assign19200_e37102: f64 = (assign19200_e37100 * 0.001);
        let (assign19200_e37153, assign19200_e37153_d_n0, assign19200_e37153_d_n2, assign19200_e37153_d_n3, assign19200_e37153_d_n4, assign19200_e37153_d_n5, assign19200_e37153_d_n6, assign19200_e37153_d_n7, assign19200_e37153_d_n8, assign19200_e37153_d_n9, assign19200_e37153_d_n10, assign19200_e37153_d_n11, assign19200_e37153_d_n13, assign19200_e37153_d_n14,) = {
            if (!(assign19200_e37098 < assign19200_e37102)) {
                let assign19200_e37108: f64 = (p.p1625 / var_isbd);
                let assign19200_e37110: f64 = (assign19200_e37108 - 10.0);
                let assign19200_e37113: f64 = (p.p1625 / var_isbd);
                let assign19200_e37115: f64 = (assign19200_e37113 - 10.0);
                let assign19200_e37118: f64 = (p.p1625 / var_isbd);
                let assign19200_e37120: f64 = (assign19200_e37118 - 10.0);
                let assign19200_e37121: f64 = (assign19200_e37115 * assign19200_e37120);
                let assign19200_e37124: f64 = (4.0 * 0.001);
                let assign19200_e37126: f64 = (assign19200_e37124 * 0.001);
                let assign19200_e37127: f64 = (assign19200_e37121 + assign19200_e37126);
                let assign19200_e37128: f64 = (assign19200_e37127).sqrt();
                let assign19200_e37129: f64 = (assign19200_e37110 + assign19200_e37128);
                let assign19200_e37130: f64 = (0.5 * assign19200_e37129);
                (assign19200_e37130, (0.5 * ((-((p.p1625 * var_isbd_dn0) / (var_isbd * var_isbd))) + ((((-((p.p1625 * var_isbd_dn0) / (var_isbd * var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * var_isbd_dn0) / (var_isbd * var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * var_isbd_dn2) / (var_isbd * var_isbd))) + ((((-((p.p1625 * var_isbd_dn2) / (var_isbd * var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * var_isbd_dn2) / (var_isbd * var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * var_isbd_dn3) / (var_isbd * var_isbd))) + ((((-((p.p1625 * var_isbd_dn3) / (var_isbd * var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * var_isbd_dn3) / (var_isbd * var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * var_isbd_dn4) / (var_isbd * var_isbd))) + ((((-((p.p1625 * var_isbd_dn4) / (var_isbd * var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * var_isbd_dn4) / (var_isbd * var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * var_isbd_dn5) / (var_isbd * var_isbd))) + ((((-((p.p1625 * var_isbd_dn5) / (var_isbd * var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * var_isbd_dn5) / (var_isbd * var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * var_isbd_dn6) / (var_isbd * var_isbd))) + ((((-((p.p1625 * var_isbd_dn6) / (var_isbd * var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * var_isbd_dn6) / (var_isbd * var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * var_isbd_dn7) / (var_isbd * var_isbd))) + ((((-((p.p1625 * var_isbd_dn7) / (var_isbd * var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * var_isbd_dn7) / (var_isbd * var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * var_isbd_dn8) / (var_isbd * var_isbd))) + ((((-((p.p1625 * var_isbd_dn8) / (var_isbd * var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * var_isbd_dn8) / (var_isbd * var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * var_isbd_dn9) / (var_isbd * var_isbd))) + ((((-((p.p1625 * var_isbd_dn9) / (var_isbd * var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * var_isbd_dn9) / (var_isbd * var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * var_isbd_dn10) / (var_isbd * var_isbd))) + ((((-((p.p1625 * var_isbd_dn10) / (var_isbd * var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * var_isbd_dn10) / (var_isbd * var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * var_isbd_dn11) / (var_isbd * var_isbd))) + ((((-((p.p1625 * var_isbd_dn11) / (var_isbd * var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * var_isbd_dn11) / (var_isbd * var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * var_isbd_dn13) / (var_isbd * var_isbd))) + ((((-((p.p1625 * var_isbd_dn13) / (var_isbd * var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * var_isbd_dn13) / (var_isbd * var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * var_isbd_dn14) / (var_isbd * var_isbd))) + ((((-((p.p1625 * var_isbd_dn14) / (var_isbd * var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * var_isbd_dn14) / (var_isbd * var_isbd))))) / (2.0 * assign19200_e37128)))),)
            } else {
                let assign19200_e37133: f64 = (p.p1625 / var_isbd);
                let assign19200_e37135: f64 = (assign19200_e37133 - 10.0);
                let assign19200_e37137: f64 = (-10000.0);
                let assign19200_e37139: f64 = (assign19200_e37137 * 0.001);
                let (assign19200_e37152, assign19200_e37152_d_n0, assign19200_e37152_d_n2, assign19200_e37152_d_n3, assign19200_e37152_d_n4, assign19200_e37152_d_n5, assign19200_e37152_d_n6, assign19200_e37152_d_n7, assign19200_e37152_d_n8, assign19200_e37152_d_n9, assign19200_e37152_d_n10, assign19200_e37152_d_n11, assign19200_e37152_d_n13, assign19200_e37152_d_n14,) = {
                    if (assign19200_e37135 < assign19200_e37139) {
                        let assign19200_e37142: f64 = (-0.001);
                        let assign19200_e37144: f64 = (assign19200_e37142 * 0.001);
                        let assign19200_e37147: f64 = (p.p1625 / var_isbd);
                        let assign19200_e37149: f64 = (assign19200_e37147 - 10.0);
                        let assign19200_e37150: f64 = (assign19200_e37144 / assign19200_e37149);
                        (assign19200_e37150, (-((assign19200_e37144 * (-((p.p1625 * var_isbd_dn0) / (var_isbd * var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * var_isbd_dn2) / (var_isbd * var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * var_isbd_dn3) / (var_isbd * var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * var_isbd_dn4) / (var_isbd * var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * var_isbd_dn5) / (var_isbd * var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * var_isbd_dn6) / (var_isbd * var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * var_isbd_dn7) / (var_isbd * var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * var_isbd_dn8) / (var_isbd * var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * var_isbd_dn9) / (var_isbd * var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * var_isbd_dn10) / (var_isbd * var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * var_isbd_dn11) / (var_isbd * var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * var_isbd_dn13) / (var_isbd * var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * var_isbd_dn14) / (var_isbd * var_isbd)))) / (assign19200_e37149 * assign19200_e37149))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign19200_e37152, assign19200_e37152_d_n0, assign19200_e37152_d_n2, assign19200_e37152_d_n3, assign19200_e37152_d_n4, assign19200_e37152_d_n5, assign19200_e37152_d_n6, assign19200_e37152_d_n7, assign19200_e37152_d_n8, assign19200_e37152_d_n9, assign19200_e37152_d_n10, assign19200_e37152_d_n11, assign19200_e37152_d_n13, assign19200_e37152_d_n14,)
            }
        };
        let assign19200_e37155: f64 = (assign19200_e37153 + 10.0);
        (assign19200_e37155, assign19200_e37153_d_n0, assign19200_e37153_d_n2, assign19200_e37153_d_n3, assign19200_e37153_d_n4, assign19200_e37153_d_n5, assign19200_e37153_d_n6, assign19200_e37153_d_n7, assign19200_e37153_d_n8, assign19200_e37153_d_n9, assign19200_e37153_d_n10, assign19200_e37153_d_n11, assign19200_e37153_d_n13, assign19200_e37153_d_n14,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    }
};
        var_t2 = assign19200_e37157;
        var_t2_dn0 = assign19200_e37157_d_n0;
        var_t2_dn2 = assign19200_e37157_d_n2;
        var_t2_dn3 = assign19200_e37157_d_n3;
        var_t2_dn4 = assign19200_e37157_d_n4;
        var_t2_dn5 = assign19200_e37157_d_n5;
        var_t2_dn6 = assign19200_e37157_d_n6;
        var_t2_dn7 = assign19200_e37157_d_n7;
        var_t2_dn8 = assign19200_e37157_d_n8;
        var_t2_dn9 = assign19200_e37157_d_n9;
        var_t2_dn10 = assign19200_e37157_d_n10;
        var_t2_dn11 = assign19200_e37157_d_n11;
        var_t2_dn13 = assign19200_e37157_d_n13;
        var_t2_dn14 = assign19200_e37157_d_n14;
        var_t2_rv = 0.0;

        let (assign19210_e37193, assign19210_e37193_d_n0, assign19210_e37193_d_n2, assign19210_e37193_d_n3, assign19210_e37193_d_n4, assign19210_e37193_d_n5, assign19210_e37193_d_n6, assign19210_e37193_d_n7, assign19210_e37193_d_n8, assign19210_e37193_d_n9, assign19210_e37193_d_n10, assign19210_e37193_d_n11, assign19210_e37193_d_n13, assign19210_e37193_d_n14,) = {
    if ((var_guard350 != 0.0) && (var_guard352 != 0.0)) {
        let assign19210_e37162: f64 = (-p.p1627);
        let assign19210_e37166: f64 = (var_t2 - 1.0);
        let assign19210_e37168: f64 = (assign19210_e37166 / p.p1629);
        let (assign19210_e37189, assign19210_e37189_d_n0, assign19210_e37189_d_n2, assign19210_e37189_d_n3, assign19210_e37189_d_n4, assign19210_e37189_d_n5, assign19210_e37189_d_n6, assign19210_e37189_d_n7, assign19210_e37189_d_n8, assign19210_e37189_d_n9, assign19210_e37189_d_n10, assign19210_e37189_d_n11, assign19210_e37189_d_n13, assign19210_e37189_d_n14,) = {
            if (!(assign19210_e37168 > 1e-38)) {
                let assign19210_e37173: f64 = (-87.498233534);
                (assign19210_e37173, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign19210_e37176: f64 = (var_t2 - 1.0);
                let assign19210_e37178: f64 = (assign19210_e37176 / p.p1629);
                let (assign19210_e37188, assign19210_e37188_d_n0, assign19210_e37188_d_n2, assign19210_e37188_d_n3, assign19210_e37188_d_n4, assign19210_e37188_d_n5, assign19210_e37188_d_n6, assign19210_e37188_d_n7, assign19210_e37188_d_n8, assign19210_e37188_d_n9, assign19210_e37188_d_n10, assign19210_e37188_d_n11, assign19210_e37188_d_n13, assign19210_e37188_d_n14,) = {
                    if (assign19210_e37178 > 1e-38) {
                        let assign19210_e37183: f64 = (var_t2 - 1.0);
                        let assign19210_e37185: f64 = (assign19210_e37183 / p.p1629);
                        let assign19210_e37186: f64 = (assign19210_e37185).ln();
                        (assign19210_e37186, ((var_t2_dn0 / p.p1629) / assign19210_e37185), ((var_t2_dn2 / p.p1629) / assign19210_e37185), ((var_t2_dn3 / p.p1629) / assign19210_e37185), ((var_t2_dn4 / p.p1629) / assign19210_e37185), ((var_t2_dn5 / p.p1629) / assign19210_e37185), ((var_t2_dn6 / p.p1629) / assign19210_e37185), ((var_t2_dn7 / p.p1629) / assign19210_e37185), ((var_t2_dn8 / p.p1629) / assign19210_e37185), ((var_t2_dn9 / p.p1629) / assign19210_e37185), ((var_t2_dn10 / p.p1629) / assign19210_e37185), ((var_t2_dn11 / p.p1629) / assign19210_e37185), ((var_t2_dn13 / p.p1629) / assign19210_e37185), ((var_t2_dn14 / p.p1629) / assign19210_e37185),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign19210_e37188, assign19210_e37188_d_n0, assign19210_e37188_d_n2, assign19210_e37188_d_n3, assign19210_e37188_d_n4, assign19210_e37188_d_n5, assign19210_e37188_d_n6, assign19210_e37188_d_n7, assign19210_e37188_d_n8, assign19210_e37188_d_n9, assign19210_e37188_d_n10, assign19210_e37188_d_n11, assign19210_e37188_d_n13, assign19210_e37188_d_n14,)
            }
        };
        let assign19210_e37190: f64 = (var_nvtmd * assign19210_e37189);
        let assign19210_e37191: f64 = (assign19210_e37162 - assign19210_e37190);
        (assign19210_e37191, (-(var_nvtmd * assign19210_e37189_d_n0)), (-(var_nvtmd * assign19210_e37189_d_n2)), (-(var_nvtmd * assign19210_e37189_d_n3)), (-((var_nvtmd_dn4 * assign19210_e37189) + (var_nvtmd * assign19210_e37189_d_n4))), (-(var_nvtmd * assign19210_e37189_d_n5)), (-(var_nvtmd * assign19210_e37189_d_n6)), (-(var_nvtmd * assign19210_e37189_d_n7)), (-(var_nvtmd * assign19210_e37189_d_n8)), (-(var_nvtmd * assign19210_e37189_d_n9)), (-(var_nvtmd * assign19210_e37189_d_n10)), (-(var_nvtmd * assign19210_e37189_d_n11)), (-(var_nvtmd * assign19210_e37189_d_n13)), (-(var_nvtmd * assign19210_e37189_d_n14)),)
    } else {
        (var_vjdmrev, var_vjdmrev_dn0, var_vjdmrev_dn2, var_vjdmrev_dn3, var_vjdmrev_dn4, var_vjdmrev_dn5, var_vjdmrev_dn6, var_vjdmrev_dn7, var_vjdmrev_dn8, var_vjdmrev_dn9, var_vjdmrev_dn10, var_vjdmrev_dn11, var_vjdmrev_dn13, var_vjdmrev_dn14,)
    }
};
        var_vjdmrev = assign19210_e37193;
        var_vjdmrev_dn0 = assign19210_e37193_d_n0;
        var_vjdmrev_dn2 = assign19210_e37193_d_n2;
        var_vjdmrev_dn3 = assign19210_e37193_d_n3;
        var_vjdmrev_dn4 = assign19210_e37193_d_n4;
        var_vjdmrev_dn5 = assign19210_e37193_d_n5;
        var_vjdmrev_dn6 = assign19210_e37193_d_n6;
        var_vjdmrev_dn7 = assign19210_e37193_d_n7;
        var_vjdmrev_dn8 = assign19210_e37193_d_n8;
        var_vjdmrev_dn9 = assign19210_e37193_d_n9;
        var_vjdmrev_dn10 = assign19210_e37193_d_n10;
        var_vjdmrev_dn11 = assign19210_e37193_d_n11;
        var_vjdmrev_dn13 = assign19210_e37193_d_n13;
        var_vjdmrev_dn14 = assign19210_e37193_d_n14;
        var_vjdmrev_rv = 0.0;

        *var_guard352_slot = var_guard352;
        *var_guard352_rv_slot = var_guard352_rv;
        *var_isbd_slot = var_isbd;
        *var_isbd_dn0_slot = var_isbd_dn0;
        *var_isbd_dn10_slot = var_isbd_dn10;
        *var_isbd_dn11_slot = var_isbd_dn11;
        *var_isbd_dn13_slot = var_isbd_dn13;
        *var_isbd_dn14_slot = var_isbd_dn14;
        *var_isbd_dn2_slot = var_isbd_dn2;
        *var_isbd_dn3_slot = var_isbd_dn3;
        *var_isbd_dn4_slot = var_isbd_dn4;
        *var_isbd_dn5_slot = var_isbd_dn5;
        *var_isbd_dn6_slot = var_isbd_dn6;
        *var_isbd_dn7_slot = var_isbd_dn7;
        *var_isbd_dn8_slot = var_isbd_dn8;
        *var_isbd_dn9_slot = var_isbd_dn9;
        *var_isbd_rv_slot = var_isbd_rv;
        *var_ivjsmrev_slot = var_ivjsmrev;
        *var_ivjsmrev_dn0_slot = var_ivjsmrev_dn0;
        *var_ivjsmrev_dn10_slot = var_ivjsmrev_dn10;
        *var_ivjsmrev_dn11_slot = var_ivjsmrev_dn11;
        *var_ivjsmrev_dn13_slot = var_ivjsmrev_dn13;
        *var_ivjsmrev_dn14_slot = var_ivjsmrev_dn14;
        *var_ivjsmrev_dn2_slot = var_ivjsmrev_dn2;
        *var_ivjsmrev_dn3_slot = var_ivjsmrev_dn3;
        *var_ivjsmrev_dn4_slot = var_ivjsmrev_dn4;
        *var_ivjsmrev_dn5_slot = var_ivjsmrev_dn5;
        *var_ivjsmrev_dn6_slot = var_ivjsmrev_dn6;
        *var_ivjsmrev_dn7_slot = var_ivjsmrev_dn7;
        *var_ivjsmrev_dn8_slot = var_ivjsmrev_dn8;
        *var_ivjsmrev_dn9_slot = var_ivjsmrev_dn9;
        *var_ivjsmrev_rv_slot = var_ivjsmrev_rv;
        *var_nvtmd_slot = var_nvtmd;
        *var_nvtmd_dn4_slot = var_nvtmd_dn4;
        *var_nvtmd_rv_slot = var_nvtmd_rv;
        *var_sslprev_slot = var_sslprev;
        *var_sslprev_dn0_slot = var_sslprev_dn0;
        *var_sslprev_dn10_slot = var_sslprev_dn10;
        *var_sslprev_dn11_slot = var_sslprev_dn11;
        *var_sslprev_dn13_slot = var_sslprev_dn13;
        *var_sslprev_dn14_slot = var_sslprev_dn14;
        *var_sslprev_dn2_slot = var_sslprev_dn2;
        *var_sslprev_dn3_slot = var_sslprev_dn3;
        *var_sslprev_dn4_slot = var_sslprev_dn4;
        *var_sslprev_dn5_slot = var_sslprev_dn5;
        *var_sslprev_dn6_slot = var_sslprev_dn6;
        *var_sslprev_dn7_slot = var_sslprev_dn7;
        *var_sslprev_dn8_slot = var_sslprev_dn8;
        *var_sslprev_dn9_slot = var_sslprev_dn9;
        *var_sslprev_rv_slot = var_sslprev_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn13_slot = var_t0_dn13;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
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
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
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
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_tb_slot = var_tb;
        *var_tb_dn0_slot = var_tb_dn0;
        *var_tb_dn10_slot = var_tb_dn10;
        *var_tb_dn11_slot = var_tb_dn11;
        *var_tb_dn13_slot = var_tb_dn13;
        *var_tb_dn14_slot = var_tb_dn14;
        *var_tb_dn2_slot = var_tb_dn2;
        *var_tb_dn3_slot = var_tb_dn3;
        *var_tb_dn4_slot = var_tb_dn4;
        *var_tb_dn5_slot = var_tb_dn5;
        *var_tb_dn6_slot = var_tb_dn6;
        *var_tb_dn7_slot = var_tb_dn7;
        *var_tb_dn8_slot = var_tb_dn8;
        *var_tb_dn9_slot = var_tb_dn9;
        *var_tb_rv_slot = var_tb_rv;
        *var_vjdmfwd_slot = var_vjdmfwd;
        *var_vjdmfwd_dn0_slot = var_vjdmfwd_dn0;
        *var_vjdmfwd_dn10_slot = var_vjdmfwd_dn10;
        *var_vjdmfwd_dn11_slot = var_vjdmfwd_dn11;
        *var_vjdmfwd_dn13_slot = var_vjdmfwd_dn13;
        *var_vjdmfwd_dn14_slot = var_vjdmfwd_dn14;
        *var_vjdmfwd_dn2_slot = var_vjdmfwd_dn2;
        *var_vjdmfwd_dn3_slot = var_vjdmfwd_dn3;
        *var_vjdmfwd_dn4_slot = var_vjdmfwd_dn4;
        *var_vjdmfwd_dn5_slot = var_vjdmfwd_dn5;
        *var_vjdmfwd_dn6_slot = var_vjdmfwd_dn6;
        *var_vjdmfwd_dn7_slot = var_vjdmfwd_dn7;
        *var_vjdmfwd_dn8_slot = var_vjdmfwd_dn8;
        *var_vjdmfwd_dn9_slot = var_vjdmfwd_dn9;
        *var_vjdmfwd_rv_slot = var_vjdmfwd_rv;
        *var_vjdmrev_slot = var_vjdmrev;
        *var_vjdmrev_dn0_slot = var_vjdmrev_dn0;
        *var_vjdmrev_dn10_slot = var_vjdmrev_dn10;
        *var_vjdmrev_dn11_slot = var_vjdmrev_dn11;
        *var_vjdmrev_dn13_slot = var_vjdmrev_dn13;
        *var_vjdmrev_dn14_slot = var_vjdmrev_dn14;
        *var_vjdmrev_dn2_slot = var_vjdmrev_dn2;
        *var_vjdmrev_dn3_slot = var_vjdmrev_dn3;
        *var_vjdmrev_dn4_slot = var_vjdmrev_dn4;
        *var_vjdmrev_dn5_slot = var_vjdmrev_dn5;
        *var_vjdmrev_dn6_slot = var_vjdmrev_dn6;
        *var_vjdmrev_dn7_slot = var_vjdmrev_dn7;
        *var_vjdmrev_dn8_slot = var_vjdmrev_dn8;
        *var_vjdmrev_dn9_slot = var_vjdmrev_dn9;
        *var_vjdmrev_rv_slot = var_vjdmrev_rv;
        *var_vjsmrev_slot = var_vjsmrev;
        *var_vjsmrev_dn0_slot = var_vjsmrev_dn0;
        *var_vjsmrev_dn10_slot = var_vjsmrev_dn10;
        *var_vjsmrev_dn11_slot = var_vjsmrev_dn11;
        *var_vjsmrev_dn13_slot = var_vjsmrev_dn13;
        *var_vjsmrev_dn14_slot = var_vjsmrev_dn14;
        *var_vjsmrev_dn2_slot = var_vjsmrev_dn2;
        *var_vjsmrev_dn3_slot = var_vjsmrev_dn3;
        *var_vjsmrev_dn4_slot = var_vjsmrev_dn4;
        *var_vjsmrev_dn5_slot = var_vjsmrev_dn5;
        *var_vjsmrev_dn6_slot = var_vjsmrev_dn6;
        *var_vjsmrev_dn7_slot = var_vjsmrev_dn7;
        *var_vjsmrev_dn8_slot = var_vjsmrev_dn8;
        *var_vjsmrev_dn9_slot = var_vjsmrev_dn9;
        *var_vjsmrev_rv_slot = var_vjsmrev_rv;
        *var_xexpbvd_slot = var_xexpbvd;
        *var_xexpbvd_dn4_slot = var_xexpbvd_dn4;
        *var_xexpbvd_rv_slot = var_xexpbvd_rv;
    }

    pub(super) fn stamp_reactive_block_72(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_cjd_t: f64,
        var_cjd_t_dn4: f64,
        var_cjs_t: f64,
        var_cjs_t_dn4: f64,
        var_cjswd_t: f64,
        var_cjswd_t_dn4: f64,
        var_cjswgd_t: f64,
        var_cjswgd_t_dn4: f64,
        var_cjswgs_t: f64,
        var_cjswgs_t_dn4: f64,
        var_cjsws_t: f64,
        var_cjsws_t_dn4: f64,
        var_devsign: f64,
        var_guard350: f64,
        var_guard352: f64,
        var_isbd: f64,
        var_isbd_dn0: f64,
        var_isbd_dn10: f64,
        var_isbd_dn11: f64,
        var_isbd_dn13: f64,
        var_isbd_dn14: f64,
        var_isbd_dn2: f64,
        var_isbd_dn3: f64,
        var_isbd_dn4: f64,
        var_isbd_dn5: f64,
        var_isbd_dn6: f64,
        var_isbd_dn7: f64,
        var_isbd_dn8: f64,
        var_isbd_dn9: f64,
        var_nfintotal: f64,
        var_nvtmd: f64,
        var_nvtmd_dn4: f64,
        var_pbd_t: f64,
        var_pbd_t_dn4: f64,
        var_pbs_t: f64,
        var_pbs_t_dn4: f64,
        var_pbswd_t: f64,
        var_pbswd_t_dn4: f64,
        var_pbswgd_t: f64,
        var_pbswgd_t_dn4: f64,
        var_pbswgs_t: f64,
        var_pbswgs_t_dn4: f64,
        var_pbsws_t: f64,
        var_pbsws_t_dn4: f64,
        var_vjdmrev: f64,
        var_vjdmrev_dn0: f64,
        var_vjdmrev_dn10: f64,
        var_vjdmrev_dn11: f64,
        var_vjdmrev_dn13: f64,
        var_vjdmrev_dn14: f64,
        var_vjdmrev_dn2: f64,
        var_vjdmrev_dn3: f64,
        var_vjdmrev_dn4: f64,
        var_vjdmrev_dn5: f64,
        var_vjdmrev_dn6: f64,
        var_vjdmrev_dn7: f64,
        var_vjdmrev_dn8: f64,
        var_vjdmrev_dn9: f64,
        var_weff0: f64,
        var_czbd_slot: &mut f64,
        var_czbd_dn4_slot: &mut f64,
        var_czbd_rv_slot: &mut f64,
        var_czbdsw_slot: &mut f64,
        var_czbdsw_dn4_slot: &mut f64,
        var_czbdsw_rv_slot: &mut f64,
        var_czbdswg_slot: &mut f64,
        var_czbdswg_dn4_slot: &mut f64,
        var_czbdswg_rv_slot: &mut f64,
        var_czbs_slot: &mut f64,
        var_czbs_dn4_slot: &mut f64,
        var_czbs_rv_slot: &mut f64,
        var_czbssw_slot: &mut f64,
        var_czbssw_dn4_slot: &mut f64,
        var_czbssw_rv_slot: &mut f64,
        var_czbsswg_slot: &mut f64,
        var_czbsswg_dn4_slot: &mut f64,
        var_czbsswg_rv_slot: &mut f64,
        var_dslprev_slot: &mut f64,
        var_dslprev_dn0_slot: &mut f64,
        var_dslprev_dn10_slot: &mut f64,
        var_dslprev_dn11_slot: &mut f64,
        var_dslprev_dn13_slot: &mut f64,
        var_dslprev_dn14_slot: &mut f64,
        var_dslprev_dn2_slot: &mut f64,
        var_dslprev_dn3_slot: &mut f64,
        var_dslprev_dn4_slot: &mut f64,
        var_dslprev_dn5_slot: &mut f64,
        var_dslprev_dn6_slot: &mut f64,
        var_dslprev_dn7_slot: &mut f64,
        var_dslprev_dn8_slot: &mut f64,
        var_dslprev_dn9_slot: &mut f64,
        var_dslprev_rv_slot: &mut f64,
        var_guard353_slot: &mut f64,
        var_guard353_rv_slot: &mut f64,
        var_guard354_slot: &mut f64,
        var_guard354_rv_slot: &mut f64,
        var_guard355_slot: &mut f64,
        var_guard355_rv_slot: &mut f64,
        var_guard356_slot: &mut f64,
        var_guard356_rv_slot: &mut f64,
        var_guard357_slot: &mut f64,
        var_guard357_rv_slot: &mut f64,
        var_guard358_slot: &mut f64,
        var_guard358_rv_slot: &mut f64,
        var_guard359_slot: &mut f64,
        var_guard359_rv_slot: &mut f64,
        var_guard360_slot: &mut f64,
        var_guard360_rv_slot: &mut f64,
        var_ivjdmrev_slot: &mut f64,
        var_ivjdmrev_dn0_slot: &mut f64,
        var_ivjdmrev_dn10_slot: &mut f64,
        var_ivjdmrev_dn11_slot: &mut f64,
        var_ivjdmrev_dn13_slot: &mut f64,
        var_ivjdmrev_dn14_slot: &mut f64,
        var_ivjdmrev_dn2_slot: &mut f64,
        var_ivjdmrev_dn3_slot: &mut f64,
        var_ivjdmrev_dn4_slot: &mut f64,
        var_ivjdmrev_dn5_slot: &mut f64,
        var_ivjdmrev_dn6_slot: &mut f64,
        var_ivjdmrev_dn7_slot: &mut f64,
        var_ivjdmrev_dn8_slot: &mut f64,
        var_ivjdmrev_dn9_slot: &mut f64,
        var_ivjdmrev_rv_slot: &mut f64,
        var_pb21d_slot: &mut f64,
        var_pb21d_dn4_slot: &mut f64,
        var_pb21d_rv_slot: &mut f64,
        var_pb21s_slot: &mut f64,
        var_pb21s_dn4_slot: &mut f64,
        var_pb21s_rv_slot: &mut f64,
        var_pb22d_slot: &mut f64,
        var_pb22d_dn4_slot: &mut f64,
        var_pb22d_rv_slot: &mut f64,
        var_pb22s_slot: &mut f64,
        var_pb22s_dn4_slot: &mut f64,
        var_pb22s_rv_slot: &mut f64,
        var_pb23d_slot: &mut f64,
        var_pb23d_dn4_slot: &mut f64,
        var_pb23d_rv_slot: &mut f64,
        var_pb23s_slot: &mut f64,
        var_pb23s_dn4_slot: &mut f64,
        var_pb23s_rv_slot: &mut f64,
        var_sigvds_slot: &mut f64,
        var_sigvds_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_vds_noswap_slot: &mut f64,
        var_vds_noswap_dn5_slot: &mut f64,
        var_vds_noswap_dn6_slot: &mut f64,
        var_vds_noswap_rv_slot: &mut f64,
        var_vec1d_slot: &mut f64,
        var_vec1d_dn4_slot: &mut f64,
        var_vec1d_rv_slot: &mut f64,
        var_vec1s_slot: &mut f64,
        var_vec1s_dn4_slot: &mut f64,
        var_vec1s_rv_slot: &mut f64,
        var_vec2d_slot: &mut f64,
        var_vec2d_dn4_slot: &mut f64,
        var_vec2d_rv_slot: &mut f64,
        var_vec2s_slot: &mut f64,
        var_vec2s_dn4_slot: &mut f64,
        var_vec2s_rv_slot: &mut f64,
        var_vec3d_slot: &mut f64,
        var_vec3d_dn4_slot: &mut f64,
        var_vec3d_rv_slot: &mut f64,
        var_vec3s_slot: &mut f64,
        var_vec3s_dn4_slot: &mut f64,
        var_vec3s_rv_slot: &mut f64,
        var_ved_jct_slot: &mut f64,
        var_ved_jct_dn3_slot: &mut f64,
        var_ved_jct_dn5_slot: &mut f64,
        var_ved_jct_rv_slot: &mut f64,
        var_ves_jct_slot: &mut f64,
        var_ves_jct_dn3_slot: &mut f64,
        var_ves_jct_dn6_slot: &mut f64,
        var_ves_jct_rv_slot: &mut f64,
        var_vgd_noswap_slot: &mut f64,
        var_vgd_noswap_dn11_slot: &mut f64,
        var_vgd_noswap_dn5_slot: &mut f64,
        var_vgd_noswap_rv_slot: &mut f64,
        var_vgdrift_slot: &mut f64,
        var_vgdrift_dn10_slot: &mut f64,
        var_vgdrift_dn14_slot: &mut f64,
        var_vgdrift_dn5_slot: &mut f64,
        var_vgdrift_rv_slot: &mut f64,
        var_vgdrift_s_slot: &mut f64,
        var_vgdrift_s_dn10_slot: &mut f64,
        var_vgdrift_s_dn13_slot: &mut f64,
        var_vgdrift_s_dn6_slot: &mut f64,
        var_vgdrift_s_rv_slot: &mut f64,
        var_vge_slot: &mut f64,
        var_vge_dn11_slot: &mut f64,
        var_vge_dn3_slot: &mut f64,
        var_vge_rv_slot: &mut f64,
        var_vgs_noswap_slot: &mut f64,
        var_vgs_noswap_dn11_slot: &mut f64,
        var_vgs_noswap_dn6_slot: &mut f64,
        var_vgs_noswap_rv_slot: &mut f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let mut var_czbd: f64 = *var_czbd_slot;
        let mut var_czbd_dn4: f64 = *var_czbd_dn4_slot;
        let mut var_czbd_rv: f64 = *var_czbd_rv_slot;
        let mut var_czbdsw: f64 = *var_czbdsw_slot;
        let mut var_czbdsw_dn4: f64 = *var_czbdsw_dn4_slot;
        let mut var_czbdsw_rv: f64 = *var_czbdsw_rv_slot;
        let mut var_czbdswg: f64 = *var_czbdswg_slot;
        let mut var_czbdswg_dn4: f64 = *var_czbdswg_dn4_slot;
        let mut var_czbdswg_rv: f64 = *var_czbdswg_rv_slot;
        let mut var_czbs: f64 = *var_czbs_slot;
        let mut var_czbs_dn4: f64 = *var_czbs_dn4_slot;
        let mut var_czbs_rv: f64 = *var_czbs_rv_slot;
        let mut var_czbssw: f64 = *var_czbssw_slot;
        let mut var_czbssw_dn4: f64 = *var_czbssw_dn4_slot;
        let mut var_czbssw_rv: f64 = *var_czbssw_rv_slot;
        let mut var_czbsswg: f64 = *var_czbsswg_slot;
        let mut var_czbsswg_dn4: f64 = *var_czbsswg_dn4_slot;
        let mut var_czbsswg_rv: f64 = *var_czbsswg_rv_slot;
        let mut var_dslprev: f64 = *var_dslprev_slot;
        let mut var_dslprev_dn0: f64 = *var_dslprev_dn0_slot;
        let mut var_dslprev_dn10: f64 = *var_dslprev_dn10_slot;
        let mut var_dslprev_dn11: f64 = *var_dslprev_dn11_slot;
        let mut var_dslprev_dn13: f64 = *var_dslprev_dn13_slot;
        let mut var_dslprev_dn14: f64 = *var_dslprev_dn14_slot;
        let mut var_dslprev_dn2: f64 = *var_dslprev_dn2_slot;
        let mut var_dslprev_dn3: f64 = *var_dslprev_dn3_slot;
        let mut var_dslprev_dn4: f64 = *var_dslprev_dn4_slot;
        let mut var_dslprev_dn5: f64 = *var_dslprev_dn5_slot;
        let mut var_dslprev_dn6: f64 = *var_dslprev_dn6_slot;
        let mut var_dslprev_dn7: f64 = *var_dslprev_dn7_slot;
        let mut var_dslprev_dn8: f64 = *var_dslprev_dn8_slot;
        let mut var_dslprev_dn9: f64 = *var_dslprev_dn9_slot;
        let mut var_dslprev_rv: f64 = *var_dslprev_rv_slot;
        let mut var_guard353: f64 = *var_guard353_slot;
        let mut var_guard353_rv: f64 = *var_guard353_rv_slot;
        let mut var_guard354: f64 = *var_guard354_slot;
        let mut var_guard354_rv: f64 = *var_guard354_rv_slot;
        let mut var_guard355: f64 = *var_guard355_slot;
        let mut var_guard355_rv: f64 = *var_guard355_rv_slot;
        let mut var_guard356: f64 = *var_guard356_slot;
        let mut var_guard356_rv: f64 = *var_guard356_rv_slot;
        let mut var_guard357: f64 = *var_guard357_slot;
        let mut var_guard357_rv: f64 = *var_guard357_rv_slot;
        let mut var_guard358: f64 = *var_guard358_slot;
        let mut var_guard358_rv: f64 = *var_guard358_rv_slot;
        let mut var_guard359: f64 = *var_guard359_slot;
        let mut var_guard359_rv: f64 = *var_guard359_rv_slot;
        let mut var_guard360: f64 = *var_guard360_slot;
        let mut var_guard360_rv: f64 = *var_guard360_rv_slot;
        let mut var_ivjdmrev: f64 = *var_ivjdmrev_slot;
        let mut var_ivjdmrev_dn0: f64 = *var_ivjdmrev_dn0_slot;
        let mut var_ivjdmrev_dn10: f64 = *var_ivjdmrev_dn10_slot;
        let mut var_ivjdmrev_dn11: f64 = *var_ivjdmrev_dn11_slot;
        let mut var_ivjdmrev_dn13: f64 = *var_ivjdmrev_dn13_slot;
        let mut var_ivjdmrev_dn14: f64 = *var_ivjdmrev_dn14_slot;
        let mut var_ivjdmrev_dn2: f64 = *var_ivjdmrev_dn2_slot;
        let mut var_ivjdmrev_dn3: f64 = *var_ivjdmrev_dn3_slot;
        let mut var_ivjdmrev_dn4: f64 = *var_ivjdmrev_dn4_slot;
        let mut var_ivjdmrev_dn5: f64 = *var_ivjdmrev_dn5_slot;
        let mut var_ivjdmrev_dn6: f64 = *var_ivjdmrev_dn6_slot;
        let mut var_ivjdmrev_dn7: f64 = *var_ivjdmrev_dn7_slot;
        let mut var_ivjdmrev_dn8: f64 = *var_ivjdmrev_dn8_slot;
        let mut var_ivjdmrev_dn9: f64 = *var_ivjdmrev_dn9_slot;
        let mut var_ivjdmrev_rv: f64 = *var_ivjdmrev_rv_slot;
        let mut var_pb21d: f64 = *var_pb21d_slot;
        let mut var_pb21d_dn4: f64 = *var_pb21d_dn4_slot;
        let mut var_pb21d_rv: f64 = *var_pb21d_rv_slot;
        let mut var_pb21s: f64 = *var_pb21s_slot;
        let mut var_pb21s_dn4: f64 = *var_pb21s_dn4_slot;
        let mut var_pb21s_rv: f64 = *var_pb21s_rv_slot;
        let mut var_pb22d: f64 = *var_pb22d_slot;
        let mut var_pb22d_dn4: f64 = *var_pb22d_dn4_slot;
        let mut var_pb22d_rv: f64 = *var_pb22d_rv_slot;
        let mut var_pb22s: f64 = *var_pb22s_slot;
        let mut var_pb22s_dn4: f64 = *var_pb22s_dn4_slot;
        let mut var_pb22s_rv: f64 = *var_pb22s_rv_slot;
        let mut var_pb23d: f64 = *var_pb23d_slot;
        let mut var_pb23d_dn4: f64 = *var_pb23d_dn4_slot;
        let mut var_pb23d_rv: f64 = *var_pb23d_rv_slot;
        let mut var_pb23s: f64 = *var_pb23s_slot;
        let mut var_pb23s_dn4: f64 = *var_pb23s_dn4_slot;
        let mut var_pb23s_rv: f64 = *var_pb23s_rv_slot;
        let mut var_sigvds: f64 = *var_sigvds_slot;
        let mut var_sigvds_rv: f64 = *var_sigvds_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_vds_noswap: f64 = *var_vds_noswap_slot;
        let mut var_vds_noswap_dn5: f64 = *var_vds_noswap_dn5_slot;
        let mut var_vds_noswap_dn6: f64 = *var_vds_noswap_dn6_slot;
        let mut var_vds_noswap_rv: f64 = *var_vds_noswap_rv_slot;
        let mut var_vec1d: f64 = *var_vec1d_slot;
        let mut var_vec1d_dn4: f64 = *var_vec1d_dn4_slot;
        let mut var_vec1d_rv: f64 = *var_vec1d_rv_slot;
        let mut var_vec1s: f64 = *var_vec1s_slot;
        let mut var_vec1s_dn4: f64 = *var_vec1s_dn4_slot;
        let mut var_vec1s_rv: f64 = *var_vec1s_rv_slot;
        let mut var_vec2d: f64 = *var_vec2d_slot;
        let mut var_vec2d_dn4: f64 = *var_vec2d_dn4_slot;
        let mut var_vec2d_rv: f64 = *var_vec2d_rv_slot;
        let mut var_vec2s: f64 = *var_vec2s_slot;
        let mut var_vec2s_dn4: f64 = *var_vec2s_dn4_slot;
        let mut var_vec2s_rv: f64 = *var_vec2s_rv_slot;
        let mut var_vec3d: f64 = *var_vec3d_slot;
        let mut var_vec3d_dn4: f64 = *var_vec3d_dn4_slot;
        let mut var_vec3d_rv: f64 = *var_vec3d_rv_slot;
        let mut var_vec3s: f64 = *var_vec3s_slot;
        let mut var_vec3s_dn4: f64 = *var_vec3s_dn4_slot;
        let mut var_vec3s_rv: f64 = *var_vec3s_rv_slot;
        let mut var_ved_jct: f64 = *var_ved_jct_slot;
        let mut var_ved_jct_dn3: f64 = *var_ved_jct_dn3_slot;
        let mut var_ved_jct_dn5: f64 = *var_ved_jct_dn5_slot;
        let mut var_ved_jct_rv: f64 = *var_ved_jct_rv_slot;
        let mut var_ves_jct: f64 = *var_ves_jct_slot;
        let mut var_ves_jct_dn3: f64 = *var_ves_jct_dn3_slot;
        let mut var_ves_jct_dn6: f64 = *var_ves_jct_dn6_slot;
        let mut var_ves_jct_rv: f64 = *var_ves_jct_rv_slot;
        let mut var_vgd_noswap: f64 = *var_vgd_noswap_slot;
        let mut var_vgd_noswap_dn11: f64 = *var_vgd_noswap_dn11_slot;
        let mut var_vgd_noswap_dn5: f64 = *var_vgd_noswap_dn5_slot;
        let mut var_vgd_noswap_rv: f64 = *var_vgd_noswap_rv_slot;
        let mut var_vgdrift: f64 = *var_vgdrift_slot;
        let mut var_vgdrift_dn10: f64 = *var_vgdrift_dn10_slot;
        let mut var_vgdrift_dn14: f64 = *var_vgdrift_dn14_slot;
        let mut var_vgdrift_dn5: f64 = *var_vgdrift_dn5_slot;
        let mut var_vgdrift_rv: f64 = *var_vgdrift_rv_slot;
        let mut var_vgdrift_s: f64 = *var_vgdrift_s_slot;
        let mut var_vgdrift_s_dn10: f64 = *var_vgdrift_s_dn10_slot;
        let mut var_vgdrift_s_dn13: f64 = *var_vgdrift_s_dn13_slot;
        let mut var_vgdrift_s_dn6: f64 = *var_vgdrift_s_dn6_slot;
        let mut var_vgdrift_s_rv: f64 = *var_vgdrift_s_rv_slot;
        let mut var_vge: f64 = *var_vge_slot;
        let mut var_vge_dn11: f64 = *var_vge_dn11_slot;
        let mut var_vge_dn3: f64 = *var_vge_dn3_slot;
        let mut var_vge_rv: f64 = *var_vge_rv_slot;
        let mut var_vgs_noswap: f64 = *var_vgs_noswap_slot;
        let mut var_vgs_noswap_dn11: f64 = *var_vgs_noswap_dn11_slot;
        let mut var_vgs_noswap_dn6: f64 = *var_vgs_noswap_dn6_slot;
        let mut var_vgs_noswap_rv: f64 = *var_vgs_noswap_rv_slot;

        let (assign19220_e37207, assign19220_e37207_d_n0, assign19220_e37207_d_n2, assign19220_e37207_d_n3, assign19220_e37207_d_n4, assign19220_e37207_d_n5, assign19220_e37207_d_n6, assign19220_e37207_d_n7, assign19220_e37207_d_n8, assign19220_e37207_d_n9, assign19220_e37207_d_n10, assign19220_e37207_d_n11, assign19220_e37207_d_n13, assign19220_e37207_d_n14,) = {
    if ((var_guard350 != 0.0) && (var_guard352 != 0.0)) {
        let assign19220_e37200: f64 = (p.p1627 + var_vjdmrev);
        let assign19220_e37201: f64 = (-assign19220_e37200);
        let assign19220_e37203: f64 = (assign19220_e37201 / var_nvtmd);
        let assign19220_e37204: f64 = { let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign19220_e37205: f64 = (p.p1629 * assign19220_e37204);
        (assign19220_e37205, (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjdmrev_dn0) / var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjdmrev_dn2) / var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjdmrev_dn3) / var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((-var_vjdmrev_dn4) * var_nvtmd) - (assign19220_e37201 * var_nvtmd_dn4)) / (var_nvtmd * var_nvtmd)))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjdmrev_dn5) / var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjdmrev_dn6) / var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjdmrev_dn7) / var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjdmrev_dn8) / var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjdmrev_dn9) / var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjdmrev_dn10) / var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjdmrev_dn11) / var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjdmrev_dn13) / var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_vjdmrev_dn14) / var_nvtmd))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn13, var_t1_dn14,)
    }
};
        var_t1 = assign19220_e37207;
        var_t1_dn0 = assign19220_e37207_d_n0;
        var_t1_dn2 = assign19220_e37207_d_n2;
        var_t1_dn3 = assign19220_e37207_d_n3;
        var_t1_dn4 = assign19220_e37207_d_n4;
        var_t1_dn5 = assign19220_e37207_d_n5;
        var_t1_dn6 = assign19220_e37207_d_n6;
        var_t1_dn7 = assign19220_e37207_d_n7;
        var_t1_dn8 = assign19220_e37207_d_n8;
        var_t1_dn9 = assign19220_e37207_d_n9;
        var_t1_dn10 = assign19220_e37207_d_n10;
        var_t1_dn11 = assign19220_e37207_d_n11;
        var_t1_dn13 = assign19220_e37207_d_n13;
        var_t1_dn14 = assign19220_e37207_d_n14;
        var_t1_rv = 0.0;

        let (assign19230_e37217, assign19230_e37217_d_n0, assign19230_e37217_d_n2, assign19230_e37217_d_n3, assign19230_e37217_d_n4, assign19230_e37217_d_n5, assign19230_e37217_d_n6, assign19230_e37217_d_n7, assign19230_e37217_d_n8, assign19230_e37217_d_n9, assign19230_e37217_d_n10, assign19230_e37217_d_n11, assign19230_e37217_d_n13, assign19230_e37217_d_n14,) = {
    if ((var_guard350 != 0.0) && (var_guard352 != 0.0)) {
        let assign19230_e37214: f64 = (1.0 + var_t1);
        let assign19230_e37215: f64 = (var_isbd * assign19230_e37214);
        (assign19230_e37215, ((var_isbd_dn0 * assign19230_e37214) + (var_isbd * var_t1_dn0)), ((var_isbd_dn2 * assign19230_e37214) + (var_isbd * var_t1_dn2)), ((var_isbd_dn3 * assign19230_e37214) + (var_isbd * var_t1_dn3)), ((var_isbd_dn4 * assign19230_e37214) + (var_isbd * var_t1_dn4)), ((var_isbd_dn5 * assign19230_e37214) + (var_isbd * var_t1_dn5)), ((var_isbd_dn6 * assign19230_e37214) + (var_isbd * var_t1_dn6)), ((var_isbd_dn7 * assign19230_e37214) + (var_isbd * var_t1_dn7)), ((var_isbd_dn8 * assign19230_e37214) + (var_isbd * var_t1_dn8)), ((var_isbd_dn9 * assign19230_e37214) + (var_isbd * var_t1_dn9)), ((var_isbd_dn10 * assign19230_e37214) + (var_isbd * var_t1_dn10)), ((var_isbd_dn11 * assign19230_e37214) + (var_isbd * var_t1_dn11)), ((var_isbd_dn13 * assign19230_e37214) + (var_isbd * var_t1_dn13)), ((var_isbd_dn14 * assign19230_e37214) + (var_isbd * var_t1_dn14)),)
    } else {
        (var_ivjdmrev, var_ivjdmrev_dn0, var_ivjdmrev_dn2, var_ivjdmrev_dn3, var_ivjdmrev_dn4, var_ivjdmrev_dn5, var_ivjdmrev_dn6, var_ivjdmrev_dn7, var_ivjdmrev_dn8, var_ivjdmrev_dn9, var_ivjdmrev_dn10, var_ivjdmrev_dn11, var_ivjdmrev_dn13, var_ivjdmrev_dn14,)
    }
};
        var_ivjdmrev = assign19230_e37217;
        var_ivjdmrev_dn0 = assign19230_e37217_d_n0;
        var_ivjdmrev_dn2 = assign19230_e37217_d_n2;
        var_ivjdmrev_dn3 = assign19230_e37217_d_n3;
        var_ivjdmrev_dn4 = assign19230_e37217_d_n4;
        var_ivjdmrev_dn5 = assign19230_e37217_d_n5;
        var_ivjdmrev_dn6 = assign19230_e37217_d_n6;
        var_ivjdmrev_dn7 = assign19230_e37217_d_n7;
        var_ivjdmrev_dn8 = assign19230_e37217_d_n8;
        var_ivjdmrev_dn9 = assign19230_e37217_d_n9;
        var_ivjdmrev_dn10 = assign19230_e37217_d_n10;
        var_ivjdmrev_dn11 = assign19230_e37217_d_n11;
        var_ivjdmrev_dn13 = assign19230_e37217_d_n13;
        var_ivjdmrev_dn14 = assign19230_e37217_d_n14;
        var_ivjdmrev_rv = 0.0;

        let (assign19240_e37228, assign19240_e37228_d_n0, assign19240_e37228_d_n2, assign19240_e37228_d_n3, assign19240_e37228_d_n4, assign19240_e37228_d_n5, assign19240_e37228_d_n6, assign19240_e37228_d_n7, assign19240_e37228_d_n8, assign19240_e37228_d_n9, assign19240_e37228_d_n10, assign19240_e37228_d_n11, assign19240_e37228_d_n13, assign19240_e37228_d_n14,) = {
    if ((var_guard350 != 0.0) && (var_guard352 != 0.0)) {
        let assign19240_e37222: f64 = (-var_isbd);
        let assign19240_e37224: f64 = (assign19240_e37222 * var_t1);
        let assign19240_e37226: f64 = (assign19240_e37224 / var_nvtmd);
        (assign19240_e37226, ((((-var_isbd_dn0) * var_t1) + (assign19240_e37222 * var_t1_dn0)) / var_nvtmd), ((((-var_isbd_dn2) * var_t1) + (assign19240_e37222 * var_t1_dn2)) / var_nvtmd), ((((-var_isbd_dn3) * var_t1) + (assign19240_e37222 * var_t1_dn3)) / var_nvtmd), ((((((-var_isbd_dn4) * var_t1) + (assign19240_e37222 * var_t1_dn4)) * var_nvtmd) - (assign19240_e37224 * var_nvtmd_dn4)) / (var_nvtmd * var_nvtmd)), ((((-var_isbd_dn5) * var_t1) + (assign19240_e37222 * var_t1_dn5)) / var_nvtmd), ((((-var_isbd_dn6) * var_t1) + (assign19240_e37222 * var_t1_dn6)) / var_nvtmd), ((((-var_isbd_dn7) * var_t1) + (assign19240_e37222 * var_t1_dn7)) / var_nvtmd), ((((-var_isbd_dn8) * var_t1) + (assign19240_e37222 * var_t1_dn8)) / var_nvtmd), ((((-var_isbd_dn9) * var_t1) + (assign19240_e37222 * var_t1_dn9)) / var_nvtmd), ((((-var_isbd_dn10) * var_t1) + (assign19240_e37222 * var_t1_dn10)) / var_nvtmd), ((((-var_isbd_dn11) * var_t1) + (assign19240_e37222 * var_t1_dn11)) / var_nvtmd), ((((-var_isbd_dn13) * var_t1) + (assign19240_e37222 * var_t1_dn13)) / var_nvtmd), ((((-var_isbd_dn14) * var_t1) + (assign19240_e37222 * var_t1_dn14)) / var_nvtmd),)
    } else {
        (var_dslprev, var_dslprev_dn0, var_dslprev_dn2, var_dslprev_dn3, var_dslprev_dn4, var_dslprev_dn5, var_dslprev_dn6, var_dslprev_dn7, var_dslprev_dn8, var_dslprev_dn9, var_dslprev_dn10, var_dslprev_dn11, var_dslprev_dn13, var_dslprev_dn14,)
    }
};
        var_dslprev = assign19240_e37228;
        var_dslprev_dn0 = assign19240_e37228_d_n0;
        var_dslprev_dn2 = assign19240_e37228_d_n2;
        var_dslprev_dn3 = assign19240_e37228_d_n3;
        var_dslprev_dn4 = assign19240_e37228_d_n4;
        var_dslprev_dn5 = assign19240_e37228_d_n5;
        var_dslprev_dn6 = assign19240_e37228_d_n6;
        var_dslprev_dn7 = assign19240_e37228_d_n7;
        var_dslprev_dn8 = assign19240_e37228_d_n8;
        var_dslprev_dn9 = assign19240_e37228_d_n9;
        var_dslprev_dn10 = assign19240_e37228_d_n10;
        var_dslprev_dn11 = assign19240_e37228_d_n11;
        var_dslprev_dn13 = assign19240_e37228_d_n13;
        var_dslprev_dn14 = assign19240_e37228_d_n14;
        var_dslprev_rv = 0.0;

        let (assign19250_e37234, assign19250_e37234_d_n4,) = {
    if (var_guard350 != 0.0) {
        let assign19250_e37232: f64 = (var_cjs_t * p.p11);
        (assign19250_e37232, (var_cjs_t_dn4 * p.p11),)
    } else {
        (var_czbs, var_czbs_dn4,)
    }
};
        var_czbs = assign19250_e37234;
        var_czbs_dn4 = assign19250_e37234_d_n4;
        var_czbs_rv = 0.0;

        let (assign19260_e37240, assign19260_e37240_d_n4,) = {
    if (var_guard350 != 0.0) {
        let assign19260_e37238: f64 = (var_cjsws_t * p.p13);
        (assign19260_e37238, (var_cjsws_t_dn4 * p.p13),)
    } else {
        (var_czbssw, var_czbssw_dn4,)
    }
};
        var_czbssw = assign19260_e37240;
        var_czbssw_dn4 = assign19260_e37240_d_n4;
        var_czbssw_rv = 0.0;

        let (assign19270_e37248, assign19270_e37248_d_n4,) = {
    if (var_guard350 != 0.0) {
        let assign19270_e37244: f64 = (var_cjswgs_t * var_weff0);
        let assign19270_e37246: f64 = (assign19270_e37244 * var_nfintotal);
        (assign19270_e37246, ((var_cjswgs_t_dn4 * var_weff0) * var_nfintotal),)
    } else {
        (var_czbsswg, var_czbsswg_dn4,)
    }
};
        var_czbsswg = assign19270_e37248;
        var_czbsswg_dn4 = assign19270_e37248_d_n4;
        var_czbsswg_rv = 0.0;

        let (assign19280_e37254, assign19280_e37254_d_n4,) = {
    if (var_guard350 != 0.0) {
        let assign19280_e37252: f64 = (var_cjd_t * p.p12);
        (assign19280_e37252, (var_cjd_t_dn4 * p.p12),)
    } else {
        (var_czbd, var_czbd_dn4,)
    }
};
        var_czbd = assign19280_e37254;
        var_czbd_dn4 = assign19280_e37254_d_n4;
        var_czbd_rv = 0.0;

        let (assign19290_e37260, assign19290_e37260_d_n4,) = {
    if (var_guard350 != 0.0) {
        let assign19290_e37258: f64 = (var_cjswd_t * p.p14);
        (assign19290_e37258, (var_cjswd_t_dn4 * p.p14),)
    } else {
        (var_czbdsw, var_czbdsw_dn4,)
    }
};
        var_czbdsw = assign19290_e37260;
        var_czbdsw_dn4 = assign19290_e37260_d_n4;
        var_czbdsw_rv = 0.0;

        let (assign19300_e37268, assign19300_e37268_d_n4,) = {
    if (var_guard350 != 0.0) {
        let assign19300_e37264: f64 = (var_cjswgd_t * var_weff0);
        let assign19300_e37266: f64 = (assign19300_e37264 * var_nfintotal);
        (assign19300_e37266, ((var_cjswgd_t_dn4 * var_weff0) * var_nfintotal),)
    } else {
        (var_czbdswg, var_czbdswg_dn4,)
    }
};
        var_czbdswg = assign19300_e37268;
        var_czbdswg_dn4 = assign19300_e37268_d_n4;
        var_czbdswg_rv = 0.0;

        let assign19310_e37271: f64 = if p.p1602 > 0.0 { 1.0 } else { 0.0 };
        var_guard353 = assign19310_e37271;
        var_guard353_rv = 0.0;

        let (assign19320_e37287, assign19320_e37287_d_n4,) = {
    if ((var_guard350 != 0.0) && (var_guard353 != 0.0)) {
        let assign19320_e37279: f64 = (1.0 / p.p1602);
        let assign19320_e37282: f64 = (1.0 / p.p1596);
        let assign19320_e37283: f64 = (assign19320_e37279).powf(assign19320_e37282);
        let assign19320_e37284: f64 = (1.0 - assign19320_e37283);
        let assign19320_e37285: f64 = (var_pbs_t * assign19320_e37284);
        (assign19320_e37285, (var_pbs_t_dn4 * assign19320_e37284),)
    } else {
        (var_vec1s, var_vec1s_dn4,)
    }
};
        var_vec1s = assign19320_e37287;
        var_vec1s_dn4 = assign19320_e37287_d_n4;
        var_vec1s_rv = 0.0;

        let (assign19330_e37310, assign19330_e37310_d_n4,) = {
    if ((var_guard350 != 0.0) && (var_guard353 != 0.0)) {
        let assign19330_e37293: f64 = (var_pbs_t * p.p1602);
        let assign19330_e37295: f64 = (assign19330_e37293 * p.p1608);
        let assign19330_e37297: f64 = (assign19330_e37295 / p.p1596);
        let assign19330_e37301: f64 = (var_vec1s / var_pbs_t);
        let assign19330_e37302: f64 = (1.0 - assign19330_e37301);
        let assign19330_e37305: f64 = (1.0 + p.p1596);
        let assign19330_e37306: f64 = (-assign19330_e37305);
        let assign19330_e37307: f64 = (assign19330_e37302).powf(assign19330_e37306);
        let assign19330_e37308: f64 = (assign19330_e37297 / assign19330_e37307);
        (assign19330_e37308, ((((((var_pbs_t_dn4 * p.p1602) * p.p1608) / p.p1596) * assign19330_e37307) - (assign19330_e37297 * if 0.0 == 0.0 && ((assign19330_e37306) as f64).is_finite() && ((assign19330_e37306) as f64).fract() == 0.0 { if assign19330_e37306 == 0.0 { 0.0 } else { (assign19330_e37306 * ((assign19330_e37302).powf(assign19330_e37306 - 1.0) * (-(((var_vec1s_dn4 * var_pbs_t) - (var_vec1s * var_pbs_t_dn4)) / (var_pbs_t * var_pbs_t))))) } } else { (assign19330_e37307 * (assign19330_e37306 * ((-(((var_vec1s_dn4 * var_pbs_t) - (var_vec1s * var_pbs_t_dn4)) / (var_pbs_t * var_pbs_t))) / assign19330_e37302))) })) / (assign19330_e37307 * assign19330_e37307)),)
    } else {
        (var_pb21s, var_pb21s_dn4,)
    }
};
        var_pb21s = assign19330_e37310;
        var_pb21s_dn4 = assign19330_e37310_d_n4;
        var_pb21s_rv = 0.0;

        let assign19340_e37313: f64 = if p.p1604 > 0.0 { 1.0 } else { 0.0 };
        var_guard354 = assign19340_e37313;
        var_guard354_rv = 0.0;

        let (assign19350_e37329, assign19350_e37329_d_n4,) = {
    if ((var_guard350 != 0.0) && (var_guard354 != 0.0)) {
        let assign19350_e37321: f64 = (1.0 / p.p1604);
        let assign19350_e37324: f64 = (1.0 / p.p1598);
        let assign19350_e37325: f64 = (assign19350_e37321).powf(assign19350_e37324);
        let assign19350_e37326: f64 = (1.0 - assign19350_e37325);
        let assign19350_e37327: f64 = (var_pbsws_t * assign19350_e37326);
        (assign19350_e37327, (var_pbsws_t_dn4 * assign19350_e37326),)
    } else {
        (var_vec2s, var_vec2s_dn4,)
    }
};
        var_vec2s = assign19350_e37329;
        var_vec2s_dn4 = assign19350_e37329_d_n4;
        var_vec2s_rv = 0.0;

        let (assign19360_e37352, assign19360_e37352_d_n4,) = {
    if ((var_guard350 != 0.0) && (var_guard354 != 0.0)) {
        let assign19360_e37335: f64 = (var_pbsws_t * p.p1604);
        let assign19360_e37337: f64 = (assign19360_e37335 * p.p1610);
        let assign19360_e37339: f64 = (assign19360_e37337 / p.p1598);
        let assign19360_e37343: f64 = (var_vec2s / var_pbsws_t);
        let assign19360_e37344: f64 = (1.0 - assign19360_e37343);
        let assign19360_e37347: f64 = (1.0 + p.p1598);
        let assign19360_e37348: f64 = (-assign19360_e37347);
        let assign19360_e37349: f64 = (assign19360_e37344).powf(assign19360_e37348);
        let assign19360_e37350: f64 = (assign19360_e37339 / assign19360_e37349);
        (assign19360_e37350, ((((((var_pbsws_t_dn4 * p.p1604) * p.p1610) / p.p1598) * assign19360_e37349) - (assign19360_e37339 * if 0.0 == 0.0 && ((assign19360_e37348) as f64).is_finite() && ((assign19360_e37348) as f64).fract() == 0.0 { if assign19360_e37348 == 0.0 { 0.0 } else { (assign19360_e37348 * ((assign19360_e37344).powf(assign19360_e37348 - 1.0) * (-(((var_vec2s_dn4 * var_pbsws_t) - (var_vec2s * var_pbsws_t_dn4)) / (var_pbsws_t * var_pbsws_t))))) } } else { (assign19360_e37349 * (assign19360_e37348 * ((-(((var_vec2s_dn4 * var_pbsws_t) - (var_vec2s * var_pbsws_t_dn4)) / (var_pbsws_t * var_pbsws_t))) / assign19360_e37344))) })) / (assign19360_e37349 * assign19360_e37349)),)
    } else {
        (var_pb22s, var_pb22s_dn4,)
    }
};
        var_pb22s = assign19360_e37352;
        var_pb22s_dn4 = assign19360_e37352_d_n4;
        var_pb22s_rv = 0.0;

        let assign19370_e37355: f64 = if p.p1606 > 0.0 { 1.0 } else { 0.0 };
        var_guard355 = assign19370_e37355;
        var_guard355_rv = 0.0;

        let (assign19380_e37371, assign19380_e37371_d_n4,) = {
    if ((var_guard350 != 0.0) && (var_guard355 != 0.0)) {
        let assign19380_e37363: f64 = (1.0 / p.p1606);
        let assign19380_e37366: f64 = (1.0 / p.p1600);
        let assign19380_e37367: f64 = (assign19380_e37363).powf(assign19380_e37366);
        let assign19380_e37368: f64 = (1.0 - assign19380_e37367);
        let assign19380_e37369: f64 = (var_pbswgs_t * assign19380_e37368);
        (assign19380_e37369, (var_pbswgs_t_dn4 * assign19380_e37368),)
    } else {
        (var_vec3s, var_vec3s_dn4,)
    }
};
        var_vec3s = assign19380_e37371;
        var_vec3s_dn4 = assign19380_e37371_d_n4;
        var_vec3s_rv = 0.0;

        let (assign19390_e37394, assign19390_e37394_d_n4,) = {
    if ((var_guard350 != 0.0) && (var_guard355 != 0.0)) {
        let assign19390_e37377: f64 = (var_pbswgs_t * p.p1606);
        let assign19390_e37379: f64 = (assign19390_e37377 * p.p1612);
        let assign19390_e37381: f64 = (assign19390_e37379 / p.p1600);
        let assign19390_e37385: f64 = (var_vec3s / var_pbswgs_t);
        let assign19390_e37386: f64 = (1.0 - assign19390_e37385);
        let assign19390_e37389: f64 = (1.0 + p.p1600);
        let assign19390_e37390: f64 = (-assign19390_e37389);
        let assign19390_e37391: f64 = (assign19390_e37386).powf(assign19390_e37390);
        let assign19390_e37392: f64 = (assign19390_e37381 / assign19390_e37391);
        (assign19390_e37392, ((((((var_pbswgs_t_dn4 * p.p1606) * p.p1612) / p.p1600) * assign19390_e37391) - (assign19390_e37381 * if 0.0 == 0.0 && ((assign19390_e37390) as f64).is_finite() && ((assign19390_e37390) as f64).fract() == 0.0 { if assign19390_e37390 == 0.0 { 0.0 } else { (assign19390_e37390 * ((assign19390_e37386).powf(assign19390_e37390 - 1.0) * (-(((var_vec3s_dn4 * var_pbswgs_t) - (var_vec3s * var_pbswgs_t_dn4)) / (var_pbswgs_t * var_pbswgs_t))))) } } else { (assign19390_e37391 * (assign19390_e37390 * ((-(((var_vec3s_dn4 * var_pbswgs_t) - (var_vec3s * var_pbswgs_t_dn4)) / (var_pbswgs_t * var_pbswgs_t))) / assign19390_e37386))) })) / (assign19390_e37391 * assign19390_e37391)),)
    } else {
        (var_pb23s, var_pb23s_dn4,)
    }
};
        var_pb23s = assign19390_e37394;
        var_pb23s_dn4 = assign19390_e37394_d_n4;
        var_pb23s_rv = 0.0;

        let assign19400_e37397: f64 = if p.p1603 > 0.0 { 1.0 } else { 0.0 };
        var_guard356 = assign19400_e37397;
        var_guard356_rv = 0.0;

        let (assign19410_e37413, assign19410_e37413_d_n4,) = {
    if ((var_guard350 != 0.0) && (var_guard356 != 0.0)) {
        let assign19410_e37405: f64 = (1.0 / p.p1603);
        let assign19410_e37408: f64 = (1.0 / p.p1597);
        let assign19410_e37409: f64 = (assign19410_e37405).powf(assign19410_e37408);
        let assign19410_e37410: f64 = (1.0 - assign19410_e37409);
        let assign19410_e37411: f64 = (var_pbd_t * assign19410_e37410);
        (assign19410_e37411, (var_pbd_t_dn4 * assign19410_e37410),)
    } else {
        (var_vec1d, var_vec1d_dn4,)
    }
};
        var_vec1d = assign19410_e37413;
        var_vec1d_dn4 = assign19410_e37413_d_n4;
        var_vec1d_rv = 0.0;

        let (assign19420_e37436, assign19420_e37436_d_n4,) = {
    if ((var_guard350 != 0.0) && (var_guard356 != 0.0)) {
        let assign19420_e37419: f64 = (var_pbd_t * p.p1603);
        let assign19420_e37421: f64 = (assign19420_e37419 * p.p1609);
        let assign19420_e37423: f64 = (assign19420_e37421 / p.p1597);
        let assign19420_e37427: f64 = (var_vec1d / var_pbd_t);
        let assign19420_e37428: f64 = (1.0 - assign19420_e37427);
        let assign19420_e37431: f64 = (1.0 + p.p1597);
        let assign19420_e37432: f64 = (-assign19420_e37431);
        let assign19420_e37433: f64 = (assign19420_e37428).powf(assign19420_e37432);
        let assign19420_e37434: f64 = (assign19420_e37423 / assign19420_e37433);
        (assign19420_e37434, ((((((var_pbd_t_dn4 * p.p1603) * p.p1609) / p.p1597) * assign19420_e37433) - (assign19420_e37423 * if 0.0 == 0.0 && ((assign19420_e37432) as f64).is_finite() && ((assign19420_e37432) as f64).fract() == 0.0 { if assign19420_e37432 == 0.0 { 0.0 } else { (assign19420_e37432 * ((assign19420_e37428).powf(assign19420_e37432 - 1.0) * (-(((var_vec1d_dn4 * var_pbd_t) - (var_vec1d * var_pbd_t_dn4)) / (var_pbd_t * var_pbd_t))))) } } else { (assign19420_e37433 * (assign19420_e37432 * ((-(((var_vec1d_dn4 * var_pbd_t) - (var_vec1d * var_pbd_t_dn4)) / (var_pbd_t * var_pbd_t))) / assign19420_e37428))) })) / (assign19420_e37433 * assign19420_e37433)),)
    } else {
        (var_pb21d, var_pb21d_dn4,)
    }
};
        var_pb21d = assign19420_e37436;
        var_pb21d_dn4 = assign19420_e37436_d_n4;
        var_pb21d_rv = 0.0;

        let assign19430_e37439: f64 = if p.p1605 > 0.0 { 1.0 } else { 0.0 };
        var_guard357 = assign19430_e37439;
        var_guard357_rv = 0.0;

        let (assign19440_e37455, assign19440_e37455_d_n4,) = {
    if ((var_guard350 != 0.0) && (var_guard357 != 0.0)) {
        let assign19440_e37447: f64 = (1.0 / p.p1605);
        let assign19440_e37450: f64 = (1.0 / p.p1599);
        let assign19440_e37451: f64 = (assign19440_e37447).powf(assign19440_e37450);
        let assign19440_e37452: f64 = (1.0 - assign19440_e37451);
        let assign19440_e37453: f64 = (var_pbswd_t * assign19440_e37452);
        (assign19440_e37453, (var_pbswd_t_dn4 * assign19440_e37452),)
    } else {
        (var_vec2d, var_vec2d_dn4,)
    }
};
        var_vec2d = assign19440_e37455;
        var_vec2d_dn4 = assign19440_e37455_d_n4;
        var_vec2d_rv = 0.0;

        let (assign19450_e37478, assign19450_e37478_d_n4,) = {
    if ((var_guard350 != 0.0) && (var_guard357 != 0.0)) {
        let assign19450_e37461: f64 = (var_pbswd_t * p.p1605);
        let assign19450_e37463: f64 = (assign19450_e37461 * p.p1611);
        let assign19450_e37465: f64 = (assign19450_e37463 / p.p1599);
        let assign19450_e37469: f64 = (var_vec2d / var_pbswd_t);
        let assign19450_e37470: f64 = (1.0 - assign19450_e37469);
        let assign19450_e37473: f64 = (1.0 + p.p1599);
        let assign19450_e37474: f64 = (-assign19450_e37473);
        let assign19450_e37475: f64 = (assign19450_e37470).powf(assign19450_e37474);
        let assign19450_e37476: f64 = (assign19450_e37465 / assign19450_e37475);
        (assign19450_e37476, ((((((var_pbswd_t_dn4 * p.p1605) * p.p1611) / p.p1599) * assign19450_e37475) - (assign19450_e37465 * if 0.0 == 0.0 && ((assign19450_e37474) as f64).is_finite() && ((assign19450_e37474) as f64).fract() == 0.0 { if assign19450_e37474 == 0.0 { 0.0 } else { (assign19450_e37474 * ((assign19450_e37470).powf(assign19450_e37474 - 1.0) * (-(((var_vec2d_dn4 * var_pbswd_t) - (var_vec2d * var_pbswd_t_dn4)) / (var_pbswd_t * var_pbswd_t))))) } } else { (assign19450_e37475 * (assign19450_e37474 * ((-(((var_vec2d_dn4 * var_pbswd_t) - (var_vec2d * var_pbswd_t_dn4)) / (var_pbswd_t * var_pbswd_t))) / assign19450_e37470))) })) / (assign19450_e37475 * assign19450_e37475)),)
    } else {
        (var_pb22d, var_pb22d_dn4,)
    }
};
        var_pb22d = assign19450_e37478;
        var_pb22d_dn4 = assign19450_e37478_d_n4;
        var_pb22d_rv = 0.0;

        let assign19460_e37481: f64 = if p.p1607 > 0.0 { 1.0 } else { 0.0 };
        var_guard358 = assign19460_e37481;
        var_guard358_rv = 0.0;

        let (assign19470_e37497, assign19470_e37497_d_n4,) = {
    if ((var_guard350 != 0.0) && (var_guard358 != 0.0)) {
        let assign19470_e37489: f64 = (1.0 / p.p1607);
        let assign19470_e37492: f64 = (1.0 / p.p1601);
        let assign19470_e37493: f64 = (assign19470_e37489).powf(assign19470_e37492);
        let assign19470_e37494: f64 = (1.0 - assign19470_e37493);
        let assign19470_e37495: f64 = (var_pbswgd_t * assign19470_e37494);
        (assign19470_e37495, (var_pbswgd_t_dn4 * assign19470_e37494),)
    } else {
        (var_vec3d, var_vec3d_dn4,)
    }
};
        var_vec3d = assign19470_e37497;
        var_vec3d_dn4 = assign19470_e37497_d_n4;
        var_vec3d_rv = 0.0;

        let (assign19480_e37520, assign19480_e37520_d_n4,) = {
    if ((var_guard350 != 0.0) && (var_guard358 != 0.0)) {
        let assign19480_e37503: f64 = (var_pbswgd_t * p.p1607);
        let assign19480_e37505: f64 = (assign19480_e37503 * p.p1613);
        let assign19480_e37507: f64 = (assign19480_e37505 / p.p1601);
        let assign19480_e37511: f64 = (var_vec3d / var_pbswgd_t);
        let assign19480_e37512: f64 = (1.0 - assign19480_e37511);
        let assign19480_e37515: f64 = (1.0 + p.p1601);
        let assign19480_e37516: f64 = (-assign19480_e37515);
        let assign19480_e37517: f64 = (assign19480_e37512).powf(assign19480_e37516);
        let assign19480_e37518: f64 = (assign19480_e37507 / assign19480_e37517);
        (assign19480_e37518, ((((((var_pbswgd_t_dn4 * p.p1607) * p.p1613) / p.p1601) * assign19480_e37517) - (assign19480_e37507 * if 0.0 == 0.0 && ((assign19480_e37516) as f64).is_finite() && ((assign19480_e37516) as f64).fract() == 0.0 { if assign19480_e37516 == 0.0 { 0.0 } else { (assign19480_e37516 * ((assign19480_e37512).powf(assign19480_e37516 - 1.0) * (-(((var_vec3d_dn4 * var_pbswgd_t) - (var_vec3d * var_pbswgd_t_dn4)) / (var_pbswgd_t * var_pbswgd_t))))) } } else { (assign19480_e37517 * (assign19480_e37516 * ((-(((var_vec3d_dn4 * var_pbswgd_t) - (var_vec3d * var_pbswgd_t_dn4)) / (var_pbswgd_t * var_pbswgd_t))) / assign19480_e37512))) })) / (assign19480_e37517 * assign19480_e37517)),)
    } else {
        (var_pb23d, var_pb23d_dn4,)
    }
};
        var_pb23d = assign19480_e37520;
        var_pb23d_dn4 = assign19480_e37520_d_n4;
        var_pb23d_rv = 0.0;

        let assign19500_e37531: f64 = (var_devsign * (nv11 - nv6));
        var_vgs_noswap = assign19500_e37531;
        var_vgs_noswap_dn6 = (-var_devsign);
        var_vgs_noswap_dn11 = var_devsign;
        var_vgs_noswap_rv = 0.0;

        let assign19510_e37534: f64 = (var_devsign * (nv5 - nv6));
        var_vds_noswap = assign19510_e37534;
        var_vds_noswap_dn5 = var_devsign;
        var_vds_noswap_dn6 = (-var_devsign);
        var_vds_noswap_rv = 0.0;

        let assign19520_e37537: f64 = (var_devsign * (nv11 - nv5));
        var_vgd_noswap = assign19520_e37537;
        var_vgd_noswap_dn5 = (-var_devsign);
        var_vgd_noswap_dn11 = var_devsign;
        var_vgd_noswap_rv = 0.0;

        let assign19530_e37540: f64 = (var_devsign * (nv3 - nv6));
        var_ves_jct = assign19530_e37540;
        var_ves_jct_dn3 = var_devsign;
        var_ves_jct_dn6 = (-var_devsign);
        var_ves_jct_rv = 0.0;

        let assign19540_e37543: f64 = (var_devsign * (nv3 - nv5));
        var_ved_jct = assign19540_e37543;
        var_ved_jct_dn3 = var_devsign;
        var_ved_jct_dn5 = (-var_devsign);
        var_ved_jct_rv = 0.0;

        let assign19550_e37546: f64 = (var_devsign * (nv11 - nv3));
        var_vge = assign19550_e37546;
        var_vge_dn3 = (-var_devsign);
        var_vge_dn11 = var_devsign;
        var_vge_rv = 0.0;

        let assign19560_e37549: f64 = if p.p76 != 2.0 { 1.0 } else { 0.0 };
        var_guard359 = assign19560_e37549;
        var_guard359_rv = 0.0;

        let (assign19570_e37555, assign19570_e37555_d_n5, assign19570_e37555_d_n10, assign19570_e37555_d_n14,) = {
    if (var_guard359 != 0.0) {
        let assign19570_e37553: f64 = (var_devsign * (nv10 - nv5));
        (assign19570_e37553, (-var_devsign), var_devsign, 0.0,)
    } else {
        (var_vgdrift, var_vgdrift_dn5, var_vgdrift_dn10, var_vgdrift_dn14,)
    }
};
        var_vgdrift = assign19570_e37555;
        var_vgdrift_dn5 = assign19570_e37555_d_n5;
        var_vgdrift_dn10 = assign19570_e37555_d_n10;
        var_vgdrift_dn14 = assign19570_e37555_d_n14;
        var_vgdrift_rv = 0.0;

        let (assign19580_e37561, assign19580_e37561_d_n6, assign19580_e37561_d_n10, assign19580_e37561_d_n13,) = {
    if (var_guard359 != 0.0) {
        let assign19580_e37559: f64 = (var_devsign * (nv10 - nv6));
        (assign19580_e37559, (-var_devsign), var_devsign, 0.0,)
    } else {
        (var_vgdrift_s, var_vgdrift_s_dn6, var_vgdrift_s_dn10, var_vgdrift_s_dn13,)
    }
};
        var_vgdrift_s = assign19580_e37561;
        var_vgdrift_s_dn6 = assign19580_e37561_d_n6;
        var_vgdrift_s_dn10 = assign19580_e37561_d_n10;
        var_vgdrift_s_dn13 = assign19580_e37561_d_n13;
        var_vgdrift_s_rv = 0.0;

        let (assign19590_e37568, assign19590_e37568_d_n5, assign19590_e37568_d_n10, assign19590_e37568_d_n14,) = {
    if (var_guard359 == 0.0) {
        let assign19590_e37566: f64 = (var_devsign * (nv14 - nv5));
        (assign19590_e37566, (-var_devsign), 0.0, var_devsign,)
    } else {
        (var_vgdrift, var_vgdrift_dn5, var_vgdrift_dn10, var_vgdrift_dn14,)
    }
};
        var_vgdrift = assign19590_e37568;
        var_vgdrift_dn5 = assign19590_e37568_d_n5;
        var_vgdrift_dn10 = assign19590_e37568_d_n10;
        var_vgdrift_dn14 = assign19590_e37568_d_n14;
        var_vgdrift_rv = 0.0;

        let (assign19600_e37575, assign19600_e37575_d_n6, assign19600_e37575_d_n10, assign19600_e37575_d_n13,) = {
    if (var_guard359 == 0.0) {
        let assign19600_e37573: f64 = (var_devsign * (nv13 - nv6));
        (assign19600_e37573, (-var_devsign), 0.0, var_devsign,)
    } else {
        (var_vgdrift_s, var_vgdrift_s_dn6, var_vgdrift_s_dn10, var_vgdrift_s_dn13,)
    }
};
        var_vgdrift_s = assign19600_e37575;
        var_vgdrift_s_dn6 = assign19600_e37575_d_n6;
        var_vgdrift_s_dn10 = assign19600_e37575_d_n10;
        var_vgdrift_s_dn13 = assign19600_e37575_d_n13;
        var_vgdrift_s_rv = 0.0;

        var_sigvds = 1.0;
        var_sigvds_rv = 0.0;

        let assign19620_e37579: f64 = if var_vds_noswap < 0.0 { 1.0 } else { 0.0 };
        var_guard360 = assign19620_e37579;
        var_guard360_rv = 0.0;

        let (assign19630_e37584,) = {
    if (var_guard360 != 0.0) {
        let assign19630_e37582: f64 = (-1.0);
        (assign19630_e37582,)
    } else {
        (var_sigvds,)
    }
};
        var_sigvds = assign19630_e37584;
        var_sigvds_rv = 0.0;

        *var_czbd_slot = var_czbd;
        *var_czbd_dn4_slot = var_czbd_dn4;
        *var_czbd_rv_slot = var_czbd_rv;
        *var_czbdsw_slot = var_czbdsw;
        *var_czbdsw_dn4_slot = var_czbdsw_dn4;
        *var_czbdsw_rv_slot = var_czbdsw_rv;
        *var_czbdswg_slot = var_czbdswg;
        *var_czbdswg_dn4_slot = var_czbdswg_dn4;
        *var_czbdswg_rv_slot = var_czbdswg_rv;
        *var_czbs_slot = var_czbs;
        *var_czbs_dn4_slot = var_czbs_dn4;
        *var_czbs_rv_slot = var_czbs_rv;
        *var_czbssw_slot = var_czbssw;
        *var_czbssw_dn4_slot = var_czbssw_dn4;
        *var_czbssw_rv_slot = var_czbssw_rv;
        *var_czbsswg_slot = var_czbsswg;
        *var_czbsswg_dn4_slot = var_czbsswg_dn4;
        *var_czbsswg_rv_slot = var_czbsswg_rv;
        *var_dslprev_slot = var_dslprev;
        *var_dslprev_dn0_slot = var_dslprev_dn0;
        *var_dslprev_dn10_slot = var_dslprev_dn10;
        *var_dslprev_dn11_slot = var_dslprev_dn11;
        *var_dslprev_dn13_slot = var_dslprev_dn13;
        *var_dslprev_dn14_slot = var_dslprev_dn14;
        *var_dslprev_dn2_slot = var_dslprev_dn2;
        *var_dslprev_dn3_slot = var_dslprev_dn3;
        *var_dslprev_dn4_slot = var_dslprev_dn4;
        *var_dslprev_dn5_slot = var_dslprev_dn5;
        *var_dslprev_dn6_slot = var_dslprev_dn6;
        *var_dslprev_dn7_slot = var_dslprev_dn7;
        *var_dslprev_dn8_slot = var_dslprev_dn8;
        *var_dslprev_dn9_slot = var_dslprev_dn9;
        *var_dslprev_rv_slot = var_dslprev_rv;
        *var_guard353_slot = var_guard353;
        *var_guard353_rv_slot = var_guard353_rv;
        *var_guard354_slot = var_guard354;
        *var_guard354_rv_slot = var_guard354_rv;
        *var_guard355_slot = var_guard355;
        *var_guard355_rv_slot = var_guard355_rv;
        *var_guard356_slot = var_guard356;
        *var_guard356_rv_slot = var_guard356_rv;
        *var_guard357_slot = var_guard357;
        *var_guard357_rv_slot = var_guard357_rv;
        *var_guard358_slot = var_guard358;
        *var_guard358_rv_slot = var_guard358_rv;
        *var_guard359_slot = var_guard359;
        *var_guard359_rv_slot = var_guard359_rv;
        *var_guard360_slot = var_guard360;
        *var_guard360_rv_slot = var_guard360_rv;
        *var_ivjdmrev_slot = var_ivjdmrev;
        *var_ivjdmrev_dn0_slot = var_ivjdmrev_dn0;
        *var_ivjdmrev_dn10_slot = var_ivjdmrev_dn10;
        *var_ivjdmrev_dn11_slot = var_ivjdmrev_dn11;
        *var_ivjdmrev_dn13_slot = var_ivjdmrev_dn13;
        *var_ivjdmrev_dn14_slot = var_ivjdmrev_dn14;
        *var_ivjdmrev_dn2_slot = var_ivjdmrev_dn2;
        *var_ivjdmrev_dn3_slot = var_ivjdmrev_dn3;
        *var_ivjdmrev_dn4_slot = var_ivjdmrev_dn4;
        *var_ivjdmrev_dn5_slot = var_ivjdmrev_dn5;
        *var_ivjdmrev_dn6_slot = var_ivjdmrev_dn6;
        *var_ivjdmrev_dn7_slot = var_ivjdmrev_dn7;
        *var_ivjdmrev_dn8_slot = var_ivjdmrev_dn8;
        *var_ivjdmrev_dn9_slot = var_ivjdmrev_dn9;
        *var_ivjdmrev_rv_slot = var_ivjdmrev_rv;
        *var_pb21d_slot = var_pb21d;
        *var_pb21d_dn4_slot = var_pb21d_dn4;
        *var_pb21d_rv_slot = var_pb21d_rv;
        *var_pb21s_slot = var_pb21s;
        *var_pb21s_dn4_slot = var_pb21s_dn4;
        *var_pb21s_rv_slot = var_pb21s_rv;
        *var_pb22d_slot = var_pb22d;
        *var_pb22d_dn4_slot = var_pb22d_dn4;
        *var_pb22d_rv_slot = var_pb22d_rv;
        *var_pb22s_slot = var_pb22s;
        *var_pb22s_dn4_slot = var_pb22s_dn4;
        *var_pb22s_rv_slot = var_pb22s_rv;
        *var_pb23d_slot = var_pb23d;
        *var_pb23d_dn4_slot = var_pb23d_dn4;
        *var_pb23d_rv_slot = var_pb23d_rv;
        *var_pb23s_slot = var_pb23s;
        *var_pb23s_dn4_slot = var_pb23s_dn4;
        *var_pb23s_rv_slot = var_pb23s_rv;
        *var_sigvds_slot = var_sigvds;
        *var_sigvds_rv_slot = var_sigvds_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_vds_noswap_slot = var_vds_noswap;
        *var_vds_noswap_dn5_slot = var_vds_noswap_dn5;
        *var_vds_noswap_dn6_slot = var_vds_noswap_dn6;
        *var_vds_noswap_rv_slot = var_vds_noswap_rv;
        *var_vec1d_slot = var_vec1d;
        *var_vec1d_dn4_slot = var_vec1d_dn4;
        *var_vec1d_rv_slot = var_vec1d_rv;
        *var_vec1s_slot = var_vec1s;
        *var_vec1s_dn4_slot = var_vec1s_dn4;
        *var_vec1s_rv_slot = var_vec1s_rv;
        *var_vec2d_slot = var_vec2d;
        *var_vec2d_dn4_slot = var_vec2d_dn4;
        *var_vec2d_rv_slot = var_vec2d_rv;
        *var_vec2s_slot = var_vec2s;
        *var_vec2s_dn4_slot = var_vec2s_dn4;
        *var_vec2s_rv_slot = var_vec2s_rv;
        *var_vec3d_slot = var_vec3d;
        *var_vec3d_dn4_slot = var_vec3d_dn4;
        *var_vec3d_rv_slot = var_vec3d_rv;
        *var_vec3s_slot = var_vec3s;
        *var_vec3s_dn4_slot = var_vec3s_dn4;
        *var_vec3s_rv_slot = var_vec3s_rv;
        *var_ved_jct_slot = var_ved_jct;
        *var_ved_jct_dn3_slot = var_ved_jct_dn3;
        *var_ved_jct_dn5_slot = var_ved_jct_dn5;
        *var_ved_jct_rv_slot = var_ved_jct_rv;
        *var_ves_jct_slot = var_ves_jct;
        *var_ves_jct_dn3_slot = var_ves_jct_dn3;
        *var_ves_jct_dn6_slot = var_ves_jct_dn6;
        *var_ves_jct_rv_slot = var_ves_jct_rv;
        *var_vgd_noswap_slot = var_vgd_noswap;
        *var_vgd_noswap_dn11_slot = var_vgd_noswap_dn11;
        *var_vgd_noswap_dn5_slot = var_vgd_noswap_dn5;
        *var_vgd_noswap_rv_slot = var_vgd_noswap_rv;
        *var_vgdrift_slot = var_vgdrift;
        *var_vgdrift_dn10_slot = var_vgdrift_dn10;
        *var_vgdrift_dn14_slot = var_vgdrift_dn14;
        *var_vgdrift_dn5_slot = var_vgdrift_dn5;
        *var_vgdrift_rv_slot = var_vgdrift_rv;
        *var_vgdrift_s_slot = var_vgdrift_s;
        *var_vgdrift_s_dn10_slot = var_vgdrift_s_dn10;
        *var_vgdrift_s_dn13_slot = var_vgdrift_s_dn13;
        *var_vgdrift_s_dn6_slot = var_vgdrift_s_dn6;
        *var_vgdrift_s_rv_slot = var_vgdrift_s_rv;
        *var_vge_slot = var_vge;
        *var_vge_dn11_slot = var_vge_dn11;
        *var_vge_dn3_slot = var_vge_dn3;
        *var_vge_rv_slot = var_vge_rv;
        *var_vgs_noswap_slot = var_vgs_noswap;
        *var_vgs_noswap_dn11_slot = var_vgs_noswap_dn11;
        *var_vgs_noswap_dn6_slot = var_vgs_noswap_dn6;
        *var_vgs_noswap_rv_slot = var_vgs_noswap_rv;
    }
}
