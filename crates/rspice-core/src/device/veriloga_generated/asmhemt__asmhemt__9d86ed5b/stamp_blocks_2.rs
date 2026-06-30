#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_tdev: f64,
        var_tdev_db0: f64,
        var_tdev_db1: f64,
        var_tdev_db10: f64,
        var_tdev_db11: f64,
        var_tdev_db12: f64,
        var_tdev_db13: f64,
        var_tdev_db14: f64,
        var_tdev_db15: f64,
        var_tdev_db16: f64,
        var_tdev_db17: f64,
        var_tdev_db18: f64,
        var_tdev_db19: f64,
        var_tdev_db2: f64,
        var_tdev_db20: f64,
        var_tdev_db21: f64,
        var_tdev_db22: f64,
        var_tdev_db23: f64,
        var_tdev_db24: f64,
        var_tdev_db25: f64,
        var_tdev_db26: f64,
        var_tdev_db27: f64,
        var_tdev_db28: f64,
        var_tdev_db29: f64,
        var_tdev_db3: f64,
        var_tdev_db30: f64,
        var_tdev_db31: f64,
        var_tdev_db32: f64,
        var_tdev_db33: f64,
        var_tdev_db34: f64,
        var_tdev_db35: f64,
        var_tdev_db36: f64,
        var_tdev_db37: f64,
        var_tdev_db38: f64,
        var_tdev_db39: f64,
        var_tdev_db4: f64,
        var_tdev_db40: f64,
        var_tdev_db41: f64,
        var_tdev_db42: f64,
        var_tdev_db43: f64,
        var_tdev_db44: f64,
        var_tdev_db45: f64,
        var_tdev_db46: f64,
        var_tdev_db47: f64,
        var_tdev_db48: f64,
        var_tdev_db49: f64,
        var_tdev_db5: f64,
        var_tdev_db50: f64,
        var_tdev_db51: f64,
        var_tdev_db52: f64,
        var_tdev_db53: f64,
        var_tdev_db54: f64,
        var_tdev_db6: f64,
        var_tdev_db7: f64,
        var_tdev_db8: f64,
        var_tdev_db9: f64,
        var_tdev_dn0: f64,
        var_tdev_dn1: f64,
        var_tdev_dn10: f64,
        var_tdev_dn11: f64,
        var_tdev_dn12: f64,
        var_tdev_dn13: f64,
        var_tdev_dn14: f64,
        var_tdev_dn15: f64,
        var_tdev_dn16: f64,
        var_tdev_dn17: f64,
        var_tdev_dn18: f64,
        var_tdev_dn19: f64,
        var_tdev_dn2: f64,
        var_tdev_dn20: f64,
        var_tdev_dn21: f64,
        var_tdev_dn22: f64,
        var_tdev_dn3: f64,
        var_tdev_dn4: f64,
        var_tdev_dn5: f64,
        var_tdev_dn6: f64,
        var_tdev_dn7: f64,
        var_tdev_dn8: f64,
        var_tdev_dn9: f64,
        var_tnom: f64,
        var_guard535_slot: &mut f64,
        var_guard576_slot: &mut f64,
        var_qfr_slot: &mut f64,
        var_qfr3_slot: &mut f64,
        var_qfr3_db0_slot: &mut f64,
        var_qfr3_db1_slot: &mut f64,
        var_qfr3_db10_slot: &mut f64,
        var_qfr3_db11_slot: &mut f64,
        var_qfr3_db12_slot: &mut f64,
        var_qfr3_db13_slot: &mut f64,
        var_qfr3_db14_slot: &mut f64,
        var_qfr3_db15_slot: &mut f64,
        var_qfr3_db16_slot: &mut f64,
        var_qfr3_db17_slot: &mut f64,
        var_qfr3_db18_slot: &mut f64,
        var_qfr3_db19_slot: &mut f64,
        var_qfr3_db2_slot: &mut f64,
        var_qfr3_db20_slot: &mut f64,
        var_qfr3_db21_slot: &mut f64,
        var_qfr3_db22_slot: &mut f64,
        var_qfr3_db23_slot: &mut f64,
        var_qfr3_db24_slot: &mut f64,
        var_qfr3_db25_slot: &mut f64,
        var_qfr3_db26_slot: &mut f64,
        var_qfr3_db27_slot: &mut f64,
        var_qfr3_db28_slot: &mut f64,
        var_qfr3_db29_slot: &mut f64,
        var_qfr3_db3_slot: &mut f64,
        var_qfr3_db30_slot: &mut f64,
        var_qfr3_db31_slot: &mut f64,
        var_qfr3_db32_slot: &mut f64,
        var_qfr3_db33_slot: &mut f64,
        var_qfr3_db34_slot: &mut f64,
        var_qfr3_db35_slot: &mut f64,
        var_qfr3_db36_slot: &mut f64,
        var_qfr3_db37_slot: &mut f64,
        var_qfr3_db38_slot: &mut f64,
        var_qfr3_db39_slot: &mut f64,
        var_qfr3_db4_slot: &mut f64,
        var_qfr3_db40_slot: &mut f64,
        var_qfr3_db41_slot: &mut f64,
        var_qfr3_db42_slot: &mut f64,
        var_qfr3_db43_slot: &mut f64,
        var_qfr3_db44_slot: &mut f64,
        var_qfr3_db45_slot: &mut f64,
        var_qfr3_db46_slot: &mut f64,
        var_qfr3_db47_slot: &mut f64,
        var_qfr3_db48_slot: &mut f64,
        var_qfr3_db49_slot: &mut f64,
        var_qfr3_db5_slot: &mut f64,
        var_qfr3_db50_slot: &mut f64,
        var_qfr3_db51_slot: &mut f64,
        var_qfr3_db52_slot: &mut f64,
        var_qfr3_db53_slot: &mut f64,
        var_qfr3_db54_slot: &mut f64,
        var_qfr3_db6_slot: &mut f64,
        var_qfr3_db7_slot: &mut f64,
        var_qfr3_db8_slot: &mut f64,
        var_qfr3_db9_slot: &mut f64,
        var_qfr3_dn0_slot: &mut f64,
        var_qfr3_dn1_slot: &mut f64,
        var_qfr3_dn10_slot: &mut f64,
        var_qfr3_dn11_slot: &mut f64,
        var_qfr3_dn12_slot: &mut f64,
        var_qfr3_dn13_slot: &mut f64,
        var_qfr3_dn14_slot: &mut f64,
        var_qfr3_dn15_slot: &mut f64,
        var_qfr3_dn16_slot: &mut f64,
        var_qfr3_dn17_slot: &mut f64,
        var_qfr3_dn18_slot: &mut f64,
        var_qfr3_dn19_slot: &mut f64,
        var_qfr3_dn2_slot: &mut f64,
        var_qfr3_dn20_slot: &mut f64,
        var_qfr3_dn21_slot: &mut f64,
        var_qfr3_dn22_slot: &mut f64,
        var_qfr3_dn3_slot: &mut f64,
        var_qfr3_dn4_slot: &mut f64,
        var_qfr3_dn5_slot: &mut f64,
        var_qfr3_dn6_slot: &mut f64,
        var_qfr3_dn7_slot: &mut f64,
        var_qfr3_dn8_slot: &mut f64,
        var_qfr3_dn9_slot: &mut f64,
        var_qfr_db0_slot: &mut f64,
        var_qfr_db1_slot: &mut f64,
        var_qfr_db10_slot: &mut f64,
        var_qfr_db11_slot: &mut f64,
        var_qfr_db12_slot: &mut f64,
        var_qfr_db13_slot: &mut f64,
        var_qfr_db14_slot: &mut f64,
        var_qfr_db15_slot: &mut f64,
        var_qfr_db16_slot: &mut f64,
        var_qfr_db17_slot: &mut f64,
        var_qfr_db18_slot: &mut f64,
        var_qfr_db19_slot: &mut f64,
        var_qfr_db2_slot: &mut f64,
        var_qfr_db20_slot: &mut f64,
        var_qfr_db21_slot: &mut f64,
        var_qfr_db22_slot: &mut f64,
        var_qfr_db23_slot: &mut f64,
        var_qfr_db24_slot: &mut f64,
        var_qfr_db25_slot: &mut f64,
        var_qfr_db26_slot: &mut f64,
        var_qfr_db27_slot: &mut f64,
        var_qfr_db28_slot: &mut f64,
        var_qfr_db29_slot: &mut f64,
        var_qfr_db3_slot: &mut f64,
        var_qfr_db30_slot: &mut f64,
        var_qfr_db31_slot: &mut f64,
        var_qfr_db32_slot: &mut f64,
        var_qfr_db33_slot: &mut f64,
        var_qfr_db34_slot: &mut f64,
        var_qfr_db35_slot: &mut f64,
        var_qfr_db36_slot: &mut f64,
        var_qfr_db37_slot: &mut f64,
        var_qfr_db38_slot: &mut f64,
        var_qfr_db39_slot: &mut f64,
        var_qfr_db4_slot: &mut f64,
        var_qfr_db40_slot: &mut f64,
        var_qfr_db41_slot: &mut f64,
        var_qfr_db42_slot: &mut f64,
        var_qfr_db43_slot: &mut f64,
        var_qfr_db44_slot: &mut f64,
        var_qfr_db45_slot: &mut f64,
        var_qfr_db46_slot: &mut f64,
        var_qfr_db47_slot: &mut f64,
        var_qfr_db48_slot: &mut f64,
        var_qfr_db49_slot: &mut f64,
        var_qfr_db5_slot: &mut f64,
        var_qfr_db50_slot: &mut f64,
        var_qfr_db51_slot: &mut f64,
        var_qfr_db52_slot: &mut f64,
        var_qfr_db53_slot: &mut f64,
        var_qfr_db54_slot: &mut f64,
        var_qfr_db6_slot: &mut f64,
        var_qfr_db7_slot: &mut f64,
        var_qfr_db8_slot: &mut f64,
        var_qfr_db9_slot: &mut f64,
        var_qfr_dn0_slot: &mut f64,
        var_qfr_dn1_slot: &mut f64,
        var_qfr_dn10_slot: &mut f64,
        var_qfr_dn11_slot: &mut f64,
        var_qfr_dn12_slot: &mut f64,
        var_qfr_dn13_slot: &mut f64,
        var_qfr_dn14_slot: &mut f64,
        var_qfr_dn15_slot: &mut f64,
        var_qfr_dn16_slot: &mut f64,
        var_qfr_dn17_slot: &mut f64,
        var_qfr_dn18_slot: &mut f64,
        var_qfr_dn19_slot: &mut f64,
        var_qfr_dn2_slot: &mut f64,
        var_qfr_dn20_slot: &mut f64,
        var_qfr_dn21_slot: &mut f64,
        var_qfr_dn22_slot: &mut f64,
        var_qfr_dn3_slot: &mut f64,
        var_qfr_dn4_slot: &mut f64,
        var_qfr_dn5_slot: &mut f64,
        var_qfr_dn6_slot: &mut f64,
        var_qfr_dn7_slot: &mut f64,
        var_qfr_dn8_slot: &mut f64,
        var_qfr_dn9_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let mut var_guard535: f64 = *var_guard535_slot;
        let mut var_guard576: f64 = *var_guard576_slot;
        let mut var_qfr: f64 = *var_qfr_slot;
        let mut var_qfr3: f64 = *var_qfr3_slot;
        let mut var_qfr3_db0: f64 = *var_qfr3_db0_slot;
        let mut var_qfr3_db1: f64 = *var_qfr3_db1_slot;
        let mut var_qfr3_db10: f64 = *var_qfr3_db10_slot;
        let mut var_qfr3_db11: f64 = *var_qfr3_db11_slot;
        let mut var_qfr3_db12: f64 = *var_qfr3_db12_slot;
        let mut var_qfr3_db13: f64 = *var_qfr3_db13_slot;
        let mut var_qfr3_db14: f64 = *var_qfr3_db14_slot;
        let mut var_qfr3_db15: f64 = *var_qfr3_db15_slot;
        let mut var_qfr3_db16: f64 = *var_qfr3_db16_slot;
        let mut var_qfr3_db17: f64 = *var_qfr3_db17_slot;
        let mut var_qfr3_db18: f64 = *var_qfr3_db18_slot;
        let mut var_qfr3_db19: f64 = *var_qfr3_db19_slot;
        let mut var_qfr3_db2: f64 = *var_qfr3_db2_slot;
        let mut var_qfr3_db20: f64 = *var_qfr3_db20_slot;
        let mut var_qfr3_db21: f64 = *var_qfr3_db21_slot;
        let mut var_qfr3_db22: f64 = *var_qfr3_db22_slot;
        let mut var_qfr3_db23: f64 = *var_qfr3_db23_slot;
        let mut var_qfr3_db24: f64 = *var_qfr3_db24_slot;
        let mut var_qfr3_db25: f64 = *var_qfr3_db25_slot;
        let mut var_qfr3_db26: f64 = *var_qfr3_db26_slot;
        let mut var_qfr3_db27: f64 = *var_qfr3_db27_slot;
        let mut var_qfr3_db28: f64 = *var_qfr3_db28_slot;
        let mut var_qfr3_db29: f64 = *var_qfr3_db29_slot;
        let mut var_qfr3_db3: f64 = *var_qfr3_db3_slot;
        let mut var_qfr3_db30: f64 = *var_qfr3_db30_slot;
        let mut var_qfr3_db31: f64 = *var_qfr3_db31_slot;
        let mut var_qfr3_db32: f64 = *var_qfr3_db32_slot;
        let mut var_qfr3_db33: f64 = *var_qfr3_db33_slot;
        let mut var_qfr3_db34: f64 = *var_qfr3_db34_slot;
        let mut var_qfr3_db35: f64 = *var_qfr3_db35_slot;
        let mut var_qfr3_db36: f64 = *var_qfr3_db36_slot;
        let mut var_qfr3_db37: f64 = *var_qfr3_db37_slot;
        let mut var_qfr3_db38: f64 = *var_qfr3_db38_slot;
        let mut var_qfr3_db39: f64 = *var_qfr3_db39_slot;
        let mut var_qfr3_db4: f64 = *var_qfr3_db4_slot;
        let mut var_qfr3_db40: f64 = *var_qfr3_db40_slot;
        let mut var_qfr3_db41: f64 = *var_qfr3_db41_slot;
        let mut var_qfr3_db42: f64 = *var_qfr3_db42_slot;
        let mut var_qfr3_db43: f64 = *var_qfr3_db43_slot;
        let mut var_qfr3_db44: f64 = *var_qfr3_db44_slot;
        let mut var_qfr3_db45: f64 = *var_qfr3_db45_slot;
        let mut var_qfr3_db46: f64 = *var_qfr3_db46_slot;
        let mut var_qfr3_db47: f64 = *var_qfr3_db47_slot;
        let mut var_qfr3_db48: f64 = *var_qfr3_db48_slot;
        let mut var_qfr3_db49: f64 = *var_qfr3_db49_slot;
        let mut var_qfr3_db5: f64 = *var_qfr3_db5_slot;
        let mut var_qfr3_db50: f64 = *var_qfr3_db50_slot;
        let mut var_qfr3_db51: f64 = *var_qfr3_db51_slot;
        let mut var_qfr3_db52: f64 = *var_qfr3_db52_slot;
        let mut var_qfr3_db53: f64 = *var_qfr3_db53_slot;
        let mut var_qfr3_db54: f64 = *var_qfr3_db54_slot;
        let mut var_qfr3_db6: f64 = *var_qfr3_db6_slot;
        let mut var_qfr3_db7: f64 = *var_qfr3_db7_slot;
        let mut var_qfr3_db8: f64 = *var_qfr3_db8_slot;
        let mut var_qfr3_db9: f64 = *var_qfr3_db9_slot;
        let mut var_qfr3_dn0: f64 = *var_qfr3_dn0_slot;
        let mut var_qfr3_dn1: f64 = *var_qfr3_dn1_slot;
        let mut var_qfr3_dn10: f64 = *var_qfr3_dn10_slot;
        let mut var_qfr3_dn11: f64 = *var_qfr3_dn11_slot;
        let mut var_qfr3_dn12: f64 = *var_qfr3_dn12_slot;
        let mut var_qfr3_dn13: f64 = *var_qfr3_dn13_slot;
        let mut var_qfr3_dn14: f64 = *var_qfr3_dn14_slot;
        let mut var_qfr3_dn15: f64 = *var_qfr3_dn15_slot;
        let mut var_qfr3_dn16: f64 = *var_qfr3_dn16_slot;
        let mut var_qfr3_dn17: f64 = *var_qfr3_dn17_slot;
        let mut var_qfr3_dn18: f64 = *var_qfr3_dn18_slot;
        let mut var_qfr3_dn19: f64 = *var_qfr3_dn19_slot;
        let mut var_qfr3_dn2: f64 = *var_qfr3_dn2_slot;
        let mut var_qfr3_dn20: f64 = *var_qfr3_dn20_slot;
        let mut var_qfr3_dn21: f64 = *var_qfr3_dn21_slot;
        let mut var_qfr3_dn22: f64 = *var_qfr3_dn22_slot;
        let mut var_qfr3_dn3: f64 = *var_qfr3_dn3_slot;
        let mut var_qfr3_dn4: f64 = *var_qfr3_dn4_slot;
        let mut var_qfr3_dn5: f64 = *var_qfr3_dn5_slot;
        let mut var_qfr3_dn6: f64 = *var_qfr3_dn6_slot;
        let mut var_qfr3_dn7: f64 = *var_qfr3_dn7_slot;
        let mut var_qfr3_dn8: f64 = *var_qfr3_dn8_slot;
        let mut var_qfr3_dn9: f64 = *var_qfr3_dn9_slot;
        let mut var_qfr_db0: f64 = *var_qfr_db0_slot;
        let mut var_qfr_db1: f64 = *var_qfr_db1_slot;
        let mut var_qfr_db10: f64 = *var_qfr_db10_slot;
        let mut var_qfr_db11: f64 = *var_qfr_db11_slot;
        let mut var_qfr_db12: f64 = *var_qfr_db12_slot;
        let mut var_qfr_db13: f64 = *var_qfr_db13_slot;
        let mut var_qfr_db14: f64 = *var_qfr_db14_slot;
        let mut var_qfr_db15: f64 = *var_qfr_db15_slot;
        let mut var_qfr_db16: f64 = *var_qfr_db16_slot;
        let mut var_qfr_db17: f64 = *var_qfr_db17_slot;
        let mut var_qfr_db18: f64 = *var_qfr_db18_slot;
        let mut var_qfr_db19: f64 = *var_qfr_db19_slot;
        let mut var_qfr_db2: f64 = *var_qfr_db2_slot;
        let mut var_qfr_db20: f64 = *var_qfr_db20_slot;
        let mut var_qfr_db21: f64 = *var_qfr_db21_slot;
        let mut var_qfr_db22: f64 = *var_qfr_db22_slot;
        let mut var_qfr_db23: f64 = *var_qfr_db23_slot;
        let mut var_qfr_db24: f64 = *var_qfr_db24_slot;
        let mut var_qfr_db25: f64 = *var_qfr_db25_slot;
        let mut var_qfr_db26: f64 = *var_qfr_db26_slot;
        let mut var_qfr_db27: f64 = *var_qfr_db27_slot;
        let mut var_qfr_db28: f64 = *var_qfr_db28_slot;
        let mut var_qfr_db29: f64 = *var_qfr_db29_slot;
        let mut var_qfr_db3: f64 = *var_qfr_db3_slot;
        let mut var_qfr_db30: f64 = *var_qfr_db30_slot;
        let mut var_qfr_db31: f64 = *var_qfr_db31_slot;
        let mut var_qfr_db32: f64 = *var_qfr_db32_slot;
        let mut var_qfr_db33: f64 = *var_qfr_db33_slot;
        let mut var_qfr_db34: f64 = *var_qfr_db34_slot;
        let mut var_qfr_db35: f64 = *var_qfr_db35_slot;
        let mut var_qfr_db36: f64 = *var_qfr_db36_slot;
        let mut var_qfr_db37: f64 = *var_qfr_db37_slot;
        let mut var_qfr_db38: f64 = *var_qfr_db38_slot;
        let mut var_qfr_db39: f64 = *var_qfr_db39_slot;
        let mut var_qfr_db4: f64 = *var_qfr_db4_slot;
        let mut var_qfr_db40: f64 = *var_qfr_db40_slot;
        let mut var_qfr_db41: f64 = *var_qfr_db41_slot;
        let mut var_qfr_db42: f64 = *var_qfr_db42_slot;
        let mut var_qfr_db43: f64 = *var_qfr_db43_slot;
        let mut var_qfr_db44: f64 = *var_qfr_db44_slot;
        let mut var_qfr_db45: f64 = *var_qfr_db45_slot;
        let mut var_qfr_db46: f64 = *var_qfr_db46_slot;
        let mut var_qfr_db47: f64 = *var_qfr_db47_slot;
        let mut var_qfr_db48: f64 = *var_qfr_db48_slot;
        let mut var_qfr_db49: f64 = *var_qfr_db49_slot;
        let mut var_qfr_db5: f64 = *var_qfr_db5_slot;
        let mut var_qfr_db50: f64 = *var_qfr_db50_slot;
        let mut var_qfr_db51: f64 = *var_qfr_db51_slot;
        let mut var_qfr_db52: f64 = *var_qfr_db52_slot;
        let mut var_qfr_db53: f64 = *var_qfr_db53_slot;
        let mut var_qfr_db54: f64 = *var_qfr_db54_slot;
        let mut var_qfr_db6: f64 = *var_qfr_db6_slot;
        let mut var_qfr_db7: f64 = *var_qfr_db7_slot;
        let mut var_qfr_db8: f64 = *var_qfr_db8_slot;
        let mut var_qfr_db9: f64 = *var_qfr_db9_slot;
        let mut var_qfr_dn0: f64 = *var_qfr_dn0_slot;
        let mut var_qfr_dn1: f64 = *var_qfr_dn1_slot;
        let mut var_qfr_dn10: f64 = *var_qfr_dn10_slot;
        let mut var_qfr_dn11: f64 = *var_qfr_dn11_slot;
        let mut var_qfr_dn12: f64 = *var_qfr_dn12_slot;
        let mut var_qfr_dn13: f64 = *var_qfr_dn13_slot;
        let mut var_qfr_dn14: f64 = *var_qfr_dn14_slot;
        let mut var_qfr_dn15: f64 = *var_qfr_dn15_slot;
        let mut var_qfr_dn16: f64 = *var_qfr_dn16_slot;
        let mut var_qfr_dn17: f64 = *var_qfr_dn17_slot;
        let mut var_qfr_dn18: f64 = *var_qfr_dn18_slot;
        let mut var_qfr_dn19: f64 = *var_qfr_dn19_slot;
        let mut var_qfr_dn2: f64 = *var_qfr_dn2_slot;
        let mut var_qfr_dn20: f64 = *var_qfr_dn20_slot;
        let mut var_qfr_dn21: f64 = *var_qfr_dn21_slot;
        let mut var_qfr_dn22: f64 = *var_qfr_dn22_slot;
        let mut var_qfr_dn3: f64 = *var_qfr_dn3_slot;
        let mut var_qfr_dn4: f64 = *var_qfr_dn4_slot;
        let mut var_qfr_dn5: f64 = *var_qfr_dn5_slot;
        let mut var_qfr_dn6: f64 = *var_qfr_dn6_slot;
        let mut var_qfr_dn7: f64 = *var_qfr_dn7_slot;
        let mut var_qfr_dn8: f64 = *var_qfr_dn8_slot;
        let mut var_qfr_dn9: f64 = *var_qfr_dn9_slot;

        if (s.b[563] && s.b[564]) {
            s.store_div_ad_rhs(354, 372, A::mul(s.ad_value(373), s.ad_value(36)));
        }

        s.b[565] = (s.v[354] > 80.0);
        s.store_scalar(565, if s.b[565] { 1.0 } else { 0.0 });

        if ((s.b[563] && s.b[564]) && s.b[565]) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
            s.store_scalar(354, 80.0);
        }

        if ((s.b[563] && s.b[564]) && (!s.b[565])) {
            s.store_scalar(355, 1.0);
        }

        if (s.b[563] && s.b[564]) {
            s.store_mul_exp_rhs(355, 355, 354);
            s.store_mul_offset_rhs(370, 137, 355, (-1.0));
        }

        if (s.b[563] && (!s.b[564])) {
            s.store_div_ad_rhs(354, 372, A::mul(s.ad_value(373), s.ad_value(36)));
        }

        s.b[566] = (s.v[354] > 80.0);
        s.store_scalar(566, if s.b[566] { 1.0 } else { 0.0 });

        if ((s.b[563] && (!s.b[564])) && s.b[566]) {
            s.store_offset(355, 354, (((-80.0)) + (1.0)));
            s.store_scalar(354, 80.0);
        }

        if ((s.b[563] && (!s.b[564])) && (!s.b[566])) {
            s.store_scalar(355, 1.0);
        }

        if (s.b[563] && (!s.b[564])) {
            s.store_mul_exp_rhs(355, 355, 354);
            s.store_mul_offset_rhs(370, 137, 355, (-1.0));
        }

        if (!s.b[563]) {
            s.store_scalar(370, 0.0);
        }

        let assign31720_e49545: f64 = if p.p255 == 2.0 { 1.0 } else { 0.0 };
        var_guard535 = assign31720_e49545;

        s.b[570] = (p.p149 == 0.0);
        s.store_scalar(570, if s.b[570] { 1.0 } else { 0.0 });

        s.b[571] = (p.p150 != 0.0);
        s.store_scalar(571, if s.b[571] { 1.0 } else { 0.0 });

        s.b[572] = (p.p150 == 1.0);
        s.store_scalar(572, if s.b[572] { 1.0 } else { 0.0 });

        s.b[573] = (p.p150 != 0.0);
        s.store_scalar(573, if s.b[573] { 1.0 } else { 0.0 });

        s.b[574] = (p.p150 == 1.0);
        s.store_scalar(574, if s.b[574] { 1.0 } else { 0.0 });

        s.b[575] = (p.p149 == 0.0);
        s.store_scalar(575, if s.b[575] { 1.0 } else { 0.0 });

        s.b[576] = (p.p151 != 0.0);
        s.store_scalar(576, if s.b[576] { 1.0 } else { 0.0 });

        s.b[577] = (p.p151 == 1.0);
        s.store_scalar(577, if s.b[577] { 1.0 } else { 0.0 });

        s.b[578] = (p.p151 != 0.0);
        s.store_scalar(578, if s.b[578] { 1.0 } else { 0.0 });

        s.b[579] = (p.p151 == 1.0);
        s.store_scalar(579, if s.b[579] { 1.0 } else { 0.0 });

        s.b[580] = (p.p149 == 0.0);
        s.store_scalar(580, if s.b[580] { 1.0 } else { 0.0 });

        s.b[581] = (p.p152 != 0.0);
        s.store_scalar(581, if s.b[581] { 1.0 } else { 0.0 });

        s.b[582] = (p.p152 == 1.0);
        s.store_scalar(582, if s.b[582] { 1.0 } else { 0.0 });

        s.b[583] = (p.p152 != 0.0);
        s.store_scalar(583, if s.b[583] { 1.0 } else { 0.0 });

        s.b[584] = (p.p152 == 1.0);
        s.store_scalar(584, if s.b[584] { 1.0 } else { 0.0 });

        s.b[585] = (p.p149 == 0.0);
        s.store_scalar(585, if s.b[585] { 1.0 } else { 0.0 });

        s.b[586] = (p.p153 != 0.0);
        s.store_scalar(586, if s.b[586] { 1.0 } else { 0.0 });

        s.b[587] = (p.p153 == 1.0);
        s.store_scalar(587, if s.b[587] { 1.0 } else { 0.0 });

        s.b[588] = (p.p153 != 0.0);
        s.store_scalar(588, if s.b[588] { 1.0 } else { 0.0 });

        s.b[589] = (p.p153 == 1.0);
        s.store_scalar(589, if s.b[589] { 1.0 } else { 0.0 });

        s.b[590] = (p.p149 == 0.0);
        s.store_scalar(590, if s.b[590] { 1.0 } else { 0.0 });

        s.b[591] = (p.p154 != 0.0);
        s.store_scalar(591, if s.b[591] { 1.0 } else { 0.0 });

        s.b[592] = (p.p154 == 1.0);
        s.store_scalar(592, if s.b[592] { 1.0 } else { 0.0 });

        s.b[593] = (p.p154 != 0.0);
        s.store_scalar(593, if s.b[593] { 1.0 } else { 0.0 });

        s.b[594] = (p.p154 == 1.0);
        s.store_scalar(594, if s.b[594] { 1.0 } else { 0.0 });

        s.b[595] = (p.p149 == 0.0);
        s.store_scalar(595, if s.b[595] { 1.0 } else { 0.0 });

        s.b[596] = (p.p155 != 0.0);
        s.store_scalar(596, if s.b[596] { 1.0 } else { 0.0 });

        s.b[597] = (p.p155 == 1.0);
        s.store_scalar(597, if s.b[597] { 1.0 } else { 0.0 });

        s.b[598] = (p.p155 != 0.0);
        s.store_scalar(598, if s.b[598] { 1.0 } else { 0.0 });

        s.b[599] = (p.p155 == 1.0);
        s.store_scalar(599, if s.b[599] { 1.0 } else { 0.0 });

        s.b[600] = (p.p149 == 0.0);
        s.store_scalar(600, if s.b[600] { 1.0 } else { 0.0 });

        s.b[601] = (p.p156 != 0.0);
        s.store_scalar(601, if s.b[601] { 1.0 } else { 0.0 });

        s.b[602] = (p.p156 == 1.0);
        s.store_scalar(602, if s.b[602] { 1.0 } else { 0.0 });

        s.b[603] = (p.p156 != 0.0);
        s.store_scalar(603, if s.b[603] { 1.0 } else { 0.0 });

        s.b[604] = (p.p156 == 1.0);
        s.store_scalar(604, if s.b[604] { 1.0 } else { 0.0 });

        s.b[605] = (p.p149 == 0.0);
        s.store_scalar(605, if s.b[605] { 1.0 } else { 0.0 });

        s.b[606] = (p.p157 != 0.0);
        s.store_scalar(606, if s.b[606] { 1.0 } else { 0.0 });

        s.b[607] = (p.p157 == 1.0);
        s.store_scalar(607, if s.b[607] { 1.0 } else { 0.0 });

        s.b[608] = (p.p157 != 0.0);
        s.store_scalar(608, if s.b[608] { 1.0 } else { 0.0 });

        s.b[609] = (p.p157 == 1.0);
        s.store_scalar(609, if s.b[609] { 1.0 } else { 0.0 });

        let assign32130_e49670: f64 = (var_tdev / var_tnom);
        let assign32130_e49672: f64 = (assign32130_e49670 - 1.0);
        let assign32130_e49674: f64 = (assign32130_e49672 * p.p227);
        let assign32130_e49675: f64 = (p.p220 + assign32130_e49674);
        let assign32130_e49677: f64 = (assign32130_e49675 * (nv0 - nv2));
        let assign32130_e49678: f64 = (p.p222 - assign32130_e49677);
        var_qfr = assign32130_e49678;
        var_qfr_dn0 = (-((((var_tdev_dn0 / var_tnom) * p.p227) * (nv0 - nv2)) + assign32130_e49675));
        var_qfr_dn1 = (-(((var_tdev_dn1 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn2 = (-((((var_tdev_dn2 / var_tnom) * p.p227) * (nv0 - nv2)) + (-assign32130_e49675)));
        var_qfr_dn3 = (-(((var_tdev_dn3 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn4 = (-(((var_tdev_dn4 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn5 = (-(((var_tdev_dn5 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn6 = (-(((var_tdev_dn6 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn7 = (-(((var_tdev_dn7 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn8 = (-(((var_tdev_dn8 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn9 = (-(((var_tdev_dn9 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn10 = (-(((var_tdev_dn10 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn11 = (-(((var_tdev_dn11 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn12 = (-(((var_tdev_dn12 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn13 = (-(((var_tdev_dn13 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn14 = (-(((var_tdev_dn14 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn15 = (-(((var_tdev_dn15 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn16 = (-(((var_tdev_dn16 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn17 = (-(((var_tdev_dn17 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn18 = (-(((var_tdev_dn18 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn19 = (-(((var_tdev_dn19 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn20 = (-(((var_tdev_dn20 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn21 = (-(((var_tdev_dn21 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_dn22 = (-(((var_tdev_dn22 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db0 = (-(((var_tdev_db0 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db1 = (-(((var_tdev_db1 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db2 = (-(((var_tdev_db2 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db3 = (-(((var_tdev_db3 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db4 = (-(((var_tdev_db4 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db5 = (-(((var_tdev_db5 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db6 = (-(((var_tdev_db6 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db7 = (-(((var_tdev_db7 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db8 = (-(((var_tdev_db8 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db9 = (-(((var_tdev_db9 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db10 = (-(((var_tdev_db10 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db11 = (-(((var_tdev_db11 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db12 = (-(((var_tdev_db12 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db13 = (-(((var_tdev_db13 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db14 = (-(((var_tdev_db14 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db15 = (-(((var_tdev_db15 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db16 = (-(((var_tdev_db16 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db17 = (-(((var_tdev_db17 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db18 = (-(((var_tdev_db18 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db19 = (-(((var_tdev_db19 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db20 = (-(((var_tdev_db20 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db21 = (-(((var_tdev_db21 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db22 = (-(((var_tdev_db22 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db23 = (-(((var_tdev_db23 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db24 = (-(((var_tdev_db24 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db25 = (-(((var_tdev_db25 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db26 = (-(((var_tdev_db26 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db27 = (-(((var_tdev_db27 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db28 = (-(((var_tdev_db28 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db29 = (-(((var_tdev_db29 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db30 = (-(((var_tdev_db30 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db31 = (-(((var_tdev_db31 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db32 = (-(((var_tdev_db32 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db33 = (-(((var_tdev_db33 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db34 = (-(((var_tdev_db34 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db35 = (-(((var_tdev_db35 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db36 = (-(((var_tdev_db36 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db37 = (-(((var_tdev_db37 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db38 = (-(((var_tdev_db38 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db39 = (-(((var_tdev_db39 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db40 = (-(((var_tdev_db40 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db41 = (-(((var_tdev_db41 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db42 = (-(((var_tdev_db42 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db43 = (-(((var_tdev_db43 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db44 = (-(((var_tdev_db44 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db45 = (-(((var_tdev_db45 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db46 = (-(((var_tdev_db46 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db47 = (-(((var_tdev_db47 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db48 = (-(((var_tdev_db48 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db49 = (-(((var_tdev_db49 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db50 = (-(((var_tdev_db50 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db51 = (-(((var_tdev_db51 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db52 = (-(((var_tdev_db52 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db53 = (-(((var_tdev_db53 / var_tnom) * p.p227) * (nv0 - nv2)));
        var_qfr_db54 = (-(((var_tdev_db54 / var_tnom) * p.p227) * (nv0 - nv2)));

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
        var_qfr_dn1 = (assign32140_e49681 * (var_qfr_dn1 - (0.5 * (var_qfr_dn1 - (((var_qfr_dn1 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn1)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn2 = (assign32140_e49681 * (var_qfr_dn2 - (0.5 * (var_qfr_dn2 - (((var_qfr_dn2 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn2)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn3 = (assign32140_e49681 * (var_qfr_dn3 - (0.5 * (var_qfr_dn3 - (((var_qfr_dn3 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn3)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn4 = (assign32140_e49681 * (var_qfr_dn4 - (0.5 * (var_qfr_dn4 - (((var_qfr_dn4 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn4)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn5 = (assign32140_e49681 * (var_qfr_dn5 - (0.5 * (var_qfr_dn5 - (((var_qfr_dn5 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn5)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn6 = (assign32140_e49681 * (var_qfr_dn6 - (0.5 * (var_qfr_dn6 - (((var_qfr_dn6 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn6)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn7 = (assign32140_e49681 * (var_qfr_dn7 - (0.5 * (var_qfr_dn7 - (((var_qfr_dn7 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn7)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn8 = (assign32140_e49681 * (var_qfr_dn8 - (0.5 * (var_qfr_dn8 - (((var_qfr_dn8 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn8)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn9 = (assign32140_e49681 * (var_qfr_dn9 - (0.5 * (var_qfr_dn9 - (((var_qfr_dn9 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn9)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn10 = (assign32140_e49681 * (var_qfr_dn10 - (0.5 * (var_qfr_dn10 - (((var_qfr_dn10 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn10)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn11 = (assign32140_e49681 * (var_qfr_dn11 - (0.5 * (var_qfr_dn11 - (((var_qfr_dn11 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn11)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn12 = (assign32140_e49681 * (var_qfr_dn12 - (0.5 * (var_qfr_dn12 - (((var_qfr_dn12 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn12)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn13 = (assign32140_e49681 * (var_qfr_dn13 - (0.5 * (var_qfr_dn13 - (((var_qfr_dn13 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn13)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn14 = (assign32140_e49681 * (var_qfr_dn14 - (0.5 * (var_qfr_dn14 - (((var_qfr_dn14 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn14)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn15 = (assign32140_e49681 * (var_qfr_dn15 - (0.5 * (var_qfr_dn15 - (((var_qfr_dn15 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn15)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn16 = (assign32140_e49681 * (var_qfr_dn16 - (0.5 * (var_qfr_dn16 - (((var_qfr_dn16 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn16)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn17 = (assign32140_e49681 * (var_qfr_dn17 - (0.5 * (var_qfr_dn17 - (((var_qfr_dn17 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn17)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn18 = (assign32140_e49681 * (var_qfr_dn18 - (0.5 * (var_qfr_dn18 - (((var_qfr_dn18 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn18)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn19 = (assign32140_e49681 * (var_qfr_dn19 - (0.5 * (var_qfr_dn19 - (((var_qfr_dn19 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn19)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn20 = (assign32140_e49681 * (var_qfr_dn20 - (0.5 * (var_qfr_dn20 - (((var_qfr_dn20 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn20)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn21 = (assign32140_e49681 * (var_qfr_dn21 - (0.5 * (var_qfr_dn21 - (((var_qfr_dn21 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn21)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn22 = (assign32140_e49681 * (var_qfr_dn22 - (0.5 * (var_qfr_dn22 - (((var_qfr_dn22 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn22)) / (2.0 * assign32140_e49698))))));
        var_qfr_db0 = (assign32140_e49681 * (var_qfr_db0 - (0.5 * (var_qfr_db0 - (((var_qfr_db0 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db0)) / (2.0 * assign32140_e49698))))));
        var_qfr_db1 = (assign32140_e49681 * (var_qfr_db1 - (0.5 * (var_qfr_db1 - (((var_qfr_db1 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db1)) / (2.0 * assign32140_e49698))))));
        var_qfr_db2 = (assign32140_e49681 * (var_qfr_db2 - (0.5 * (var_qfr_db2 - (((var_qfr_db2 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db2)) / (2.0 * assign32140_e49698))))));
        var_qfr_db3 = (assign32140_e49681 * (var_qfr_db3 - (0.5 * (var_qfr_db3 - (((var_qfr_db3 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db3)) / (2.0 * assign32140_e49698))))));
        var_qfr_db4 = (assign32140_e49681 * (var_qfr_db4 - (0.5 * (var_qfr_db4 - (((var_qfr_db4 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db4)) / (2.0 * assign32140_e49698))))));
        var_qfr_db5 = (assign32140_e49681 * (var_qfr_db5 - (0.5 * (var_qfr_db5 - (((var_qfr_db5 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db5)) / (2.0 * assign32140_e49698))))));
        var_qfr_db6 = (assign32140_e49681 * (var_qfr_db6 - (0.5 * (var_qfr_db6 - (((var_qfr_db6 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db6)) / (2.0 * assign32140_e49698))))));
        var_qfr_db7 = (assign32140_e49681 * (var_qfr_db7 - (0.5 * (var_qfr_db7 - (((var_qfr_db7 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db7)) / (2.0 * assign32140_e49698))))));
        var_qfr_db8 = (assign32140_e49681 * (var_qfr_db8 - (0.5 * (var_qfr_db8 - (((var_qfr_db8 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db8)) / (2.0 * assign32140_e49698))))));
        var_qfr_db9 = (assign32140_e49681 * (var_qfr_db9 - (0.5 * (var_qfr_db9 - (((var_qfr_db9 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db9)) / (2.0 * assign32140_e49698))))));
        var_qfr_db10 = (assign32140_e49681 * (var_qfr_db10 - (0.5 * (var_qfr_db10 - (((var_qfr_db10 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db10)) / (2.0 * assign32140_e49698))))));
        var_qfr_db11 = (assign32140_e49681 * (var_qfr_db11 - (0.5 * (var_qfr_db11 - (((var_qfr_db11 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db11)) / (2.0 * assign32140_e49698))))));
        var_qfr_db12 = (assign32140_e49681 * (var_qfr_db12 - (0.5 * (var_qfr_db12 - (((var_qfr_db12 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db12)) / (2.0 * assign32140_e49698))))));
        var_qfr_db13 = (assign32140_e49681 * (var_qfr_db13 - (0.5 * (var_qfr_db13 - (((var_qfr_db13 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db13)) / (2.0 * assign32140_e49698))))));
        var_qfr_db14 = (assign32140_e49681 * (var_qfr_db14 - (0.5 * (var_qfr_db14 - (((var_qfr_db14 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db14)) / (2.0 * assign32140_e49698))))));
        var_qfr_db15 = (assign32140_e49681 * (var_qfr_db15 - (0.5 * (var_qfr_db15 - (((var_qfr_db15 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db15)) / (2.0 * assign32140_e49698))))));
        var_qfr_db16 = (assign32140_e49681 * (var_qfr_db16 - (0.5 * (var_qfr_db16 - (((var_qfr_db16 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db16)) / (2.0 * assign32140_e49698))))));
        var_qfr_db17 = (assign32140_e49681 * (var_qfr_db17 - (0.5 * (var_qfr_db17 - (((var_qfr_db17 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db17)) / (2.0 * assign32140_e49698))))));
        var_qfr_db18 = (assign32140_e49681 * (var_qfr_db18 - (0.5 * (var_qfr_db18 - (((var_qfr_db18 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db18)) / (2.0 * assign32140_e49698))))));
        var_qfr_db19 = (assign32140_e49681 * (var_qfr_db19 - (0.5 * (var_qfr_db19 - (((var_qfr_db19 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db19)) / (2.0 * assign32140_e49698))))));
        var_qfr_db20 = (assign32140_e49681 * (var_qfr_db20 - (0.5 * (var_qfr_db20 - (((var_qfr_db20 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db20)) / (2.0 * assign32140_e49698))))));
        var_qfr_db21 = (assign32140_e49681 * (var_qfr_db21 - (0.5 * (var_qfr_db21 - (((var_qfr_db21 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db21)) / (2.0 * assign32140_e49698))))));
        var_qfr_db22 = (assign32140_e49681 * (var_qfr_db22 - (0.5 * (var_qfr_db22 - (((var_qfr_db22 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db22)) / (2.0 * assign32140_e49698))))));
        var_qfr_db23 = (assign32140_e49681 * (var_qfr_db23 - (0.5 * (var_qfr_db23 - (((var_qfr_db23 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db23)) / (2.0 * assign32140_e49698))))));
        var_qfr_db24 = (assign32140_e49681 * (var_qfr_db24 - (0.5 * (var_qfr_db24 - (((var_qfr_db24 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db24)) / (2.0 * assign32140_e49698))))));
        var_qfr_db25 = (assign32140_e49681 * (var_qfr_db25 - (0.5 * (var_qfr_db25 - (((var_qfr_db25 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db25)) / (2.0 * assign32140_e49698))))));
        var_qfr_db26 = (assign32140_e49681 * (var_qfr_db26 - (0.5 * (var_qfr_db26 - (((var_qfr_db26 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db26)) / (2.0 * assign32140_e49698))))));
        var_qfr_db27 = (assign32140_e49681 * (var_qfr_db27 - (0.5 * (var_qfr_db27 - (((var_qfr_db27 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db27)) / (2.0 * assign32140_e49698))))));
        var_qfr_db28 = (assign32140_e49681 * (var_qfr_db28 - (0.5 * (var_qfr_db28 - (((var_qfr_db28 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db28)) / (2.0 * assign32140_e49698))))));
        var_qfr_db29 = (assign32140_e49681 * (var_qfr_db29 - (0.5 * (var_qfr_db29 - (((var_qfr_db29 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db29)) / (2.0 * assign32140_e49698))))));
        var_qfr_db30 = (assign32140_e49681 * (var_qfr_db30 - (0.5 * (var_qfr_db30 - (((var_qfr_db30 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db30)) / (2.0 * assign32140_e49698))))));
        var_qfr_db31 = (assign32140_e49681 * (var_qfr_db31 - (0.5 * (var_qfr_db31 - (((var_qfr_db31 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db31)) / (2.0 * assign32140_e49698))))));
        var_qfr_db32 = (assign32140_e49681 * (var_qfr_db32 - (0.5 * (var_qfr_db32 - (((var_qfr_db32 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db32)) / (2.0 * assign32140_e49698))))));
        var_qfr_db33 = (assign32140_e49681 * (var_qfr_db33 - (0.5 * (var_qfr_db33 - (((var_qfr_db33 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db33)) / (2.0 * assign32140_e49698))))));
        var_qfr_db34 = (assign32140_e49681 * (var_qfr_db34 - (0.5 * (var_qfr_db34 - (((var_qfr_db34 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db34)) / (2.0 * assign32140_e49698))))));
        var_qfr_db35 = (assign32140_e49681 * (var_qfr_db35 - (0.5 * (var_qfr_db35 - (((var_qfr_db35 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db35)) / (2.0 * assign32140_e49698))))));
        var_qfr_db36 = (assign32140_e49681 * (var_qfr_db36 - (0.5 * (var_qfr_db36 - (((var_qfr_db36 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db36)) / (2.0 * assign32140_e49698))))));
        var_qfr_db37 = (assign32140_e49681 * (var_qfr_db37 - (0.5 * (var_qfr_db37 - (((var_qfr_db37 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db37)) / (2.0 * assign32140_e49698))))));
        var_qfr_db38 = (assign32140_e49681 * (var_qfr_db38 - (0.5 * (var_qfr_db38 - (((var_qfr_db38 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db38)) / (2.0 * assign32140_e49698))))));
        var_qfr_db39 = (assign32140_e49681 * (var_qfr_db39 - (0.5 * (var_qfr_db39 - (((var_qfr_db39 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db39)) / (2.0 * assign32140_e49698))))));
        var_qfr_db40 = (assign32140_e49681 * (var_qfr_db40 - (0.5 * (var_qfr_db40 - (((var_qfr_db40 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db40)) / (2.0 * assign32140_e49698))))));
        var_qfr_db41 = (assign32140_e49681 * (var_qfr_db41 - (0.5 * (var_qfr_db41 - (((var_qfr_db41 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db41)) / (2.0 * assign32140_e49698))))));
        var_qfr_db42 = (assign32140_e49681 * (var_qfr_db42 - (0.5 * (var_qfr_db42 - (((var_qfr_db42 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db42)) / (2.0 * assign32140_e49698))))));
        var_qfr_db43 = (assign32140_e49681 * (var_qfr_db43 - (0.5 * (var_qfr_db43 - (((var_qfr_db43 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db43)) / (2.0 * assign32140_e49698))))));
        var_qfr_db44 = (assign32140_e49681 * (var_qfr_db44 - (0.5 * (var_qfr_db44 - (((var_qfr_db44 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db44)) / (2.0 * assign32140_e49698))))));
        var_qfr_db45 = (assign32140_e49681 * (var_qfr_db45 - (0.5 * (var_qfr_db45 - (((var_qfr_db45 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db45)) / (2.0 * assign32140_e49698))))));
        var_qfr_db46 = (assign32140_e49681 * (var_qfr_db46 - (0.5 * (var_qfr_db46 - (((var_qfr_db46 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db46)) / (2.0 * assign32140_e49698))))));
        var_qfr_db47 = (assign32140_e49681 * (var_qfr_db47 - (0.5 * (var_qfr_db47 - (((var_qfr_db47 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db47)) / (2.0 * assign32140_e49698))))));
        var_qfr_db48 = (assign32140_e49681 * (var_qfr_db48 - (0.5 * (var_qfr_db48 - (((var_qfr_db48 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db48)) / (2.0 * assign32140_e49698))))));
        var_qfr_db49 = (assign32140_e49681 * (var_qfr_db49 - (0.5 * (var_qfr_db49 - (((var_qfr_db49 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db49)) / (2.0 * assign32140_e49698))))));
        var_qfr_db50 = (assign32140_e49681 * (var_qfr_db50 - (0.5 * (var_qfr_db50 - (((var_qfr_db50 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db50)) / (2.0 * assign32140_e49698))))));
        var_qfr_db51 = (assign32140_e49681 * (var_qfr_db51 - (0.5 * (var_qfr_db51 - (((var_qfr_db51 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db51)) / (2.0 * assign32140_e49698))))));
        var_qfr_db52 = (assign32140_e49681 * (var_qfr_db52 - (0.5 * (var_qfr_db52 - (((var_qfr_db52 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db52)) / (2.0 * assign32140_e49698))))));
        var_qfr_db53 = (assign32140_e49681 * (var_qfr_db53 - (0.5 * (var_qfr_db53 - (((var_qfr_db53 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db53)) / (2.0 * assign32140_e49698))))));
        var_qfr_db54 = (assign32140_e49681 * (var_qfr_db54 - (0.5 * (var_qfr_db54 - (((var_qfr_db54 * assign32140_e49694) + (assign32140_e49691 * var_qfr_db54)) / (2.0 * assign32140_e49698))))));

        s.store_scaled_add_offset_sqrt_square_offset_ad(136, A::sub_from_scalar(p.p218, A::scale_offset(s.ad_value(82), ((1.0 / (var_tnom)) * (p.p226)), (((-1.0)) * (p.p226)))), 1e-18, (-1e-18), ((0.25 * 1e-19) * 1e-19), 0.5);

        s.store_mul_scaled_voltage(196, 136, (p.p4 * p.p5), ctx, nodes, Some(9), Some(2));

        let assign32170_e49756: f64 = (p.p4 * p.p5);
        let assign32170_e49758: f64 = (assign32170_e49756 * p.p219);
        let assign32170_e49760: f64 = (assign32170_e49758 * (nv2 - nv0));
        var_qfr3 = assign32170_e49760;
        var_qfr3_dn0 = (-assign32170_e49758);
        var_qfr3_dn1 = 0.0;
        var_qfr3_dn2 = assign32170_e49758;
        var_qfr3_dn3 = 0.0;
        var_qfr3_dn4 = 0.0;
        var_qfr3_dn5 = 0.0;
        var_qfr3_dn6 = 0.0;
        var_qfr3_dn7 = 0.0;
        var_qfr3_dn8 = 0.0;
        var_qfr3_dn9 = 0.0;
        var_qfr3_dn10 = 0.0;
        var_qfr3_dn11 = 0.0;
        var_qfr3_dn12 = 0.0;
        var_qfr3_dn13 = 0.0;
        var_qfr3_dn14 = 0.0;
        var_qfr3_dn15 = 0.0;
        var_qfr3_dn16 = 0.0;
        var_qfr3_dn17 = 0.0;
        var_qfr3_dn18 = 0.0;
        var_qfr3_dn19 = 0.0;
        var_qfr3_dn20 = 0.0;
        var_qfr3_dn21 = 0.0;
        var_qfr3_dn22 = 0.0;
        var_qfr3_db0 = 0.0;
        var_qfr3_db1 = 0.0;
        var_qfr3_db2 = 0.0;
        var_qfr3_db3 = 0.0;
        var_qfr3_db4 = 0.0;
        var_qfr3_db5 = 0.0;
        var_qfr3_db6 = 0.0;
        var_qfr3_db7 = 0.0;
        var_qfr3_db8 = 0.0;
        var_qfr3_db9 = 0.0;
        var_qfr3_db10 = 0.0;
        var_qfr3_db11 = 0.0;
        var_qfr3_db12 = 0.0;
        var_qfr3_db13 = 0.0;
        var_qfr3_db14 = 0.0;
        var_qfr3_db15 = 0.0;
        var_qfr3_db16 = 0.0;
        var_qfr3_db17 = 0.0;
        var_qfr3_db18 = 0.0;
        var_qfr3_db19 = 0.0;
        var_qfr3_db20 = 0.0;
        var_qfr3_db21 = 0.0;
        var_qfr3_db22 = 0.0;
        var_qfr3_db23 = 0.0;
        var_qfr3_db24 = 0.0;
        var_qfr3_db25 = 0.0;
        var_qfr3_db26 = 0.0;
        var_qfr3_db27 = 0.0;
        var_qfr3_db28 = 0.0;
        var_qfr3_db29 = 0.0;
        var_qfr3_db30 = 0.0;
        var_qfr3_db31 = 0.0;
        var_qfr3_db32 = 0.0;
        var_qfr3_db33 = 0.0;
        var_qfr3_db34 = 0.0;
        var_qfr3_db35 = 0.0;
        var_qfr3_db36 = 0.0;
        var_qfr3_db37 = 0.0;
        var_qfr3_db38 = 0.0;
        var_qfr3_db39 = 0.0;
        var_qfr3_db40 = 0.0;
        var_qfr3_db41 = 0.0;
        var_qfr3_db42 = 0.0;
        var_qfr3_db43 = 0.0;
        var_qfr3_db44 = 0.0;
        var_qfr3_db45 = 0.0;
        var_qfr3_db46 = 0.0;
        var_qfr3_db47 = 0.0;
        var_qfr3_db48 = 0.0;
        var_qfr3_db49 = 0.0;
        var_qfr3_db50 = 0.0;
        var_qfr3_db51 = 0.0;
        var_qfr3_db52 = 0.0;
        var_qfr3_db53 = 0.0;
        var_qfr3_db54 = 0.0;

        s.store_offset_scaled_ad(136, A::scale_offset(s.ad_value(82), ((1.0 / (var_tnom)) * (p.p225)), (((-1.0)) * (p.p225))), (-(1.0 - { let limited_exp_arg = ((-((p.p229) as f64).ln()) / p.p228); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })), ((p.p224) * ((1.0 - { let limited_exp_arg = ((-((p.p229) as f64).ln()) / p.p228); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))));

        s.store_div_scaled_inputs2_mixed_iai(90, 136, 1.0, A::voltage(ctx, nodes, Some(2), Some(0)), (-1.0), 36, 1.0);

        s.store_sqrt_offset_ad(91, A::mul_scaled_lhs(s.ad_value(90), p.p230, s.ad_value(90)), 1.92);

        s.store_scaled_add(137, 90, 91, 0.5);

        s.store_add_scaled_product_indices(106, 136, 1.0, 36, 137, (-1.0));

        s.store_ln_ad(192, A::sub_from_scalar(1.0, A::scale(s.ad_value(106), 1.0 / (p.p224))));

        s.store_mul_sub_from_scalar_lhs_scaled_ad(193, p.p224, A::scale_offset(s.ad_value(82), ((1.0 / (var_tnom)) * (p.p225)), (((-1.0)) * (p.p225))), A::sub_from_scalar(1.0, A::limited_exp_scaled_input(s.ad_value(192), (1.0 - p.p228))), (p.p223 * 1.0 / ((1.0 - p.p228))));

        s.store_add_scaled_inputs3_mixed_iai(194, 193, (p.p4 * p.p5), A::voltage(ctx, nodes, Some(2), Some(0)), ((p.p229 * p.p223) * (p.p4 * p.p5)), 106, ((-(p.p229 * p.p223)) * (p.p4 * p.p5)));

        let assign32260_e49851: f64 = if ((p.p31 == 1.0) && (p.p32 > 0.0)) { 1.0 } else { 0.0 };
        var_guard576 = assign32260_e49851;


        *var_guard535_slot = var_guard535;
        *var_guard576_slot = var_guard576;
        *var_qfr_slot = var_qfr;
        *var_qfr3_slot = var_qfr3;
        *var_qfr3_db0_slot = var_qfr3_db0;
        *var_qfr3_db1_slot = var_qfr3_db1;
        *var_qfr3_db10_slot = var_qfr3_db10;
        *var_qfr3_db11_slot = var_qfr3_db11;
        *var_qfr3_db12_slot = var_qfr3_db12;
        *var_qfr3_db13_slot = var_qfr3_db13;
        *var_qfr3_db14_slot = var_qfr3_db14;
        *var_qfr3_db15_slot = var_qfr3_db15;
        *var_qfr3_db16_slot = var_qfr3_db16;
        *var_qfr3_db17_slot = var_qfr3_db17;
        *var_qfr3_db18_slot = var_qfr3_db18;
        *var_qfr3_db19_slot = var_qfr3_db19;
        *var_qfr3_db2_slot = var_qfr3_db2;
        *var_qfr3_db20_slot = var_qfr3_db20;
        *var_qfr3_db21_slot = var_qfr3_db21;
        *var_qfr3_db22_slot = var_qfr3_db22;
        *var_qfr3_db23_slot = var_qfr3_db23;
        *var_qfr3_db24_slot = var_qfr3_db24;
        *var_qfr3_db25_slot = var_qfr3_db25;
        *var_qfr3_db26_slot = var_qfr3_db26;
        *var_qfr3_db27_slot = var_qfr3_db27;
        *var_qfr3_db28_slot = var_qfr3_db28;
        *var_qfr3_db29_slot = var_qfr3_db29;
        *var_qfr3_db3_slot = var_qfr3_db3;
        *var_qfr3_db30_slot = var_qfr3_db30;
        *var_qfr3_db31_slot = var_qfr3_db31;
        *var_qfr3_db32_slot = var_qfr3_db32;
        *var_qfr3_db33_slot = var_qfr3_db33;
        *var_qfr3_db34_slot = var_qfr3_db34;
        *var_qfr3_db35_slot = var_qfr3_db35;
        *var_qfr3_db36_slot = var_qfr3_db36;
        *var_qfr3_db37_slot = var_qfr3_db37;
        *var_qfr3_db38_slot = var_qfr3_db38;
        *var_qfr3_db39_slot = var_qfr3_db39;
        *var_qfr3_db4_slot = var_qfr3_db4;
        *var_qfr3_db40_slot = var_qfr3_db40;
        *var_qfr3_db41_slot = var_qfr3_db41;
        *var_qfr3_db42_slot = var_qfr3_db42;
        *var_qfr3_db43_slot = var_qfr3_db43;
        *var_qfr3_db44_slot = var_qfr3_db44;
        *var_qfr3_db45_slot = var_qfr3_db45;
        *var_qfr3_db46_slot = var_qfr3_db46;
        *var_qfr3_db47_slot = var_qfr3_db47;
        *var_qfr3_db48_slot = var_qfr3_db48;
        *var_qfr3_db49_slot = var_qfr3_db49;
        *var_qfr3_db5_slot = var_qfr3_db5;
        *var_qfr3_db50_slot = var_qfr3_db50;
        *var_qfr3_db51_slot = var_qfr3_db51;
        *var_qfr3_db52_slot = var_qfr3_db52;
        *var_qfr3_db53_slot = var_qfr3_db53;
        *var_qfr3_db54_slot = var_qfr3_db54;
        *var_qfr3_db6_slot = var_qfr3_db6;
        *var_qfr3_db7_slot = var_qfr3_db7;
        *var_qfr3_db8_slot = var_qfr3_db8;
        *var_qfr3_db9_slot = var_qfr3_db9;
        *var_qfr3_dn0_slot = var_qfr3_dn0;
        *var_qfr3_dn1_slot = var_qfr3_dn1;
        *var_qfr3_dn10_slot = var_qfr3_dn10;
        *var_qfr3_dn11_slot = var_qfr3_dn11;
        *var_qfr3_dn12_slot = var_qfr3_dn12;
        *var_qfr3_dn13_slot = var_qfr3_dn13;
        *var_qfr3_dn14_slot = var_qfr3_dn14;
        *var_qfr3_dn15_slot = var_qfr3_dn15;
        *var_qfr3_dn16_slot = var_qfr3_dn16;
        *var_qfr3_dn17_slot = var_qfr3_dn17;
        *var_qfr3_dn18_slot = var_qfr3_dn18;
        *var_qfr3_dn19_slot = var_qfr3_dn19;
        *var_qfr3_dn2_slot = var_qfr3_dn2;
        *var_qfr3_dn20_slot = var_qfr3_dn20;
        *var_qfr3_dn21_slot = var_qfr3_dn21;
        *var_qfr3_dn22_slot = var_qfr3_dn22;
        *var_qfr3_dn3_slot = var_qfr3_dn3;
        *var_qfr3_dn4_slot = var_qfr3_dn4;
        *var_qfr3_dn5_slot = var_qfr3_dn5;
        *var_qfr3_dn6_slot = var_qfr3_dn6;
        *var_qfr3_dn7_slot = var_qfr3_dn7;
        *var_qfr3_dn8_slot = var_qfr3_dn8;
        *var_qfr3_dn9_slot = var_qfr3_dn9;
        *var_qfr_db0_slot = var_qfr_db0;
        *var_qfr_db1_slot = var_qfr_db1;
        *var_qfr_db10_slot = var_qfr_db10;
        *var_qfr_db11_slot = var_qfr_db11;
        *var_qfr_db12_slot = var_qfr_db12;
        *var_qfr_db13_slot = var_qfr_db13;
        *var_qfr_db14_slot = var_qfr_db14;
        *var_qfr_db15_slot = var_qfr_db15;
        *var_qfr_db16_slot = var_qfr_db16;
        *var_qfr_db17_slot = var_qfr_db17;
        *var_qfr_db18_slot = var_qfr_db18;
        *var_qfr_db19_slot = var_qfr_db19;
        *var_qfr_db2_slot = var_qfr_db2;
        *var_qfr_db20_slot = var_qfr_db20;
        *var_qfr_db21_slot = var_qfr_db21;
        *var_qfr_db22_slot = var_qfr_db22;
        *var_qfr_db23_slot = var_qfr_db23;
        *var_qfr_db24_slot = var_qfr_db24;
        *var_qfr_db25_slot = var_qfr_db25;
        *var_qfr_db26_slot = var_qfr_db26;
        *var_qfr_db27_slot = var_qfr_db27;
        *var_qfr_db28_slot = var_qfr_db28;
        *var_qfr_db29_slot = var_qfr_db29;
        *var_qfr_db3_slot = var_qfr_db3;
        *var_qfr_db30_slot = var_qfr_db30;
        *var_qfr_db31_slot = var_qfr_db31;
        *var_qfr_db32_slot = var_qfr_db32;
        *var_qfr_db33_slot = var_qfr_db33;
        *var_qfr_db34_slot = var_qfr_db34;
        *var_qfr_db35_slot = var_qfr_db35;
        *var_qfr_db36_slot = var_qfr_db36;
        *var_qfr_db37_slot = var_qfr_db37;
        *var_qfr_db38_slot = var_qfr_db38;
        *var_qfr_db39_slot = var_qfr_db39;
        *var_qfr_db4_slot = var_qfr_db4;
        *var_qfr_db40_slot = var_qfr_db40;
        *var_qfr_db41_slot = var_qfr_db41;
        *var_qfr_db42_slot = var_qfr_db42;
        *var_qfr_db43_slot = var_qfr_db43;
        *var_qfr_db44_slot = var_qfr_db44;
        *var_qfr_db45_slot = var_qfr_db45;
        *var_qfr_db46_slot = var_qfr_db46;
        *var_qfr_db47_slot = var_qfr_db47;
        *var_qfr_db48_slot = var_qfr_db48;
        *var_qfr_db49_slot = var_qfr_db49;
        *var_qfr_db5_slot = var_qfr_db5;
        *var_qfr_db50_slot = var_qfr_db50;
        *var_qfr_db51_slot = var_qfr_db51;
        *var_qfr_db52_slot = var_qfr_db52;
        *var_qfr_db53_slot = var_qfr_db53;
        *var_qfr_db54_slot = var_qfr_db54;
        *var_qfr_db6_slot = var_qfr_db6;
        *var_qfr_db7_slot = var_qfr_db7;
        *var_qfr_db8_slot = var_qfr_db8;
        *var_qfr_db9_slot = var_qfr_db9;
        *var_qfr_dn0_slot = var_qfr_dn0;
        *var_qfr_dn1_slot = var_qfr_dn1;
        *var_qfr_dn10_slot = var_qfr_dn10;
        *var_qfr_dn11_slot = var_qfr_dn11;
        *var_qfr_dn12_slot = var_qfr_dn12;
        *var_qfr_dn13_slot = var_qfr_dn13;
        *var_qfr_dn14_slot = var_qfr_dn14;
        *var_qfr_dn15_slot = var_qfr_dn15;
        *var_qfr_dn16_slot = var_qfr_dn16;
        *var_qfr_dn17_slot = var_qfr_dn17;
        *var_qfr_dn18_slot = var_qfr_dn18;
        *var_qfr_dn19_slot = var_qfr_dn19;
        *var_qfr_dn2_slot = var_qfr_dn2;
        *var_qfr_dn20_slot = var_qfr_dn20;
        *var_qfr_dn21_slot = var_qfr_dn21;
        *var_qfr_dn22_slot = var_qfr_dn22;
        *var_qfr_dn3_slot = var_qfr_dn3;
        *var_qfr_dn4_slot = var_qfr_dn4;
        *var_qfr_dn5_slot = var_qfr_dn5;
        *var_qfr_dn6_slot = var_qfr_dn6;
        *var_qfr_dn7_slot = var_qfr_dn7;
        *var_qfr_dn8_slot = var_qfr_dn8;
        *var_qfr_dn9_slot = var_qfr_dn9;
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

    pub(super) fn stamp_reactive_block_7(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[448] && s.b[449]) && s.b[455]) {
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

        if ((s.b[448] && s.b[449]) && s.b[455]) {
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

        if ((s.b[448] && s.b[449]) && s.b[455]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p169, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p170, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(237, 128, 86);
        }

        if ((s.b[448] && s.b[449]) && (!s.b[455])) {
            s.store_add(237, 100, 86);
        }

        if (s.b[448] && s.b[449]) {
            s.store_scaled_add(238, 236, 237, 0.5);
            s.store_sub(239, 237, 236);
            s.store_mul_add_scaled_inputs3_offset_rhs(135, 239, s.ad_value(234), 1.0, s.ad_value(238), (-1.0), s.ad_value(83), 1.0, 0.0);
            s.store_mul_scaled_abs_ad_rhs(136, 235, 1.0 / (p.p9), A::sub(s.ad_value(234), s.ad_value(238)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);
            s.store_scaled_mul(96, 95, 235, (p.p4 * (p.p5 * 1.0 / (p.p161))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(244), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(239), (p.p25 * p.p25), s.ad_value(239)), 1.0);
            s.store_div(93, 98, 92);
            s.store_sub(90, 237, 236);
            s.store_add_scaled_inputs3_indices(91, 234, 1.0, 83, 1.0, 238, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 235, s.ad_value(234), ((p.p4 * p.p5) * p.p161), s.ad_value(238), (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p236) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p235, 1.0);
            s.store_div_from_scalar(190, p.p234, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p160);
            s.store_mul_add_scaled_inputs3_offset_rhs(240, 191, s.ad_value(234), ((p.p4 * p.p5) * p.p161), s.ad_value(238), (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);
            s.store_add_scaled_inputs3_indices(136, 234, 1.0, 83, 1.0, 238, -1.0);
            s.store_add_scaled_inputs(90, 236, 0.3333333333333333, 237, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(239)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(239)), 239, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(241, 191, 234, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p161) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p161) * p.p5) * 0.5)));
        }

        s.b[456] = (s.v[52] < 0.0);
        s.store_scalar(456, if s.b[456] { 1.0 } else { 0.0 });

        if ((s.b[448] && s.b[449]) && s.b[456]) {
            s.store_sub_scaled_inputs(241, 240, (-1.0), 241, 1.0);
        }

        if (s.b[448] && (!s.b[449])) {
            s.store_scalar(240, 0.0);
            s.store_scalar(241, 0.0);
        }

        s.b[457] = (p.p151 != 0.0);
        s.store_scalar(457, if s.b[457] { 1.0 } else { 0.0 });

        s.b[458] = (p.p151 == 1.0);
        s.store_scalar(458, if s.b[458] { 1.0 } else { 0.0 });

        if (((!s.b[448]) && s.b[457]) && s.b[458]) {
            s.store_voltage(54, ctx, nodes, Some(9), Some(8));
        }

        if (((!s.b[448]) && s.b[457]) && (!s.b[458])) {
            s.store_voltage(54, ctx, nodes, Some(2), Some(8));
        }

        if ((!s.b[448]) && s.b[457]) {
            s.copy_ad(234, 54);
            s.store_scalar(146, (1.0 + p.p165));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_offset_scaled(88, 82, ((1.0 / (s.v[35])) * (p.p162)), (((((-1.0)) * (p.p162))) + (p.p159)));
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

        if ((!s.b[448]) && s.b[457]) {
            let assign11070_ad_e17009: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, assign11070_ad_e17009, (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(assign11070_ad_e17009, ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
        }

        if ((!s.b[448]) && s.b[457]) {
            s.store_div_scaled_inputs_indices(136, 234, 1.0, 83, 2.0);
        }

        s.b[459] = (s.v[136] < 200.0);
        s.store_scalar(459, if s.b[459] { 1.0 } else { 0.0 });

        if (((!s.b[448]) && s.b[457]) && s.b[459]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(234), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[448]) && s.b[457]) && (!s.b[459])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(234), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[448]) && s.b[457]) {
            s.store_sub_div_rhs_indices(100, 234, 153, 99);
        }

        s.b[460] = ((((s.v[100] - s.v[234])) as f64).abs() > 1e-19);
        s.store_scalar(460, if s.b[460] { 1.0 } else { 0.0 });

        if (((!s.b[448]) && s.b[457]) && s.b[460]) {
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

        if (((!s.b[448]) && s.b[457]) && s.b[460]) {
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

        if (((!s.b[448]) && s.b[457]) && s.b[460]) {
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

    }

    pub(super) fn stamp_reactive_block_8(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[448]) && s.b[457]) && s.b[460]) {
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

        if (((!s.b[448]) && s.b[457]) && s.b[460]) {
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

        if (((!s.b[448]) && s.b[457]) && (!s.b[460])) {
            s.copy_ad(236, 100);
        }

        if ((!s.b[448]) && s.b[457]) {
            s.store_scalar(243, 0.0);
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

        if ((!s.b[448]) && s.b[457]) {
            let assign11700_ad_e18105: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, assign11700_ad_e18105, (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(assign11700_ad_e18105, ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
        }

        if ((!s.b[448]) && s.b[457]) {
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[461] = (s.v[136] < 200.0);
        s.store_scalar(461, if s.b[461] { 1.0 } else { 0.0 });

        if (((!s.b[448]) && s.b[457]) && s.b[461]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[448]) && s.b[457]) && (!s.b[461])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[448]) && s.b[457]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[462] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.store_scalar(462, if s.b[462] { 1.0 } else { 0.0 });

        if (((!s.b[448]) && s.b[457]) && s.b[462]) {
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

        if (((!s.b[448]) && s.b[457]) && s.b[462]) {
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

        if (((!s.b[448]) && s.b[457]) && s.b[462]) {
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

        if (((!s.b[448]) && s.b[457]) && s.b[462]) {
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

        if (((!s.b[448]) && s.b[457]) && s.b[462]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p169, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p170, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(237, 128, 86);
        }

        if (((!s.b[448]) && s.b[457]) && (!s.b[462])) {
            s.store_add(237, 100, 86);
        }

        if ((!s.b[448]) && s.b[457]) {
            s.store_scaled_add(238, 236, 237, 0.5);
            s.store_sub(239, 237, 236);
            s.store_sub(90, 237, 236);
            s.store_add_scaled_inputs3_indices(91, 234, 1.0, 83, 1.0, 238, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 235, s.ad_value(234), ((p.p4 * p.p5) * p.p161), s.ad_value(238), (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p236) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p235, 1.0);
            s.store_div_from_scalar(190, p.p234, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p160);
            s.store_mul_add_scaled_inputs3_offset_rhs(240, 191, s.ad_value(234), ((p.p4 * p.p5) * p.p161), s.ad_value(238), (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);
            s.store_add_scaled_inputs3_indices(136, 234, 1.0, 83, 1.0, 238, -1.0);
            s.store_add_scaled_inputs(90, 236, 0.3333333333333333, 237, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(239)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(239)), 239, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(241, 191, 234, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p161) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p161) * p.p5) * 0.5)));
        }

        if ((!s.b[448]) && (!s.b[457])) {
            s.store_scalar(240, 0.0);
            s.store_scalar(241, 0.0);
        }

        s.b[463] = (p.p149 == 0.0);
        s.store_scalar(463, if s.b[463] { 1.0 } else { 0.0 });

        s.b[464] = (p.p152 != 0.0);
        s.store_scalar(464, if s.b[464] { 1.0 } else { 0.0 });

        if (s.b[463] && s.b[464]) {
            s.store_voltage(57, ctx, nodes, Some(16), Some(15));
        }

        s.b[465] = (p.p152 == 1.0);
        s.store_scalar(465, if s.b[465] { 1.0 } else { 0.0 });

        if ((s.b[463] && s.b[464]) && s.b[465]) {
            s.store_voltage(58, ctx, nodes, Some(9), Some(15));
            s.store_voltage(59, ctx, nodes, Some(9), Some(16));
        }

        if ((s.b[463] && s.b[464]) && (!s.b[465])) {
            s.store_voltage(58, ctx, nodes, Some(2), Some(15));
            s.store_voltage(59, ctx, nodes, Some(2), Some(16));
        }

        if (s.b[463] && s.b[464]) {
            s.store_scalar(56, 1.0);
        }

        s.b[466] = (s.v[57] < 0.0);
        s.store_scalar(466, if s.b[466] { 1.0 } else { 0.0 });

        if ((s.b[463] && s.b[464]) && s.b[466]) {
            s.store_scalar(56, (-1.0));
            s.store_mul(255, 56, 57);
        }

    }

    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[463] && s.b[464]) && s.b[466]) {
            s.copy_ad(254, 59);
        }

        if ((s.b[463] && s.b[464]) && (!s.b[466])) {
            s.copy_ad(255, 57);
            s.copy_ad(254, 58);
        }

        if (s.b[463] && s.b[464]) {
            s.store_offset_sqrt_ad(256, A::offset(A::square(s.ad_value(255)), 0.01), (-0.1));
            s.store_offset_scaled(146, 256, p.p179, (1.0 + p.p178));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_ad(88, A::sub_from_scalar(p.p172, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p175)), (((-1.0)) * (p.p175)))), A::div_scaled_inputs(s.ad_value(256), (p.p181 * p.p180), A::sqrt_square_offset(s.ad_value(256), (p.p181 * p.p181)), 1.0));
            s.store_scalar(247, (p.p9 / p.p173));
            s.store_div_from_scalar_scaled_mul(136, p.p174, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p171), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 254, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(254), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);
            s.store_sub(246, 160, 88);
            s.store_div_scaled_inputs_indices(84, 247, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 247, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 246, 246, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
        }

        if (s.b[463] && s.b[464]) {
            let assign12620_ad_e19511: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, assign12620_ad_e19511, (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(assign12620_ad_e19511, ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
        }

        if (s.b[463] && s.b[464]) {
            s.store_div_scaled_inputs_indices(136, 246, 1.0, 83, 2.0);
        }

        s.b[467] = (s.v[136] < 200.0);
        s.store_scalar(467, if s.b[467] { 1.0 } else { 0.0 });

        if ((s.b[463] && s.b[464]) && s.b[467]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(246), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[463] && s.b[464]) && (!s.b[467])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(246), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[463] && s.b[464]) {
            s.store_sub_div_rhs_indices(100, 246, 153, 99);
        }

        s.b[468] = ((((s.v[100] - s.v[246])) as f64).abs() > 1e-19);
        s.store_scalar(468, if s.b[468] { 1.0 } else { 0.0 });

        if ((s.b[463] && s.b[464]) && s.b[468]) {
            s.store_sub(101, 246, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if ((s.b[463] && s.b[464]) && s.b[468]) {
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

        if ((s.b[463] && s.b[464]) && s.b[468]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 246, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p182, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p183, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[463] && s.b[464]) && s.b[468]) {
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

        if ((s.b[463] && s.b[464]) && s.b[468]) {
            s.store_scaled_mul(121, 136, 137, p.p182);
            s.store_scaled_mul(122, 136, 137, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(248, 128);
        }

        if ((s.b[463] && s.b[464]) && (!s.b[468])) {
            s.copy_ad(248, 100);
        }

        if (s.b[463] && s.b[464]) {
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p176);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p177);
            s.store_mul_scaled_abs_ad_rhs(136, 247, 1.0 / (p.p9), A::sub(s.ad_value(246), s.ad_value(248)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(248)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 246, 246, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p174, 136, p.p174, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(255), s.ad_value(85)), p.p18);
            s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));
            s.store_mul(86, 255, 90);
            s.store_sub(39, 246, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
        }

        if (s.b[463] && s.b[464]) {
            let assign13240_ad_e20540: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, assign13240_ad_e20540, (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(assign13240_ad_e20540, ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
        }

        if (s.b[463] && s.b[464]) {
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[469] = (s.v[136] < 200.0);
        s.store_scalar(469, if s.b[469] { 1.0 } else { 0.0 });

        if ((s.b[463] && s.b[464]) && s.b[469]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[463] && s.b[464]) && (!s.b[469])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[463] && s.b[464]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[470] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.store_scalar(470, if s.b[470] { 1.0 } else { 0.0 });

        if ((s.b[463] && s.b[464]) && s.b[470]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if ((s.b[463] && s.b[464]) && s.b[470]) {
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

        if ((s.b[463] && s.b[464]) && s.b[470]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
        }

    }

    pub(super) fn stamp_reactive_block_10(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[463] && s.b[464]) && s.b[470]) {
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p182, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p183, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[463] && s.b[464]) && s.b[470]) {
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

        if ((s.b[463] && s.b[464]) && s.b[470]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p182, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p183, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(249, 128, 86);
        }

        if ((s.b[463] && s.b[464]) && (!s.b[470])) {
            s.store_add(249, 100, 86);
        }

        if (s.b[463] && s.b[464]) {
            s.store_scaled_add(250, 248, 249, 0.5);
            s.store_sub(251, 249, 248);
            s.store_mul_add_scaled_inputs3_offset_rhs(135, 251, s.ad_value(246), 1.0, s.ad_value(250), (-1.0), s.ad_value(83), 1.0, 0.0);
            s.store_mul_scaled_abs_ad_rhs(136, 247, 1.0 / (p.p9), A::sub(s.ad_value(246), s.ad_value(250)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);
            s.store_scaled_mul(96, 95, 247, (p.p4 * (p.p5 * 1.0 / (p.p174))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(256), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(251), (p.p25 * p.p25), s.ad_value(251)), 1.0);
            s.store_div(93, 98, 92);
            s.store_sub(90, 249, 248);
            s.store_add_scaled_inputs3_indices(91, 246, 1.0, 83, 1.0, 250, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 247, s.ad_value(246), ((p.p4 * p.p5) * p.p174), s.ad_value(250), (((-1.0)) * (((p.p4 * p.p5) * p.p174))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p174), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p239) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p238, 1.0);
            s.store_div_from_scalar(190, p.p237, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p173);
            s.store_mul_add_scaled_inputs3_offset_rhs(252, 191, s.ad_value(246), ((p.p4 * p.p5) * p.p174), s.ad_value(250), (((-1.0)) * (((p.p4 * p.p5) * p.p174))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p174), 0.0);
            s.store_add_scaled_inputs3_indices(136, 246, 1.0, 83, 1.0, 250, -1.0);
            s.store_add_scaled_inputs(90, 248, 0.3333333333333333, 249, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(251)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(251)), 251, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(253, 191, 246, (-(((p.p4 * p.p174) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p174) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p174) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p174) * p.p5) * 0.5)));
        }

        s.b[471] = (s.v[56] < 0.0);
        s.store_scalar(471, if s.b[471] { 1.0 } else { 0.0 });

        if ((s.b[463] && s.b[464]) && s.b[471]) {
            s.store_sub_scaled_inputs(253, 252, (-1.0), 253, 1.0);
        }

        if (s.b[463] && (!s.b[464])) {
            s.store_scalar(252, 0.0);
            s.store_scalar(253, 0.0);
        }

        s.b[472] = (p.p152 != 0.0);
        s.store_scalar(472, if s.b[472] { 1.0 } else { 0.0 });

        s.b[473] = (p.p152 == 1.0);
        s.store_scalar(473, if s.b[473] { 1.0 } else { 0.0 });

        if (((!s.b[463]) && s.b[472]) && s.b[473]) {
            s.store_voltage(58, ctx, nodes, Some(9), Some(7));
        }

        if (((!s.b[463]) && s.b[472]) && (!s.b[473])) {
            s.store_voltage(58, ctx, nodes, Some(2), Some(7));
        }

        if ((!s.b[463]) && s.b[472]) {
            s.copy_ad(254, 58);
            s.store_scalar(146, (1.0 + p.p178));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_from_scalar_ad(88, p.p172, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p175)), (((-1.0)) * (p.p175))));
            s.store_scalar(247, (p.p9 / p.p173));
            s.store_div_from_scalar_scaled_mul(136, p.p174, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p171), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 254, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(254), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);
            s.store_sub(246, 160, 88);
            s.store_div_scaled_inputs_indices(84, 247, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 247, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 246, 246, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
        }

        if ((!s.b[463]) && s.b[472]) {
            let assign14160_ad_e21938: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, assign14160_ad_e21938, (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(assign14160_ad_e21938, ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
        }

        if ((!s.b[463]) && s.b[472]) {
            s.store_div_scaled_inputs_indices(136, 246, 1.0, 83, 2.0);
        }

        s.b[474] = (s.v[136] < 200.0);
        s.store_scalar(474, if s.b[474] { 1.0 } else { 0.0 });

        if (((!s.b[463]) && s.b[472]) && s.b[474]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(246), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[463]) && s.b[472]) && (!s.b[474])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(246), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[463]) && s.b[472]) {
            s.store_sub_div_rhs_indices(100, 246, 153, 99);
        }

        s.b[475] = ((((s.v[100] - s.v[246])) as f64).abs() > 1e-19);
        s.store_scalar(475, if s.b[475] { 1.0 } else { 0.0 });

        if (((!s.b[463]) && s.b[472]) && s.b[475]) {
            s.store_sub(101, 246, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[463]) && s.b[472]) && s.b[475]) {
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

        if (((!s.b[463]) && s.b[472]) && s.b[475]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 246, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p182, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p183, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

    }

    pub(super) fn stamp_reactive_block_11(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[463]) && s.b[472]) && s.b[475]) {
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

        if (((!s.b[463]) && s.b[472]) && s.b[475]) {
            s.store_scaled_mul(121, 136, 137, p.p182);
            s.store_scaled_mul(122, 136, 137, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(248, 128);
        }

        if (((!s.b[463]) && s.b[472]) && (!s.b[475])) {
            s.copy_ad(248, 100);
        }

        if ((!s.b[463]) && s.b[472]) {
            s.store_scalar(255, 0.0);
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p176);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p177);
            s.store_mul_scaled_abs_ad_rhs(136, 247, 1.0 / (p.p9), A::sub(s.ad_value(246), s.ad_value(248)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(248)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 246, 246, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p174, 136, p.p174, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(255), s.ad_value(85)), p.p18);
            s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));
            s.store_mul(86, 255, 90);
            s.store_sub(39, 246, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
        }

        if ((!s.b[463]) && s.b[472]) {
            let assign14790_ad_e23034: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, assign14790_ad_e23034, (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(assign14790_ad_e23034, ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
        }

        if ((!s.b[463]) && s.b[472]) {
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[476] = (s.v[136] < 200.0);
        s.store_scalar(476, if s.b[476] { 1.0 } else { 0.0 });

        if (((!s.b[463]) && s.b[472]) && s.b[476]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[463]) && s.b[472]) && (!s.b[476])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[463]) && s.b[472]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[477] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.store_scalar(477, if s.b[477] { 1.0 } else { 0.0 });

        if (((!s.b[463]) && s.b[472]) && s.b[477]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[463]) && s.b[472]) && s.b[477]) {
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

        if (((!s.b[463]) && s.b[472]) && s.b[477]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p182, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p183, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if (((!s.b[463]) && s.b[472]) && s.b[477]) {
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

        if (((!s.b[463]) && s.b[472]) && s.b[477]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p182, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p183, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(249, 128, 86);
        }

        if (((!s.b[463]) && s.b[472]) && (!s.b[477])) {
            s.store_add(249, 100, 86);
        }

        if ((!s.b[463]) && s.b[472]) {
            s.store_scaled_add(250, 248, 249, 0.5);
            s.store_sub(251, 249, 248);
            s.store_sub(90, 249, 248);
            s.store_add_scaled_inputs3_indices(91, 246, 1.0, 83, 1.0, 250, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 247, s.ad_value(246), ((p.p4 * p.p5) * p.p174), s.ad_value(250), (((-1.0)) * (((p.p4 * p.p5) * p.p174))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p174), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p239) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p238, 1.0);
            s.store_div_from_scalar(190, p.p237, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p173);
            s.store_mul_add_scaled_inputs3_offset_rhs(252, 191, s.ad_value(246), ((p.p4 * p.p5) * p.p174), s.ad_value(250), (((-1.0)) * (((p.p4 * p.p5) * p.p174))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p174), 0.0);
            s.store_add_scaled_inputs3_indices(136, 246, 1.0, 83, 1.0, 250, -1.0);
            s.store_add_scaled_inputs(90, 248, 0.3333333333333333, 249, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(251)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(251)), 251, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(253, 191, 246, (-(((p.p4 * p.p174) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p174) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p174) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p174) * p.p5) * 0.5)));
        }

        if ((!s.b[463]) && (!s.b[472])) {
            s.store_scalar(252, 0.0);
            s.store_scalar(253, 0.0);
        }

        s.b[478] = (p.p149 == 0.0);
        s.store_scalar(478, if s.b[478] { 1.0 } else { 0.0 });

        s.b[479] = (p.p153 != 0.0);
        s.store_scalar(479, if s.b[479] { 1.0 } else { 0.0 });

        if (s.b[478] && s.b[479]) {
            s.store_voltage(61, ctx, nodes, Some(19), Some(20));
        }

        s.b[480] = (p.p153 == 1.0);
        s.store_scalar(480, if s.b[480] { 1.0 } else { 0.0 });

        if ((s.b[478] && s.b[479]) && s.b[480]) {
            s.store_voltage(62, ctx, nodes, Some(9), Some(20));
            s.store_voltage(63, ctx, nodes, Some(9), Some(19));
        }

        if ((s.b[478] && s.b[479]) && (!s.b[480])) {
            s.store_voltage(62, ctx, nodes, Some(2), Some(20));
            s.store_voltage(63, ctx, nodes, Some(2), Some(19));
        }

        if (s.b[478] && s.b[479]) {
            s.store_scalar(60, 1.0);
        }

        s.b[481] = (s.v[61] < 0.0);
        s.store_scalar(481, if s.b[481] { 1.0 } else { 0.0 });

        if ((s.b[478] && s.b[479]) && s.b[481]) {
            s.store_scalar(60, (-1.0));
            s.store_mul(267, 60, 61);
        }

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[478] && s.b[479]) && s.b[481]) {
            s.copy_ad(266, 63);
        }

        if ((s.b[478] && s.b[479]) && (!s.b[481])) {
            s.copy_ad(267, 61);
            s.copy_ad(266, 62);
        }

        if (s.b[478] && s.b[479]) {
            s.store_offset_sqrt_ad(268, A::offset(A::square(s.ad_value(267)), 0.01), (-0.1));
            s.store_offset_scaled(146, 268, p.p179, (1.0 + p.p178));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_ad(88, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p175)), (((((-1.0)) * (p.p175))) + (p.p172))), A::div_scaled_inputs(s.ad_value(268), (p.p181 * p.p180), A::sqrt_square_offset(s.ad_value(268), (p.p181 * p.p181)), 1.0));
            s.store_scalar(259, (p.p9 / p.p173));
            s.store_div_from_scalar_scaled_mul(136, p.p174, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p171), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 266, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(266), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);
            s.store_sub(258, 160, 88);
            s.store_div_scaled_inputs_indices(84, 259, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 259, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 258, 258, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
        }

        if (s.b[478] && s.b[479]) {
            let assign15710_ad_e24440: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, assign15710_ad_e24440, (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(assign15710_ad_e24440, ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
        }

        if (s.b[478] && s.b[479]) {
            s.store_div_scaled_inputs_indices(136, 258, 1.0, 83, 2.0);
        }

        s.b[482] = (s.v[136] < 200.0);
        s.store_scalar(482, if s.b[482] { 1.0 } else { 0.0 });

        if ((s.b[478] && s.b[479]) && s.b[482]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(258), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[478] && s.b[479]) && (!s.b[482])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(258), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[478] && s.b[479]) {
            s.store_sub_div_rhs_indices(100, 258, 153, 99);
        }

        s.b[483] = ((((s.v[100] - s.v[258])) as f64).abs() > 1e-19);
        s.store_scalar(483, if s.b[483] { 1.0 } else { 0.0 });

        if ((s.b[478] && s.b[479]) && s.b[483]) {
            s.store_sub(101, 258, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if ((s.b[478] && s.b[479]) && s.b[483]) {
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

        if ((s.b[478] && s.b[479]) && s.b[483]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 258, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p182, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p183, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[478] && s.b[479]) && s.b[483]) {
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

        if ((s.b[478] && s.b[479]) && s.b[483]) {
            s.store_scaled_mul(121, 136, 137, p.p182);
            s.store_scaled_mul(122, 136, 137, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(260, 128);
        }

        if ((s.b[478] && s.b[479]) && (!s.b[483])) {
            s.copy_ad(260, 100);
        }

        if (s.b[478] && s.b[479]) {
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p176);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p177);
            s.store_mul_scaled_abs_ad_rhs(136, 259, 1.0 / (p.p9), A::sub(s.ad_value(258), s.ad_value(260)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(260)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 258, 258, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p174, 136, p.p174, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(267), s.ad_value(85)), p.p18);
            s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));
            s.store_mul(86, 267, 90);
            s.store_sub(39, 258, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
        }

        if (s.b[478] && s.b[479]) {
            let assign16330_ad_e25469: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, assign16330_ad_e25469, (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(assign16330_ad_e25469, ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
        }

        if (s.b[478] && s.b[479]) {
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[484] = (s.v[136] < 200.0);
        s.store_scalar(484, if s.b[484] { 1.0 } else { 0.0 });

        if ((s.b[478] && s.b[479]) && s.b[484]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[478] && s.b[479]) && (!s.b[484])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[478] && s.b[479]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[485] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.store_scalar(485, if s.b[485] { 1.0 } else { 0.0 });

        if ((s.b[478] && s.b[479]) && s.b[485]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if ((s.b[478] && s.b[479]) && s.b[485]) {
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

        if ((s.b[478] && s.b[479]) && s.b[485]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[478] && s.b[479]) && s.b[485]) {
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p182, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p183, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[478] && s.b[479]) && s.b[485]) {
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

        if ((s.b[478] && s.b[479]) && s.b[485]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p182, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p183, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(261, 128, 86);
        }

        if ((s.b[478] && s.b[479]) && (!s.b[485])) {
            s.store_add(261, 100, 86);
        }

        if (s.b[478] && s.b[479]) {
            s.store_scaled_add(262, 260, 261, 0.5);
            s.store_sub(263, 261, 260);
            s.store_mul_add_scaled_inputs3_offset_rhs(135, 263, s.ad_value(258), 1.0, s.ad_value(262), (-1.0), s.ad_value(83), 1.0, 0.0);
            s.store_mul_scaled_abs_ad_rhs(136, 259, 1.0 / (p.p9), A::sub(s.ad_value(258), s.ad_value(262)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);
            s.store_scaled_mul(96, 95, 259, (p.p4 * (p.p5 * 1.0 / (p.p174))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(268), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(263), (p.p25 * p.p25), s.ad_value(263)), 1.0);
            s.store_div(93, 98, 92);
            s.store_sub(90, 261, 260);
            s.store_add_scaled_inputs3_indices(91, 258, 1.0, 83, 1.0, 262, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 259, s.ad_value(258), ((p.p4 * p.p5) * p.p174), s.ad_value(262), (((-1.0)) * (((p.p4 * p.p5) * p.p174))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p174), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p239) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p238, 1.0);
            s.store_div_from_scalar(190, p.p237, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p173);
            s.store_mul_add_scaled_inputs3_offset_rhs(264, 191, s.ad_value(258), ((p.p4 * p.p5) * p.p174), s.ad_value(262), (((-1.0)) * (((p.p4 * p.p5) * p.p174))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p174), 0.0);
            s.store_add_scaled_inputs3_indices(136, 258, 1.0, 83, 1.0, 262, -1.0);
            s.store_add_scaled_inputs(90, 260, 0.3333333333333333, 261, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(263)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(263)), 263, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(265, 191, 258, (-(((p.p4 * p.p174) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p174) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p174) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p174) * p.p5) * 0.5)));
        }

        s.b[486] = (s.v[60] < 0.0);
        s.store_scalar(486, if s.b[486] { 1.0 } else { 0.0 });

        if ((s.b[478] && s.b[479]) && s.b[486]) {
            s.store_sub_scaled_inputs(265, 264, (-1.0), 265, 1.0);
        }

        if (s.b[478] && (!s.b[479])) {
            s.store_scalar(264, 0.0);
            s.store_scalar(265, 0.0);
        }

        s.b[487] = (p.p153 != 0.0);
        s.store_scalar(487, if s.b[487] { 1.0 } else { 0.0 });

        s.b[488] = (p.p153 == 1.0);
        s.store_scalar(488, if s.b[488] { 1.0 } else { 0.0 });

        if (((!s.b[478]) && s.b[487]) && s.b[488]) {
            s.store_voltage(62, ctx, nodes, Some(9), Some(8));
        }

        if (((!s.b[478]) && s.b[487]) && (!s.b[488])) {
            s.store_voltage(62, ctx, nodes, Some(2), Some(8));
        }

        if ((!s.b[478]) && s.b[487]) {
            s.copy_ad(266, 62);
            s.store_scalar(146, (1.0 + p.p178));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_offset_scaled(88, 82, ((1.0 / (s.v[35])) * (p.p175)), (((((-1.0)) * (p.p175))) + (p.p172)));
            s.store_scalar(259, (p.p9 / p.p173));
            s.store_div_from_scalar_scaled_mul(136, p.p174, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p171), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 266, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(266), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);
            s.store_sub(258, 160, 88);
            s.store_div_scaled_inputs_indices(84, 259, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 259, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 258, 258, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
        }

        if ((!s.b[478]) && s.b[487]) {
            let assign17250_ad_e26867: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, assign17250_ad_e26867, (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(assign17250_ad_e26867, ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_div_scaled_inputs_indices(136, 258, 1.0, 83, 2.0);
        }

        s.b[489] = (s.v[136] < 200.0);
        s.store_scalar(489, if s.b[489] { 1.0 } else { 0.0 });

        if (((!s.b[478]) && s.b[487]) && s.b[489]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(258), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[478]) && s.b[487]) && (!s.b[489])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(258), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_sub_div_rhs_indices(100, 258, 153, 99);
        }

        s.b[490] = ((((s.v[100] - s.v[258])) as f64).abs() > 1e-19);
        s.store_scalar(490, if s.b[490] { 1.0 } else { 0.0 });

        if (((!s.b[478]) && s.b[487]) && s.b[490]) {
            s.store_sub(101, 258, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[478]) && s.b[487]) && s.b[490]) {
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

        if (((!s.b[478]) && s.b[487]) && s.b[490]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 258, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p182, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p183, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[478]) && s.b[487]) && s.b[490]) {
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

        if (((!s.b[478]) && s.b[487]) && s.b[490]) {
            s.store_scaled_mul(121, 136, 137, p.p182);
            s.store_scaled_mul(122, 136, 137, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(260, 128);
        }

        if (((!s.b[478]) && s.b[487]) && (!s.b[490])) {
            s.copy_ad(260, 100);
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_scalar(267, 0.0);
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p176);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p177);
            s.store_mul_scaled_abs_ad_rhs(136, 259, 1.0 / (p.p9), A::sub(s.ad_value(258), s.ad_value(260)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(260)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 258, 258, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p174, 136, p.p174, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(267), s.ad_value(85)), p.p18);
            s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));
            s.store_mul(86, 267, 90);
            s.store_sub(39, 258, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
        }

        if ((!s.b[478]) && s.b[487]) {
            let assign17880_ad_e27963: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, assign17880_ad_e27963, (-(p.p182 / 3.0)), A::add_scaled_offset_product_rhs(assign17880_ad_e27963, ((2.0 * p.p182) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[491] = (s.v[136] < 200.0);
        s.store_scalar(491, if s.b[491] { 1.0 } else { 0.0 });

        if (((!s.b[478]) && s.b[487]) && s.b[491]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[478]) && s.b[487]) && (!s.b[491])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[492] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.store_scalar(492, if s.b[492] { 1.0 } else { 0.0 });

        if (((!s.b[478]) && s.b[487]) && s.b[492]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p182);
            s.store_scaled_mul(103, 136, 90, p.p183);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[478]) && s.b[487]) && s.b[492]) {
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

        if (((!s.b[478]) && s.b[487]) && s.b[492]) {
            s.store_scaled_mul(107, 136, 91, p.p182);
            s.store_scaled_mul(108, 136, 91, p.p183);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p182, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p183, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if (((!s.b[478]) && s.b[487]) && s.b[492]) {
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

        if (((!s.b[478]) && s.b[487]) && s.b[492]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p182, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p183, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(261, 128, 86);
        }

        if (((!s.b[478]) && s.b[487]) && (!s.b[492])) {
            s.store_add(261, 100, 86);
        }

        if ((!s.b[478]) && s.b[487]) {
            s.store_scaled_add(262, 260, 261, 0.5);
            s.store_sub(263, 261, 260);
            s.store_sub(90, 261, 260);
            s.store_add_scaled_inputs3_indices(91, 258, 1.0, 83, 1.0, 262, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 259, s.ad_value(258), ((p.p4 * p.p5) * p.p174), s.ad_value(262), (((-1.0)) * (((p.p4 * p.p5) * p.p174))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p174), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p239) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p238, 1.0);
            s.store_div_from_scalar(190, p.p237, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p173);
            s.store_mul_add_scaled_inputs3_offset_rhs(264, 191, s.ad_value(258), ((p.p4 * p.p5) * p.p174), s.ad_value(262), (((-1.0)) * (((p.p4 * p.p5) * p.p174))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p174), 0.0);
            s.store_add_scaled_inputs3_indices(136, 258, 1.0, 83, 1.0, 262, -1.0);
            s.store_add_scaled_inputs(90, 260, 0.3333333333333333, 261, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(263)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(263)), 263, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(265, 191, 258, (-(((p.p4 * p.p174) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p174) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p174) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p174) * p.p5) * 0.5)));
        }

        if ((!s.b[478]) && (!s.b[487])) {
            s.store_scalar(264, 0.0);
            s.store_scalar(265, 0.0);
        }

        s.b[493] = (p.p149 == 0.0);
        s.store_scalar(493, if s.b[493] { 1.0 } else { 0.0 });

        s.b[494] = (p.p154 != 0.0);
        s.store_scalar(494, if s.b[494] { 1.0 } else { 0.0 });

        if (s.b[493] && s.b[494]) {
            s.store_voltage(65, ctx, nodes, Some(17), Some(16));
        }

        s.b[495] = (p.p154 == 1.0);
        s.store_scalar(495, if s.b[495] { 1.0 } else { 0.0 });

        if ((s.b[493] && s.b[494]) && s.b[495]) {
            s.store_voltage(66, ctx, nodes, Some(9), Some(16));
            s.store_voltage(67, ctx, nodes, Some(9), Some(17));
        }

        if ((s.b[493] && s.b[494]) && (!s.b[495])) {
            s.store_voltage(66, ctx, nodes, Some(2), Some(16));
            s.store_voltage(67, ctx, nodes, Some(2), Some(17));
        }

        if (s.b[493] && s.b[494]) {
            s.store_scalar(64, 1.0);
        }

        s.b[496] = (s.v[65] < 0.0);
        s.store_scalar(496, if s.b[496] { 1.0 } else { 0.0 });

        if ((s.b[493] && s.b[494]) && s.b[496]) {
            s.store_scalar(64, (-1.0));
            s.store_mul(279, 64, 65);
        }

    }
}
