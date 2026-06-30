#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_112(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_guard461_slot: &mut f64,
        var_vsch_slot: &mut f64,
        var_vsch_db0_slot: &mut f64,
        var_vsch_db1_slot: &mut f64,
        var_vsch_db10_slot: &mut f64,
        var_vsch_db11_slot: &mut f64,
        var_vsch_db12_slot: &mut f64,
        var_vsch_db13_slot: &mut f64,
        var_vsch_db14_slot: &mut f64,
        var_vsch_db15_slot: &mut f64,
        var_vsch_db16_slot: &mut f64,
        var_vsch_db17_slot: &mut f64,
        var_vsch_db18_slot: &mut f64,
        var_vsch_db19_slot: &mut f64,
        var_vsch_db2_slot: &mut f64,
        var_vsch_db20_slot: &mut f64,
        var_vsch_db21_slot: &mut f64,
        var_vsch_db22_slot: &mut f64,
        var_vsch_db23_slot: &mut f64,
        var_vsch_db24_slot: &mut f64,
        var_vsch_db25_slot: &mut f64,
        var_vsch_db26_slot: &mut f64,
        var_vsch_db27_slot: &mut f64,
        var_vsch_db28_slot: &mut f64,
        var_vsch_db29_slot: &mut f64,
        var_vsch_db3_slot: &mut f64,
        var_vsch_db30_slot: &mut f64,
        var_vsch_db31_slot: &mut f64,
        var_vsch_db32_slot: &mut f64,
        var_vsch_db33_slot: &mut f64,
        var_vsch_db34_slot: &mut f64,
        var_vsch_db35_slot: &mut f64,
        var_vsch_db4_slot: &mut f64,
        var_vsch_db5_slot: &mut f64,
        var_vsch_db6_slot: &mut f64,
        var_vsch_db7_slot: &mut f64,
        var_vsch_db8_slot: &mut f64,
        var_vsch_db9_slot: &mut f64,
        var_vsch_dn0_slot: &mut f64,
        var_vsch_dn1_slot: &mut f64,
        var_vsch_dn10_slot: &mut f64,
        var_vsch_dn11_slot: &mut f64,
        var_vsch_dn12_slot: &mut f64,
        var_vsch_dn13_slot: &mut f64,
        var_vsch_dn14_slot: &mut f64,
        var_vsch_dn15_slot: &mut f64,
        var_vsch_dn16_slot: &mut f64,
        var_vsch_dn17_slot: &mut f64,
        var_vsch_dn18_slot: &mut f64,
        var_vsch_dn19_slot: &mut f64,
        var_vsch_dn2_slot: &mut f64,
        var_vsch_dn20_slot: &mut f64,
        var_vsch_dn21_slot: &mut f64,
        var_vsch_dn22_slot: &mut f64,
        var_vsch_dn23_slot: &mut f64,
        var_vsch_dn24_slot: &mut f64,
        var_vsch_dn25_slot: &mut f64,
        var_vsch_dn26_slot: &mut f64,
        var_vsch_dn27_slot: &mut f64,
        var_vsch_dn28_slot: &mut f64,
        var_vsch_dn29_slot: &mut f64,
        var_vsch_dn3_slot: &mut f64,
        var_vsch_dn4_slot: &mut f64,
        var_vsch_dn5_slot: &mut f64,
        var_vsch_dn6_slot: &mut f64,
        var_vsch_dn7_slot: &mut f64,
        var_vsch_dn8_slot: &mut f64,
        var_vsch_dn9_slot: &mut f64,
        var_vschfc2_slot: &mut f64,
        var_vschfc2_db0_slot: &mut f64,
        var_vschfc2_db1_slot: &mut f64,
        var_vschfc2_db10_slot: &mut f64,
        var_vschfc2_db11_slot: &mut f64,
        var_vschfc2_db12_slot: &mut f64,
        var_vschfc2_db13_slot: &mut f64,
        var_vschfc2_db14_slot: &mut f64,
        var_vschfc2_db15_slot: &mut f64,
        var_vschfc2_db16_slot: &mut f64,
        var_vschfc2_db17_slot: &mut f64,
        var_vschfc2_db18_slot: &mut f64,
        var_vschfc2_db19_slot: &mut f64,
        var_vschfc2_db2_slot: &mut f64,
        var_vschfc2_db20_slot: &mut f64,
        var_vschfc2_db21_slot: &mut f64,
        var_vschfc2_db22_slot: &mut f64,
        var_vschfc2_db23_slot: &mut f64,
        var_vschfc2_db24_slot: &mut f64,
        var_vschfc2_db25_slot: &mut f64,
        var_vschfc2_db26_slot: &mut f64,
        var_vschfc2_db27_slot: &mut f64,
        var_vschfc2_db28_slot: &mut f64,
        var_vschfc2_db29_slot: &mut f64,
        var_vschfc2_db3_slot: &mut f64,
        var_vschfc2_db30_slot: &mut f64,
        var_vschfc2_db31_slot: &mut f64,
        var_vschfc2_db32_slot: &mut f64,
        var_vschfc2_db33_slot: &mut f64,
        var_vschfc2_db34_slot: &mut f64,
        var_vschfc2_db35_slot: &mut f64,
        var_vschfc2_db4_slot: &mut f64,
        var_vschfc2_db5_slot: &mut f64,
        var_vschfc2_db6_slot: &mut f64,
        var_vschfc2_db7_slot: &mut f64,
        var_vschfc2_db8_slot: &mut f64,
        var_vschfc2_db9_slot: &mut f64,
        var_vschfc2_dn0_slot: &mut f64,
        var_vschfc2_dn1_slot: &mut f64,
        var_vschfc2_dn10_slot: &mut f64,
        var_vschfc2_dn11_slot: &mut f64,
        var_vschfc2_dn12_slot: &mut f64,
        var_vschfc2_dn13_slot: &mut f64,
        var_vschfc2_dn14_slot: &mut f64,
        var_vschfc2_dn15_slot: &mut f64,
        var_vschfc2_dn16_slot: &mut f64,
        var_vschfc2_dn17_slot: &mut f64,
        var_vschfc2_dn18_slot: &mut f64,
        var_vschfc2_dn19_slot: &mut f64,
        var_vschfc2_dn2_slot: &mut f64,
        var_vschfc2_dn20_slot: &mut f64,
        var_vschfc2_dn21_slot: &mut f64,
        var_vschfc2_dn22_slot: &mut f64,
        var_vschfc2_dn23_slot: &mut f64,
        var_vschfc2_dn24_slot: &mut f64,
        var_vschfc2_dn25_slot: &mut f64,
        var_vschfc2_dn26_slot: &mut f64,
        var_vschfc2_dn27_slot: &mut f64,
        var_vschfc2_dn28_slot: &mut f64,
        var_vschfc2_dn29_slot: &mut f64,
        var_vschfc2_dn3_slot: &mut f64,
        var_vschfc2_dn4_slot: &mut f64,
        var_vschfc2_dn5_slot: &mut f64,
        var_vschfc2_dn6_slot: &mut f64,
        var_vschfc2_dn7_slot: &mut f64,
        var_vschfc2_dn8_slot: &mut f64,
        var_vschfc2_dn9_slot: &mut f64,
        var_vschfc3_slot: &mut f64,
        var_vschfc3_db0_slot: &mut f64,
        var_vschfc3_db1_slot: &mut f64,
        var_vschfc3_db10_slot: &mut f64,
        var_vschfc3_db11_slot: &mut f64,
        var_vschfc3_db12_slot: &mut f64,
        var_vschfc3_db13_slot: &mut f64,
        var_vschfc3_db14_slot: &mut f64,
        var_vschfc3_db15_slot: &mut f64,
        var_vschfc3_db16_slot: &mut f64,
        var_vschfc3_db17_slot: &mut f64,
        var_vschfc3_db18_slot: &mut f64,
        var_vschfc3_db19_slot: &mut f64,
        var_vschfc3_db2_slot: &mut f64,
        var_vschfc3_db20_slot: &mut f64,
        var_vschfc3_db21_slot: &mut f64,
        var_vschfc3_db22_slot: &mut f64,
        var_vschfc3_db23_slot: &mut f64,
        var_vschfc3_db24_slot: &mut f64,
        var_vschfc3_db25_slot: &mut f64,
        var_vschfc3_db26_slot: &mut f64,
        var_vschfc3_db27_slot: &mut f64,
        var_vschfc3_db28_slot: &mut f64,
        var_vschfc3_db29_slot: &mut f64,
        var_vschfc3_db3_slot: &mut f64,
        var_vschfc3_db30_slot: &mut f64,
        var_vschfc3_db31_slot: &mut f64,
        var_vschfc3_db32_slot: &mut f64,
        var_vschfc3_db33_slot: &mut f64,
        var_vschfc3_db34_slot: &mut f64,
        var_vschfc3_db35_slot: &mut f64,
        var_vschfc3_db4_slot: &mut f64,
        var_vschfc3_db5_slot: &mut f64,
        var_vschfc3_db6_slot: &mut f64,
        var_vschfc3_db7_slot: &mut f64,
        var_vschfc3_db8_slot: &mut f64,
        var_vschfc3_db9_slot: &mut f64,
        var_vschfc3_dn0_slot: &mut f64,
        var_vschfc3_dn1_slot: &mut f64,
        var_vschfc3_dn10_slot: &mut f64,
        var_vschfc3_dn11_slot: &mut f64,
        var_vschfc3_dn12_slot: &mut f64,
        var_vschfc3_dn13_slot: &mut f64,
        var_vschfc3_dn14_slot: &mut f64,
        var_vschfc3_dn15_slot: &mut f64,
        var_vschfc3_dn16_slot: &mut f64,
        var_vschfc3_dn17_slot: &mut f64,
        var_vschfc3_dn18_slot: &mut f64,
        var_vschfc3_dn19_slot: &mut f64,
        var_vschfc3_dn2_slot: &mut f64,
        var_vschfc3_dn20_slot: &mut f64,
        var_vschfc3_dn21_slot: &mut f64,
        var_vschfc3_dn22_slot: &mut f64,
        var_vschfc3_dn23_slot: &mut f64,
        var_vschfc3_dn24_slot: &mut f64,
        var_vschfc3_dn25_slot: &mut f64,
        var_vschfc3_dn26_slot: &mut f64,
        var_vschfc3_dn27_slot: &mut f64,
        var_vschfc3_dn28_slot: &mut f64,
        var_vschfc3_dn29_slot: &mut f64,
        var_vschfc3_dn3_slot: &mut f64,
        var_vschfc3_dn4_slot: &mut f64,
        var_vschfc3_dn5_slot: &mut f64,
        var_vschfc3_dn6_slot: &mut f64,
        var_vschfc3_dn7_slot: &mut f64,
        var_vschfc3_dn8_slot: &mut f64,
        var_vschfc3_dn9_slot: &mut f64,
        var_vschfc4_slot: &mut f64,
        var_vschfc4_db0_slot: &mut f64,
        var_vschfc4_db1_slot: &mut f64,
        var_vschfc4_db10_slot: &mut f64,
        var_vschfc4_db11_slot: &mut f64,
        var_vschfc4_db12_slot: &mut f64,
        var_vschfc4_db13_slot: &mut f64,
        var_vschfc4_db14_slot: &mut f64,
        var_vschfc4_db15_slot: &mut f64,
        var_vschfc4_db16_slot: &mut f64,
        var_vschfc4_db17_slot: &mut f64,
        var_vschfc4_db18_slot: &mut f64,
        var_vschfc4_db19_slot: &mut f64,
        var_vschfc4_db2_slot: &mut f64,
        var_vschfc4_db20_slot: &mut f64,
        var_vschfc4_db21_slot: &mut f64,
        var_vschfc4_db22_slot: &mut f64,
        var_vschfc4_db23_slot: &mut f64,
        var_vschfc4_db24_slot: &mut f64,
        var_vschfc4_db25_slot: &mut f64,
        var_vschfc4_db26_slot: &mut f64,
        var_vschfc4_db27_slot: &mut f64,
        var_vschfc4_db28_slot: &mut f64,
        var_vschfc4_db29_slot: &mut f64,
        var_vschfc4_db3_slot: &mut f64,
        var_vschfc4_db30_slot: &mut f64,
        var_vschfc4_db31_slot: &mut f64,
        var_vschfc4_db32_slot: &mut f64,
        var_vschfc4_db33_slot: &mut f64,
        var_vschfc4_db34_slot: &mut f64,
        var_vschfc4_db35_slot: &mut f64,
        var_vschfc4_db4_slot: &mut f64,
        var_vschfc4_db5_slot: &mut f64,
        var_vschfc4_db6_slot: &mut f64,
        var_vschfc4_db7_slot: &mut f64,
        var_vschfc4_db8_slot: &mut f64,
        var_vschfc4_db9_slot: &mut f64,
        var_vschfc4_dn0_slot: &mut f64,
        var_vschfc4_dn1_slot: &mut f64,
        var_vschfc4_dn10_slot: &mut f64,
        var_vschfc4_dn11_slot: &mut f64,
        var_vschfc4_dn12_slot: &mut f64,
        var_vschfc4_dn13_slot: &mut f64,
        var_vschfc4_dn14_slot: &mut f64,
        var_vschfc4_dn15_slot: &mut f64,
        var_vschfc4_dn16_slot: &mut f64,
        var_vschfc4_dn17_slot: &mut f64,
        var_vschfc4_dn18_slot: &mut f64,
        var_vschfc4_dn19_slot: &mut f64,
        var_vschfc4_dn2_slot: &mut f64,
        var_vschfc4_dn20_slot: &mut f64,
        var_vschfc4_dn21_slot: &mut f64,
        var_vschfc4_dn22_slot: &mut f64,
        var_vschfc4_dn23_slot: &mut f64,
        var_vschfc4_dn24_slot: &mut f64,
        var_vschfc4_dn25_slot: &mut f64,
        var_vschfc4_dn26_slot: &mut f64,
        var_vschfc4_dn27_slot: &mut f64,
        var_vschfc4_dn28_slot: &mut f64,
        var_vschfc4_dn29_slot: &mut f64,
        var_vschfc4_dn3_slot: &mut f64,
        var_vschfc4_dn4_slot: &mut f64,
        var_vschfc4_dn5_slot: &mut f64,
        var_vschfc4_dn6_slot: &mut f64,
        var_vschfc4_dn7_slot: &mut f64,
        var_vschfc4_dn8_slot: &mut f64,
        var_vschfc4_dn9_slot: &mut f64,
        var_vschfc5_slot: &mut f64,
        var_vschfc5_db0_slot: &mut f64,
        var_vschfc5_db1_slot: &mut f64,
        var_vschfc5_db10_slot: &mut f64,
        var_vschfc5_db11_slot: &mut f64,
        var_vschfc5_db12_slot: &mut f64,
        var_vschfc5_db13_slot: &mut f64,
        var_vschfc5_db14_slot: &mut f64,
        var_vschfc5_db15_slot: &mut f64,
        var_vschfc5_db16_slot: &mut f64,
        var_vschfc5_db17_slot: &mut f64,
        var_vschfc5_db18_slot: &mut f64,
        var_vschfc5_db19_slot: &mut f64,
        var_vschfc5_db2_slot: &mut f64,
        var_vschfc5_db20_slot: &mut f64,
        var_vschfc5_db21_slot: &mut f64,
        var_vschfc5_db22_slot: &mut f64,
        var_vschfc5_db23_slot: &mut f64,
        var_vschfc5_db24_slot: &mut f64,
        var_vschfc5_db25_slot: &mut f64,
        var_vschfc5_db26_slot: &mut f64,
        var_vschfc5_db27_slot: &mut f64,
        var_vschfc5_db28_slot: &mut f64,
        var_vschfc5_db29_slot: &mut f64,
        var_vschfc5_db3_slot: &mut f64,
        var_vschfc5_db30_slot: &mut f64,
        var_vschfc5_db31_slot: &mut f64,
        var_vschfc5_db32_slot: &mut f64,
        var_vschfc5_db33_slot: &mut f64,
        var_vschfc5_db34_slot: &mut f64,
        var_vschfc5_db35_slot: &mut f64,
        var_vschfc5_db4_slot: &mut f64,
        var_vschfc5_db5_slot: &mut f64,
        var_vschfc5_db6_slot: &mut f64,
        var_vschfc5_db7_slot: &mut f64,
        var_vschfc5_db8_slot: &mut f64,
        var_vschfc5_db9_slot: &mut f64,
        var_vschfc5_dn0_slot: &mut f64,
        var_vschfc5_dn1_slot: &mut f64,
        var_vschfc5_dn10_slot: &mut f64,
        var_vschfc5_dn11_slot: &mut f64,
        var_vschfc5_dn12_slot: &mut f64,
        var_vschfc5_dn13_slot: &mut f64,
        var_vschfc5_dn14_slot: &mut f64,
        var_vschfc5_dn15_slot: &mut f64,
        var_vschfc5_dn16_slot: &mut f64,
        var_vschfc5_dn17_slot: &mut f64,
        var_vschfc5_dn18_slot: &mut f64,
        var_vschfc5_dn19_slot: &mut f64,
        var_vschfc5_dn2_slot: &mut f64,
        var_vschfc5_dn20_slot: &mut f64,
        var_vschfc5_dn21_slot: &mut f64,
        var_vschfc5_dn22_slot: &mut f64,
        var_vschfc5_dn23_slot: &mut f64,
        var_vschfc5_dn24_slot: &mut f64,
        var_vschfc5_dn25_slot: &mut f64,
        var_vschfc5_dn26_slot: &mut f64,
        var_vschfc5_dn27_slot: &mut f64,
        var_vschfc5_dn28_slot: &mut f64,
        var_vschfc5_dn29_slot: &mut f64,
        var_vschfc5_dn3_slot: &mut f64,
        var_vschfc5_dn4_slot: &mut f64,
        var_vschfc5_dn5_slot: &mut f64,
        var_vschfc5_dn6_slot: &mut f64,
        var_vschfc5_dn7_slot: &mut f64,
        var_vschfc5_dn8_slot: &mut f64,
        var_vschfc5_dn9_slot: &mut f64,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let mut var_guard461: f64 = *var_guard461_slot;
        let mut var_vsch: f64 = *var_vsch_slot;
        let mut var_vsch_db0: f64 = *var_vsch_db0_slot;
        let mut var_vsch_db1: f64 = *var_vsch_db1_slot;
        let mut var_vsch_db10: f64 = *var_vsch_db10_slot;
        let mut var_vsch_db11: f64 = *var_vsch_db11_slot;
        let mut var_vsch_db12: f64 = *var_vsch_db12_slot;
        let mut var_vsch_db13: f64 = *var_vsch_db13_slot;
        let mut var_vsch_db14: f64 = *var_vsch_db14_slot;
        let mut var_vsch_db15: f64 = *var_vsch_db15_slot;
        let mut var_vsch_db16: f64 = *var_vsch_db16_slot;
        let mut var_vsch_db17: f64 = *var_vsch_db17_slot;
        let mut var_vsch_db18: f64 = *var_vsch_db18_slot;
        let mut var_vsch_db19: f64 = *var_vsch_db19_slot;
        let mut var_vsch_db2: f64 = *var_vsch_db2_slot;
        let mut var_vsch_db20: f64 = *var_vsch_db20_slot;
        let mut var_vsch_db21: f64 = *var_vsch_db21_slot;
        let mut var_vsch_db22: f64 = *var_vsch_db22_slot;
        let mut var_vsch_db23: f64 = *var_vsch_db23_slot;
        let mut var_vsch_db24: f64 = *var_vsch_db24_slot;
        let mut var_vsch_db25: f64 = *var_vsch_db25_slot;
        let mut var_vsch_db26: f64 = *var_vsch_db26_slot;
        let mut var_vsch_db27: f64 = *var_vsch_db27_slot;
        let mut var_vsch_db28: f64 = *var_vsch_db28_slot;
        let mut var_vsch_db29: f64 = *var_vsch_db29_slot;
        let mut var_vsch_db3: f64 = *var_vsch_db3_slot;
        let mut var_vsch_db30: f64 = *var_vsch_db30_slot;
        let mut var_vsch_db31: f64 = *var_vsch_db31_slot;
        let mut var_vsch_db32: f64 = *var_vsch_db32_slot;
        let mut var_vsch_db33: f64 = *var_vsch_db33_slot;
        let mut var_vsch_db34: f64 = *var_vsch_db34_slot;
        let mut var_vsch_db35: f64 = *var_vsch_db35_slot;
        let mut var_vsch_db4: f64 = *var_vsch_db4_slot;
        let mut var_vsch_db5: f64 = *var_vsch_db5_slot;
        let mut var_vsch_db6: f64 = *var_vsch_db6_slot;
        let mut var_vsch_db7: f64 = *var_vsch_db7_slot;
        let mut var_vsch_db8: f64 = *var_vsch_db8_slot;
        let mut var_vsch_db9: f64 = *var_vsch_db9_slot;
        let mut var_vsch_dn0: f64 = *var_vsch_dn0_slot;
        let mut var_vsch_dn1: f64 = *var_vsch_dn1_slot;
        let mut var_vsch_dn10: f64 = *var_vsch_dn10_slot;
        let mut var_vsch_dn11: f64 = *var_vsch_dn11_slot;
        let mut var_vsch_dn12: f64 = *var_vsch_dn12_slot;
        let mut var_vsch_dn13: f64 = *var_vsch_dn13_slot;
        let mut var_vsch_dn14: f64 = *var_vsch_dn14_slot;
        let mut var_vsch_dn15: f64 = *var_vsch_dn15_slot;
        let mut var_vsch_dn16: f64 = *var_vsch_dn16_slot;
        let mut var_vsch_dn17: f64 = *var_vsch_dn17_slot;
        let mut var_vsch_dn18: f64 = *var_vsch_dn18_slot;
        let mut var_vsch_dn19: f64 = *var_vsch_dn19_slot;
        let mut var_vsch_dn2: f64 = *var_vsch_dn2_slot;
        let mut var_vsch_dn20: f64 = *var_vsch_dn20_slot;
        let mut var_vsch_dn21: f64 = *var_vsch_dn21_slot;
        let mut var_vsch_dn22: f64 = *var_vsch_dn22_slot;
        let mut var_vsch_dn23: f64 = *var_vsch_dn23_slot;
        let mut var_vsch_dn24: f64 = *var_vsch_dn24_slot;
        let mut var_vsch_dn25: f64 = *var_vsch_dn25_slot;
        let mut var_vsch_dn26: f64 = *var_vsch_dn26_slot;
        let mut var_vsch_dn27: f64 = *var_vsch_dn27_slot;
        let mut var_vsch_dn28: f64 = *var_vsch_dn28_slot;
        let mut var_vsch_dn29: f64 = *var_vsch_dn29_slot;
        let mut var_vsch_dn3: f64 = *var_vsch_dn3_slot;
        let mut var_vsch_dn4: f64 = *var_vsch_dn4_slot;
        let mut var_vsch_dn5: f64 = *var_vsch_dn5_slot;
        let mut var_vsch_dn6: f64 = *var_vsch_dn6_slot;
        let mut var_vsch_dn7: f64 = *var_vsch_dn7_slot;
        let mut var_vsch_dn8: f64 = *var_vsch_dn8_slot;
        let mut var_vsch_dn9: f64 = *var_vsch_dn9_slot;
        let mut var_vschfc2: f64 = *var_vschfc2_slot;
        let mut var_vschfc2_db0: f64 = *var_vschfc2_db0_slot;
        let mut var_vschfc2_db1: f64 = *var_vschfc2_db1_slot;
        let mut var_vschfc2_db10: f64 = *var_vschfc2_db10_slot;
        let mut var_vschfc2_db11: f64 = *var_vschfc2_db11_slot;
        let mut var_vschfc2_db12: f64 = *var_vschfc2_db12_slot;
        let mut var_vschfc2_db13: f64 = *var_vschfc2_db13_slot;
        let mut var_vschfc2_db14: f64 = *var_vschfc2_db14_slot;
        let mut var_vschfc2_db15: f64 = *var_vschfc2_db15_slot;
        let mut var_vschfc2_db16: f64 = *var_vschfc2_db16_slot;
        let mut var_vschfc2_db17: f64 = *var_vschfc2_db17_slot;
        let mut var_vschfc2_db18: f64 = *var_vschfc2_db18_slot;
        let mut var_vschfc2_db19: f64 = *var_vschfc2_db19_slot;
        let mut var_vschfc2_db2: f64 = *var_vschfc2_db2_slot;
        let mut var_vschfc2_db20: f64 = *var_vschfc2_db20_slot;
        let mut var_vschfc2_db21: f64 = *var_vschfc2_db21_slot;
        let mut var_vschfc2_db22: f64 = *var_vschfc2_db22_slot;
        let mut var_vschfc2_db23: f64 = *var_vschfc2_db23_slot;
        let mut var_vschfc2_db24: f64 = *var_vschfc2_db24_slot;
        let mut var_vschfc2_db25: f64 = *var_vschfc2_db25_slot;
        let mut var_vschfc2_db26: f64 = *var_vschfc2_db26_slot;
        let mut var_vschfc2_db27: f64 = *var_vschfc2_db27_slot;
        let mut var_vschfc2_db28: f64 = *var_vschfc2_db28_slot;
        let mut var_vschfc2_db29: f64 = *var_vschfc2_db29_slot;
        let mut var_vschfc2_db3: f64 = *var_vschfc2_db3_slot;
        let mut var_vschfc2_db30: f64 = *var_vschfc2_db30_slot;
        let mut var_vschfc2_db31: f64 = *var_vschfc2_db31_slot;
        let mut var_vschfc2_db32: f64 = *var_vschfc2_db32_slot;
        let mut var_vschfc2_db33: f64 = *var_vschfc2_db33_slot;
        let mut var_vschfc2_db34: f64 = *var_vschfc2_db34_slot;
        let mut var_vschfc2_db35: f64 = *var_vschfc2_db35_slot;
        let mut var_vschfc2_db4: f64 = *var_vschfc2_db4_slot;
        let mut var_vschfc2_db5: f64 = *var_vschfc2_db5_slot;
        let mut var_vschfc2_db6: f64 = *var_vschfc2_db6_slot;
        let mut var_vschfc2_db7: f64 = *var_vschfc2_db7_slot;
        let mut var_vschfc2_db8: f64 = *var_vschfc2_db8_slot;
        let mut var_vschfc2_db9: f64 = *var_vschfc2_db9_slot;
        let mut var_vschfc2_dn0: f64 = *var_vschfc2_dn0_slot;
        let mut var_vschfc2_dn1: f64 = *var_vschfc2_dn1_slot;
        let mut var_vschfc2_dn10: f64 = *var_vschfc2_dn10_slot;
        let mut var_vschfc2_dn11: f64 = *var_vschfc2_dn11_slot;
        let mut var_vschfc2_dn12: f64 = *var_vschfc2_dn12_slot;
        let mut var_vschfc2_dn13: f64 = *var_vschfc2_dn13_slot;
        let mut var_vschfc2_dn14: f64 = *var_vschfc2_dn14_slot;
        let mut var_vschfc2_dn15: f64 = *var_vschfc2_dn15_slot;
        let mut var_vschfc2_dn16: f64 = *var_vschfc2_dn16_slot;
        let mut var_vschfc2_dn17: f64 = *var_vschfc2_dn17_slot;
        let mut var_vschfc2_dn18: f64 = *var_vschfc2_dn18_slot;
        let mut var_vschfc2_dn19: f64 = *var_vschfc2_dn19_slot;
        let mut var_vschfc2_dn2: f64 = *var_vschfc2_dn2_slot;
        let mut var_vschfc2_dn20: f64 = *var_vschfc2_dn20_slot;
        let mut var_vschfc2_dn21: f64 = *var_vschfc2_dn21_slot;
        let mut var_vschfc2_dn22: f64 = *var_vschfc2_dn22_slot;
        let mut var_vschfc2_dn23: f64 = *var_vschfc2_dn23_slot;
        let mut var_vschfc2_dn24: f64 = *var_vschfc2_dn24_slot;
        let mut var_vschfc2_dn25: f64 = *var_vschfc2_dn25_slot;
        let mut var_vschfc2_dn26: f64 = *var_vschfc2_dn26_slot;
        let mut var_vschfc2_dn27: f64 = *var_vschfc2_dn27_slot;
        let mut var_vschfc2_dn28: f64 = *var_vschfc2_dn28_slot;
        let mut var_vschfc2_dn29: f64 = *var_vschfc2_dn29_slot;
        let mut var_vschfc2_dn3: f64 = *var_vschfc2_dn3_slot;
        let mut var_vschfc2_dn4: f64 = *var_vschfc2_dn4_slot;
        let mut var_vschfc2_dn5: f64 = *var_vschfc2_dn5_slot;
        let mut var_vschfc2_dn6: f64 = *var_vschfc2_dn6_slot;
        let mut var_vschfc2_dn7: f64 = *var_vschfc2_dn7_slot;
        let mut var_vschfc2_dn8: f64 = *var_vschfc2_dn8_slot;
        let mut var_vschfc2_dn9: f64 = *var_vschfc2_dn9_slot;
        let mut var_vschfc3: f64 = *var_vschfc3_slot;
        let mut var_vschfc3_db0: f64 = *var_vschfc3_db0_slot;
        let mut var_vschfc3_db1: f64 = *var_vschfc3_db1_slot;
        let mut var_vschfc3_db10: f64 = *var_vschfc3_db10_slot;
        let mut var_vschfc3_db11: f64 = *var_vschfc3_db11_slot;
        let mut var_vschfc3_db12: f64 = *var_vschfc3_db12_slot;
        let mut var_vschfc3_db13: f64 = *var_vschfc3_db13_slot;
        let mut var_vschfc3_db14: f64 = *var_vschfc3_db14_slot;
        let mut var_vschfc3_db15: f64 = *var_vschfc3_db15_slot;
        let mut var_vschfc3_db16: f64 = *var_vschfc3_db16_slot;
        let mut var_vschfc3_db17: f64 = *var_vschfc3_db17_slot;
        let mut var_vschfc3_db18: f64 = *var_vschfc3_db18_slot;
        let mut var_vschfc3_db19: f64 = *var_vschfc3_db19_slot;
        let mut var_vschfc3_db2: f64 = *var_vschfc3_db2_slot;
        let mut var_vschfc3_db20: f64 = *var_vschfc3_db20_slot;
        let mut var_vschfc3_db21: f64 = *var_vschfc3_db21_slot;
        let mut var_vschfc3_db22: f64 = *var_vschfc3_db22_slot;
        let mut var_vschfc3_db23: f64 = *var_vschfc3_db23_slot;
        let mut var_vschfc3_db24: f64 = *var_vschfc3_db24_slot;
        let mut var_vschfc3_db25: f64 = *var_vschfc3_db25_slot;
        let mut var_vschfc3_db26: f64 = *var_vschfc3_db26_slot;
        let mut var_vschfc3_db27: f64 = *var_vschfc3_db27_slot;
        let mut var_vschfc3_db28: f64 = *var_vschfc3_db28_slot;
        let mut var_vschfc3_db29: f64 = *var_vschfc3_db29_slot;
        let mut var_vschfc3_db3: f64 = *var_vschfc3_db3_slot;
        let mut var_vschfc3_db30: f64 = *var_vschfc3_db30_slot;
        let mut var_vschfc3_db31: f64 = *var_vschfc3_db31_slot;
        let mut var_vschfc3_db32: f64 = *var_vschfc3_db32_slot;
        let mut var_vschfc3_db33: f64 = *var_vschfc3_db33_slot;
        let mut var_vschfc3_db34: f64 = *var_vschfc3_db34_slot;
        let mut var_vschfc3_db35: f64 = *var_vschfc3_db35_slot;
        let mut var_vschfc3_db4: f64 = *var_vschfc3_db4_slot;
        let mut var_vschfc3_db5: f64 = *var_vschfc3_db5_slot;
        let mut var_vschfc3_db6: f64 = *var_vschfc3_db6_slot;
        let mut var_vschfc3_db7: f64 = *var_vschfc3_db7_slot;
        let mut var_vschfc3_db8: f64 = *var_vschfc3_db8_slot;
        let mut var_vschfc3_db9: f64 = *var_vschfc3_db9_slot;
        let mut var_vschfc3_dn0: f64 = *var_vschfc3_dn0_slot;
        let mut var_vschfc3_dn1: f64 = *var_vschfc3_dn1_slot;
        let mut var_vschfc3_dn10: f64 = *var_vschfc3_dn10_slot;
        let mut var_vschfc3_dn11: f64 = *var_vschfc3_dn11_slot;
        let mut var_vschfc3_dn12: f64 = *var_vschfc3_dn12_slot;
        let mut var_vschfc3_dn13: f64 = *var_vschfc3_dn13_slot;
        let mut var_vschfc3_dn14: f64 = *var_vschfc3_dn14_slot;
        let mut var_vschfc3_dn15: f64 = *var_vschfc3_dn15_slot;
        let mut var_vschfc3_dn16: f64 = *var_vschfc3_dn16_slot;
        let mut var_vschfc3_dn17: f64 = *var_vschfc3_dn17_slot;
        let mut var_vschfc3_dn18: f64 = *var_vschfc3_dn18_slot;
        let mut var_vschfc3_dn19: f64 = *var_vschfc3_dn19_slot;
        let mut var_vschfc3_dn2: f64 = *var_vschfc3_dn2_slot;
        let mut var_vschfc3_dn20: f64 = *var_vschfc3_dn20_slot;
        let mut var_vschfc3_dn21: f64 = *var_vschfc3_dn21_slot;
        let mut var_vschfc3_dn22: f64 = *var_vschfc3_dn22_slot;
        let mut var_vschfc3_dn23: f64 = *var_vschfc3_dn23_slot;
        let mut var_vschfc3_dn24: f64 = *var_vschfc3_dn24_slot;
        let mut var_vschfc3_dn25: f64 = *var_vschfc3_dn25_slot;
        let mut var_vschfc3_dn26: f64 = *var_vschfc3_dn26_slot;
        let mut var_vschfc3_dn27: f64 = *var_vschfc3_dn27_slot;
        let mut var_vschfc3_dn28: f64 = *var_vschfc3_dn28_slot;
        let mut var_vschfc3_dn29: f64 = *var_vschfc3_dn29_slot;
        let mut var_vschfc3_dn3: f64 = *var_vschfc3_dn3_slot;
        let mut var_vschfc3_dn4: f64 = *var_vschfc3_dn4_slot;
        let mut var_vschfc3_dn5: f64 = *var_vschfc3_dn5_slot;
        let mut var_vschfc3_dn6: f64 = *var_vschfc3_dn6_slot;
        let mut var_vschfc3_dn7: f64 = *var_vschfc3_dn7_slot;
        let mut var_vschfc3_dn8: f64 = *var_vschfc3_dn8_slot;
        let mut var_vschfc3_dn9: f64 = *var_vschfc3_dn9_slot;
        let mut var_vschfc4: f64 = *var_vschfc4_slot;
        let mut var_vschfc4_db0: f64 = *var_vschfc4_db0_slot;
        let mut var_vschfc4_db1: f64 = *var_vschfc4_db1_slot;
        let mut var_vschfc4_db10: f64 = *var_vschfc4_db10_slot;
        let mut var_vschfc4_db11: f64 = *var_vschfc4_db11_slot;
        let mut var_vschfc4_db12: f64 = *var_vschfc4_db12_slot;
        let mut var_vschfc4_db13: f64 = *var_vschfc4_db13_slot;
        let mut var_vschfc4_db14: f64 = *var_vschfc4_db14_slot;
        let mut var_vschfc4_db15: f64 = *var_vschfc4_db15_slot;
        let mut var_vschfc4_db16: f64 = *var_vschfc4_db16_slot;
        let mut var_vschfc4_db17: f64 = *var_vschfc4_db17_slot;
        let mut var_vschfc4_db18: f64 = *var_vschfc4_db18_slot;
        let mut var_vschfc4_db19: f64 = *var_vschfc4_db19_slot;
        let mut var_vschfc4_db2: f64 = *var_vschfc4_db2_slot;
        let mut var_vschfc4_db20: f64 = *var_vschfc4_db20_slot;
        let mut var_vschfc4_db21: f64 = *var_vschfc4_db21_slot;
        let mut var_vschfc4_db22: f64 = *var_vschfc4_db22_slot;
        let mut var_vschfc4_db23: f64 = *var_vschfc4_db23_slot;
        let mut var_vschfc4_db24: f64 = *var_vschfc4_db24_slot;
        let mut var_vschfc4_db25: f64 = *var_vschfc4_db25_slot;
        let mut var_vschfc4_db26: f64 = *var_vschfc4_db26_slot;
        let mut var_vschfc4_db27: f64 = *var_vschfc4_db27_slot;
        let mut var_vschfc4_db28: f64 = *var_vschfc4_db28_slot;
        let mut var_vschfc4_db29: f64 = *var_vschfc4_db29_slot;
        let mut var_vschfc4_db3: f64 = *var_vschfc4_db3_slot;
        let mut var_vschfc4_db30: f64 = *var_vschfc4_db30_slot;
        let mut var_vschfc4_db31: f64 = *var_vschfc4_db31_slot;
        let mut var_vschfc4_db32: f64 = *var_vschfc4_db32_slot;
        let mut var_vschfc4_db33: f64 = *var_vschfc4_db33_slot;
        let mut var_vschfc4_db34: f64 = *var_vschfc4_db34_slot;
        let mut var_vschfc4_db35: f64 = *var_vschfc4_db35_slot;
        let mut var_vschfc4_db4: f64 = *var_vschfc4_db4_slot;
        let mut var_vschfc4_db5: f64 = *var_vschfc4_db5_slot;
        let mut var_vschfc4_db6: f64 = *var_vschfc4_db6_slot;
        let mut var_vschfc4_db7: f64 = *var_vschfc4_db7_slot;
        let mut var_vschfc4_db8: f64 = *var_vschfc4_db8_slot;
        let mut var_vschfc4_db9: f64 = *var_vschfc4_db9_slot;
        let mut var_vschfc4_dn0: f64 = *var_vschfc4_dn0_slot;
        let mut var_vschfc4_dn1: f64 = *var_vschfc4_dn1_slot;
        let mut var_vschfc4_dn10: f64 = *var_vschfc4_dn10_slot;
        let mut var_vschfc4_dn11: f64 = *var_vschfc4_dn11_slot;
        let mut var_vschfc4_dn12: f64 = *var_vschfc4_dn12_slot;
        let mut var_vschfc4_dn13: f64 = *var_vschfc4_dn13_slot;
        let mut var_vschfc4_dn14: f64 = *var_vschfc4_dn14_slot;
        let mut var_vschfc4_dn15: f64 = *var_vschfc4_dn15_slot;
        let mut var_vschfc4_dn16: f64 = *var_vschfc4_dn16_slot;
        let mut var_vschfc4_dn17: f64 = *var_vschfc4_dn17_slot;
        let mut var_vschfc4_dn18: f64 = *var_vschfc4_dn18_slot;
        let mut var_vschfc4_dn19: f64 = *var_vschfc4_dn19_slot;
        let mut var_vschfc4_dn2: f64 = *var_vschfc4_dn2_slot;
        let mut var_vschfc4_dn20: f64 = *var_vschfc4_dn20_slot;
        let mut var_vschfc4_dn21: f64 = *var_vschfc4_dn21_slot;
        let mut var_vschfc4_dn22: f64 = *var_vschfc4_dn22_slot;
        let mut var_vschfc4_dn23: f64 = *var_vschfc4_dn23_slot;
        let mut var_vschfc4_dn24: f64 = *var_vschfc4_dn24_slot;
        let mut var_vschfc4_dn25: f64 = *var_vschfc4_dn25_slot;
        let mut var_vschfc4_dn26: f64 = *var_vschfc4_dn26_slot;
        let mut var_vschfc4_dn27: f64 = *var_vschfc4_dn27_slot;
        let mut var_vschfc4_dn28: f64 = *var_vschfc4_dn28_slot;
        let mut var_vschfc4_dn29: f64 = *var_vschfc4_dn29_slot;
        let mut var_vschfc4_dn3: f64 = *var_vschfc4_dn3_slot;
        let mut var_vschfc4_dn4: f64 = *var_vschfc4_dn4_slot;
        let mut var_vschfc4_dn5: f64 = *var_vschfc4_dn5_slot;
        let mut var_vschfc4_dn6: f64 = *var_vschfc4_dn6_slot;
        let mut var_vschfc4_dn7: f64 = *var_vschfc4_dn7_slot;
        let mut var_vschfc4_dn8: f64 = *var_vschfc4_dn8_slot;
        let mut var_vschfc4_dn9: f64 = *var_vschfc4_dn9_slot;
        let mut var_vschfc5: f64 = *var_vschfc5_slot;
        let mut var_vschfc5_db0: f64 = *var_vschfc5_db0_slot;
        let mut var_vschfc5_db1: f64 = *var_vschfc5_db1_slot;
        let mut var_vschfc5_db10: f64 = *var_vschfc5_db10_slot;
        let mut var_vschfc5_db11: f64 = *var_vschfc5_db11_slot;
        let mut var_vschfc5_db12: f64 = *var_vschfc5_db12_slot;
        let mut var_vschfc5_db13: f64 = *var_vschfc5_db13_slot;
        let mut var_vschfc5_db14: f64 = *var_vschfc5_db14_slot;
        let mut var_vschfc5_db15: f64 = *var_vschfc5_db15_slot;
        let mut var_vschfc5_db16: f64 = *var_vschfc5_db16_slot;
        let mut var_vschfc5_db17: f64 = *var_vschfc5_db17_slot;
        let mut var_vschfc5_db18: f64 = *var_vschfc5_db18_slot;
        let mut var_vschfc5_db19: f64 = *var_vschfc5_db19_slot;
        let mut var_vschfc5_db2: f64 = *var_vschfc5_db2_slot;
        let mut var_vschfc5_db20: f64 = *var_vschfc5_db20_slot;
        let mut var_vschfc5_db21: f64 = *var_vschfc5_db21_slot;
        let mut var_vschfc5_db22: f64 = *var_vschfc5_db22_slot;
        let mut var_vschfc5_db23: f64 = *var_vschfc5_db23_slot;
        let mut var_vschfc5_db24: f64 = *var_vschfc5_db24_slot;
        let mut var_vschfc5_db25: f64 = *var_vschfc5_db25_slot;
        let mut var_vschfc5_db26: f64 = *var_vschfc5_db26_slot;
        let mut var_vschfc5_db27: f64 = *var_vschfc5_db27_slot;
        let mut var_vschfc5_db28: f64 = *var_vschfc5_db28_slot;
        let mut var_vschfc5_db29: f64 = *var_vschfc5_db29_slot;
        let mut var_vschfc5_db3: f64 = *var_vschfc5_db3_slot;
        let mut var_vschfc5_db30: f64 = *var_vschfc5_db30_slot;
        let mut var_vschfc5_db31: f64 = *var_vschfc5_db31_slot;
        let mut var_vschfc5_db32: f64 = *var_vschfc5_db32_slot;
        let mut var_vschfc5_db33: f64 = *var_vschfc5_db33_slot;
        let mut var_vschfc5_db34: f64 = *var_vschfc5_db34_slot;
        let mut var_vschfc5_db35: f64 = *var_vschfc5_db35_slot;
        let mut var_vschfc5_db4: f64 = *var_vschfc5_db4_slot;
        let mut var_vschfc5_db5: f64 = *var_vschfc5_db5_slot;
        let mut var_vschfc5_db6: f64 = *var_vschfc5_db6_slot;
        let mut var_vschfc5_db7: f64 = *var_vschfc5_db7_slot;
        let mut var_vschfc5_db8: f64 = *var_vschfc5_db8_slot;
        let mut var_vschfc5_db9: f64 = *var_vschfc5_db9_slot;
        let mut var_vschfc5_dn0: f64 = *var_vschfc5_dn0_slot;
        let mut var_vschfc5_dn1: f64 = *var_vschfc5_dn1_slot;
        let mut var_vschfc5_dn10: f64 = *var_vschfc5_dn10_slot;
        let mut var_vschfc5_dn11: f64 = *var_vschfc5_dn11_slot;
        let mut var_vschfc5_dn12: f64 = *var_vschfc5_dn12_slot;
        let mut var_vschfc5_dn13: f64 = *var_vschfc5_dn13_slot;
        let mut var_vschfc5_dn14: f64 = *var_vschfc5_dn14_slot;
        let mut var_vschfc5_dn15: f64 = *var_vschfc5_dn15_slot;
        let mut var_vschfc5_dn16: f64 = *var_vschfc5_dn16_slot;
        let mut var_vschfc5_dn17: f64 = *var_vschfc5_dn17_slot;
        let mut var_vschfc5_dn18: f64 = *var_vschfc5_dn18_slot;
        let mut var_vschfc5_dn19: f64 = *var_vschfc5_dn19_slot;
        let mut var_vschfc5_dn2: f64 = *var_vschfc5_dn2_slot;
        let mut var_vschfc5_dn20: f64 = *var_vschfc5_dn20_slot;
        let mut var_vschfc5_dn21: f64 = *var_vschfc5_dn21_slot;
        let mut var_vschfc5_dn22: f64 = *var_vschfc5_dn22_slot;
        let mut var_vschfc5_dn23: f64 = *var_vschfc5_dn23_slot;
        let mut var_vschfc5_dn24: f64 = *var_vschfc5_dn24_slot;
        let mut var_vschfc5_dn25: f64 = *var_vschfc5_dn25_slot;
        let mut var_vschfc5_dn26: f64 = *var_vschfc5_dn26_slot;
        let mut var_vschfc5_dn27: f64 = *var_vschfc5_dn27_slot;
        let mut var_vschfc5_dn28: f64 = *var_vschfc5_dn28_slot;
        let mut var_vschfc5_dn29: f64 = *var_vschfc5_dn29_slot;
        let mut var_vschfc5_dn3: f64 = *var_vschfc5_dn3_slot;
        let mut var_vschfc5_dn4: f64 = *var_vschfc5_dn4_slot;
        let mut var_vschfc5_dn5: f64 = *var_vschfc5_dn5_slot;
        let mut var_vschfc5_dn6: f64 = *var_vschfc5_dn6_slot;
        let mut var_vschfc5_dn7: f64 = *var_vschfc5_dn7_slot;
        let mut var_vschfc5_dn8: f64 = *var_vschfc5_dn8_slot;
        let mut var_vschfc5_dn9: f64 = *var_vschfc5_dn9_slot;

        var_vschfc2 = 0.0;
        var_vschfc2_dn0 = 0.0;
        var_vschfc2_dn1 = 0.0;
        var_vschfc2_dn2 = 0.0;
        var_vschfc2_dn3 = 0.0;
        var_vschfc2_dn4 = 0.0;
        var_vschfc2_dn5 = 0.0;
        var_vschfc2_dn6 = 0.0;
        var_vschfc2_dn7 = 0.0;
        var_vschfc2_dn8 = 0.0;
        var_vschfc2_dn9 = 0.0;
        var_vschfc2_dn10 = 0.0;
        var_vschfc2_dn11 = 0.0;
        var_vschfc2_dn12 = 0.0;
        var_vschfc2_dn13 = 0.0;
        var_vschfc2_dn14 = 0.0;
        var_vschfc2_dn15 = 0.0;
        var_vschfc2_dn16 = 0.0;
        var_vschfc2_dn17 = 0.0;
        var_vschfc2_dn18 = 0.0;
        var_vschfc2_dn19 = 0.0;
        var_vschfc2_dn20 = 0.0;
        var_vschfc2_dn21 = 0.0;
        var_vschfc2_dn22 = 0.0;
        var_vschfc2_dn23 = 0.0;
        var_vschfc2_dn24 = 0.0;
        var_vschfc2_dn25 = 0.0;
        var_vschfc2_dn26 = 0.0;
        var_vschfc2_dn27 = 0.0;
        var_vschfc2_dn28 = 0.0;
        var_vschfc2_dn29 = 0.0;
        var_vschfc2_db0 = 0.0;
        var_vschfc2_db1 = 0.0;
        var_vschfc2_db2 = 0.0;
        var_vschfc2_db3 = 0.0;
        var_vschfc2_db4 = 0.0;
        var_vschfc2_db5 = 0.0;
        var_vschfc2_db6 = 0.0;
        var_vschfc2_db7 = 0.0;
        var_vschfc2_db8 = 0.0;
        var_vschfc2_db9 = 0.0;
        var_vschfc2_db10 = 0.0;
        var_vschfc2_db11 = 0.0;
        var_vschfc2_db12 = 0.0;
        var_vschfc2_db13 = 0.0;
        var_vschfc2_db14 = 0.0;
        var_vschfc2_db15 = 0.0;
        var_vschfc2_db16 = 0.0;
        var_vschfc2_db17 = 0.0;
        var_vschfc2_db18 = 0.0;
        var_vschfc2_db19 = 0.0;
        var_vschfc2_db20 = 0.0;
        var_vschfc2_db21 = 0.0;
        var_vschfc2_db22 = 0.0;
        var_vschfc2_db23 = 0.0;
        var_vschfc2_db24 = 0.0;
        var_vschfc2_db25 = 0.0;
        var_vschfc2_db26 = 0.0;
        var_vschfc2_db27 = 0.0;
        var_vschfc2_db28 = 0.0;
        var_vschfc2_db29 = 0.0;
        var_vschfc2_db30 = 0.0;
        var_vschfc2_db31 = 0.0;
        var_vschfc2_db32 = 0.0;
        var_vschfc2_db33 = 0.0;
        var_vschfc2_db34 = 0.0;
        var_vschfc2_db35 = 0.0;

        var_vschfc3 = 0.0;
        var_vschfc3_dn0 = 0.0;
        var_vschfc3_dn1 = 0.0;
        var_vschfc3_dn2 = 0.0;
        var_vschfc3_dn3 = 0.0;
        var_vschfc3_dn4 = 0.0;
        var_vschfc3_dn5 = 0.0;
        var_vschfc3_dn6 = 0.0;
        var_vschfc3_dn7 = 0.0;
        var_vschfc3_dn8 = 0.0;
        var_vschfc3_dn9 = 0.0;
        var_vschfc3_dn10 = 0.0;
        var_vschfc3_dn11 = 0.0;
        var_vschfc3_dn12 = 0.0;
        var_vschfc3_dn13 = 0.0;
        var_vschfc3_dn14 = 0.0;
        var_vschfc3_dn15 = 0.0;
        var_vschfc3_dn16 = 0.0;
        var_vschfc3_dn17 = 0.0;
        var_vschfc3_dn18 = 0.0;
        var_vschfc3_dn19 = 0.0;
        var_vschfc3_dn20 = 0.0;
        var_vschfc3_dn21 = 0.0;
        var_vschfc3_dn22 = 0.0;
        var_vschfc3_dn23 = 0.0;
        var_vschfc3_dn24 = 0.0;
        var_vschfc3_dn25 = 0.0;
        var_vschfc3_dn26 = 0.0;
        var_vschfc3_dn27 = 0.0;
        var_vschfc3_dn28 = 0.0;
        var_vschfc3_dn29 = 0.0;
        var_vschfc3_db0 = 0.0;
        var_vschfc3_db1 = 0.0;
        var_vschfc3_db2 = 0.0;
        var_vschfc3_db3 = 0.0;
        var_vschfc3_db4 = 0.0;
        var_vschfc3_db5 = 0.0;
        var_vschfc3_db6 = 0.0;
        var_vschfc3_db7 = 0.0;
        var_vschfc3_db8 = 0.0;
        var_vschfc3_db9 = 0.0;
        var_vschfc3_db10 = 0.0;
        var_vschfc3_db11 = 0.0;
        var_vschfc3_db12 = 0.0;
        var_vschfc3_db13 = 0.0;
        var_vschfc3_db14 = 0.0;
        var_vschfc3_db15 = 0.0;
        var_vschfc3_db16 = 0.0;
        var_vschfc3_db17 = 0.0;
        var_vschfc3_db18 = 0.0;
        var_vschfc3_db19 = 0.0;
        var_vschfc3_db20 = 0.0;
        var_vschfc3_db21 = 0.0;
        var_vschfc3_db22 = 0.0;
        var_vschfc3_db23 = 0.0;
        var_vschfc3_db24 = 0.0;
        var_vschfc3_db25 = 0.0;
        var_vschfc3_db26 = 0.0;
        var_vschfc3_db27 = 0.0;
        var_vschfc3_db28 = 0.0;
        var_vschfc3_db29 = 0.0;
        var_vschfc3_db30 = 0.0;
        var_vschfc3_db31 = 0.0;
        var_vschfc3_db32 = 0.0;
        var_vschfc3_db33 = 0.0;
        var_vschfc3_db34 = 0.0;
        var_vschfc3_db35 = 0.0;

        var_vschfc4 = 0.0;
        var_vschfc4_dn0 = 0.0;
        var_vschfc4_dn1 = 0.0;
        var_vschfc4_dn2 = 0.0;
        var_vschfc4_dn3 = 0.0;
        var_vschfc4_dn4 = 0.0;
        var_vschfc4_dn5 = 0.0;
        var_vschfc4_dn6 = 0.0;
        var_vschfc4_dn7 = 0.0;
        var_vschfc4_dn8 = 0.0;
        var_vschfc4_dn9 = 0.0;
        var_vschfc4_dn10 = 0.0;
        var_vschfc4_dn11 = 0.0;
        var_vschfc4_dn12 = 0.0;
        var_vschfc4_dn13 = 0.0;
        var_vschfc4_dn14 = 0.0;
        var_vschfc4_dn15 = 0.0;
        var_vschfc4_dn16 = 0.0;
        var_vschfc4_dn17 = 0.0;
        var_vschfc4_dn18 = 0.0;
        var_vschfc4_dn19 = 0.0;
        var_vschfc4_dn20 = 0.0;
        var_vschfc4_dn21 = 0.0;
        var_vschfc4_dn22 = 0.0;
        var_vschfc4_dn23 = 0.0;
        var_vschfc4_dn24 = 0.0;
        var_vschfc4_dn25 = 0.0;
        var_vschfc4_dn26 = 0.0;
        var_vschfc4_dn27 = 0.0;
        var_vschfc4_dn28 = 0.0;
        var_vschfc4_dn29 = 0.0;
        var_vschfc4_db0 = 0.0;
        var_vschfc4_db1 = 0.0;
        var_vschfc4_db2 = 0.0;
        var_vschfc4_db3 = 0.0;
        var_vschfc4_db4 = 0.0;
        var_vschfc4_db5 = 0.0;
        var_vschfc4_db6 = 0.0;
        var_vschfc4_db7 = 0.0;
        var_vschfc4_db8 = 0.0;
        var_vschfc4_db9 = 0.0;
        var_vschfc4_db10 = 0.0;
        var_vschfc4_db11 = 0.0;
        var_vschfc4_db12 = 0.0;
        var_vschfc4_db13 = 0.0;
        var_vschfc4_db14 = 0.0;
        var_vschfc4_db15 = 0.0;
        var_vschfc4_db16 = 0.0;
        var_vschfc4_db17 = 0.0;
        var_vschfc4_db18 = 0.0;
        var_vschfc4_db19 = 0.0;
        var_vschfc4_db20 = 0.0;
        var_vschfc4_db21 = 0.0;
        var_vschfc4_db22 = 0.0;
        var_vschfc4_db23 = 0.0;
        var_vschfc4_db24 = 0.0;
        var_vschfc4_db25 = 0.0;
        var_vschfc4_db26 = 0.0;
        var_vschfc4_db27 = 0.0;
        var_vschfc4_db28 = 0.0;
        var_vschfc4_db29 = 0.0;
        var_vschfc4_db30 = 0.0;
        var_vschfc4_db31 = 0.0;
        var_vschfc4_db32 = 0.0;
        var_vschfc4_db33 = 0.0;
        var_vschfc4_db34 = 0.0;
        var_vschfc4_db35 = 0.0;

        var_vschfc5 = 0.0;
        var_vschfc5_dn0 = 0.0;
        var_vschfc5_dn1 = 0.0;
        var_vschfc5_dn2 = 0.0;
        var_vschfc5_dn3 = 0.0;
        var_vschfc5_dn4 = 0.0;
        var_vschfc5_dn5 = 0.0;
        var_vschfc5_dn6 = 0.0;
        var_vschfc5_dn7 = 0.0;
        var_vschfc5_dn8 = 0.0;
        var_vschfc5_dn9 = 0.0;
        var_vschfc5_dn10 = 0.0;
        var_vschfc5_dn11 = 0.0;
        var_vschfc5_dn12 = 0.0;
        var_vschfc5_dn13 = 0.0;
        var_vschfc5_dn14 = 0.0;
        var_vschfc5_dn15 = 0.0;
        var_vschfc5_dn16 = 0.0;
        var_vschfc5_dn17 = 0.0;
        var_vschfc5_dn18 = 0.0;
        var_vschfc5_dn19 = 0.0;
        var_vschfc5_dn20 = 0.0;
        var_vschfc5_dn21 = 0.0;
        var_vschfc5_dn22 = 0.0;
        var_vschfc5_dn23 = 0.0;
        var_vschfc5_dn24 = 0.0;
        var_vschfc5_dn25 = 0.0;
        var_vschfc5_dn26 = 0.0;
        var_vschfc5_dn27 = 0.0;
        var_vschfc5_dn28 = 0.0;
        var_vschfc5_dn29 = 0.0;
        var_vschfc5_db0 = 0.0;
        var_vschfc5_db1 = 0.0;
        var_vschfc5_db2 = 0.0;
        var_vschfc5_db3 = 0.0;
        var_vschfc5_db4 = 0.0;
        var_vschfc5_db5 = 0.0;
        var_vschfc5_db6 = 0.0;
        var_vschfc5_db7 = 0.0;
        var_vschfc5_db8 = 0.0;
        var_vschfc5_db9 = 0.0;
        var_vschfc5_db10 = 0.0;
        var_vschfc5_db11 = 0.0;
        var_vschfc5_db12 = 0.0;
        var_vschfc5_db13 = 0.0;
        var_vschfc5_db14 = 0.0;
        var_vschfc5_db15 = 0.0;
        var_vschfc5_db16 = 0.0;
        var_vschfc5_db17 = 0.0;
        var_vschfc5_db18 = 0.0;
        var_vschfc5_db19 = 0.0;
        var_vschfc5_db20 = 0.0;
        var_vschfc5_db21 = 0.0;
        var_vschfc5_db22 = 0.0;
        var_vschfc5_db23 = 0.0;
        var_vschfc5_db24 = 0.0;
        var_vschfc5_db25 = 0.0;
        var_vschfc5_db26 = 0.0;
        var_vschfc5_db27 = 0.0;
        var_vschfc5_db28 = 0.0;
        var_vschfc5_db29 = 0.0;
        var_vschfc5_db30 = 0.0;
        var_vschfc5_db31 = 0.0;
        var_vschfc5_db32 = 0.0;
        var_vschfc5_db33 = 0.0;
        var_vschfc5_db34 = 0.0;
        var_vschfc5_db35 = 0.0;

        let assign41530_e39902: f64 = if p.p291 == 1.0 { 1.0 } else { 0.0 };
        var_guard461 = assign41530_e39902;

        let (assign41540_e39908, assign41540_e39908_d_n0, assign41540_e39908_d_n1, assign41540_e39908_d_n2, assign41540_e39908_d_n3, assign41540_e39908_d_n4, assign41540_e39908_d_n5, assign41540_e39908_d_n6, assign41540_e39908_d_n7, assign41540_e39908_d_n8, assign41540_e39908_d_n9, assign41540_e39908_d_n10, assign41540_e39908_d_n11, assign41540_e39908_d_n12, assign41540_e39908_d_n13, assign41540_e39908_d_n14, assign41540_e39908_d_n15, assign41540_e39908_d_n16, assign41540_e39908_d_n17, assign41540_e39908_d_n18, assign41540_e39908_d_n19, assign41540_e39908_d_n20, assign41540_e39908_d_n21, assign41540_e39908_d_n22, assign41540_e39908_d_n23, assign41540_e39908_d_n24, assign41540_e39908_d_n25, assign41540_e39908_d_n26, assign41540_e39908_d_n27, assign41540_e39908_d_n28, assign41540_e39908_d_n29, assign41540_e39908_d_b0, assign41540_e39908_d_b1, assign41540_e39908_d_b2, assign41540_e39908_d_b3, assign41540_e39908_d_b4, assign41540_e39908_d_b5, assign41540_e39908_d_b6, assign41540_e39908_d_b7, assign41540_e39908_d_b8, assign41540_e39908_d_b9, assign41540_e39908_d_b10, assign41540_e39908_d_b11, assign41540_e39908_d_b12, assign41540_e39908_d_b13, assign41540_e39908_d_b14, assign41540_e39908_d_b15, assign41540_e39908_d_b16, assign41540_e39908_d_b17, assign41540_e39908_d_b18, assign41540_e39908_d_b19, assign41540_e39908_d_b20, assign41540_e39908_d_b21, assign41540_e39908_d_b22, assign41540_e39908_d_b23, assign41540_e39908_d_b24, assign41540_e39908_d_b25, assign41540_e39908_d_b26, assign41540_e39908_d_b27, assign41540_e39908_d_b28, assign41540_e39908_d_b29, assign41540_e39908_d_b30, assign41540_e39908_d_b31, assign41540_e39908_d_b32, assign41540_e39908_d_b33, assign41540_e39908_d_b34, assign41540_e39908_d_b35,) = {
    if (var_guard461 != 0.0) {
        let assign41540_e39906: f64 = (p.p6 * (nv8 - nv7));
        (assign41540_e39906, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (-p.p6), p.p6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vsch, var_vsch_dn0, var_vsch_dn1, var_vsch_dn2, var_vsch_dn3, var_vsch_dn4, var_vsch_dn5, var_vsch_dn6, var_vsch_dn7, var_vsch_dn8, var_vsch_dn9, var_vsch_dn10, var_vsch_dn11, var_vsch_dn12, var_vsch_dn13, var_vsch_dn14, var_vsch_dn15, var_vsch_dn16, var_vsch_dn17, var_vsch_dn18, var_vsch_dn19, var_vsch_dn20, var_vsch_dn21, var_vsch_dn22, var_vsch_dn23, var_vsch_dn24, var_vsch_dn25, var_vsch_dn26, var_vsch_dn27, var_vsch_dn28, var_vsch_dn29, var_vsch_db0, var_vsch_db1, var_vsch_db2, var_vsch_db3, var_vsch_db4, var_vsch_db5, var_vsch_db6, var_vsch_db7, var_vsch_db8, var_vsch_db9, var_vsch_db10, var_vsch_db11, var_vsch_db12, var_vsch_db13, var_vsch_db14, var_vsch_db15, var_vsch_db16, var_vsch_db17, var_vsch_db18, var_vsch_db19, var_vsch_db20, var_vsch_db21, var_vsch_db22, var_vsch_db23, var_vsch_db24, var_vsch_db25, var_vsch_db26, var_vsch_db27, var_vsch_db28, var_vsch_db29, var_vsch_db30, var_vsch_db31, var_vsch_db32, var_vsch_db33, var_vsch_db34, var_vsch_db35,)
    }
};
        var_vsch = assign41540_e39908;
        var_vsch_dn0 = assign41540_e39908_d_n0;
        var_vsch_dn1 = assign41540_e39908_d_n1;
        var_vsch_dn2 = assign41540_e39908_d_n2;
        var_vsch_dn3 = assign41540_e39908_d_n3;
        var_vsch_dn4 = assign41540_e39908_d_n4;
        var_vsch_dn5 = assign41540_e39908_d_n5;
        var_vsch_dn6 = assign41540_e39908_d_n6;
        var_vsch_dn7 = assign41540_e39908_d_n7;
        var_vsch_dn8 = assign41540_e39908_d_n8;
        var_vsch_dn9 = assign41540_e39908_d_n9;
        var_vsch_dn10 = assign41540_e39908_d_n10;
        var_vsch_dn11 = assign41540_e39908_d_n11;
        var_vsch_dn12 = assign41540_e39908_d_n12;
        var_vsch_dn13 = assign41540_e39908_d_n13;
        var_vsch_dn14 = assign41540_e39908_d_n14;
        var_vsch_dn15 = assign41540_e39908_d_n15;
        var_vsch_dn16 = assign41540_e39908_d_n16;
        var_vsch_dn17 = assign41540_e39908_d_n17;
        var_vsch_dn18 = assign41540_e39908_d_n18;
        var_vsch_dn19 = assign41540_e39908_d_n19;
        var_vsch_dn20 = assign41540_e39908_d_n20;
        var_vsch_dn21 = assign41540_e39908_d_n21;
        var_vsch_dn22 = assign41540_e39908_d_n22;
        var_vsch_dn23 = assign41540_e39908_d_n23;
        var_vsch_dn24 = assign41540_e39908_d_n24;
        var_vsch_dn25 = assign41540_e39908_d_n25;
        var_vsch_dn26 = assign41540_e39908_d_n26;
        var_vsch_dn27 = assign41540_e39908_d_n27;
        var_vsch_dn28 = assign41540_e39908_d_n28;
        var_vsch_dn29 = assign41540_e39908_d_n29;
        var_vsch_db0 = assign41540_e39908_d_b0;
        var_vsch_db1 = assign41540_e39908_d_b1;
        var_vsch_db2 = assign41540_e39908_d_b2;
        var_vsch_db3 = assign41540_e39908_d_b3;
        var_vsch_db4 = assign41540_e39908_d_b4;
        var_vsch_db5 = assign41540_e39908_d_b5;
        var_vsch_db6 = assign41540_e39908_d_b6;
        var_vsch_db7 = assign41540_e39908_d_b7;
        var_vsch_db8 = assign41540_e39908_d_b8;
        var_vsch_db9 = assign41540_e39908_d_b9;
        var_vsch_db10 = assign41540_e39908_d_b10;
        var_vsch_db11 = assign41540_e39908_d_b11;
        var_vsch_db12 = assign41540_e39908_d_b12;
        var_vsch_db13 = assign41540_e39908_d_b13;
        var_vsch_db14 = assign41540_e39908_d_b14;
        var_vsch_db15 = assign41540_e39908_d_b15;
        var_vsch_db16 = assign41540_e39908_d_b16;
        var_vsch_db17 = assign41540_e39908_d_b17;
        var_vsch_db18 = assign41540_e39908_d_b18;
        var_vsch_db19 = assign41540_e39908_d_b19;
        var_vsch_db20 = assign41540_e39908_d_b20;
        var_vsch_db21 = assign41540_e39908_d_b21;
        var_vsch_db22 = assign41540_e39908_d_b22;
        var_vsch_db23 = assign41540_e39908_d_b23;
        var_vsch_db24 = assign41540_e39908_d_b24;
        var_vsch_db25 = assign41540_e39908_d_b25;
        var_vsch_db26 = assign41540_e39908_d_b26;
        var_vsch_db27 = assign41540_e39908_d_b27;
        var_vsch_db28 = assign41540_e39908_d_b28;
        var_vsch_db29 = assign41540_e39908_d_b29;
        var_vsch_db30 = assign41540_e39908_d_b30;
        var_vsch_db31 = assign41540_e39908_d_b31;
        var_vsch_db32 = assign41540_e39908_d_b32;
        var_vsch_db33 = assign41540_e39908_d_b33;
        var_vsch_db34 = assign41540_e39908_d_b34;
        var_vsch_db35 = assign41540_e39908_d_b35;

        if (s.v[2418] != 0.0) {
            s.store_scalar(2419, 0.0);
            s.store_scalar(2420, 0.0);
            s.store_scalar(2421, 0.0);
            s.copy_ad(2422, 234);
            s.copy_ad(2423, 113);
            s.store_scalar(2424, p.p294);
            s.store_scalar(2425, p.p296);
            s.store_scalar(2426, p.p295);
            s.store_scalar(2427, p.p292);
            s.store_scalar(2428, 4.0);
            s.store_scalar(2429, 600.0);
            s.copy_ad(2430, 112);
            s.store_scalar(2431, (p.p0 * (1.0 - p.p311)));
            s.store_scalar(2432, p.p2);
            s.store_scalar(2433, p.p293);
            s.store_scalar(2434, 0.0);
            s.store_scalar(2435, p.p299);
            s.store_scalar(2436, p.p300);
            s.store_scalar(2437, p.p298);
            s.store_scalar(2438, p.p297);
            s.store_scalar(2439, 0.0);
            s.store_scalar(2440, 0.0);
            s.store_scalar(2441, p.p6);
            s.store_scalar(2442, 0.0);
            s.store_scalar(2443, 0.0);
            s.store_scalar(2444, 0.0);
            s.store_scalar(2445, 0.0);
            s.store_scalar(2446, 0.0);
            s.store_scalar(2447, 0.0);
            s.store_scalar(2448, 0.0);
            s.store_scalar(2449, 0.0);
            s.store_scalar(2450, 0.0);
            s.store_scalar(2451, 0.0);
            s.store_scalar(2452, 0.0);
            s.store_scalar(2453, 0.0);
            s.store_scalar(2454, 0.0);
            s.store_scalar(2455, 0.0);
            s.store_scalar(2456, 0.0);
            s.store_scalar(2457, 0.0);
            s.store_scalar(2458, 0.0);
        }


        *var_guard461_slot = var_guard461;
        *var_vsch_slot = var_vsch;
        *var_vsch_db0_slot = var_vsch_db0;
        *var_vsch_db1_slot = var_vsch_db1;
        *var_vsch_db10_slot = var_vsch_db10;
        *var_vsch_db11_slot = var_vsch_db11;
        *var_vsch_db12_slot = var_vsch_db12;
        *var_vsch_db13_slot = var_vsch_db13;
        *var_vsch_db14_slot = var_vsch_db14;
        *var_vsch_db15_slot = var_vsch_db15;
        *var_vsch_db16_slot = var_vsch_db16;
        *var_vsch_db17_slot = var_vsch_db17;
        *var_vsch_db18_slot = var_vsch_db18;
        *var_vsch_db19_slot = var_vsch_db19;
        *var_vsch_db2_slot = var_vsch_db2;
        *var_vsch_db20_slot = var_vsch_db20;
        *var_vsch_db21_slot = var_vsch_db21;
        *var_vsch_db22_slot = var_vsch_db22;
        *var_vsch_db23_slot = var_vsch_db23;
        *var_vsch_db24_slot = var_vsch_db24;
        *var_vsch_db25_slot = var_vsch_db25;
        *var_vsch_db26_slot = var_vsch_db26;
        *var_vsch_db27_slot = var_vsch_db27;
        *var_vsch_db28_slot = var_vsch_db28;
        *var_vsch_db29_slot = var_vsch_db29;
        *var_vsch_db3_slot = var_vsch_db3;
        *var_vsch_db30_slot = var_vsch_db30;
        *var_vsch_db31_slot = var_vsch_db31;
        *var_vsch_db32_slot = var_vsch_db32;
        *var_vsch_db33_slot = var_vsch_db33;
        *var_vsch_db34_slot = var_vsch_db34;
        *var_vsch_db35_slot = var_vsch_db35;
        *var_vsch_db4_slot = var_vsch_db4;
        *var_vsch_db5_slot = var_vsch_db5;
        *var_vsch_db6_slot = var_vsch_db6;
        *var_vsch_db7_slot = var_vsch_db7;
        *var_vsch_db8_slot = var_vsch_db8;
        *var_vsch_db9_slot = var_vsch_db9;
        *var_vsch_dn0_slot = var_vsch_dn0;
        *var_vsch_dn1_slot = var_vsch_dn1;
        *var_vsch_dn10_slot = var_vsch_dn10;
        *var_vsch_dn11_slot = var_vsch_dn11;
        *var_vsch_dn12_slot = var_vsch_dn12;
        *var_vsch_dn13_slot = var_vsch_dn13;
        *var_vsch_dn14_slot = var_vsch_dn14;
        *var_vsch_dn15_slot = var_vsch_dn15;
        *var_vsch_dn16_slot = var_vsch_dn16;
        *var_vsch_dn17_slot = var_vsch_dn17;
        *var_vsch_dn18_slot = var_vsch_dn18;
        *var_vsch_dn19_slot = var_vsch_dn19;
        *var_vsch_dn2_slot = var_vsch_dn2;
        *var_vsch_dn20_slot = var_vsch_dn20;
        *var_vsch_dn21_slot = var_vsch_dn21;
        *var_vsch_dn22_slot = var_vsch_dn22;
        *var_vsch_dn23_slot = var_vsch_dn23;
        *var_vsch_dn24_slot = var_vsch_dn24;
        *var_vsch_dn25_slot = var_vsch_dn25;
        *var_vsch_dn26_slot = var_vsch_dn26;
        *var_vsch_dn27_slot = var_vsch_dn27;
        *var_vsch_dn28_slot = var_vsch_dn28;
        *var_vsch_dn29_slot = var_vsch_dn29;
        *var_vsch_dn3_slot = var_vsch_dn3;
        *var_vsch_dn4_slot = var_vsch_dn4;
        *var_vsch_dn5_slot = var_vsch_dn5;
        *var_vsch_dn6_slot = var_vsch_dn6;
        *var_vsch_dn7_slot = var_vsch_dn7;
        *var_vsch_dn8_slot = var_vsch_dn8;
        *var_vsch_dn9_slot = var_vsch_dn9;
        *var_vschfc2_slot = var_vschfc2;
        *var_vschfc2_db0_slot = var_vschfc2_db0;
        *var_vschfc2_db1_slot = var_vschfc2_db1;
        *var_vschfc2_db10_slot = var_vschfc2_db10;
        *var_vschfc2_db11_slot = var_vschfc2_db11;
        *var_vschfc2_db12_slot = var_vschfc2_db12;
        *var_vschfc2_db13_slot = var_vschfc2_db13;
        *var_vschfc2_db14_slot = var_vschfc2_db14;
        *var_vschfc2_db15_slot = var_vschfc2_db15;
        *var_vschfc2_db16_slot = var_vschfc2_db16;
        *var_vschfc2_db17_slot = var_vschfc2_db17;
        *var_vschfc2_db18_slot = var_vschfc2_db18;
        *var_vschfc2_db19_slot = var_vschfc2_db19;
        *var_vschfc2_db2_slot = var_vschfc2_db2;
        *var_vschfc2_db20_slot = var_vschfc2_db20;
        *var_vschfc2_db21_slot = var_vschfc2_db21;
        *var_vschfc2_db22_slot = var_vschfc2_db22;
        *var_vschfc2_db23_slot = var_vschfc2_db23;
        *var_vschfc2_db24_slot = var_vschfc2_db24;
        *var_vschfc2_db25_slot = var_vschfc2_db25;
        *var_vschfc2_db26_slot = var_vschfc2_db26;
        *var_vschfc2_db27_slot = var_vschfc2_db27;
        *var_vschfc2_db28_slot = var_vschfc2_db28;
        *var_vschfc2_db29_slot = var_vschfc2_db29;
        *var_vschfc2_db3_slot = var_vschfc2_db3;
        *var_vschfc2_db30_slot = var_vschfc2_db30;
        *var_vschfc2_db31_slot = var_vschfc2_db31;
        *var_vschfc2_db32_slot = var_vschfc2_db32;
        *var_vschfc2_db33_slot = var_vschfc2_db33;
        *var_vschfc2_db34_slot = var_vschfc2_db34;
        *var_vschfc2_db35_slot = var_vschfc2_db35;
        *var_vschfc2_db4_slot = var_vschfc2_db4;
        *var_vschfc2_db5_slot = var_vschfc2_db5;
        *var_vschfc2_db6_slot = var_vschfc2_db6;
        *var_vschfc2_db7_slot = var_vschfc2_db7;
        *var_vschfc2_db8_slot = var_vschfc2_db8;
        *var_vschfc2_db9_slot = var_vschfc2_db9;
        *var_vschfc2_dn0_slot = var_vschfc2_dn0;
        *var_vschfc2_dn1_slot = var_vschfc2_dn1;
        *var_vschfc2_dn10_slot = var_vschfc2_dn10;
        *var_vschfc2_dn11_slot = var_vschfc2_dn11;
        *var_vschfc2_dn12_slot = var_vschfc2_dn12;
        *var_vschfc2_dn13_slot = var_vschfc2_dn13;
        *var_vschfc2_dn14_slot = var_vschfc2_dn14;
        *var_vschfc2_dn15_slot = var_vschfc2_dn15;
        *var_vschfc2_dn16_slot = var_vschfc2_dn16;
        *var_vschfc2_dn17_slot = var_vschfc2_dn17;
        *var_vschfc2_dn18_slot = var_vschfc2_dn18;
        *var_vschfc2_dn19_slot = var_vschfc2_dn19;
        *var_vschfc2_dn2_slot = var_vschfc2_dn2;
        *var_vschfc2_dn20_slot = var_vschfc2_dn20;
        *var_vschfc2_dn21_slot = var_vschfc2_dn21;
        *var_vschfc2_dn22_slot = var_vschfc2_dn22;
        *var_vschfc2_dn23_slot = var_vschfc2_dn23;
        *var_vschfc2_dn24_slot = var_vschfc2_dn24;
        *var_vschfc2_dn25_slot = var_vschfc2_dn25;
        *var_vschfc2_dn26_slot = var_vschfc2_dn26;
        *var_vschfc2_dn27_slot = var_vschfc2_dn27;
        *var_vschfc2_dn28_slot = var_vschfc2_dn28;
        *var_vschfc2_dn29_slot = var_vschfc2_dn29;
        *var_vschfc2_dn3_slot = var_vschfc2_dn3;
        *var_vschfc2_dn4_slot = var_vschfc2_dn4;
        *var_vschfc2_dn5_slot = var_vschfc2_dn5;
        *var_vschfc2_dn6_slot = var_vschfc2_dn6;
        *var_vschfc2_dn7_slot = var_vschfc2_dn7;
        *var_vschfc2_dn8_slot = var_vschfc2_dn8;
        *var_vschfc2_dn9_slot = var_vschfc2_dn9;
        *var_vschfc3_slot = var_vschfc3;
        *var_vschfc3_db0_slot = var_vschfc3_db0;
        *var_vschfc3_db1_slot = var_vschfc3_db1;
        *var_vschfc3_db10_slot = var_vschfc3_db10;
        *var_vschfc3_db11_slot = var_vschfc3_db11;
        *var_vschfc3_db12_slot = var_vschfc3_db12;
        *var_vschfc3_db13_slot = var_vschfc3_db13;
        *var_vschfc3_db14_slot = var_vschfc3_db14;
        *var_vschfc3_db15_slot = var_vschfc3_db15;
        *var_vschfc3_db16_slot = var_vschfc3_db16;
        *var_vschfc3_db17_slot = var_vschfc3_db17;
        *var_vschfc3_db18_slot = var_vschfc3_db18;
        *var_vschfc3_db19_slot = var_vschfc3_db19;
        *var_vschfc3_db2_slot = var_vschfc3_db2;
        *var_vschfc3_db20_slot = var_vschfc3_db20;
        *var_vschfc3_db21_slot = var_vschfc3_db21;
        *var_vschfc3_db22_slot = var_vschfc3_db22;
        *var_vschfc3_db23_slot = var_vschfc3_db23;
        *var_vschfc3_db24_slot = var_vschfc3_db24;
        *var_vschfc3_db25_slot = var_vschfc3_db25;
        *var_vschfc3_db26_slot = var_vschfc3_db26;
        *var_vschfc3_db27_slot = var_vschfc3_db27;
        *var_vschfc3_db28_slot = var_vschfc3_db28;
        *var_vschfc3_db29_slot = var_vschfc3_db29;
        *var_vschfc3_db3_slot = var_vschfc3_db3;
        *var_vschfc3_db30_slot = var_vschfc3_db30;
        *var_vschfc3_db31_slot = var_vschfc3_db31;
        *var_vschfc3_db32_slot = var_vschfc3_db32;
        *var_vschfc3_db33_slot = var_vschfc3_db33;
        *var_vschfc3_db34_slot = var_vschfc3_db34;
        *var_vschfc3_db35_slot = var_vschfc3_db35;
        *var_vschfc3_db4_slot = var_vschfc3_db4;
        *var_vschfc3_db5_slot = var_vschfc3_db5;
        *var_vschfc3_db6_slot = var_vschfc3_db6;
        *var_vschfc3_db7_slot = var_vschfc3_db7;
        *var_vschfc3_db8_slot = var_vschfc3_db8;
        *var_vschfc3_db9_slot = var_vschfc3_db9;
        *var_vschfc3_dn0_slot = var_vschfc3_dn0;
        *var_vschfc3_dn1_slot = var_vschfc3_dn1;
        *var_vschfc3_dn10_slot = var_vschfc3_dn10;
        *var_vschfc3_dn11_slot = var_vschfc3_dn11;
        *var_vschfc3_dn12_slot = var_vschfc3_dn12;
        *var_vschfc3_dn13_slot = var_vschfc3_dn13;
        *var_vschfc3_dn14_slot = var_vschfc3_dn14;
        *var_vschfc3_dn15_slot = var_vschfc3_dn15;
        *var_vschfc3_dn16_slot = var_vschfc3_dn16;
        *var_vschfc3_dn17_slot = var_vschfc3_dn17;
        *var_vschfc3_dn18_slot = var_vschfc3_dn18;
        *var_vschfc3_dn19_slot = var_vschfc3_dn19;
        *var_vschfc3_dn2_slot = var_vschfc3_dn2;
        *var_vschfc3_dn20_slot = var_vschfc3_dn20;
        *var_vschfc3_dn21_slot = var_vschfc3_dn21;
        *var_vschfc3_dn22_slot = var_vschfc3_dn22;
        *var_vschfc3_dn23_slot = var_vschfc3_dn23;
        *var_vschfc3_dn24_slot = var_vschfc3_dn24;
        *var_vschfc3_dn25_slot = var_vschfc3_dn25;
        *var_vschfc3_dn26_slot = var_vschfc3_dn26;
        *var_vschfc3_dn27_slot = var_vschfc3_dn27;
        *var_vschfc3_dn28_slot = var_vschfc3_dn28;
        *var_vschfc3_dn29_slot = var_vschfc3_dn29;
        *var_vschfc3_dn3_slot = var_vschfc3_dn3;
        *var_vschfc3_dn4_slot = var_vschfc3_dn4;
        *var_vschfc3_dn5_slot = var_vschfc3_dn5;
        *var_vschfc3_dn6_slot = var_vschfc3_dn6;
        *var_vschfc3_dn7_slot = var_vschfc3_dn7;
        *var_vschfc3_dn8_slot = var_vschfc3_dn8;
        *var_vschfc3_dn9_slot = var_vschfc3_dn9;
        *var_vschfc4_slot = var_vschfc4;
        *var_vschfc4_db0_slot = var_vschfc4_db0;
        *var_vschfc4_db1_slot = var_vschfc4_db1;
        *var_vschfc4_db10_slot = var_vschfc4_db10;
        *var_vschfc4_db11_slot = var_vschfc4_db11;
        *var_vschfc4_db12_slot = var_vschfc4_db12;
        *var_vschfc4_db13_slot = var_vschfc4_db13;
        *var_vschfc4_db14_slot = var_vschfc4_db14;
        *var_vschfc4_db15_slot = var_vschfc4_db15;
        *var_vschfc4_db16_slot = var_vschfc4_db16;
        *var_vschfc4_db17_slot = var_vschfc4_db17;
        *var_vschfc4_db18_slot = var_vschfc4_db18;
        *var_vschfc4_db19_slot = var_vschfc4_db19;
        *var_vschfc4_db2_slot = var_vschfc4_db2;
        *var_vschfc4_db20_slot = var_vschfc4_db20;
        *var_vschfc4_db21_slot = var_vschfc4_db21;
        *var_vschfc4_db22_slot = var_vschfc4_db22;
        *var_vschfc4_db23_slot = var_vschfc4_db23;
        *var_vschfc4_db24_slot = var_vschfc4_db24;
        *var_vschfc4_db25_slot = var_vschfc4_db25;
        *var_vschfc4_db26_slot = var_vschfc4_db26;
        *var_vschfc4_db27_slot = var_vschfc4_db27;
        *var_vschfc4_db28_slot = var_vschfc4_db28;
        *var_vschfc4_db29_slot = var_vschfc4_db29;
        *var_vschfc4_db3_slot = var_vschfc4_db3;
        *var_vschfc4_db30_slot = var_vschfc4_db30;
        *var_vschfc4_db31_slot = var_vschfc4_db31;
        *var_vschfc4_db32_slot = var_vschfc4_db32;
        *var_vschfc4_db33_slot = var_vschfc4_db33;
        *var_vschfc4_db34_slot = var_vschfc4_db34;
        *var_vschfc4_db35_slot = var_vschfc4_db35;
        *var_vschfc4_db4_slot = var_vschfc4_db4;
        *var_vschfc4_db5_slot = var_vschfc4_db5;
        *var_vschfc4_db6_slot = var_vschfc4_db6;
        *var_vschfc4_db7_slot = var_vschfc4_db7;
        *var_vschfc4_db8_slot = var_vschfc4_db8;
        *var_vschfc4_db9_slot = var_vschfc4_db9;
        *var_vschfc4_dn0_slot = var_vschfc4_dn0;
        *var_vschfc4_dn1_slot = var_vschfc4_dn1;
        *var_vschfc4_dn10_slot = var_vschfc4_dn10;
        *var_vschfc4_dn11_slot = var_vschfc4_dn11;
        *var_vschfc4_dn12_slot = var_vschfc4_dn12;
        *var_vschfc4_dn13_slot = var_vschfc4_dn13;
        *var_vschfc4_dn14_slot = var_vschfc4_dn14;
        *var_vschfc4_dn15_slot = var_vschfc4_dn15;
        *var_vschfc4_dn16_slot = var_vschfc4_dn16;
        *var_vschfc4_dn17_slot = var_vschfc4_dn17;
        *var_vschfc4_dn18_slot = var_vschfc4_dn18;
        *var_vschfc4_dn19_slot = var_vschfc4_dn19;
        *var_vschfc4_dn2_slot = var_vschfc4_dn2;
        *var_vschfc4_dn20_slot = var_vschfc4_dn20;
        *var_vschfc4_dn21_slot = var_vschfc4_dn21;
        *var_vschfc4_dn22_slot = var_vschfc4_dn22;
        *var_vschfc4_dn23_slot = var_vschfc4_dn23;
        *var_vschfc4_dn24_slot = var_vschfc4_dn24;
        *var_vschfc4_dn25_slot = var_vschfc4_dn25;
        *var_vschfc4_dn26_slot = var_vschfc4_dn26;
        *var_vschfc4_dn27_slot = var_vschfc4_dn27;
        *var_vschfc4_dn28_slot = var_vschfc4_dn28;
        *var_vschfc4_dn29_slot = var_vschfc4_dn29;
        *var_vschfc4_dn3_slot = var_vschfc4_dn3;
        *var_vschfc4_dn4_slot = var_vschfc4_dn4;
        *var_vschfc4_dn5_slot = var_vschfc4_dn5;
        *var_vschfc4_dn6_slot = var_vschfc4_dn6;
        *var_vschfc4_dn7_slot = var_vschfc4_dn7;
        *var_vschfc4_dn8_slot = var_vschfc4_dn8;
        *var_vschfc4_dn9_slot = var_vschfc4_dn9;
        *var_vschfc5_slot = var_vschfc5;
        *var_vschfc5_db0_slot = var_vschfc5_db0;
        *var_vschfc5_db1_slot = var_vschfc5_db1;
        *var_vschfc5_db10_slot = var_vschfc5_db10;
        *var_vschfc5_db11_slot = var_vschfc5_db11;
        *var_vschfc5_db12_slot = var_vschfc5_db12;
        *var_vschfc5_db13_slot = var_vschfc5_db13;
        *var_vschfc5_db14_slot = var_vschfc5_db14;
        *var_vschfc5_db15_slot = var_vschfc5_db15;
        *var_vschfc5_db16_slot = var_vschfc5_db16;
        *var_vschfc5_db17_slot = var_vschfc5_db17;
        *var_vschfc5_db18_slot = var_vschfc5_db18;
        *var_vschfc5_db19_slot = var_vschfc5_db19;
        *var_vschfc5_db2_slot = var_vschfc5_db2;
        *var_vschfc5_db20_slot = var_vschfc5_db20;
        *var_vschfc5_db21_slot = var_vschfc5_db21;
        *var_vschfc5_db22_slot = var_vschfc5_db22;
        *var_vschfc5_db23_slot = var_vschfc5_db23;
        *var_vschfc5_db24_slot = var_vschfc5_db24;
        *var_vschfc5_db25_slot = var_vschfc5_db25;
        *var_vschfc5_db26_slot = var_vschfc5_db26;
        *var_vschfc5_db27_slot = var_vschfc5_db27;
        *var_vschfc5_db28_slot = var_vschfc5_db28;
        *var_vschfc5_db29_slot = var_vschfc5_db29;
        *var_vschfc5_db3_slot = var_vschfc5_db3;
        *var_vschfc5_db30_slot = var_vschfc5_db30;
        *var_vschfc5_db31_slot = var_vschfc5_db31;
        *var_vschfc5_db32_slot = var_vschfc5_db32;
        *var_vschfc5_db33_slot = var_vschfc5_db33;
        *var_vschfc5_db34_slot = var_vschfc5_db34;
        *var_vschfc5_db35_slot = var_vschfc5_db35;
        *var_vschfc5_db4_slot = var_vschfc5_db4;
        *var_vschfc5_db5_slot = var_vschfc5_db5;
        *var_vschfc5_db6_slot = var_vschfc5_db6;
        *var_vschfc5_db7_slot = var_vschfc5_db7;
        *var_vschfc5_db8_slot = var_vschfc5_db8;
        *var_vschfc5_db9_slot = var_vschfc5_db9;
        *var_vschfc5_dn0_slot = var_vschfc5_dn0;
        *var_vschfc5_dn1_slot = var_vschfc5_dn1;
        *var_vschfc5_dn10_slot = var_vschfc5_dn10;
        *var_vschfc5_dn11_slot = var_vschfc5_dn11;
        *var_vschfc5_dn12_slot = var_vschfc5_dn12;
        *var_vschfc5_dn13_slot = var_vschfc5_dn13;
        *var_vschfc5_dn14_slot = var_vschfc5_dn14;
        *var_vschfc5_dn15_slot = var_vschfc5_dn15;
        *var_vschfc5_dn16_slot = var_vschfc5_dn16;
        *var_vschfc5_dn17_slot = var_vschfc5_dn17;
        *var_vschfc5_dn18_slot = var_vschfc5_dn18;
        *var_vschfc5_dn19_slot = var_vschfc5_dn19;
        *var_vschfc5_dn2_slot = var_vschfc5_dn2;
        *var_vschfc5_dn20_slot = var_vschfc5_dn20;
        *var_vschfc5_dn21_slot = var_vschfc5_dn21;
        *var_vschfc5_dn22_slot = var_vschfc5_dn22;
        *var_vschfc5_dn23_slot = var_vschfc5_dn23;
        *var_vschfc5_dn24_slot = var_vschfc5_dn24;
        *var_vschfc5_dn25_slot = var_vschfc5_dn25;
        *var_vschfc5_dn26_slot = var_vschfc5_dn26;
        *var_vschfc5_dn27_slot = var_vschfc5_dn27;
        *var_vschfc5_dn28_slot = var_vschfc5_dn28;
        *var_vschfc5_dn29_slot = var_vschfc5_dn29;
        *var_vschfc5_dn3_slot = var_vschfc5_dn3;
        *var_vschfc5_dn4_slot = var_vschfc5_dn4;
        *var_vschfc5_dn5_slot = var_vschfc5_dn5;
        *var_vschfc5_dn6_slot = var_vschfc5_dn6;
        *var_vschfc5_dn7_slot = var_vschfc5_dn7;
        *var_vschfc5_dn8_slot = var_vschfc5_dn8;
        *var_vschfc5_dn9_slot = var_vschfc5_dn9;
    }

    pub(super) fn stamp_transient_block_113(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.v[2418] != 0.0) {
            s.store_scalar(2459, 0.0);
            s.store_scalar(2460, 0.0);
            s.store_scalar(2461, 0.0);
            s.store_scalar(2462, 0.0);
            s.store_scalar(2463, 0.0);
            s.store_scalar(2464, 0.0);
            s.store_scalar(2465, 0.0);
            s.store_scalar(2466, 0.0);
            s.store_scalar(2467, 0.0);
            s.store_scalar(2468, 0.0);
            s.store_scalar(2469, 0.0);
            s.store_scalar(2470, 0.0);
            s.store_scalar(2471, 0.0);
            s.store_scalar(2472, 0.0);
            s.store_scalar(2473, 0.0);
            s.store_scalar(2474, 0.0);
            s.store_mul_scaled_ad_lhs(2454, A::div(s.ad_value(2439), s.ad_value(2423)), 2440, -1.0);
        }

        if (s.v[2418] != 0.0) {
            if ((!(s.v[2454] > 50.0)) && (!(s.v[2454] < (-50.0)))) {
                s.store_exp(2444, 2454);
            } else {
                if ((!(s.v[2454] > 50.0)) && (s.v[2454] < (-50.0))) {
                    s.store_scalar(2444, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2454] > 50.0) {
                        s.store_scaled_offset(2444, 2454, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2444, 0.0);
                    }
                }
            }
        }

        if (s.v[2418] != 0.0) {
            s.store_add_scaled_product_right_ad(2450, 2454, 1.0, 2428, A::sub_scaled_inputs(s.ad_value(2422), -1.0, s.ad_value(2429), 1.0), 1.0);
            s.store_add_scaled_product_indices(2451, 2454, 1.0, 2428, 2429, -1.0);
        }

        if (s.v[2418] != 0.0) {
            if ((!(s.v[2450] > 50.0)) && (!(s.v[2450] < (-50.0)))) {
                s.store_exp(2452, 2450);
            } else {
                if ((!(s.v[2450] > 50.0)) && (s.v[2450] < (-50.0))) {
                    s.store_scalar(2452, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2450] > 50.0) {
                        s.store_scaled_offset(2452, 2450, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2452, 0.0);
                    }
                }
            }
        }

        if (s.v[2418] != 0.0) {
            if ((!(s.v[2451] > 50.0)) && (!(s.v[2451] < (-50.0)))) {
                s.store_exp(2453, 2451);
            } else {
                if ((!(s.v[2451] > 50.0)) && (s.v[2451] < (-50.0))) {
                    s.store_scalar(2453, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2451] > 50.0) {
                        s.store_scaled_offset(2453, 2451, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2453, 0.0);
                    }
                }
            }
        }

        if (s.v[2418] != 0.0) {
            s.store_sub(2446, 2452, 2453);
            s.store_mul_ad_product_lhs_mixed_ai(2420, A::mul3(s.ad_value(2441), s.ad_value(2431), s.ad_value(2432)), 2433, 2430);
            s.store_add_scaled_product_left_ad(2456, 2454, 1.0, A::div(s.ad_value(2427), s.ad_value(2423)), 2422, 1.0);
        }

        if (s.v[2418] != 0.0) {
            if ((!(s.v[2456] > 50.0)) && (!(s.v[2456] < (-50.0)))) {
                s.store_exp(2457, 2456);
            } else {
                if ((!(s.v[2456] > 50.0)) && (s.v[2456] < (-50.0))) {
                    s.store_scalar(2457, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2456] > 50.0) {
                        s.store_scaled_offset(2457, 2456, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2457, 0.0);
                    }
                }
            }
        }

        s.b[2475] = (s.v[2426] == 1.0);
        s.store_scalar(2475, if s.b[2475] { 1.0 } else { 0.0 });

        if ((s.v[2418] != 0.0) && s.b[2475]) {
            s.store_mul_sub_ad_rhs(2447, 2420, A::add_scaled_product(s.ad_value(2457), 1.0, s.ad_value(2434), s.ad_value(2446), (-1.0)), s.ad_value(2444));
        }

        if ((s.v[2418] != 0.0) && (!s.b[2475])) {
            s.store_add_scaled_product_right_ad(2461, 2454, 1.0, 2428, A::sub_scaled_inputs(s.ad_value(2424), -1.0, s.ad_value(2429), 1.0), 1.0);
        }

        if ((s.v[2418] != 0.0) && (!s.b[2475])) {
            if ((!(s.v[2461] > 50.0)) && (!(s.v[2461] < (-50.0)))) {
                s.store_exp(2462, 2461);
            } else {
                if ((!(s.v[2461] > 50.0)) && (s.v[2461] < (-50.0))) {
                    s.store_scalar(2462, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2461] > 50.0) {
                        s.store_scaled_offset(2462, 2461, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2462, 0.0);
                    }
                }
            }
        }

        if ((s.v[2418] != 0.0) && (!s.b[2475])) {
            s.store_sub(2463, 2462, 2453);
            s.store_add_scaled_product_left_ad(2464, 2454, 1.0, A::div(s.ad_value(2427), s.ad_value(2423)), 2424, 1.0);
        }

        if ((s.v[2418] != 0.0) && (!s.b[2475])) {
            if ((!(s.v[2464] > 50.0)) && (!(s.v[2464] < (-50.0)))) {
                s.store_exp(2465, 2464);
            } else {
                if ((!(s.v[2464] > 50.0)) && (s.v[2464] < (-50.0))) {
                    s.store_scalar(2465, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2464] > 50.0) {
                        s.store_scaled_offset(2465, 2464, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2465, 0.0);
                    }
                }
            }
        }

        if ((s.v[2418] != 0.0) && (!s.b[2475])) {
            s.store_sub_ad_lhs(2466, A::add_scaled_product(s.ad_value(2465), 1.0, s.ad_value(2434), s.ad_value(2463), (-1.0)), 2444);
            s.store_mul_sub_ad_rhs(2467, 2420, A::add_scaled_product(s.ad_value(2457), 1.0, s.ad_value(2434), s.ad_value(2446), (-1.0)), s.ad_value(2444));
        }

        s.b[2476] = (s.v[2426] > 0.0);
        s.store_scalar(2476, if s.b[2476] { 1.0 } else { 0.0 });

        if (((s.v[2418] != 0.0) && (!s.b[2475])) && s.b[2476]) {
            s.store_mul(2460, 2426, 2427);
            s.store_add_scaled_product_left_ad(2468, 2454, 1.0, A::div(s.ad_value(2460), s.ad_value(2423)), 2424, 1.0);
        }

        if (((s.v[2418] != 0.0) && (!s.b[2475])) && s.b[2476]) {
            if ((!(s.v[2468] > 50.0)) && (!(s.v[2468] < (-50.0)))) {
                s.store_exp(2469, 2468);
            } else {
                if ((!(s.v[2468] > 50.0)) && (s.v[2468] < (-50.0))) {
                    s.store_scalar(2469, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2468] > 50.0) {
                        s.store_scaled_offset(2469, 2468, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2469, 0.0);
                    }
                }
            }
        }

        if (((s.v[2418] != 0.0) && (!s.b[2475])) && s.b[2476]) {
            s.store_sub_ad_lhs(2470, A::add_scaled_product(s.ad_value(2469), 1.0, s.ad_value(2434), s.ad_value(2463), (-1.0)), 2444);
            s.store_add_scaled_product_left_ad(2471, 2454, 1.0, A::div(s.ad_value(2460), s.ad_value(2423)), 2422, 1.0);
        }

        if (((s.v[2418] != 0.0) && (!s.b[2475])) && s.b[2476]) {
            if ((!(s.v[2471] > 50.0)) && (!(s.v[2471] < (-50.0)))) {
                s.store_exp(2472, 2471);
            } else {
                if ((!(s.v[2471] > 50.0)) && (s.v[2471] < (-50.0))) {
                    s.store_scalar(2472, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2471] > 50.0) {
                        s.store_scaled_offset(2472, 2471, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2472, 0.0);
                    }
                }
            }
        }

        if (((s.v[2418] != 0.0) && (!s.b[2475])) && s.b[2476]) {
            s.store_div_scaled_product_indices(2473, 2420, 2466, 1.0, 2470, 1.0);
            s.store_mul_sub_ad_rhs(2474, 2473, A::add_scaled_product(s.ad_value(2472), 1.0, s.ad_value(2434), s.ad_value(2446), (-1.0)), s.ad_value(2444));
        }

        if (((s.v[2418] != 0.0) && (!s.b[2475])) && (!s.b[2476])) {
            s.store_mul(2474, 2420, 2466);
        }

        if ((s.v[2418] != 0.0) && (!s.b[2475])) {
            s.store_mul_square_lhs(2443, 2425, 2423);
            s.store_div_scaled_inputs3_indices(2455, 2422, 1.0, 2424, -1.0, 2443, (-(-0.5)), 2443, 1.0);
        }

        s.b[2477] = (s.v[2455] > 50.0);
        s.store_scalar(2477, if s.b[2477] { 1.0 } else { 0.0 });

        if (((s.v[2418] != 0.0) && (!s.b[2475])) && s.b[2477]) {
            s.store_scalar(2445, 0.0);
        }

        s.b[2478] = (s.v[2455] < (-50.0));
        s.store_scalar(2478, if s.b[2478] { 1.0 } else { 0.0 });

        if ((((s.v[2418] != 0.0) && (!s.b[2475])) && (!s.b[2477])) && s.b[2478]) {
            s.store_scalar(2445, 1.0);
        }

        if ((((s.v[2418] != 0.0) && (!s.b[2475])) && (!s.b[2477])) && (!s.b[2478])) {
            s.store_div_from_scalar_offset_ad(2445, 1.0, A::exp(s.ad_value(2455)), 1.0);
        }

        if ((s.v[2418] != 0.0) && (!s.b[2475])) {
            s.store_add_scaled_product_value_ad(2447, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(2445), s.ad_value(2474)), 1.0, 2445, 2467, 1.0);
        }

        if (s.v[2418] != 0.0) {
            s.store_div_scaled_inputs_mixed_ia(2448, 2422, -1.0, A::pow(A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::mul(A::div(s.ad_value(2422), s.ad_value(2435)), A::tanh_scaled_input(A::div(s.ad_value(2422), s.ad_value(2435)), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt_square_offset(A::div(s.ad_value(2422), s.ad_value(2435)), p.p53)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(2436)), 1.0), A::div_from_scalar(1.0, s.ad_value(2436))), 1.0);
        }

        if (s.v[2418] != 0.0) {
            s.store_mul_ad_product_lhs_mixed_ai(2421, A::mul3_scaled_output(s.ad_value(2441), s.ad_value(2431), s.ad_value(2432), -1.0), 2437, 2430);
            s.store_mul_div_lhs(2458, 2438, 2423, 2448);
        }

        if (s.v[2418] != 0.0) {
            if ((!(s.v[2458] > 50.0)) && (!(s.v[2458] < (-50.0)))) {
                s.store_exp(2459, 2458);
            } else {
                if ((!(s.v[2458] > 50.0)) && (s.v[2458] < (-50.0))) {
                    s.store_scalar(2459, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2458] > 50.0) {
                        s.store_scaled_offset(2459, 2458, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2459, 0.0);
                    }
                }
            }
        }

        if (s.v[2418] != 0.0) {
            s.store_mul_offset_rhs(2449, 2421, 2459, (-1.0));
            s.store_add(2442, 2447, 2449);
            s.copy_ad(2419, 2442);
            s.copy_ad(235, 2419);
        }

        s.b[2479] = (p.p301 == 1.0);
        s.store_scalar(2479, if s.b[2479] { 1.0 } else { 0.0 });

        if ((s.v[2418] != 0.0) && s.b[2479]) {
            s.store_scalar(2480, 0.0);
            s.store_scalar(2481, 0.0);
            s.store_scalar(2482, 0.0);
            s.copy_ad(2483, 234);
            s.copy_ad(2484, 113);
            s.store_scalar(2485, 1.0);
            s.store_scalar(2486, 10.0);
            s.store_scalar(2487, 1.0);
            s.store_scalar(2488, 0.0);
            s.store_scalar(2489, 4.0);
            s.store_scalar(2490, 600.0);
            s.copy_ad(2491, 112);
            s.store_scalar(2492, (p.p0 * (1.0 - p.p311)));
            s.store_scalar(2493, p.p2);
            s.store_scalar(2494, 0.0);
            s.store_scalar(2495, 0.0);
            s.store_scalar(2496, p.p304);
            s.store_scalar(2497, p.p305);
            s.store_scalar(2498, p.p303);
            s.store_scalar(2499, p.p302);
            s.store_scalar(2500, 0.0);
            s.store_scalar(2501, 0.0);
            s.store_scalar(2502, p.p6);
        }

    }

    pub(super) fn stamp_transient_block_114(
        s: &mut Scratch,
        p: &Parameters,
        var_vsch: f64,
        var_guard473_slot: &mut f64,
    ) {
        let mut var_guard473: f64 = *var_guard473_slot;

        if ((s.v[2418] != 0.0) && s.b[2479]) {
            s.store_scalar(2503, 0.0);
            s.store_scalar(2504, 0.0);
            s.store_scalar(2505, 0.0);
            s.store_scalar(2506, 0.0);
            s.store_scalar(2507, 0.0);
            s.store_scalar(2508, 0.0);
            s.store_scalar(2509, 0.0);
            s.store_scalar(2510, 0.0);
            s.store_scalar(2511, 0.0);
            s.store_scalar(2512, 0.0);
            s.store_scalar(2513, 0.0);
            s.store_scalar(2514, 0.0);
            s.store_scalar(2515, 0.0);
            s.store_scalar(2516, 0.0);
            s.store_scalar(2517, 0.0);
            s.store_scalar(2518, 0.0);
            s.store_scalar(2519, 0.0);
            s.store_scalar(2520, 0.0);
            s.store_scalar(2521, 0.0);
            s.store_scalar(2522, 0.0);
            s.store_scalar(2523, 0.0);
            s.store_scalar(2524, 0.0);
            s.store_scalar(2525, 0.0);
            s.store_scalar(2526, 0.0);
            s.store_scalar(2527, 0.0);
            s.store_scalar(2528, 0.0);
            s.store_scalar(2529, 0.0);
            s.store_scalar(2530, 0.0);
            s.store_scalar(2531, 0.0);
            s.store_scalar(2532, 0.0);
            s.store_scalar(2533, 0.0);
            s.store_scalar(2534, 0.0);
            s.store_scalar(2535, 0.0);
            s.store_mul_scaled_ad_lhs(2515, A::div(s.ad_value(2500), s.ad_value(2484)), 2501, -1.0);
        }

        if ((s.v[2418] != 0.0) && s.b[2479]) {
            if ((!(s.v[2515] > 50.0)) && (!(s.v[2515] < (-50.0)))) {
                s.store_exp(2505, 2515);
            } else {
                if ((!(s.v[2515] > 50.0)) && (s.v[2515] < (-50.0))) {
                    s.store_scalar(2505, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2515] > 50.0) {
                        s.store_scaled_offset(2505, 2515, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2505, 0.0);
                    }
                }
            }
        }

        if ((s.v[2418] != 0.0) && s.b[2479]) {
            s.store_add_scaled_product_right_ad(2511, 2515, 1.0, 2489, A::sub_scaled_inputs(s.ad_value(2483), -1.0, s.ad_value(2490), 1.0), 1.0);
            s.store_add_scaled_product_indices(2512, 2515, 1.0, 2489, 2490, -1.0);
        }

        if ((s.v[2418] != 0.0) && s.b[2479]) {
            if ((!(s.v[2511] > 50.0)) && (!(s.v[2511] < (-50.0)))) {
                s.store_exp(2513, 2511);
            } else {
                if ((!(s.v[2511] > 50.0)) && (s.v[2511] < (-50.0))) {
                    s.store_scalar(2513, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2511] > 50.0) {
                        s.store_scaled_offset(2513, 2511, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2513, 0.0);
                    }
                }
            }
        }

        if ((s.v[2418] != 0.0) && s.b[2479]) {
            if ((!(s.v[2512] > 50.0)) && (!(s.v[2512] < (-50.0)))) {
                s.store_exp(2514, 2512);
            } else {
                if ((!(s.v[2512] > 50.0)) && (s.v[2512] < (-50.0))) {
                    s.store_scalar(2514, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2512] > 50.0) {
                        s.store_scaled_offset(2514, 2512, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2514, 0.0);
                    }
                }
            }
        }

        if ((s.v[2418] != 0.0) && s.b[2479]) {
            s.store_sub(2507, 2513, 2514);
            s.store_mul_ad_product_lhs_mixed_ai(2481, A::mul3(s.ad_value(2502), s.ad_value(2492), s.ad_value(2493)), 2494, 2491);
            s.store_add_scaled_product_left_ad(2517, 2515, 1.0, A::div(s.ad_value(2488), s.ad_value(2484)), 2483, 1.0);
        }

        if ((s.v[2418] != 0.0) && s.b[2479]) {
            if ((!(s.v[2517] > 50.0)) && (!(s.v[2517] < (-50.0)))) {
                s.store_exp(2518, 2517);
            } else {
                if ((!(s.v[2517] > 50.0)) && (s.v[2517] < (-50.0))) {
                    s.store_scalar(2518, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2517] > 50.0) {
                        s.store_scaled_offset(2518, 2517, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2518, 0.0);
                    }
                }
            }
        }

        s.b[2536] = (s.v[2487] == 1.0);
        s.store_scalar(2536, if s.b[2536] { 1.0 } else { 0.0 });

        if (((s.v[2418] != 0.0) && s.b[2479]) && s.b[2536]) {
            s.store_mul_sub_ad_rhs(2508, 2481, A::add_scaled_product(s.ad_value(2518), 1.0, s.ad_value(2495), s.ad_value(2507), (-1.0)), s.ad_value(2505));
        }

        if (((s.v[2418] != 0.0) && s.b[2479]) && (!s.b[2536])) {
            s.store_add_scaled_product_right_ad(2522, 2515, 1.0, 2489, A::sub_scaled_inputs(s.ad_value(2485), -1.0, s.ad_value(2490), 1.0), 1.0);
        }

        if (((s.v[2418] != 0.0) && s.b[2479]) && (!s.b[2536])) {
            if ((!(s.v[2522] > 50.0)) && (!(s.v[2522] < (-50.0)))) {
                s.store_exp(2523, 2522);
            } else {
                if ((!(s.v[2522] > 50.0)) && (s.v[2522] < (-50.0))) {
                    s.store_scalar(2523, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2522] > 50.0) {
                        s.store_scaled_offset(2523, 2522, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2523, 0.0);
                    }
                }
            }
        }

        if (((s.v[2418] != 0.0) && s.b[2479]) && (!s.b[2536])) {
            s.store_sub(2524, 2523, 2514);
            s.store_add_scaled_product_left_ad(2525, 2515, 1.0, A::div(s.ad_value(2488), s.ad_value(2484)), 2485, 1.0);
        }

        if (((s.v[2418] != 0.0) && s.b[2479]) && (!s.b[2536])) {
            if ((!(s.v[2525] > 50.0)) && (!(s.v[2525] < (-50.0)))) {
                s.store_exp(2526, 2525);
            } else {
                if ((!(s.v[2525] > 50.0)) && (s.v[2525] < (-50.0))) {
                    s.store_scalar(2526, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2525] > 50.0) {
                        s.store_scaled_offset(2526, 2525, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2526, 0.0);
                    }
                }
            }
        }

        if (((s.v[2418] != 0.0) && s.b[2479]) && (!s.b[2536])) {
            s.store_sub_ad_lhs(2527, A::add_scaled_product(s.ad_value(2526), 1.0, s.ad_value(2495), s.ad_value(2524), (-1.0)), 2505);
            s.store_mul_sub_ad_rhs(2528, 2481, A::add_scaled_product(s.ad_value(2518), 1.0, s.ad_value(2495), s.ad_value(2507), (-1.0)), s.ad_value(2505));
        }

        s.b[2537] = (s.v[2487] > 0.0);
        s.store_scalar(2537, if s.b[2537] { 1.0 } else { 0.0 });

        if ((((s.v[2418] != 0.0) && s.b[2479]) && (!s.b[2536])) && s.b[2537]) {
            s.store_mul(2521, 2487, 2488);
            s.store_add_scaled_product_left_ad(2529, 2515, 1.0, A::div(s.ad_value(2521), s.ad_value(2484)), 2485, 1.0);
        }

        if ((((s.v[2418] != 0.0) && s.b[2479]) && (!s.b[2536])) && s.b[2537]) {
            if ((!(s.v[2529] > 50.0)) && (!(s.v[2529] < (-50.0)))) {
                s.store_exp(2530, 2529);
            } else {
                if ((!(s.v[2529] > 50.0)) && (s.v[2529] < (-50.0))) {
                    s.store_scalar(2530, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2529] > 50.0) {
                        s.store_scaled_offset(2530, 2529, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2530, 0.0);
                    }
                }
            }
        }

        if ((((s.v[2418] != 0.0) && s.b[2479]) && (!s.b[2536])) && s.b[2537]) {
            s.store_sub_ad_lhs(2531, A::add_scaled_product(s.ad_value(2530), 1.0, s.ad_value(2495), s.ad_value(2524), (-1.0)), 2505);
            s.store_add_scaled_product_left_ad(2532, 2515, 1.0, A::div(s.ad_value(2521), s.ad_value(2484)), 2483, 1.0);
        }

        if ((((s.v[2418] != 0.0) && s.b[2479]) && (!s.b[2536])) && s.b[2537]) {
            if ((!(s.v[2532] > 50.0)) && (!(s.v[2532] < (-50.0)))) {
                s.store_exp(2533, 2532);
            } else {
                if ((!(s.v[2532] > 50.0)) && (s.v[2532] < (-50.0))) {
                    s.store_scalar(2533, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2532] > 50.0) {
                        s.store_scaled_offset(2533, 2532, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2533, 0.0);
                    }
                }
            }
        }

        if ((((s.v[2418] != 0.0) && s.b[2479]) && (!s.b[2536])) && s.b[2537]) {
            s.store_div_scaled_product_indices(2534, 2481, 2527, 1.0, 2531, 1.0);
            s.store_mul_sub_ad_rhs(2535, 2534, A::add_scaled_product(s.ad_value(2533), 1.0, s.ad_value(2495), s.ad_value(2507), (-1.0)), s.ad_value(2505));
        }

        if ((((s.v[2418] != 0.0) && s.b[2479]) && (!s.b[2536])) && (!s.b[2537])) {
            s.store_mul(2535, 2481, 2527);
        }

        if (((s.v[2418] != 0.0) && s.b[2479]) && (!s.b[2536])) {
            s.store_mul_square_lhs(2504, 2486, 2484);
            s.store_div_scaled_inputs3_indices(2516, 2483, 1.0, 2485, -1.0, 2504, (-(-0.5)), 2504, 1.0);
        }

        s.b[2538] = (s.v[2516] > 50.0);
        s.store_scalar(2538, if s.b[2538] { 1.0 } else { 0.0 });

        if ((((s.v[2418] != 0.0) && s.b[2479]) && (!s.b[2536])) && s.b[2538]) {
            s.store_scalar(2506, 0.0);
        }

        s.b[2539] = (s.v[2516] < (-50.0));
        s.store_scalar(2539, if s.b[2539] { 1.0 } else { 0.0 });

        if (((((s.v[2418] != 0.0) && s.b[2479]) && (!s.b[2536])) && (!s.b[2538])) && s.b[2539]) {
            s.store_scalar(2506, 1.0);
        }

        if (((((s.v[2418] != 0.0) && s.b[2479]) && (!s.b[2536])) && (!s.b[2538])) && (!s.b[2539])) {
            s.store_div_from_scalar_offset_ad(2506, 1.0, A::exp(s.ad_value(2516)), 1.0);
        }

        if (((s.v[2418] != 0.0) && s.b[2479]) && (!s.b[2536])) {
            s.store_add_scaled_product_value_ad(2508, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(2506), s.ad_value(2535)), 1.0, 2506, 2528, 1.0);
        }

        if ((s.v[2418] != 0.0) && s.b[2479]) {
            s.store_div_scaled_inputs_mixed_ia(2509, 2483, -1.0, A::pow(A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::mul(A::div(s.ad_value(2483), s.ad_value(2496)), A::tanh_scaled_input(A::div(s.ad_value(2483), s.ad_value(2496)), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt_square_offset(A::div(s.ad_value(2483), s.ad_value(2496)), p.p53)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(2497)), 1.0), A::div_from_scalar(1.0, s.ad_value(2497))), 1.0);
        }

        if ((s.v[2418] != 0.0) && s.b[2479]) {
            s.store_mul_ad_product_lhs_mixed_ai(2482, A::mul3_scaled_output(s.ad_value(2502), s.ad_value(2492), s.ad_value(2493), -1.0), 2498, 2491);
            s.store_mul_div_lhs(2519, 2499, 2484, 2509);
        }

        if ((s.v[2418] != 0.0) && s.b[2479]) {
            if ((!(s.v[2519] > 50.0)) && (!(s.v[2519] < (-50.0)))) {
                s.store_exp(2520, 2519);
            } else {
                if ((!(s.v[2519] > 50.0)) && (s.v[2519] < (-50.0))) {
                    s.store_scalar(2520, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2519] > 50.0) {
                        s.store_scaled_offset(2520, 2519, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2520, 0.0);
                    }
                }
            }
        }

        if ((s.v[2418] != 0.0) && s.b[2479]) {
            s.store_mul_offset_rhs(2510, 2482, 2520, (-1.0));
            s.store_add(2503, 2508, 2510);
            s.copy_ad(2480, 2503);
            s.copy_ad(238, 2480);
        }

        let assign43620_e42207: f64 = (p.p308 * p.p306);
        let assign43620_e42208: f64 = if var_vsch <= assign43620_e42207 { 1.0 } else { 0.0 };
        var_guard473 = assign43620_e42208;


        *var_guard473_slot = var_guard473;
    }

    pub(super) fn stamp_transient_block_115(
        p: &Parameters,
        var_guard461: f64,
        var_guard473: f64,
        var_vsch: f64,
        var_vsch_db0: f64,
        var_vsch_db1: f64,
        var_vsch_db10: f64,
        var_vsch_db11: f64,
        var_vsch_db12: f64,
        var_vsch_db13: f64,
        var_vsch_db14: f64,
        var_vsch_db15: f64,
        var_vsch_db16: f64,
        var_vsch_db17: f64,
        var_vsch_db18: f64,
        var_vsch_db19: f64,
        var_vsch_db2: f64,
        var_vsch_db20: f64,
        var_vsch_db21: f64,
        var_vsch_db22: f64,
        var_vsch_db23: f64,
        var_vsch_db24: f64,
        var_vsch_db25: f64,
        var_vsch_db26: f64,
        var_vsch_db27: f64,
        var_vsch_db28: f64,
        var_vsch_db29: f64,
        var_vsch_db3: f64,
        var_vsch_db30: f64,
        var_vsch_db31: f64,
        var_vsch_db32: f64,
        var_vsch_db33: f64,
        var_vsch_db34: f64,
        var_vsch_db35: f64,
        var_vsch_db4: f64,
        var_vsch_db5: f64,
        var_vsch_db6: f64,
        var_vsch_db7: f64,
        var_vsch_db8: f64,
        var_vsch_db9: f64,
        var_vsch_dn0: f64,
        var_vsch_dn1: f64,
        var_vsch_dn10: f64,
        var_vsch_dn11: f64,
        var_vsch_dn12: f64,
        var_vsch_dn13: f64,
        var_vsch_dn14: f64,
        var_vsch_dn15: f64,
        var_vsch_dn16: f64,
        var_vsch_dn17: f64,
        var_vsch_dn18: f64,
        var_vsch_dn19: f64,
        var_vsch_dn2: f64,
        var_vsch_dn20: f64,
        var_vsch_dn21: f64,
        var_vsch_dn22: f64,
        var_vsch_dn23: f64,
        var_vsch_dn24: f64,
        var_vsch_dn25: f64,
        var_vsch_dn26: f64,
        var_vsch_dn27: f64,
        var_vsch_dn28: f64,
        var_vsch_dn29: f64,
        var_vsch_dn3: f64,
        var_vsch_dn4: f64,
        var_vsch_dn5: f64,
        var_vsch_dn6: f64,
        var_vsch_dn7: f64,
        var_vsch_dn8: f64,
        var_vsch_dn9: f64,
        var_guard474_slot: &mut f64,
        var_guard475_slot: &mut f64,
        var_guard476_slot: &mut f64,
        var_qsch_slot: &mut f64,
        var_qsch0_slot: &mut f64,
        var_qsch1_slot: &mut f64,
        var_qsch1_db0_slot: &mut f64,
        var_qsch1_db1_slot: &mut f64,
        var_qsch1_db10_slot: &mut f64,
        var_qsch1_db11_slot: &mut f64,
        var_qsch1_db12_slot: &mut f64,
        var_qsch1_db13_slot: &mut f64,
        var_qsch1_db14_slot: &mut f64,
        var_qsch1_db15_slot: &mut f64,
        var_qsch1_db16_slot: &mut f64,
        var_qsch1_db17_slot: &mut f64,
        var_qsch1_db18_slot: &mut f64,
        var_qsch1_db19_slot: &mut f64,
        var_qsch1_db2_slot: &mut f64,
        var_qsch1_db20_slot: &mut f64,
        var_qsch1_db21_slot: &mut f64,
        var_qsch1_db22_slot: &mut f64,
        var_qsch1_db23_slot: &mut f64,
        var_qsch1_db24_slot: &mut f64,
        var_qsch1_db25_slot: &mut f64,
        var_qsch1_db26_slot: &mut f64,
        var_qsch1_db27_slot: &mut f64,
        var_qsch1_db28_slot: &mut f64,
        var_qsch1_db29_slot: &mut f64,
        var_qsch1_db3_slot: &mut f64,
        var_qsch1_db30_slot: &mut f64,
        var_qsch1_db31_slot: &mut f64,
        var_qsch1_db32_slot: &mut f64,
        var_qsch1_db33_slot: &mut f64,
        var_qsch1_db34_slot: &mut f64,
        var_qsch1_db35_slot: &mut f64,
        var_qsch1_db4_slot: &mut f64,
        var_qsch1_db5_slot: &mut f64,
        var_qsch1_db6_slot: &mut f64,
        var_qsch1_db7_slot: &mut f64,
        var_qsch1_db8_slot: &mut f64,
        var_qsch1_db9_slot: &mut f64,
        var_qsch1_dn0_slot: &mut f64,
        var_qsch1_dn1_slot: &mut f64,
        var_qsch1_dn10_slot: &mut f64,
        var_qsch1_dn11_slot: &mut f64,
        var_qsch1_dn12_slot: &mut f64,
        var_qsch1_dn13_slot: &mut f64,
        var_qsch1_dn14_slot: &mut f64,
        var_qsch1_dn15_slot: &mut f64,
        var_qsch1_dn16_slot: &mut f64,
        var_qsch1_dn17_slot: &mut f64,
        var_qsch1_dn18_slot: &mut f64,
        var_qsch1_dn19_slot: &mut f64,
        var_qsch1_dn2_slot: &mut f64,
        var_qsch1_dn20_slot: &mut f64,
        var_qsch1_dn21_slot: &mut f64,
        var_qsch1_dn22_slot: &mut f64,
        var_qsch1_dn23_slot: &mut f64,
        var_qsch1_dn24_slot: &mut f64,
        var_qsch1_dn25_slot: &mut f64,
        var_qsch1_dn26_slot: &mut f64,
        var_qsch1_dn27_slot: &mut f64,
        var_qsch1_dn28_slot: &mut f64,
        var_qsch1_dn29_slot: &mut f64,
        var_qsch1_dn3_slot: &mut f64,
        var_qsch1_dn4_slot: &mut f64,
        var_qsch1_dn5_slot: &mut f64,
        var_qsch1_dn6_slot: &mut f64,
        var_qsch1_dn7_slot: &mut f64,
        var_qsch1_dn8_slot: &mut f64,
        var_qsch1_dn9_slot: &mut f64,
        var_qsch1c_slot: &mut f64,
        var_qsch2_slot: &mut f64,
        var_qsch2_db0_slot: &mut f64,
        var_qsch2_db1_slot: &mut f64,
        var_qsch2_db10_slot: &mut f64,
        var_qsch2_db11_slot: &mut f64,
        var_qsch2_db12_slot: &mut f64,
        var_qsch2_db13_slot: &mut f64,
        var_qsch2_db14_slot: &mut f64,
        var_qsch2_db15_slot: &mut f64,
        var_qsch2_db16_slot: &mut f64,
        var_qsch2_db17_slot: &mut f64,
        var_qsch2_db18_slot: &mut f64,
        var_qsch2_db19_slot: &mut f64,
        var_qsch2_db2_slot: &mut f64,
        var_qsch2_db20_slot: &mut f64,
        var_qsch2_db21_slot: &mut f64,
        var_qsch2_db22_slot: &mut f64,
        var_qsch2_db23_slot: &mut f64,
        var_qsch2_db24_slot: &mut f64,
        var_qsch2_db25_slot: &mut f64,
        var_qsch2_db26_slot: &mut f64,
        var_qsch2_db27_slot: &mut f64,
        var_qsch2_db28_slot: &mut f64,
        var_qsch2_db29_slot: &mut f64,
        var_qsch2_db3_slot: &mut f64,
        var_qsch2_db30_slot: &mut f64,
        var_qsch2_db31_slot: &mut f64,
        var_qsch2_db32_slot: &mut f64,
        var_qsch2_db33_slot: &mut f64,
        var_qsch2_db34_slot: &mut f64,
        var_qsch2_db35_slot: &mut f64,
        var_qsch2_db4_slot: &mut f64,
        var_qsch2_db5_slot: &mut f64,
        var_qsch2_db6_slot: &mut f64,
        var_qsch2_db7_slot: &mut f64,
        var_qsch2_db8_slot: &mut f64,
        var_qsch2_db9_slot: &mut f64,
        var_qsch2_dn0_slot: &mut f64,
        var_qsch2_dn1_slot: &mut f64,
        var_qsch2_dn10_slot: &mut f64,
        var_qsch2_dn11_slot: &mut f64,
        var_qsch2_dn12_slot: &mut f64,
        var_qsch2_dn13_slot: &mut f64,
        var_qsch2_dn14_slot: &mut f64,
        var_qsch2_dn15_slot: &mut f64,
        var_qsch2_dn16_slot: &mut f64,
        var_qsch2_dn17_slot: &mut f64,
        var_qsch2_dn18_slot: &mut f64,
        var_qsch2_dn19_slot: &mut f64,
        var_qsch2_dn2_slot: &mut f64,
        var_qsch2_dn20_slot: &mut f64,
        var_qsch2_dn21_slot: &mut f64,
        var_qsch2_dn22_slot: &mut f64,
        var_qsch2_dn23_slot: &mut f64,
        var_qsch2_dn24_slot: &mut f64,
        var_qsch2_dn25_slot: &mut f64,
        var_qsch2_dn26_slot: &mut f64,
        var_qsch2_dn27_slot: &mut f64,
        var_qsch2_dn28_slot: &mut f64,
        var_qsch2_dn29_slot: &mut f64,
        var_qsch2_dn3_slot: &mut f64,
        var_qsch2_dn4_slot: &mut f64,
        var_qsch2_dn5_slot: &mut f64,
        var_qsch2_dn6_slot: &mut f64,
        var_qsch2_dn7_slot: &mut f64,
        var_qsch2_dn8_slot: &mut f64,
        var_qsch2_dn9_slot: &mut f64,
        var_qsch2c_slot: &mut f64,
        var_qsch3c_slot: &mut f64,
        var_qsch_db0_slot: &mut f64,
        var_qsch_db1_slot: &mut f64,
        var_qsch_db10_slot: &mut f64,
        var_qsch_db11_slot: &mut f64,
        var_qsch_db12_slot: &mut f64,
        var_qsch_db13_slot: &mut f64,
        var_qsch_db14_slot: &mut f64,
        var_qsch_db15_slot: &mut f64,
        var_qsch_db16_slot: &mut f64,
        var_qsch_db17_slot: &mut f64,
        var_qsch_db18_slot: &mut f64,
        var_qsch_db19_slot: &mut f64,
        var_qsch_db2_slot: &mut f64,
        var_qsch_db20_slot: &mut f64,
        var_qsch_db21_slot: &mut f64,
        var_qsch_db22_slot: &mut f64,
        var_qsch_db23_slot: &mut f64,
        var_qsch_db24_slot: &mut f64,
        var_qsch_db25_slot: &mut f64,
        var_qsch_db26_slot: &mut f64,
        var_qsch_db27_slot: &mut f64,
        var_qsch_db28_slot: &mut f64,
        var_qsch_db29_slot: &mut f64,
        var_qsch_db3_slot: &mut f64,
        var_qsch_db30_slot: &mut f64,
        var_qsch_db31_slot: &mut f64,
        var_qsch_db32_slot: &mut f64,
        var_qsch_db33_slot: &mut f64,
        var_qsch_db34_slot: &mut f64,
        var_qsch_db35_slot: &mut f64,
        var_qsch_db4_slot: &mut f64,
        var_qsch_db5_slot: &mut f64,
        var_qsch_db6_slot: &mut f64,
        var_qsch_db7_slot: &mut f64,
        var_qsch_db8_slot: &mut f64,
        var_qsch_db9_slot: &mut f64,
        var_qsch_dn0_slot: &mut f64,
        var_qsch_dn1_slot: &mut f64,
        var_qsch_dn10_slot: &mut f64,
        var_qsch_dn11_slot: &mut f64,
        var_qsch_dn12_slot: &mut f64,
        var_qsch_dn13_slot: &mut f64,
        var_qsch_dn14_slot: &mut f64,
        var_qsch_dn15_slot: &mut f64,
        var_qsch_dn16_slot: &mut f64,
        var_qsch_dn17_slot: &mut f64,
        var_qsch_dn18_slot: &mut f64,
        var_qsch_dn19_slot: &mut f64,
        var_qsch_dn2_slot: &mut f64,
        var_qsch_dn20_slot: &mut f64,
        var_qsch_dn21_slot: &mut f64,
        var_qsch_dn22_slot: &mut f64,
        var_qsch_dn23_slot: &mut f64,
        var_qsch_dn24_slot: &mut f64,
        var_qsch_dn25_slot: &mut f64,
        var_qsch_dn26_slot: &mut f64,
        var_qsch_dn27_slot: &mut f64,
        var_qsch_dn28_slot: &mut f64,
        var_qsch_dn29_slot: &mut f64,
        var_qsch_dn3_slot: &mut f64,
        var_qsch_dn4_slot: &mut f64,
        var_qsch_dn5_slot: &mut f64,
        var_qsch_dn6_slot: &mut f64,
        var_qsch_dn7_slot: &mut f64,
        var_qsch_dn8_slot: &mut f64,
        var_qsch_dn9_slot: &mut f64,
        var_vschfc1_slot: &mut f64,
        var_vschfc1_db0_slot: &mut f64,
        var_vschfc1_db1_slot: &mut f64,
        var_vschfc1_db10_slot: &mut f64,
        var_vschfc1_db11_slot: &mut f64,
        var_vschfc1_db12_slot: &mut f64,
        var_vschfc1_db13_slot: &mut f64,
        var_vschfc1_db14_slot: &mut f64,
        var_vschfc1_db15_slot: &mut f64,
        var_vschfc1_db16_slot: &mut f64,
        var_vschfc1_db17_slot: &mut f64,
        var_vschfc1_db18_slot: &mut f64,
        var_vschfc1_db19_slot: &mut f64,
        var_vschfc1_db2_slot: &mut f64,
        var_vschfc1_db20_slot: &mut f64,
        var_vschfc1_db21_slot: &mut f64,
        var_vschfc1_db22_slot: &mut f64,
        var_vschfc1_db23_slot: &mut f64,
        var_vschfc1_db24_slot: &mut f64,
        var_vschfc1_db25_slot: &mut f64,
        var_vschfc1_db26_slot: &mut f64,
        var_vschfc1_db27_slot: &mut f64,
        var_vschfc1_db28_slot: &mut f64,
        var_vschfc1_db29_slot: &mut f64,
        var_vschfc1_db3_slot: &mut f64,
        var_vschfc1_db30_slot: &mut f64,
        var_vschfc1_db31_slot: &mut f64,
        var_vschfc1_db32_slot: &mut f64,
        var_vschfc1_db33_slot: &mut f64,
        var_vschfc1_db34_slot: &mut f64,
        var_vschfc1_db35_slot: &mut f64,
        var_vschfc1_db4_slot: &mut f64,
        var_vschfc1_db5_slot: &mut f64,
        var_vschfc1_db6_slot: &mut f64,
        var_vschfc1_db7_slot: &mut f64,
        var_vschfc1_db8_slot: &mut f64,
        var_vschfc1_db9_slot: &mut f64,
        var_vschfc1_dn0_slot: &mut f64,
        var_vschfc1_dn1_slot: &mut f64,
        var_vschfc1_dn10_slot: &mut f64,
        var_vschfc1_dn11_slot: &mut f64,
        var_vschfc1_dn12_slot: &mut f64,
        var_vschfc1_dn13_slot: &mut f64,
        var_vschfc1_dn14_slot: &mut f64,
        var_vschfc1_dn15_slot: &mut f64,
        var_vschfc1_dn16_slot: &mut f64,
        var_vschfc1_dn17_slot: &mut f64,
        var_vschfc1_dn18_slot: &mut f64,
        var_vschfc1_dn19_slot: &mut f64,
        var_vschfc1_dn2_slot: &mut f64,
        var_vschfc1_dn20_slot: &mut f64,
        var_vschfc1_dn21_slot: &mut f64,
        var_vschfc1_dn22_slot: &mut f64,
        var_vschfc1_dn23_slot: &mut f64,
        var_vschfc1_dn24_slot: &mut f64,
        var_vschfc1_dn25_slot: &mut f64,
        var_vschfc1_dn26_slot: &mut f64,
        var_vschfc1_dn27_slot: &mut f64,
        var_vschfc1_dn28_slot: &mut f64,
        var_vschfc1_dn29_slot: &mut f64,
        var_vschfc1_dn3_slot: &mut f64,
        var_vschfc1_dn4_slot: &mut f64,
        var_vschfc1_dn5_slot: &mut f64,
        var_vschfc1_dn6_slot: &mut f64,
        var_vschfc1_dn7_slot: &mut f64,
        var_vschfc1_dn8_slot: &mut f64,
        var_vschfc1_dn9_slot: &mut f64,
        var_vschfc2_slot: &mut f64,
        var_vschfc2_db0_slot: &mut f64,
        var_vschfc2_db1_slot: &mut f64,
        var_vschfc2_db10_slot: &mut f64,
        var_vschfc2_db11_slot: &mut f64,
        var_vschfc2_db12_slot: &mut f64,
        var_vschfc2_db13_slot: &mut f64,
        var_vschfc2_db14_slot: &mut f64,
        var_vschfc2_db15_slot: &mut f64,
        var_vschfc2_db16_slot: &mut f64,
        var_vschfc2_db17_slot: &mut f64,
        var_vschfc2_db18_slot: &mut f64,
        var_vschfc2_db19_slot: &mut f64,
        var_vschfc2_db2_slot: &mut f64,
        var_vschfc2_db20_slot: &mut f64,
        var_vschfc2_db21_slot: &mut f64,
        var_vschfc2_db22_slot: &mut f64,
        var_vschfc2_db23_slot: &mut f64,
        var_vschfc2_db24_slot: &mut f64,
        var_vschfc2_db25_slot: &mut f64,
        var_vschfc2_db26_slot: &mut f64,
        var_vschfc2_db27_slot: &mut f64,
        var_vschfc2_db28_slot: &mut f64,
        var_vschfc2_db29_slot: &mut f64,
        var_vschfc2_db3_slot: &mut f64,
        var_vschfc2_db30_slot: &mut f64,
        var_vschfc2_db31_slot: &mut f64,
        var_vschfc2_db32_slot: &mut f64,
        var_vschfc2_db33_slot: &mut f64,
        var_vschfc2_db34_slot: &mut f64,
        var_vschfc2_db35_slot: &mut f64,
        var_vschfc2_db4_slot: &mut f64,
        var_vschfc2_db5_slot: &mut f64,
        var_vschfc2_db6_slot: &mut f64,
        var_vschfc2_db7_slot: &mut f64,
        var_vschfc2_db8_slot: &mut f64,
        var_vschfc2_db9_slot: &mut f64,
        var_vschfc2_dn0_slot: &mut f64,
        var_vschfc2_dn1_slot: &mut f64,
        var_vschfc2_dn10_slot: &mut f64,
        var_vschfc2_dn11_slot: &mut f64,
        var_vschfc2_dn12_slot: &mut f64,
        var_vschfc2_dn13_slot: &mut f64,
        var_vschfc2_dn14_slot: &mut f64,
        var_vschfc2_dn15_slot: &mut f64,
        var_vschfc2_dn16_slot: &mut f64,
        var_vschfc2_dn17_slot: &mut f64,
        var_vschfc2_dn18_slot: &mut f64,
        var_vschfc2_dn19_slot: &mut f64,
        var_vschfc2_dn2_slot: &mut f64,
        var_vschfc2_dn20_slot: &mut f64,
        var_vschfc2_dn21_slot: &mut f64,
        var_vschfc2_dn22_slot: &mut f64,
        var_vschfc2_dn23_slot: &mut f64,
        var_vschfc2_dn24_slot: &mut f64,
        var_vschfc2_dn25_slot: &mut f64,
        var_vschfc2_dn26_slot: &mut f64,
        var_vschfc2_dn27_slot: &mut f64,
        var_vschfc2_dn28_slot: &mut f64,
        var_vschfc2_dn29_slot: &mut f64,
        var_vschfc2_dn3_slot: &mut f64,
        var_vschfc2_dn4_slot: &mut f64,
        var_vschfc2_dn5_slot: &mut f64,
        var_vschfc2_dn6_slot: &mut f64,
        var_vschfc2_dn7_slot: &mut f64,
        var_vschfc2_dn8_slot: &mut f64,
        var_vschfc2_dn9_slot: &mut f64,
    ) {
        let mut var_guard474: f64 = *var_guard474_slot;
        let mut var_guard475: f64 = *var_guard475_slot;
        let mut var_guard476: f64 = *var_guard476_slot;
        let mut var_qsch: f64 = *var_qsch_slot;
        let mut var_qsch0: f64 = *var_qsch0_slot;
        let mut var_qsch1: f64 = *var_qsch1_slot;
        let mut var_qsch1_db0: f64 = *var_qsch1_db0_slot;
        let mut var_qsch1_db1: f64 = *var_qsch1_db1_slot;
        let mut var_qsch1_db10: f64 = *var_qsch1_db10_slot;
        let mut var_qsch1_db11: f64 = *var_qsch1_db11_slot;
        let mut var_qsch1_db12: f64 = *var_qsch1_db12_slot;
        let mut var_qsch1_db13: f64 = *var_qsch1_db13_slot;
        let mut var_qsch1_db14: f64 = *var_qsch1_db14_slot;
        let mut var_qsch1_db15: f64 = *var_qsch1_db15_slot;
        let mut var_qsch1_db16: f64 = *var_qsch1_db16_slot;
        let mut var_qsch1_db17: f64 = *var_qsch1_db17_slot;
        let mut var_qsch1_db18: f64 = *var_qsch1_db18_slot;
        let mut var_qsch1_db19: f64 = *var_qsch1_db19_slot;
        let mut var_qsch1_db2: f64 = *var_qsch1_db2_slot;
        let mut var_qsch1_db20: f64 = *var_qsch1_db20_slot;
        let mut var_qsch1_db21: f64 = *var_qsch1_db21_slot;
        let mut var_qsch1_db22: f64 = *var_qsch1_db22_slot;
        let mut var_qsch1_db23: f64 = *var_qsch1_db23_slot;
        let mut var_qsch1_db24: f64 = *var_qsch1_db24_slot;
        let mut var_qsch1_db25: f64 = *var_qsch1_db25_slot;
        let mut var_qsch1_db26: f64 = *var_qsch1_db26_slot;
        let mut var_qsch1_db27: f64 = *var_qsch1_db27_slot;
        let mut var_qsch1_db28: f64 = *var_qsch1_db28_slot;
        let mut var_qsch1_db29: f64 = *var_qsch1_db29_slot;
        let mut var_qsch1_db3: f64 = *var_qsch1_db3_slot;
        let mut var_qsch1_db30: f64 = *var_qsch1_db30_slot;
        let mut var_qsch1_db31: f64 = *var_qsch1_db31_slot;
        let mut var_qsch1_db32: f64 = *var_qsch1_db32_slot;
        let mut var_qsch1_db33: f64 = *var_qsch1_db33_slot;
        let mut var_qsch1_db34: f64 = *var_qsch1_db34_slot;
        let mut var_qsch1_db35: f64 = *var_qsch1_db35_slot;
        let mut var_qsch1_db4: f64 = *var_qsch1_db4_slot;
        let mut var_qsch1_db5: f64 = *var_qsch1_db5_slot;
        let mut var_qsch1_db6: f64 = *var_qsch1_db6_slot;
        let mut var_qsch1_db7: f64 = *var_qsch1_db7_slot;
        let mut var_qsch1_db8: f64 = *var_qsch1_db8_slot;
        let mut var_qsch1_db9: f64 = *var_qsch1_db9_slot;
        let mut var_qsch1_dn0: f64 = *var_qsch1_dn0_slot;
        let mut var_qsch1_dn1: f64 = *var_qsch1_dn1_slot;
        let mut var_qsch1_dn10: f64 = *var_qsch1_dn10_slot;
        let mut var_qsch1_dn11: f64 = *var_qsch1_dn11_slot;
        let mut var_qsch1_dn12: f64 = *var_qsch1_dn12_slot;
        let mut var_qsch1_dn13: f64 = *var_qsch1_dn13_slot;
        let mut var_qsch1_dn14: f64 = *var_qsch1_dn14_slot;
        let mut var_qsch1_dn15: f64 = *var_qsch1_dn15_slot;
        let mut var_qsch1_dn16: f64 = *var_qsch1_dn16_slot;
        let mut var_qsch1_dn17: f64 = *var_qsch1_dn17_slot;
        let mut var_qsch1_dn18: f64 = *var_qsch1_dn18_slot;
        let mut var_qsch1_dn19: f64 = *var_qsch1_dn19_slot;
        let mut var_qsch1_dn2: f64 = *var_qsch1_dn2_slot;
        let mut var_qsch1_dn20: f64 = *var_qsch1_dn20_slot;
        let mut var_qsch1_dn21: f64 = *var_qsch1_dn21_slot;
        let mut var_qsch1_dn22: f64 = *var_qsch1_dn22_slot;
        let mut var_qsch1_dn23: f64 = *var_qsch1_dn23_slot;
        let mut var_qsch1_dn24: f64 = *var_qsch1_dn24_slot;
        let mut var_qsch1_dn25: f64 = *var_qsch1_dn25_slot;
        let mut var_qsch1_dn26: f64 = *var_qsch1_dn26_slot;
        let mut var_qsch1_dn27: f64 = *var_qsch1_dn27_slot;
        let mut var_qsch1_dn28: f64 = *var_qsch1_dn28_slot;
        let mut var_qsch1_dn29: f64 = *var_qsch1_dn29_slot;
        let mut var_qsch1_dn3: f64 = *var_qsch1_dn3_slot;
        let mut var_qsch1_dn4: f64 = *var_qsch1_dn4_slot;
        let mut var_qsch1_dn5: f64 = *var_qsch1_dn5_slot;
        let mut var_qsch1_dn6: f64 = *var_qsch1_dn6_slot;
        let mut var_qsch1_dn7: f64 = *var_qsch1_dn7_slot;
        let mut var_qsch1_dn8: f64 = *var_qsch1_dn8_slot;
        let mut var_qsch1_dn9: f64 = *var_qsch1_dn9_slot;
        let mut var_qsch1c: f64 = *var_qsch1c_slot;
        let mut var_qsch2: f64 = *var_qsch2_slot;
        let mut var_qsch2_db0: f64 = *var_qsch2_db0_slot;
        let mut var_qsch2_db1: f64 = *var_qsch2_db1_slot;
        let mut var_qsch2_db10: f64 = *var_qsch2_db10_slot;
        let mut var_qsch2_db11: f64 = *var_qsch2_db11_slot;
        let mut var_qsch2_db12: f64 = *var_qsch2_db12_slot;
        let mut var_qsch2_db13: f64 = *var_qsch2_db13_slot;
        let mut var_qsch2_db14: f64 = *var_qsch2_db14_slot;
        let mut var_qsch2_db15: f64 = *var_qsch2_db15_slot;
        let mut var_qsch2_db16: f64 = *var_qsch2_db16_slot;
        let mut var_qsch2_db17: f64 = *var_qsch2_db17_slot;
        let mut var_qsch2_db18: f64 = *var_qsch2_db18_slot;
        let mut var_qsch2_db19: f64 = *var_qsch2_db19_slot;
        let mut var_qsch2_db2: f64 = *var_qsch2_db2_slot;
        let mut var_qsch2_db20: f64 = *var_qsch2_db20_slot;
        let mut var_qsch2_db21: f64 = *var_qsch2_db21_slot;
        let mut var_qsch2_db22: f64 = *var_qsch2_db22_slot;
        let mut var_qsch2_db23: f64 = *var_qsch2_db23_slot;
        let mut var_qsch2_db24: f64 = *var_qsch2_db24_slot;
        let mut var_qsch2_db25: f64 = *var_qsch2_db25_slot;
        let mut var_qsch2_db26: f64 = *var_qsch2_db26_slot;
        let mut var_qsch2_db27: f64 = *var_qsch2_db27_slot;
        let mut var_qsch2_db28: f64 = *var_qsch2_db28_slot;
        let mut var_qsch2_db29: f64 = *var_qsch2_db29_slot;
        let mut var_qsch2_db3: f64 = *var_qsch2_db3_slot;
        let mut var_qsch2_db30: f64 = *var_qsch2_db30_slot;
        let mut var_qsch2_db31: f64 = *var_qsch2_db31_slot;
        let mut var_qsch2_db32: f64 = *var_qsch2_db32_slot;
        let mut var_qsch2_db33: f64 = *var_qsch2_db33_slot;
        let mut var_qsch2_db34: f64 = *var_qsch2_db34_slot;
        let mut var_qsch2_db35: f64 = *var_qsch2_db35_slot;
        let mut var_qsch2_db4: f64 = *var_qsch2_db4_slot;
        let mut var_qsch2_db5: f64 = *var_qsch2_db5_slot;
        let mut var_qsch2_db6: f64 = *var_qsch2_db6_slot;
        let mut var_qsch2_db7: f64 = *var_qsch2_db7_slot;
        let mut var_qsch2_db8: f64 = *var_qsch2_db8_slot;
        let mut var_qsch2_db9: f64 = *var_qsch2_db9_slot;
        let mut var_qsch2_dn0: f64 = *var_qsch2_dn0_slot;
        let mut var_qsch2_dn1: f64 = *var_qsch2_dn1_slot;
        let mut var_qsch2_dn10: f64 = *var_qsch2_dn10_slot;
        let mut var_qsch2_dn11: f64 = *var_qsch2_dn11_slot;
        let mut var_qsch2_dn12: f64 = *var_qsch2_dn12_slot;
        let mut var_qsch2_dn13: f64 = *var_qsch2_dn13_slot;
        let mut var_qsch2_dn14: f64 = *var_qsch2_dn14_slot;
        let mut var_qsch2_dn15: f64 = *var_qsch2_dn15_slot;
        let mut var_qsch2_dn16: f64 = *var_qsch2_dn16_slot;
        let mut var_qsch2_dn17: f64 = *var_qsch2_dn17_slot;
        let mut var_qsch2_dn18: f64 = *var_qsch2_dn18_slot;
        let mut var_qsch2_dn19: f64 = *var_qsch2_dn19_slot;
        let mut var_qsch2_dn2: f64 = *var_qsch2_dn2_slot;
        let mut var_qsch2_dn20: f64 = *var_qsch2_dn20_slot;
        let mut var_qsch2_dn21: f64 = *var_qsch2_dn21_slot;
        let mut var_qsch2_dn22: f64 = *var_qsch2_dn22_slot;
        let mut var_qsch2_dn23: f64 = *var_qsch2_dn23_slot;
        let mut var_qsch2_dn24: f64 = *var_qsch2_dn24_slot;
        let mut var_qsch2_dn25: f64 = *var_qsch2_dn25_slot;
        let mut var_qsch2_dn26: f64 = *var_qsch2_dn26_slot;
        let mut var_qsch2_dn27: f64 = *var_qsch2_dn27_slot;
        let mut var_qsch2_dn28: f64 = *var_qsch2_dn28_slot;
        let mut var_qsch2_dn29: f64 = *var_qsch2_dn29_slot;
        let mut var_qsch2_dn3: f64 = *var_qsch2_dn3_slot;
        let mut var_qsch2_dn4: f64 = *var_qsch2_dn4_slot;
        let mut var_qsch2_dn5: f64 = *var_qsch2_dn5_slot;
        let mut var_qsch2_dn6: f64 = *var_qsch2_dn6_slot;
        let mut var_qsch2_dn7: f64 = *var_qsch2_dn7_slot;
        let mut var_qsch2_dn8: f64 = *var_qsch2_dn8_slot;
        let mut var_qsch2_dn9: f64 = *var_qsch2_dn9_slot;
        let mut var_qsch2c: f64 = *var_qsch2c_slot;
        let mut var_qsch3c: f64 = *var_qsch3c_slot;
        let mut var_qsch_db0: f64 = *var_qsch_db0_slot;
        let mut var_qsch_db1: f64 = *var_qsch_db1_slot;
        let mut var_qsch_db10: f64 = *var_qsch_db10_slot;
        let mut var_qsch_db11: f64 = *var_qsch_db11_slot;
        let mut var_qsch_db12: f64 = *var_qsch_db12_slot;
        let mut var_qsch_db13: f64 = *var_qsch_db13_slot;
        let mut var_qsch_db14: f64 = *var_qsch_db14_slot;
        let mut var_qsch_db15: f64 = *var_qsch_db15_slot;
        let mut var_qsch_db16: f64 = *var_qsch_db16_slot;
        let mut var_qsch_db17: f64 = *var_qsch_db17_slot;
        let mut var_qsch_db18: f64 = *var_qsch_db18_slot;
        let mut var_qsch_db19: f64 = *var_qsch_db19_slot;
        let mut var_qsch_db2: f64 = *var_qsch_db2_slot;
        let mut var_qsch_db20: f64 = *var_qsch_db20_slot;
        let mut var_qsch_db21: f64 = *var_qsch_db21_slot;
        let mut var_qsch_db22: f64 = *var_qsch_db22_slot;
        let mut var_qsch_db23: f64 = *var_qsch_db23_slot;
        let mut var_qsch_db24: f64 = *var_qsch_db24_slot;
        let mut var_qsch_db25: f64 = *var_qsch_db25_slot;
        let mut var_qsch_db26: f64 = *var_qsch_db26_slot;
        let mut var_qsch_db27: f64 = *var_qsch_db27_slot;
        let mut var_qsch_db28: f64 = *var_qsch_db28_slot;
        let mut var_qsch_db29: f64 = *var_qsch_db29_slot;
        let mut var_qsch_db3: f64 = *var_qsch_db3_slot;
        let mut var_qsch_db30: f64 = *var_qsch_db30_slot;
        let mut var_qsch_db31: f64 = *var_qsch_db31_slot;
        let mut var_qsch_db32: f64 = *var_qsch_db32_slot;
        let mut var_qsch_db33: f64 = *var_qsch_db33_slot;
        let mut var_qsch_db34: f64 = *var_qsch_db34_slot;
        let mut var_qsch_db35: f64 = *var_qsch_db35_slot;
        let mut var_qsch_db4: f64 = *var_qsch_db4_slot;
        let mut var_qsch_db5: f64 = *var_qsch_db5_slot;
        let mut var_qsch_db6: f64 = *var_qsch_db6_slot;
        let mut var_qsch_db7: f64 = *var_qsch_db7_slot;
        let mut var_qsch_db8: f64 = *var_qsch_db8_slot;
        let mut var_qsch_db9: f64 = *var_qsch_db9_slot;
        let mut var_qsch_dn0: f64 = *var_qsch_dn0_slot;
        let mut var_qsch_dn1: f64 = *var_qsch_dn1_slot;
        let mut var_qsch_dn10: f64 = *var_qsch_dn10_slot;
        let mut var_qsch_dn11: f64 = *var_qsch_dn11_slot;
        let mut var_qsch_dn12: f64 = *var_qsch_dn12_slot;
        let mut var_qsch_dn13: f64 = *var_qsch_dn13_slot;
        let mut var_qsch_dn14: f64 = *var_qsch_dn14_slot;
        let mut var_qsch_dn15: f64 = *var_qsch_dn15_slot;
        let mut var_qsch_dn16: f64 = *var_qsch_dn16_slot;
        let mut var_qsch_dn17: f64 = *var_qsch_dn17_slot;
        let mut var_qsch_dn18: f64 = *var_qsch_dn18_slot;
        let mut var_qsch_dn19: f64 = *var_qsch_dn19_slot;
        let mut var_qsch_dn2: f64 = *var_qsch_dn2_slot;
        let mut var_qsch_dn20: f64 = *var_qsch_dn20_slot;
        let mut var_qsch_dn21: f64 = *var_qsch_dn21_slot;
        let mut var_qsch_dn22: f64 = *var_qsch_dn22_slot;
        let mut var_qsch_dn23: f64 = *var_qsch_dn23_slot;
        let mut var_qsch_dn24: f64 = *var_qsch_dn24_slot;
        let mut var_qsch_dn25: f64 = *var_qsch_dn25_slot;
        let mut var_qsch_dn26: f64 = *var_qsch_dn26_slot;
        let mut var_qsch_dn27: f64 = *var_qsch_dn27_slot;
        let mut var_qsch_dn28: f64 = *var_qsch_dn28_slot;
        let mut var_qsch_dn29: f64 = *var_qsch_dn29_slot;
        let mut var_qsch_dn3: f64 = *var_qsch_dn3_slot;
        let mut var_qsch_dn4: f64 = *var_qsch_dn4_slot;
        let mut var_qsch_dn5: f64 = *var_qsch_dn5_slot;
        let mut var_qsch_dn6: f64 = *var_qsch_dn6_slot;
        let mut var_qsch_dn7: f64 = *var_qsch_dn7_slot;
        let mut var_qsch_dn8: f64 = *var_qsch_dn8_slot;
        let mut var_qsch_dn9: f64 = *var_qsch_dn9_slot;
        let mut var_vschfc1: f64 = *var_vschfc1_slot;
        let mut var_vschfc1_db0: f64 = *var_vschfc1_db0_slot;
        let mut var_vschfc1_db1: f64 = *var_vschfc1_db1_slot;
        let mut var_vschfc1_db10: f64 = *var_vschfc1_db10_slot;
        let mut var_vschfc1_db11: f64 = *var_vschfc1_db11_slot;
        let mut var_vschfc1_db12: f64 = *var_vschfc1_db12_slot;
        let mut var_vschfc1_db13: f64 = *var_vschfc1_db13_slot;
        let mut var_vschfc1_db14: f64 = *var_vschfc1_db14_slot;
        let mut var_vschfc1_db15: f64 = *var_vschfc1_db15_slot;
        let mut var_vschfc1_db16: f64 = *var_vschfc1_db16_slot;
        let mut var_vschfc1_db17: f64 = *var_vschfc1_db17_slot;
        let mut var_vschfc1_db18: f64 = *var_vschfc1_db18_slot;
        let mut var_vschfc1_db19: f64 = *var_vschfc1_db19_slot;
        let mut var_vschfc1_db2: f64 = *var_vschfc1_db2_slot;
        let mut var_vschfc1_db20: f64 = *var_vschfc1_db20_slot;
        let mut var_vschfc1_db21: f64 = *var_vschfc1_db21_slot;
        let mut var_vschfc1_db22: f64 = *var_vschfc1_db22_slot;
        let mut var_vschfc1_db23: f64 = *var_vschfc1_db23_slot;
        let mut var_vschfc1_db24: f64 = *var_vschfc1_db24_slot;
        let mut var_vschfc1_db25: f64 = *var_vschfc1_db25_slot;
        let mut var_vschfc1_db26: f64 = *var_vschfc1_db26_slot;
        let mut var_vschfc1_db27: f64 = *var_vschfc1_db27_slot;
        let mut var_vschfc1_db28: f64 = *var_vschfc1_db28_slot;
        let mut var_vschfc1_db29: f64 = *var_vschfc1_db29_slot;
        let mut var_vschfc1_db3: f64 = *var_vschfc1_db3_slot;
        let mut var_vschfc1_db30: f64 = *var_vschfc1_db30_slot;
        let mut var_vschfc1_db31: f64 = *var_vschfc1_db31_slot;
        let mut var_vschfc1_db32: f64 = *var_vschfc1_db32_slot;
        let mut var_vschfc1_db33: f64 = *var_vschfc1_db33_slot;
        let mut var_vschfc1_db34: f64 = *var_vschfc1_db34_slot;
        let mut var_vschfc1_db35: f64 = *var_vschfc1_db35_slot;
        let mut var_vschfc1_db4: f64 = *var_vschfc1_db4_slot;
        let mut var_vschfc1_db5: f64 = *var_vschfc1_db5_slot;
        let mut var_vschfc1_db6: f64 = *var_vschfc1_db6_slot;
        let mut var_vschfc1_db7: f64 = *var_vschfc1_db7_slot;
        let mut var_vschfc1_db8: f64 = *var_vschfc1_db8_slot;
        let mut var_vschfc1_db9: f64 = *var_vschfc1_db9_slot;
        let mut var_vschfc1_dn0: f64 = *var_vschfc1_dn0_slot;
        let mut var_vschfc1_dn1: f64 = *var_vschfc1_dn1_slot;
        let mut var_vschfc1_dn10: f64 = *var_vschfc1_dn10_slot;
        let mut var_vschfc1_dn11: f64 = *var_vschfc1_dn11_slot;
        let mut var_vschfc1_dn12: f64 = *var_vschfc1_dn12_slot;
        let mut var_vschfc1_dn13: f64 = *var_vschfc1_dn13_slot;
        let mut var_vschfc1_dn14: f64 = *var_vschfc1_dn14_slot;
        let mut var_vschfc1_dn15: f64 = *var_vschfc1_dn15_slot;
        let mut var_vschfc1_dn16: f64 = *var_vschfc1_dn16_slot;
        let mut var_vschfc1_dn17: f64 = *var_vschfc1_dn17_slot;
        let mut var_vschfc1_dn18: f64 = *var_vschfc1_dn18_slot;
        let mut var_vschfc1_dn19: f64 = *var_vschfc1_dn19_slot;
        let mut var_vschfc1_dn2: f64 = *var_vschfc1_dn2_slot;
        let mut var_vschfc1_dn20: f64 = *var_vschfc1_dn20_slot;
        let mut var_vschfc1_dn21: f64 = *var_vschfc1_dn21_slot;
        let mut var_vschfc1_dn22: f64 = *var_vschfc1_dn22_slot;
        let mut var_vschfc1_dn23: f64 = *var_vschfc1_dn23_slot;
        let mut var_vschfc1_dn24: f64 = *var_vschfc1_dn24_slot;
        let mut var_vschfc1_dn25: f64 = *var_vschfc1_dn25_slot;
        let mut var_vschfc1_dn26: f64 = *var_vschfc1_dn26_slot;
        let mut var_vschfc1_dn27: f64 = *var_vschfc1_dn27_slot;
        let mut var_vschfc1_dn28: f64 = *var_vschfc1_dn28_slot;
        let mut var_vschfc1_dn29: f64 = *var_vschfc1_dn29_slot;
        let mut var_vschfc1_dn3: f64 = *var_vschfc1_dn3_slot;
        let mut var_vschfc1_dn4: f64 = *var_vschfc1_dn4_slot;
        let mut var_vschfc1_dn5: f64 = *var_vschfc1_dn5_slot;
        let mut var_vschfc1_dn6: f64 = *var_vschfc1_dn6_slot;
        let mut var_vschfc1_dn7: f64 = *var_vschfc1_dn7_slot;
        let mut var_vschfc1_dn8: f64 = *var_vschfc1_dn8_slot;
        let mut var_vschfc1_dn9: f64 = *var_vschfc1_dn9_slot;
        let mut var_vschfc2: f64 = *var_vschfc2_slot;
        let mut var_vschfc2_db0: f64 = *var_vschfc2_db0_slot;
        let mut var_vschfc2_db1: f64 = *var_vschfc2_db1_slot;
        let mut var_vschfc2_db10: f64 = *var_vschfc2_db10_slot;
        let mut var_vschfc2_db11: f64 = *var_vschfc2_db11_slot;
        let mut var_vschfc2_db12: f64 = *var_vschfc2_db12_slot;
        let mut var_vschfc2_db13: f64 = *var_vschfc2_db13_slot;
        let mut var_vschfc2_db14: f64 = *var_vschfc2_db14_slot;
        let mut var_vschfc2_db15: f64 = *var_vschfc2_db15_slot;
        let mut var_vschfc2_db16: f64 = *var_vschfc2_db16_slot;
        let mut var_vschfc2_db17: f64 = *var_vschfc2_db17_slot;
        let mut var_vschfc2_db18: f64 = *var_vschfc2_db18_slot;
        let mut var_vschfc2_db19: f64 = *var_vschfc2_db19_slot;
        let mut var_vschfc2_db2: f64 = *var_vschfc2_db2_slot;
        let mut var_vschfc2_db20: f64 = *var_vschfc2_db20_slot;
        let mut var_vschfc2_db21: f64 = *var_vschfc2_db21_slot;
        let mut var_vschfc2_db22: f64 = *var_vschfc2_db22_slot;
        let mut var_vschfc2_db23: f64 = *var_vschfc2_db23_slot;
        let mut var_vschfc2_db24: f64 = *var_vschfc2_db24_slot;
        let mut var_vschfc2_db25: f64 = *var_vschfc2_db25_slot;
        let mut var_vschfc2_db26: f64 = *var_vschfc2_db26_slot;
        let mut var_vschfc2_db27: f64 = *var_vschfc2_db27_slot;
        let mut var_vschfc2_db28: f64 = *var_vschfc2_db28_slot;
        let mut var_vschfc2_db29: f64 = *var_vschfc2_db29_slot;
        let mut var_vschfc2_db3: f64 = *var_vschfc2_db3_slot;
        let mut var_vschfc2_db30: f64 = *var_vschfc2_db30_slot;
        let mut var_vschfc2_db31: f64 = *var_vschfc2_db31_slot;
        let mut var_vschfc2_db32: f64 = *var_vschfc2_db32_slot;
        let mut var_vschfc2_db33: f64 = *var_vschfc2_db33_slot;
        let mut var_vschfc2_db34: f64 = *var_vschfc2_db34_slot;
        let mut var_vschfc2_db35: f64 = *var_vschfc2_db35_slot;
        let mut var_vschfc2_db4: f64 = *var_vschfc2_db4_slot;
        let mut var_vschfc2_db5: f64 = *var_vschfc2_db5_slot;
        let mut var_vschfc2_db6: f64 = *var_vschfc2_db6_slot;
        let mut var_vschfc2_db7: f64 = *var_vschfc2_db7_slot;
        let mut var_vschfc2_db8: f64 = *var_vschfc2_db8_slot;
        let mut var_vschfc2_db9: f64 = *var_vschfc2_db9_slot;
        let mut var_vschfc2_dn0: f64 = *var_vschfc2_dn0_slot;
        let mut var_vschfc2_dn1: f64 = *var_vschfc2_dn1_slot;
        let mut var_vschfc2_dn10: f64 = *var_vschfc2_dn10_slot;
        let mut var_vschfc2_dn11: f64 = *var_vschfc2_dn11_slot;
        let mut var_vschfc2_dn12: f64 = *var_vschfc2_dn12_slot;
        let mut var_vschfc2_dn13: f64 = *var_vschfc2_dn13_slot;
        let mut var_vschfc2_dn14: f64 = *var_vschfc2_dn14_slot;
        let mut var_vschfc2_dn15: f64 = *var_vschfc2_dn15_slot;
        let mut var_vschfc2_dn16: f64 = *var_vschfc2_dn16_slot;
        let mut var_vschfc2_dn17: f64 = *var_vschfc2_dn17_slot;
        let mut var_vschfc2_dn18: f64 = *var_vschfc2_dn18_slot;
        let mut var_vschfc2_dn19: f64 = *var_vschfc2_dn19_slot;
        let mut var_vschfc2_dn2: f64 = *var_vschfc2_dn2_slot;
        let mut var_vschfc2_dn20: f64 = *var_vschfc2_dn20_slot;
        let mut var_vschfc2_dn21: f64 = *var_vschfc2_dn21_slot;
        let mut var_vschfc2_dn22: f64 = *var_vschfc2_dn22_slot;
        let mut var_vschfc2_dn23: f64 = *var_vschfc2_dn23_slot;
        let mut var_vschfc2_dn24: f64 = *var_vschfc2_dn24_slot;
        let mut var_vschfc2_dn25: f64 = *var_vschfc2_dn25_slot;
        let mut var_vschfc2_dn26: f64 = *var_vschfc2_dn26_slot;
        let mut var_vschfc2_dn27: f64 = *var_vschfc2_dn27_slot;
        let mut var_vschfc2_dn28: f64 = *var_vschfc2_dn28_slot;
        let mut var_vschfc2_dn29: f64 = *var_vschfc2_dn29_slot;
        let mut var_vschfc2_dn3: f64 = *var_vschfc2_dn3_slot;
        let mut var_vschfc2_dn4: f64 = *var_vschfc2_dn4_slot;
        let mut var_vschfc2_dn5: f64 = *var_vschfc2_dn5_slot;
        let mut var_vschfc2_dn6: f64 = *var_vschfc2_dn6_slot;
        let mut var_vschfc2_dn7: f64 = *var_vschfc2_dn7_slot;
        let mut var_vschfc2_dn8: f64 = *var_vschfc2_dn8_slot;
        let mut var_vschfc2_dn9: f64 = *var_vschfc2_dn9_slot;

        let (assign43630_e42237, assign43630_e42237_d_n0, assign43630_e42237_d_n1, assign43630_e42237_d_n2, assign43630_e42237_d_n3, assign43630_e42237_d_n4, assign43630_e42237_d_n5, assign43630_e42237_d_n6, assign43630_e42237_d_n7, assign43630_e42237_d_n8, assign43630_e42237_d_n9, assign43630_e42237_d_n10, assign43630_e42237_d_n11, assign43630_e42237_d_n12, assign43630_e42237_d_n13, assign43630_e42237_d_n14, assign43630_e42237_d_n15, assign43630_e42237_d_n16, assign43630_e42237_d_n17, assign43630_e42237_d_n18, assign43630_e42237_d_n19, assign43630_e42237_d_n20, assign43630_e42237_d_n21, assign43630_e42237_d_n22, assign43630_e42237_d_n23, assign43630_e42237_d_n24, assign43630_e42237_d_n25, assign43630_e42237_d_n26, assign43630_e42237_d_n27, assign43630_e42237_d_n28, assign43630_e42237_d_n29, assign43630_e42237_d_b0, assign43630_e42237_d_b1, assign43630_e42237_d_b2, assign43630_e42237_d_b3, assign43630_e42237_d_b4, assign43630_e42237_d_b5, assign43630_e42237_d_b6, assign43630_e42237_d_b7, assign43630_e42237_d_b8, assign43630_e42237_d_b9, assign43630_e42237_d_b10, assign43630_e42237_d_b11, assign43630_e42237_d_b12, assign43630_e42237_d_b13, assign43630_e42237_d_b14, assign43630_e42237_d_b15, assign43630_e42237_d_b16, assign43630_e42237_d_b17, assign43630_e42237_d_b18, assign43630_e42237_d_b19, assign43630_e42237_d_b20, assign43630_e42237_d_b21, assign43630_e42237_d_b22, assign43630_e42237_d_b23, assign43630_e42237_d_b24, assign43630_e42237_d_b25, assign43630_e42237_d_b26, assign43630_e42237_d_b27, assign43630_e42237_d_b28, assign43630_e42237_d_b29, assign43630_e42237_d_b30, assign43630_e42237_d_b31, assign43630_e42237_d_b32, assign43630_e42237_d_b33, assign43630_e42237_d_b34, assign43630_e42237_d_b35,) = {
    if ((var_guard461 != 0.0) && (var_guard473 != 0.0)) {
        let assign43630_e42214: f64 = (p.p6 * 2.0);
        let assign43630_e42216: f64 = (assign43630_e42214 * p.p307);
        let assign43630_e42218: f64 = (assign43630_e42216 * p.p0);
        let assign43630_e42221: f64 = (1.0 - p.p311);
        let assign43630_e42222: f64 = (assign43630_e42218 * assign43630_e42221);
        let assign43630_e42224: f64 = (assign43630_e42222 * p.p2);
        let assign43630_e42226: f64 = (assign43630_e42224 * p.p306);
        let assign43630_e42231: f64 = (var_vsch / p.p306);
        let assign43630_e42232: f64 = (1.0 - assign43630_e42231);
        let assign43630_e42233: f64 = (assign43630_e42232).sqrt();
        let assign43630_e42234: f64 = (1.0 - assign43630_e42233);
        let assign43630_e42235: f64 = (assign43630_e42226 * assign43630_e42234);
        (assign43630_e42235, (assign43630_e42226 * (-((-(var_vsch_dn0 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn1 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn2 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn3 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn4 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn5 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn6 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn7 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn8 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn9 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn10 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn11 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn12 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn13 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn14 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn15 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn16 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn17 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn18 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn19 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn20 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn21 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn22 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn23 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn24 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn25 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn26 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn27 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn28 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_dn29 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db0 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db1 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db2 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db3 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db4 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db5 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db6 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db7 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db8 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db9 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db10 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db11 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db12 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db13 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db14 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db15 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db16 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db17 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db18 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db19 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db20 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db21 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db22 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db23 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db24 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db25 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db26 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db27 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db28 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db29 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db30 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db31 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db32 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db33 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db34 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(var_vsch_db35 / p.p306)) / (2.0 * assign43630_e42233)))),)
    } else {
        (var_qsch, var_qsch_dn0, var_qsch_dn1, var_qsch_dn2, var_qsch_dn3, var_qsch_dn4, var_qsch_dn5, var_qsch_dn6, var_qsch_dn7, var_qsch_dn8, var_qsch_dn9, var_qsch_dn10, var_qsch_dn11, var_qsch_dn12, var_qsch_dn13, var_qsch_dn14, var_qsch_dn15, var_qsch_dn16, var_qsch_dn17, var_qsch_dn18, var_qsch_dn19, var_qsch_dn20, var_qsch_dn21, var_qsch_dn22, var_qsch_dn23, var_qsch_dn24, var_qsch_dn25, var_qsch_dn26, var_qsch_dn27, var_qsch_dn28, var_qsch_dn29, var_qsch_db0, var_qsch_db1, var_qsch_db2, var_qsch_db3, var_qsch_db4, var_qsch_db5, var_qsch_db6, var_qsch_db7, var_qsch_db8, var_qsch_db9, var_qsch_db10, var_qsch_db11, var_qsch_db12, var_qsch_db13, var_qsch_db14, var_qsch_db15, var_qsch_db16, var_qsch_db17, var_qsch_db18, var_qsch_db19, var_qsch_db20, var_qsch_db21, var_qsch_db22, var_qsch_db23, var_qsch_db24, var_qsch_db25, var_qsch_db26, var_qsch_db27, var_qsch_db28, var_qsch_db29, var_qsch_db30, var_qsch_db31, var_qsch_db32, var_qsch_db33, var_qsch_db34, var_qsch_db35,)
    }
};
        var_qsch = assign43630_e42237;
        var_qsch_dn0 = assign43630_e42237_d_n0;
        var_qsch_dn1 = assign43630_e42237_d_n1;
        var_qsch_dn2 = assign43630_e42237_d_n2;
        var_qsch_dn3 = assign43630_e42237_d_n3;
        var_qsch_dn4 = assign43630_e42237_d_n4;
        var_qsch_dn5 = assign43630_e42237_d_n5;
        var_qsch_dn6 = assign43630_e42237_d_n6;
        var_qsch_dn7 = assign43630_e42237_d_n7;
        var_qsch_dn8 = assign43630_e42237_d_n8;
        var_qsch_dn9 = assign43630_e42237_d_n9;
        var_qsch_dn10 = assign43630_e42237_d_n10;
        var_qsch_dn11 = assign43630_e42237_d_n11;
        var_qsch_dn12 = assign43630_e42237_d_n12;
        var_qsch_dn13 = assign43630_e42237_d_n13;
        var_qsch_dn14 = assign43630_e42237_d_n14;
        var_qsch_dn15 = assign43630_e42237_d_n15;
        var_qsch_dn16 = assign43630_e42237_d_n16;
        var_qsch_dn17 = assign43630_e42237_d_n17;
        var_qsch_dn18 = assign43630_e42237_d_n18;
        var_qsch_dn19 = assign43630_e42237_d_n19;
        var_qsch_dn20 = assign43630_e42237_d_n20;
        var_qsch_dn21 = assign43630_e42237_d_n21;
        var_qsch_dn22 = assign43630_e42237_d_n22;
        var_qsch_dn23 = assign43630_e42237_d_n23;
        var_qsch_dn24 = assign43630_e42237_d_n24;
        var_qsch_dn25 = assign43630_e42237_d_n25;
        var_qsch_dn26 = assign43630_e42237_d_n26;
        var_qsch_dn27 = assign43630_e42237_d_n27;
        var_qsch_dn28 = assign43630_e42237_d_n28;
        var_qsch_dn29 = assign43630_e42237_d_n29;
        var_qsch_db0 = assign43630_e42237_d_b0;
        var_qsch_db1 = assign43630_e42237_d_b1;
        var_qsch_db2 = assign43630_e42237_d_b2;
        var_qsch_db3 = assign43630_e42237_d_b3;
        var_qsch_db4 = assign43630_e42237_d_b4;
        var_qsch_db5 = assign43630_e42237_d_b5;
        var_qsch_db6 = assign43630_e42237_d_b6;
        var_qsch_db7 = assign43630_e42237_d_b7;
        var_qsch_db8 = assign43630_e42237_d_b8;
        var_qsch_db9 = assign43630_e42237_d_b9;
        var_qsch_db10 = assign43630_e42237_d_b10;
        var_qsch_db11 = assign43630_e42237_d_b11;
        var_qsch_db12 = assign43630_e42237_d_b12;
        var_qsch_db13 = assign43630_e42237_d_b13;
        var_qsch_db14 = assign43630_e42237_d_b14;
        var_qsch_db15 = assign43630_e42237_d_b15;
        var_qsch_db16 = assign43630_e42237_d_b16;
        var_qsch_db17 = assign43630_e42237_d_b17;
        var_qsch_db18 = assign43630_e42237_d_b18;
        var_qsch_db19 = assign43630_e42237_d_b19;
        var_qsch_db20 = assign43630_e42237_d_b20;
        var_qsch_db21 = assign43630_e42237_d_b21;
        var_qsch_db22 = assign43630_e42237_d_b22;
        var_qsch_db23 = assign43630_e42237_d_b23;
        var_qsch_db24 = assign43630_e42237_d_b24;
        var_qsch_db25 = assign43630_e42237_d_b25;
        var_qsch_db26 = assign43630_e42237_d_b26;
        var_qsch_db27 = assign43630_e42237_d_b27;
        var_qsch_db28 = assign43630_e42237_d_b28;
        var_qsch_db29 = assign43630_e42237_d_b29;
        var_qsch_db30 = assign43630_e42237_d_b30;
        var_qsch_db31 = assign43630_e42237_d_b31;
        var_qsch_db32 = assign43630_e42237_d_b32;
        var_qsch_db33 = assign43630_e42237_d_b33;
        var_qsch_db34 = assign43630_e42237_d_b34;
        var_qsch_db35 = assign43630_e42237_d_b35;

        let (assign43640_e42249,) = {
    if ((var_guard461 != 0.0) && (var_guard473 == 0.0)) {
        let assign43640_e42245: f64 = (1.0 - p.p308);
        let assign43640_e42246: f64 = (assign43640_e42245).sqrt();
        let assign43640_e42247: f64 = (1.0 - assign43640_e42246);
        (assign43640_e42247,)
    } else {
        (var_qsch0,)
    }
};
        var_qsch0 = assign43640_e42249;

        let assign43650_e42252: f64 = if p.p309 >= 1.0 { 1.0 } else { 0.0 };
        var_guard474 = assign43650_e42252;

        let (assign43660_e42270,) = {
    if (((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) {
        let assign43660_e42262: f64 = (2.0 * p.p306);
        let assign43660_e42265: f64 = (1.0 - p.p308);
        let assign43660_e42266: f64 = (assign43660_e42265).sqrt();
        let assign43660_e42267: f64 = (assign43660_e42262 * assign43660_e42266);
        let assign43660_e42268: f64 = (1.0 / assign43660_e42267);
        (assign43660_e42268,)
    } else {
        (var_qsch1c,)
    }
};
        var_qsch1c = assign43660_e42270;

        let (assign43670_e42283, assign43670_e42283_d_n0, assign43670_e42283_d_n1, assign43670_e42283_d_n2, assign43670_e42283_d_n3, assign43670_e42283_d_n4, assign43670_e42283_d_n5, assign43670_e42283_d_n6, assign43670_e42283_d_n7, assign43670_e42283_d_n8, assign43670_e42283_d_n9, assign43670_e42283_d_n10, assign43670_e42283_d_n11, assign43670_e42283_d_n12, assign43670_e42283_d_n13, assign43670_e42283_d_n14, assign43670_e42283_d_n15, assign43670_e42283_d_n16, assign43670_e42283_d_n17, assign43670_e42283_d_n18, assign43670_e42283_d_n19, assign43670_e42283_d_n20, assign43670_e42283_d_n21, assign43670_e42283_d_n22, assign43670_e42283_d_n23, assign43670_e42283_d_n24, assign43670_e42283_d_n25, assign43670_e42283_d_n26, assign43670_e42283_d_n27, assign43670_e42283_d_n28, assign43670_e42283_d_n29, assign43670_e42283_d_b0, assign43670_e42283_d_b1, assign43670_e42283_d_b2, assign43670_e42283_d_b3, assign43670_e42283_d_b4, assign43670_e42283_d_b5, assign43670_e42283_d_b6, assign43670_e42283_d_b7, assign43670_e42283_d_b8, assign43670_e42283_d_b9, assign43670_e42283_d_b10, assign43670_e42283_d_b11, assign43670_e42283_d_b12, assign43670_e42283_d_b13, assign43670_e42283_d_b14, assign43670_e42283_d_b15, assign43670_e42283_d_b16, assign43670_e42283_d_b17, assign43670_e42283_d_b18, assign43670_e42283_d_b19, assign43670_e42283_d_b20, assign43670_e42283_d_b21, assign43670_e42283_d_b22, assign43670_e42283_d_b23, assign43670_e42283_d_b24, assign43670_e42283_d_b25, assign43670_e42283_d_b26, assign43670_e42283_d_b27, assign43670_e42283_d_b28, assign43670_e42283_d_b29, assign43670_e42283_d_b30, assign43670_e42283_d_b31, assign43670_e42283_d_b32, assign43670_e42283_d_b33, assign43670_e42283_d_b34, assign43670_e42283_d_b35,) = {
    if (((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) {
        let assign43670_e42280: f64 = (p.p308 * p.p306);
        let assign43670_e42281: f64 = (var_vsch - assign43670_e42280);
        (assign43670_e42281, var_vsch_dn0, var_vsch_dn1, var_vsch_dn2, var_vsch_dn3, var_vsch_dn4, var_vsch_dn5, var_vsch_dn6, var_vsch_dn7, var_vsch_dn8, var_vsch_dn9, var_vsch_dn10, var_vsch_dn11, var_vsch_dn12, var_vsch_dn13, var_vsch_dn14, var_vsch_dn15, var_vsch_dn16, var_vsch_dn17, var_vsch_dn18, var_vsch_dn19, var_vsch_dn20, var_vsch_dn21, var_vsch_dn22, var_vsch_dn23, var_vsch_dn24, var_vsch_dn25, var_vsch_dn26, var_vsch_dn27, var_vsch_dn28, var_vsch_dn29, var_vsch_db0, var_vsch_db1, var_vsch_db2, var_vsch_db3, var_vsch_db4, var_vsch_db5, var_vsch_db6, var_vsch_db7, var_vsch_db8, var_vsch_db9, var_vsch_db10, var_vsch_db11, var_vsch_db12, var_vsch_db13, var_vsch_db14, var_vsch_db15, var_vsch_db16, var_vsch_db17, var_vsch_db18, var_vsch_db19, var_vsch_db20, var_vsch_db21, var_vsch_db22, var_vsch_db23, var_vsch_db24, var_vsch_db25, var_vsch_db26, var_vsch_db27, var_vsch_db28, var_vsch_db29, var_vsch_db30, var_vsch_db31, var_vsch_db32, var_vsch_db33, var_vsch_db34, var_vsch_db35,)
    } else {
        (var_vschfc1, var_vschfc1_dn0, var_vschfc1_dn1, var_vschfc1_dn2, var_vschfc1_dn3, var_vschfc1_dn4, var_vschfc1_dn5, var_vschfc1_dn6, var_vschfc1_dn7, var_vschfc1_dn8, var_vschfc1_dn9, var_vschfc1_dn10, var_vschfc1_dn11, var_vschfc1_dn12, var_vschfc1_dn13, var_vschfc1_dn14, var_vschfc1_dn15, var_vschfc1_dn16, var_vschfc1_dn17, var_vschfc1_dn18, var_vschfc1_dn19, var_vschfc1_dn20, var_vschfc1_dn21, var_vschfc1_dn22, var_vschfc1_dn23, var_vschfc1_dn24, var_vschfc1_dn25, var_vschfc1_dn26, var_vschfc1_dn27, var_vschfc1_dn28, var_vschfc1_dn29, var_vschfc1_db0, var_vschfc1_db1, var_vschfc1_db2, var_vschfc1_db3, var_vschfc1_db4, var_vschfc1_db5, var_vschfc1_db6, var_vschfc1_db7, var_vschfc1_db8, var_vschfc1_db9, var_vschfc1_db10, var_vschfc1_db11, var_vschfc1_db12, var_vschfc1_db13, var_vschfc1_db14, var_vschfc1_db15, var_vschfc1_db16, var_vschfc1_db17, var_vschfc1_db18, var_vschfc1_db19, var_vschfc1_db20, var_vschfc1_db21, var_vschfc1_db22, var_vschfc1_db23, var_vschfc1_db24, var_vschfc1_db25, var_vschfc1_db26, var_vschfc1_db27, var_vschfc1_db28, var_vschfc1_db29, var_vschfc1_db30, var_vschfc1_db31, var_vschfc1_db32, var_vschfc1_db33, var_vschfc1_db34, var_vschfc1_db35,)
    }
};
        var_vschfc1 = assign43670_e42283;
        var_vschfc1_dn0 = assign43670_e42283_d_n0;
        var_vschfc1_dn1 = assign43670_e42283_d_n1;
        var_vschfc1_dn2 = assign43670_e42283_d_n2;
        var_vschfc1_dn3 = assign43670_e42283_d_n3;
        var_vschfc1_dn4 = assign43670_e42283_d_n4;
        var_vschfc1_dn5 = assign43670_e42283_d_n5;
        var_vschfc1_dn6 = assign43670_e42283_d_n6;
        var_vschfc1_dn7 = assign43670_e42283_d_n7;
        var_vschfc1_dn8 = assign43670_e42283_d_n8;
        var_vschfc1_dn9 = assign43670_e42283_d_n9;
        var_vschfc1_dn10 = assign43670_e42283_d_n10;
        var_vschfc1_dn11 = assign43670_e42283_d_n11;
        var_vschfc1_dn12 = assign43670_e42283_d_n12;
        var_vschfc1_dn13 = assign43670_e42283_d_n13;
        var_vschfc1_dn14 = assign43670_e42283_d_n14;
        var_vschfc1_dn15 = assign43670_e42283_d_n15;
        var_vschfc1_dn16 = assign43670_e42283_d_n16;
        var_vschfc1_dn17 = assign43670_e42283_d_n17;
        var_vschfc1_dn18 = assign43670_e42283_d_n18;
        var_vschfc1_dn19 = assign43670_e42283_d_n19;
        var_vschfc1_dn20 = assign43670_e42283_d_n20;
        var_vschfc1_dn21 = assign43670_e42283_d_n21;
        var_vschfc1_dn22 = assign43670_e42283_d_n22;
        var_vschfc1_dn23 = assign43670_e42283_d_n23;
        var_vschfc1_dn24 = assign43670_e42283_d_n24;
        var_vschfc1_dn25 = assign43670_e42283_d_n25;
        var_vschfc1_dn26 = assign43670_e42283_d_n26;
        var_vschfc1_dn27 = assign43670_e42283_d_n27;
        var_vschfc1_dn28 = assign43670_e42283_d_n28;
        var_vschfc1_dn29 = assign43670_e42283_d_n29;
        var_vschfc1_db0 = assign43670_e42283_d_b0;
        var_vschfc1_db1 = assign43670_e42283_d_b1;
        var_vschfc1_db2 = assign43670_e42283_d_b2;
        var_vschfc1_db3 = assign43670_e42283_d_b3;
        var_vschfc1_db4 = assign43670_e42283_d_b4;
        var_vschfc1_db5 = assign43670_e42283_d_b5;
        var_vschfc1_db6 = assign43670_e42283_d_b6;
        var_vschfc1_db7 = assign43670_e42283_d_b7;
        var_vschfc1_db8 = assign43670_e42283_d_b8;
        var_vschfc1_db9 = assign43670_e42283_d_b9;
        var_vschfc1_db10 = assign43670_e42283_d_b10;
        var_vschfc1_db11 = assign43670_e42283_d_b11;
        var_vschfc1_db12 = assign43670_e42283_d_b12;
        var_vschfc1_db13 = assign43670_e42283_d_b13;
        var_vschfc1_db14 = assign43670_e42283_d_b14;
        var_vschfc1_db15 = assign43670_e42283_d_b15;
        var_vschfc1_db16 = assign43670_e42283_d_b16;
        var_vschfc1_db17 = assign43670_e42283_d_b17;
        var_vschfc1_db18 = assign43670_e42283_d_b18;
        var_vschfc1_db19 = assign43670_e42283_d_b19;
        var_vschfc1_db20 = assign43670_e42283_d_b20;
        var_vschfc1_db21 = assign43670_e42283_d_b21;
        var_vschfc1_db22 = assign43670_e42283_d_b22;
        var_vschfc1_db23 = assign43670_e42283_d_b23;
        var_vschfc1_db24 = assign43670_e42283_d_b24;
        var_vschfc1_db25 = assign43670_e42283_d_b25;
        var_vschfc1_db26 = assign43670_e42283_d_b26;
        var_vschfc1_db27 = assign43670_e42283_d_b27;
        var_vschfc1_db28 = assign43670_e42283_d_b28;
        var_vschfc1_db29 = assign43670_e42283_d_b29;
        var_vschfc1_db30 = assign43670_e42283_d_b30;
        var_vschfc1_db31 = assign43670_e42283_d_b31;
        var_vschfc1_db32 = assign43670_e42283_d_b32;
        var_vschfc1_db33 = assign43670_e42283_d_b33;
        var_vschfc1_db34 = assign43670_e42283_d_b34;
        var_vschfc1_db35 = assign43670_e42283_d_b35;

        let (assign43680_e42294, assign43680_e42294_d_n0, assign43680_e42294_d_n1, assign43680_e42294_d_n2, assign43680_e42294_d_n3, assign43680_e42294_d_n4, assign43680_e42294_d_n5, assign43680_e42294_d_n6, assign43680_e42294_d_n7, assign43680_e42294_d_n8, assign43680_e42294_d_n9, assign43680_e42294_d_n10, assign43680_e42294_d_n11, assign43680_e42294_d_n12, assign43680_e42294_d_n13, assign43680_e42294_d_n14, assign43680_e42294_d_n15, assign43680_e42294_d_n16, assign43680_e42294_d_n17, assign43680_e42294_d_n18, assign43680_e42294_d_n19, assign43680_e42294_d_n20, assign43680_e42294_d_n21, assign43680_e42294_d_n22, assign43680_e42294_d_n23, assign43680_e42294_d_n24, assign43680_e42294_d_n25, assign43680_e42294_d_n26, assign43680_e42294_d_n27, assign43680_e42294_d_n28, assign43680_e42294_d_n29, assign43680_e42294_d_b0, assign43680_e42294_d_b1, assign43680_e42294_d_b2, assign43680_e42294_d_b3, assign43680_e42294_d_b4, assign43680_e42294_d_b5, assign43680_e42294_d_b6, assign43680_e42294_d_b7, assign43680_e42294_d_b8, assign43680_e42294_d_b9, assign43680_e42294_d_b10, assign43680_e42294_d_b11, assign43680_e42294_d_b12, assign43680_e42294_d_b13, assign43680_e42294_d_b14, assign43680_e42294_d_b15, assign43680_e42294_d_b16, assign43680_e42294_d_b17, assign43680_e42294_d_b18, assign43680_e42294_d_b19, assign43680_e42294_d_b20, assign43680_e42294_d_b21, assign43680_e42294_d_b22, assign43680_e42294_d_b23, assign43680_e42294_d_b24, assign43680_e42294_d_b25, assign43680_e42294_d_b26, assign43680_e42294_d_b27, assign43680_e42294_d_b28, assign43680_e42294_d_b29, assign43680_e42294_d_b30, assign43680_e42294_d_b31, assign43680_e42294_d_b32, assign43680_e42294_d_b33, assign43680_e42294_d_b34, assign43680_e42294_d_b35,) = {
    if (((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) {
        let assign43680_e42292: f64 = (var_qsch1c * var_vschfc1);
        (assign43680_e42292, (var_qsch1c * var_vschfc1_dn0), (var_qsch1c * var_vschfc1_dn1), (var_qsch1c * var_vschfc1_dn2), (var_qsch1c * var_vschfc1_dn3), (var_qsch1c * var_vschfc1_dn4), (var_qsch1c * var_vschfc1_dn5), (var_qsch1c * var_vschfc1_dn6), (var_qsch1c * var_vschfc1_dn7), (var_qsch1c * var_vschfc1_dn8), (var_qsch1c * var_vschfc1_dn9), (var_qsch1c * var_vschfc1_dn10), (var_qsch1c * var_vschfc1_dn11), (var_qsch1c * var_vschfc1_dn12), (var_qsch1c * var_vschfc1_dn13), (var_qsch1c * var_vschfc1_dn14), (var_qsch1c * var_vschfc1_dn15), (var_qsch1c * var_vschfc1_dn16), (var_qsch1c * var_vschfc1_dn17), (var_qsch1c * var_vschfc1_dn18), (var_qsch1c * var_vschfc1_dn19), (var_qsch1c * var_vschfc1_dn20), (var_qsch1c * var_vschfc1_dn21), (var_qsch1c * var_vschfc1_dn22), (var_qsch1c * var_vschfc1_dn23), (var_qsch1c * var_vschfc1_dn24), (var_qsch1c * var_vschfc1_dn25), (var_qsch1c * var_vschfc1_dn26), (var_qsch1c * var_vschfc1_dn27), (var_qsch1c * var_vschfc1_dn28), (var_qsch1c * var_vschfc1_dn29), (var_qsch1c * var_vschfc1_db0), (var_qsch1c * var_vschfc1_db1), (var_qsch1c * var_vschfc1_db2), (var_qsch1c * var_vschfc1_db3), (var_qsch1c * var_vschfc1_db4), (var_qsch1c * var_vschfc1_db5), (var_qsch1c * var_vschfc1_db6), (var_qsch1c * var_vschfc1_db7), (var_qsch1c * var_vschfc1_db8), (var_qsch1c * var_vschfc1_db9), (var_qsch1c * var_vschfc1_db10), (var_qsch1c * var_vschfc1_db11), (var_qsch1c * var_vschfc1_db12), (var_qsch1c * var_vschfc1_db13), (var_qsch1c * var_vschfc1_db14), (var_qsch1c * var_vschfc1_db15), (var_qsch1c * var_vschfc1_db16), (var_qsch1c * var_vschfc1_db17), (var_qsch1c * var_vschfc1_db18), (var_qsch1c * var_vschfc1_db19), (var_qsch1c * var_vschfc1_db20), (var_qsch1c * var_vschfc1_db21), (var_qsch1c * var_vschfc1_db22), (var_qsch1c * var_vschfc1_db23), (var_qsch1c * var_vschfc1_db24), (var_qsch1c * var_vschfc1_db25), (var_qsch1c * var_vschfc1_db26), (var_qsch1c * var_vschfc1_db27), (var_qsch1c * var_vschfc1_db28), (var_qsch1c * var_vschfc1_db29), (var_qsch1c * var_vschfc1_db30), (var_qsch1c * var_vschfc1_db31), (var_qsch1c * var_vschfc1_db32), (var_qsch1c * var_vschfc1_db33), (var_qsch1c * var_vschfc1_db34), (var_qsch1c * var_vschfc1_db35),)
    } else {
        (var_qsch1, var_qsch1_dn0, var_qsch1_dn1, var_qsch1_dn2, var_qsch1_dn3, var_qsch1_dn4, var_qsch1_dn5, var_qsch1_dn6, var_qsch1_dn7, var_qsch1_dn8, var_qsch1_dn9, var_qsch1_dn10, var_qsch1_dn11, var_qsch1_dn12, var_qsch1_dn13, var_qsch1_dn14, var_qsch1_dn15, var_qsch1_dn16, var_qsch1_dn17, var_qsch1_dn18, var_qsch1_dn19, var_qsch1_dn20, var_qsch1_dn21, var_qsch1_dn22, var_qsch1_dn23, var_qsch1_dn24, var_qsch1_dn25, var_qsch1_dn26, var_qsch1_dn27, var_qsch1_dn28, var_qsch1_dn29, var_qsch1_db0, var_qsch1_db1, var_qsch1_db2, var_qsch1_db3, var_qsch1_db4, var_qsch1_db5, var_qsch1_db6, var_qsch1_db7, var_qsch1_db8, var_qsch1_db9, var_qsch1_db10, var_qsch1_db11, var_qsch1_db12, var_qsch1_db13, var_qsch1_db14, var_qsch1_db15, var_qsch1_db16, var_qsch1_db17, var_qsch1_db18, var_qsch1_db19, var_qsch1_db20, var_qsch1_db21, var_qsch1_db22, var_qsch1_db23, var_qsch1_db24, var_qsch1_db25, var_qsch1_db26, var_qsch1_db27, var_qsch1_db28, var_qsch1_db29, var_qsch1_db30, var_qsch1_db31, var_qsch1_db32, var_qsch1_db33, var_qsch1_db34, var_qsch1_db35,)
    }
};
        var_qsch1 = assign43680_e42294;
        var_qsch1_dn0 = assign43680_e42294_d_n0;
        var_qsch1_dn1 = assign43680_e42294_d_n1;
        var_qsch1_dn2 = assign43680_e42294_d_n2;
        var_qsch1_dn3 = assign43680_e42294_d_n3;
        var_qsch1_dn4 = assign43680_e42294_d_n4;
        var_qsch1_dn5 = assign43680_e42294_d_n5;
        var_qsch1_dn6 = assign43680_e42294_d_n6;
        var_qsch1_dn7 = assign43680_e42294_d_n7;
        var_qsch1_dn8 = assign43680_e42294_d_n8;
        var_qsch1_dn9 = assign43680_e42294_d_n9;
        var_qsch1_dn10 = assign43680_e42294_d_n10;
        var_qsch1_dn11 = assign43680_e42294_d_n11;
        var_qsch1_dn12 = assign43680_e42294_d_n12;
        var_qsch1_dn13 = assign43680_e42294_d_n13;
        var_qsch1_dn14 = assign43680_e42294_d_n14;
        var_qsch1_dn15 = assign43680_e42294_d_n15;
        var_qsch1_dn16 = assign43680_e42294_d_n16;
        var_qsch1_dn17 = assign43680_e42294_d_n17;
        var_qsch1_dn18 = assign43680_e42294_d_n18;
        var_qsch1_dn19 = assign43680_e42294_d_n19;
        var_qsch1_dn20 = assign43680_e42294_d_n20;
        var_qsch1_dn21 = assign43680_e42294_d_n21;
        var_qsch1_dn22 = assign43680_e42294_d_n22;
        var_qsch1_dn23 = assign43680_e42294_d_n23;
        var_qsch1_dn24 = assign43680_e42294_d_n24;
        var_qsch1_dn25 = assign43680_e42294_d_n25;
        var_qsch1_dn26 = assign43680_e42294_d_n26;
        var_qsch1_dn27 = assign43680_e42294_d_n27;
        var_qsch1_dn28 = assign43680_e42294_d_n28;
        var_qsch1_dn29 = assign43680_e42294_d_n29;
        var_qsch1_db0 = assign43680_e42294_d_b0;
        var_qsch1_db1 = assign43680_e42294_d_b1;
        var_qsch1_db2 = assign43680_e42294_d_b2;
        var_qsch1_db3 = assign43680_e42294_d_b3;
        var_qsch1_db4 = assign43680_e42294_d_b4;
        var_qsch1_db5 = assign43680_e42294_d_b5;
        var_qsch1_db6 = assign43680_e42294_d_b6;
        var_qsch1_db7 = assign43680_e42294_d_b7;
        var_qsch1_db8 = assign43680_e42294_d_b8;
        var_qsch1_db9 = assign43680_e42294_d_b9;
        var_qsch1_db10 = assign43680_e42294_d_b10;
        var_qsch1_db11 = assign43680_e42294_d_b11;
        var_qsch1_db12 = assign43680_e42294_d_b12;
        var_qsch1_db13 = assign43680_e42294_d_b13;
        var_qsch1_db14 = assign43680_e42294_d_b14;
        var_qsch1_db15 = assign43680_e42294_d_b15;
        var_qsch1_db16 = assign43680_e42294_d_b16;
        var_qsch1_db17 = assign43680_e42294_d_b17;
        var_qsch1_db18 = assign43680_e42294_d_b18;
        var_qsch1_db19 = assign43680_e42294_d_b19;
        var_qsch1_db20 = assign43680_e42294_d_b20;
        var_qsch1_db21 = assign43680_e42294_d_b21;
        var_qsch1_db22 = assign43680_e42294_d_b22;
        var_qsch1_db23 = assign43680_e42294_d_b23;
        var_qsch1_db24 = assign43680_e42294_d_b24;
        var_qsch1_db25 = assign43680_e42294_d_b25;
        var_qsch1_db26 = assign43680_e42294_d_b26;
        var_qsch1_db27 = assign43680_e42294_d_b27;
        var_qsch1_db28 = assign43680_e42294_d_b28;
        var_qsch1_db29 = assign43680_e42294_d_b29;
        var_qsch1_db30 = assign43680_e42294_d_b30;
        var_qsch1_db31 = assign43680_e42294_d_b31;
        var_qsch1_db32 = assign43680_e42294_d_b32;
        var_qsch1_db33 = assign43680_e42294_d_b33;
        var_qsch1_db34 = assign43680_e42294_d_b34;
        var_qsch1_db35 = assign43680_e42294_d_b35;

        let assign43690_e42297: f64 = if p.p309 >= 2.0 { 1.0 } else { 0.0 };
        var_guard475 = assign43690_e42297;

        let (assign43700_e42316,) = {
    if ((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) {
        let assign43700_e42309: f64 = (4.0 * p.p306);
        let assign43700_e42312: f64 = (1.0 - p.p308);
        let assign43700_e42313: f64 = (assign43700_e42309 * assign43700_e42312);
        let assign43700_e42314: f64 = (var_qsch1c / assign43700_e42313);
        (assign43700_e42314,)
    } else {
        (var_qsch2c,)
    }
};
        var_qsch2c = assign43700_e42316;

        let (assign43710_e42329, assign43710_e42329_d_n0, assign43710_e42329_d_n1, assign43710_e42329_d_n2, assign43710_e42329_d_n3, assign43710_e42329_d_n4, assign43710_e42329_d_n5, assign43710_e42329_d_n6, assign43710_e42329_d_n7, assign43710_e42329_d_n8, assign43710_e42329_d_n9, assign43710_e42329_d_n10, assign43710_e42329_d_n11, assign43710_e42329_d_n12, assign43710_e42329_d_n13, assign43710_e42329_d_n14, assign43710_e42329_d_n15, assign43710_e42329_d_n16, assign43710_e42329_d_n17, assign43710_e42329_d_n18, assign43710_e42329_d_n19, assign43710_e42329_d_n20, assign43710_e42329_d_n21, assign43710_e42329_d_n22, assign43710_e42329_d_n23, assign43710_e42329_d_n24, assign43710_e42329_d_n25, assign43710_e42329_d_n26, assign43710_e42329_d_n27, assign43710_e42329_d_n28, assign43710_e42329_d_n29, assign43710_e42329_d_b0, assign43710_e42329_d_b1, assign43710_e42329_d_b2, assign43710_e42329_d_b3, assign43710_e42329_d_b4, assign43710_e42329_d_b5, assign43710_e42329_d_b6, assign43710_e42329_d_b7, assign43710_e42329_d_b8, assign43710_e42329_d_b9, assign43710_e42329_d_b10, assign43710_e42329_d_b11, assign43710_e42329_d_b12, assign43710_e42329_d_b13, assign43710_e42329_d_b14, assign43710_e42329_d_b15, assign43710_e42329_d_b16, assign43710_e42329_d_b17, assign43710_e42329_d_b18, assign43710_e42329_d_b19, assign43710_e42329_d_b20, assign43710_e42329_d_b21, assign43710_e42329_d_b22, assign43710_e42329_d_b23, assign43710_e42329_d_b24, assign43710_e42329_d_b25, assign43710_e42329_d_b26, assign43710_e42329_d_b27, assign43710_e42329_d_b28, assign43710_e42329_d_b29, assign43710_e42329_d_b30, assign43710_e42329_d_b31, assign43710_e42329_d_b32, assign43710_e42329_d_b33, assign43710_e42329_d_b34, assign43710_e42329_d_b35,) = {
    if ((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) {
        let assign43710_e42327: f64 = (var_vschfc1 * var_vschfc1);
        (assign43710_e42327, ((var_vschfc1_dn0 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn0)), ((var_vschfc1_dn1 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn1)), ((var_vschfc1_dn2 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn2)), ((var_vschfc1_dn3 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn3)), ((var_vschfc1_dn4 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn4)), ((var_vschfc1_dn5 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn5)), ((var_vschfc1_dn6 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn6)), ((var_vschfc1_dn7 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn7)), ((var_vschfc1_dn8 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn8)), ((var_vschfc1_dn9 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn9)), ((var_vschfc1_dn10 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn10)), ((var_vschfc1_dn11 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn11)), ((var_vschfc1_dn12 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn12)), ((var_vschfc1_dn13 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn13)), ((var_vschfc1_dn14 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn14)), ((var_vschfc1_dn15 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn15)), ((var_vschfc1_dn16 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn16)), ((var_vschfc1_dn17 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn17)), ((var_vschfc1_dn18 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn18)), ((var_vschfc1_dn19 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn19)), ((var_vschfc1_dn20 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn20)), ((var_vschfc1_dn21 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn21)), ((var_vschfc1_dn22 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn22)), ((var_vschfc1_dn23 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn23)), ((var_vschfc1_dn24 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn24)), ((var_vschfc1_dn25 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn25)), ((var_vschfc1_dn26 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn26)), ((var_vschfc1_dn27 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn27)), ((var_vschfc1_dn28 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn28)), ((var_vschfc1_dn29 * var_vschfc1) + (var_vschfc1 * var_vschfc1_dn29)), ((var_vschfc1_db0 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db0)), ((var_vschfc1_db1 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db1)), ((var_vschfc1_db2 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db2)), ((var_vschfc1_db3 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db3)), ((var_vschfc1_db4 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db4)), ((var_vschfc1_db5 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db5)), ((var_vschfc1_db6 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db6)), ((var_vschfc1_db7 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db7)), ((var_vschfc1_db8 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db8)), ((var_vschfc1_db9 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db9)), ((var_vschfc1_db10 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db10)), ((var_vschfc1_db11 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db11)), ((var_vschfc1_db12 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db12)), ((var_vschfc1_db13 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db13)), ((var_vschfc1_db14 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db14)), ((var_vschfc1_db15 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db15)), ((var_vschfc1_db16 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db16)), ((var_vschfc1_db17 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db17)), ((var_vschfc1_db18 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db18)), ((var_vschfc1_db19 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db19)), ((var_vschfc1_db20 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db20)), ((var_vschfc1_db21 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db21)), ((var_vschfc1_db22 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db22)), ((var_vschfc1_db23 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db23)), ((var_vschfc1_db24 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db24)), ((var_vschfc1_db25 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db25)), ((var_vschfc1_db26 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db26)), ((var_vschfc1_db27 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db27)), ((var_vschfc1_db28 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db28)), ((var_vschfc1_db29 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db29)), ((var_vschfc1_db30 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db30)), ((var_vschfc1_db31 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db31)), ((var_vschfc1_db32 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db32)), ((var_vschfc1_db33 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db33)), ((var_vschfc1_db34 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db34)), ((var_vschfc1_db35 * var_vschfc1) + (var_vschfc1 * var_vschfc1_db35)),)
    } else {
        (var_vschfc2, var_vschfc2_dn0, var_vschfc2_dn1, var_vschfc2_dn2, var_vschfc2_dn3, var_vschfc2_dn4, var_vschfc2_dn5, var_vschfc2_dn6, var_vschfc2_dn7, var_vschfc2_dn8, var_vschfc2_dn9, var_vschfc2_dn10, var_vschfc2_dn11, var_vschfc2_dn12, var_vschfc2_dn13, var_vschfc2_dn14, var_vschfc2_dn15, var_vschfc2_dn16, var_vschfc2_dn17, var_vschfc2_dn18, var_vschfc2_dn19, var_vschfc2_dn20, var_vschfc2_dn21, var_vschfc2_dn22, var_vschfc2_dn23, var_vschfc2_dn24, var_vschfc2_dn25, var_vschfc2_dn26, var_vschfc2_dn27, var_vschfc2_dn28, var_vschfc2_dn29, var_vschfc2_db0, var_vschfc2_db1, var_vschfc2_db2, var_vschfc2_db3, var_vschfc2_db4, var_vschfc2_db5, var_vschfc2_db6, var_vschfc2_db7, var_vschfc2_db8, var_vschfc2_db9, var_vschfc2_db10, var_vschfc2_db11, var_vschfc2_db12, var_vschfc2_db13, var_vschfc2_db14, var_vschfc2_db15, var_vschfc2_db16, var_vschfc2_db17, var_vschfc2_db18, var_vschfc2_db19, var_vschfc2_db20, var_vschfc2_db21, var_vschfc2_db22, var_vschfc2_db23, var_vschfc2_db24, var_vschfc2_db25, var_vschfc2_db26, var_vschfc2_db27, var_vschfc2_db28, var_vschfc2_db29, var_vschfc2_db30, var_vschfc2_db31, var_vschfc2_db32, var_vschfc2_db33, var_vschfc2_db34, var_vschfc2_db35,)
    }
};
        var_vschfc2 = assign43710_e42329;
        var_vschfc2_dn0 = assign43710_e42329_d_n0;
        var_vschfc2_dn1 = assign43710_e42329_d_n1;
        var_vschfc2_dn2 = assign43710_e42329_d_n2;
        var_vschfc2_dn3 = assign43710_e42329_d_n3;
        var_vschfc2_dn4 = assign43710_e42329_d_n4;
        var_vschfc2_dn5 = assign43710_e42329_d_n5;
        var_vschfc2_dn6 = assign43710_e42329_d_n6;
        var_vschfc2_dn7 = assign43710_e42329_d_n7;
        var_vschfc2_dn8 = assign43710_e42329_d_n8;
        var_vschfc2_dn9 = assign43710_e42329_d_n9;
        var_vschfc2_dn10 = assign43710_e42329_d_n10;
        var_vschfc2_dn11 = assign43710_e42329_d_n11;
        var_vschfc2_dn12 = assign43710_e42329_d_n12;
        var_vschfc2_dn13 = assign43710_e42329_d_n13;
        var_vschfc2_dn14 = assign43710_e42329_d_n14;
        var_vschfc2_dn15 = assign43710_e42329_d_n15;
        var_vschfc2_dn16 = assign43710_e42329_d_n16;
        var_vschfc2_dn17 = assign43710_e42329_d_n17;
        var_vschfc2_dn18 = assign43710_e42329_d_n18;
        var_vschfc2_dn19 = assign43710_e42329_d_n19;
        var_vschfc2_dn20 = assign43710_e42329_d_n20;
        var_vschfc2_dn21 = assign43710_e42329_d_n21;
        var_vschfc2_dn22 = assign43710_e42329_d_n22;
        var_vschfc2_dn23 = assign43710_e42329_d_n23;
        var_vschfc2_dn24 = assign43710_e42329_d_n24;
        var_vschfc2_dn25 = assign43710_e42329_d_n25;
        var_vschfc2_dn26 = assign43710_e42329_d_n26;
        var_vschfc2_dn27 = assign43710_e42329_d_n27;
        var_vschfc2_dn28 = assign43710_e42329_d_n28;
        var_vschfc2_dn29 = assign43710_e42329_d_n29;
        var_vschfc2_db0 = assign43710_e42329_d_b0;
        var_vschfc2_db1 = assign43710_e42329_d_b1;
        var_vschfc2_db2 = assign43710_e42329_d_b2;
        var_vschfc2_db3 = assign43710_e42329_d_b3;
        var_vschfc2_db4 = assign43710_e42329_d_b4;
        var_vschfc2_db5 = assign43710_e42329_d_b5;
        var_vschfc2_db6 = assign43710_e42329_d_b6;
        var_vschfc2_db7 = assign43710_e42329_d_b7;
        var_vschfc2_db8 = assign43710_e42329_d_b8;
        var_vschfc2_db9 = assign43710_e42329_d_b9;
        var_vschfc2_db10 = assign43710_e42329_d_b10;
        var_vschfc2_db11 = assign43710_e42329_d_b11;
        var_vschfc2_db12 = assign43710_e42329_d_b12;
        var_vschfc2_db13 = assign43710_e42329_d_b13;
        var_vschfc2_db14 = assign43710_e42329_d_b14;
        var_vschfc2_db15 = assign43710_e42329_d_b15;
        var_vschfc2_db16 = assign43710_e42329_d_b16;
        var_vschfc2_db17 = assign43710_e42329_d_b17;
        var_vschfc2_db18 = assign43710_e42329_d_b18;
        var_vschfc2_db19 = assign43710_e42329_d_b19;
        var_vschfc2_db20 = assign43710_e42329_d_b20;
        var_vschfc2_db21 = assign43710_e42329_d_b21;
        var_vschfc2_db22 = assign43710_e42329_d_b22;
        var_vschfc2_db23 = assign43710_e42329_d_b23;
        var_vschfc2_db24 = assign43710_e42329_d_b24;
        var_vschfc2_db25 = assign43710_e42329_d_b25;
        var_vschfc2_db26 = assign43710_e42329_d_b26;
        var_vschfc2_db27 = assign43710_e42329_d_b27;
        var_vschfc2_db28 = assign43710_e42329_d_b28;
        var_vschfc2_db29 = assign43710_e42329_d_b29;
        var_vschfc2_db30 = assign43710_e42329_d_b30;
        var_vschfc2_db31 = assign43710_e42329_d_b31;
        var_vschfc2_db32 = assign43710_e42329_d_b32;
        var_vschfc2_db33 = assign43710_e42329_d_b33;
        var_vschfc2_db34 = assign43710_e42329_d_b34;
        var_vschfc2_db35 = assign43710_e42329_d_b35;

        let (assign43720_e42342, assign43720_e42342_d_n0, assign43720_e42342_d_n1, assign43720_e42342_d_n2, assign43720_e42342_d_n3, assign43720_e42342_d_n4, assign43720_e42342_d_n5, assign43720_e42342_d_n6, assign43720_e42342_d_n7, assign43720_e42342_d_n8, assign43720_e42342_d_n9, assign43720_e42342_d_n10, assign43720_e42342_d_n11, assign43720_e42342_d_n12, assign43720_e42342_d_n13, assign43720_e42342_d_n14, assign43720_e42342_d_n15, assign43720_e42342_d_n16, assign43720_e42342_d_n17, assign43720_e42342_d_n18, assign43720_e42342_d_n19, assign43720_e42342_d_n20, assign43720_e42342_d_n21, assign43720_e42342_d_n22, assign43720_e42342_d_n23, assign43720_e42342_d_n24, assign43720_e42342_d_n25, assign43720_e42342_d_n26, assign43720_e42342_d_n27, assign43720_e42342_d_n28, assign43720_e42342_d_n29, assign43720_e42342_d_b0, assign43720_e42342_d_b1, assign43720_e42342_d_b2, assign43720_e42342_d_b3, assign43720_e42342_d_b4, assign43720_e42342_d_b5, assign43720_e42342_d_b6, assign43720_e42342_d_b7, assign43720_e42342_d_b8, assign43720_e42342_d_b9, assign43720_e42342_d_b10, assign43720_e42342_d_b11, assign43720_e42342_d_b12, assign43720_e42342_d_b13, assign43720_e42342_d_b14, assign43720_e42342_d_b15, assign43720_e42342_d_b16, assign43720_e42342_d_b17, assign43720_e42342_d_b18, assign43720_e42342_d_b19, assign43720_e42342_d_b20, assign43720_e42342_d_b21, assign43720_e42342_d_b22, assign43720_e42342_d_b23, assign43720_e42342_d_b24, assign43720_e42342_d_b25, assign43720_e42342_d_b26, assign43720_e42342_d_b27, assign43720_e42342_d_b28, assign43720_e42342_d_b29, assign43720_e42342_d_b30, assign43720_e42342_d_b31, assign43720_e42342_d_b32, assign43720_e42342_d_b33, assign43720_e42342_d_b34, assign43720_e42342_d_b35,) = {
    if ((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) {
        let assign43720_e42340: f64 = (var_qsch2c * var_vschfc2);
        (assign43720_e42340, (var_qsch2c * var_vschfc2_dn0), (var_qsch2c * var_vschfc2_dn1), (var_qsch2c * var_vschfc2_dn2), (var_qsch2c * var_vschfc2_dn3), (var_qsch2c * var_vschfc2_dn4), (var_qsch2c * var_vschfc2_dn5), (var_qsch2c * var_vschfc2_dn6), (var_qsch2c * var_vschfc2_dn7), (var_qsch2c * var_vschfc2_dn8), (var_qsch2c * var_vschfc2_dn9), (var_qsch2c * var_vschfc2_dn10), (var_qsch2c * var_vschfc2_dn11), (var_qsch2c * var_vschfc2_dn12), (var_qsch2c * var_vschfc2_dn13), (var_qsch2c * var_vschfc2_dn14), (var_qsch2c * var_vschfc2_dn15), (var_qsch2c * var_vschfc2_dn16), (var_qsch2c * var_vschfc2_dn17), (var_qsch2c * var_vschfc2_dn18), (var_qsch2c * var_vschfc2_dn19), (var_qsch2c * var_vschfc2_dn20), (var_qsch2c * var_vschfc2_dn21), (var_qsch2c * var_vschfc2_dn22), (var_qsch2c * var_vschfc2_dn23), (var_qsch2c * var_vschfc2_dn24), (var_qsch2c * var_vschfc2_dn25), (var_qsch2c * var_vschfc2_dn26), (var_qsch2c * var_vschfc2_dn27), (var_qsch2c * var_vschfc2_dn28), (var_qsch2c * var_vschfc2_dn29), (var_qsch2c * var_vschfc2_db0), (var_qsch2c * var_vschfc2_db1), (var_qsch2c * var_vschfc2_db2), (var_qsch2c * var_vschfc2_db3), (var_qsch2c * var_vschfc2_db4), (var_qsch2c * var_vschfc2_db5), (var_qsch2c * var_vschfc2_db6), (var_qsch2c * var_vschfc2_db7), (var_qsch2c * var_vschfc2_db8), (var_qsch2c * var_vschfc2_db9), (var_qsch2c * var_vschfc2_db10), (var_qsch2c * var_vschfc2_db11), (var_qsch2c * var_vschfc2_db12), (var_qsch2c * var_vschfc2_db13), (var_qsch2c * var_vschfc2_db14), (var_qsch2c * var_vschfc2_db15), (var_qsch2c * var_vschfc2_db16), (var_qsch2c * var_vschfc2_db17), (var_qsch2c * var_vschfc2_db18), (var_qsch2c * var_vschfc2_db19), (var_qsch2c * var_vschfc2_db20), (var_qsch2c * var_vschfc2_db21), (var_qsch2c * var_vschfc2_db22), (var_qsch2c * var_vschfc2_db23), (var_qsch2c * var_vschfc2_db24), (var_qsch2c * var_vschfc2_db25), (var_qsch2c * var_vschfc2_db26), (var_qsch2c * var_vschfc2_db27), (var_qsch2c * var_vschfc2_db28), (var_qsch2c * var_vschfc2_db29), (var_qsch2c * var_vschfc2_db30), (var_qsch2c * var_vschfc2_db31), (var_qsch2c * var_vschfc2_db32), (var_qsch2c * var_vschfc2_db33), (var_qsch2c * var_vschfc2_db34), (var_qsch2c * var_vschfc2_db35),)
    } else {
        (var_qsch2, var_qsch2_dn0, var_qsch2_dn1, var_qsch2_dn2, var_qsch2_dn3, var_qsch2_dn4, var_qsch2_dn5, var_qsch2_dn6, var_qsch2_dn7, var_qsch2_dn8, var_qsch2_dn9, var_qsch2_dn10, var_qsch2_dn11, var_qsch2_dn12, var_qsch2_dn13, var_qsch2_dn14, var_qsch2_dn15, var_qsch2_dn16, var_qsch2_dn17, var_qsch2_dn18, var_qsch2_dn19, var_qsch2_dn20, var_qsch2_dn21, var_qsch2_dn22, var_qsch2_dn23, var_qsch2_dn24, var_qsch2_dn25, var_qsch2_dn26, var_qsch2_dn27, var_qsch2_dn28, var_qsch2_dn29, var_qsch2_db0, var_qsch2_db1, var_qsch2_db2, var_qsch2_db3, var_qsch2_db4, var_qsch2_db5, var_qsch2_db6, var_qsch2_db7, var_qsch2_db8, var_qsch2_db9, var_qsch2_db10, var_qsch2_db11, var_qsch2_db12, var_qsch2_db13, var_qsch2_db14, var_qsch2_db15, var_qsch2_db16, var_qsch2_db17, var_qsch2_db18, var_qsch2_db19, var_qsch2_db20, var_qsch2_db21, var_qsch2_db22, var_qsch2_db23, var_qsch2_db24, var_qsch2_db25, var_qsch2_db26, var_qsch2_db27, var_qsch2_db28, var_qsch2_db29, var_qsch2_db30, var_qsch2_db31, var_qsch2_db32, var_qsch2_db33, var_qsch2_db34, var_qsch2_db35,)
    }
};
        var_qsch2 = assign43720_e42342;
        var_qsch2_dn0 = assign43720_e42342_d_n0;
        var_qsch2_dn1 = assign43720_e42342_d_n1;
        var_qsch2_dn2 = assign43720_e42342_d_n2;
        var_qsch2_dn3 = assign43720_e42342_d_n3;
        var_qsch2_dn4 = assign43720_e42342_d_n4;
        var_qsch2_dn5 = assign43720_e42342_d_n5;
        var_qsch2_dn6 = assign43720_e42342_d_n6;
        var_qsch2_dn7 = assign43720_e42342_d_n7;
        var_qsch2_dn8 = assign43720_e42342_d_n8;
        var_qsch2_dn9 = assign43720_e42342_d_n9;
        var_qsch2_dn10 = assign43720_e42342_d_n10;
        var_qsch2_dn11 = assign43720_e42342_d_n11;
        var_qsch2_dn12 = assign43720_e42342_d_n12;
        var_qsch2_dn13 = assign43720_e42342_d_n13;
        var_qsch2_dn14 = assign43720_e42342_d_n14;
        var_qsch2_dn15 = assign43720_e42342_d_n15;
        var_qsch2_dn16 = assign43720_e42342_d_n16;
        var_qsch2_dn17 = assign43720_e42342_d_n17;
        var_qsch2_dn18 = assign43720_e42342_d_n18;
        var_qsch2_dn19 = assign43720_e42342_d_n19;
        var_qsch2_dn20 = assign43720_e42342_d_n20;
        var_qsch2_dn21 = assign43720_e42342_d_n21;
        var_qsch2_dn22 = assign43720_e42342_d_n22;
        var_qsch2_dn23 = assign43720_e42342_d_n23;
        var_qsch2_dn24 = assign43720_e42342_d_n24;
        var_qsch2_dn25 = assign43720_e42342_d_n25;
        var_qsch2_dn26 = assign43720_e42342_d_n26;
        var_qsch2_dn27 = assign43720_e42342_d_n27;
        var_qsch2_dn28 = assign43720_e42342_d_n28;
        var_qsch2_dn29 = assign43720_e42342_d_n29;
        var_qsch2_db0 = assign43720_e42342_d_b0;
        var_qsch2_db1 = assign43720_e42342_d_b1;
        var_qsch2_db2 = assign43720_e42342_d_b2;
        var_qsch2_db3 = assign43720_e42342_d_b3;
        var_qsch2_db4 = assign43720_e42342_d_b4;
        var_qsch2_db5 = assign43720_e42342_d_b5;
        var_qsch2_db6 = assign43720_e42342_d_b6;
        var_qsch2_db7 = assign43720_e42342_d_b7;
        var_qsch2_db8 = assign43720_e42342_d_b8;
        var_qsch2_db9 = assign43720_e42342_d_b9;
        var_qsch2_db10 = assign43720_e42342_d_b10;
        var_qsch2_db11 = assign43720_e42342_d_b11;
        var_qsch2_db12 = assign43720_e42342_d_b12;
        var_qsch2_db13 = assign43720_e42342_d_b13;
        var_qsch2_db14 = assign43720_e42342_d_b14;
        var_qsch2_db15 = assign43720_e42342_d_b15;
        var_qsch2_db16 = assign43720_e42342_d_b16;
        var_qsch2_db17 = assign43720_e42342_d_b17;
        var_qsch2_db18 = assign43720_e42342_d_b18;
        var_qsch2_db19 = assign43720_e42342_d_b19;
        var_qsch2_db20 = assign43720_e42342_d_b20;
        var_qsch2_db21 = assign43720_e42342_d_b21;
        var_qsch2_db22 = assign43720_e42342_d_b22;
        var_qsch2_db23 = assign43720_e42342_d_b23;
        var_qsch2_db24 = assign43720_e42342_d_b24;
        var_qsch2_db25 = assign43720_e42342_d_b25;
        var_qsch2_db26 = assign43720_e42342_d_b26;
        var_qsch2_db27 = assign43720_e42342_d_b27;
        var_qsch2_db28 = assign43720_e42342_d_b28;
        var_qsch2_db29 = assign43720_e42342_d_b29;
        var_qsch2_db30 = assign43720_e42342_d_b30;
        var_qsch2_db31 = assign43720_e42342_d_b31;
        var_qsch2_db32 = assign43720_e42342_d_b32;
        var_qsch2_db33 = assign43720_e42342_d_b33;
        var_qsch2_db34 = assign43720_e42342_d_b34;
        var_qsch2_db35 = assign43720_e42342_d_b35;

        let assign43730_e42345: f64 = if p.p309 >= 3.0 { 1.0 } else { 0.0 };
        var_guard476 = assign43730_e42345;

        let (assign43740_e42366,) = {
    if (((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) {
        let assign43740_e42359: f64 = (2.0 * p.p306);
        let assign43740_e42362: f64 = (1.0 - p.p308);
        let assign43740_e42363: f64 = (assign43740_e42359 * assign43740_e42362);
        let assign43740_e42364: f64 = (var_qsch2c / assign43740_e42363);
        (assign43740_e42364,)
    } else {
        (var_qsch3c,)
    }
};
        var_qsch3c = assign43740_e42366;


        *var_guard474_slot = var_guard474;
        *var_guard475_slot = var_guard475;
        *var_guard476_slot = var_guard476;
        *var_qsch_slot = var_qsch;
        *var_qsch0_slot = var_qsch0;
        *var_qsch1_slot = var_qsch1;
        *var_qsch1_db0_slot = var_qsch1_db0;
        *var_qsch1_db1_slot = var_qsch1_db1;
        *var_qsch1_db10_slot = var_qsch1_db10;
        *var_qsch1_db11_slot = var_qsch1_db11;
        *var_qsch1_db12_slot = var_qsch1_db12;
        *var_qsch1_db13_slot = var_qsch1_db13;
        *var_qsch1_db14_slot = var_qsch1_db14;
        *var_qsch1_db15_slot = var_qsch1_db15;
        *var_qsch1_db16_slot = var_qsch1_db16;
        *var_qsch1_db17_slot = var_qsch1_db17;
        *var_qsch1_db18_slot = var_qsch1_db18;
        *var_qsch1_db19_slot = var_qsch1_db19;
        *var_qsch1_db2_slot = var_qsch1_db2;
        *var_qsch1_db20_slot = var_qsch1_db20;
        *var_qsch1_db21_slot = var_qsch1_db21;
        *var_qsch1_db22_slot = var_qsch1_db22;
        *var_qsch1_db23_slot = var_qsch1_db23;
        *var_qsch1_db24_slot = var_qsch1_db24;
        *var_qsch1_db25_slot = var_qsch1_db25;
        *var_qsch1_db26_slot = var_qsch1_db26;
        *var_qsch1_db27_slot = var_qsch1_db27;
        *var_qsch1_db28_slot = var_qsch1_db28;
        *var_qsch1_db29_slot = var_qsch1_db29;
        *var_qsch1_db3_slot = var_qsch1_db3;
        *var_qsch1_db30_slot = var_qsch1_db30;
        *var_qsch1_db31_slot = var_qsch1_db31;
        *var_qsch1_db32_slot = var_qsch1_db32;
        *var_qsch1_db33_slot = var_qsch1_db33;
        *var_qsch1_db34_slot = var_qsch1_db34;
        *var_qsch1_db35_slot = var_qsch1_db35;
        *var_qsch1_db4_slot = var_qsch1_db4;
        *var_qsch1_db5_slot = var_qsch1_db5;
        *var_qsch1_db6_slot = var_qsch1_db6;
        *var_qsch1_db7_slot = var_qsch1_db7;
        *var_qsch1_db8_slot = var_qsch1_db8;
        *var_qsch1_db9_slot = var_qsch1_db9;
        *var_qsch1_dn0_slot = var_qsch1_dn0;
        *var_qsch1_dn1_slot = var_qsch1_dn1;
        *var_qsch1_dn10_slot = var_qsch1_dn10;
        *var_qsch1_dn11_slot = var_qsch1_dn11;
        *var_qsch1_dn12_slot = var_qsch1_dn12;
        *var_qsch1_dn13_slot = var_qsch1_dn13;
        *var_qsch1_dn14_slot = var_qsch1_dn14;
        *var_qsch1_dn15_slot = var_qsch1_dn15;
        *var_qsch1_dn16_slot = var_qsch1_dn16;
        *var_qsch1_dn17_slot = var_qsch1_dn17;
        *var_qsch1_dn18_slot = var_qsch1_dn18;
        *var_qsch1_dn19_slot = var_qsch1_dn19;
        *var_qsch1_dn2_slot = var_qsch1_dn2;
        *var_qsch1_dn20_slot = var_qsch1_dn20;
        *var_qsch1_dn21_slot = var_qsch1_dn21;
        *var_qsch1_dn22_slot = var_qsch1_dn22;
        *var_qsch1_dn23_slot = var_qsch1_dn23;
        *var_qsch1_dn24_slot = var_qsch1_dn24;
        *var_qsch1_dn25_slot = var_qsch1_dn25;
        *var_qsch1_dn26_slot = var_qsch1_dn26;
        *var_qsch1_dn27_slot = var_qsch1_dn27;
        *var_qsch1_dn28_slot = var_qsch1_dn28;
        *var_qsch1_dn29_slot = var_qsch1_dn29;
        *var_qsch1_dn3_slot = var_qsch1_dn3;
        *var_qsch1_dn4_slot = var_qsch1_dn4;
        *var_qsch1_dn5_slot = var_qsch1_dn5;
        *var_qsch1_dn6_slot = var_qsch1_dn6;
        *var_qsch1_dn7_slot = var_qsch1_dn7;
        *var_qsch1_dn8_slot = var_qsch1_dn8;
        *var_qsch1_dn9_slot = var_qsch1_dn9;
        *var_qsch1c_slot = var_qsch1c;
        *var_qsch2_slot = var_qsch2;
        *var_qsch2_db0_slot = var_qsch2_db0;
        *var_qsch2_db1_slot = var_qsch2_db1;
        *var_qsch2_db10_slot = var_qsch2_db10;
        *var_qsch2_db11_slot = var_qsch2_db11;
        *var_qsch2_db12_slot = var_qsch2_db12;
        *var_qsch2_db13_slot = var_qsch2_db13;
        *var_qsch2_db14_slot = var_qsch2_db14;
        *var_qsch2_db15_slot = var_qsch2_db15;
        *var_qsch2_db16_slot = var_qsch2_db16;
        *var_qsch2_db17_slot = var_qsch2_db17;
        *var_qsch2_db18_slot = var_qsch2_db18;
        *var_qsch2_db19_slot = var_qsch2_db19;
        *var_qsch2_db2_slot = var_qsch2_db2;
        *var_qsch2_db20_slot = var_qsch2_db20;
        *var_qsch2_db21_slot = var_qsch2_db21;
        *var_qsch2_db22_slot = var_qsch2_db22;
        *var_qsch2_db23_slot = var_qsch2_db23;
        *var_qsch2_db24_slot = var_qsch2_db24;
        *var_qsch2_db25_slot = var_qsch2_db25;
        *var_qsch2_db26_slot = var_qsch2_db26;
        *var_qsch2_db27_slot = var_qsch2_db27;
        *var_qsch2_db28_slot = var_qsch2_db28;
        *var_qsch2_db29_slot = var_qsch2_db29;
        *var_qsch2_db3_slot = var_qsch2_db3;
        *var_qsch2_db30_slot = var_qsch2_db30;
        *var_qsch2_db31_slot = var_qsch2_db31;
        *var_qsch2_db32_slot = var_qsch2_db32;
        *var_qsch2_db33_slot = var_qsch2_db33;
        *var_qsch2_db34_slot = var_qsch2_db34;
        *var_qsch2_db35_slot = var_qsch2_db35;
        *var_qsch2_db4_slot = var_qsch2_db4;
        *var_qsch2_db5_slot = var_qsch2_db5;
        *var_qsch2_db6_slot = var_qsch2_db6;
        *var_qsch2_db7_slot = var_qsch2_db7;
        *var_qsch2_db8_slot = var_qsch2_db8;
        *var_qsch2_db9_slot = var_qsch2_db9;
        *var_qsch2_dn0_slot = var_qsch2_dn0;
        *var_qsch2_dn1_slot = var_qsch2_dn1;
        *var_qsch2_dn10_slot = var_qsch2_dn10;
        *var_qsch2_dn11_slot = var_qsch2_dn11;
        *var_qsch2_dn12_slot = var_qsch2_dn12;
        *var_qsch2_dn13_slot = var_qsch2_dn13;
        *var_qsch2_dn14_slot = var_qsch2_dn14;
        *var_qsch2_dn15_slot = var_qsch2_dn15;
        *var_qsch2_dn16_slot = var_qsch2_dn16;
        *var_qsch2_dn17_slot = var_qsch2_dn17;
        *var_qsch2_dn18_slot = var_qsch2_dn18;
        *var_qsch2_dn19_slot = var_qsch2_dn19;
        *var_qsch2_dn2_slot = var_qsch2_dn2;
        *var_qsch2_dn20_slot = var_qsch2_dn20;
        *var_qsch2_dn21_slot = var_qsch2_dn21;
        *var_qsch2_dn22_slot = var_qsch2_dn22;
        *var_qsch2_dn23_slot = var_qsch2_dn23;
        *var_qsch2_dn24_slot = var_qsch2_dn24;
        *var_qsch2_dn25_slot = var_qsch2_dn25;
        *var_qsch2_dn26_slot = var_qsch2_dn26;
        *var_qsch2_dn27_slot = var_qsch2_dn27;
        *var_qsch2_dn28_slot = var_qsch2_dn28;
        *var_qsch2_dn29_slot = var_qsch2_dn29;
        *var_qsch2_dn3_slot = var_qsch2_dn3;
        *var_qsch2_dn4_slot = var_qsch2_dn4;
        *var_qsch2_dn5_slot = var_qsch2_dn5;
        *var_qsch2_dn6_slot = var_qsch2_dn6;
        *var_qsch2_dn7_slot = var_qsch2_dn7;
        *var_qsch2_dn8_slot = var_qsch2_dn8;
        *var_qsch2_dn9_slot = var_qsch2_dn9;
        *var_qsch2c_slot = var_qsch2c;
        *var_qsch3c_slot = var_qsch3c;
        *var_qsch_db0_slot = var_qsch_db0;
        *var_qsch_db1_slot = var_qsch_db1;
        *var_qsch_db10_slot = var_qsch_db10;
        *var_qsch_db11_slot = var_qsch_db11;
        *var_qsch_db12_slot = var_qsch_db12;
        *var_qsch_db13_slot = var_qsch_db13;
        *var_qsch_db14_slot = var_qsch_db14;
        *var_qsch_db15_slot = var_qsch_db15;
        *var_qsch_db16_slot = var_qsch_db16;
        *var_qsch_db17_slot = var_qsch_db17;
        *var_qsch_db18_slot = var_qsch_db18;
        *var_qsch_db19_slot = var_qsch_db19;
        *var_qsch_db2_slot = var_qsch_db2;
        *var_qsch_db20_slot = var_qsch_db20;
        *var_qsch_db21_slot = var_qsch_db21;
        *var_qsch_db22_slot = var_qsch_db22;
        *var_qsch_db23_slot = var_qsch_db23;
        *var_qsch_db24_slot = var_qsch_db24;
        *var_qsch_db25_slot = var_qsch_db25;
        *var_qsch_db26_slot = var_qsch_db26;
        *var_qsch_db27_slot = var_qsch_db27;
        *var_qsch_db28_slot = var_qsch_db28;
        *var_qsch_db29_slot = var_qsch_db29;
        *var_qsch_db3_slot = var_qsch_db3;
        *var_qsch_db30_slot = var_qsch_db30;
        *var_qsch_db31_slot = var_qsch_db31;
        *var_qsch_db32_slot = var_qsch_db32;
        *var_qsch_db33_slot = var_qsch_db33;
        *var_qsch_db34_slot = var_qsch_db34;
        *var_qsch_db35_slot = var_qsch_db35;
        *var_qsch_db4_slot = var_qsch_db4;
        *var_qsch_db5_slot = var_qsch_db5;
        *var_qsch_db6_slot = var_qsch_db6;
        *var_qsch_db7_slot = var_qsch_db7;
        *var_qsch_db8_slot = var_qsch_db8;
        *var_qsch_db9_slot = var_qsch_db9;
        *var_qsch_dn0_slot = var_qsch_dn0;
        *var_qsch_dn1_slot = var_qsch_dn1;
        *var_qsch_dn10_slot = var_qsch_dn10;
        *var_qsch_dn11_slot = var_qsch_dn11;
        *var_qsch_dn12_slot = var_qsch_dn12;
        *var_qsch_dn13_slot = var_qsch_dn13;
        *var_qsch_dn14_slot = var_qsch_dn14;
        *var_qsch_dn15_slot = var_qsch_dn15;
        *var_qsch_dn16_slot = var_qsch_dn16;
        *var_qsch_dn17_slot = var_qsch_dn17;
        *var_qsch_dn18_slot = var_qsch_dn18;
        *var_qsch_dn19_slot = var_qsch_dn19;
        *var_qsch_dn2_slot = var_qsch_dn2;
        *var_qsch_dn20_slot = var_qsch_dn20;
        *var_qsch_dn21_slot = var_qsch_dn21;
        *var_qsch_dn22_slot = var_qsch_dn22;
        *var_qsch_dn23_slot = var_qsch_dn23;
        *var_qsch_dn24_slot = var_qsch_dn24;
        *var_qsch_dn25_slot = var_qsch_dn25;
        *var_qsch_dn26_slot = var_qsch_dn26;
        *var_qsch_dn27_slot = var_qsch_dn27;
        *var_qsch_dn28_slot = var_qsch_dn28;
        *var_qsch_dn29_slot = var_qsch_dn29;
        *var_qsch_dn3_slot = var_qsch_dn3;
        *var_qsch_dn4_slot = var_qsch_dn4;
        *var_qsch_dn5_slot = var_qsch_dn5;
        *var_qsch_dn6_slot = var_qsch_dn6;
        *var_qsch_dn7_slot = var_qsch_dn7;
        *var_qsch_dn8_slot = var_qsch_dn8;
        *var_qsch_dn9_slot = var_qsch_dn9;
        *var_vschfc1_slot = var_vschfc1;
        *var_vschfc1_db0_slot = var_vschfc1_db0;
        *var_vschfc1_db1_slot = var_vschfc1_db1;
        *var_vschfc1_db10_slot = var_vschfc1_db10;
        *var_vschfc1_db11_slot = var_vschfc1_db11;
        *var_vschfc1_db12_slot = var_vschfc1_db12;
        *var_vschfc1_db13_slot = var_vschfc1_db13;
        *var_vschfc1_db14_slot = var_vschfc1_db14;
        *var_vschfc1_db15_slot = var_vschfc1_db15;
        *var_vschfc1_db16_slot = var_vschfc1_db16;
        *var_vschfc1_db17_slot = var_vschfc1_db17;
        *var_vschfc1_db18_slot = var_vschfc1_db18;
        *var_vschfc1_db19_slot = var_vschfc1_db19;
        *var_vschfc1_db2_slot = var_vschfc1_db2;
        *var_vschfc1_db20_slot = var_vschfc1_db20;
        *var_vschfc1_db21_slot = var_vschfc1_db21;
        *var_vschfc1_db22_slot = var_vschfc1_db22;
        *var_vschfc1_db23_slot = var_vschfc1_db23;
        *var_vschfc1_db24_slot = var_vschfc1_db24;
        *var_vschfc1_db25_slot = var_vschfc1_db25;
        *var_vschfc1_db26_slot = var_vschfc1_db26;
        *var_vschfc1_db27_slot = var_vschfc1_db27;
        *var_vschfc1_db28_slot = var_vschfc1_db28;
        *var_vschfc1_db29_slot = var_vschfc1_db29;
        *var_vschfc1_db3_slot = var_vschfc1_db3;
        *var_vschfc1_db30_slot = var_vschfc1_db30;
        *var_vschfc1_db31_slot = var_vschfc1_db31;
        *var_vschfc1_db32_slot = var_vschfc1_db32;
        *var_vschfc1_db33_slot = var_vschfc1_db33;
        *var_vschfc1_db34_slot = var_vschfc1_db34;
        *var_vschfc1_db35_slot = var_vschfc1_db35;
        *var_vschfc1_db4_slot = var_vschfc1_db4;
        *var_vschfc1_db5_slot = var_vschfc1_db5;
        *var_vschfc1_db6_slot = var_vschfc1_db6;
        *var_vschfc1_db7_slot = var_vschfc1_db7;
        *var_vschfc1_db8_slot = var_vschfc1_db8;
        *var_vschfc1_db9_slot = var_vschfc1_db9;
        *var_vschfc1_dn0_slot = var_vschfc1_dn0;
        *var_vschfc1_dn1_slot = var_vschfc1_dn1;
        *var_vschfc1_dn10_slot = var_vschfc1_dn10;
        *var_vschfc1_dn11_slot = var_vschfc1_dn11;
        *var_vschfc1_dn12_slot = var_vschfc1_dn12;
        *var_vschfc1_dn13_slot = var_vschfc1_dn13;
        *var_vschfc1_dn14_slot = var_vschfc1_dn14;
        *var_vschfc1_dn15_slot = var_vschfc1_dn15;
        *var_vschfc1_dn16_slot = var_vschfc1_dn16;
        *var_vschfc1_dn17_slot = var_vschfc1_dn17;
        *var_vschfc1_dn18_slot = var_vschfc1_dn18;
        *var_vschfc1_dn19_slot = var_vschfc1_dn19;
        *var_vschfc1_dn2_slot = var_vschfc1_dn2;
        *var_vschfc1_dn20_slot = var_vschfc1_dn20;
        *var_vschfc1_dn21_slot = var_vschfc1_dn21;
        *var_vschfc1_dn22_slot = var_vschfc1_dn22;
        *var_vschfc1_dn23_slot = var_vschfc1_dn23;
        *var_vschfc1_dn24_slot = var_vschfc1_dn24;
        *var_vschfc1_dn25_slot = var_vschfc1_dn25;
        *var_vschfc1_dn26_slot = var_vschfc1_dn26;
        *var_vschfc1_dn27_slot = var_vschfc1_dn27;
        *var_vschfc1_dn28_slot = var_vschfc1_dn28;
        *var_vschfc1_dn29_slot = var_vschfc1_dn29;
        *var_vschfc1_dn3_slot = var_vschfc1_dn3;
        *var_vschfc1_dn4_slot = var_vschfc1_dn4;
        *var_vschfc1_dn5_slot = var_vschfc1_dn5;
        *var_vschfc1_dn6_slot = var_vschfc1_dn6;
        *var_vschfc1_dn7_slot = var_vschfc1_dn7;
        *var_vschfc1_dn8_slot = var_vschfc1_dn8;
        *var_vschfc1_dn9_slot = var_vschfc1_dn9;
        *var_vschfc2_slot = var_vschfc2;
        *var_vschfc2_db0_slot = var_vschfc2_db0;
        *var_vschfc2_db1_slot = var_vschfc2_db1;
        *var_vschfc2_db10_slot = var_vschfc2_db10;
        *var_vschfc2_db11_slot = var_vschfc2_db11;
        *var_vschfc2_db12_slot = var_vschfc2_db12;
        *var_vschfc2_db13_slot = var_vschfc2_db13;
        *var_vschfc2_db14_slot = var_vschfc2_db14;
        *var_vschfc2_db15_slot = var_vschfc2_db15;
        *var_vschfc2_db16_slot = var_vschfc2_db16;
        *var_vschfc2_db17_slot = var_vschfc2_db17;
        *var_vschfc2_db18_slot = var_vschfc2_db18;
        *var_vschfc2_db19_slot = var_vschfc2_db19;
        *var_vschfc2_db2_slot = var_vschfc2_db2;
        *var_vschfc2_db20_slot = var_vschfc2_db20;
        *var_vschfc2_db21_slot = var_vschfc2_db21;
        *var_vschfc2_db22_slot = var_vschfc2_db22;
        *var_vschfc2_db23_slot = var_vschfc2_db23;
        *var_vschfc2_db24_slot = var_vschfc2_db24;
        *var_vschfc2_db25_slot = var_vschfc2_db25;
        *var_vschfc2_db26_slot = var_vschfc2_db26;
        *var_vschfc2_db27_slot = var_vschfc2_db27;
        *var_vschfc2_db28_slot = var_vschfc2_db28;
        *var_vschfc2_db29_slot = var_vschfc2_db29;
        *var_vschfc2_db3_slot = var_vschfc2_db3;
        *var_vschfc2_db30_slot = var_vschfc2_db30;
        *var_vschfc2_db31_slot = var_vschfc2_db31;
        *var_vschfc2_db32_slot = var_vschfc2_db32;
        *var_vschfc2_db33_slot = var_vschfc2_db33;
        *var_vschfc2_db34_slot = var_vschfc2_db34;
        *var_vschfc2_db35_slot = var_vschfc2_db35;
        *var_vschfc2_db4_slot = var_vschfc2_db4;
        *var_vschfc2_db5_slot = var_vschfc2_db5;
        *var_vschfc2_db6_slot = var_vschfc2_db6;
        *var_vschfc2_db7_slot = var_vschfc2_db7;
        *var_vschfc2_db8_slot = var_vschfc2_db8;
        *var_vschfc2_db9_slot = var_vschfc2_db9;
        *var_vschfc2_dn0_slot = var_vschfc2_dn0;
        *var_vschfc2_dn1_slot = var_vschfc2_dn1;
        *var_vschfc2_dn10_slot = var_vschfc2_dn10;
        *var_vschfc2_dn11_slot = var_vschfc2_dn11;
        *var_vschfc2_dn12_slot = var_vschfc2_dn12;
        *var_vschfc2_dn13_slot = var_vschfc2_dn13;
        *var_vschfc2_dn14_slot = var_vschfc2_dn14;
        *var_vschfc2_dn15_slot = var_vschfc2_dn15;
        *var_vschfc2_dn16_slot = var_vschfc2_dn16;
        *var_vschfc2_dn17_slot = var_vschfc2_dn17;
        *var_vschfc2_dn18_slot = var_vschfc2_dn18;
        *var_vschfc2_dn19_slot = var_vschfc2_dn19;
        *var_vschfc2_dn2_slot = var_vschfc2_dn2;
        *var_vschfc2_dn20_slot = var_vschfc2_dn20;
        *var_vschfc2_dn21_slot = var_vschfc2_dn21;
        *var_vschfc2_dn22_slot = var_vschfc2_dn22;
        *var_vschfc2_dn23_slot = var_vschfc2_dn23;
        *var_vschfc2_dn24_slot = var_vschfc2_dn24;
        *var_vschfc2_dn25_slot = var_vschfc2_dn25;
        *var_vschfc2_dn26_slot = var_vschfc2_dn26;
        *var_vschfc2_dn27_slot = var_vschfc2_dn27;
        *var_vschfc2_dn28_slot = var_vschfc2_dn28;
        *var_vschfc2_dn29_slot = var_vschfc2_dn29;
        *var_vschfc2_dn3_slot = var_vschfc2_dn3;
        *var_vschfc2_dn4_slot = var_vschfc2_dn4;
        *var_vschfc2_dn5_slot = var_vschfc2_dn5;
        *var_vschfc2_dn6_slot = var_vschfc2_dn6;
        *var_vschfc2_dn7_slot = var_vschfc2_dn7;
        *var_vschfc2_dn8_slot = var_vschfc2_dn8;
        *var_vschfc2_dn9_slot = var_vschfc2_dn9;
    }

    pub(super) fn stamp_transient_block_116(
        p: &Parameters,
        var_guard461: f64,
        var_guard473: f64,
        var_guard474: f64,
        var_guard475: f64,
        var_guard476: f64,
        var_qsch3c: f64,
        var_vschfc1: f64,
        var_vschfc1_db0: f64,
        var_vschfc1_db1: f64,
        var_vschfc1_db10: f64,
        var_vschfc1_db11: f64,
        var_vschfc1_db12: f64,
        var_vschfc1_db13: f64,
        var_vschfc1_db14: f64,
        var_vschfc1_db15: f64,
        var_vschfc1_db16: f64,
        var_vschfc1_db17: f64,
        var_vschfc1_db18: f64,
        var_vschfc1_db19: f64,
        var_vschfc1_db2: f64,
        var_vschfc1_db20: f64,
        var_vschfc1_db21: f64,
        var_vschfc1_db22: f64,
        var_vschfc1_db23: f64,
        var_vschfc1_db24: f64,
        var_vschfc1_db25: f64,
        var_vschfc1_db26: f64,
        var_vschfc1_db27: f64,
        var_vschfc1_db28: f64,
        var_vschfc1_db29: f64,
        var_vschfc1_db3: f64,
        var_vschfc1_db30: f64,
        var_vschfc1_db31: f64,
        var_vschfc1_db32: f64,
        var_vschfc1_db33: f64,
        var_vschfc1_db34: f64,
        var_vschfc1_db35: f64,
        var_vschfc1_db4: f64,
        var_vschfc1_db5: f64,
        var_vschfc1_db6: f64,
        var_vschfc1_db7: f64,
        var_vschfc1_db8: f64,
        var_vschfc1_db9: f64,
        var_vschfc1_dn0: f64,
        var_vschfc1_dn1: f64,
        var_vschfc1_dn10: f64,
        var_vschfc1_dn11: f64,
        var_vschfc1_dn12: f64,
        var_vschfc1_dn13: f64,
        var_vschfc1_dn14: f64,
        var_vschfc1_dn15: f64,
        var_vschfc1_dn16: f64,
        var_vschfc1_dn17: f64,
        var_vschfc1_dn18: f64,
        var_vschfc1_dn19: f64,
        var_vschfc1_dn2: f64,
        var_vschfc1_dn20: f64,
        var_vschfc1_dn21: f64,
        var_vschfc1_dn22: f64,
        var_vschfc1_dn23: f64,
        var_vschfc1_dn24: f64,
        var_vschfc1_dn25: f64,
        var_vschfc1_dn26: f64,
        var_vschfc1_dn27: f64,
        var_vschfc1_dn28: f64,
        var_vschfc1_dn29: f64,
        var_vschfc1_dn3: f64,
        var_vschfc1_dn4: f64,
        var_vschfc1_dn5: f64,
        var_vschfc1_dn6: f64,
        var_vschfc1_dn7: f64,
        var_vschfc1_dn8: f64,
        var_vschfc1_dn9: f64,
        var_vschfc2: f64,
        var_vschfc2_db0: f64,
        var_vschfc2_db1: f64,
        var_vschfc2_db10: f64,
        var_vschfc2_db11: f64,
        var_vschfc2_db12: f64,
        var_vschfc2_db13: f64,
        var_vschfc2_db14: f64,
        var_vschfc2_db15: f64,
        var_vschfc2_db16: f64,
        var_vschfc2_db17: f64,
        var_vschfc2_db18: f64,
        var_vschfc2_db19: f64,
        var_vschfc2_db2: f64,
        var_vschfc2_db20: f64,
        var_vschfc2_db21: f64,
        var_vschfc2_db22: f64,
        var_vschfc2_db23: f64,
        var_vschfc2_db24: f64,
        var_vschfc2_db25: f64,
        var_vschfc2_db26: f64,
        var_vschfc2_db27: f64,
        var_vschfc2_db28: f64,
        var_vschfc2_db29: f64,
        var_vschfc2_db3: f64,
        var_vschfc2_db30: f64,
        var_vschfc2_db31: f64,
        var_vschfc2_db32: f64,
        var_vschfc2_db33: f64,
        var_vschfc2_db34: f64,
        var_vschfc2_db35: f64,
        var_vschfc2_db4: f64,
        var_vschfc2_db5: f64,
        var_vschfc2_db6: f64,
        var_vschfc2_db7: f64,
        var_vschfc2_db8: f64,
        var_vschfc2_db9: f64,
        var_vschfc2_dn0: f64,
        var_vschfc2_dn1: f64,
        var_vschfc2_dn10: f64,
        var_vschfc2_dn11: f64,
        var_vschfc2_dn12: f64,
        var_vschfc2_dn13: f64,
        var_vschfc2_dn14: f64,
        var_vschfc2_dn15: f64,
        var_vschfc2_dn16: f64,
        var_vschfc2_dn17: f64,
        var_vschfc2_dn18: f64,
        var_vschfc2_dn19: f64,
        var_vschfc2_dn2: f64,
        var_vschfc2_dn20: f64,
        var_vschfc2_dn21: f64,
        var_vschfc2_dn22: f64,
        var_vschfc2_dn23: f64,
        var_vschfc2_dn24: f64,
        var_vschfc2_dn25: f64,
        var_vschfc2_dn26: f64,
        var_vschfc2_dn27: f64,
        var_vschfc2_dn28: f64,
        var_vschfc2_dn29: f64,
        var_vschfc2_dn3: f64,
        var_vschfc2_dn4: f64,
        var_vschfc2_dn5: f64,
        var_vschfc2_dn6: f64,
        var_vschfc2_dn7: f64,
        var_vschfc2_dn8: f64,
        var_vschfc2_dn9: f64,
        var_guard477_slot: &mut f64,
        var_guard478_slot: &mut f64,
        var_qsch3_slot: &mut f64,
        var_qsch3_db0_slot: &mut f64,
        var_qsch3_db1_slot: &mut f64,
        var_qsch3_db10_slot: &mut f64,
        var_qsch3_db11_slot: &mut f64,
        var_qsch3_db12_slot: &mut f64,
        var_qsch3_db13_slot: &mut f64,
        var_qsch3_db14_slot: &mut f64,
        var_qsch3_db15_slot: &mut f64,
        var_qsch3_db16_slot: &mut f64,
        var_qsch3_db17_slot: &mut f64,
        var_qsch3_db18_slot: &mut f64,
        var_qsch3_db19_slot: &mut f64,
        var_qsch3_db2_slot: &mut f64,
        var_qsch3_db20_slot: &mut f64,
        var_qsch3_db21_slot: &mut f64,
        var_qsch3_db22_slot: &mut f64,
        var_qsch3_db23_slot: &mut f64,
        var_qsch3_db24_slot: &mut f64,
        var_qsch3_db25_slot: &mut f64,
        var_qsch3_db26_slot: &mut f64,
        var_qsch3_db27_slot: &mut f64,
        var_qsch3_db28_slot: &mut f64,
        var_qsch3_db29_slot: &mut f64,
        var_qsch3_db3_slot: &mut f64,
        var_qsch3_db30_slot: &mut f64,
        var_qsch3_db31_slot: &mut f64,
        var_qsch3_db32_slot: &mut f64,
        var_qsch3_db33_slot: &mut f64,
        var_qsch3_db34_slot: &mut f64,
        var_qsch3_db35_slot: &mut f64,
        var_qsch3_db4_slot: &mut f64,
        var_qsch3_db5_slot: &mut f64,
        var_qsch3_db6_slot: &mut f64,
        var_qsch3_db7_slot: &mut f64,
        var_qsch3_db8_slot: &mut f64,
        var_qsch3_db9_slot: &mut f64,
        var_qsch3_dn0_slot: &mut f64,
        var_qsch3_dn1_slot: &mut f64,
        var_qsch3_dn10_slot: &mut f64,
        var_qsch3_dn11_slot: &mut f64,
        var_qsch3_dn12_slot: &mut f64,
        var_qsch3_dn13_slot: &mut f64,
        var_qsch3_dn14_slot: &mut f64,
        var_qsch3_dn15_slot: &mut f64,
        var_qsch3_dn16_slot: &mut f64,
        var_qsch3_dn17_slot: &mut f64,
        var_qsch3_dn18_slot: &mut f64,
        var_qsch3_dn19_slot: &mut f64,
        var_qsch3_dn2_slot: &mut f64,
        var_qsch3_dn20_slot: &mut f64,
        var_qsch3_dn21_slot: &mut f64,
        var_qsch3_dn22_slot: &mut f64,
        var_qsch3_dn23_slot: &mut f64,
        var_qsch3_dn24_slot: &mut f64,
        var_qsch3_dn25_slot: &mut f64,
        var_qsch3_dn26_slot: &mut f64,
        var_qsch3_dn27_slot: &mut f64,
        var_qsch3_dn28_slot: &mut f64,
        var_qsch3_dn29_slot: &mut f64,
        var_qsch3_dn3_slot: &mut f64,
        var_qsch3_dn4_slot: &mut f64,
        var_qsch3_dn5_slot: &mut f64,
        var_qsch3_dn6_slot: &mut f64,
        var_qsch3_dn7_slot: &mut f64,
        var_qsch3_dn8_slot: &mut f64,
        var_qsch3_dn9_slot: &mut f64,
        var_qsch4_slot: &mut f64,
        var_qsch4_db0_slot: &mut f64,
        var_qsch4_db1_slot: &mut f64,
        var_qsch4_db10_slot: &mut f64,
        var_qsch4_db11_slot: &mut f64,
        var_qsch4_db12_slot: &mut f64,
        var_qsch4_db13_slot: &mut f64,
        var_qsch4_db14_slot: &mut f64,
        var_qsch4_db15_slot: &mut f64,
        var_qsch4_db16_slot: &mut f64,
        var_qsch4_db17_slot: &mut f64,
        var_qsch4_db18_slot: &mut f64,
        var_qsch4_db19_slot: &mut f64,
        var_qsch4_db2_slot: &mut f64,
        var_qsch4_db20_slot: &mut f64,
        var_qsch4_db21_slot: &mut f64,
        var_qsch4_db22_slot: &mut f64,
        var_qsch4_db23_slot: &mut f64,
        var_qsch4_db24_slot: &mut f64,
        var_qsch4_db25_slot: &mut f64,
        var_qsch4_db26_slot: &mut f64,
        var_qsch4_db27_slot: &mut f64,
        var_qsch4_db28_slot: &mut f64,
        var_qsch4_db29_slot: &mut f64,
        var_qsch4_db3_slot: &mut f64,
        var_qsch4_db30_slot: &mut f64,
        var_qsch4_db31_slot: &mut f64,
        var_qsch4_db32_slot: &mut f64,
        var_qsch4_db33_slot: &mut f64,
        var_qsch4_db34_slot: &mut f64,
        var_qsch4_db35_slot: &mut f64,
        var_qsch4_db4_slot: &mut f64,
        var_qsch4_db5_slot: &mut f64,
        var_qsch4_db6_slot: &mut f64,
        var_qsch4_db7_slot: &mut f64,
        var_qsch4_db8_slot: &mut f64,
        var_qsch4_db9_slot: &mut f64,
        var_qsch4_dn0_slot: &mut f64,
        var_qsch4_dn1_slot: &mut f64,
        var_qsch4_dn10_slot: &mut f64,
        var_qsch4_dn11_slot: &mut f64,
        var_qsch4_dn12_slot: &mut f64,
        var_qsch4_dn13_slot: &mut f64,
        var_qsch4_dn14_slot: &mut f64,
        var_qsch4_dn15_slot: &mut f64,
        var_qsch4_dn16_slot: &mut f64,
        var_qsch4_dn17_slot: &mut f64,
        var_qsch4_dn18_slot: &mut f64,
        var_qsch4_dn19_slot: &mut f64,
        var_qsch4_dn2_slot: &mut f64,
        var_qsch4_dn20_slot: &mut f64,
        var_qsch4_dn21_slot: &mut f64,
        var_qsch4_dn22_slot: &mut f64,
        var_qsch4_dn23_slot: &mut f64,
        var_qsch4_dn24_slot: &mut f64,
        var_qsch4_dn25_slot: &mut f64,
        var_qsch4_dn26_slot: &mut f64,
        var_qsch4_dn27_slot: &mut f64,
        var_qsch4_dn28_slot: &mut f64,
        var_qsch4_dn29_slot: &mut f64,
        var_qsch4_dn3_slot: &mut f64,
        var_qsch4_dn4_slot: &mut f64,
        var_qsch4_dn5_slot: &mut f64,
        var_qsch4_dn6_slot: &mut f64,
        var_qsch4_dn7_slot: &mut f64,
        var_qsch4_dn8_slot: &mut f64,
        var_qsch4_dn9_slot: &mut f64,
        var_qsch4c_slot: &mut f64,
        var_qsch5_slot: &mut f64,
        var_qsch5_db0_slot: &mut f64,
        var_qsch5_db1_slot: &mut f64,
        var_qsch5_db10_slot: &mut f64,
        var_qsch5_db11_slot: &mut f64,
        var_qsch5_db12_slot: &mut f64,
        var_qsch5_db13_slot: &mut f64,
        var_qsch5_db14_slot: &mut f64,
        var_qsch5_db15_slot: &mut f64,
        var_qsch5_db16_slot: &mut f64,
        var_qsch5_db17_slot: &mut f64,
        var_qsch5_db18_slot: &mut f64,
        var_qsch5_db19_slot: &mut f64,
        var_qsch5_db2_slot: &mut f64,
        var_qsch5_db20_slot: &mut f64,
        var_qsch5_db21_slot: &mut f64,
        var_qsch5_db22_slot: &mut f64,
        var_qsch5_db23_slot: &mut f64,
        var_qsch5_db24_slot: &mut f64,
        var_qsch5_db25_slot: &mut f64,
        var_qsch5_db26_slot: &mut f64,
        var_qsch5_db27_slot: &mut f64,
        var_qsch5_db28_slot: &mut f64,
        var_qsch5_db29_slot: &mut f64,
        var_qsch5_db3_slot: &mut f64,
        var_qsch5_db30_slot: &mut f64,
        var_qsch5_db31_slot: &mut f64,
        var_qsch5_db32_slot: &mut f64,
        var_qsch5_db33_slot: &mut f64,
        var_qsch5_db34_slot: &mut f64,
        var_qsch5_db35_slot: &mut f64,
        var_qsch5_db4_slot: &mut f64,
        var_qsch5_db5_slot: &mut f64,
        var_qsch5_db6_slot: &mut f64,
        var_qsch5_db7_slot: &mut f64,
        var_qsch5_db8_slot: &mut f64,
        var_qsch5_db9_slot: &mut f64,
        var_qsch5_dn0_slot: &mut f64,
        var_qsch5_dn1_slot: &mut f64,
        var_qsch5_dn10_slot: &mut f64,
        var_qsch5_dn11_slot: &mut f64,
        var_qsch5_dn12_slot: &mut f64,
        var_qsch5_dn13_slot: &mut f64,
        var_qsch5_dn14_slot: &mut f64,
        var_qsch5_dn15_slot: &mut f64,
        var_qsch5_dn16_slot: &mut f64,
        var_qsch5_dn17_slot: &mut f64,
        var_qsch5_dn18_slot: &mut f64,
        var_qsch5_dn19_slot: &mut f64,
        var_qsch5_dn2_slot: &mut f64,
        var_qsch5_dn20_slot: &mut f64,
        var_qsch5_dn21_slot: &mut f64,
        var_qsch5_dn22_slot: &mut f64,
        var_qsch5_dn23_slot: &mut f64,
        var_qsch5_dn24_slot: &mut f64,
        var_qsch5_dn25_slot: &mut f64,
        var_qsch5_dn26_slot: &mut f64,
        var_qsch5_dn27_slot: &mut f64,
        var_qsch5_dn28_slot: &mut f64,
        var_qsch5_dn29_slot: &mut f64,
        var_qsch5_dn3_slot: &mut f64,
        var_qsch5_dn4_slot: &mut f64,
        var_qsch5_dn5_slot: &mut f64,
        var_qsch5_dn6_slot: &mut f64,
        var_qsch5_dn7_slot: &mut f64,
        var_qsch5_dn8_slot: &mut f64,
        var_qsch5_dn9_slot: &mut f64,
        var_qsch5c_slot: &mut f64,
        var_vschfc3_slot: &mut f64,
        var_vschfc3_db0_slot: &mut f64,
        var_vschfc3_db1_slot: &mut f64,
        var_vschfc3_db10_slot: &mut f64,
        var_vschfc3_db11_slot: &mut f64,
        var_vschfc3_db12_slot: &mut f64,
        var_vschfc3_db13_slot: &mut f64,
        var_vschfc3_db14_slot: &mut f64,
        var_vschfc3_db15_slot: &mut f64,
        var_vschfc3_db16_slot: &mut f64,
        var_vschfc3_db17_slot: &mut f64,
        var_vschfc3_db18_slot: &mut f64,
        var_vschfc3_db19_slot: &mut f64,
        var_vschfc3_db2_slot: &mut f64,
        var_vschfc3_db20_slot: &mut f64,
        var_vschfc3_db21_slot: &mut f64,
        var_vschfc3_db22_slot: &mut f64,
        var_vschfc3_db23_slot: &mut f64,
        var_vschfc3_db24_slot: &mut f64,
        var_vschfc3_db25_slot: &mut f64,
        var_vschfc3_db26_slot: &mut f64,
        var_vschfc3_db27_slot: &mut f64,
        var_vschfc3_db28_slot: &mut f64,
        var_vschfc3_db29_slot: &mut f64,
        var_vschfc3_db3_slot: &mut f64,
        var_vschfc3_db30_slot: &mut f64,
        var_vschfc3_db31_slot: &mut f64,
        var_vschfc3_db32_slot: &mut f64,
        var_vschfc3_db33_slot: &mut f64,
        var_vschfc3_db34_slot: &mut f64,
        var_vschfc3_db35_slot: &mut f64,
        var_vschfc3_db4_slot: &mut f64,
        var_vschfc3_db5_slot: &mut f64,
        var_vschfc3_db6_slot: &mut f64,
        var_vschfc3_db7_slot: &mut f64,
        var_vschfc3_db8_slot: &mut f64,
        var_vschfc3_db9_slot: &mut f64,
        var_vschfc3_dn0_slot: &mut f64,
        var_vschfc3_dn1_slot: &mut f64,
        var_vschfc3_dn10_slot: &mut f64,
        var_vschfc3_dn11_slot: &mut f64,
        var_vschfc3_dn12_slot: &mut f64,
        var_vschfc3_dn13_slot: &mut f64,
        var_vschfc3_dn14_slot: &mut f64,
        var_vschfc3_dn15_slot: &mut f64,
        var_vschfc3_dn16_slot: &mut f64,
        var_vschfc3_dn17_slot: &mut f64,
        var_vschfc3_dn18_slot: &mut f64,
        var_vschfc3_dn19_slot: &mut f64,
        var_vschfc3_dn2_slot: &mut f64,
        var_vschfc3_dn20_slot: &mut f64,
        var_vschfc3_dn21_slot: &mut f64,
        var_vschfc3_dn22_slot: &mut f64,
        var_vschfc3_dn23_slot: &mut f64,
        var_vschfc3_dn24_slot: &mut f64,
        var_vschfc3_dn25_slot: &mut f64,
        var_vschfc3_dn26_slot: &mut f64,
        var_vschfc3_dn27_slot: &mut f64,
        var_vschfc3_dn28_slot: &mut f64,
        var_vschfc3_dn29_slot: &mut f64,
        var_vschfc3_dn3_slot: &mut f64,
        var_vschfc3_dn4_slot: &mut f64,
        var_vschfc3_dn5_slot: &mut f64,
        var_vschfc3_dn6_slot: &mut f64,
        var_vschfc3_dn7_slot: &mut f64,
        var_vschfc3_dn8_slot: &mut f64,
        var_vschfc3_dn9_slot: &mut f64,
        var_vschfc4_slot: &mut f64,
        var_vschfc4_db0_slot: &mut f64,
        var_vschfc4_db1_slot: &mut f64,
        var_vschfc4_db10_slot: &mut f64,
        var_vschfc4_db11_slot: &mut f64,
        var_vschfc4_db12_slot: &mut f64,
        var_vschfc4_db13_slot: &mut f64,
        var_vschfc4_db14_slot: &mut f64,
        var_vschfc4_db15_slot: &mut f64,
        var_vschfc4_db16_slot: &mut f64,
        var_vschfc4_db17_slot: &mut f64,
        var_vschfc4_db18_slot: &mut f64,
        var_vschfc4_db19_slot: &mut f64,
        var_vschfc4_db2_slot: &mut f64,
        var_vschfc4_db20_slot: &mut f64,
        var_vschfc4_db21_slot: &mut f64,
        var_vschfc4_db22_slot: &mut f64,
        var_vschfc4_db23_slot: &mut f64,
        var_vschfc4_db24_slot: &mut f64,
        var_vschfc4_db25_slot: &mut f64,
        var_vschfc4_db26_slot: &mut f64,
        var_vschfc4_db27_slot: &mut f64,
        var_vschfc4_db28_slot: &mut f64,
        var_vschfc4_db29_slot: &mut f64,
        var_vschfc4_db3_slot: &mut f64,
        var_vschfc4_db30_slot: &mut f64,
        var_vschfc4_db31_slot: &mut f64,
        var_vschfc4_db32_slot: &mut f64,
        var_vschfc4_db33_slot: &mut f64,
        var_vschfc4_db34_slot: &mut f64,
        var_vschfc4_db35_slot: &mut f64,
        var_vschfc4_db4_slot: &mut f64,
        var_vschfc4_db5_slot: &mut f64,
        var_vschfc4_db6_slot: &mut f64,
        var_vschfc4_db7_slot: &mut f64,
        var_vschfc4_db8_slot: &mut f64,
        var_vschfc4_db9_slot: &mut f64,
        var_vschfc4_dn0_slot: &mut f64,
        var_vschfc4_dn1_slot: &mut f64,
        var_vschfc4_dn10_slot: &mut f64,
        var_vschfc4_dn11_slot: &mut f64,
        var_vschfc4_dn12_slot: &mut f64,
        var_vschfc4_dn13_slot: &mut f64,
        var_vschfc4_dn14_slot: &mut f64,
        var_vschfc4_dn15_slot: &mut f64,
        var_vschfc4_dn16_slot: &mut f64,
        var_vschfc4_dn17_slot: &mut f64,
        var_vschfc4_dn18_slot: &mut f64,
        var_vschfc4_dn19_slot: &mut f64,
        var_vschfc4_dn2_slot: &mut f64,
        var_vschfc4_dn20_slot: &mut f64,
        var_vschfc4_dn21_slot: &mut f64,
        var_vschfc4_dn22_slot: &mut f64,
        var_vschfc4_dn23_slot: &mut f64,
        var_vschfc4_dn24_slot: &mut f64,
        var_vschfc4_dn25_slot: &mut f64,
        var_vschfc4_dn26_slot: &mut f64,
        var_vschfc4_dn27_slot: &mut f64,
        var_vschfc4_dn28_slot: &mut f64,
        var_vschfc4_dn29_slot: &mut f64,
        var_vschfc4_dn3_slot: &mut f64,
        var_vschfc4_dn4_slot: &mut f64,
        var_vschfc4_dn5_slot: &mut f64,
        var_vschfc4_dn6_slot: &mut f64,
        var_vschfc4_dn7_slot: &mut f64,
        var_vschfc4_dn8_slot: &mut f64,
        var_vschfc4_dn9_slot: &mut f64,
        var_vschfc5_slot: &mut f64,
        var_vschfc5_db0_slot: &mut f64,
        var_vschfc5_db1_slot: &mut f64,
        var_vschfc5_db10_slot: &mut f64,
        var_vschfc5_db11_slot: &mut f64,
        var_vschfc5_db12_slot: &mut f64,
        var_vschfc5_db13_slot: &mut f64,
        var_vschfc5_db14_slot: &mut f64,
        var_vschfc5_db15_slot: &mut f64,
        var_vschfc5_db16_slot: &mut f64,
        var_vschfc5_db17_slot: &mut f64,
        var_vschfc5_db18_slot: &mut f64,
        var_vschfc5_db19_slot: &mut f64,
        var_vschfc5_db2_slot: &mut f64,
        var_vschfc5_db20_slot: &mut f64,
        var_vschfc5_db21_slot: &mut f64,
        var_vschfc5_db22_slot: &mut f64,
        var_vschfc5_db23_slot: &mut f64,
        var_vschfc5_db24_slot: &mut f64,
        var_vschfc5_db25_slot: &mut f64,
        var_vschfc5_db26_slot: &mut f64,
        var_vschfc5_db27_slot: &mut f64,
        var_vschfc5_db28_slot: &mut f64,
        var_vschfc5_db29_slot: &mut f64,
        var_vschfc5_db3_slot: &mut f64,
        var_vschfc5_db30_slot: &mut f64,
        var_vschfc5_db31_slot: &mut f64,
        var_vschfc5_db32_slot: &mut f64,
        var_vschfc5_db33_slot: &mut f64,
        var_vschfc5_db34_slot: &mut f64,
        var_vschfc5_db35_slot: &mut f64,
        var_vschfc5_db4_slot: &mut f64,
        var_vschfc5_db5_slot: &mut f64,
        var_vschfc5_db6_slot: &mut f64,
        var_vschfc5_db7_slot: &mut f64,
        var_vschfc5_db8_slot: &mut f64,
        var_vschfc5_db9_slot: &mut f64,
        var_vschfc5_dn0_slot: &mut f64,
        var_vschfc5_dn1_slot: &mut f64,
        var_vschfc5_dn10_slot: &mut f64,
        var_vschfc5_dn11_slot: &mut f64,
        var_vschfc5_dn12_slot: &mut f64,
        var_vschfc5_dn13_slot: &mut f64,
        var_vschfc5_dn14_slot: &mut f64,
        var_vschfc5_dn15_slot: &mut f64,
        var_vschfc5_dn16_slot: &mut f64,
        var_vschfc5_dn17_slot: &mut f64,
        var_vschfc5_dn18_slot: &mut f64,
        var_vschfc5_dn19_slot: &mut f64,
        var_vschfc5_dn2_slot: &mut f64,
        var_vschfc5_dn20_slot: &mut f64,
        var_vschfc5_dn21_slot: &mut f64,
        var_vschfc5_dn22_slot: &mut f64,
        var_vschfc5_dn23_slot: &mut f64,
        var_vschfc5_dn24_slot: &mut f64,
        var_vschfc5_dn25_slot: &mut f64,
        var_vschfc5_dn26_slot: &mut f64,
        var_vschfc5_dn27_slot: &mut f64,
        var_vschfc5_dn28_slot: &mut f64,
        var_vschfc5_dn29_slot: &mut f64,
        var_vschfc5_dn3_slot: &mut f64,
        var_vschfc5_dn4_slot: &mut f64,
        var_vschfc5_dn5_slot: &mut f64,
        var_vschfc5_dn6_slot: &mut f64,
        var_vschfc5_dn7_slot: &mut f64,
        var_vschfc5_dn8_slot: &mut f64,
        var_vschfc5_dn9_slot: &mut f64,
    ) {
        let mut var_guard477: f64 = *var_guard477_slot;
        let mut var_guard478: f64 = *var_guard478_slot;
        let mut var_qsch3: f64 = *var_qsch3_slot;
        let mut var_qsch3_db0: f64 = *var_qsch3_db0_slot;
        let mut var_qsch3_db1: f64 = *var_qsch3_db1_slot;
        let mut var_qsch3_db10: f64 = *var_qsch3_db10_slot;
        let mut var_qsch3_db11: f64 = *var_qsch3_db11_slot;
        let mut var_qsch3_db12: f64 = *var_qsch3_db12_slot;
        let mut var_qsch3_db13: f64 = *var_qsch3_db13_slot;
        let mut var_qsch3_db14: f64 = *var_qsch3_db14_slot;
        let mut var_qsch3_db15: f64 = *var_qsch3_db15_slot;
        let mut var_qsch3_db16: f64 = *var_qsch3_db16_slot;
        let mut var_qsch3_db17: f64 = *var_qsch3_db17_slot;
        let mut var_qsch3_db18: f64 = *var_qsch3_db18_slot;
        let mut var_qsch3_db19: f64 = *var_qsch3_db19_slot;
        let mut var_qsch3_db2: f64 = *var_qsch3_db2_slot;
        let mut var_qsch3_db20: f64 = *var_qsch3_db20_slot;
        let mut var_qsch3_db21: f64 = *var_qsch3_db21_slot;
        let mut var_qsch3_db22: f64 = *var_qsch3_db22_slot;
        let mut var_qsch3_db23: f64 = *var_qsch3_db23_slot;
        let mut var_qsch3_db24: f64 = *var_qsch3_db24_slot;
        let mut var_qsch3_db25: f64 = *var_qsch3_db25_slot;
        let mut var_qsch3_db26: f64 = *var_qsch3_db26_slot;
        let mut var_qsch3_db27: f64 = *var_qsch3_db27_slot;
        let mut var_qsch3_db28: f64 = *var_qsch3_db28_slot;
        let mut var_qsch3_db29: f64 = *var_qsch3_db29_slot;
        let mut var_qsch3_db3: f64 = *var_qsch3_db3_slot;
        let mut var_qsch3_db30: f64 = *var_qsch3_db30_slot;
        let mut var_qsch3_db31: f64 = *var_qsch3_db31_slot;
        let mut var_qsch3_db32: f64 = *var_qsch3_db32_slot;
        let mut var_qsch3_db33: f64 = *var_qsch3_db33_slot;
        let mut var_qsch3_db34: f64 = *var_qsch3_db34_slot;
        let mut var_qsch3_db35: f64 = *var_qsch3_db35_slot;
        let mut var_qsch3_db4: f64 = *var_qsch3_db4_slot;
        let mut var_qsch3_db5: f64 = *var_qsch3_db5_slot;
        let mut var_qsch3_db6: f64 = *var_qsch3_db6_slot;
        let mut var_qsch3_db7: f64 = *var_qsch3_db7_slot;
        let mut var_qsch3_db8: f64 = *var_qsch3_db8_slot;
        let mut var_qsch3_db9: f64 = *var_qsch3_db9_slot;
        let mut var_qsch3_dn0: f64 = *var_qsch3_dn0_slot;
        let mut var_qsch3_dn1: f64 = *var_qsch3_dn1_slot;
        let mut var_qsch3_dn10: f64 = *var_qsch3_dn10_slot;
        let mut var_qsch3_dn11: f64 = *var_qsch3_dn11_slot;
        let mut var_qsch3_dn12: f64 = *var_qsch3_dn12_slot;
        let mut var_qsch3_dn13: f64 = *var_qsch3_dn13_slot;
        let mut var_qsch3_dn14: f64 = *var_qsch3_dn14_slot;
        let mut var_qsch3_dn15: f64 = *var_qsch3_dn15_slot;
        let mut var_qsch3_dn16: f64 = *var_qsch3_dn16_slot;
        let mut var_qsch3_dn17: f64 = *var_qsch3_dn17_slot;
        let mut var_qsch3_dn18: f64 = *var_qsch3_dn18_slot;
        let mut var_qsch3_dn19: f64 = *var_qsch3_dn19_slot;
        let mut var_qsch3_dn2: f64 = *var_qsch3_dn2_slot;
        let mut var_qsch3_dn20: f64 = *var_qsch3_dn20_slot;
        let mut var_qsch3_dn21: f64 = *var_qsch3_dn21_slot;
        let mut var_qsch3_dn22: f64 = *var_qsch3_dn22_slot;
        let mut var_qsch3_dn23: f64 = *var_qsch3_dn23_slot;
        let mut var_qsch3_dn24: f64 = *var_qsch3_dn24_slot;
        let mut var_qsch3_dn25: f64 = *var_qsch3_dn25_slot;
        let mut var_qsch3_dn26: f64 = *var_qsch3_dn26_slot;
        let mut var_qsch3_dn27: f64 = *var_qsch3_dn27_slot;
        let mut var_qsch3_dn28: f64 = *var_qsch3_dn28_slot;
        let mut var_qsch3_dn29: f64 = *var_qsch3_dn29_slot;
        let mut var_qsch3_dn3: f64 = *var_qsch3_dn3_slot;
        let mut var_qsch3_dn4: f64 = *var_qsch3_dn4_slot;
        let mut var_qsch3_dn5: f64 = *var_qsch3_dn5_slot;
        let mut var_qsch3_dn6: f64 = *var_qsch3_dn6_slot;
        let mut var_qsch3_dn7: f64 = *var_qsch3_dn7_slot;
        let mut var_qsch3_dn8: f64 = *var_qsch3_dn8_slot;
        let mut var_qsch3_dn9: f64 = *var_qsch3_dn9_slot;
        let mut var_qsch4: f64 = *var_qsch4_slot;
        let mut var_qsch4_db0: f64 = *var_qsch4_db0_slot;
        let mut var_qsch4_db1: f64 = *var_qsch4_db1_slot;
        let mut var_qsch4_db10: f64 = *var_qsch4_db10_slot;
        let mut var_qsch4_db11: f64 = *var_qsch4_db11_slot;
        let mut var_qsch4_db12: f64 = *var_qsch4_db12_slot;
        let mut var_qsch4_db13: f64 = *var_qsch4_db13_slot;
        let mut var_qsch4_db14: f64 = *var_qsch4_db14_slot;
        let mut var_qsch4_db15: f64 = *var_qsch4_db15_slot;
        let mut var_qsch4_db16: f64 = *var_qsch4_db16_slot;
        let mut var_qsch4_db17: f64 = *var_qsch4_db17_slot;
        let mut var_qsch4_db18: f64 = *var_qsch4_db18_slot;
        let mut var_qsch4_db19: f64 = *var_qsch4_db19_slot;
        let mut var_qsch4_db2: f64 = *var_qsch4_db2_slot;
        let mut var_qsch4_db20: f64 = *var_qsch4_db20_slot;
        let mut var_qsch4_db21: f64 = *var_qsch4_db21_slot;
        let mut var_qsch4_db22: f64 = *var_qsch4_db22_slot;
        let mut var_qsch4_db23: f64 = *var_qsch4_db23_slot;
        let mut var_qsch4_db24: f64 = *var_qsch4_db24_slot;
        let mut var_qsch4_db25: f64 = *var_qsch4_db25_slot;
        let mut var_qsch4_db26: f64 = *var_qsch4_db26_slot;
        let mut var_qsch4_db27: f64 = *var_qsch4_db27_slot;
        let mut var_qsch4_db28: f64 = *var_qsch4_db28_slot;
        let mut var_qsch4_db29: f64 = *var_qsch4_db29_slot;
        let mut var_qsch4_db3: f64 = *var_qsch4_db3_slot;
        let mut var_qsch4_db30: f64 = *var_qsch4_db30_slot;
        let mut var_qsch4_db31: f64 = *var_qsch4_db31_slot;
        let mut var_qsch4_db32: f64 = *var_qsch4_db32_slot;
        let mut var_qsch4_db33: f64 = *var_qsch4_db33_slot;
        let mut var_qsch4_db34: f64 = *var_qsch4_db34_slot;
        let mut var_qsch4_db35: f64 = *var_qsch4_db35_slot;
        let mut var_qsch4_db4: f64 = *var_qsch4_db4_slot;
        let mut var_qsch4_db5: f64 = *var_qsch4_db5_slot;
        let mut var_qsch4_db6: f64 = *var_qsch4_db6_slot;
        let mut var_qsch4_db7: f64 = *var_qsch4_db7_slot;
        let mut var_qsch4_db8: f64 = *var_qsch4_db8_slot;
        let mut var_qsch4_db9: f64 = *var_qsch4_db9_slot;
        let mut var_qsch4_dn0: f64 = *var_qsch4_dn0_slot;
        let mut var_qsch4_dn1: f64 = *var_qsch4_dn1_slot;
        let mut var_qsch4_dn10: f64 = *var_qsch4_dn10_slot;
        let mut var_qsch4_dn11: f64 = *var_qsch4_dn11_slot;
        let mut var_qsch4_dn12: f64 = *var_qsch4_dn12_slot;
        let mut var_qsch4_dn13: f64 = *var_qsch4_dn13_slot;
        let mut var_qsch4_dn14: f64 = *var_qsch4_dn14_slot;
        let mut var_qsch4_dn15: f64 = *var_qsch4_dn15_slot;
        let mut var_qsch4_dn16: f64 = *var_qsch4_dn16_slot;
        let mut var_qsch4_dn17: f64 = *var_qsch4_dn17_slot;
        let mut var_qsch4_dn18: f64 = *var_qsch4_dn18_slot;
        let mut var_qsch4_dn19: f64 = *var_qsch4_dn19_slot;
        let mut var_qsch4_dn2: f64 = *var_qsch4_dn2_slot;
        let mut var_qsch4_dn20: f64 = *var_qsch4_dn20_slot;
        let mut var_qsch4_dn21: f64 = *var_qsch4_dn21_slot;
        let mut var_qsch4_dn22: f64 = *var_qsch4_dn22_slot;
        let mut var_qsch4_dn23: f64 = *var_qsch4_dn23_slot;
        let mut var_qsch4_dn24: f64 = *var_qsch4_dn24_slot;
        let mut var_qsch4_dn25: f64 = *var_qsch4_dn25_slot;
        let mut var_qsch4_dn26: f64 = *var_qsch4_dn26_slot;
        let mut var_qsch4_dn27: f64 = *var_qsch4_dn27_slot;
        let mut var_qsch4_dn28: f64 = *var_qsch4_dn28_slot;
        let mut var_qsch4_dn29: f64 = *var_qsch4_dn29_slot;
        let mut var_qsch4_dn3: f64 = *var_qsch4_dn3_slot;
        let mut var_qsch4_dn4: f64 = *var_qsch4_dn4_slot;
        let mut var_qsch4_dn5: f64 = *var_qsch4_dn5_slot;
        let mut var_qsch4_dn6: f64 = *var_qsch4_dn6_slot;
        let mut var_qsch4_dn7: f64 = *var_qsch4_dn7_slot;
        let mut var_qsch4_dn8: f64 = *var_qsch4_dn8_slot;
        let mut var_qsch4_dn9: f64 = *var_qsch4_dn9_slot;
        let mut var_qsch4c: f64 = *var_qsch4c_slot;
        let mut var_qsch5: f64 = *var_qsch5_slot;
        let mut var_qsch5_db0: f64 = *var_qsch5_db0_slot;
        let mut var_qsch5_db1: f64 = *var_qsch5_db1_slot;
        let mut var_qsch5_db10: f64 = *var_qsch5_db10_slot;
        let mut var_qsch5_db11: f64 = *var_qsch5_db11_slot;
        let mut var_qsch5_db12: f64 = *var_qsch5_db12_slot;
        let mut var_qsch5_db13: f64 = *var_qsch5_db13_slot;
        let mut var_qsch5_db14: f64 = *var_qsch5_db14_slot;
        let mut var_qsch5_db15: f64 = *var_qsch5_db15_slot;
        let mut var_qsch5_db16: f64 = *var_qsch5_db16_slot;
        let mut var_qsch5_db17: f64 = *var_qsch5_db17_slot;
        let mut var_qsch5_db18: f64 = *var_qsch5_db18_slot;
        let mut var_qsch5_db19: f64 = *var_qsch5_db19_slot;
        let mut var_qsch5_db2: f64 = *var_qsch5_db2_slot;
        let mut var_qsch5_db20: f64 = *var_qsch5_db20_slot;
        let mut var_qsch5_db21: f64 = *var_qsch5_db21_slot;
        let mut var_qsch5_db22: f64 = *var_qsch5_db22_slot;
        let mut var_qsch5_db23: f64 = *var_qsch5_db23_slot;
        let mut var_qsch5_db24: f64 = *var_qsch5_db24_slot;
        let mut var_qsch5_db25: f64 = *var_qsch5_db25_slot;
        let mut var_qsch5_db26: f64 = *var_qsch5_db26_slot;
        let mut var_qsch5_db27: f64 = *var_qsch5_db27_slot;
        let mut var_qsch5_db28: f64 = *var_qsch5_db28_slot;
        let mut var_qsch5_db29: f64 = *var_qsch5_db29_slot;
        let mut var_qsch5_db3: f64 = *var_qsch5_db3_slot;
        let mut var_qsch5_db30: f64 = *var_qsch5_db30_slot;
        let mut var_qsch5_db31: f64 = *var_qsch5_db31_slot;
        let mut var_qsch5_db32: f64 = *var_qsch5_db32_slot;
        let mut var_qsch5_db33: f64 = *var_qsch5_db33_slot;
        let mut var_qsch5_db34: f64 = *var_qsch5_db34_slot;
        let mut var_qsch5_db35: f64 = *var_qsch5_db35_slot;
        let mut var_qsch5_db4: f64 = *var_qsch5_db4_slot;
        let mut var_qsch5_db5: f64 = *var_qsch5_db5_slot;
        let mut var_qsch5_db6: f64 = *var_qsch5_db6_slot;
        let mut var_qsch5_db7: f64 = *var_qsch5_db7_slot;
        let mut var_qsch5_db8: f64 = *var_qsch5_db8_slot;
        let mut var_qsch5_db9: f64 = *var_qsch5_db9_slot;
        let mut var_qsch5_dn0: f64 = *var_qsch5_dn0_slot;
        let mut var_qsch5_dn1: f64 = *var_qsch5_dn1_slot;
        let mut var_qsch5_dn10: f64 = *var_qsch5_dn10_slot;
        let mut var_qsch5_dn11: f64 = *var_qsch5_dn11_slot;
        let mut var_qsch5_dn12: f64 = *var_qsch5_dn12_slot;
        let mut var_qsch5_dn13: f64 = *var_qsch5_dn13_slot;
        let mut var_qsch5_dn14: f64 = *var_qsch5_dn14_slot;
        let mut var_qsch5_dn15: f64 = *var_qsch5_dn15_slot;
        let mut var_qsch5_dn16: f64 = *var_qsch5_dn16_slot;
        let mut var_qsch5_dn17: f64 = *var_qsch5_dn17_slot;
        let mut var_qsch5_dn18: f64 = *var_qsch5_dn18_slot;
        let mut var_qsch5_dn19: f64 = *var_qsch5_dn19_slot;
        let mut var_qsch5_dn2: f64 = *var_qsch5_dn2_slot;
        let mut var_qsch5_dn20: f64 = *var_qsch5_dn20_slot;
        let mut var_qsch5_dn21: f64 = *var_qsch5_dn21_slot;
        let mut var_qsch5_dn22: f64 = *var_qsch5_dn22_slot;
        let mut var_qsch5_dn23: f64 = *var_qsch5_dn23_slot;
        let mut var_qsch5_dn24: f64 = *var_qsch5_dn24_slot;
        let mut var_qsch5_dn25: f64 = *var_qsch5_dn25_slot;
        let mut var_qsch5_dn26: f64 = *var_qsch5_dn26_slot;
        let mut var_qsch5_dn27: f64 = *var_qsch5_dn27_slot;
        let mut var_qsch5_dn28: f64 = *var_qsch5_dn28_slot;
        let mut var_qsch5_dn29: f64 = *var_qsch5_dn29_slot;
        let mut var_qsch5_dn3: f64 = *var_qsch5_dn3_slot;
        let mut var_qsch5_dn4: f64 = *var_qsch5_dn4_slot;
        let mut var_qsch5_dn5: f64 = *var_qsch5_dn5_slot;
        let mut var_qsch5_dn6: f64 = *var_qsch5_dn6_slot;
        let mut var_qsch5_dn7: f64 = *var_qsch5_dn7_slot;
        let mut var_qsch5_dn8: f64 = *var_qsch5_dn8_slot;
        let mut var_qsch5_dn9: f64 = *var_qsch5_dn9_slot;
        let mut var_qsch5c: f64 = *var_qsch5c_slot;
        let mut var_vschfc3: f64 = *var_vschfc3_slot;
        let mut var_vschfc3_db0: f64 = *var_vschfc3_db0_slot;
        let mut var_vschfc3_db1: f64 = *var_vschfc3_db1_slot;
        let mut var_vschfc3_db10: f64 = *var_vschfc3_db10_slot;
        let mut var_vschfc3_db11: f64 = *var_vschfc3_db11_slot;
        let mut var_vschfc3_db12: f64 = *var_vschfc3_db12_slot;
        let mut var_vschfc3_db13: f64 = *var_vschfc3_db13_slot;
        let mut var_vschfc3_db14: f64 = *var_vschfc3_db14_slot;
        let mut var_vschfc3_db15: f64 = *var_vschfc3_db15_slot;
        let mut var_vschfc3_db16: f64 = *var_vschfc3_db16_slot;
        let mut var_vschfc3_db17: f64 = *var_vschfc3_db17_slot;
        let mut var_vschfc3_db18: f64 = *var_vschfc3_db18_slot;
        let mut var_vschfc3_db19: f64 = *var_vschfc3_db19_slot;
        let mut var_vschfc3_db2: f64 = *var_vschfc3_db2_slot;
        let mut var_vschfc3_db20: f64 = *var_vschfc3_db20_slot;
        let mut var_vschfc3_db21: f64 = *var_vschfc3_db21_slot;
        let mut var_vschfc3_db22: f64 = *var_vschfc3_db22_slot;
        let mut var_vschfc3_db23: f64 = *var_vschfc3_db23_slot;
        let mut var_vschfc3_db24: f64 = *var_vschfc3_db24_slot;
        let mut var_vschfc3_db25: f64 = *var_vschfc3_db25_slot;
        let mut var_vschfc3_db26: f64 = *var_vschfc3_db26_slot;
        let mut var_vschfc3_db27: f64 = *var_vschfc3_db27_slot;
        let mut var_vschfc3_db28: f64 = *var_vschfc3_db28_slot;
        let mut var_vschfc3_db29: f64 = *var_vschfc3_db29_slot;
        let mut var_vschfc3_db3: f64 = *var_vschfc3_db3_slot;
        let mut var_vschfc3_db30: f64 = *var_vschfc3_db30_slot;
        let mut var_vschfc3_db31: f64 = *var_vschfc3_db31_slot;
        let mut var_vschfc3_db32: f64 = *var_vschfc3_db32_slot;
        let mut var_vschfc3_db33: f64 = *var_vschfc3_db33_slot;
        let mut var_vschfc3_db34: f64 = *var_vschfc3_db34_slot;
        let mut var_vschfc3_db35: f64 = *var_vschfc3_db35_slot;
        let mut var_vschfc3_db4: f64 = *var_vschfc3_db4_slot;
        let mut var_vschfc3_db5: f64 = *var_vschfc3_db5_slot;
        let mut var_vschfc3_db6: f64 = *var_vschfc3_db6_slot;
        let mut var_vschfc3_db7: f64 = *var_vschfc3_db7_slot;
        let mut var_vschfc3_db8: f64 = *var_vschfc3_db8_slot;
        let mut var_vschfc3_db9: f64 = *var_vschfc3_db9_slot;
        let mut var_vschfc3_dn0: f64 = *var_vschfc3_dn0_slot;
        let mut var_vschfc3_dn1: f64 = *var_vschfc3_dn1_slot;
        let mut var_vschfc3_dn10: f64 = *var_vschfc3_dn10_slot;
        let mut var_vschfc3_dn11: f64 = *var_vschfc3_dn11_slot;
        let mut var_vschfc3_dn12: f64 = *var_vschfc3_dn12_slot;
        let mut var_vschfc3_dn13: f64 = *var_vschfc3_dn13_slot;
        let mut var_vschfc3_dn14: f64 = *var_vschfc3_dn14_slot;
        let mut var_vschfc3_dn15: f64 = *var_vschfc3_dn15_slot;
        let mut var_vschfc3_dn16: f64 = *var_vschfc3_dn16_slot;
        let mut var_vschfc3_dn17: f64 = *var_vschfc3_dn17_slot;
        let mut var_vschfc3_dn18: f64 = *var_vschfc3_dn18_slot;
        let mut var_vschfc3_dn19: f64 = *var_vschfc3_dn19_slot;
        let mut var_vschfc3_dn2: f64 = *var_vschfc3_dn2_slot;
        let mut var_vschfc3_dn20: f64 = *var_vschfc3_dn20_slot;
        let mut var_vschfc3_dn21: f64 = *var_vschfc3_dn21_slot;
        let mut var_vschfc3_dn22: f64 = *var_vschfc3_dn22_slot;
        let mut var_vschfc3_dn23: f64 = *var_vschfc3_dn23_slot;
        let mut var_vschfc3_dn24: f64 = *var_vschfc3_dn24_slot;
        let mut var_vschfc3_dn25: f64 = *var_vschfc3_dn25_slot;
        let mut var_vschfc3_dn26: f64 = *var_vschfc3_dn26_slot;
        let mut var_vschfc3_dn27: f64 = *var_vschfc3_dn27_slot;
        let mut var_vschfc3_dn28: f64 = *var_vschfc3_dn28_slot;
        let mut var_vschfc3_dn29: f64 = *var_vschfc3_dn29_slot;
        let mut var_vschfc3_dn3: f64 = *var_vschfc3_dn3_slot;
        let mut var_vschfc3_dn4: f64 = *var_vschfc3_dn4_slot;
        let mut var_vschfc3_dn5: f64 = *var_vschfc3_dn5_slot;
        let mut var_vschfc3_dn6: f64 = *var_vschfc3_dn6_slot;
        let mut var_vschfc3_dn7: f64 = *var_vschfc3_dn7_slot;
        let mut var_vschfc3_dn8: f64 = *var_vschfc3_dn8_slot;
        let mut var_vschfc3_dn9: f64 = *var_vschfc3_dn9_slot;
        let mut var_vschfc4: f64 = *var_vschfc4_slot;
        let mut var_vschfc4_db0: f64 = *var_vschfc4_db0_slot;
        let mut var_vschfc4_db1: f64 = *var_vschfc4_db1_slot;
        let mut var_vschfc4_db10: f64 = *var_vschfc4_db10_slot;
        let mut var_vschfc4_db11: f64 = *var_vschfc4_db11_slot;
        let mut var_vschfc4_db12: f64 = *var_vschfc4_db12_slot;
        let mut var_vschfc4_db13: f64 = *var_vschfc4_db13_slot;
        let mut var_vschfc4_db14: f64 = *var_vschfc4_db14_slot;
        let mut var_vschfc4_db15: f64 = *var_vschfc4_db15_slot;
        let mut var_vschfc4_db16: f64 = *var_vschfc4_db16_slot;
        let mut var_vschfc4_db17: f64 = *var_vschfc4_db17_slot;
        let mut var_vschfc4_db18: f64 = *var_vschfc4_db18_slot;
        let mut var_vschfc4_db19: f64 = *var_vschfc4_db19_slot;
        let mut var_vschfc4_db2: f64 = *var_vschfc4_db2_slot;
        let mut var_vschfc4_db20: f64 = *var_vschfc4_db20_slot;
        let mut var_vschfc4_db21: f64 = *var_vschfc4_db21_slot;
        let mut var_vschfc4_db22: f64 = *var_vschfc4_db22_slot;
        let mut var_vschfc4_db23: f64 = *var_vschfc4_db23_slot;
        let mut var_vschfc4_db24: f64 = *var_vschfc4_db24_slot;
        let mut var_vschfc4_db25: f64 = *var_vschfc4_db25_slot;
        let mut var_vschfc4_db26: f64 = *var_vschfc4_db26_slot;
        let mut var_vschfc4_db27: f64 = *var_vschfc4_db27_slot;
        let mut var_vschfc4_db28: f64 = *var_vschfc4_db28_slot;
        let mut var_vschfc4_db29: f64 = *var_vschfc4_db29_slot;
        let mut var_vschfc4_db3: f64 = *var_vschfc4_db3_slot;
        let mut var_vschfc4_db30: f64 = *var_vschfc4_db30_slot;
        let mut var_vschfc4_db31: f64 = *var_vschfc4_db31_slot;
        let mut var_vschfc4_db32: f64 = *var_vschfc4_db32_slot;
        let mut var_vschfc4_db33: f64 = *var_vschfc4_db33_slot;
        let mut var_vschfc4_db34: f64 = *var_vschfc4_db34_slot;
        let mut var_vschfc4_db35: f64 = *var_vschfc4_db35_slot;
        let mut var_vschfc4_db4: f64 = *var_vschfc4_db4_slot;
        let mut var_vschfc4_db5: f64 = *var_vschfc4_db5_slot;
        let mut var_vschfc4_db6: f64 = *var_vschfc4_db6_slot;
        let mut var_vschfc4_db7: f64 = *var_vschfc4_db7_slot;
        let mut var_vschfc4_db8: f64 = *var_vschfc4_db8_slot;
        let mut var_vschfc4_db9: f64 = *var_vschfc4_db9_slot;
        let mut var_vschfc4_dn0: f64 = *var_vschfc4_dn0_slot;
        let mut var_vschfc4_dn1: f64 = *var_vschfc4_dn1_slot;
        let mut var_vschfc4_dn10: f64 = *var_vschfc4_dn10_slot;
        let mut var_vschfc4_dn11: f64 = *var_vschfc4_dn11_slot;
        let mut var_vschfc4_dn12: f64 = *var_vschfc4_dn12_slot;
        let mut var_vschfc4_dn13: f64 = *var_vschfc4_dn13_slot;
        let mut var_vschfc4_dn14: f64 = *var_vschfc4_dn14_slot;
        let mut var_vschfc4_dn15: f64 = *var_vschfc4_dn15_slot;
        let mut var_vschfc4_dn16: f64 = *var_vschfc4_dn16_slot;
        let mut var_vschfc4_dn17: f64 = *var_vschfc4_dn17_slot;
        let mut var_vschfc4_dn18: f64 = *var_vschfc4_dn18_slot;
        let mut var_vschfc4_dn19: f64 = *var_vschfc4_dn19_slot;
        let mut var_vschfc4_dn2: f64 = *var_vschfc4_dn2_slot;
        let mut var_vschfc4_dn20: f64 = *var_vschfc4_dn20_slot;
        let mut var_vschfc4_dn21: f64 = *var_vschfc4_dn21_slot;
        let mut var_vschfc4_dn22: f64 = *var_vschfc4_dn22_slot;
        let mut var_vschfc4_dn23: f64 = *var_vschfc4_dn23_slot;
        let mut var_vschfc4_dn24: f64 = *var_vschfc4_dn24_slot;
        let mut var_vschfc4_dn25: f64 = *var_vschfc4_dn25_slot;
        let mut var_vschfc4_dn26: f64 = *var_vschfc4_dn26_slot;
        let mut var_vschfc4_dn27: f64 = *var_vschfc4_dn27_slot;
        let mut var_vschfc4_dn28: f64 = *var_vschfc4_dn28_slot;
        let mut var_vschfc4_dn29: f64 = *var_vschfc4_dn29_slot;
        let mut var_vschfc4_dn3: f64 = *var_vschfc4_dn3_slot;
        let mut var_vschfc4_dn4: f64 = *var_vschfc4_dn4_slot;
        let mut var_vschfc4_dn5: f64 = *var_vschfc4_dn5_slot;
        let mut var_vschfc4_dn6: f64 = *var_vschfc4_dn6_slot;
        let mut var_vschfc4_dn7: f64 = *var_vschfc4_dn7_slot;
        let mut var_vschfc4_dn8: f64 = *var_vschfc4_dn8_slot;
        let mut var_vschfc4_dn9: f64 = *var_vschfc4_dn9_slot;
        let mut var_vschfc5: f64 = *var_vschfc5_slot;
        let mut var_vschfc5_db0: f64 = *var_vschfc5_db0_slot;
        let mut var_vschfc5_db1: f64 = *var_vschfc5_db1_slot;
        let mut var_vschfc5_db10: f64 = *var_vschfc5_db10_slot;
        let mut var_vschfc5_db11: f64 = *var_vschfc5_db11_slot;
        let mut var_vschfc5_db12: f64 = *var_vschfc5_db12_slot;
        let mut var_vschfc5_db13: f64 = *var_vschfc5_db13_slot;
        let mut var_vschfc5_db14: f64 = *var_vschfc5_db14_slot;
        let mut var_vschfc5_db15: f64 = *var_vschfc5_db15_slot;
        let mut var_vschfc5_db16: f64 = *var_vschfc5_db16_slot;
        let mut var_vschfc5_db17: f64 = *var_vschfc5_db17_slot;
        let mut var_vschfc5_db18: f64 = *var_vschfc5_db18_slot;
        let mut var_vschfc5_db19: f64 = *var_vschfc5_db19_slot;
        let mut var_vschfc5_db2: f64 = *var_vschfc5_db2_slot;
        let mut var_vschfc5_db20: f64 = *var_vschfc5_db20_slot;
        let mut var_vschfc5_db21: f64 = *var_vschfc5_db21_slot;
        let mut var_vschfc5_db22: f64 = *var_vschfc5_db22_slot;
        let mut var_vschfc5_db23: f64 = *var_vschfc5_db23_slot;
        let mut var_vschfc5_db24: f64 = *var_vschfc5_db24_slot;
        let mut var_vschfc5_db25: f64 = *var_vschfc5_db25_slot;
        let mut var_vschfc5_db26: f64 = *var_vschfc5_db26_slot;
        let mut var_vschfc5_db27: f64 = *var_vschfc5_db27_slot;
        let mut var_vschfc5_db28: f64 = *var_vschfc5_db28_slot;
        let mut var_vschfc5_db29: f64 = *var_vschfc5_db29_slot;
        let mut var_vschfc5_db3: f64 = *var_vschfc5_db3_slot;
        let mut var_vschfc5_db30: f64 = *var_vschfc5_db30_slot;
        let mut var_vschfc5_db31: f64 = *var_vschfc5_db31_slot;
        let mut var_vschfc5_db32: f64 = *var_vschfc5_db32_slot;
        let mut var_vschfc5_db33: f64 = *var_vschfc5_db33_slot;
        let mut var_vschfc5_db34: f64 = *var_vschfc5_db34_slot;
        let mut var_vschfc5_db35: f64 = *var_vschfc5_db35_slot;
        let mut var_vschfc5_db4: f64 = *var_vschfc5_db4_slot;
        let mut var_vschfc5_db5: f64 = *var_vschfc5_db5_slot;
        let mut var_vschfc5_db6: f64 = *var_vschfc5_db6_slot;
        let mut var_vschfc5_db7: f64 = *var_vschfc5_db7_slot;
        let mut var_vschfc5_db8: f64 = *var_vschfc5_db8_slot;
        let mut var_vschfc5_db9: f64 = *var_vschfc5_db9_slot;
        let mut var_vschfc5_dn0: f64 = *var_vschfc5_dn0_slot;
        let mut var_vschfc5_dn1: f64 = *var_vschfc5_dn1_slot;
        let mut var_vschfc5_dn10: f64 = *var_vschfc5_dn10_slot;
        let mut var_vschfc5_dn11: f64 = *var_vschfc5_dn11_slot;
        let mut var_vschfc5_dn12: f64 = *var_vschfc5_dn12_slot;
        let mut var_vschfc5_dn13: f64 = *var_vschfc5_dn13_slot;
        let mut var_vschfc5_dn14: f64 = *var_vschfc5_dn14_slot;
        let mut var_vschfc5_dn15: f64 = *var_vschfc5_dn15_slot;
        let mut var_vschfc5_dn16: f64 = *var_vschfc5_dn16_slot;
        let mut var_vschfc5_dn17: f64 = *var_vschfc5_dn17_slot;
        let mut var_vschfc5_dn18: f64 = *var_vschfc5_dn18_slot;
        let mut var_vschfc5_dn19: f64 = *var_vschfc5_dn19_slot;
        let mut var_vschfc5_dn2: f64 = *var_vschfc5_dn2_slot;
        let mut var_vschfc5_dn20: f64 = *var_vschfc5_dn20_slot;
        let mut var_vschfc5_dn21: f64 = *var_vschfc5_dn21_slot;
        let mut var_vschfc5_dn22: f64 = *var_vschfc5_dn22_slot;
        let mut var_vschfc5_dn23: f64 = *var_vschfc5_dn23_slot;
        let mut var_vschfc5_dn24: f64 = *var_vschfc5_dn24_slot;
        let mut var_vschfc5_dn25: f64 = *var_vschfc5_dn25_slot;
        let mut var_vschfc5_dn26: f64 = *var_vschfc5_dn26_slot;
        let mut var_vschfc5_dn27: f64 = *var_vschfc5_dn27_slot;
        let mut var_vschfc5_dn28: f64 = *var_vschfc5_dn28_slot;
        let mut var_vschfc5_dn29: f64 = *var_vschfc5_dn29_slot;
        let mut var_vschfc5_dn3: f64 = *var_vschfc5_dn3_slot;
        let mut var_vschfc5_dn4: f64 = *var_vschfc5_dn4_slot;
        let mut var_vschfc5_dn5: f64 = *var_vschfc5_dn5_slot;
        let mut var_vschfc5_dn6: f64 = *var_vschfc5_dn6_slot;
        let mut var_vschfc5_dn7: f64 = *var_vschfc5_dn7_slot;
        let mut var_vschfc5_dn8: f64 = *var_vschfc5_dn8_slot;
        let mut var_vschfc5_dn9: f64 = *var_vschfc5_dn9_slot;

        let (assign43750_e42381, assign43750_e42381_d_n0, assign43750_e42381_d_n1, assign43750_e42381_d_n2, assign43750_e42381_d_n3, assign43750_e42381_d_n4, assign43750_e42381_d_n5, assign43750_e42381_d_n6, assign43750_e42381_d_n7, assign43750_e42381_d_n8, assign43750_e42381_d_n9, assign43750_e42381_d_n10, assign43750_e42381_d_n11, assign43750_e42381_d_n12, assign43750_e42381_d_n13, assign43750_e42381_d_n14, assign43750_e42381_d_n15, assign43750_e42381_d_n16, assign43750_e42381_d_n17, assign43750_e42381_d_n18, assign43750_e42381_d_n19, assign43750_e42381_d_n20, assign43750_e42381_d_n21, assign43750_e42381_d_n22, assign43750_e42381_d_n23, assign43750_e42381_d_n24, assign43750_e42381_d_n25, assign43750_e42381_d_n26, assign43750_e42381_d_n27, assign43750_e42381_d_n28, assign43750_e42381_d_n29, assign43750_e42381_d_b0, assign43750_e42381_d_b1, assign43750_e42381_d_b2, assign43750_e42381_d_b3, assign43750_e42381_d_b4, assign43750_e42381_d_b5, assign43750_e42381_d_b6, assign43750_e42381_d_b7, assign43750_e42381_d_b8, assign43750_e42381_d_b9, assign43750_e42381_d_b10, assign43750_e42381_d_b11, assign43750_e42381_d_b12, assign43750_e42381_d_b13, assign43750_e42381_d_b14, assign43750_e42381_d_b15, assign43750_e42381_d_b16, assign43750_e42381_d_b17, assign43750_e42381_d_b18, assign43750_e42381_d_b19, assign43750_e42381_d_b20, assign43750_e42381_d_b21, assign43750_e42381_d_b22, assign43750_e42381_d_b23, assign43750_e42381_d_b24, assign43750_e42381_d_b25, assign43750_e42381_d_b26, assign43750_e42381_d_b27, assign43750_e42381_d_b28, assign43750_e42381_d_b29, assign43750_e42381_d_b30, assign43750_e42381_d_b31, assign43750_e42381_d_b32, assign43750_e42381_d_b33, assign43750_e42381_d_b34, assign43750_e42381_d_b35,) = {
    if (((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) {
        let assign43750_e42379: f64 = (var_vschfc2 * var_vschfc1);
        (assign43750_e42379, ((var_vschfc2_dn0 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn0)), ((var_vschfc2_dn1 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn1)), ((var_vschfc2_dn2 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn2)), ((var_vschfc2_dn3 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn3)), ((var_vschfc2_dn4 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn4)), ((var_vschfc2_dn5 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn5)), ((var_vschfc2_dn6 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn6)), ((var_vschfc2_dn7 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn7)), ((var_vschfc2_dn8 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn8)), ((var_vschfc2_dn9 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn9)), ((var_vschfc2_dn10 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn10)), ((var_vschfc2_dn11 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn11)), ((var_vschfc2_dn12 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn12)), ((var_vschfc2_dn13 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn13)), ((var_vschfc2_dn14 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn14)), ((var_vschfc2_dn15 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn15)), ((var_vschfc2_dn16 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn16)), ((var_vschfc2_dn17 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn17)), ((var_vschfc2_dn18 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn18)), ((var_vschfc2_dn19 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn19)), ((var_vschfc2_dn20 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn20)), ((var_vschfc2_dn21 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn21)), ((var_vschfc2_dn22 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn22)), ((var_vschfc2_dn23 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn23)), ((var_vschfc2_dn24 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn24)), ((var_vschfc2_dn25 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn25)), ((var_vschfc2_dn26 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn26)), ((var_vschfc2_dn27 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn27)), ((var_vschfc2_dn28 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn28)), ((var_vschfc2_dn29 * var_vschfc1) + (var_vschfc2 * var_vschfc1_dn29)), ((var_vschfc2_db0 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db0)), ((var_vschfc2_db1 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db1)), ((var_vschfc2_db2 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db2)), ((var_vschfc2_db3 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db3)), ((var_vschfc2_db4 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db4)), ((var_vschfc2_db5 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db5)), ((var_vschfc2_db6 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db6)), ((var_vschfc2_db7 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db7)), ((var_vschfc2_db8 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db8)), ((var_vschfc2_db9 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db9)), ((var_vschfc2_db10 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db10)), ((var_vschfc2_db11 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db11)), ((var_vschfc2_db12 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db12)), ((var_vschfc2_db13 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db13)), ((var_vschfc2_db14 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db14)), ((var_vschfc2_db15 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db15)), ((var_vschfc2_db16 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db16)), ((var_vschfc2_db17 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db17)), ((var_vschfc2_db18 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db18)), ((var_vschfc2_db19 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db19)), ((var_vschfc2_db20 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db20)), ((var_vschfc2_db21 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db21)), ((var_vschfc2_db22 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db22)), ((var_vschfc2_db23 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db23)), ((var_vschfc2_db24 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db24)), ((var_vschfc2_db25 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db25)), ((var_vschfc2_db26 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db26)), ((var_vschfc2_db27 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db27)), ((var_vschfc2_db28 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db28)), ((var_vschfc2_db29 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db29)), ((var_vschfc2_db30 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db30)), ((var_vschfc2_db31 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db31)), ((var_vschfc2_db32 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db32)), ((var_vschfc2_db33 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db33)), ((var_vschfc2_db34 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db34)), ((var_vschfc2_db35 * var_vschfc1) + (var_vschfc2 * var_vschfc1_db35)),)
    } else {
        (var_vschfc3, var_vschfc3_dn0, var_vschfc3_dn1, var_vschfc3_dn2, var_vschfc3_dn3, var_vschfc3_dn4, var_vschfc3_dn5, var_vschfc3_dn6, var_vschfc3_dn7, var_vschfc3_dn8, var_vschfc3_dn9, var_vschfc3_dn10, var_vschfc3_dn11, var_vschfc3_dn12, var_vschfc3_dn13, var_vschfc3_dn14, var_vschfc3_dn15, var_vschfc3_dn16, var_vschfc3_dn17, var_vschfc3_dn18, var_vschfc3_dn19, var_vschfc3_dn20, var_vschfc3_dn21, var_vschfc3_dn22, var_vschfc3_dn23, var_vschfc3_dn24, var_vschfc3_dn25, var_vschfc3_dn26, var_vschfc3_dn27, var_vschfc3_dn28, var_vschfc3_dn29, var_vschfc3_db0, var_vschfc3_db1, var_vschfc3_db2, var_vschfc3_db3, var_vschfc3_db4, var_vschfc3_db5, var_vschfc3_db6, var_vschfc3_db7, var_vschfc3_db8, var_vschfc3_db9, var_vschfc3_db10, var_vschfc3_db11, var_vschfc3_db12, var_vschfc3_db13, var_vschfc3_db14, var_vschfc3_db15, var_vschfc3_db16, var_vschfc3_db17, var_vschfc3_db18, var_vschfc3_db19, var_vschfc3_db20, var_vschfc3_db21, var_vschfc3_db22, var_vschfc3_db23, var_vschfc3_db24, var_vschfc3_db25, var_vschfc3_db26, var_vschfc3_db27, var_vschfc3_db28, var_vschfc3_db29, var_vschfc3_db30, var_vschfc3_db31, var_vschfc3_db32, var_vschfc3_db33, var_vschfc3_db34, var_vschfc3_db35,)
    }
};
        var_vschfc3 = assign43750_e42381;
        var_vschfc3_dn0 = assign43750_e42381_d_n0;
        var_vschfc3_dn1 = assign43750_e42381_d_n1;
        var_vschfc3_dn2 = assign43750_e42381_d_n2;
        var_vschfc3_dn3 = assign43750_e42381_d_n3;
        var_vschfc3_dn4 = assign43750_e42381_d_n4;
        var_vschfc3_dn5 = assign43750_e42381_d_n5;
        var_vschfc3_dn6 = assign43750_e42381_d_n6;
        var_vschfc3_dn7 = assign43750_e42381_d_n7;
        var_vschfc3_dn8 = assign43750_e42381_d_n8;
        var_vschfc3_dn9 = assign43750_e42381_d_n9;
        var_vschfc3_dn10 = assign43750_e42381_d_n10;
        var_vschfc3_dn11 = assign43750_e42381_d_n11;
        var_vschfc3_dn12 = assign43750_e42381_d_n12;
        var_vschfc3_dn13 = assign43750_e42381_d_n13;
        var_vschfc3_dn14 = assign43750_e42381_d_n14;
        var_vschfc3_dn15 = assign43750_e42381_d_n15;
        var_vschfc3_dn16 = assign43750_e42381_d_n16;
        var_vschfc3_dn17 = assign43750_e42381_d_n17;
        var_vschfc3_dn18 = assign43750_e42381_d_n18;
        var_vschfc3_dn19 = assign43750_e42381_d_n19;
        var_vschfc3_dn20 = assign43750_e42381_d_n20;
        var_vschfc3_dn21 = assign43750_e42381_d_n21;
        var_vschfc3_dn22 = assign43750_e42381_d_n22;
        var_vschfc3_dn23 = assign43750_e42381_d_n23;
        var_vschfc3_dn24 = assign43750_e42381_d_n24;
        var_vschfc3_dn25 = assign43750_e42381_d_n25;
        var_vschfc3_dn26 = assign43750_e42381_d_n26;
        var_vschfc3_dn27 = assign43750_e42381_d_n27;
        var_vschfc3_dn28 = assign43750_e42381_d_n28;
        var_vschfc3_dn29 = assign43750_e42381_d_n29;
        var_vschfc3_db0 = assign43750_e42381_d_b0;
        var_vschfc3_db1 = assign43750_e42381_d_b1;
        var_vschfc3_db2 = assign43750_e42381_d_b2;
        var_vschfc3_db3 = assign43750_e42381_d_b3;
        var_vschfc3_db4 = assign43750_e42381_d_b4;
        var_vschfc3_db5 = assign43750_e42381_d_b5;
        var_vschfc3_db6 = assign43750_e42381_d_b6;
        var_vschfc3_db7 = assign43750_e42381_d_b7;
        var_vschfc3_db8 = assign43750_e42381_d_b8;
        var_vschfc3_db9 = assign43750_e42381_d_b9;
        var_vschfc3_db10 = assign43750_e42381_d_b10;
        var_vschfc3_db11 = assign43750_e42381_d_b11;
        var_vschfc3_db12 = assign43750_e42381_d_b12;
        var_vschfc3_db13 = assign43750_e42381_d_b13;
        var_vschfc3_db14 = assign43750_e42381_d_b14;
        var_vschfc3_db15 = assign43750_e42381_d_b15;
        var_vschfc3_db16 = assign43750_e42381_d_b16;
        var_vschfc3_db17 = assign43750_e42381_d_b17;
        var_vschfc3_db18 = assign43750_e42381_d_b18;
        var_vschfc3_db19 = assign43750_e42381_d_b19;
        var_vschfc3_db20 = assign43750_e42381_d_b20;
        var_vschfc3_db21 = assign43750_e42381_d_b21;
        var_vschfc3_db22 = assign43750_e42381_d_b22;
        var_vschfc3_db23 = assign43750_e42381_d_b23;
        var_vschfc3_db24 = assign43750_e42381_d_b24;
        var_vschfc3_db25 = assign43750_e42381_d_b25;
        var_vschfc3_db26 = assign43750_e42381_d_b26;
        var_vschfc3_db27 = assign43750_e42381_d_b27;
        var_vschfc3_db28 = assign43750_e42381_d_b28;
        var_vschfc3_db29 = assign43750_e42381_d_b29;
        var_vschfc3_db30 = assign43750_e42381_d_b30;
        var_vschfc3_db31 = assign43750_e42381_d_b31;
        var_vschfc3_db32 = assign43750_e42381_d_b32;
        var_vschfc3_db33 = assign43750_e42381_d_b33;
        var_vschfc3_db34 = assign43750_e42381_d_b34;
        var_vschfc3_db35 = assign43750_e42381_d_b35;

        let (assign43760_e42396, assign43760_e42396_d_n0, assign43760_e42396_d_n1, assign43760_e42396_d_n2, assign43760_e42396_d_n3, assign43760_e42396_d_n4, assign43760_e42396_d_n5, assign43760_e42396_d_n6, assign43760_e42396_d_n7, assign43760_e42396_d_n8, assign43760_e42396_d_n9, assign43760_e42396_d_n10, assign43760_e42396_d_n11, assign43760_e42396_d_n12, assign43760_e42396_d_n13, assign43760_e42396_d_n14, assign43760_e42396_d_n15, assign43760_e42396_d_n16, assign43760_e42396_d_n17, assign43760_e42396_d_n18, assign43760_e42396_d_n19, assign43760_e42396_d_n20, assign43760_e42396_d_n21, assign43760_e42396_d_n22, assign43760_e42396_d_n23, assign43760_e42396_d_n24, assign43760_e42396_d_n25, assign43760_e42396_d_n26, assign43760_e42396_d_n27, assign43760_e42396_d_n28, assign43760_e42396_d_n29, assign43760_e42396_d_b0, assign43760_e42396_d_b1, assign43760_e42396_d_b2, assign43760_e42396_d_b3, assign43760_e42396_d_b4, assign43760_e42396_d_b5, assign43760_e42396_d_b6, assign43760_e42396_d_b7, assign43760_e42396_d_b8, assign43760_e42396_d_b9, assign43760_e42396_d_b10, assign43760_e42396_d_b11, assign43760_e42396_d_b12, assign43760_e42396_d_b13, assign43760_e42396_d_b14, assign43760_e42396_d_b15, assign43760_e42396_d_b16, assign43760_e42396_d_b17, assign43760_e42396_d_b18, assign43760_e42396_d_b19, assign43760_e42396_d_b20, assign43760_e42396_d_b21, assign43760_e42396_d_b22, assign43760_e42396_d_b23, assign43760_e42396_d_b24, assign43760_e42396_d_b25, assign43760_e42396_d_b26, assign43760_e42396_d_b27, assign43760_e42396_d_b28, assign43760_e42396_d_b29, assign43760_e42396_d_b30, assign43760_e42396_d_b31, assign43760_e42396_d_b32, assign43760_e42396_d_b33, assign43760_e42396_d_b34, assign43760_e42396_d_b35,) = {
    if (((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) {
        let assign43760_e42394: f64 = (var_qsch3c * var_vschfc3);
        (assign43760_e42394, (var_qsch3c * var_vschfc3_dn0), (var_qsch3c * var_vschfc3_dn1), (var_qsch3c * var_vschfc3_dn2), (var_qsch3c * var_vschfc3_dn3), (var_qsch3c * var_vschfc3_dn4), (var_qsch3c * var_vschfc3_dn5), (var_qsch3c * var_vschfc3_dn6), (var_qsch3c * var_vschfc3_dn7), (var_qsch3c * var_vschfc3_dn8), (var_qsch3c * var_vschfc3_dn9), (var_qsch3c * var_vschfc3_dn10), (var_qsch3c * var_vschfc3_dn11), (var_qsch3c * var_vschfc3_dn12), (var_qsch3c * var_vschfc3_dn13), (var_qsch3c * var_vschfc3_dn14), (var_qsch3c * var_vschfc3_dn15), (var_qsch3c * var_vschfc3_dn16), (var_qsch3c * var_vschfc3_dn17), (var_qsch3c * var_vschfc3_dn18), (var_qsch3c * var_vschfc3_dn19), (var_qsch3c * var_vschfc3_dn20), (var_qsch3c * var_vschfc3_dn21), (var_qsch3c * var_vschfc3_dn22), (var_qsch3c * var_vschfc3_dn23), (var_qsch3c * var_vschfc3_dn24), (var_qsch3c * var_vschfc3_dn25), (var_qsch3c * var_vschfc3_dn26), (var_qsch3c * var_vschfc3_dn27), (var_qsch3c * var_vschfc3_dn28), (var_qsch3c * var_vschfc3_dn29), (var_qsch3c * var_vschfc3_db0), (var_qsch3c * var_vschfc3_db1), (var_qsch3c * var_vschfc3_db2), (var_qsch3c * var_vschfc3_db3), (var_qsch3c * var_vschfc3_db4), (var_qsch3c * var_vschfc3_db5), (var_qsch3c * var_vschfc3_db6), (var_qsch3c * var_vschfc3_db7), (var_qsch3c * var_vschfc3_db8), (var_qsch3c * var_vschfc3_db9), (var_qsch3c * var_vschfc3_db10), (var_qsch3c * var_vschfc3_db11), (var_qsch3c * var_vschfc3_db12), (var_qsch3c * var_vschfc3_db13), (var_qsch3c * var_vschfc3_db14), (var_qsch3c * var_vschfc3_db15), (var_qsch3c * var_vschfc3_db16), (var_qsch3c * var_vschfc3_db17), (var_qsch3c * var_vschfc3_db18), (var_qsch3c * var_vschfc3_db19), (var_qsch3c * var_vschfc3_db20), (var_qsch3c * var_vschfc3_db21), (var_qsch3c * var_vschfc3_db22), (var_qsch3c * var_vschfc3_db23), (var_qsch3c * var_vschfc3_db24), (var_qsch3c * var_vschfc3_db25), (var_qsch3c * var_vschfc3_db26), (var_qsch3c * var_vschfc3_db27), (var_qsch3c * var_vschfc3_db28), (var_qsch3c * var_vschfc3_db29), (var_qsch3c * var_vschfc3_db30), (var_qsch3c * var_vschfc3_db31), (var_qsch3c * var_vschfc3_db32), (var_qsch3c * var_vschfc3_db33), (var_qsch3c * var_vschfc3_db34), (var_qsch3c * var_vschfc3_db35),)
    } else {
        (var_qsch3, var_qsch3_dn0, var_qsch3_dn1, var_qsch3_dn2, var_qsch3_dn3, var_qsch3_dn4, var_qsch3_dn5, var_qsch3_dn6, var_qsch3_dn7, var_qsch3_dn8, var_qsch3_dn9, var_qsch3_dn10, var_qsch3_dn11, var_qsch3_dn12, var_qsch3_dn13, var_qsch3_dn14, var_qsch3_dn15, var_qsch3_dn16, var_qsch3_dn17, var_qsch3_dn18, var_qsch3_dn19, var_qsch3_dn20, var_qsch3_dn21, var_qsch3_dn22, var_qsch3_dn23, var_qsch3_dn24, var_qsch3_dn25, var_qsch3_dn26, var_qsch3_dn27, var_qsch3_dn28, var_qsch3_dn29, var_qsch3_db0, var_qsch3_db1, var_qsch3_db2, var_qsch3_db3, var_qsch3_db4, var_qsch3_db5, var_qsch3_db6, var_qsch3_db7, var_qsch3_db8, var_qsch3_db9, var_qsch3_db10, var_qsch3_db11, var_qsch3_db12, var_qsch3_db13, var_qsch3_db14, var_qsch3_db15, var_qsch3_db16, var_qsch3_db17, var_qsch3_db18, var_qsch3_db19, var_qsch3_db20, var_qsch3_db21, var_qsch3_db22, var_qsch3_db23, var_qsch3_db24, var_qsch3_db25, var_qsch3_db26, var_qsch3_db27, var_qsch3_db28, var_qsch3_db29, var_qsch3_db30, var_qsch3_db31, var_qsch3_db32, var_qsch3_db33, var_qsch3_db34, var_qsch3_db35,)
    }
};
        var_qsch3 = assign43760_e42396;
        var_qsch3_dn0 = assign43760_e42396_d_n0;
        var_qsch3_dn1 = assign43760_e42396_d_n1;
        var_qsch3_dn2 = assign43760_e42396_d_n2;
        var_qsch3_dn3 = assign43760_e42396_d_n3;
        var_qsch3_dn4 = assign43760_e42396_d_n4;
        var_qsch3_dn5 = assign43760_e42396_d_n5;
        var_qsch3_dn6 = assign43760_e42396_d_n6;
        var_qsch3_dn7 = assign43760_e42396_d_n7;
        var_qsch3_dn8 = assign43760_e42396_d_n8;
        var_qsch3_dn9 = assign43760_e42396_d_n9;
        var_qsch3_dn10 = assign43760_e42396_d_n10;
        var_qsch3_dn11 = assign43760_e42396_d_n11;
        var_qsch3_dn12 = assign43760_e42396_d_n12;
        var_qsch3_dn13 = assign43760_e42396_d_n13;
        var_qsch3_dn14 = assign43760_e42396_d_n14;
        var_qsch3_dn15 = assign43760_e42396_d_n15;
        var_qsch3_dn16 = assign43760_e42396_d_n16;
        var_qsch3_dn17 = assign43760_e42396_d_n17;
        var_qsch3_dn18 = assign43760_e42396_d_n18;
        var_qsch3_dn19 = assign43760_e42396_d_n19;
        var_qsch3_dn20 = assign43760_e42396_d_n20;
        var_qsch3_dn21 = assign43760_e42396_d_n21;
        var_qsch3_dn22 = assign43760_e42396_d_n22;
        var_qsch3_dn23 = assign43760_e42396_d_n23;
        var_qsch3_dn24 = assign43760_e42396_d_n24;
        var_qsch3_dn25 = assign43760_e42396_d_n25;
        var_qsch3_dn26 = assign43760_e42396_d_n26;
        var_qsch3_dn27 = assign43760_e42396_d_n27;
        var_qsch3_dn28 = assign43760_e42396_d_n28;
        var_qsch3_dn29 = assign43760_e42396_d_n29;
        var_qsch3_db0 = assign43760_e42396_d_b0;
        var_qsch3_db1 = assign43760_e42396_d_b1;
        var_qsch3_db2 = assign43760_e42396_d_b2;
        var_qsch3_db3 = assign43760_e42396_d_b3;
        var_qsch3_db4 = assign43760_e42396_d_b4;
        var_qsch3_db5 = assign43760_e42396_d_b5;
        var_qsch3_db6 = assign43760_e42396_d_b6;
        var_qsch3_db7 = assign43760_e42396_d_b7;
        var_qsch3_db8 = assign43760_e42396_d_b8;
        var_qsch3_db9 = assign43760_e42396_d_b9;
        var_qsch3_db10 = assign43760_e42396_d_b10;
        var_qsch3_db11 = assign43760_e42396_d_b11;
        var_qsch3_db12 = assign43760_e42396_d_b12;
        var_qsch3_db13 = assign43760_e42396_d_b13;
        var_qsch3_db14 = assign43760_e42396_d_b14;
        var_qsch3_db15 = assign43760_e42396_d_b15;
        var_qsch3_db16 = assign43760_e42396_d_b16;
        var_qsch3_db17 = assign43760_e42396_d_b17;
        var_qsch3_db18 = assign43760_e42396_d_b18;
        var_qsch3_db19 = assign43760_e42396_d_b19;
        var_qsch3_db20 = assign43760_e42396_d_b20;
        var_qsch3_db21 = assign43760_e42396_d_b21;
        var_qsch3_db22 = assign43760_e42396_d_b22;
        var_qsch3_db23 = assign43760_e42396_d_b23;
        var_qsch3_db24 = assign43760_e42396_d_b24;
        var_qsch3_db25 = assign43760_e42396_d_b25;
        var_qsch3_db26 = assign43760_e42396_d_b26;
        var_qsch3_db27 = assign43760_e42396_d_b27;
        var_qsch3_db28 = assign43760_e42396_d_b28;
        var_qsch3_db29 = assign43760_e42396_d_b29;
        var_qsch3_db30 = assign43760_e42396_d_b30;
        var_qsch3_db31 = assign43760_e42396_d_b31;
        var_qsch3_db32 = assign43760_e42396_d_b32;
        var_qsch3_db33 = assign43760_e42396_d_b33;
        var_qsch3_db34 = assign43760_e42396_d_b34;
        var_qsch3_db35 = assign43760_e42396_d_b35;

        let assign43770_e42399: f64 = if p.p309 >= 4.0 { 1.0 } else { 0.0 };
        var_guard477 = assign43770_e42399;

        let (assign43780_e42424,) = {
    if ((((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) && (var_guard477 != 0.0)) {
        let assign43780_e42414: f64 = (5.0 * var_qsch3c);
        let assign43780_e42417: f64 = (8.0 * p.p306);
        let assign43780_e42420: f64 = (1.0 - p.p308);
        let assign43780_e42421: f64 = (assign43780_e42417 * assign43780_e42420);
        let assign43780_e42422: f64 = (assign43780_e42414 / assign43780_e42421);
        (assign43780_e42422,)
    } else {
        (var_qsch4c,)
    }
};
        var_qsch4c = assign43780_e42424;

        let (assign43790_e42441, assign43790_e42441_d_n0, assign43790_e42441_d_n1, assign43790_e42441_d_n2, assign43790_e42441_d_n3, assign43790_e42441_d_n4, assign43790_e42441_d_n5, assign43790_e42441_d_n6, assign43790_e42441_d_n7, assign43790_e42441_d_n8, assign43790_e42441_d_n9, assign43790_e42441_d_n10, assign43790_e42441_d_n11, assign43790_e42441_d_n12, assign43790_e42441_d_n13, assign43790_e42441_d_n14, assign43790_e42441_d_n15, assign43790_e42441_d_n16, assign43790_e42441_d_n17, assign43790_e42441_d_n18, assign43790_e42441_d_n19, assign43790_e42441_d_n20, assign43790_e42441_d_n21, assign43790_e42441_d_n22, assign43790_e42441_d_n23, assign43790_e42441_d_n24, assign43790_e42441_d_n25, assign43790_e42441_d_n26, assign43790_e42441_d_n27, assign43790_e42441_d_n28, assign43790_e42441_d_n29, assign43790_e42441_d_b0, assign43790_e42441_d_b1, assign43790_e42441_d_b2, assign43790_e42441_d_b3, assign43790_e42441_d_b4, assign43790_e42441_d_b5, assign43790_e42441_d_b6, assign43790_e42441_d_b7, assign43790_e42441_d_b8, assign43790_e42441_d_b9, assign43790_e42441_d_b10, assign43790_e42441_d_b11, assign43790_e42441_d_b12, assign43790_e42441_d_b13, assign43790_e42441_d_b14, assign43790_e42441_d_b15, assign43790_e42441_d_b16, assign43790_e42441_d_b17, assign43790_e42441_d_b18, assign43790_e42441_d_b19, assign43790_e42441_d_b20, assign43790_e42441_d_b21, assign43790_e42441_d_b22, assign43790_e42441_d_b23, assign43790_e42441_d_b24, assign43790_e42441_d_b25, assign43790_e42441_d_b26, assign43790_e42441_d_b27, assign43790_e42441_d_b28, assign43790_e42441_d_b29, assign43790_e42441_d_b30, assign43790_e42441_d_b31, assign43790_e42441_d_b32, assign43790_e42441_d_b33, assign43790_e42441_d_b34, assign43790_e42441_d_b35,) = {
    if ((((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) && (var_guard477 != 0.0)) {
        let assign43790_e42439: f64 = (var_vschfc3 * var_vschfc1);
        (assign43790_e42439, ((var_vschfc3_dn0 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn0)), ((var_vschfc3_dn1 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn1)), ((var_vschfc3_dn2 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn2)), ((var_vschfc3_dn3 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn3)), ((var_vschfc3_dn4 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn4)), ((var_vschfc3_dn5 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn5)), ((var_vschfc3_dn6 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn6)), ((var_vschfc3_dn7 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn7)), ((var_vschfc3_dn8 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn8)), ((var_vschfc3_dn9 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn9)), ((var_vschfc3_dn10 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn10)), ((var_vschfc3_dn11 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn11)), ((var_vschfc3_dn12 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn12)), ((var_vschfc3_dn13 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn13)), ((var_vschfc3_dn14 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn14)), ((var_vschfc3_dn15 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn15)), ((var_vschfc3_dn16 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn16)), ((var_vschfc3_dn17 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn17)), ((var_vschfc3_dn18 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn18)), ((var_vschfc3_dn19 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn19)), ((var_vschfc3_dn20 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn20)), ((var_vschfc3_dn21 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn21)), ((var_vschfc3_dn22 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn22)), ((var_vschfc3_dn23 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn23)), ((var_vschfc3_dn24 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn24)), ((var_vschfc3_dn25 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn25)), ((var_vschfc3_dn26 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn26)), ((var_vschfc3_dn27 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn27)), ((var_vschfc3_dn28 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn28)), ((var_vschfc3_dn29 * var_vschfc1) + (var_vschfc3 * var_vschfc1_dn29)), ((var_vschfc3_db0 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db0)), ((var_vschfc3_db1 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db1)), ((var_vschfc3_db2 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db2)), ((var_vschfc3_db3 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db3)), ((var_vschfc3_db4 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db4)), ((var_vschfc3_db5 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db5)), ((var_vschfc3_db6 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db6)), ((var_vschfc3_db7 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db7)), ((var_vschfc3_db8 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db8)), ((var_vschfc3_db9 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db9)), ((var_vschfc3_db10 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db10)), ((var_vschfc3_db11 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db11)), ((var_vschfc3_db12 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db12)), ((var_vschfc3_db13 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db13)), ((var_vschfc3_db14 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db14)), ((var_vschfc3_db15 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db15)), ((var_vschfc3_db16 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db16)), ((var_vschfc3_db17 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db17)), ((var_vschfc3_db18 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db18)), ((var_vschfc3_db19 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db19)), ((var_vschfc3_db20 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db20)), ((var_vschfc3_db21 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db21)), ((var_vschfc3_db22 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db22)), ((var_vschfc3_db23 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db23)), ((var_vschfc3_db24 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db24)), ((var_vschfc3_db25 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db25)), ((var_vschfc3_db26 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db26)), ((var_vschfc3_db27 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db27)), ((var_vschfc3_db28 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db28)), ((var_vschfc3_db29 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db29)), ((var_vschfc3_db30 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db30)), ((var_vschfc3_db31 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db31)), ((var_vschfc3_db32 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db32)), ((var_vschfc3_db33 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db33)), ((var_vschfc3_db34 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db34)), ((var_vschfc3_db35 * var_vschfc1) + (var_vschfc3 * var_vschfc1_db35)),)
    } else {
        (var_vschfc4, var_vschfc4_dn0, var_vschfc4_dn1, var_vschfc4_dn2, var_vschfc4_dn3, var_vschfc4_dn4, var_vschfc4_dn5, var_vschfc4_dn6, var_vschfc4_dn7, var_vschfc4_dn8, var_vschfc4_dn9, var_vschfc4_dn10, var_vschfc4_dn11, var_vschfc4_dn12, var_vschfc4_dn13, var_vschfc4_dn14, var_vschfc4_dn15, var_vschfc4_dn16, var_vschfc4_dn17, var_vschfc4_dn18, var_vschfc4_dn19, var_vschfc4_dn20, var_vschfc4_dn21, var_vschfc4_dn22, var_vschfc4_dn23, var_vschfc4_dn24, var_vschfc4_dn25, var_vschfc4_dn26, var_vschfc4_dn27, var_vschfc4_dn28, var_vschfc4_dn29, var_vschfc4_db0, var_vschfc4_db1, var_vschfc4_db2, var_vschfc4_db3, var_vschfc4_db4, var_vschfc4_db5, var_vschfc4_db6, var_vschfc4_db7, var_vschfc4_db8, var_vschfc4_db9, var_vschfc4_db10, var_vschfc4_db11, var_vschfc4_db12, var_vschfc4_db13, var_vschfc4_db14, var_vschfc4_db15, var_vschfc4_db16, var_vschfc4_db17, var_vschfc4_db18, var_vschfc4_db19, var_vschfc4_db20, var_vschfc4_db21, var_vschfc4_db22, var_vschfc4_db23, var_vschfc4_db24, var_vschfc4_db25, var_vschfc4_db26, var_vschfc4_db27, var_vschfc4_db28, var_vschfc4_db29, var_vschfc4_db30, var_vschfc4_db31, var_vschfc4_db32, var_vschfc4_db33, var_vschfc4_db34, var_vschfc4_db35,)
    }
};
        var_vschfc4 = assign43790_e42441;
        var_vschfc4_dn0 = assign43790_e42441_d_n0;
        var_vschfc4_dn1 = assign43790_e42441_d_n1;
        var_vschfc4_dn2 = assign43790_e42441_d_n2;
        var_vschfc4_dn3 = assign43790_e42441_d_n3;
        var_vschfc4_dn4 = assign43790_e42441_d_n4;
        var_vschfc4_dn5 = assign43790_e42441_d_n5;
        var_vschfc4_dn6 = assign43790_e42441_d_n6;
        var_vschfc4_dn7 = assign43790_e42441_d_n7;
        var_vschfc4_dn8 = assign43790_e42441_d_n8;
        var_vschfc4_dn9 = assign43790_e42441_d_n9;
        var_vschfc4_dn10 = assign43790_e42441_d_n10;
        var_vschfc4_dn11 = assign43790_e42441_d_n11;
        var_vschfc4_dn12 = assign43790_e42441_d_n12;
        var_vschfc4_dn13 = assign43790_e42441_d_n13;
        var_vschfc4_dn14 = assign43790_e42441_d_n14;
        var_vschfc4_dn15 = assign43790_e42441_d_n15;
        var_vschfc4_dn16 = assign43790_e42441_d_n16;
        var_vschfc4_dn17 = assign43790_e42441_d_n17;
        var_vschfc4_dn18 = assign43790_e42441_d_n18;
        var_vschfc4_dn19 = assign43790_e42441_d_n19;
        var_vschfc4_dn20 = assign43790_e42441_d_n20;
        var_vschfc4_dn21 = assign43790_e42441_d_n21;
        var_vschfc4_dn22 = assign43790_e42441_d_n22;
        var_vschfc4_dn23 = assign43790_e42441_d_n23;
        var_vschfc4_dn24 = assign43790_e42441_d_n24;
        var_vschfc4_dn25 = assign43790_e42441_d_n25;
        var_vschfc4_dn26 = assign43790_e42441_d_n26;
        var_vschfc4_dn27 = assign43790_e42441_d_n27;
        var_vschfc4_dn28 = assign43790_e42441_d_n28;
        var_vschfc4_dn29 = assign43790_e42441_d_n29;
        var_vschfc4_db0 = assign43790_e42441_d_b0;
        var_vschfc4_db1 = assign43790_e42441_d_b1;
        var_vschfc4_db2 = assign43790_e42441_d_b2;
        var_vschfc4_db3 = assign43790_e42441_d_b3;
        var_vschfc4_db4 = assign43790_e42441_d_b4;
        var_vschfc4_db5 = assign43790_e42441_d_b5;
        var_vschfc4_db6 = assign43790_e42441_d_b6;
        var_vschfc4_db7 = assign43790_e42441_d_b7;
        var_vschfc4_db8 = assign43790_e42441_d_b8;
        var_vschfc4_db9 = assign43790_e42441_d_b9;
        var_vschfc4_db10 = assign43790_e42441_d_b10;
        var_vschfc4_db11 = assign43790_e42441_d_b11;
        var_vschfc4_db12 = assign43790_e42441_d_b12;
        var_vschfc4_db13 = assign43790_e42441_d_b13;
        var_vschfc4_db14 = assign43790_e42441_d_b14;
        var_vschfc4_db15 = assign43790_e42441_d_b15;
        var_vschfc4_db16 = assign43790_e42441_d_b16;
        var_vschfc4_db17 = assign43790_e42441_d_b17;
        var_vschfc4_db18 = assign43790_e42441_d_b18;
        var_vschfc4_db19 = assign43790_e42441_d_b19;
        var_vschfc4_db20 = assign43790_e42441_d_b20;
        var_vschfc4_db21 = assign43790_e42441_d_b21;
        var_vschfc4_db22 = assign43790_e42441_d_b22;
        var_vschfc4_db23 = assign43790_e42441_d_b23;
        var_vschfc4_db24 = assign43790_e42441_d_b24;
        var_vschfc4_db25 = assign43790_e42441_d_b25;
        var_vschfc4_db26 = assign43790_e42441_d_b26;
        var_vschfc4_db27 = assign43790_e42441_d_b27;
        var_vschfc4_db28 = assign43790_e42441_d_b28;
        var_vschfc4_db29 = assign43790_e42441_d_b29;
        var_vschfc4_db30 = assign43790_e42441_d_b30;
        var_vschfc4_db31 = assign43790_e42441_d_b31;
        var_vschfc4_db32 = assign43790_e42441_d_b32;
        var_vschfc4_db33 = assign43790_e42441_d_b33;
        var_vschfc4_db34 = assign43790_e42441_d_b34;
        var_vschfc4_db35 = assign43790_e42441_d_b35;

        let (assign43800_e42458, assign43800_e42458_d_n0, assign43800_e42458_d_n1, assign43800_e42458_d_n2, assign43800_e42458_d_n3, assign43800_e42458_d_n4, assign43800_e42458_d_n5, assign43800_e42458_d_n6, assign43800_e42458_d_n7, assign43800_e42458_d_n8, assign43800_e42458_d_n9, assign43800_e42458_d_n10, assign43800_e42458_d_n11, assign43800_e42458_d_n12, assign43800_e42458_d_n13, assign43800_e42458_d_n14, assign43800_e42458_d_n15, assign43800_e42458_d_n16, assign43800_e42458_d_n17, assign43800_e42458_d_n18, assign43800_e42458_d_n19, assign43800_e42458_d_n20, assign43800_e42458_d_n21, assign43800_e42458_d_n22, assign43800_e42458_d_n23, assign43800_e42458_d_n24, assign43800_e42458_d_n25, assign43800_e42458_d_n26, assign43800_e42458_d_n27, assign43800_e42458_d_n28, assign43800_e42458_d_n29, assign43800_e42458_d_b0, assign43800_e42458_d_b1, assign43800_e42458_d_b2, assign43800_e42458_d_b3, assign43800_e42458_d_b4, assign43800_e42458_d_b5, assign43800_e42458_d_b6, assign43800_e42458_d_b7, assign43800_e42458_d_b8, assign43800_e42458_d_b9, assign43800_e42458_d_b10, assign43800_e42458_d_b11, assign43800_e42458_d_b12, assign43800_e42458_d_b13, assign43800_e42458_d_b14, assign43800_e42458_d_b15, assign43800_e42458_d_b16, assign43800_e42458_d_b17, assign43800_e42458_d_b18, assign43800_e42458_d_b19, assign43800_e42458_d_b20, assign43800_e42458_d_b21, assign43800_e42458_d_b22, assign43800_e42458_d_b23, assign43800_e42458_d_b24, assign43800_e42458_d_b25, assign43800_e42458_d_b26, assign43800_e42458_d_b27, assign43800_e42458_d_b28, assign43800_e42458_d_b29, assign43800_e42458_d_b30, assign43800_e42458_d_b31, assign43800_e42458_d_b32, assign43800_e42458_d_b33, assign43800_e42458_d_b34, assign43800_e42458_d_b35,) = {
    if ((((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) && (var_guard477 != 0.0)) {
        let assign43800_e42456: f64 = (var_qsch4c * var_vschfc4);
        (assign43800_e42456, (var_qsch4c * var_vschfc4_dn0), (var_qsch4c * var_vschfc4_dn1), (var_qsch4c * var_vschfc4_dn2), (var_qsch4c * var_vschfc4_dn3), (var_qsch4c * var_vschfc4_dn4), (var_qsch4c * var_vschfc4_dn5), (var_qsch4c * var_vschfc4_dn6), (var_qsch4c * var_vschfc4_dn7), (var_qsch4c * var_vschfc4_dn8), (var_qsch4c * var_vschfc4_dn9), (var_qsch4c * var_vschfc4_dn10), (var_qsch4c * var_vschfc4_dn11), (var_qsch4c * var_vschfc4_dn12), (var_qsch4c * var_vschfc4_dn13), (var_qsch4c * var_vschfc4_dn14), (var_qsch4c * var_vschfc4_dn15), (var_qsch4c * var_vschfc4_dn16), (var_qsch4c * var_vschfc4_dn17), (var_qsch4c * var_vschfc4_dn18), (var_qsch4c * var_vschfc4_dn19), (var_qsch4c * var_vschfc4_dn20), (var_qsch4c * var_vschfc4_dn21), (var_qsch4c * var_vschfc4_dn22), (var_qsch4c * var_vschfc4_dn23), (var_qsch4c * var_vschfc4_dn24), (var_qsch4c * var_vschfc4_dn25), (var_qsch4c * var_vschfc4_dn26), (var_qsch4c * var_vschfc4_dn27), (var_qsch4c * var_vschfc4_dn28), (var_qsch4c * var_vschfc4_dn29), (var_qsch4c * var_vschfc4_db0), (var_qsch4c * var_vschfc4_db1), (var_qsch4c * var_vschfc4_db2), (var_qsch4c * var_vschfc4_db3), (var_qsch4c * var_vschfc4_db4), (var_qsch4c * var_vschfc4_db5), (var_qsch4c * var_vschfc4_db6), (var_qsch4c * var_vschfc4_db7), (var_qsch4c * var_vschfc4_db8), (var_qsch4c * var_vschfc4_db9), (var_qsch4c * var_vschfc4_db10), (var_qsch4c * var_vschfc4_db11), (var_qsch4c * var_vschfc4_db12), (var_qsch4c * var_vschfc4_db13), (var_qsch4c * var_vschfc4_db14), (var_qsch4c * var_vschfc4_db15), (var_qsch4c * var_vschfc4_db16), (var_qsch4c * var_vschfc4_db17), (var_qsch4c * var_vschfc4_db18), (var_qsch4c * var_vschfc4_db19), (var_qsch4c * var_vschfc4_db20), (var_qsch4c * var_vschfc4_db21), (var_qsch4c * var_vschfc4_db22), (var_qsch4c * var_vschfc4_db23), (var_qsch4c * var_vschfc4_db24), (var_qsch4c * var_vschfc4_db25), (var_qsch4c * var_vschfc4_db26), (var_qsch4c * var_vschfc4_db27), (var_qsch4c * var_vschfc4_db28), (var_qsch4c * var_vschfc4_db29), (var_qsch4c * var_vschfc4_db30), (var_qsch4c * var_vschfc4_db31), (var_qsch4c * var_vschfc4_db32), (var_qsch4c * var_vschfc4_db33), (var_qsch4c * var_vschfc4_db34), (var_qsch4c * var_vschfc4_db35),)
    } else {
        (var_qsch4, var_qsch4_dn0, var_qsch4_dn1, var_qsch4_dn2, var_qsch4_dn3, var_qsch4_dn4, var_qsch4_dn5, var_qsch4_dn6, var_qsch4_dn7, var_qsch4_dn8, var_qsch4_dn9, var_qsch4_dn10, var_qsch4_dn11, var_qsch4_dn12, var_qsch4_dn13, var_qsch4_dn14, var_qsch4_dn15, var_qsch4_dn16, var_qsch4_dn17, var_qsch4_dn18, var_qsch4_dn19, var_qsch4_dn20, var_qsch4_dn21, var_qsch4_dn22, var_qsch4_dn23, var_qsch4_dn24, var_qsch4_dn25, var_qsch4_dn26, var_qsch4_dn27, var_qsch4_dn28, var_qsch4_dn29, var_qsch4_db0, var_qsch4_db1, var_qsch4_db2, var_qsch4_db3, var_qsch4_db4, var_qsch4_db5, var_qsch4_db6, var_qsch4_db7, var_qsch4_db8, var_qsch4_db9, var_qsch4_db10, var_qsch4_db11, var_qsch4_db12, var_qsch4_db13, var_qsch4_db14, var_qsch4_db15, var_qsch4_db16, var_qsch4_db17, var_qsch4_db18, var_qsch4_db19, var_qsch4_db20, var_qsch4_db21, var_qsch4_db22, var_qsch4_db23, var_qsch4_db24, var_qsch4_db25, var_qsch4_db26, var_qsch4_db27, var_qsch4_db28, var_qsch4_db29, var_qsch4_db30, var_qsch4_db31, var_qsch4_db32, var_qsch4_db33, var_qsch4_db34, var_qsch4_db35,)
    }
};
        var_qsch4 = assign43800_e42458;
        var_qsch4_dn0 = assign43800_e42458_d_n0;
        var_qsch4_dn1 = assign43800_e42458_d_n1;
        var_qsch4_dn2 = assign43800_e42458_d_n2;
        var_qsch4_dn3 = assign43800_e42458_d_n3;
        var_qsch4_dn4 = assign43800_e42458_d_n4;
        var_qsch4_dn5 = assign43800_e42458_d_n5;
        var_qsch4_dn6 = assign43800_e42458_d_n6;
        var_qsch4_dn7 = assign43800_e42458_d_n7;
        var_qsch4_dn8 = assign43800_e42458_d_n8;
        var_qsch4_dn9 = assign43800_e42458_d_n9;
        var_qsch4_dn10 = assign43800_e42458_d_n10;
        var_qsch4_dn11 = assign43800_e42458_d_n11;
        var_qsch4_dn12 = assign43800_e42458_d_n12;
        var_qsch4_dn13 = assign43800_e42458_d_n13;
        var_qsch4_dn14 = assign43800_e42458_d_n14;
        var_qsch4_dn15 = assign43800_e42458_d_n15;
        var_qsch4_dn16 = assign43800_e42458_d_n16;
        var_qsch4_dn17 = assign43800_e42458_d_n17;
        var_qsch4_dn18 = assign43800_e42458_d_n18;
        var_qsch4_dn19 = assign43800_e42458_d_n19;
        var_qsch4_dn20 = assign43800_e42458_d_n20;
        var_qsch4_dn21 = assign43800_e42458_d_n21;
        var_qsch4_dn22 = assign43800_e42458_d_n22;
        var_qsch4_dn23 = assign43800_e42458_d_n23;
        var_qsch4_dn24 = assign43800_e42458_d_n24;
        var_qsch4_dn25 = assign43800_e42458_d_n25;
        var_qsch4_dn26 = assign43800_e42458_d_n26;
        var_qsch4_dn27 = assign43800_e42458_d_n27;
        var_qsch4_dn28 = assign43800_e42458_d_n28;
        var_qsch4_dn29 = assign43800_e42458_d_n29;
        var_qsch4_db0 = assign43800_e42458_d_b0;
        var_qsch4_db1 = assign43800_e42458_d_b1;
        var_qsch4_db2 = assign43800_e42458_d_b2;
        var_qsch4_db3 = assign43800_e42458_d_b3;
        var_qsch4_db4 = assign43800_e42458_d_b4;
        var_qsch4_db5 = assign43800_e42458_d_b5;
        var_qsch4_db6 = assign43800_e42458_d_b6;
        var_qsch4_db7 = assign43800_e42458_d_b7;
        var_qsch4_db8 = assign43800_e42458_d_b8;
        var_qsch4_db9 = assign43800_e42458_d_b9;
        var_qsch4_db10 = assign43800_e42458_d_b10;
        var_qsch4_db11 = assign43800_e42458_d_b11;
        var_qsch4_db12 = assign43800_e42458_d_b12;
        var_qsch4_db13 = assign43800_e42458_d_b13;
        var_qsch4_db14 = assign43800_e42458_d_b14;
        var_qsch4_db15 = assign43800_e42458_d_b15;
        var_qsch4_db16 = assign43800_e42458_d_b16;
        var_qsch4_db17 = assign43800_e42458_d_b17;
        var_qsch4_db18 = assign43800_e42458_d_b18;
        var_qsch4_db19 = assign43800_e42458_d_b19;
        var_qsch4_db20 = assign43800_e42458_d_b20;
        var_qsch4_db21 = assign43800_e42458_d_b21;
        var_qsch4_db22 = assign43800_e42458_d_b22;
        var_qsch4_db23 = assign43800_e42458_d_b23;
        var_qsch4_db24 = assign43800_e42458_d_b24;
        var_qsch4_db25 = assign43800_e42458_d_b25;
        var_qsch4_db26 = assign43800_e42458_d_b26;
        var_qsch4_db27 = assign43800_e42458_d_b27;
        var_qsch4_db28 = assign43800_e42458_d_b28;
        var_qsch4_db29 = assign43800_e42458_d_b29;
        var_qsch4_db30 = assign43800_e42458_d_b30;
        var_qsch4_db31 = assign43800_e42458_d_b31;
        var_qsch4_db32 = assign43800_e42458_d_b32;
        var_qsch4_db33 = assign43800_e42458_d_b33;
        var_qsch4_db34 = assign43800_e42458_d_b34;
        var_qsch4_db35 = assign43800_e42458_d_b35;

        let assign43810_e42461: f64 = if p.p309 >= 5.0 { 1.0 } else { 0.0 };
        var_guard478 = assign43810_e42461;

        let (assign43820_e42488,) = {
    if (((((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) && (var_guard477 != 0.0)) && (var_guard478 != 0.0)) {
        let assign43820_e42478: f64 = (7.0 * var_qsch4c);
        let assign43820_e42481: f64 = (10.0 * p.p306);
        let assign43820_e42484: f64 = (1.0 - p.p308);
        let assign43820_e42485: f64 = (assign43820_e42481 * assign43820_e42484);
        let assign43820_e42486: f64 = (assign43820_e42478 / assign43820_e42485);
        (assign43820_e42486,)
    } else {
        (var_qsch5c,)
    }
};
        var_qsch5c = assign43820_e42488;

        let (assign43830_e42507, assign43830_e42507_d_n0, assign43830_e42507_d_n1, assign43830_e42507_d_n2, assign43830_e42507_d_n3, assign43830_e42507_d_n4, assign43830_e42507_d_n5, assign43830_e42507_d_n6, assign43830_e42507_d_n7, assign43830_e42507_d_n8, assign43830_e42507_d_n9, assign43830_e42507_d_n10, assign43830_e42507_d_n11, assign43830_e42507_d_n12, assign43830_e42507_d_n13, assign43830_e42507_d_n14, assign43830_e42507_d_n15, assign43830_e42507_d_n16, assign43830_e42507_d_n17, assign43830_e42507_d_n18, assign43830_e42507_d_n19, assign43830_e42507_d_n20, assign43830_e42507_d_n21, assign43830_e42507_d_n22, assign43830_e42507_d_n23, assign43830_e42507_d_n24, assign43830_e42507_d_n25, assign43830_e42507_d_n26, assign43830_e42507_d_n27, assign43830_e42507_d_n28, assign43830_e42507_d_n29, assign43830_e42507_d_b0, assign43830_e42507_d_b1, assign43830_e42507_d_b2, assign43830_e42507_d_b3, assign43830_e42507_d_b4, assign43830_e42507_d_b5, assign43830_e42507_d_b6, assign43830_e42507_d_b7, assign43830_e42507_d_b8, assign43830_e42507_d_b9, assign43830_e42507_d_b10, assign43830_e42507_d_b11, assign43830_e42507_d_b12, assign43830_e42507_d_b13, assign43830_e42507_d_b14, assign43830_e42507_d_b15, assign43830_e42507_d_b16, assign43830_e42507_d_b17, assign43830_e42507_d_b18, assign43830_e42507_d_b19, assign43830_e42507_d_b20, assign43830_e42507_d_b21, assign43830_e42507_d_b22, assign43830_e42507_d_b23, assign43830_e42507_d_b24, assign43830_e42507_d_b25, assign43830_e42507_d_b26, assign43830_e42507_d_b27, assign43830_e42507_d_b28, assign43830_e42507_d_b29, assign43830_e42507_d_b30, assign43830_e42507_d_b31, assign43830_e42507_d_b32, assign43830_e42507_d_b33, assign43830_e42507_d_b34, assign43830_e42507_d_b35,) = {
    if (((((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) && (var_guard477 != 0.0)) && (var_guard478 != 0.0)) {
        let assign43830_e42505: f64 = (var_vschfc4 * var_vschfc1);
        (assign43830_e42505, ((var_vschfc4_dn0 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn0)), ((var_vschfc4_dn1 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn1)), ((var_vschfc4_dn2 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn2)), ((var_vschfc4_dn3 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn3)), ((var_vschfc4_dn4 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn4)), ((var_vschfc4_dn5 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn5)), ((var_vschfc4_dn6 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn6)), ((var_vschfc4_dn7 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn7)), ((var_vschfc4_dn8 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn8)), ((var_vschfc4_dn9 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn9)), ((var_vschfc4_dn10 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn10)), ((var_vschfc4_dn11 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn11)), ((var_vschfc4_dn12 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn12)), ((var_vschfc4_dn13 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn13)), ((var_vschfc4_dn14 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn14)), ((var_vschfc4_dn15 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn15)), ((var_vschfc4_dn16 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn16)), ((var_vschfc4_dn17 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn17)), ((var_vschfc4_dn18 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn18)), ((var_vschfc4_dn19 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn19)), ((var_vschfc4_dn20 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn20)), ((var_vschfc4_dn21 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn21)), ((var_vschfc4_dn22 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn22)), ((var_vschfc4_dn23 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn23)), ((var_vschfc4_dn24 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn24)), ((var_vschfc4_dn25 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn25)), ((var_vschfc4_dn26 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn26)), ((var_vschfc4_dn27 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn27)), ((var_vschfc4_dn28 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn28)), ((var_vschfc4_dn29 * var_vschfc1) + (var_vschfc4 * var_vschfc1_dn29)), ((var_vschfc4_db0 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db0)), ((var_vschfc4_db1 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db1)), ((var_vschfc4_db2 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db2)), ((var_vschfc4_db3 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db3)), ((var_vschfc4_db4 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db4)), ((var_vschfc4_db5 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db5)), ((var_vschfc4_db6 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db6)), ((var_vschfc4_db7 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db7)), ((var_vschfc4_db8 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db8)), ((var_vschfc4_db9 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db9)), ((var_vschfc4_db10 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db10)), ((var_vschfc4_db11 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db11)), ((var_vschfc4_db12 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db12)), ((var_vschfc4_db13 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db13)), ((var_vschfc4_db14 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db14)), ((var_vschfc4_db15 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db15)), ((var_vschfc4_db16 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db16)), ((var_vschfc4_db17 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db17)), ((var_vschfc4_db18 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db18)), ((var_vschfc4_db19 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db19)), ((var_vschfc4_db20 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db20)), ((var_vschfc4_db21 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db21)), ((var_vschfc4_db22 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db22)), ((var_vschfc4_db23 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db23)), ((var_vschfc4_db24 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db24)), ((var_vschfc4_db25 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db25)), ((var_vschfc4_db26 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db26)), ((var_vschfc4_db27 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db27)), ((var_vschfc4_db28 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db28)), ((var_vschfc4_db29 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db29)), ((var_vschfc4_db30 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db30)), ((var_vschfc4_db31 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db31)), ((var_vschfc4_db32 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db32)), ((var_vschfc4_db33 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db33)), ((var_vschfc4_db34 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db34)), ((var_vschfc4_db35 * var_vschfc1) + (var_vschfc4 * var_vschfc1_db35)),)
    } else {
        (var_vschfc5, var_vschfc5_dn0, var_vschfc5_dn1, var_vschfc5_dn2, var_vschfc5_dn3, var_vschfc5_dn4, var_vschfc5_dn5, var_vschfc5_dn6, var_vschfc5_dn7, var_vschfc5_dn8, var_vschfc5_dn9, var_vschfc5_dn10, var_vschfc5_dn11, var_vschfc5_dn12, var_vschfc5_dn13, var_vschfc5_dn14, var_vschfc5_dn15, var_vschfc5_dn16, var_vschfc5_dn17, var_vschfc5_dn18, var_vschfc5_dn19, var_vschfc5_dn20, var_vschfc5_dn21, var_vschfc5_dn22, var_vschfc5_dn23, var_vschfc5_dn24, var_vschfc5_dn25, var_vschfc5_dn26, var_vschfc5_dn27, var_vschfc5_dn28, var_vschfc5_dn29, var_vschfc5_db0, var_vschfc5_db1, var_vschfc5_db2, var_vschfc5_db3, var_vschfc5_db4, var_vschfc5_db5, var_vschfc5_db6, var_vschfc5_db7, var_vschfc5_db8, var_vschfc5_db9, var_vschfc5_db10, var_vschfc5_db11, var_vschfc5_db12, var_vschfc5_db13, var_vschfc5_db14, var_vschfc5_db15, var_vschfc5_db16, var_vschfc5_db17, var_vschfc5_db18, var_vschfc5_db19, var_vschfc5_db20, var_vschfc5_db21, var_vschfc5_db22, var_vschfc5_db23, var_vschfc5_db24, var_vschfc5_db25, var_vschfc5_db26, var_vschfc5_db27, var_vschfc5_db28, var_vschfc5_db29, var_vschfc5_db30, var_vschfc5_db31, var_vschfc5_db32, var_vschfc5_db33, var_vschfc5_db34, var_vschfc5_db35,)
    }
};
        var_vschfc5 = assign43830_e42507;
        var_vschfc5_dn0 = assign43830_e42507_d_n0;
        var_vschfc5_dn1 = assign43830_e42507_d_n1;
        var_vschfc5_dn2 = assign43830_e42507_d_n2;
        var_vschfc5_dn3 = assign43830_e42507_d_n3;
        var_vschfc5_dn4 = assign43830_e42507_d_n4;
        var_vschfc5_dn5 = assign43830_e42507_d_n5;
        var_vschfc5_dn6 = assign43830_e42507_d_n6;
        var_vschfc5_dn7 = assign43830_e42507_d_n7;
        var_vschfc5_dn8 = assign43830_e42507_d_n8;
        var_vschfc5_dn9 = assign43830_e42507_d_n9;
        var_vschfc5_dn10 = assign43830_e42507_d_n10;
        var_vschfc5_dn11 = assign43830_e42507_d_n11;
        var_vschfc5_dn12 = assign43830_e42507_d_n12;
        var_vschfc5_dn13 = assign43830_e42507_d_n13;
        var_vschfc5_dn14 = assign43830_e42507_d_n14;
        var_vschfc5_dn15 = assign43830_e42507_d_n15;
        var_vschfc5_dn16 = assign43830_e42507_d_n16;
        var_vschfc5_dn17 = assign43830_e42507_d_n17;
        var_vschfc5_dn18 = assign43830_e42507_d_n18;
        var_vschfc5_dn19 = assign43830_e42507_d_n19;
        var_vschfc5_dn20 = assign43830_e42507_d_n20;
        var_vschfc5_dn21 = assign43830_e42507_d_n21;
        var_vschfc5_dn22 = assign43830_e42507_d_n22;
        var_vschfc5_dn23 = assign43830_e42507_d_n23;
        var_vschfc5_dn24 = assign43830_e42507_d_n24;
        var_vschfc5_dn25 = assign43830_e42507_d_n25;
        var_vschfc5_dn26 = assign43830_e42507_d_n26;
        var_vschfc5_dn27 = assign43830_e42507_d_n27;
        var_vschfc5_dn28 = assign43830_e42507_d_n28;
        var_vschfc5_dn29 = assign43830_e42507_d_n29;
        var_vschfc5_db0 = assign43830_e42507_d_b0;
        var_vschfc5_db1 = assign43830_e42507_d_b1;
        var_vschfc5_db2 = assign43830_e42507_d_b2;
        var_vschfc5_db3 = assign43830_e42507_d_b3;
        var_vschfc5_db4 = assign43830_e42507_d_b4;
        var_vschfc5_db5 = assign43830_e42507_d_b5;
        var_vschfc5_db6 = assign43830_e42507_d_b6;
        var_vschfc5_db7 = assign43830_e42507_d_b7;
        var_vschfc5_db8 = assign43830_e42507_d_b8;
        var_vschfc5_db9 = assign43830_e42507_d_b9;
        var_vschfc5_db10 = assign43830_e42507_d_b10;
        var_vschfc5_db11 = assign43830_e42507_d_b11;
        var_vschfc5_db12 = assign43830_e42507_d_b12;
        var_vschfc5_db13 = assign43830_e42507_d_b13;
        var_vschfc5_db14 = assign43830_e42507_d_b14;
        var_vschfc5_db15 = assign43830_e42507_d_b15;
        var_vschfc5_db16 = assign43830_e42507_d_b16;
        var_vschfc5_db17 = assign43830_e42507_d_b17;
        var_vschfc5_db18 = assign43830_e42507_d_b18;
        var_vschfc5_db19 = assign43830_e42507_d_b19;
        var_vschfc5_db20 = assign43830_e42507_d_b20;
        var_vschfc5_db21 = assign43830_e42507_d_b21;
        var_vschfc5_db22 = assign43830_e42507_d_b22;
        var_vschfc5_db23 = assign43830_e42507_d_b23;
        var_vschfc5_db24 = assign43830_e42507_d_b24;
        var_vschfc5_db25 = assign43830_e42507_d_b25;
        var_vschfc5_db26 = assign43830_e42507_d_b26;
        var_vschfc5_db27 = assign43830_e42507_d_b27;
        var_vschfc5_db28 = assign43830_e42507_d_b28;
        var_vschfc5_db29 = assign43830_e42507_d_b29;
        var_vschfc5_db30 = assign43830_e42507_d_b30;
        var_vschfc5_db31 = assign43830_e42507_d_b31;
        var_vschfc5_db32 = assign43830_e42507_d_b32;
        var_vschfc5_db33 = assign43830_e42507_d_b33;
        var_vschfc5_db34 = assign43830_e42507_d_b34;
        var_vschfc5_db35 = assign43830_e42507_d_b35;

        let (assign43840_e42526, assign43840_e42526_d_n0, assign43840_e42526_d_n1, assign43840_e42526_d_n2, assign43840_e42526_d_n3, assign43840_e42526_d_n4, assign43840_e42526_d_n5, assign43840_e42526_d_n6, assign43840_e42526_d_n7, assign43840_e42526_d_n8, assign43840_e42526_d_n9, assign43840_e42526_d_n10, assign43840_e42526_d_n11, assign43840_e42526_d_n12, assign43840_e42526_d_n13, assign43840_e42526_d_n14, assign43840_e42526_d_n15, assign43840_e42526_d_n16, assign43840_e42526_d_n17, assign43840_e42526_d_n18, assign43840_e42526_d_n19, assign43840_e42526_d_n20, assign43840_e42526_d_n21, assign43840_e42526_d_n22, assign43840_e42526_d_n23, assign43840_e42526_d_n24, assign43840_e42526_d_n25, assign43840_e42526_d_n26, assign43840_e42526_d_n27, assign43840_e42526_d_n28, assign43840_e42526_d_n29, assign43840_e42526_d_b0, assign43840_e42526_d_b1, assign43840_e42526_d_b2, assign43840_e42526_d_b3, assign43840_e42526_d_b4, assign43840_e42526_d_b5, assign43840_e42526_d_b6, assign43840_e42526_d_b7, assign43840_e42526_d_b8, assign43840_e42526_d_b9, assign43840_e42526_d_b10, assign43840_e42526_d_b11, assign43840_e42526_d_b12, assign43840_e42526_d_b13, assign43840_e42526_d_b14, assign43840_e42526_d_b15, assign43840_e42526_d_b16, assign43840_e42526_d_b17, assign43840_e42526_d_b18, assign43840_e42526_d_b19, assign43840_e42526_d_b20, assign43840_e42526_d_b21, assign43840_e42526_d_b22, assign43840_e42526_d_b23, assign43840_e42526_d_b24, assign43840_e42526_d_b25, assign43840_e42526_d_b26, assign43840_e42526_d_b27, assign43840_e42526_d_b28, assign43840_e42526_d_b29, assign43840_e42526_d_b30, assign43840_e42526_d_b31, assign43840_e42526_d_b32, assign43840_e42526_d_b33, assign43840_e42526_d_b34, assign43840_e42526_d_b35,) = {
    if (((((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) && (var_guard477 != 0.0)) && (var_guard478 != 0.0)) {
        let assign43840_e42524: f64 = (var_qsch5c * var_vschfc5);
        (assign43840_e42524, (var_qsch5c * var_vschfc5_dn0), (var_qsch5c * var_vschfc5_dn1), (var_qsch5c * var_vschfc5_dn2), (var_qsch5c * var_vschfc5_dn3), (var_qsch5c * var_vschfc5_dn4), (var_qsch5c * var_vschfc5_dn5), (var_qsch5c * var_vschfc5_dn6), (var_qsch5c * var_vschfc5_dn7), (var_qsch5c * var_vschfc5_dn8), (var_qsch5c * var_vschfc5_dn9), (var_qsch5c * var_vschfc5_dn10), (var_qsch5c * var_vschfc5_dn11), (var_qsch5c * var_vschfc5_dn12), (var_qsch5c * var_vschfc5_dn13), (var_qsch5c * var_vschfc5_dn14), (var_qsch5c * var_vschfc5_dn15), (var_qsch5c * var_vschfc5_dn16), (var_qsch5c * var_vschfc5_dn17), (var_qsch5c * var_vschfc5_dn18), (var_qsch5c * var_vschfc5_dn19), (var_qsch5c * var_vschfc5_dn20), (var_qsch5c * var_vschfc5_dn21), (var_qsch5c * var_vschfc5_dn22), (var_qsch5c * var_vschfc5_dn23), (var_qsch5c * var_vschfc5_dn24), (var_qsch5c * var_vschfc5_dn25), (var_qsch5c * var_vschfc5_dn26), (var_qsch5c * var_vschfc5_dn27), (var_qsch5c * var_vschfc5_dn28), (var_qsch5c * var_vschfc5_dn29), (var_qsch5c * var_vschfc5_db0), (var_qsch5c * var_vschfc5_db1), (var_qsch5c * var_vschfc5_db2), (var_qsch5c * var_vschfc5_db3), (var_qsch5c * var_vschfc5_db4), (var_qsch5c * var_vschfc5_db5), (var_qsch5c * var_vschfc5_db6), (var_qsch5c * var_vschfc5_db7), (var_qsch5c * var_vschfc5_db8), (var_qsch5c * var_vschfc5_db9), (var_qsch5c * var_vschfc5_db10), (var_qsch5c * var_vschfc5_db11), (var_qsch5c * var_vschfc5_db12), (var_qsch5c * var_vschfc5_db13), (var_qsch5c * var_vschfc5_db14), (var_qsch5c * var_vschfc5_db15), (var_qsch5c * var_vschfc5_db16), (var_qsch5c * var_vschfc5_db17), (var_qsch5c * var_vschfc5_db18), (var_qsch5c * var_vschfc5_db19), (var_qsch5c * var_vschfc5_db20), (var_qsch5c * var_vschfc5_db21), (var_qsch5c * var_vschfc5_db22), (var_qsch5c * var_vschfc5_db23), (var_qsch5c * var_vschfc5_db24), (var_qsch5c * var_vschfc5_db25), (var_qsch5c * var_vschfc5_db26), (var_qsch5c * var_vschfc5_db27), (var_qsch5c * var_vschfc5_db28), (var_qsch5c * var_vschfc5_db29), (var_qsch5c * var_vschfc5_db30), (var_qsch5c * var_vschfc5_db31), (var_qsch5c * var_vschfc5_db32), (var_qsch5c * var_vschfc5_db33), (var_qsch5c * var_vschfc5_db34), (var_qsch5c * var_vschfc5_db35),)
    } else {
        (var_qsch5, var_qsch5_dn0, var_qsch5_dn1, var_qsch5_dn2, var_qsch5_dn3, var_qsch5_dn4, var_qsch5_dn5, var_qsch5_dn6, var_qsch5_dn7, var_qsch5_dn8, var_qsch5_dn9, var_qsch5_dn10, var_qsch5_dn11, var_qsch5_dn12, var_qsch5_dn13, var_qsch5_dn14, var_qsch5_dn15, var_qsch5_dn16, var_qsch5_dn17, var_qsch5_dn18, var_qsch5_dn19, var_qsch5_dn20, var_qsch5_dn21, var_qsch5_dn22, var_qsch5_dn23, var_qsch5_dn24, var_qsch5_dn25, var_qsch5_dn26, var_qsch5_dn27, var_qsch5_dn28, var_qsch5_dn29, var_qsch5_db0, var_qsch5_db1, var_qsch5_db2, var_qsch5_db3, var_qsch5_db4, var_qsch5_db5, var_qsch5_db6, var_qsch5_db7, var_qsch5_db8, var_qsch5_db9, var_qsch5_db10, var_qsch5_db11, var_qsch5_db12, var_qsch5_db13, var_qsch5_db14, var_qsch5_db15, var_qsch5_db16, var_qsch5_db17, var_qsch5_db18, var_qsch5_db19, var_qsch5_db20, var_qsch5_db21, var_qsch5_db22, var_qsch5_db23, var_qsch5_db24, var_qsch5_db25, var_qsch5_db26, var_qsch5_db27, var_qsch5_db28, var_qsch5_db29, var_qsch5_db30, var_qsch5_db31, var_qsch5_db32, var_qsch5_db33, var_qsch5_db34, var_qsch5_db35,)
    }
};
        var_qsch5 = assign43840_e42526;
        var_qsch5_dn0 = assign43840_e42526_d_n0;
        var_qsch5_dn1 = assign43840_e42526_d_n1;
        var_qsch5_dn2 = assign43840_e42526_d_n2;
        var_qsch5_dn3 = assign43840_e42526_d_n3;
        var_qsch5_dn4 = assign43840_e42526_d_n4;
        var_qsch5_dn5 = assign43840_e42526_d_n5;
        var_qsch5_dn6 = assign43840_e42526_d_n6;
        var_qsch5_dn7 = assign43840_e42526_d_n7;
        var_qsch5_dn8 = assign43840_e42526_d_n8;
        var_qsch5_dn9 = assign43840_e42526_d_n9;
        var_qsch5_dn10 = assign43840_e42526_d_n10;
        var_qsch5_dn11 = assign43840_e42526_d_n11;
        var_qsch5_dn12 = assign43840_e42526_d_n12;
        var_qsch5_dn13 = assign43840_e42526_d_n13;
        var_qsch5_dn14 = assign43840_e42526_d_n14;
        var_qsch5_dn15 = assign43840_e42526_d_n15;
        var_qsch5_dn16 = assign43840_e42526_d_n16;
        var_qsch5_dn17 = assign43840_e42526_d_n17;
        var_qsch5_dn18 = assign43840_e42526_d_n18;
        var_qsch5_dn19 = assign43840_e42526_d_n19;
        var_qsch5_dn20 = assign43840_e42526_d_n20;
        var_qsch5_dn21 = assign43840_e42526_d_n21;
        var_qsch5_dn22 = assign43840_e42526_d_n22;
        var_qsch5_dn23 = assign43840_e42526_d_n23;
        var_qsch5_dn24 = assign43840_e42526_d_n24;
        var_qsch5_dn25 = assign43840_e42526_d_n25;
        var_qsch5_dn26 = assign43840_e42526_d_n26;
        var_qsch5_dn27 = assign43840_e42526_d_n27;
        var_qsch5_dn28 = assign43840_e42526_d_n28;
        var_qsch5_dn29 = assign43840_e42526_d_n29;
        var_qsch5_db0 = assign43840_e42526_d_b0;
        var_qsch5_db1 = assign43840_e42526_d_b1;
        var_qsch5_db2 = assign43840_e42526_d_b2;
        var_qsch5_db3 = assign43840_e42526_d_b3;
        var_qsch5_db4 = assign43840_e42526_d_b4;
        var_qsch5_db5 = assign43840_e42526_d_b5;
        var_qsch5_db6 = assign43840_e42526_d_b6;
        var_qsch5_db7 = assign43840_e42526_d_b7;
        var_qsch5_db8 = assign43840_e42526_d_b8;
        var_qsch5_db9 = assign43840_e42526_d_b9;
        var_qsch5_db10 = assign43840_e42526_d_b10;
        var_qsch5_db11 = assign43840_e42526_d_b11;
        var_qsch5_db12 = assign43840_e42526_d_b12;
        var_qsch5_db13 = assign43840_e42526_d_b13;
        var_qsch5_db14 = assign43840_e42526_d_b14;
        var_qsch5_db15 = assign43840_e42526_d_b15;
        var_qsch5_db16 = assign43840_e42526_d_b16;
        var_qsch5_db17 = assign43840_e42526_d_b17;
        var_qsch5_db18 = assign43840_e42526_d_b18;
        var_qsch5_db19 = assign43840_e42526_d_b19;
        var_qsch5_db20 = assign43840_e42526_d_b20;
        var_qsch5_db21 = assign43840_e42526_d_b21;
        var_qsch5_db22 = assign43840_e42526_d_b22;
        var_qsch5_db23 = assign43840_e42526_d_b23;
        var_qsch5_db24 = assign43840_e42526_d_b24;
        var_qsch5_db25 = assign43840_e42526_d_b25;
        var_qsch5_db26 = assign43840_e42526_d_b26;
        var_qsch5_db27 = assign43840_e42526_d_b27;
        var_qsch5_db28 = assign43840_e42526_d_b28;
        var_qsch5_db29 = assign43840_e42526_d_b29;
        var_qsch5_db30 = assign43840_e42526_d_b30;
        var_qsch5_db31 = assign43840_e42526_d_b31;
        var_qsch5_db32 = assign43840_e42526_d_b32;
        var_qsch5_db33 = assign43840_e42526_d_b33;
        var_qsch5_db34 = assign43840_e42526_d_b34;
        var_qsch5_db35 = assign43840_e42526_d_b35;

        let (assign43850_e42544,) = {
    if (((((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) && (var_guard477 != 0.0)) && (var_guard478 == 0.0)) {
        (0.0,)
    } else {
        (var_qsch5c,)
    }
};
        var_qsch5c = assign43850_e42544;

        let (assign43860_e42560,) = {
    if ((((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 != 0.0)) && (var_guard477 == 0.0)) {
        (0.0,)
    } else {
        (var_qsch4c,)
    }
};
        var_qsch4c = assign43860_e42560;


        *var_guard477_slot = var_guard477;
        *var_guard478_slot = var_guard478;
        *var_qsch3_slot = var_qsch3;
        *var_qsch3_db0_slot = var_qsch3_db0;
        *var_qsch3_db1_slot = var_qsch3_db1;
        *var_qsch3_db10_slot = var_qsch3_db10;
        *var_qsch3_db11_slot = var_qsch3_db11;
        *var_qsch3_db12_slot = var_qsch3_db12;
        *var_qsch3_db13_slot = var_qsch3_db13;
        *var_qsch3_db14_slot = var_qsch3_db14;
        *var_qsch3_db15_slot = var_qsch3_db15;
        *var_qsch3_db16_slot = var_qsch3_db16;
        *var_qsch3_db17_slot = var_qsch3_db17;
        *var_qsch3_db18_slot = var_qsch3_db18;
        *var_qsch3_db19_slot = var_qsch3_db19;
        *var_qsch3_db2_slot = var_qsch3_db2;
        *var_qsch3_db20_slot = var_qsch3_db20;
        *var_qsch3_db21_slot = var_qsch3_db21;
        *var_qsch3_db22_slot = var_qsch3_db22;
        *var_qsch3_db23_slot = var_qsch3_db23;
        *var_qsch3_db24_slot = var_qsch3_db24;
        *var_qsch3_db25_slot = var_qsch3_db25;
        *var_qsch3_db26_slot = var_qsch3_db26;
        *var_qsch3_db27_slot = var_qsch3_db27;
        *var_qsch3_db28_slot = var_qsch3_db28;
        *var_qsch3_db29_slot = var_qsch3_db29;
        *var_qsch3_db3_slot = var_qsch3_db3;
        *var_qsch3_db30_slot = var_qsch3_db30;
        *var_qsch3_db31_slot = var_qsch3_db31;
        *var_qsch3_db32_slot = var_qsch3_db32;
        *var_qsch3_db33_slot = var_qsch3_db33;
        *var_qsch3_db34_slot = var_qsch3_db34;
        *var_qsch3_db35_slot = var_qsch3_db35;
        *var_qsch3_db4_slot = var_qsch3_db4;
        *var_qsch3_db5_slot = var_qsch3_db5;
        *var_qsch3_db6_slot = var_qsch3_db6;
        *var_qsch3_db7_slot = var_qsch3_db7;
        *var_qsch3_db8_slot = var_qsch3_db8;
        *var_qsch3_db9_slot = var_qsch3_db9;
        *var_qsch3_dn0_slot = var_qsch3_dn0;
        *var_qsch3_dn1_slot = var_qsch3_dn1;
        *var_qsch3_dn10_slot = var_qsch3_dn10;
        *var_qsch3_dn11_slot = var_qsch3_dn11;
        *var_qsch3_dn12_slot = var_qsch3_dn12;
        *var_qsch3_dn13_slot = var_qsch3_dn13;
        *var_qsch3_dn14_slot = var_qsch3_dn14;
        *var_qsch3_dn15_slot = var_qsch3_dn15;
        *var_qsch3_dn16_slot = var_qsch3_dn16;
        *var_qsch3_dn17_slot = var_qsch3_dn17;
        *var_qsch3_dn18_slot = var_qsch3_dn18;
        *var_qsch3_dn19_slot = var_qsch3_dn19;
        *var_qsch3_dn2_slot = var_qsch3_dn2;
        *var_qsch3_dn20_slot = var_qsch3_dn20;
        *var_qsch3_dn21_slot = var_qsch3_dn21;
        *var_qsch3_dn22_slot = var_qsch3_dn22;
        *var_qsch3_dn23_slot = var_qsch3_dn23;
        *var_qsch3_dn24_slot = var_qsch3_dn24;
        *var_qsch3_dn25_slot = var_qsch3_dn25;
        *var_qsch3_dn26_slot = var_qsch3_dn26;
        *var_qsch3_dn27_slot = var_qsch3_dn27;
        *var_qsch3_dn28_slot = var_qsch3_dn28;
        *var_qsch3_dn29_slot = var_qsch3_dn29;
        *var_qsch3_dn3_slot = var_qsch3_dn3;
        *var_qsch3_dn4_slot = var_qsch3_dn4;
        *var_qsch3_dn5_slot = var_qsch3_dn5;
        *var_qsch3_dn6_slot = var_qsch3_dn6;
        *var_qsch3_dn7_slot = var_qsch3_dn7;
        *var_qsch3_dn8_slot = var_qsch3_dn8;
        *var_qsch3_dn9_slot = var_qsch3_dn9;
        *var_qsch4_slot = var_qsch4;
        *var_qsch4_db0_slot = var_qsch4_db0;
        *var_qsch4_db1_slot = var_qsch4_db1;
        *var_qsch4_db10_slot = var_qsch4_db10;
        *var_qsch4_db11_slot = var_qsch4_db11;
        *var_qsch4_db12_slot = var_qsch4_db12;
        *var_qsch4_db13_slot = var_qsch4_db13;
        *var_qsch4_db14_slot = var_qsch4_db14;
        *var_qsch4_db15_slot = var_qsch4_db15;
        *var_qsch4_db16_slot = var_qsch4_db16;
        *var_qsch4_db17_slot = var_qsch4_db17;
        *var_qsch4_db18_slot = var_qsch4_db18;
        *var_qsch4_db19_slot = var_qsch4_db19;
        *var_qsch4_db2_slot = var_qsch4_db2;
        *var_qsch4_db20_slot = var_qsch4_db20;
        *var_qsch4_db21_slot = var_qsch4_db21;
        *var_qsch4_db22_slot = var_qsch4_db22;
        *var_qsch4_db23_slot = var_qsch4_db23;
        *var_qsch4_db24_slot = var_qsch4_db24;
        *var_qsch4_db25_slot = var_qsch4_db25;
        *var_qsch4_db26_slot = var_qsch4_db26;
        *var_qsch4_db27_slot = var_qsch4_db27;
        *var_qsch4_db28_slot = var_qsch4_db28;
        *var_qsch4_db29_slot = var_qsch4_db29;
        *var_qsch4_db3_slot = var_qsch4_db3;
        *var_qsch4_db30_slot = var_qsch4_db30;
        *var_qsch4_db31_slot = var_qsch4_db31;
        *var_qsch4_db32_slot = var_qsch4_db32;
        *var_qsch4_db33_slot = var_qsch4_db33;
        *var_qsch4_db34_slot = var_qsch4_db34;
        *var_qsch4_db35_slot = var_qsch4_db35;
        *var_qsch4_db4_slot = var_qsch4_db4;
        *var_qsch4_db5_slot = var_qsch4_db5;
        *var_qsch4_db6_slot = var_qsch4_db6;
        *var_qsch4_db7_slot = var_qsch4_db7;
        *var_qsch4_db8_slot = var_qsch4_db8;
        *var_qsch4_db9_slot = var_qsch4_db9;
        *var_qsch4_dn0_slot = var_qsch4_dn0;
        *var_qsch4_dn1_slot = var_qsch4_dn1;
        *var_qsch4_dn10_slot = var_qsch4_dn10;
        *var_qsch4_dn11_slot = var_qsch4_dn11;
        *var_qsch4_dn12_slot = var_qsch4_dn12;
        *var_qsch4_dn13_slot = var_qsch4_dn13;
        *var_qsch4_dn14_slot = var_qsch4_dn14;
        *var_qsch4_dn15_slot = var_qsch4_dn15;
        *var_qsch4_dn16_slot = var_qsch4_dn16;
        *var_qsch4_dn17_slot = var_qsch4_dn17;
        *var_qsch4_dn18_slot = var_qsch4_dn18;
        *var_qsch4_dn19_slot = var_qsch4_dn19;
        *var_qsch4_dn2_slot = var_qsch4_dn2;
        *var_qsch4_dn20_slot = var_qsch4_dn20;
        *var_qsch4_dn21_slot = var_qsch4_dn21;
        *var_qsch4_dn22_slot = var_qsch4_dn22;
        *var_qsch4_dn23_slot = var_qsch4_dn23;
        *var_qsch4_dn24_slot = var_qsch4_dn24;
        *var_qsch4_dn25_slot = var_qsch4_dn25;
        *var_qsch4_dn26_slot = var_qsch4_dn26;
        *var_qsch4_dn27_slot = var_qsch4_dn27;
        *var_qsch4_dn28_slot = var_qsch4_dn28;
        *var_qsch4_dn29_slot = var_qsch4_dn29;
        *var_qsch4_dn3_slot = var_qsch4_dn3;
        *var_qsch4_dn4_slot = var_qsch4_dn4;
        *var_qsch4_dn5_slot = var_qsch4_dn5;
        *var_qsch4_dn6_slot = var_qsch4_dn6;
        *var_qsch4_dn7_slot = var_qsch4_dn7;
        *var_qsch4_dn8_slot = var_qsch4_dn8;
        *var_qsch4_dn9_slot = var_qsch4_dn9;
        *var_qsch4c_slot = var_qsch4c;
        *var_qsch5_slot = var_qsch5;
        *var_qsch5_db0_slot = var_qsch5_db0;
        *var_qsch5_db1_slot = var_qsch5_db1;
        *var_qsch5_db10_slot = var_qsch5_db10;
        *var_qsch5_db11_slot = var_qsch5_db11;
        *var_qsch5_db12_slot = var_qsch5_db12;
        *var_qsch5_db13_slot = var_qsch5_db13;
        *var_qsch5_db14_slot = var_qsch5_db14;
        *var_qsch5_db15_slot = var_qsch5_db15;
        *var_qsch5_db16_slot = var_qsch5_db16;
        *var_qsch5_db17_slot = var_qsch5_db17;
        *var_qsch5_db18_slot = var_qsch5_db18;
        *var_qsch5_db19_slot = var_qsch5_db19;
        *var_qsch5_db2_slot = var_qsch5_db2;
        *var_qsch5_db20_slot = var_qsch5_db20;
        *var_qsch5_db21_slot = var_qsch5_db21;
        *var_qsch5_db22_slot = var_qsch5_db22;
        *var_qsch5_db23_slot = var_qsch5_db23;
        *var_qsch5_db24_slot = var_qsch5_db24;
        *var_qsch5_db25_slot = var_qsch5_db25;
        *var_qsch5_db26_slot = var_qsch5_db26;
        *var_qsch5_db27_slot = var_qsch5_db27;
        *var_qsch5_db28_slot = var_qsch5_db28;
        *var_qsch5_db29_slot = var_qsch5_db29;
        *var_qsch5_db3_slot = var_qsch5_db3;
        *var_qsch5_db30_slot = var_qsch5_db30;
        *var_qsch5_db31_slot = var_qsch5_db31;
        *var_qsch5_db32_slot = var_qsch5_db32;
        *var_qsch5_db33_slot = var_qsch5_db33;
        *var_qsch5_db34_slot = var_qsch5_db34;
        *var_qsch5_db35_slot = var_qsch5_db35;
        *var_qsch5_db4_slot = var_qsch5_db4;
        *var_qsch5_db5_slot = var_qsch5_db5;
        *var_qsch5_db6_slot = var_qsch5_db6;
        *var_qsch5_db7_slot = var_qsch5_db7;
        *var_qsch5_db8_slot = var_qsch5_db8;
        *var_qsch5_db9_slot = var_qsch5_db9;
        *var_qsch5_dn0_slot = var_qsch5_dn0;
        *var_qsch5_dn1_slot = var_qsch5_dn1;
        *var_qsch5_dn10_slot = var_qsch5_dn10;
        *var_qsch5_dn11_slot = var_qsch5_dn11;
        *var_qsch5_dn12_slot = var_qsch5_dn12;
        *var_qsch5_dn13_slot = var_qsch5_dn13;
        *var_qsch5_dn14_slot = var_qsch5_dn14;
        *var_qsch5_dn15_slot = var_qsch5_dn15;
        *var_qsch5_dn16_slot = var_qsch5_dn16;
        *var_qsch5_dn17_slot = var_qsch5_dn17;
        *var_qsch5_dn18_slot = var_qsch5_dn18;
        *var_qsch5_dn19_slot = var_qsch5_dn19;
        *var_qsch5_dn2_slot = var_qsch5_dn2;
        *var_qsch5_dn20_slot = var_qsch5_dn20;
        *var_qsch5_dn21_slot = var_qsch5_dn21;
        *var_qsch5_dn22_slot = var_qsch5_dn22;
        *var_qsch5_dn23_slot = var_qsch5_dn23;
        *var_qsch5_dn24_slot = var_qsch5_dn24;
        *var_qsch5_dn25_slot = var_qsch5_dn25;
        *var_qsch5_dn26_slot = var_qsch5_dn26;
        *var_qsch5_dn27_slot = var_qsch5_dn27;
        *var_qsch5_dn28_slot = var_qsch5_dn28;
        *var_qsch5_dn29_slot = var_qsch5_dn29;
        *var_qsch5_dn3_slot = var_qsch5_dn3;
        *var_qsch5_dn4_slot = var_qsch5_dn4;
        *var_qsch5_dn5_slot = var_qsch5_dn5;
        *var_qsch5_dn6_slot = var_qsch5_dn6;
        *var_qsch5_dn7_slot = var_qsch5_dn7;
        *var_qsch5_dn8_slot = var_qsch5_dn8;
        *var_qsch5_dn9_slot = var_qsch5_dn9;
        *var_qsch5c_slot = var_qsch5c;
        *var_vschfc3_slot = var_vschfc3;
        *var_vschfc3_db0_slot = var_vschfc3_db0;
        *var_vschfc3_db1_slot = var_vschfc3_db1;
        *var_vschfc3_db10_slot = var_vschfc3_db10;
        *var_vschfc3_db11_slot = var_vschfc3_db11;
        *var_vschfc3_db12_slot = var_vschfc3_db12;
        *var_vschfc3_db13_slot = var_vschfc3_db13;
        *var_vschfc3_db14_slot = var_vschfc3_db14;
        *var_vschfc3_db15_slot = var_vschfc3_db15;
        *var_vschfc3_db16_slot = var_vschfc3_db16;
        *var_vschfc3_db17_slot = var_vschfc3_db17;
        *var_vschfc3_db18_slot = var_vschfc3_db18;
        *var_vschfc3_db19_slot = var_vschfc3_db19;
        *var_vschfc3_db2_slot = var_vschfc3_db2;
        *var_vschfc3_db20_slot = var_vschfc3_db20;
        *var_vschfc3_db21_slot = var_vschfc3_db21;
        *var_vschfc3_db22_slot = var_vschfc3_db22;
        *var_vschfc3_db23_slot = var_vschfc3_db23;
        *var_vschfc3_db24_slot = var_vschfc3_db24;
        *var_vschfc3_db25_slot = var_vschfc3_db25;
        *var_vschfc3_db26_slot = var_vschfc3_db26;
        *var_vschfc3_db27_slot = var_vschfc3_db27;
        *var_vschfc3_db28_slot = var_vschfc3_db28;
        *var_vschfc3_db29_slot = var_vschfc3_db29;
        *var_vschfc3_db3_slot = var_vschfc3_db3;
        *var_vschfc3_db30_slot = var_vschfc3_db30;
        *var_vschfc3_db31_slot = var_vschfc3_db31;
        *var_vschfc3_db32_slot = var_vschfc3_db32;
        *var_vschfc3_db33_slot = var_vschfc3_db33;
        *var_vschfc3_db34_slot = var_vschfc3_db34;
        *var_vschfc3_db35_slot = var_vschfc3_db35;
        *var_vschfc3_db4_slot = var_vschfc3_db4;
        *var_vschfc3_db5_slot = var_vschfc3_db5;
        *var_vschfc3_db6_slot = var_vschfc3_db6;
        *var_vschfc3_db7_slot = var_vschfc3_db7;
        *var_vschfc3_db8_slot = var_vschfc3_db8;
        *var_vschfc3_db9_slot = var_vschfc3_db9;
        *var_vschfc3_dn0_slot = var_vschfc3_dn0;
        *var_vschfc3_dn1_slot = var_vschfc3_dn1;
        *var_vschfc3_dn10_slot = var_vschfc3_dn10;
        *var_vschfc3_dn11_slot = var_vschfc3_dn11;
        *var_vschfc3_dn12_slot = var_vschfc3_dn12;
        *var_vschfc3_dn13_slot = var_vschfc3_dn13;
        *var_vschfc3_dn14_slot = var_vschfc3_dn14;
        *var_vschfc3_dn15_slot = var_vschfc3_dn15;
        *var_vschfc3_dn16_slot = var_vschfc3_dn16;
        *var_vschfc3_dn17_slot = var_vschfc3_dn17;
        *var_vschfc3_dn18_slot = var_vschfc3_dn18;
        *var_vschfc3_dn19_slot = var_vschfc3_dn19;
        *var_vschfc3_dn2_slot = var_vschfc3_dn2;
        *var_vschfc3_dn20_slot = var_vschfc3_dn20;
        *var_vschfc3_dn21_slot = var_vschfc3_dn21;
        *var_vschfc3_dn22_slot = var_vschfc3_dn22;
        *var_vschfc3_dn23_slot = var_vschfc3_dn23;
        *var_vschfc3_dn24_slot = var_vschfc3_dn24;
        *var_vschfc3_dn25_slot = var_vschfc3_dn25;
        *var_vschfc3_dn26_slot = var_vschfc3_dn26;
        *var_vschfc3_dn27_slot = var_vschfc3_dn27;
        *var_vschfc3_dn28_slot = var_vschfc3_dn28;
        *var_vschfc3_dn29_slot = var_vschfc3_dn29;
        *var_vschfc3_dn3_slot = var_vschfc3_dn3;
        *var_vschfc3_dn4_slot = var_vschfc3_dn4;
        *var_vschfc3_dn5_slot = var_vschfc3_dn5;
        *var_vschfc3_dn6_slot = var_vschfc3_dn6;
        *var_vschfc3_dn7_slot = var_vschfc3_dn7;
        *var_vschfc3_dn8_slot = var_vschfc3_dn8;
        *var_vschfc3_dn9_slot = var_vschfc3_dn9;
        *var_vschfc4_slot = var_vschfc4;
        *var_vschfc4_db0_slot = var_vschfc4_db0;
        *var_vschfc4_db1_slot = var_vschfc4_db1;
        *var_vschfc4_db10_slot = var_vschfc4_db10;
        *var_vschfc4_db11_slot = var_vschfc4_db11;
        *var_vschfc4_db12_slot = var_vschfc4_db12;
        *var_vschfc4_db13_slot = var_vschfc4_db13;
        *var_vschfc4_db14_slot = var_vschfc4_db14;
        *var_vschfc4_db15_slot = var_vschfc4_db15;
        *var_vschfc4_db16_slot = var_vschfc4_db16;
        *var_vschfc4_db17_slot = var_vschfc4_db17;
        *var_vschfc4_db18_slot = var_vschfc4_db18;
        *var_vschfc4_db19_slot = var_vschfc4_db19;
        *var_vschfc4_db2_slot = var_vschfc4_db2;
        *var_vschfc4_db20_slot = var_vschfc4_db20;
        *var_vschfc4_db21_slot = var_vschfc4_db21;
        *var_vschfc4_db22_slot = var_vschfc4_db22;
        *var_vschfc4_db23_slot = var_vschfc4_db23;
        *var_vschfc4_db24_slot = var_vschfc4_db24;
        *var_vschfc4_db25_slot = var_vschfc4_db25;
        *var_vschfc4_db26_slot = var_vschfc4_db26;
        *var_vschfc4_db27_slot = var_vschfc4_db27;
        *var_vschfc4_db28_slot = var_vschfc4_db28;
        *var_vschfc4_db29_slot = var_vschfc4_db29;
        *var_vschfc4_db3_slot = var_vschfc4_db3;
        *var_vschfc4_db30_slot = var_vschfc4_db30;
        *var_vschfc4_db31_slot = var_vschfc4_db31;
        *var_vschfc4_db32_slot = var_vschfc4_db32;
        *var_vschfc4_db33_slot = var_vschfc4_db33;
        *var_vschfc4_db34_slot = var_vschfc4_db34;
        *var_vschfc4_db35_slot = var_vschfc4_db35;
        *var_vschfc4_db4_slot = var_vschfc4_db4;
        *var_vschfc4_db5_slot = var_vschfc4_db5;
        *var_vschfc4_db6_slot = var_vschfc4_db6;
        *var_vschfc4_db7_slot = var_vschfc4_db7;
        *var_vschfc4_db8_slot = var_vschfc4_db8;
        *var_vschfc4_db9_slot = var_vschfc4_db9;
        *var_vschfc4_dn0_slot = var_vschfc4_dn0;
        *var_vschfc4_dn1_slot = var_vschfc4_dn1;
        *var_vschfc4_dn10_slot = var_vschfc4_dn10;
        *var_vschfc4_dn11_slot = var_vschfc4_dn11;
        *var_vschfc4_dn12_slot = var_vschfc4_dn12;
        *var_vschfc4_dn13_slot = var_vschfc4_dn13;
        *var_vschfc4_dn14_slot = var_vschfc4_dn14;
        *var_vschfc4_dn15_slot = var_vschfc4_dn15;
        *var_vschfc4_dn16_slot = var_vschfc4_dn16;
        *var_vschfc4_dn17_slot = var_vschfc4_dn17;
        *var_vschfc4_dn18_slot = var_vschfc4_dn18;
        *var_vschfc4_dn19_slot = var_vschfc4_dn19;
        *var_vschfc4_dn2_slot = var_vschfc4_dn2;
        *var_vschfc4_dn20_slot = var_vschfc4_dn20;
        *var_vschfc4_dn21_slot = var_vschfc4_dn21;
        *var_vschfc4_dn22_slot = var_vschfc4_dn22;
        *var_vschfc4_dn23_slot = var_vschfc4_dn23;
        *var_vschfc4_dn24_slot = var_vschfc4_dn24;
        *var_vschfc4_dn25_slot = var_vschfc4_dn25;
        *var_vschfc4_dn26_slot = var_vschfc4_dn26;
        *var_vschfc4_dn27_slot = var_vschfc4_dn27;
        *var_vschfc4_dn28_slot = var_vschfc4_dn28;
        *var_vschfc4_dn29_slot = var_vschfc4_dn29;
        *var_vschfc4_dn3_slot = var_vschfc4_dn3;
        *var_vschfc4_dn4_slot = var_vschfc4_dn4;
        *var_vschfc4_dn5_slot = var_vschfc4_dn5;
        *var_vschfc4_dn6_slot = var_vschfc4_dn6;
        *var_vschfc4_dn7_slot = var_vschfc4_dn7;
        *var_vschfc4_dn8_slot = var_vschfc4_dn8;
        *var_vschfc4_dn9_slot = var_vschfc4_dn9;
        *var_vschfc5_slot = var_vschfc5;
        *var_vschfc5_db0_slot = var_vschfc5_db0;
        *var_vschfc5_db1_slot = var_vschfc5_db1;
        *var_vschfc5_db10_slot = var_vschfc5_db10;
        *var_vschfc5_db11_slot = var_vschfc5_db11;
        *var_vschfc5_db12_slot = var_vschfc5_db12;
        *var_vschfc5_db13_slot = var_vschfc5_db13;
        *var_vschfc5_db14_slot = var_vschfc5_db14;
        *var_vschfc5_db15_slot = var_vschfc5_db15;
        *var_vschfc5_db16_slot = var_vschfc5_db16;
        *var_vschfc5_db17_slot = var_vschfc5_db17;
        *var_vschfc5_db18_slot = var_vschfc5_db18;
        *var_vschfc5_db19_slot = var_vschfc5_db19;
        *var_vschfc5_db2_slot = var_vschfc5_db2;
        *var_vschfc5_db20_slot = var_vschfc5_db20;
        *var_vschfc5_db21_slot = var_vschfc5_db21;
        *var_vschfc5_db22_slot = var_vschfc5_db22;
        *var_vschfc5_db23_slot = var_vschfc5_db23;
        *var_vschfc5_db24_slot = var_vschfc5_db24;
        *var_vschfc5_db25_slot = var_vschfc5_db25;
        *var_vschfc5_db26_slot = var_vschfc5_db26;
        *var_vschfc5_db27_slot = var_vschfc5_db27;
        *var_vschfc5_db28_slot = var_vschfc5_db28;
        *var_vschfc5_db29_slot = var_vschfc5_db29;
        *var_vschfc5_db3_slot = var_vschfc5_db3;
        *var_vschfc5_db30_slot = var_vschfc5_db30;
        *var_vschfc5_db31_slot = var_vschfc5_db31;
        *var_vschfc5_db32_slot = var_vschfc5_db32;
        *var_vschfc5_db33_slot = var_vschfc5_db33;
        *var_vschfc5_db34_slot = var_vschfc5_db34;
        *var_vschfc5_db35_slot = var_vschfc5_db35;
        *var_vschfc5_db4_slot = var_vschfc5_db4;
        *var_vschfc5_db5_slot = var_vschfc5_db5;
        *var_vschfc5_db6_slot = var_vschfc5_db6;
        *var_vschfc5_db7_slot = var_vschfc5_db7;
        *var_vschfc5_db8_slot = var_vschfc5_db8;
        *var_vschfc5_db9_slot = var_vschfc5_db9;
        *var_vschfc5_dn0_slot = var_vschfc5_dn0;
        *var_vschfc5_dn1_slot = var_vschfc5_dn1;
        *var_vschfc5_dn10_slot = var_vschfc5_dn10;
        *var_vschfc5_dn11_slot = var_vschfc5_dn11;
        *var_vschfc5_dn12_slot = var_vschfc5_dn12;
        *var_vschfc5_dn13_slot = var_vschfc5_dn13;
        *var_vschfc5_dn14_slot = var_vschfc5_dn14;
        *var_vschfc5_dn15_slot = var_vschfc5_dn15;
        *var_vschfc5_dn16_slot = var_vschfc5_dn16;
        *var_vschfc5_dn17_slot = var_vschfc5_dn17;
        *var_vschfc5_dn18_slot = var_vschfc5_dn18;
        *var_vschfc5_dn19_slot = var_vschfc5_dn19;
        *var_vschfc5_dn2_slot = var_vschfc5_dn2;
        *var_vschfc5_dn20_slot = var_vschfc5_dn20;
        *var_vschfc5_dn21_slot = var_vschfc5_dn21;
        *var_vschfc5_dn22_slot = var_vschfc5_dn22;
        *var_vschfc5_dn23_slot = var_vschfc5_dn23;
        *var_vschfc5_dn24_slot = var_vschfc5_dn24;
        *var_vschfc5_dn25_slot = var_vschfc5_dn25;
        *var_vschfc5_dn26_slot = var_vschfc5_dn26;
        *var_vschfc5_dn27_slot = var_vschfc5_dn27;
        *var_vschfc5_dn28_slot = var_vschfc5_dn28;
        *var_vschfc5_dn29_slot = var_vschfc5_dn29;
        *var_vschfc5_dn3_slot = var_vschfc5_dn3;
        *var_vschfc5_dn4_slot = var_vschfc5_dn4;
        *var_vschfc5_dn5_slot = var_vschfc5_dn5;
        *var_vschfc5_dn6_slot = var_vschfc5_dn6;
        *var_vschfc5_dn7_slot = var_vschfc5_dn7;
        *var_vschfc5_dn8_slot = var_vschfc5_dn8;
        *var_vschfc5_dn9_slot = var_vschfc5_dn9;
    }

    pub(super) fn stamp_transient_block_117(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_guard461: f64,
        var_guard473: f64,
        var_guard474: f64,
        var_guard475: f64,
        var_guard476: f64,
        var_qsch0: f64,
        var_qsch1: f64,
        var_qsch1_db0: f64,
        var_qsch1_db1: f64,
        var_qsch1_db10: f64,
        var_qsch1_db11: f64,
        var_qsch1_db12: f64,
        var_qsch1_db13: f64,
        var_qsch1_db14: f64,
        var_qsch1_db15: f64,
        var_qsch1_db16: f64,
        var_qsch1_db17: f64,
        var_qsch1_db18: f64,
        var_qsch1_db19: f64,
        var_qsch1_db2: f64,
        var_qsch1_db20: f64,
        var_qsch1_db21: f64,
        var_qsch1_db22: f64,
        var_qsch1_db23: f64,
        var_qsch1_db24: f64,
        var_qsch1_db25: f64,
        var_qsch1_db26: f64,
        var_qsch1_db27: f64,
        var_qsch1_db28: f64,
        var_qsch1_db29: f64,
        var_qsch1_db3: f64,
        var_qsch1_db30: f64,
        var_qsch1_db31: f64,
        var_qsch1_db32: f64,
        var_qsch1_db33: f64,
        var_qsch1_db34: f64,
        var_qsch1_db35: f64,
        var_qsch1_db4: f64,
        var_qsch1_db5: f64,
        var_qsch1_db6: f64,
        var_qsch1_db7: f64,
        var_qsch1_db8: f64,
        var_qsch1_db9: f64,
        var_qsch1_dn0: f64,
        var_qsch1_dn1: f64,
        var_qsch1_dn10: f64,
        var_qsch1_dn11: f64,
        var_qsch1_dn12: f64,
        var_qsch1_dn13: f64,
        var_qsch1_dn14: f64,
        var_qsch1_dn15: f64,
        var_qsch1_dn16: f64,
        var_qsch1_dn17: f64,
        var_qsch1_dn18: f64,
        var_qsch1_dn19: f64,
        var_qsch1_dn2: f64,
        var_qsch1_dn20: f64,
        var_qsch1_dn21: f64,
        var_qsch1_dn22: f64,
        var_qsch1_dn23: f64,
        var_qsch1_dn24: f64,
        var_qsch1_dn25: f64,
        var_qsch1_dn26: f64,
        var_qsch1_dn27: f64,
        var_qsch1_dn28: f64,
        var_qsch1_dn29: f64,
        var_qsch1_dn3: f64,
        var_qsch1_dn4: f64,
        var_qsch1_dn5: f64,
        var_qsch1_dn6: f64,
        var_qsch1_dn7: f64,
        var_qsch1_dn8: f64,
        var_qsch1_dn9: f64,
        var_qsch2: f64,
        var_qsch2_db0: f64,
        var_qsch2_db1: f64,
        var_qsch2_db10: f64,
        var_qsch2_db11: f64,
        var_qsch2_db12: f64,
        var_qsch2_db13: f64,
        var_qsch2_db14: f64,
        var_qsch2_db15: f64,
        var_qsch2_db16: f64,
        var_qsch2_db17: f64,
        var_qsch2_db18: f64,
        var_qsch2_db19: f64,
        var_qsch2_db2: f64,
        var_qsch2_db20: f64,
        var_qsch2_db21: f64,
        var_qsch2_db22: f64,
        var_qsch2_db23: f64,
        var_qsch2_db24: f64,
        var_qsch2_db25: f64,
        var_qsch2_db26: f64,
        var_qsch2_db27: f64,
        var_qsch2_db28: f64,
        var_qsch2_db29: f64,
        var_qsch2_db3: f64,
        var_qsch2_db30: f64,
        var_qsch2_db31: f64,
        var_qsch2_db32: f64,
        var_qsch2_db33: f64,
        var_qsch2_db34: f64,
        var_qsch2_db35: f64,
        var_qsch2_db4: f64,
        var_qsch2_db5: f64,
        var_qsch2_db6: f64,
        var_qsch2_db7: f64,
        var_qsch2_db8: f64,
        var_qsch2_db9: f64,
        var_qsch2_dn0: f64,
        var_qsch2_dn1: f64,
        var_qsch2_dn10: f64,
        var_qsch2_dn11: f64,
        var_qsch2_dn12: f64,
        var_qsch2_dn13: f64,
        var_qsch2_dn14: f64,
        var_qsch2_dn15: f64,
        var_qsch2_dn16: f64,
        var_qsch2_dn17: f64,
        var_qsch2_dn18: f64,
        var_qsch2_dn19: f64,
        var_qsch2_dn2: f64,
        var_qsch2_dn20: f64,
        var_qsch2_dn21: f64,
        var_qsch2_dn22: f64,
        var_qsch2_dn23: f64,
        var_qsch2_dn24: f64,
        var_qsch2_dn25: f64,
        var_qsch2_dn26: f64,
        var_qsch2_dn27: f64,
        var_qsch2_dn28: f64,
        var_qsch2_dn29: f64,
        var_qsch2_dn3: f64,
        var_qsch2_dn4: f64,
        var_qsch2_dn5: f64,
        var_qsch2_dn6: f64,
        var_qsch2_dn7: f64,
        var_qsch2_dn8: f64,
        var_qsch2_dn9: f64,
        var_qsch3: f64,
        var_qsch3_db0: f64,
        var_qsch3_db1: f64,
        var_qsch3_db10: f64,
        var_qsch3_db11: f64,
        var_qsch3_db12: f64,
        var_qsch3_db13: f64,
        var_qsch3_db14: f64,
        var_qsch3_db15: f64,
        var_qsch3_db16: f64,
        var_qsch3_db17: f64,
        var_qsch3_db18: f64,
        var_qsch3_db19: f64,
        var_qsch3_db2: f64,
        var_qsch3_db20: f64,
        var_qsch3_db21: f64,
        var_qsch3_db22: f64,
        var_qsch3_db23: f64,
        var_qsch3_db24: f64,
        var_qsch3_db25: f64,
        var_qsch3_db26: f64,
        var_qsch3_db27: f64,
        var_qsch3_db28: f64,
        var_qsch3_db29: f64,
        var_qsch3_db3: f64,
        var_qsch3_db30: f64,
        var_qsch3_db31: f64,
        var_qsch3_db32: f64,
        var_qsch3_db33: f64,
        var_qsch3_db34: f64,
        var_qsch3_db35: f64,
        var_qsch3_db4: f64,
        var_qsch3_db5: f64,
        var_qsch3_db6: f64,
        var_qsch3_db7: f64,
        var_qsch3_db8: f64,
        var_qsch3_db9: f64,
        var_qsch3_dn0: f64,
        var_qsch3_dn1: f64,
        var_qsch3_dn10: f64,
        var_qsch3_dn11: f64,
        var_qsch3_dn12: f64,
        var_qsch3_dn13: f64,
        var_qsch3_dn14: f64,
        var_qsch3_dn15: f64,
        var_qsch3_dn16: f64,
        var_qsch3_dn17: f64,
        var_qsch3_dn18: f64,
        var_qsch3_dn19: f64,
        var_qsch3_dn2: f64,
        var_qsch3_dn20: f64,
        var_qsch3_dn21: f64,
        var_qsch3_dn22: f64,
        var_qsch3_dn23: f64,
        var_qsch3_dn24: f64,
        var_qsch3_dn25: f64,
        var_qsch3_dn26: f64,
        var_qsch3_dn27: f64,
        var_qsch3_dn28: f64,
        var_qsch3_dn29: f64,
        var_qsch3_dn3: f64,
        var_qsch3_dn4: f64,
        var_qsch3_dn5: f64,
        var_qsch3_dn6: f64,
        var_qsch3_dn7: f64,
        var_qsch3_dn8: f64,
        var_qsch3_dn9: f64,
        var_qsch4: f64,
        var_qsch4_db0: f64,
        var_qsch4_db1: f64,
        var_qsch4_db10: f64,
        var_qsch4_db11: f64,
        var_qsch4_db12: f64,
        var_qsch4_db13: f64,
        var_qsch4_db14: f64,
        var_qsch4_db15: f64,
        var_qsch4_db16: f64,
        var_qsch4_db17: f64,
        var_qsch4_db18: f64,
        var_qsch4_db19: f64,
        var_qsch4_db2: f64,
        var_qsch4_db20: f64,
        var_qsch4_db21: f64,
        var_qsch4_db22: f64,
        var_qsch4_db23: f64,
        var_qsch4_db24: f64,
        var_qsch4_db25: f64,
        var_qsch4_db26: f64,
        var_qsch4_db27: f64,
        var_qsch4_db28: f64,
        var_qsch4_db29: f64,
        var_qsch4_db3: f64,
        var_qsch4_db30: f64,
        var_qsch4_db31: f64,
        var_qsch4_db32: f64,
        var_qsch4_db33: f64,
        var_qsch4_db34: f64,
        var_qsch4_db35: f64,
        var_qsch4_db4: f64,
        var_qsch4_db5: f64,
        var_qsch4_db6: f64,
        var_qsch4_db7: f64,
        var_qsch4_db8: f64,
        var_qsch4_db9: f64,
        var_qsch4_dn0: f64,
        var_qsch4_dn1: f64,
        var_qsch4_dn10: f64,
        var_qsch4_dn11: f64,
        var_qsch4_dn12: f64,
        var_qsch4_dn13: f64,
        var_qsch4_dn14: f64,
        var_qsch4_dn15: f64,
        var_qsch4_dn16: f64,
        var_qsch4_dn17: f64,
        var_qsch4_dn18: f64,
        var_qsch4_dn19: f64,
        var_qsch4_dn2: f64,
        var_qsch4_dn20: f64,
        var_qsch4_dn21: f64,
        var_qsch4_dn22: f64,
        var_qsch4_dn23: f64,
        var_qsch4_dn24: f64,
        var_qsch4_dn25: f64,
        var_qsch4_dn26: f64,
        var_qsch4_dn27: f64,
        var_qsch4_dn28: f64,
        var_qsch4_dn29: f64,
        var_qsch4_dn3: f64,
        var_qsch4_dn4: f64,
        var_qsch4_dn5: f64,
        var_qsch4_dn6: f64,
        var_qsch4_dn7: f64,
        var_qsch4_dn8: f64,
        var_qsch4_dn9: f64,
        var_qsch5: f64,
        var_qsch5_db0: f64,
        var_qsch5_db1: f64,
        var_qsch5_db10: f64,
        var_qsch5_db11: f64,
        var_qsch5_db12: f64,
        var_qsch5_db13: f64,
        var_qsch5_db14: f64,
        var_qsch5_db15: f64,
        var_qsch5_db16: f64,
        var_qsch5_db17: f64,
        var_qsch5_db18: f64,
        var_qsch5_db19: f64,
        var_qsch5_db2: f64,
        var_qsch5_db20: f64,
        var_qsch5_db21: f64,
        var_qsch5_db22: f64,
        var_qsch5_db23: f64,
        var_qsch5_db24: f64,
        var_qsch5_db25: f64,
        var_qsch5_db26: f64,
        var_qsch5_db27: f64,
        var_qsch5_db28: f64,
        var_qsch5_db29: f64,
        var_qsch5_db3: f64,
        var_qsch5_db30: f64,
        var_qsch5_db31: f64,
        var_qsch5_db32: f64,
        var_qsch5_db33: f64,
        var_qsch5_db34: f64,
        var_qsch5_db35: f64,
        var_qsch5_db4: f64,
        var_qsch5_db5: f64,
        var_qsch5_db6: f64,
        var_qsch5_db7: f64,
        var_qsch5_db8: f64,
        var_qsch5_db9: f64,
        var_qsch5_dn0: f64,
        var_qsch5_dn1: f64,
        var_qsch5_dn10: f64,
        var_qsch5_dn11: f64,
        var_qsch5_dn12: f64,
        var_qsch5_dn13: f64,
        var_qsch5_dn14: f64,
        var_qsch5_dn15: f64,
        var_qsch5_dn16: f64,
        var_qsch5_dn17: f64,
        var_qsch5_dn18: f64,
        var_qsch5_dn19: f64,
        var_qsch5_dn2: f64,
        var_qsch5_dn20: f64,
        var_qsch5_dn21: f64,
        var_qsch5_dn22: f64,
        var_qsch5_dn23: f64,
        var_qsch5_dn24: f64,
        var_qsch5_dn25: f64,
        var_qsch5_dn26: f64,
        var_qsch5_dn27: f64,
        var_qsch5_dn28: f64,
        var_qsch5_dn29: f64,
        var_qsch5_dn3: f64,
        var_qsch5_dn4: f64,
        var_qsch5_dn5: f64,
        var_qsch5_dn6: f64,
        var_qsch5_dn7: f64,
        var_qsch5_dn8: f64,
        var_qsch5_dn9: f64,
        var_qsch_slot: &mut f64,
        var_qsch1c_slot: &mut f64,
        var_qsch2c_slot: &mut f64,
        var_qsch3c_slot: &mut f64,
        var_qsch_db0_slot: &mut f64,
        var_qsch_db1_slot: &mut f64,
        var_qsch_db10_slot: &mut f64,
        var_qsch_db11_slot: &mut f64,
        var_qsch_db12_slot: &mut f64,
        var_qsch_db13_slot: &mut f64,
        var_qsch_db14_slot: &mut f64,
        var_qsch_db15_slot: &mut f64,
        var_qsch_db16_slot: &mut f64,
        var_qsch_db17_slot: &mut f64,
        var_qsch_db18_slot: &mut f64,
        var_qsch_db19_slot: &mut f64,
        var_qsch_db2_slot: &mut f64,
        var_qsch_db20_slot: &mut f64,
        var_qsch_db21_slot: &mut f64,
        var_qsch_db22_slot: &mut f64,
        var_qsch_db23_slot: &mut f64,
        var_qsch_db24_slot: &mut f64,
        var_qsch_db25_slot: &mut f64,
        var_qsch_db26_slot: &mut f64,
        var_qsch_db27_slot: &mut f64,
        var_qsch_db28_slot: &mut f64,
        var_qsch_db29_slot: &mut f64,
        var_qsch_db3_slot: &mut f64,
        var_qsch_db30_slot: &mut f64,
        var_qsch_db31_slot: &mut f64,
        var_qsch_db32_slot: &mut f64,
        var_qsch_db33_slot: &mut f64,
        var_qsch_db34_slot: &mut f64,
        var_qsch_db35_slot: &mut f64,
        var_qsch_db4_slot: &mut f64,
        var_qsch_db5_slot: &mut f64,
        var_qsch_db6_slot: &mut f64,
        var_qsch_db7_slot: &mut f64,
        var_qsch_db8_slot: &mut f64,
        var_qsch_db9_slot: &mut f64,
        var_qsch_dn0_slot: &mut f64,
        var_qsch_dn1_slot: &mut f64,
        var_qsch_dn10_slot: &mut f64,
        var_qsch_dn11_slot: &mut f64,
        var_qsch_dn12_slot: &mut f64,
        var_qsch_dn13_slot: &mut f64,
        var_qsch_dn14_slot: &mut f64,
        var_qsch_dn15_slot: &mut f64,
        var_qsch_dn16_slot: &mut f64,
        var_qsch_dn17_slot: &mut f64,
        var_qsch_dn18_slot: &mut f64,
        var_qsch_dn19_slot: &mut f64,
        var_qsch_dn2_slot: &mut f64,
        var_qsch_dn20_slot: &mut f64,
        var_qsch_dn21_slot: &mut f64,
        var_qsch_dn22_slot: &mut f64,
        var_qsch_dn23_slot: &mut f64,
        var_qsch_dn24_slot: &mut f64,
        var_qsch_dn25_slot: &mut f64,
        var_qsch_dn26_slot: &mut f64,
        var_qsch_dn27_slot: &mut f64,
        var_qsch_dn28_slot: &mut f64,
        var_qsch_dn29_slot: &mut f64,
        var_qsch_dn3_slot: &mut f64,
        var_qsch_dn4_slot: &mut f64,
        var_qsch_dn5_slot: &mut f64,
        var_qsch_dn6_slot: &mut f64,
        var_qsch_dn7_slot: &mut f64,
        var_qsch_dn8_slot: &mut f64,
        var_qsch_dn9_slot: &mut f64,
    ) {
        let mut var_qsch: f64 = *var_qsch_slot;
        let mut var_qsch1c: f64 = *var_qsch1c_slot;
        let mut var_qsch2c: f64 = *var_qsch2c_slot;
        let mut var_qsch3c: f64 = *var_qsch3c_slot;
        let mut var_qsch_db0: f64 = *var_qsch_db0_slot;
        let mut var_qsch_db1: f64 = *var_qsch_db1_slot;
        let mut var_qsch_db10: f64 = *var_qsch_db10_slot;
        let mut var_qsch_db11: f64 = *var_qsch_db11_slot;
        let mut var_qsch_db12: f64 = *var_qsch_db12_slot;
        let mut var_qsch_db13: f64 = *var_qsch_db13_slot;
        let mut var_qsch_db14: f64 = *var_qsch_db14_slot;
        let mut var_qsch_db15: f64 = *var_qsch_db15_slot;
        let mut var_qsch_db16: f64 = *var_qsch_db16_slot;
        let mut var_qsch_db17: f64 = *var_qsch_db17_slot;
        let mut var_qsch_db18: f64 = *var_qsch_db18_slot;
        let mut var_qsch_db19: f64 = *var_qsch_db19_slot;
        let mut var_qsch_db2: f64 = *var_qsch_db2_slot;
        let mut var_qsch_db20: f64 = *var_qsch_db20_slot;
        let mut var_qsch_db21: f64 = *var_qsch_db21_slot;
        let mut var_qsch_db22: f64 = *var_qsch_db22_slot;
        let mut var_qsch_db23: f64 = *var_qsch_db23_slot;
        let mut var_qsch_db24: f64 = *var_qsch_db24_slot;
        let mut var_qsch_db25: f64 = *var_qsch_db25_slot;
        let mut var_qsch_db26: f64 = *var_qsch_db26_slot;
        let mut var_qsch_db27: f64 = *var_qsch_db27_slot;
        let mut var_qsch_db28: f64 = *var_qsch_db28_slot;
        let mut var_qsch_db29: f64 = *var_qsch_db29_slot;
        let mut var_qsch_db3: f64 = *var_qsch_db3_slot;
        let mut var_qsch_db30: f64 = *var_qsch_db30_slot;
        let mut var_qsch_db31: f64 = *var_qsch_db31_slot;
        let mut var_qsch_db32: f64 = *var_qsch_db32_slot;
        let mut var_qsch_db33: f64 = *var_qsch_db33_slot;
        let mut var_qsch_db34: f64 = *var_qsch_db34_slot;
        let mut var_qsch_db35: f64 = *var_qsch_db35_slot;
        let mut var_qsch_db4: f64 = *var_qsch_db4_slot;
        let mut var_qsch_db5: f64 = *var_qsch_db5_slot;
        let mut var_qsch_db6: f64 = *var_qsch_db6_slot;
        let mut var_qsch_db7: f64 = *var_qsch_db7_slot;
        let mut var_qsch_db8: f64 = *var_qsch_db8_slot;
        let mut var_qsch_db9: f64 = *var_qsch_db9_slot;
        let mut var_qsch_dn0: f64 = *var_qsch_dn0_slot;
        let mut var_qsch_dn1: f64 = *var_qsch_dn1_slot;
        let mut var_qsch_dn10: f64 = *var_qsch_dn10_slot;
        let mut var_qsch_dn11: f64 = *var_qsch_dn11_slot;
        let mut var_qsch_dn12: f64 = *var_qsch_dn12_slot;
        let mut var_qsch_dn13: f64 = *var_qsch_dn13_slot;
        let mut var_qsch_dn14: f64 = *var_qsch_dn14_slot;
        let mut var_qsch_dn15: f64 = *var_qsch_dn15_slot;
        let mut var_qsch_dn16: f64 = *var_qsch_dn16_slot;
        let mut var_qsch_dn17: f64 = *var_qsch_dn17_slot;
        let mut var_qsch_dn18: f64 = *var_qsch_dn18_slot;
        let mut var_qsch_dn19: f64 = *var_qsch_dn19_slot;
        let mut var_qsch_dn2: f64 = *var_qsch_dn2_slot;
        let mut var_qsch_dn20: f64 = *var_qsch_dn20_slot;
        let mut var_qsch_dn21: f64 = *var_qsch_dn21_slot;
        let mut var_qsch_dn22: f64 = *var_qsch_dn22_slot;
        let mut var_qsch_dn23: f64 = *var_qsch_dn23_slot;
        let mut var_qsch_dn24: f64 = *var_qsch_dn24_slot;
        let mut var_qsch_dn25: f64 = *var_qsch_dn25_slot;
        let mut var_qsch_dn26: f64 = *var_qsch_dn26_slot;
        let mut var_qsch_dn27: f64 = *var_qsch_dn27_slot;
        let mut var_qsch_dn28: f64 = *var_qsch_dn28_slot;
        let mut var_qsch_dn29: f64 = *var_qsch_dn29_slot;
        let mut var_qsch_dn3: f64 = *var_qsch_dn3_slot;
        let mut var_qsch_dn4: f64 = *var_qsch_dn4_slot;
        let mut var_qsch_dn5: f64 = *var_qsch_dn5_slot;
        let mut var_qsch_dn6: f64 = *var_qsch_dn6_slot;
        let mut var_qsch_dn7: f64 = *var_qsch_dn7_slot;
        let mut var_qsch_dn8: f64 = *var_qsch_dn8_slot;
        let mut var_qsch_dn9: f64 = *var_qsch_dn9_slot;

        let (assign43870_e42574,) = {
    if (((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 != 0.0)) && (var_guard476 == 0.0)) {
        (0.0,)
    } else {
        (var_qsch3c,)
    }
};
        var_qsch3c = assign43870_e42574;

        let (assign43880_e42586,) = {
    if ((((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 != 0.0)) && (var_guard475 == 0.0)) {
        (0.0,)
    } else {
        (var_qsch2c,)
    }
};
        var_qsch2c = assign43880_e42586;

        let (assign43890_e42596,) = {
    if (((var_guard461 != 0.0) && (var_guard473 == 0.0)) && (var_guard474 == 0.0)) {
        (0.0,)
    } else {
        (var_qsch1c,)
    }
};
        var_qsch1c = assign43890_e42596;

        let (assign43900_e42629, assign43900_e42629_d_n0, assign43900_e42629_d_n1, assign43900_e42629_d_n2, assign43900_e42629_d_n3, assign43900_e42629_d_n4, assign43900_e42629_d_n5, assign43900_e42629_d_n6, assign43900_e42629_d_n7, assign43900_e42629_d_n8, assign43900_e42629_d_n9, assign43900_e42629_d_n10, assign43900_e42629_d_n11, assign43900_e42629_d_n12, assign43900_e42629_d_n13, assign43900_e42629_d_n14, assign43900_e42629_d_n15, assign43900_e42629_d_n16, assign43900_e42629_d_n17, assign43900_e42629_d_n18, assign43900_e42629_d_n19, assign43900_e42629_d_n20, assign43900_e42629_d_n21, assign43900_e42629_d_n22, assign43900_e42629_d_n23, assign43900_e42629_d_n24, assign43900_e42629_d_n25, assign43900_e42629_d_n26, assign43900_e42629_d_n27, assign43900_e42629_d_n28, assign43900_e42629_d_n29, assign43900_e42629_d_b0, assign43900_e42629_d_b1, assign43900_e42629_d_b2, assign43900_e42629_d_b3, assign43900_e42629_d_b4, assign43900_e42629_d_b5, assign43900_e42629_d_b6, assign43900_e42629_d_b7, assign43900_e42629_d_b8, assign43900_e42629_d_b9, assign43900_e42629_d_b10, assign43900_e42629_d_b11, assign43900_e42629_d_b12, assign43900_e42629_d_b13, assign43900_e42629_d_b14, assign43900_e42629_d_b15, assign43900_e42629_d_b16, assign43900_e42629_d_b17, assign43900_e42629_d_b18, assign43900_e42629_d_b19, assign43900_e42629_d_b20, assign43900_e42629_d_b21, assign43900_e42629_d_b22, assign43900_e42629_d_b23, assign43900_e42629_d_b24, assign43900_e42629_d_b25, assign43900_e42629_d_b26, assign43900_e42629_d_b27, assign43900_e42629_d_b28, assign43900_e42629_d_b29, assign43900_e42629_d_b30, assign43900_e42629_d_b31, assign43900_e42629_d_b32, assign43900_e42629_d_b33, assign43900_e42629_d_b34, assign43900_e42629_d_b35,) = {
    if ((var_guard461 != 0.0) && (var_guard473 == 0.0)) {
        let assign43900_e42603: f64 = (p.p6 * 2.0);
        let assign43900_e42605: f64 = (assign43900_e42603 * p.p307);
        let assign43900_e42607: f64 = (assign43900_e42605 * p.p0);
        let assign43900_e42610: f64 = (1.0 - p.p311);
        let assign43900_e42611: f64 = (assign43900_e42607 * assign43900_e42610);
        let assign43900_e42613: f64 = (assign43900_e42611 * p.p2);
        let assign43900_e42615: f64 = (assign43900_e42613 * p.p306);
        let assign43900_e42618: f64 = (var_qsch0 + var_qsch1);
        let assign43900_e42620: f64 = (assign43900_e42618 + var_qsch2);
        let assign43900_e42622: f64 = (assign43900_e42620 + var_qsch3);
        let assign43900_e42624: f64 = (assign43900_e42622 + var_qsch4);
        let assign43900_e42626: f64 = (assign43900_e42624 + var_qsch5);
        let assign43900_e42627: f64 = (assign43900_e42615 * assign43900_e42626);
        (assign43900_e42627, (assign43900_e42615 * ((((var_qsch1_dn0 + var_qsch2_dn0) + var_qsch3_dn0) + var_qsch4_dn0) + var_qsch5_dn0)), (assign43900_e42615 * ((((var_qsch1_dn1 + var_qsch2_dn1) + var_qsch3_dn1) + var_qsch4_dn1) + var_qsch5_dn1)), (assign43900_e42615 * ((((var_qsch1_dn2 + var_qsch2_dn2) + var_qsch3_dn2) + var_qsch4_dn2) + var_qsch5_dn2)), (assign43900_e42615 * ((((var_qsch1_dn3 + var_qsch2_dn3) + var_qsch3_dn3) + var_qsch4_dn3) + var_qsch5_dn3)), (assign43900_e42615 * ((((var_qsch1_dn4 + var_qsch2_dn4) + var_qsch3_dn4) + var_qsch4_dn4) + var_qsch5_dn4)), (assign43900_e42615 * ((((var_qsch1_dn5 + var_qsch2_dn5) + var_qsch3_dn5) + var_qsch4_dn5) + var_qsch5_dn5)), (assign43900_e42615 * ((((var_qsch1_dn6 + var_qsch2_dn6) + var_qsch3_dn6) + var_qsch4_dn6) + var_qsch5_dn6)), (assign43900_e42615 * ((((var_qsch1_dn7 + var_qsch2_dn7) + var_qsch3_dn7) + var_qsch4_dn7) + var_qsch5_dn7)), (assign43900_e42615 * ((((var_qsch1_dn8 + var_qsch2_dn8) + var_qsch3_dn8) + var_qsch4_dn8) + var_qsch5_dn8)), (assign43900_e42615 * ((((var_qsch1_dn9 + var_qsch2_dn9) + var_qsch3_dn9) + var_qsch4_dn9) + var_qsch5_dn9)), (assign43900_e42615 * ((((var_qsch1_dn10 + var_qsch2_dn10) + var_qsch3_dn10) + var_qsch4_dn10) + var_qsch5_dn10)), (assign43900_e42615 * ((((var_qsch1_dn11 + var_qsch2_dn11) + var_qsch3_dn11) + var_qsch4_dn11) + var_qsch5_dn11)), (assign43900_e42615 * ((((var_qsch1_dn12 + var_qsch2_dn12) + var_qsch3_dn12) + var_qsch4_dn12) + var_qsch5_dn12)), (assign43900_e42615 * ((((var_qsch1_dn13 + var_qsch2_dn13) + var_qsch3_dn13) + var_qsch4_dn13) + var_qsch5_dn13)), (assign43900_e42615 * ((((var_qsch1_dn14 + var_qsch2_dn14) + var_qsch3_dn14) + var_qsch4_dn14) + var_qsch5_dn14)), (assign43900_e42615 * ((((var_qsch1_dn15 + var_qsch2_dn15) + var_qsch3_dn15) + var_qsch4_dn15) + var_qsch5_dn15)), (assign43900_e42615 * ((((var_qsch1_dn16 + var_qsch2_dn16) + var_qsch3_dn16) + var_qsch4_dn16) + var_qsch5_dn16)), (assign43900_e42615 * ((((var_qsch1_dn17 + var_qsch2_dn17) + var_qsch3_dn17) + var_qsch4_dn17) + var_qsch5_dn17)), (assign43900_e42615 * ((((var_qsch1_dn18 + var_qsch2_dn18) + var_qsch3_dn18) + var_qsch4_dn18) + var_qsch5_dn18)), (assign43900_e42615 * ((((var_qsch1_dn19 + var_qsch2_dn19) + var_qsch3_dn19) + var_qsch4_dn19) + var_qsch5_dn19)), (assign43900_e42615 * ((((var_qsch1_dn20 + var_qsch2_dn20) + var_qsch3_dn20) + var_qsch4_dn20) + var_qsch5_dn20)), (assign43900_e42615 * ((((var_qsch1_dn21 + var_qsch2_dn21) + var_qsch3_dn21) + var_qsch4_dn21) + var_qsch5_dn21)), (assign43900_e42615 * ((((var_qsch1_dn22 + var_qsch2_dn22) + var_qsch3_dn22) + var_qsch4_dn22) + var_qsch5_dn22)), (assign43900_e42615 * ((((var_qsch1_dn23 + var_qsch2_dn23) + var_qsch3_dn23) + var_qsch4_dn23) + var_qsch5_dn23)), (assign43900_e42615 * ((((var_qsch1_dn24 + var_qsch2_dn24) + var_qsch3_dn24) + var_qsch4_dn24) + var_qsch5_dn24)), (assign43900_e42615 * ((((var_qsch1_dn25 + var_qsch2_dn25) + var_qsch3_dn25) + var_qsch4_dn25) + var_qsch5_dn25)), (assign43900_e42615 * ((((var_qsch1_dn26 + var_qsch2_dn26) + var_qsch3_dn26) + var_qsch4_dn26) + var_qsch5_dn26)), (assign43900_e42615 * ((((var_qsch1_dn27 + var_qsch2_dn27) + var_qsch3_dn27) + var_qsch4_dn27) + var_qsch5_dn27)), (assign43900_e42615 * ((((var_qsch1_dn28 + var_qsch2_dn28) + var_qsch3_dn28) + var_qsch4_dn28) + var_qsch5_dn28)), (assign43900_e42615 * ((((var_qsch1_dn29 + var_qsch2_dn29) + var_qsch3_dn29) + var_qsch4_dn29) + var_qsch5_dn29)), (assign43900_e42615 * ((((var_qsch1_db0 + var_qsch2_db0) + var_qsch3_db0) + var_qsch4_db0) + var_qsch5_db0)), (assign43900_e42615 * ((((var_qsch1_db1 + var_qsch2_db1) + var_qsch3_db1) + var_qsch4_db1) + var_qsch5_db1)), (assign43900_e42615 * ((((var_qsch1_db2 + var_qsch2_db2) + var_qsch3_db2) + var_qsch4_db2) + var_qsch5_db2)), (assign43900_e42615 * ((((var_qsch1_db3 + var_qsch2_db3) + var_qsch3_db3) + var_qsch4_db3) + var_qsch5_db3)), (assign43900_e42615 * ((((var_qsch1_db4 + var_qsch2_db4) + var_qsch3_db4) + var_qsch4_db4) + var_qsch5_db4)), (assign43900_e42615 * ((((var_qsch1_db5 + var_qsch2_db5) + var_qsch3_db5) + var_qsch4_db5) + var_qsch5_db5)), (assign43900_e42615 * ((((var_qsch1_db6 + var_qsch2_db6) + var_qsch3_db6) + var_qsch4_db6) + var_qsch5_db6)), (assign43900_e42615 * ((((var_qsch1_db7 + var_qsch2_db7) + var_qsch3_db7) + var_qsch4_db7) + var_qsch5_db7)), (assign43900_e42615 * ((((var_qsch1_db8 + var_qsch2_db8) + var_qsch3_db8) + var_qsch4_db8) + var_qsch5_db8)), (assign43900_e42615 * ((((var_qsch1_db9 + var_qsch2_db9) + var_qsch3_db9) + var_qsch4_db9) + var_qsch5_db9)), (assign43900_e42615 * ((((var_qsch1_db10 + var_qsch2_db10) + var_qsch3_db10) + var_qsch4_db10) + var_qsch5_db10)), (assign43900_e42615 * ((((var_qsch1_db11 + var_qsch2_db11) + var_qsch3_db11) + var_qsch4_db11) + var_qsch5_db11)), (assign43900_e42615 * ((((var_qsch1_db12 + var_qsch2_db12) + var_qsch3_db12) + var_qsch4_db12) + var_qsch5_db12)), (assign43900_e42615 * ((((var_qsch1_db13 + var_qsch2_db13) + var_qsch3_db13) + var_qsch4_db13) + var_qsch5_db13)), (assign43900_e42615 * ((((var_qsch1_db14 + var_qsch2_db14) + var_qsch3_db14) + var_qsch4_db14) + var_qsch5_db14)), (assign43900_e42615 * ((((var_qsch1_db15 + var_qsch2_db15) + var_qsch3_db15) + var_qsch4_db15) + var_qsch5_db15)), (assign43900_e42615 * ((((var_qsch1_db16 + var_qsch2_db16) + var_qsch3_db16) + var_qsch4_db16) + var_qsch5_db16)), (assign43900_e42615 * ((((var_qsch1_db17 + var_qsch2_db17) + var_qsch3_db17) + var_qsch4_db17) + var_qsch5_db17)), (assign43900_e42615 * ((((var_qsch1_db18 + var_qsch2_db18) + var_qsch3_db18) + var_qsch4_db18) + var_qsch5_db18)), (assign43900_e42615 * ((((var_qsch1_db19 + var_qsch2_db19) + var_qsch3_db19) + var_qsch4_db19) + var_qsch5_db19)), (assign43900_e42615 * ((((var_qsch1_db20 + var_qsch2_db20) + var_qsch3_db20) + var_qsch4_db20) + var_qsch5_db20)), (assign43900_e42615 * ((((var_qsch1_db21 + var_qsch2_db21) + var_qsch3_db21) + var_qsch4_db21) + var_qsch5_db21)), (assign43900_e42615 * ((((var_qsch1_db22 + var_qsch2_db22) + var_qsch3_db22) + var_qsch4_db22) + var_qsch5_db22)), (assign43900_e42615 * ((((var_qsch1_db23 + var_qsch2_db23) + var_qsch3_db23) + var_qsch4_db23) + var_qsch5_db23)), (assign43900_e42615 * ((((var_qsch1_db24 + var_qsch2_db24) + var_qsch3_db24) + var_qsch4_db24) + var_qsch5_db24)), (assign43900_e42615 * ((((var_qsch1_db25 + var_qsch2_db25) + var_qsch3_db25) + var_qsch4_db25) + var_qsch5_db25)), (assign43900_e42615 * ((((var_qsch1_db26 + var_qsch2_db26) + var_qsch3_db26) + var_qsch4_db26) + var_qsch5_db26)), (assign43900_e42615 * ((((var_qsch1_db27 + var_qsch2_db27) + var_qsch3_db27) + var_qsch4_db27) + var_qsch5_db27)), (assign43900_e42615 * ((((var_qsch1_db28 + var_qsch2_db28) + var_qsch3_db28) + var_qsch4_db28) + var_qsch5_db28)), (assign43900_e42615 * ((((var_qsch1_db29 + var_qsch2_db29) + var_qsch3_db29) + var_qsch4_db29) + var_qsch5_db29)), (assign43900_e42615 * ((((var_qsch1_db30 + var_qsch2_db30) + var_qsch3_db30) + var_qsch4_db30) + var_qsch5_db30)), (assign43900_e42615 * ((((var_qsch1_db31 + var_qsch2_db31) + var_qsch3_db31) + var_qsch4_db31) + var_qsch5_db31)), (assign43900_e42615 * ((((var_qsch1_db32 + var_qsch2_db32) + var_qsch3_db32) + var_qsch4_db32) + var_qsch5_db32)), (assign43900_e42615 * ((((var_qsch1_db33 + var_qsch2_db33) + var_qsch3_db33) + var_qsch4_db33) + var_qsch5_db33)), (assign43900_e42615 * ((((var_qsch1_db34 + var_qsch2_db34) + var_qsch3_db34) + var_qsch4_db34) + var_qsch5_db34)), (assign43900_e42615 * ((((var_qsch1_db35 + var_qsch2_db35) + var_qsch3_db35) + var_qsch4_db35) + var_qsch5_db35)),)
    } else {
        (var_qsch, var_qsch_dn0, var_qsch_dn1, var_qsch_dn2, var_qsch_dn3, var_qsch_dn4, var_qsch_dn5, var_qsch_dn6, var_qsch_dn7, var_qsch_dn8, var_qsch_dn9, var_qsch_dn10, var_qsch_dn11, var_qsch_dn12, var_qsch_dn13, var_qsch_dn14, var_qsch_dn15, var_qsch_dn16, var_qsch_dn17, var_qsch_dn18, var_qsch_dn19, var_qsch_dn20, var_qsch_dn21, var_qsch_dn22, var_qsch_dn23, var_qsch_dn24, var_qsch_dn25, var_qsch_dn26, var_qsch_dn27, var_qsch_dn28, var_qsch_dn29, var_qsch_db0, var_qsch_db1, var_qsch_db2, var_qsch_db3, var_qsch_db4, var_qsch_db5, var_qsch_db6, var_qsch_db7, var_qsch_db8, var_qsch_db9, var_qsch_db10, var_qsch_db11, var_qsch_db12, var_qsch_db13, var_qsch_db14, var_qsch_db15, var_qsch_db16, var_qsch_db17, var_qsch_db18, var_qsch_db19, var_qsch_db20, var_qsch_db21, var_qsch_db22, var_qsch_db23, var_qsch_db24, var_qsch_db25, var_qsch_db26, var_qsch_db27, var_qsch_db28, var_qsch_db29, var_qsch_db30, var_qsch_db31, var_qsch_db32, var_qsch_db33, var_qsch_db34, var_qsch_db35,)
    }
};
        var_qsch = assign43900_e42629;
        var_qsch_dn0 = assign43900_e42629_d_n0;
        var_qsch_dn1 = assign43900_e42629_d_n1;
        var_qsch_dn2 = assign43900_e42629_d_n2;
        var_qsch_dn3 = assign43900_e42629_d_n3;
        var_qsch_dn4 = assign43900_e42629_d_n4;
        var_qsch_dn5 = assign43900_e42629_d_n5;
        var_qsch_dn6 = assign43900_e42629_d_n6;
        var_qsch_dn7 = assign43900_e42629_d_n7;
        var_qsch_dn8 = assign43900_e42629_d_n8;
        var_qsch_dn9 = assign43900_e42629_d_n9;
        var_qsch_dn10 = assign43900_e42629_d_n10;
        var_qsch_dn11 = assign43900_e42629_d_n11;
        var_qsch_dn12 = assign43900_e42629_d_n12;
        var_qsch_dn13 = assign43900_e42629_d_n13;
        var_qsch_dn14 = assign43900_e42629_d_n14;
        var_qsch_dn15 = assign43900_e42629_d_n15;
        var_qsch_dn16 = assign43900_e42629_d_n16;
        var_qsch_dn17 = assign43900_e42629_d_n17;
        var_qsch_dn18 = assign43900_e42629_d_n18;
        var_qsch_dn19 = assign43900_e42629_d_n19;
        var_qsch_dn20 = assign43900_e42629_d_n20;
        var_qsch_dn21 = assign43900_e42629_d_n21;
        var_qsch_dn22 = assign43900_e42629_d_n22;
        var_qsch_dn23 = assign43900_e42629_d_n23;
        var_qsch_dn24 = assign43900_e42629_d_n24;
        var_qsch_dn25 = assign43900_e42629_d_n25;
        var_qsch_dn26 = assign43900_e42629_d_n26;
        var_qsch_dn27 = assign43900_e42629_d_n27;
        var_qsch_dn28 = assign43900_e42629_d_n28;
        var_qsch_dn29 = assign43900_e42629_d_n29;
        var_qsch_db0 = assign43900_e42629_d_b0;
        var_qsch_db1 = assign43900_e42629_d_b1;
        var_qsch_db2 = assign43900_e42629_d_b2;
        var_qsch_db3 = assign43900_e42629_d_b3;
        var_qsch_db4 = assign43900_e42629_d_b4;
        var_qsch_db5 = assign43900_e42629_d_b5;
        var_qsch_db6 = assign43900_e42629_d_b6;
        var_qsch_db7 = assign43900_e42629_d_b7;
        var_qsch_db8 = assign43900_e42629_d_b8;
        var_qsch_db9 = assign43900_e42629_d_b9;
        var_qsch_db10 = assign43900_e42629_d_b10;
        var_qsch_db11 = assign43900_e42629_d_b11;
        var_qsch_db12 = assign43900_e42629_d_b12;
        var_qsch_db13 = assign43900_e42629_d_b13;
        var_qsch_db14 = assign43900_e42629_d_b14;
        var_qsch_db15 = assign43900_e42629_d_b15;
        var_qsch_db16 = assign43900_e42629_d_b16;
        var_qsch_db17 = assign43900_e42629_d_b17;
        var_qsch_db18 = assign43900_e42629_d_b18;
        var_qsch_db19 = assign43900_e42629_d_b19;
        var_qsch_db20 = assign43900_e42629_d_b20;
        var_qsch_db21 = assign43900_e42629_d_b21;
        var_qsch_db22 = assign43900_e42629_d_b22;
        var_qsch_db23 = assign43900_e42629_d_b23;
        var_qsch_db24 = assign43900_e42629_d_b24;
        var_qsch_db25 = assign43900_e42629_d_b25;
        var_qsch_db26 = assign43900_e42629_d_b26;
        var_qsch_db27 = assign43900_e42629_d_b27;
        var_qsch_db28 = assign43900_e42629_d_b28;
        var_qsch_db29 = assign43900_e42629_d_b29;
        var_qsch_db30 = assign43900_e42629_d_b30;
        var_qsch_db31 = assign43900_e42629_d_b31;
        var_qsch_db32 = assign43900_e42629_d_b32;
        var_qsch_db33 = assign43900_e42629_d_b33;
        var_qsch_db34 = assign43900_e42629_d_b34;
        var_qsch_db35 = assign43900_e42629_d_b35;

        s.store_scalar(148, 0.0);

        s.store_scalar(149, 0.0);

        s.store_add_scaled_voltages(146, ctx, nodes, Some(19), Some(18), p.p6, Some(19), Some(8), p.p6);

        s.store_add_scaled_voltages(147, ctx, nodes, Some(18), Some(19), p.p6, Some(18), Some(8), p.p6);

        s.b[2547] = (p.p312 == 1.0);
        s.store_scalar(2547, if s.b[2547] { 1.0 } else { 0.0 });

        s.b[2548] = (p.p313 == 0.0);
        s.store_scalar(2548, if s.b[2548] { 1.0 } else { 0.0 });

        if (s.b[2547] && s.b[2548]) {
            s.store_add_scaled_voltages(146, ctx, nodes, Some(2), Some(0), p.p6, Some(2), Some(8), p.p6);
            s.store_add_scaled_voltages(147, ctx, nodes, Some(0), Some(2), p.p6, Some(0), Some(8), p.p6);
        }

        if s.b[2547] {
            s.store_scalar(2549, 0.0);
            s.store_scalar(2550, 0.0);
            s.store_scalar(2551, 0.0);
            s.copy_ad(2552, 146);
            s.copy_ad(2553, 113);
            s.store_scalar(2554, p.p260);
            s.store_scalar(2555, p.p262);
            s.store_scalar(2556, p.p261);
            s.store_scalar(2557, 0.0);
            s.store_scalar(2558, p.p317);
            s.store_scalar(2559, p.p316);
            s.copy_ad(2560, 112);
            s.store_scalar(2561, p.p0);
            s.store_scalar(2562, p.p2);
            s.store_scalar(2563, p.p314);
            s.store_scalar(2564, 1.0);
            s.store_scalar(2565, p.p270);
            s.store_scalar(2566, p.p271);
            s.store_scalar(2567, 0.0);
            s.store_scalar(2568, p.p268);
            s.store_scalar(2569, 0.0);
            s.store_scalar(2570, p.p256);
            s.store_scalar(2571, p.p6);
            s.store_scalar(2572, 0.0);
            s.store_scalar(2573, 0.0);
            s.store_scalar(2574, 0.0);
            s.store_scalar(2575, 0.0);
            s.store_scalar(2576, 0.0);
            s.store_scalar(2577, 0.0);
            s.store_scalar(2578, 0.0);
            s.store_scalar(2579, 0.0);
            s.store_scalar(2580, 0.0);
            s.store_scalar(2581, 0.0);
            s.store_scalar(2582, 0.0);
            s.store_scalar(2583, 0.0);
            s.store_scalar(2584, 0.0);
            s.store_scalar(2585, 0.0);
            s.store_scalar(2586, 0.0);
            s.store_scalar(2587, 0.0);
            s.store_scalar(2588, 0.0);
            s.store_scalar(2589, 0.0);
            s.store_scalar(2590, 0.0);
            s.store_scalar(2591, 0.0);
            s.store_scalar(2592, 0.0);
            s.store_scalar(2593, 0.0);
            s.store_scalar(2594, 0.0);
            s.store_scalar(2595, 0.0);
            s.store_scalar(2596, 0.0);
            s.store_scalar(2597, 0.0);
            s.store_scalar(2598, 0.0);
            s.store_scalar(2599, 0.0);
            s.store_scalar(2600, 0.0);
            s.store_scalar(2601, 0.0);
            s.store_scalar(2602, 0.0);
            s.store_scalar(2603, 0.0);
            s.store_scalar(2604, 0.0);
            s.store_mul_scaled_ad_lhs(2584, A::div(s.ad_value(2569), s.ad_value(2553)), 2570, -1.0);
        }

        if s.b[2547] {
            if ((!(s.v[2584] > 50.0)) && (!(s.v[2584] < (-50.0)))) {
                s.store_exp(2574, 2584);
            } else {
                if ((!(s.v[2584] > 50.0)) && (s.v[2584] < (-50.0))) {
                    s.store_scalar(2574, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2584] > 50.0) {
                        s.store_scaled_offset(2574, 2584, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2574, 0.0);
                    }
                }
            }
        }

        if s.b[2547] {
            s.store_add_scaled_product_right_ad(2580, 2584, 1.0, 2558, A::sub_scaled_inputs(s.ad_value(2552), -1.0, s.ad_value(2559), 1.0), 1.0);
            s.store_add_scaled_product_indices(2581, 2584, 1.0, 2558, 2559, -1.0);
        }

        if s.b[2547] {
            if ((!(s.v[2580] > 50.0)) && (!(s.v[2580] < (-50.0)))) {
                s.store_exp(2582, 2580);
            } else {
                if ((!(s.v[2580] > 50.0)) && (s.v[2580] < (-50.0))) {
                    s.store_scalar(2582, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2580] > 50.0) {
                        s.store_scaled_offset(2582, 2580, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2582, 0.0);
                    }
                }
            }
        }

        if s.b[2547] {
            if ((!(s.v[2581] > 50.0)) && (!(s.v[2581] < (-50.0)))) {
                s.store_exp(2583, 2581);
            } else {
                if ((!(s.v[2581] > 50.0)) && (s.v[2581] < (-50.0))) {
                    s.store_scalar(2583, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2581] > 50.0) {
                        s.store_scaled_offset(2583, 2581, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2583, 0.0);
                    }
                }
            }
        }

        if s.b[2547] {
            s.store_sub(2576, 2582, 2583);
            s.store_mul_ad_product_lhs_mixed_ai(2550, A::mul3(s.ad_value(2571), s.ad_value(2561), s.ad_value(2562)), 2563, 2560);
            s.store_add_scaled_product_left_ad(2586, 2584, 1.0, A::div(s.ad_value(2557), s.ad_value(2553)), 2552, 1.0);
        }

        if s.b[2547] {
            if ((!(s.v[2586] > 50.0)) && (!(s.v[2586] < (-50.0)))) {
                s.store_exp(2587, 2586);
            } else {
                if ((!(s.v[2586] > 50.0)) && (s.v[2586] < (-50.0))) {
                    s.store_scalar(2587, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2586] > 50.0) {
                        s.store_scaled_offset(2587, 2586, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2587, 0.0);
                    }
                }
            }
        }

        s.b[2605] = (s.v[2556] == 1.0);
        s.store_scalar(2605, if s.b[2605] { 1.0 } else { 0.0 });

        if (s.b[2547] && s.b[2605]) {
            s.store_mul_sub_ad_rhs(2577, 2550, A::add_scaled_product(s.ad_value(2587), 1.0, s.ad_value(2564), s.ad_value(2576), (-1.0)), s.ad_value(2574));
        }

        if (s.b[2547] && (!s.b[2605])) {
            s.store_add_scaled_product_right_ad(2591, 2584, 1.0, 2558, A::sub_scaled_inputs(s.ad_value(2554), -1.0, s.ad_value(2559), 1.0), 1.0);
        }

        if (s.b[2547] && (!s.b[2605])) {
            if ((!(s.v[2591] > 50.0)) && (!(s.v[2591] < (-50.0)))) {
                s.store_exp(2592, 2591);
            } else {
                if ((!(s.v[2591] > 50.0)) && (s.v[2591] < (-50.0))) {
                    s.store_scalar(2592, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2591] > 50.0) {
                        s.store_scaled_offset(2592, 2591, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2592, 0.0);
                    }
                }
            }
        }

        if (s.b[2547] && (!s.b[2605])) {
            s.store_sub(2593, 2592, 2583);
        }


        *var_qsch_slot = var_qsch;
        *var_qsch1c_slot = var_qsch1c;
        *var_qsch2c_slot = var_qsch2c;
        *var_qsch3c_slot = var_qsch3c;
        *var_qsch_db0_slot = var_qsch_db0;
        *var_qsch_db1_slot = var_qsch_db1;
        *var_qsch_db10_slot = var_qsch_db10;
        *var_qsch_db11_slot = var_qsch_db11;
        *var_qsch_db12_slot = var_qsch_db12;
        *var_qsch_db13_slot = var_qsch_db13;
        *var_qsch_db14_slot = var_qsch_db14;
        *var_qsch_db15_slot = var_qsch_db15;
        *var_qsch_db16_slot = var_qsch_db16;
        *var_qsch_db17_slot = var_qsch_db17;
        *var_qsch_db18_slot = var_qsch_db18;
        *var_qsch_db19_slot = var_qsch_db19;
        *var_qsch_db2_slot = var_qsch_db2;
        *var_qsch_db20_slot = var_qsch_db20;
        *var_qsch_db21_slot = var_qsch_db21;
        *var_qsch_db22_slot = var_qsch_db22;
        *var_qsch_db23_slot = var_qsch_db23;
        *var_qsch_db24_slot = var_qsch_db24;
        *var_qsch_db25_slot = var_qsch_db25;
        *var_qsch_db26_slot = var_qsch_db26;
        *var_qsch_db27_slot = var_qsch_db27;
        *var_qsch_db28_slot = var_qsch_db28;
        *var_qsch_db29_slot = var_qsch_db29;
        *var_qsch_db3_slot = var_qsch_db3;
        *var_qsch_db30_slot = var_qsch_db30;
        *var_qsch_db31_slot = var_qsch_db31;
        *var_qsch_db32_slot = var_qsch_db32;
        *var_qsch_db33_slot = var_qsch_db33;
        *var_qsch_db34_slot = var_qsch_db34;
        *var_qsch_db35_slot = var_qsch_db35;
        *var_qsch_db4_slot = var_qsch_db4;
        *var_qsch_db5_slot = var_qsch_db5;
        *var_qsch_db6_slot = var_qsch_db6;
        *var_qsch_db7_slot = var_qsch_db7;
        *var_qsch_db8_slot = var_qsch_db8;
        *var_qsch_db9_slot = var_qsch_db9;
        *var_qsch_dn0_slot = var_qsch_dn0;
        *var_qsch_dn1_slot = var_qsch_dn1;
        *var_qsch_dn10_slot = var_qsch_dn10;
        *var_qsch_dn11_slot = var_qsch_dn11;
        *var_qsch_dn12_slot = var_qsch_dn12;
        *var_qsch_dn13_slot = var_qsch_dn13;
        *var_qsch_dn14_slot = var_qsch_dn14;
        *var_qsch_dn15_slot = var_qsch_dn15;
        *var_qsch_dn16_slot = var_qsch_dn16;
        *var_qsch_dn17_slot = var_qsch_dn17;
        *var_qsch_dn18_slot = var_qsch_dn18;
        *var_qsch_dn19_slot = var_qsch_dn19;
        *var_qsch_dn2_slot = var_qsch_dn2;
        *var_qsch_dn20_slot = var_qsch_dn20;
        *var_qsch_dn21_slot = var_qsch_dn21;
        *var_qsch_dn22_slot = var_qsch_dn22;
        *var_qsch_dn23_slot = var_qsch_dn23;
        *var_qsch_dn24_slot = var_qsch_dn24;
        *var_qsch_dn25_slot = var_qsch_dn25;
        *var_qsch_dn26_slot = var_qsch_dn26;
        *var_qsch_dn27_slot = var_qsch_dn27;
        *var_qsch_dn28_slot = var_qsch_dn28;
        *var_qsch_dn29_slot = var_qsch_dn29;
        *var_qsch_dn3_slot = var_qsch_dn3;
        *var_qsch_dn4_slot = var_qsch_dn4;
        *var_qsch_dn5_slot = var_qsch_dn5;
        *var_qsch_dn6_slot = var_qsch_dn6;
        *var_qsch_dn7_slot = var_qsch_dn7;
        *var_qsch_dn8_slot = var_qsch_dn8;
        *var_qsch_dn9_slot = var_qsch_dn9;
    }

    pub(super) fn stamp_transient_block_118(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2547] && (!s.b[2605])) {
            s.store_add_scaled_product_left_ad(2594, 2584, 1.0, A::div(s.ad_value(2557), s.ad_value(2553)), 2554, 1.0);
        }

        if (s.b[2547] && (!s.b[2605])) {
            if ((!(s.v[2594] > 50.0)) && (!(s.v[2594] < (-50.0)))) {
                s.store_exp(2595, 2594);
            } else {
                if ((!(s.v[2594] > 50.0)) && (s.v[2594] < (-50.0))) {
                    s.store_scalar(2595, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2594] > 50.0) {
                        s.store_scaled_offset(2595, 2594, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2595, 0.0);
                    }
                }
            }
        }

        if (s.b[2547] && (!s.b[2605])) {
            s.store_sub_ad_lhs(2596, A::add_scaled_product(s.ad_value(2595), 1.0, s.ad_value(2564), s.ad_value(2593), (-1.0)), 2574);
            s.store_mul_sub_ad_rhs(2597, 2550, A::add_scaled_product(s.ad_value(2587), 1.0, s.ad_value(2564), s.ad_value(2576), (-1.0)), s.ad_value(2574));
        }

        s.b[2606] = (s.v[2556] > 0.0);
        s.store_scalar(2606, if s.b[2606] { 1.0 } else { 0.0 });

        if ((s.b[2547] && (!s.b[2605])) && s.b[2606]) {
            s.store_mul(2590, 2556, 2557);
            s.store_add_scaled_product_left_ad(2598, 2584, 1.0, A::div(s.ad_value(2590), s.ad_value(2553)), 2554, 1.0);
        }

        if ((s.b[2547] && (!s.b[2605])) && s.b[2606]) {
            if ((!(s.v[2598] > 50.0)) && (!(s.v[2598] < (-50.0)))) {
                s.store_exp(2599, 2598);
            } else {
                if ((!(s.v[2598] > 50.0)) && (s.v[2598] < (-50.0))) {
                    s.store_scalar(2599, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2598] > 50.0) {
                        s.store_scaled_offset(2599, 2598, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2599, 0.0);
                    }
                }
            }
        }

        if ((s.b[2547] && (!s.b[2605])) && s.b[2606]) {
            s.store_sub_ad_lhs(2600, A::add_scaled_product(s.ad_value(2599), 1.0, s.ad_value(2564), s.ad_value(2593), (-1.0)), 2574);
            s.store_add_scaled_product_left_ad(2601, 2584, 1.0, A::div(s.ad_value(2590), s.ad_value(2553)), 2552, 1.0);
        }

        if ((s.b[2547] && (!s.b[2605])) && s.b[2606]) {
            if ((!(s.v[2601] > 50.0)) && (!(s.v[2601] < (-50.0)))) {
                s.store_exp(2602, 2601);
            } else {
                if ((!(s.v[2601] > 50.0)) && (s.v[2601] < (-50.0))) {
                    s.store_scalar(2602, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2601] > 50.0) {
                        s.store_scaled_offset(2602, 2601, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2602, 0.0);
                    }
                }
            }
        }

        if ((s.b[2547] && (!s.b[2605])) && s.b[2606]) {
            s.store_div_scaled_product_indices(2603, 2550, 2596, 1.0, 2600, 1.0);
            s.store_mul_sub_ad_rhs(2604, 2603, A::add_scaled_product(s.ad_value(2602), 1.0, s.ad_value(2564), s.ad_value(2576), (-1.0)), s.ad_value(2574));
        }

        if ((s.b[2547] && (!s.b[2605])) && (!s.b[2606])) {
            s.store_mul(2604, 2550, 2596);
        }

        if (s.b[2547] && (!s.b[2605])) {
            s.store_mul_square_lhs(2573, 2555, 2553);
            s.store_div_scaled_inputs3_indices(2585, 2552, 1.0, 2554, -1.0, 2573, (-(-0.5)), 2573, 1.0);
        }

        s.b[2607] = (s.v[2585] > 50.0);
        s.store_scalar(2607, if s.b[2607] { 1.0 } else { 0.0 });

        if ((s.b[2547] && (!s.b[2605])) && s.b[2607]) {
            s.store_scalar(2575, 0.0);
        }

        s.b[2608] = (s.v[2585] < (-50.0));
        s.store_scalar(2608, if s.b[2608] { 1.0 } else { 0.0 });

        if (((s.b[2547] && (!s.b[2605])) && (!s.b[2607])) && s.b[2608]) {
            s.store_scalar(2575, 1.0);
        }

        if (((s.b[2547] && (!s.b[2605])) && (!s.b[2607])) && (!s.b[2608])) {
            s.store_div_from_scalar_offset_ad(2575, 1.0, A::exp(s.ad_value(2585)), 1.0);
        }

        if (s.b[2547] && (!s.b[2605])) {
            s.store_add_scaled_product_value_ad(2577, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(2575), s.ad_value(2604)), 1.0, 2575, 2597, 1.0);
        }

        if s.b[2547] {
            s.store_div_scaled_inputs_mixed_ia(2578, 2552, -1.0, A::pow(A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::mul(A::div(s.ad_value(2552), s.ad_value(2565)), A::tanh_scaled_input(A::div(s.ad_value(2552), s.ad_value(2565)), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt_square_offset(A::div(s.ad_value(2552), s.ad_value(2565)), p.p53)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(2566)), 1.0), A::div_from_scalar(1.0, s.ad_value(2566))), 1.0);
        }

        if s.b[2547] {
            s.store_mul_ad_product_lhs_mixed_ai(2551, A::mul3_scaled_output(s.ad_value(2571), s.ad_value(2561), s.ad_value(2562), -1.0), 2567, 2560);
            s.store_mul_div_lhs(2588, 2568, 2553, 2578);
        }

        if s.b[2547] {
            if ((!(s.v[2588] > 50.0)) && (!(s.v[2588] < (-50.0)))) {
                s.store_exp(2589, 2588);
            } else {
                if ((!(s.v[2588] > 50.0)) && (s.v[2588] < (-50.0))) {
                    s.store_scalar(2589, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2588] > 50.0) {
                        s.store_scaled_offset(2589, 2588, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2589, 0.0);
                    }
                }
            }
        }

        if s.b[2547] {
            s.store_mul_offset_rhs(2579, 2551, 2589, (-1.0));
            s.store_add(2572, 2577, 2579);
            s.copy_ad(2549, 2572);
            s.copy_ad(148, 2549);
            s.store_scalar(2609, 0.0);
            s.store_scalar(2610, 0.0);
            s.store_scalar(2611, 0.0);
            s.copy_ad(2612, 147);
            s.copy_ad(2613, 113);
            s.store_scalar(2614, p.p265);
            s.store_scalar(2615, p.p267);
            s.store_scalar(2616, p.p266);
            s.store_scalar(2617, 0.0);
            s.store_scalar(2618, p.p319);
            s.store_scalar(2619, p.p318);
            s.copy_ad(2620, 112);
            s.store_scalar(2621, p.p0);
            s.store_scalar(2622, p.p2);
            s.store_scalar(2623, p.p315);
            s.store_scalar(2624, 1.0);
            s.store_scalar(2625, p.p274);
            s.store_scalar(2626, p.p275);
            s.store_scalar(2627, 0.0);
            s.store_scalar(2628, p.p272);
            s.store_scalar(2629, 0.0);
            s.store_scalar(2630, p.p256);
            s.store_scalar(2631, p.p6);
            s.store_scalar(2632, 0.0);
            s.store_scalar(2633, 0.0);
            s.store_scalar(2634, 0.0);
            s.store_scalar(2635, 0.0);
            s.store_scalar(2636, 0.0);
            s.store_scalar(2637, 0.0);
            s.store_scalar(2638, 0.0);
            s.store_scalar(2639, 0.0);
            s.store_scalar(2640, 0.0);
            s.store_scalar(2641, 0.0);
            s.store_scalar(2642, 0.0);
            s.store_scalar(2643, 0.0);
            s.store_scalar(2644, 0.0);
            s.store_scalar(2645, 0.0);
            s.store_scalar(2646, 0.0);
            s.store_scalar(2647, 0.0);
            s.store_scalar(2648, 0.0);
            s.store_scalar(2649, 0.0);
            s.store_scalar(2650, 0.0);
            s.store_scalar(2651, 0.0);
            s.store_scalar(2652, 0.0);
            s.store_scalar(2653, 0.0);
            s.store_scalar(2654, 0.0);
            s.store_scalar(2655, 0.0);
            s.store_scalar(2656, 0.0);
            s.store_scalar(2657, 0.0);
            s.store_scalar(2658, 0.0);
            s.store_scalar(2659, 0.0);
            s.store_scalar(2660, 0.0);
            s.store_scalar(2661, 0.0);
            s.store_scalar(2662, 0.0);
            s.store_scalar(2663, 0.0);
            s.store_scalar(2664, 0.0);
            s.store_mul_scaled_ad_lhs(2644, A::div(s.ad_value(2629), s.ad_value(2613)), 2630, -1.0);
        }

        if s.b[2547] {
            if ((!(s.v[2644] > 50.0)) && (!(s.v[2644] < (-50.0)))) {
                s.store_exp(2634, 2644);
            } else {
                if ((!(s.v[2644] > 50.0)) && (s.v[2644] < (-50.0))) {
                    s.store_scalar(2634, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2644] > 50.0) {
                        s.store_scaled_offset(2634, 2644, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2634, 0.0);
                    }
                }
            }
        }

        if s.b[2547] {
            s.store_add_scaled_product_right_ad(2640, 2644, 1.0, 2618, A::sub_scaled_inputs(s.ad_value(2612), -1.0, s.ad_value(2619), 1.0), 1.0);
            s.store_add_scaled_product_indices(2641, 2644, 1.0, 2618, 2619, -1.0);
        }

        if s.b[2547] {
            if ((!(s.v[2640] > 50.0)) && (!(s.v[2640] < (-50.0)))) {
                s.store_exp(2642, 2640);
            } else {
                if ((!(s.v[2640] > 50.0)) && (s.v[2640] < (-50.0))) {
                    s.store_scalar(2642, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2640] > 50.0) {
                        s.store_scaled_offset(2642, 2640, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2642, 0.0);
                    }
                }
            }
        }

        if s.b[2547] {
            if ((!(s.v[2641] > 50.0)) && (!(s.v[2641] < (-50.0)))) {
                s.store_exp(2643, 2641);
            } else {
                if ((!(s.v[2641] > 50.0)) && (s.v[2641] < (-50.0))) {
                    s.store_scalar(2643, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2641] > 50.0) {
                        s.store_scaled_offset(2643, 2641, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2643, 0.0);
                    }
                }
            }
        }

        if s.b[2547] {
            s.store_sub(2636, 2642, 2643);
            s.store_mul_ad_product_lhs_mixed_ai(2610, A::mul3(s.ad_value(2631), s.ad_value(2621), s.ad_value(2622)), 2623, 2620);
        }

    }

    pub(super) fn stamp_transient_block_119(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_cofsmt: f64,
        var_cofsmt0: f64,
        var_cofsmt0_db0: f64,
        var_cofsmt0_db1: f64,
        var_cofsmt0_db10: f64,
        var_cofsmt0_db11: f64,
        var_cofsmt0_db12: f64,
        var_cofsmt0_db13: f64,
        var_cofsmt0_db14: f64,
        var_cofsmt0_db15: f64,
        var_cofsmt0_db16: f64,
        var_cofsmt0_db17: f64,
        var_cofsmt0_db18: f64,
        var_cofsmt0_db19: f64,
        var_cofsmt0_db2: f64,
        var_cofsmt0_db20: f64,
        var_cofsmt0_db21: f64,
        var_cofsmt0_db22: f64,
        var_cofsmt0_db23: f64,
        var_cofsmt0_db24: f64,
        var_cofsmt0_db25: f64,
        var_cofsmt0_db26: f64,
        var_cofsmt0_db27: f64,
        var_cofsmt0_db28: f64,
        var_cofsmt0_db29: f64,
        var_cofsmt0_db3: f64,
        var_cofsmt0_db30: f64,
        var_cofsmt0_db31: f64,
        var_cofsmt0_db32: f64,
        var_cofsmt0_db33: f64,
        var_cofsmt0_db34: f64,
        var_cofsmt0_db35: f64,
        var_cofsmt0_db4: f64,
        var_cofsmt0_db5: f64,
        var_cofsmt0_db6: f64,
        var_cofsmt0_db7: f64,
        var_cofsmt0_db8: f64,
        var_cofsmt0_db9: f64,
        var_cofsmt0_dn0: f64,
        var_cofsmt0_dn1: f64,
        var_cofsmt0_dn10: f64,
        var_cofsmt0_dn11: f64,
        var_cofsmt0_dn12: f64,
        var_cofsmt0_dn13: f64,
        var_cofsmt0_dn14: f64,
        var_cofsmt0_dn15: f64,
        var_cofsmt0_dn16: f64,
        var_cofsmt0_dn17: f64,
        var_cofsmt0_dn18: f64,
        var_cofsmt0_dn19: f64,
        var_cofsmt0_dn2: f64,
        var_cofsmt0_dn20: f64,
        var_cofsmt0_dn21: f64,
        var_cofsmt0_dn22: f64,
        var_cofsmt0_dn23: f64,
        var_cofsmt0_dn24: f64,
        var_cofsmt0_dn25: f64,
        var_cofsmt0_dn26: f64,
        var_cofsmt0_dn27: f64,
        var_cofsmt0_dn28: f64,
        var_cofsmt0_dn29: f64,
        var_cofsmt0_dn3: f64,
        var_cofsmt0_dn4: f64,
        var_cofsmt0_dn5: f64,
        var_cofsmt0_dn6: f64,
        var_cofsmt0_dn7: f64,
        var_cofsmt0_dn8: f64,
        var_cofsmt0_dn9: f64,
        var_cofsmt_db0: f64,
        var_cofsmt_db1: f64,
        var_cofsmt_db10: f64,
        var_cofsmt_db11: f64,
        var_cofsmt_db12: f64,
        var_cofsmt_db13: f64,
        var_cofsmt_db14: f64,
        var_cofsmt_db15: f64,
        var_cofsmt_db16: f64,
        var_cofsmt_db17: f64,
        var_cofsmt_db18: f64,
        var_cofsmt_db19: f64,
        var_cofsmt_db2: f64,
        var_cofsmt_db20: f64,
        var_cofsmt_db21: f64,
        var_cofsmt_db22: f64,
        var_cofsmt_db23: f64,
        var_cofsmt_db24: f64,
        var_cofsmt_db25: f64,
        var_cofsmt_db26: f64,
        var_cofsmt_db27: f64,
        var_cofsmt_db28: f64,
        var_cofsmt_db29: f64,
        var_cofsmt_db3: f64,
        var_cofsmt_db30: f64,
        var_cofsmt_db31: f64,
        var_cofsmt_db32: f64,
        var_cofsmt_db33: f64,
        var_cofsmt_db34: f64,
        var_cofsmt_db35: f64,
        var_cofsmt_db4: f64,
        var_cofsmt_db5: f64,
        var_cofsmt_db6: f64,
        var_cofsmt_db7: f64,
        var_cofsmt_db8: f64,
        var_cofsmt_db9: f64,
        var_cofsmt_dn0: f64,
        var_cofsmt_dn1: f64,
        var_cofsmt_dn10: f64,
        var_cofsmt_dn11: f64,
        var_cofsmt_dn12: f64,
        var_cofsmt_dn13: f64,
        var_cofsmt_dn14: f64,
        var_cofsmt_dn15: f64,
        var_cofsmt_dn16: f64,
        var_cofsmt_dn17: f64,
        var_cofsmt_dn18: f64,
        var_cofsmt_dn19: f64,
        var_cofsmt_dn2: f64,
        var_cofsmt_dn20: f64,
        var_cofsmt_dn21: f64,
        var_cofsmt_dn22: f64,
        var_cofsmt_dn23: f64,
        var_cofsmt_dn24: f64,
        var_cofsmt_dn25: f64,
        var_cofsmt_dn26: f64,
        var_cofsmt_dn27: f64,
        var_cofsmt_dn28: f64,
        var_cofsmt_dn29: f64,
        var_cofsmt_dn3: f64,
        var_cofsmt_dn4: f64,
        var_cofsmt_dn5: f64,
        var_cofsmt_dn6: f64,
        var_cofsmt_dn7: f64,
        var_cofsmt_dn8: f64,
        var_cofsmt_dn9: f64,
        var_rcd_w: f64,
        var_rcs_w: f64,
        var_guard493_slot: &mut f64,
        var_guard494_slot: &mut f64,
        var_guard497_slot: &mut f64,
        var_guard498_slot: &mut f64,
        var_qofs_slot: &mut f64,
        var_qofs_db0_slot: &mut f64,
        var_qofs_db1_slot: &mut f64,
        var_qofs_db10_slot: &mut f64,
        var_qofs_db11_slot: &mut f64,
        var_qofs_db12_slot: &mut f64,
        var_qofs_db13_slot: &mut f64,
        var_qofs_db14_slot: &mut f64,
        var_qofs_db15_slot: &mut f64,
        var_qofs_db16_slot: &mut f64,
        var_qofs_db17_slot: &mut f64,
        var_qofs_db18_slot: &mut f64,
        var_qofs_db19_slot: &mut f64,
        var_qofs_db2_slot: &mut f64,
        var_qofs_db20_slot: &mut f64,
        var_qofs_db21_slot: &mut f64,
        var_qofs_db22_slot: &mut f64,
        var_qofs_db23_slot: &mut f64,
        var_qofs_db24_slot: &mut f64,
        var_qofs_db25_slot: &mut f64,
        var_qofs_db26_slot: &mut f64,
        var_qofs_db27_slot: &mut f64,
        var_qofs_db28_slot: &mut f64,
        var_qofs_db29_slot: &mut f64,
        var_qofs_db3_slot: &mut f64,
        var_qofs_db30_slot: &mut f64,
        var_qofs_db31_slot: &mut f64,
        var_qofs_db32_slot: &mut f64,
        var_qofs_db33_slot: &mut f64,
        var_qofs_db34_slot: &mut f64,
        var_qofs_db35_slot: &mut f64,
        var_qofs_db4_slot: &mut f64,
        var_qofs_db5_slot: &mut f64,
        var_qofs_db6_slot: &mut f64,
        var_qofs_db7_slot: &mut f64,
        var_qofs_db8_slot: &mut f64,
        var_qofs_db9_slot: &mut f64,
        var_qofs_dn0_slot: &mut f64,
        var_qofs_dn1_slot: &mut f64,
        var_qofs_dn10_slot: &mut f64,
        var_qofs_dn11_slot: &mut f64,
        var_qofs_dn12_slot: &mut f64,
        var_qofs_dn13_slot: &mut f64,
        var_qofs_dn14_slot: &mut f64,
        var_qofs_dn15_slot: &mut f64,
        var_qofs_dn16_slot: &mut f64,
        var_qofs_dn17_slot: &mut f64,
        var_qofs_dn18_slot: &mut f64,
        var_qofs_dn19_slot: &mut f64,
        var_qofs_dn2_slot: &mut f64,
        var_qofs_dn20_slot: &mut f64,
        var_qofs_dn21_slot: &mut f64,
        var_qofs_dn22_slot: &mut f64,
        var_qofs_dn23_slot: &mut f64,
        var_qofs_dn24_slot: &mut f64,
        var_qofs_dn25_slot: &mut f64,
        var_qofs_dn26_slot: &mut f64,
        var_qofs_dn27_slot: &mut f64,
        var_qofs_dn28_slot: &mut f64,
        var_qofs_dn29_slot: &mut f64,
        var_qofs_dn3_slot: &mut f64,
        var_qofs_dn4_slot: &mut f64,
        var_qofs_dn5_slot: &mut f64,
        var_qofs_dn6_slot: &mut f64,
        var_qofs_dn7_slot: &mut f64,
        var_qofs_dn8_slot: &mut f64,
        var_qofs_dn9_slot: &mut f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let mut var_guard493: f64 = *var_guard493_slot;
        let mut var_guard494: f64 = *var_guard494_slot;
        let mut var_guard497: f64 = *var_guard497_slot;
        let mut var_guard498: f64 = *var_guard498_slot;
        let mut var_qofs: f64 = *var_qofs_slot;
        let mut var_qofs_db0: f64 = *var_qofs_db0_slot;
        let mut var_qofs_db1: f64 = *var_qofs_db1_slot;
        let mut var_qofs_db10: f64 = *var_qofs_db10_slot;
        let mut var_qofs_db11: f64 = *var_qofs_db11_slot;
        let mut var_qofs_db12: f64 = *var_qofs_db12_slot;
        let mut var_qofs_db13: f64 = *var_qofs_db13_slot;
        let mut var_qofs_db14: f64 = *var_qofs_db14_slot;
        let mut var_qofs_db15: f64 = *var_qofs_db15_slot;
        let mut var_qofs_db16: f64 = *var_qofs_db16_slot;
        let mut var_qofs_db17: f64 = *var_qofs_db17_slot;
        let mut var_qofs_db18: f64 = *var_qofs_db18_slot;
        let mut var_qofs_db19: f64 = *var_qofs_db19_slot;
        let mut var_qofs_db2: f64 = *var_qofs_db2_slot;
        let mut var_qofs_db20: f64 = *var_qofs_db20_slot;
        let mut var_qofs_db21: f64 = *var_qofs_db21_slot;
        let mut var_qofs_db22: f64 = *var_qofs_db22_slot;
        let mut var_qofs_db23: f64 = *var_qofs_db23_slot;
        let mut var_qofs_db24: f64 = *var_qofs_db24_slot;
        let mut var_qofs_db25: f64 = *var_qofs_db25_slot;
        let mut var_qofs_db26: f64 = *var_qofs_db26_slot;
        let mut var_qofs_db27: f64 = *var_qofs_db27_slot;
        let mut var_qofs_db28: f64 = *var_qofs_db28_slot;
        let mut var_qofs_db29: f64 = *var_qofs_db29_slot;
        let mut var_qofs_db3: f64 = *var_qofs_db3_slot;
        let mut var_qofs_db30: f64 = *var_qofs_db30_slot;
        let mut var_qofs_db31: f64 = *var_qofs_db31_slot;
        let mut var_qofs_db32: f64 = *var_qofs_db32_slot;
        let mut var_qofs_db33: f64 = *var_qofs_db33_slot;
        let mut var_qofs_db34: f64 = *var_qofs_db34_slot;
        let mut var_qofs_db35: f64 = *var_qofs_db35_slot;
        let mut var_qofs_db4: f64 = *var_qofs_db4_slot;
        let mut var_qofs_db5: f64 = *var_qofs_db5_slot;
        let mut var_qofs_db6: f64 = *var_qofs_db6_slot;
        let mut var_qofs_db7: f64 = *var_qofs_db7_slot;
        let mut var_qofs_db8: f64 = *var_qofs_db8_slot;
        let mut var_qofs_db9: f64 = *var_qofs_db9_slot;
        let mut var_qofs_dn0: f64 = *var_qofs_dn0_slot;
        let mut var_qofs_dn1: f64 = *var_qofs_dn1_slot;
        let mut var_qofs_dn10: f64 = *var_qofs_dn10_slot;
        let mut var_qofs_dn11: f64 = *var_qofs_dn11_slot;
        let mut var_qofs_dn12: f64 = *var_qofs_dn12_slot;
        let mut var_qofs_dn13: f64 = *var_qofs_dn13_slot;
        let mut var_qofs_dn14: f64 = *var_qofs_dn14_slot;
        let mut var_qofs_dn15: f64 = *var_qofs_dn15_slot;
        let mut var_qofs_dn16: f64 = *var_qofs_dn16_slot;
        let mut var_qofs_dn17: f64 = *var_qofs_dn17_slot;
        let mut var_qofs_dn18: f64 = *var_qofs_dn18_slot;
        let mut var_qofs_dn19: f64 = *var_qofs_dn19_slot;
        let mut var_qofs_dn2: f64 = *var_qofs_dn2_slot;
        let mut var_qofs_dn20: f64 = *var_qofs_dn20_slot;
        let mut var_qofs_dn21: f64 = *var_qofs_dn21_slot;
        let mut var_qofs_dn22: f64 = *var_qofs_dn22_slot;
        let mut var_qofs_dn23: f64 = *var_qofs_dn23_slot;
        let mut var_qofs_dn24: f64 = *var_qofs_dn24_slot;
        let mut var_qofs_dn25: f64 = *var_qofs_dn25_slot;
        let mut var_qofs_dn26: f64 = *var_qofs_dn26_slot;
        let mut var_qofs_dn27: f64 = *var_qofs_dn27_slot;
        let mut var_qofs_dn28: f64 = *var_qofs_dn28_slot;
        let mut var_qofs_dn29: f64 = *var_qofs_dn29_slot;
        let mut var_qofs_dn3: f64 = *var_qofs_dn3_slot;
        let mut var_qofs_dn4: f64 = *var_qofs_dn4_slot;
        let mut var_qofs_dn5: f64 = *var_qofs_dn5_slot;
        let mut var_qofs_dn6: f64 = *var_qofs_dn6_slot;
        let mut var_qofs_dn7: f64 = *var_qofs_dn7_slot;
        let mut var_qofs_dn8: f64 = *var_qofs_dn8_slot;
        let mut var_qofs_dn9: f64 = *var_qofs_dn9_slot;

        if s.b[2547] {
            s.store_add_scaled_product_left_ad(2646, 2644, 1.0, A::div(s.ad_value(2617), s.ad_value(2613)), 2612, 1.0);
        }

        if s.b[2547] {
            if ((!(s.v[2646] > 50.0)) && (!(s.v[2646] < (-50.0)))) {
                s.store_exp(2647, 2646);
            } else {
                if ((!(s.v[2646] > 50.0)) && (s.v[2646] < (-50.0))) {
                    s.store_scalar(2647, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2646] > 50.0) {
                        s.store_scaled_offset(2647, 2646, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2647, 0.0);
                    }
                }
            }
        }

        s.b[2665] = (s.v[2616] == 1.0);
        s.store_scalar(2665, if s.b[2665] { 1.0 } else { 0.0 });

        if (s.b[2547] && s.b[2665]) {
            s.store_mul_sub_ad_rhs(2637, 2610, A::add_scaled_product(s.ad_value(2647), 1.0, s.ad_value(2624), s.ad_value(2636), (-1.0)), s.ad_value(2634));
        }

        if (s.b[2547] && (!s.b[2665])) {
            s.store_add_scaled_product_right_ad(2651, 2644, 1.0, 2618, A::sub_scaled_inputs(s.ad_value(2614), -1.0, s.ad_value(2619), 1.0), 1.0);
        }

        if (s.b[2547] && (!s.b[2665])) {
            if ((!(s.v[2651] > 50.0)) && (!(s.v[2651] < (-50.0)))) {
                s.store_exp(2652, 2651);
            } else {
                if ((!(s.v[2651] > 50.0)) && (s.v[2651] < (-50.0))) {
                    s.store_scalar(2652, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2651] > 50.0) {
                        s.store_scaled_offset(2652, 2651, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2652, 0.0);
                    }
                }
            }
        }

        if (s.b[2547] && (!s.b[2665])) {
            s.store_sub(2653, 2652, 2643);
            s.store_add_scaled_product_left_ad(2654, 2644, 1.0, A::div(s.ad_value(2617), s.ad_value(2613)), 2614, 1.0);
        }

        if (s.b[2547] && (!s.b[2665])) {
            if ((!(s.v[2654] > 50.0)) && (!(s.v[2654] < (-50.0)))) {
                s.store_exp(2655, 2654);
            } else {
                if ((!(s.v[2654] > 50.0)) && (s.v[2654] < (-50.0))) {
                    s.store_scalar(2655, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2654] > 50.0) {
                        s.store_scaled_offset(2655, 2654, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2655, 0.0);
                    }
                }
            }
        }

        if (s.b[2547] && (!s.b[2665])) {
            s.store_sub_ad_lhs(2656, A::add_scaled_product(s.ad_value(2655), 1.0, s.ad_value(2624), s.ad_value(2653), (-1.0)), 2634);
            s.store_mul_sub_ad_rhs(2657, 2610, A::add_scaled_product(s.ad_value(2647), 1.0, s.ad_value(2624), s.ad_value(2636), (-1.0)), s.ad_value(2634));
        }

        s.b[2666] = (s.v[2616] > 0.0);
        s.store_scalar(2666, if s.b[2666] { 1.0 } else { 0.0 });

        if ((s.b[2547] && (!s.b[2665])) && s.b[2666]) {
            s.store_mul(2650, 2616, 2617);
            s.store_add_scaled_product_left_ad(2658, 2644, 1.0, A::div(s.ad_value(2650), s.ad_value(2613)), 2614, 1.0);
        }

        if ((s.b[2547] && (!s.b[2665])) && s.b[2666]) {
            if ((!(s.v[2658] > 50.0)) && (!(s.v[2658] < (-50.0)))) {
                s.store_exp(2659, 2658);
            } else {
                if ((!(s.v[2658] > 50.0)) && (s.v[2658] < (-50.0))) {
                    s.store_scalar(2659, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2658] > 50.0) {
                        s.store_scaled_offset(2659, 2658, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2659, 0.0);
                    }
                }
            }
        }

        if ((s.b[2547] && (!s.b[2665])) && s.b[2666]) {
            s.store_sub_ad_lhs(2660, A::add_scaled_product(s.ad_value(2659), 1.0, s.ad_value(2624), s.ad_value(2653), (-1.0)), 2634);
            s.store_add_scaled_product_left_ad(2661, 2644, 1.0, A::div(s.ad_value(2650), s.ad_value(2613)), 2612, 1.0);
        }

        if ((s.b[2547] && (!s.b[2665])) && s.b[2666]) {
            if ((!(s.v[2661] > 50.0)) && (!(s.v[2661] < (-50.0)))) {
                s.store_exp(2662, 2661);
            } else {
                if ((!(s.v[2661] > 50.0)) && (s.v[2661] < (-50.0))) {
                    s.store_scalar(2662, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2661] > 50.0) {
                        s.store_scaled_offset(2662, 2661, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2662, 0.0);
                    }
                }
            }
        }

        if ((s.b[2547] && (!s.b[2665])) && s.b[2666]) {
            s.store_div_scaled_product_indices(2663, 2610, 2656, 1.0, 2660, 1.0);
            s.store_mul_sub_ad_rhs(2664, 2663, A::add_scaled_product(s.ad_value(2662), 1.0, s.ad_value(2624), s.ad_value(2636), (-1.0)), s.ad_value(2634));
        }

        if ((s.b[2547] && (!s.b[2665])) && (!s.b[2666])) {
            s.store_mul(2664, 2610, 2656);
        }

        if (s.b[2547] && (!s.b[2665])) {
            s.store_mul_square_lhs(2633, 2615, 2613);
            s.store_div_scaled_inputs3_indices(2645, 2612, 1.0, 2614, -1.0, 2633, (-(-0.5)), 2633, 1.0);
        }

        s.b[2667] = (s.v[2645] > 50.0);
        s.store_scalar(2667, if s.b[2667] { 1.0 } else { 0.0 });

        if ((s.b[2547] && (!s.b[2665])) && s.b[2667]) {
            s.store_scalar(2635, 0.0);
        }

        s.b[2668] = (s.v[2645] < (-50.0));
        s.store_scalar(2668, if s.b[2668] { 1.0 } else { 0.0 });

        if (((s.b[2547] && (!s.b[2665])) && (!s.b[2667])) && s.b[2668]) {
            s.store_scalar(2635, 1.0);
        }

        if (((s.b[2547] && (!s.b[2665])) && (!s.b[2667])) && (!s.b[2668])) {
            s.store_div_from_scalar_offset_ad(2635, 1.0, A::exp(s.ad_value(2645)), 1.0);
        }

        if (s.b[2547] && (!s.b[2665])) {
            s.store_add_scaled_product_value_ad(2637, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(2635), s.ad_value(2664)), 1.0, 2635, 2657, 1.0);
        }

        if s.b[2547] {
            s.store_div_scaled_inputs_mixed_ia(2638, 2612, -1.0, A::pow(A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::mul(A::div(s.ad_value(2612), s.ad_value(2625)), A::tanh_scaled_input(A::div(s.ad_value(2612), s.ad_value(2625)), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt_square_offset(A::div(s.ad_value(2612), s.ad_value(2625)), p.p53)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(2626)), 1.0), A::div_from_scalar(1.0, s.ad_value(2626))), 1.0);
        }

        if s.b[2547] {
            s.store_mul_ad_product_lhs_mixed_ai(2611, A::mul3_scaled_output(s.ad_value(2631), s.ad_value(2621), s.ad_value(2622), -1.0), 2627, 2620);
            s.store_mul_div_lhs(2648, 2628, 2613, 2638);
        }

        if s.b[2547] {
            if ((!(s.v[2648] > 50.0)) && (!(s.v[2648] < (-50.0)))) {
                s.store_exp(2649, 2648);
            } else {
                if ((!(s.v[2648] > 50.0)) && (s.v[2648] < (-50.0))) {
                    s.store_scalar(2649, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2648] > 50.0) {
                        s.store_scaled_offset(2649, 2648, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2649, 0.0);
                    }
                }
            }
        }

        if s.b[2547] {
            s.store_mul_offset_rhs(2639, 2611, 2649, (-1.0));
            s.store_add(2632, 2637, 2639);
            s.copy_ad(2609, 2632);
            s.copy_ad(149, 2609);
        }

        s.b[2669] = (p.p313 == 0.0);
        s.store_scalar(2669, if s.b[2669] { 1.0 } else { 0.0 });

        let assign46120_e44786: f64 = if ((var_rcd_w >= p.p353) && (var_rcd_w > 0.0)) { 1.0 } else { 0.0 };
        var_guard493 = assign46120_e44786;

        let assign46130_e44793: f64 = if ((var_rcs_w >= p.p353) && (var_rcs_w > 0.0)) { 1.0 } else { 0.0 };
        var_guard494 = assign46130_e44793;

        let assign46160_e44810: f64 = ((nv6 - nv2) - p.p27);
        let assign46160_e44812: f64 = (assign46160_e44810 / p.p28);
        let assign46160_e44814: f64 = if assign46160_e44812 > 50.0 { 1.0 } else { 0.0 };
        var_guard497 = assign46160_e44814;

        let (assign46170_e44830, assign46170_e44830_d_n0, assign46170_e44830_d_n1, assign46170_e44830_d_n2, assign46170_e44830_d_n3, assign46170_e44830_d_n4, assign46170_e44830_d_n5, assign46170_e44830_d_n6, assign46170_e44830_d_n7, assign46170_e44830_d_n8, assign46170_e44830_d_n9, assign46170_e44830_d_n10, assign46170_e44830_d_n11, assign46170_e44830_d_n12, assign46170_e44830_d_n13, assign46170_e44830_d_n14, assign46170_e44830_d_n15, assign46170_e44830_d_n16, assign46170_e44830_d_n17, assign46170_e44830_d_n18, assign46170_e44830_d_n19, assign46170_e44830_d_n20, assign46170_e44830_d_n21, assign46170_e44830_d_n22, assign46170_e44830_d_n23, assign46170_e44830_d_n24, assign46170_e44830_d_n25, assign46170_e44830_d_n26, assign46170_e44830_d_n27, assign46170_e44830_d_n28, assign46170_e44830_d_n29, assign46170_e44830_d_b0, assign46170_e44830_d_b1, assign46170_e44830_d_b2, assign46170_e44830_d_b3, assign46170_e44830_d_b4, assign46170_e44830_d_b5, assign46170_e44830_d_b6, assign46170_e44830_d_b7, assign46170_e44830_d_b8, assign46170_e44830_d_b9, assign46170_e44830_d_b10, assign46170_e44830_d_b11, assign46170_e44830_d_b12, assign46170_e44830_d_b13, assign46170_e44830_d_b14, assign46170_e44830_d_b15, assign46170_e44830_d_b16, assign46170_e44830_d_b17, assign46170_e44830_d_b18, assign46170_e44830_d_b19, assign46170_e44830_d_b20, assign46170_e44830_d_b21, assign46170_e44830_d_b22, assign46170_e44830_d_b23, assign46170_e44830_d_b24, assign46170_e44830_d_b25, assign46170_e44830_d_b26, assign46170_e44830_d_b27, assign46170_e44830_d_b28, assign46170_e44830_d_b29, assign46170_e44830_d_b30, assign46170_e44830_d_b31, assign46170_e44830_d_b32, assign46170_e44830_d_b33, assign46170_e44830_d_b34, assign46170_e44830_d_b35,) = {
    if (var_guard497 != 0.0) {
        let assign46170_e44818: f64 = (p.p0 * p.p2);
        let assign46170_e44821: f64 = (var_cofsmt0 * (nv6 - nv2));
        let assign46170_e44825: f64 = ((nv6 - nv2) - p.p27);
        let assign46170_e44826: f64 = (var_cofsmt * assign46170_e44825);
        let assign46170_e44827: f64 = (assign46170_e44821 + assign46170_e44826);
        let assign46170_e44828: f64 = (assign46170_e44818 * assign46170_e44827);
        (assign46170_e44828, (assign46170_e44818 * ((var_cofsmt0_dn0 * (nv6 - nv2)) + (var_cofsmt_dn0 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn1 * (nv6 - nv2)) + (var_cofsmt_dn1 * assign46170_e44825))), (assign46170_e44818 * (((var_cofsmt0_dn2 * (nv6 - nv2)) + (-var_cofsmt0)) + ((var_cofsmt_dn2 * assign46170_e44825) + (-var_cofsmt)))), (assign46170_e44818 * ((var_cofsmt0_dn3 * (nv6 - nv2)) + (var_cofsmt_dn3 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn4 * (nv6 - nv2)) + (var_cofsmt_dn4 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn5 * (nv6 - nv2)) + (var_cofsmt_dn5 * assign46170_e44825))), (assign46170_e44818 * (((var_cofsmt0_dn6 * (nv6 - nv2)) + var_cofsmt0) + ((var_cofsmt_dn6 * assign46170_e44825) + var_cofsmt))), (assign46170_e44818 * ((var_cofsmt0_dn7 * (nv6 - nv2)) + (var_cofsmt_dn7 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn8 * (nv6 - nv2)) + (var_cofsmt_dn8 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn9 * (nv6 - nv2)) + (var_cofsmt_dn9 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn10 * (nv6 - nv2)) + (var_cofsmt_dn10 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn11 * (nv6 - nv2)) + (var_cofsmt_dn11 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn12 * (nv6 - nv2)) + (var_cofsmt_dn12 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn13 * (nv6 - nv2)) + (var_cofsmt_dn13 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn14 * (nv6 - nv2)) + (var_cofsmt_dn14 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn15 * (nv6 - nv2)) + (var_cofsmt_dn15 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn16 * (nv6 - nv2)) + (var_cofsmt_dn16 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn17 * (nv6 - nv2)) + (var_cofsmt_dn17 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn18 * (nv6 - nv2)) + (var_cofsmt_dn18 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn19 * (nv6 - nv2)) + (var_cofsmt_dn19 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn20 * (nv6 - nv2)) + (var_cofsmt_dn20 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn21 * (nv6 - nv2)) + (var_cofsmt_dn21 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn22 * (nv6 - nv2)) + (var_cofsmt_dn22 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn23 * (nv6 - nv2)) + (var_cofsmt_dn23 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn24 * (nv6 - nv2)) + (var_cofsmt_dn24 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn25 * (nv6 - nv2)) + (var_cofsmt_dn25 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn26 * (nv6 - nv2)) + (var_cofsmt_dn26 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn27 * (nv6 - nv2)) + (var_cofsmt_dn27 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn28 * (nv6 - nv2)) + (var_cofsmt_dn28 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_dn29 * (nv6 - nv2)) + (var_cofsmt_dn29 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db0 * (nv6 - nv2)) + (var_cofsmt_db0 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db1 * (nv6 - nv2)) + (var_cofsmt_db1 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db2 * (nv6 - nv2)) + (var_cofsmt_db2 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db3 * (nv6 - nv2)) + (var_cofsmt_db3 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db4 * (nv6 - nv2)) + (var_cofsmt_db4 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db5 * (nv6 - nv2)) + (var_cofsmt_db5 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db6 * (nv6 - nv2)) + (var_cofsmt_db6 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db7 * (nv6 - nv2)) + (var_cofsmt_db7 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db8 * (nv6 - nv2)) + (var_cofsmt_db8 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db9 * (nv6 - nv2)) + (var_cofsmt_db9 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db10 * (nv6 - nv2)) + (var_cofsmt_db10 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db11 * (nv6 - nv2)) + (var_cofsmt_db11 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db12 * (nv6 - nv2)) + (var_cofsmt_db12 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db13 * (nv6 - nv2)) + (var_cofsmt_db13 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db14 * (nv6 - nv2)) + (var_cofsmt_db14 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db15 * (nv6 - nv2)) + (var_cofsmt_db15 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db16 * (nv6 - nv2)) + (var_cofsmt_db16 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db17 * (nv6 - nv2)) + (var_cofsmt_db17 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db18 * (nv6 - nv2)) + (var_cofsmt_db18 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db19 * (nv6 - nv2)) + (var_cofsmt_db19 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db20 * (nv6 - nv2)) + (var_cofsmt_db20 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db21 * (nv6 - nv2)) + (var_cofsmt_db21 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db22 * (nv6 - nv2)) + (var_cofsmt_db22 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db23 * (nv6 - nv2)) + (var_cofsmt_db23 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db24 * (nv6 - nv2)) + (var_cofsmt_db24 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db25 * (nv6 - nv2)) + (var_cofsmt_db25 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db26 * (nv6 - nv2)) + (var_cofsmt_db26 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db27 * (nv6 - nv2)) + (var_cofsmt_db27 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db28 * (nv6 - nv2)) + (var_cofsmt_db28 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db29 * (nv6 - nv2)) + (var_cofsmt_db29 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db30 * (nv6 - nv2)) + (var_cofsmt_db30 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db31 * (nv6 - nv2)) + (var_cofsmt_db31 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db32 * (nv6 - nv2)) + (var_cofsmt_db32 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db33 * (nv6 - nv2)) + (var_cofsmt_db33 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db34 * (nv6 - nv2)) + (var_cofsmt_db34 * assign46170_e44825))), (assign46170_e44818 * ((var_cofsmt0_db35 * (nv6 - nv2)) + (var_cofsmt_db35 * assign46170_e44825))),)
    } else {
        (var_qofs, var_qofs_dn0, var_qofs_dn1, var_qofs_dn2, var_qofs_dn3, var_qofs_dn4, var_qofs_dn5, var_qofs_dn6, var_qofs_dn7, var_qofs_dn8, var_qofs_dn9, var_qofs_dn10, var_qofs_dn11, var_qofs_dn12, var_qofs_dn13, var_qofs_dn14, var_qofs_dn15, var_qofs_dn16, var_qofs_dn17, var_qofs_dn18, var_qofs_dn19, var_qofs_dn20, var_qofs_dn21, var_qofs_dn22, var_qofs_dn23, var_qofs_dn24, var_qofs_dn25, var_qofs_dn26, var_qofs_dn27, var_qofs_dn28, var_qofs_dn29, var_qofs_db0, var_qofs_db1, var_qofs_db2, var_qofs_db3, var_qofs_db4, var_qofs_db5, var_qofs_db6, var_qofs_db7, var_qofs_db8, var_qofs_db9, var_qofs_db10, var_qofs_db11, var_qofs_db12, var_qofs_db13, var_qofs_db14, var_qofs_db15, var_qofs_db16, var_qofs_db17, var_qofs_db18, var_qofs_db19, var_qofs_db20, var_qofs_db21, var_qofs_db22, var_qofs_db23, var_qofs_db24, var_qofs_db25, var_qofs_db26, var_qofs_db27, var_qofs_db28, var_qofs_db29, var_qofs_db30, var_qofs_db31, var_qofs_db32, var_qofs_db33, var_qofs_db34, var_qofs_db35,)
    }
};
        var_qofs = assign46170_e44830;
        var_qofs_dn0 = assign46170_e44830_d_n0;
        var_qofs_dn1 = assign46170_e44830_d_n1;
        var_qofs_dn2 = assign46170_e44830_d_n2;
        var_qofs_dn3 = assign46170_e44830_d_n3;
        var_qofs_dn4 = assign46170_e44830_d_n4;
        var_qofs_dn5 = assign46170_e44830_d_n5;
        var_qofs_dn6 = assign46170_e44830_d_n6;
        var_qofs_dn7 = assign46170_e44830_d_n7;
        var_qofs_dn8 = assign46170_e44830_d_n8;
        var_qofs_dn9 = assign46170_e44830_d_n9;
        var_qofs_dn10 = assign46170_e44830_d_n10;
        var_qofs_dn11 = assign46170_e44830_d_n11;
        var_qofs_dn12 = assign46170_e44830_d_n12;
        var_qofs_dn13 = assign46170_e44830_d_n13;
        var_qofs_dn14 = assign46170_e44830_d_n14;
        var_qofs_dn15 = assign46170_e44830_d_n15;
        var_qofs_dn16 = assign46170_e44830_d_n16;
        var_qofs_dn17 = assign46170_e44830_d_n17;
        var_qofs_dn18 = assign46170_e44830_d_n18;
        var_qofs_dn19 = assign46170_e44830_d_n19;
        var_qofs_dn20 = assign46170_e44830_d_n20;
        var_qofs_dn21 = assign46170_e44830_d_n21;
        var_qofs_dn22 = assign46170_e44830_d_n22;
        var_qofs_dn23 = assign46170_e44830_d_n23;
        var_qofs_dn24 = assign46170_e44830_d_n24;
        var_qofs_dn25 = assign46170_e44830_d_n25;
        var_qofs_dn26 = assign46170_e44830_d_n26;
        var_qofs_dn27 = assign46170_e44830_d_n27;
        var_qofs_dn28 = assign46170_e44830_d_n28;
        var_qofs_dn29 = assign46170_e44830_d_n29;
        var_qofs_db0 = assign46170_e44830_d_b0;
        var_qofs_db1 = assign46170_e44830_d_b1;
        var_qofs_db2 = assign46170_e44830_d_b2;
        var_qofs_db3 = assign46170_e44830_d_b3;
        var_qofs_db4 = assign46170_e44830_d_b4;
        var_qofs_db5 = assign46170_e44830_d_b5;
        var_qofs_db6 = assign46170_e44830_d_b6;
        var_qofs_db7 = assign46170_e44830_d_b7;
        var_qofs_db8 = assign46170_e44830_d_b8;
        var_qofs_db9 = assign46170_e44830_d_b9;
        var_qofs_db10 = assign46170_e44830_d_b10;
        var_qofs_db11 = assign46170_e44830_d_b11;
        var_qofs_db12 = assign46170_e44830_d_b12;
        var_qofs_db13 = assign46170_e44830_d_b13;
        var_qofs_db14 = assign46170_e44830_d_b14;
        var_qofs_db15 = assign46170_e44830_d_b15;
        var_qofs_db16 = assign46170_e44830_d_b16;
        var_qofs_db17 = assign46170_e44830_d_b17;
        var_qofs_db18 = assign46170_e44830_d_b18;
        var_qofs_db19 = assign46170_e44830_d_b19;
        var_qofs_db20 = assign46170_e44830_d_b20;
        var_qofs_db21 = assign46170_e44830_d_b21;
        var_qofs_db22 = assign46170_e44830_d_b22;
        var_qofs_db23 = assign46170_e44830_d_b23;
        var_qofs_db24 = assign46170_e44830_d_b24;
        var_qofs_db25 = assign46170_e44830_d_b25;
        var_qofs_db26 = assign46170_e44830_d_b26;
        var_qofs_db27 = assign46170_e44830_d_b27;
        var_qofs_db28 = assign46170_e44830_d_b28;
        var_qofs_db29 = assign46170_e44830_d_b29;
        var_qofs_db30 = assign46170_e44830_d_b30;
        var_qofs_db31 = assign46170_e44830_d_b31;
        var_qofs_db32 = assign46170_e44830_d_b32;
        var_qofs_db33 = assign46170_e44830_d_b33;
        var_qofs_db34 = assign46170_e44830_d_b34;
        var_qofs_db35 = assign46170_e44830_d_b35;

        let assign46180_e44833: f64 = ((nv6 - nv2) - p.p27);
        let assign46180_e44835: f64 = (assign46180_e44833 / p.p28);
        let assign46180_e44837: f64 = (-50.0);
        let assign46180_e44838: f64 = if assign46180_e44835 < assign46180_e44837 { 1.0 } else { 0.0 };
        var_guard498 = assign46180_e44838;

        let (assign46190_e44862, assign46190_e44862_d_n0, assign46190_e44862_d_n1, assign46190_e44862_d_n2, assign46190_e44862_d_n3, assign46190_e44862_d_n4, assign46190_e44862_d_n5, assign46190_e44862_d_n6, assign46190_e44862_d_n7, assign46190_e44862_d_n8, assign46190_e44862_d_n9, assign46190_e44862_d_n10, assign46190_e44862_d_n11, assign46190_e44862_d_n12, assign46190_e44862_d_n13, assign46190_e44862_d_n14, assign46190_e44862_d_n15, assign46190_e44862_d_n16, assign46190_e44862_d_n17, assign46190_e44862_d_n18, assign46190_e44862_d_n19, assign46190_e44862_d_n20, assign46190_e44862_d_n21, assign46190_e44862_d_n22, assign46190_e44862_d_n23, assign46190_e44862_d_n24, assign46190_e44862_d_n25, assign46190_e44862_d_n26, assign46190_e44862_d_n27, assign46190_e44862_d_n28, assign46190_e44862_d_n29, assign46190_e44862_d_b0, assign46190_e44862_d_b1, assign46190_e44862_d_b2, assign46190_e44862_d_b3, assign46190_e44862_d_b4, assign46190_e44862_d_b5, assign46190_e44862_d_b6, assign46190_e44862_d_b7, assign46190_e44862_d_b8, assign46190_e44862_d_b9, assign46190_e44862_d_b10, assign46190_e44862_d_b11, assign46190_e44862_d_b12, assign46190_e44862_d_b13, assign46190_e44862_d_b14, assign46190_e44862_d_b15, assign46190_e44862_d_b16, assign46190_e44862_d_b17, assign46190_e44862_d_b18, assign46190_e44862_d_b19, assign46190_e44862_d_b20, assign46190_e44862_d_b21, assign46190_e44862_d_b22, assign46190_e44862_d_b23, assign46190_e44862_d_b24, assign46190_e44862_d_b25, assign46190_e44862_d_b26, assign46190_e44862_d_b27, assign46190_e44862_d_b28, assign46190_e44862_d_b29, assign46190_e44862_d_b30, assign46190_e44862_d_b31, assign46190_e44862_d_b32, assign46190_e44862_d_b33, assign46190_e44862_d_b34, assign46190_e44862_d_b35,) = {
    if ((var_guard497 == 0.0) && (var_guard498 != 0.0)) {
        let assign46190_e44845: f64 = (p.p0 * p.p2);
        let assign46190_e44848: f64 = (var_cofsmt0 * (nv6 - nv2));
        let assign46190_e44851: f64 = (var_cofsmt * p.p28);
        let assign46190_e44854: f64 = ((nv6 - nv2) - p.p27);
        let assign46190_e44856: f64 = (assign46190_e44854 / p.p28);
        let assign46190_e44857: f64 = (assign46190_e44856).exp();
        let assign46190_e44858: f64 = (assign46190_e44851 * assign46190_e44857);
        let assign46190_e44859: f64 = (assign46190_e44848 + assign46190_e44858);
        let assign46190_e44860: f64 = (assign46190_e44845 * assign46190_e44859);
        (assign46190_e44860, (assign46190_e44845 * ((var_cofsmt0_dn0 * (nv6 - nv2)) + ((var_cofsmt_dn0 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn1 * (nv6 - nv2)) + ((var_cofsmt_dn1 * p.p28) * assign46190_e44857))), (assign46190_e44845 * (((var_cofsmt0_dn2 * (nv6 - nv2)) + (-var_cofsmt0)) + (((var_cofsmt_dn2 * p.p28) * assign46190_e44857) + (assign46190_e44851 * (assign46190_e44857 * (-1.0 / p.p28)))))), (assign46190_e44845 * ((var_cofsmt0_dn3 * (nv6 - nv2)) + ((var_cofsmt_dn3 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn4 * (nv6 - nv2)) + ((var_cofsmt_dn4 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn5 * (nv6 - nv2)) + ((var_cofsmt_dn5 * p.p28) * assign46190_e44857))), (assign46190_e44845 * (((var_cofsmt0_dn6 * (nv6 - nv2)) + var_cofsmt0) + (((var_cofsmt_dn6 * p.p28) * assign46190_e44857) + (assign46190_e44851 * (assign46190_e44857 * (1.0 / p.p28)))))), (assign46190_e44845 * ((var_cofsmt0_dn7 * (nv6 - nv2)) + ((var_cofsmt_dn7 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn8 * (nv6 - nv2)) + ((var_cofsmt_dn8 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn9 * (nv6 - nv2)) + ((var_cofsmt_dn9 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn10 * (nv6 - nv2)) + ((var_cofsmt_dn10 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn11 * (nv6 - nv2)) + ((var_cofsmt_dn11 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn12 * (nv6 - nv2)) + ((var_cofsmt_dn12 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn13 * (nv6 - nv2)) + ((var_cofsmt_dn13 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn14 * (nv6 - nv2)) + ((var_cofsmt_dn14 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn15 * (nv6 - nv2)) + ((var_cofsmt_dn15 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn16 * (nv6 - nv2)) + ((var_cofsmt_dn16 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn17 * (nv6 - nv2)) + ((var_cofsmt_dn17 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn18 * (nv6 - nv2)) + ((var_cofsmt_dn18 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn19 * (nv6 - nv2)) + ((var_cofsmt_dn19 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn20 * (nv6 - nv2)) + ((var_cofsmt_dn20 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn21 * (nv6 - nv2)) + ((var_cofsmt_dn21 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn22 * (nv6 - nv2)) + ((var_cofsmt_dn22 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn23 * (nv6 - nv2)) + ((var_cofsmt_dn23 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn24 * (nv6 - nv2)) + ((var_cofsmt_dn24 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn25 * (nv6 - nv2)) + ((var_cofsmt_dn25 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn26 * (nv6 - nv2)) + ((var_cofsmt_dn26 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn27 * (nv6 - nv2)) + ((var_cofsmt_dn27 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn28 * (nv6 - nv2)) + ((var_cofsmt_dn28 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_dn29 * (nv6 - nv2)) + ((var_cofsmt_dn29 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db0 * (nv6 - nv2)) + ((var_cofsmt_db0 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db1 * (nv6 - nv2)) + ((var_cofsmt_db1 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db2 * (nv6 - nv2)) + ((var_cofsmt_db2 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db3 * (nv6 - nv2)) + ((var_cofsmt_db3 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db4 * (nv6 - nv2)) + ((var_cofsmt_db4 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db5 * (nv6 - nv2)) + ((var_cofsmt_db5 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db6 * (nv6 - nv2)) + ((var_cofsmt_db6 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db7 * (nv6 - nv2)) + ((var_cofsmt_db7 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db8 * (nv6 - nv2)) + ((var_cofsmt_db8 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db9 * (nv6 - nv2)) + ((var_cofsmt_db9 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db10 * (nv6 - nv2)) + ((var_cofsmt_db10 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db11 * (nv6 - nv2)) + ((var_cofsmt_db11 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db12 * (nv6 - nv2)) + ((var_cofsmt_db12 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db13 * (nv6 - nv2)) + ((var_cofsmt_db13 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db14 * (nv6 - nv2)) + ((var_cofsmt_db14 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db15 * (nv6 - nv2)) + ((var_cofsmt_db15 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db16 * (nv6 - nv2)) + ((var_cofsmt_db16 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db17 * (nv6 - nv2)) + ((var_cofsmt_db17 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db18 * (nv6 - nv2)) + ((var_cofsmt_db18 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db19 * (nv6 - nv2)) + ((var_cofsmt_db19 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db20 * (nv6 - nv2)) + ((var_cofsmt_db20 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db21 * (nv6 - nv2)) + ((var_cofsmt_db21 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db22 * (nv6 - nv2)) + ((var_cofsmt_db22 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db23 * (nv6 - nv2)) + ((var_cofsmt_db23 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db24 * (nv6 - nv2)) + ((var_cofsmt_db24 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db25 * (nv6 - nv2)) + ((var_cofsmt_db25 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db26 * (nv6 - nv2)) + ((var_cofsmt_db26 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db27 * (nv6 - nv2)) + ((var_cofsmt_db27 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db28 * (nv6 - nv2)) + ((var_cofsmt_db28 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db29 * (nv6 - nv2)) + ((var_cofsmt_db29 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db30 * (nv6 - nv2)) + ((var_cofsmt_db30 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db31 * (nv6 - nv2)) + ((var_cofsmt_db31 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db32 * (nv6 - nv2)) + ((var_cofsmt_db32 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db33 * (nv6 - nv2)) + ((var_cofsmt_db33 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db34 * (nv6 - nv2)) + ((var_cofsmt_db34 * p.p28) * assign46190_e44857))), (assign46190_e44845 * ((var_cofsmt0_db35 * (nv6 - nv2)) + ((var_cofsmt_db35 * p.p28) * assign46190_e44857))),)
    } else {
        (var_qofs, var_qofs_dn0, var_qofs_dn1, var_qofs_dn2, var_qofs_dn3, var_qofs_dn4, var_qofs_dn5, var_qofs_dn6, var_qofs_dn7, var_qofs_dn8, var_qofs_dn9, var_qofs_dn10, var_qofs_dn11, var_qofs_dn12, var_qofs_dn13, var_qofs_dn14, var_qofs_dn15, var_qofs_dn16, var_qofs_dn17, var_qofs_dn18, var_qofs_dn19, var_qofs_dn20, var_qofs_dn21, var_qofs_dn22, var_qofs_dn23, var_qofs_dn24, var_qofs_dn25, var_qofs_dn26, var_qofs_dn27, var_qofs_dn28, var_qofs_dn29, var_qofs_db0, var_qofs_db1, var_qofs_db2, var_qofs_db3, var_qofs_db4, var_qofs_db5, var_qofs_db6, var_qofs_db7, var_qofs_db8, var_qofs_db9, var_qofs_db10, var_qofs_db11, var_qofs_db12, var_qofs_db13, var_qofs_db14, var_qofs_db15, var_qofs_db16, var_qofs_db17, var_qofs_db18, var_qofs_db19, var_qofs_db20, var_qofs_db21, var_qofs_db22, var_qofs_db23, var_qofs_db24, var_qofs_db25, var_qofs_db26, var_qofs_db27, var_qofs_db28, var_qofs_db29, var_qofs_db30, var_qofs_db31, var_qofs_db32, var_qofs_db33, var_qofs_db34, var_qofs_db35,)
    }
};
        var_qofs = assign46190_e44862;
        var_qofs_dn0 = assign46190_e44862_d_n0;
        var_qofs_dn1 = assign46190_e44862_d_n1;
        var_qofs_dn2 = assign46190_e44862_d_n2;
        var_qofs_dn3 = assign46190_e44862_d_n3;
        var_qofs_dn4 = assign46190_e44862_d_n4;
        var_qofs_dn5 = assign46190_e44862_d_n5;
        var_qofs_dn6 = assign46190_e44862_d_n6;
        var_qofs_dn7 = assign46190_e44862_d_n7;
        var_qofs_dn8 = assign46190_e44862_d_n8;
        var_qofs_dn9 = assign46190_e44862_d_n9;
        var_qofs_dn10 = assign46190_e44862_d_n10;
        var_qofs_dn11 = assign46190_e44862_d_n11;
        var_qofs_dn12 = assign46190_e44862_d_n12;
        var_qofs_dn13 = assign46190_e44862_d_n13;
        var_qofs_dn14 = assign46190_e44862_d_n14;
        var_qofs_dn15 = assign46190_e44862_d_n15;
        var_qofs_dn16 = assign46190_e44862_d_n16;
        var_qofs_dn17 = assign46190_e44862_d_n17;
        var_qofs_dn18 = assign46190_e44862_d_n18;
        var_qofs_dn19 = assign46190_e44862_d_n19;
        var_qofs_dn20 = assign46190_e44862_d_n20;
        var_qofs_dn21 = assign46190_e44862_d_n21;
        var_qofs_dn22 = assign46190_e44862_d_n22;
        var_qofs_dn23 = assign46190_e44862_d_n23;
        var_qofs_dn24 = assign46190_e44862_d_n24;
        var_qofs_dn25 = assign46190_e44862_d_n25;
        var_qofs_dn26 = assign46190_e44862_d_n26;
        var_qofs_dn27 = assign46190_e44862_d_n27;
        var_qofs_dn28 = assign46190_e44862_d_n28;
        var_qofs_dn29 = assign46190_e44862_d_n29;
        var_qofs_db0 = assign46190_e44862_d_b0;
        var_qofs_db1 = assign46190_e44862_d_b1;
        var_qofs_db2 = assign46190_e44862_d_b2;
        var_qofs_db3 = assign46190_e44862_d_b3;
        var_qofs_db4 = assign46190_e44862_d_b4;
        var_qofs_db5 = assign46190_e44862_d_b5;
        var_qofs_db6 = assign46190_e44862_d_b6;
        var_qofs_db7 = assign46190_e44862_d_b7;
        var_qofs_db8 = assign46190_e44862_d_b8;
        var_qofs_db9 = assign46190_e44862_d_b9;
        var_qofs_db10 = assign46190_e44862_d_b10;
        var_qofs_db11 = assign46190_e44862_d_b11;
        var_qofs_db12 = assign46190_e44862_d_b12;
        var_qofs_db13 = assign46190_e44862_d_b13;
        var_qofs_db14 = assign46190_e44862_d_b14;
        var_qofs_db15 = assign46190_e44862_d_b15;
        var_qofs_db16 = assign46190_e44862_d_b16;
        var_qofs_db17 = assign46190_e44862_d_b17;
        var_qofs_db18 = assign46190_e44862_d_b18;
        var_qofs_db19 = assign46190_e44862_d_b19;
        var_qofs_db20 = assign46190_e44862_d_b20;
        var_qofs_db21 = assign46190_e44862_d_b21;
        var_qofs_db22 = assign46190_e44862_d_b22;
        var_qofs_db23 = assign46190_e44862_d_b23;
        var_qofs_db24 = assign46190_e44862_d_b24;
        var_qofs_db25 = assign46190_e44862_d_b25;
        var_qofs_db26 = assign46190_e44862_d_b26;
        var_qofs_db27 = assign46190_e44862_d_b27;
        var_qofs_db28 = assign46190_e44862_d_b28;
        var_qofs_db29 = assign46190_e44862_d_b29;
        var_qofs_db30 = assign46190_e44862_d_b30;
        var_qofs_db31 = assign46190_e44862_d_b31;
        var_qofs_db32 = assign46190_e44862_d_b32;
        var_qofs_db33 = assign46190_e44862_d_b33;
        var_qofs_db34 = assign46190_e44862_d_b34;
        var_qofs_db35 = assign46190_e44862_d_b35;


        *var_guard493_slot = var_guard493;
        *var_guard494_slot = var_guard494;
        *var_guard497_slot = var_guard497;
        *var_guard498_slot = var_guard498;
        *var_qofs_slot = var_qofs;
        *var_qofs_db0_slot = var_qofs_db0;
        *var_qofs_db1_slot = var_qofs_db1;
        *var_qofs_db10_slot = var_qofs_db10;
        *var_qofs_db11_slot = var_qofs_db11;
        *var_qofs_db12_slot = var_qofs_db12;
        *var_qofs_db13_slot = var_qofs_db13;
        *var_qofs_db14_slot = var_qofs_db14;
        *var_qofs_db15_slot = var_qofs_db15;
        *var_qofs_db16_slot = var_qofs_db16;
        *var_qofs_db17_slot = var_qofs_db17;
        *var_qofs_db18_slot = var_qofs_db18;
        *var_qofs_db19_slot = var_qofs_db19;
        *var_qofs_db2_slot = var_qofs_db2;
        *var_qofs_db20_slot = var_qofs_db20;
        *var_qofs_db21_slot = var_qofs_db21;
        *var_qofs_db22_slot = var_qofs_db22;
        *var_qofs_db23_slot = var_qofs_db23;
        *var_qofs_db24_slot = var_qofs_db24;
        *var_qofs_db25_slot = var_qofs_db25;
        *var_qofs_db26_slot = var_qofs_db26;
        *var_qofs_db27_slot = var_qofs_db27;
        *var_qofs_db28_slot = var_qofs_db28;
        *var_qofs_db29_slot = var_qofs_db29;
        *var_qofs_db3_slot = var_qofs_db3;
        *var_qofs_db30_slot = var_qofs_db30;
        *var_qofs_db31_slot = var_qofs_db31;
        *var_qofs_db32_slot = var_qofs_db32;
        *var_qofs_db33_slot = var_qofs_db33;
        *var_qofs_db34_slot = var_qofs_db34;
        *var_qofs_db35_slot = var_qofs_db35;
        *var_qofs_db4_slot = var_qofs_db4;
        *var_qofs_db5_slot = var_qofs_db5;
        *var_qofs_db6_slot = var_qofs_db6;
        *var_qofs_db7_slot = var_qofs_db7;
        *var_qofs_db8_slot = var_qofs_db8;
        *var_qofs_db9_slot = var_qofs_db9;
        *var_qofs_dn0_slot = var_qofs_dn0;
        *var_qofs_dn1_slot = var_qofs_dn1;
        *var_qofs_dn10_slot = var_qofs_dn10;
        *var_qofs_dn11_slot = var_qofs_dn11;
        *var_qofs_dn12_slot = var_qofs_dn12;
        *var_qofs_dn13_slot = var_qofs_dn13;
        *var_qofs_dn14_slot = var_qofs_dn14;
        *var_qofs_dn15_slot = var_qofs_dn15;
        *var_qofs_dn16_slot = var_qofs_dn16;
        *var_qofs_dn17_slot = var_qofs_dn17;
        *var_qofs_dn18_slot = var_qofs_dn18;
        *var_qofs_dn19_slot = var_qofs_dn19;
        *var_qofs_dn2_slot = var_qofs_dn2;
        *var_qofs_dn20_slot = var_qofs_dn20;
        *var_qofs_dn21_slot = var_qofs_dn21;
        *var_qofs_dn22_slot = var_qofs_dn22;
        *var_qofs_dn23_slot = var_qofs_dn23;
        *var_qofs_dn24_slot = var_qofs_dn24;
        *var_qofs_dn25_slot = var_qofs_dn25;
        *var_qofs_dn26_slot = var_qofs_dn26;
        *var_qofs_dn27_slot = var_qofs_dn27;
        *var_qofs_dn28_slot = var_qofs_dn28;
        *var_qofs_dn29_slot = var_qofs_dn29;
        *var_qofs_dn3_slot = var_qofs_dn3;
        *var_qofs_dn4_slot = var_qofs_dn4;
        *var_qofs_dn5_slot = var_qofs_dn5;
        *var_qofs_dn6_slot = var_qofs_dn6;
        *var_qofs_dn7_slot = var_qofs_dn7;
        *var_qofs_dn8_slot = var_qofs_dn8;
        *var_qofs_dn9_slot = var_qofs_dn9;
    }

    pub(super) fn stamp_transient_block_120(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_cofdmt: f64,
        var_cofdmt0: f64,
        var_cofdmt0_db0: f64,
        var_cofdmt0_db1: f64,
        var_cofdmt0_db10: f64,
        var_cofdmt0_db11: f64,
        var_cofdmt0_db12: f64,
        var_cofdmt0_db13: f64,
        var_cofdmt0_db14: f64,
        var_cofdmt0_db15: f64,
        var_cofdmt0_db16: f64,
        var_cofdmt0_db17: f64,
        var_cofdmt0_db18: f64,
        var_cofdmt0_db19: f64,
        var_cofdmt0_db2: f64,
        var_cofdmt0_db20: f64,
        var_cofdmt0_db21: f64,
        var_cofdmt0_db22: f64,
        var_cofdmt0_db23: f64,
        var_cofdmt0_db24: f64,
        var_cofdmt0_db25: f64,
        var_cofdmt0_db26: f64,
        var_cofdmt0_db27: f64,
        var_cofdmt0_db28: f64,
        var_cofdmt0_db29: f64,
        var_cofdmt0_db3: f64,
        var_cofdmt0_db30: f64,
        var_cofdmt0_db31: f64,
        var_cofdmt0_db32: f64,
        var_cofdmt0_db33: f64,
        var_cofdmt0_db34: f64,
        var_cofdmt0_db35: f64,
        var_cofdmt0_db4: f64,
        var_cofdmt0_db5: f64,
        var_cofdmt0_db6: f64,
        var_cofdmt0_db7: f64,
        var_cofdmt0_db8: f64,
        var_cofdmt0_db9: f64,
        var_cofdmt0_dn0: f64,
        var_cofdmt0_dn1: f64,
        var_cofdmt0_dn10: f64,
        var_cofdmt0_dn11: f64,
        var_cofdmt0_dn12: f64,
        var_cofdmt0_dn13: f64,
        var_cofdmt0_dn14: f64,
        var_cofdmt0_dn15: f64,
        var_cofdmt0_dn16: f64,
        var_cofdmt0_dn17: f64,
        var_cofdmt0_dn18: f64,
        var_cofdmt0_dn19: f64,
        var_cofdmt0_dn2: f64,
        var_cofdmt0_dn20: f64,
        var_cofdmt0_dn21: f64,
        var_cofdmt0_dn22: f64,
        var_cofdmt0_dn23: f64,
        var_cofdmt0_dn24: f64,
        var_cofdmt0_dn25: f64,
        var_cofdmt0_dn26: f64,
        var_cofdmt0_dn27: f64,
        var_cofdmt0_dn28: f64,
        var_cofdmt0_dn29: f64,
        var_cofdmt0_dn3: f64,
        var_cofdmt0_dn4: f64,
        var_cofdmt0_dn5: f64,
        var_cofdmt0_dn6: f64,
        var_cofdmt0_dn7: f64,
        var_cofdmt0_dn8: f64,
        var_cofdmt0_dn9: f64,
        var_cofdmt_db0: f64,
        var_cofdmt_db1: f64,
        var_cofdmt_db10: f64,
        var_cofdmt_db11: f64,
        var_cofdmt_db12: f64,
        var_cofdmt_db13: f64,
        var_cofdmt_db14: f64,
        var_cofdmt_db15: f64,
        var_cofdmt_db16: f64,
        var_cofdmt_db17: f64,
        var_cofdmt_db18: f64,
        var_cofdmt_db19: f64,
        var_cofdmt_db2: f64,
        var_cofdmt_db20: f64,
        var_cofdmt_db21: f64,
        var_cofdmt_db22: f64,
        var_cofdmt_db23: f64,
        var_cofdmt_db24: f64,
        var_cofdmt_db25: f64,
        var_cofdmt_db26: f64,
        var_cofdmt_db27: f64,
        var_cofdmt_db28: f64,
        var_cofdmt_db29: f64,
        var_cofdmt_db3: f64,
        var_cofdmt_db30: f64,
        var_cofdmt_db31: f64,
        var_cofdmt_db32: f64,
        var_cofdmt_db33: f64,
        var_cofdmt_db34: f64,
        var_cofdmt_db35: f64,
        var_cofdmt_db4: f64,
        var_cofdmt_db5: f64,
        var_cofdmt_db6: f64,
        var_cofdmt_db7: f64,
        var_cofdmt_db8: f64,
        var_cofdmt_db9: f64,
        var_cofdmt_dn0: f64,
        var_cofdmt_dn1: f64,
        var_cofdmt_dn10: f64,
        var_cofdmt_dn11: f64,
        var_cofdmt_dn12: f64,
        var_cofdmt_dn13: f64,
        var_cofdmt_dn14: f64,
        var_cofdmt_dn15: f64,
        var_cofdmt_dn16: f64,
        var_cofdmt_dn17: f64,
        var_cofdmt_dn18: f64,
        var_cofdmt_dn19: f64,
        var_cofdmt_dn2: f64,
        var_cofdmt_dn20: f64,
        var_cofdmt_dn21: f64,
        var_cofdmt_dn22: f64,
        var_cofdmt_dn23: f64,
        var_cofdmt_dn24: f64,
        var_cofdmt_dn25: f64,
        var_cofdmt_dn26: f64,
        var_cofdmt_dn27: f64,
        var_cofdmt_dn28: f64,
        var_cofdmt_dn29: f64,
        var_cofdmt_dn3: f64,
        var_cofdmt_dn4: f64,
        var_cofdmt_dn5: f64,
        var_cofdmt_dn6: f64,
        var_cofdmt_dn7: f64,
        var_cofdmt_dn8: f64,
        var_cofdmt_dn9: f64,
        var_cofdsmt: f64,
        var_cofdsmt0: f64,
        var_cofdsmt0_db0: f64,
        var_cofdsmt0_db1: f64,
        var_cofdsmt0_db10: f64,
        var_cofdsmt0_db11: f64,
        var_cofdsmt0_db12: f64,
        var_cofdsmt0_db13: f64,
        var_cofdsmt0_db14: f64,
        var_cofdsmt0_db15: f64,
        var_cofdsmt0_db16: f64,
        var_cofdsmt0_db17: f64,
        var_cofdsmt0_db18: f64,
        var_cofdsmt0_db19: f64,
        var_cofdsmt0_db2: f64,
        var_cofdsmt0_db20: f64,
        var_cofdsmt0_db21: f64,
        var_cofdsmt0_db22: f64,
        var_cofdsmt0_db23: f64,
        var_cofdsmt0_db24: f64,
        var_cofdsmt0_db25: f64,
        var_cofdsmt0_db26: f64,
        var_cofdsmt0_db27: f64,
        var_cofdsmt0_db28: f64,
        var_cofdsmt0_db29: f64,
        var_cofdsmt0_db3: f64,
        var_cofdsmt0_db30: f64,
        var_cofdsmt0_db31: f64,
        var_cofdsmt0_db32: f64,
        var_cofdsmt0_db33: f64,
        var_cofdsmt0_db34: f64,
        var_cofdsmt0_db35: f64,
        var_cofdsmt0_db4: f64,
        var_cofdsmt0_db5: f64,
        var_cofdsmt0_db6: f64,
        var_cofdsmt0_db7: f64,
        var_cofdsmt0_db8: f64,
        var_cofdsmt0_db9: f64,
        var_cofdsmt0_dn0: f64,
        var_cofdsmt0_dn1: f64,
        var_cofdsmt0_dn10: f64,
        var_cofdsmt0_dn11: f64,
        var_cofdsmt0_dn12: f64,
        var_cofdsmt0_dn13: f64,
        var_cofdsmt0_dn14: f64,
        var_cofdsmt0_dn15: f64,
        var_cofdsmt0_dn16: f64,
        var_cofdsmt0_dn17: f64,
        var_cofdsmt0_dn18: f64,
        var_cofdsmt0_dn19: f64,
        var_cofdsmt0_dn2: f64,
        var_cofdsmt0_dn20: f64,
        var_cofdsmt0_dn21: f64,
        var_cofdsmt0_dn22: f64,
        var_cofdsmt0_dn23: f64,
        var_cofdsmt0_dn24: f64,
        var_cofdsmt0_dn25: f64,
        var_cofdsmt0_dn26: f64,
        var_cofdsmt0_dn27: f64,
        var_cofdsmt0_dn28: f64,
        var_cofdsmt0_dn29: f64,
        var_cofdsmt0_dn3: f64,
        var_cofdsmt0_dn4: f64,
        var_cofdsmt0_dn5: f64,
        var_cofdsmt0_dn6: f64,
        var_cofdsmt0_dn7: f64,
        var_cofdsmt0_dn8: f64,
        var_cofdsmt0_dn9: f64,
        var_cofdsmt_db0: f64,
        var_cofdsmt_db1: f64,
        var_cofdsmt_db10: f64,
        var_cofdsmt_db11: f64,
        var_cofdsmt_db12: f64,
        var_cofdsmt_db13: f64,
        var_cofdsmt_db14: f64,
        var_cofdsmt_db15: f64,
        var_cofdsmt_db16: f64,
        var_cofdsmt_db17: f64,
        var_cofdsmt_db18: f64,
        var_cofdsmt_db19: f64,
        var_cofdsmt_db2: f64,
        var_cofdsmt_db20: f64,
        var_cofdsmt_db21: f64,
        var_cofdsmt_db22: f64,
        var_cofdsmt_db23: f64,
        var_cofdsmt_db24: f64,
        var_cofdsmt_db25: f64,
        var_cofdsmt_db26: f64,
        var_cofdsmt_db27: f64,
        var_cofdsmt_db28: f64,
        var_cofdsmt_db29: f64,
        var_cofdsmt_db3: f64,
        var_cofdsmt_db30: f64,
        var_cofdsmt_db31: f64,
        var_cofdsmt_db32: f64,
        var_cofdsmt_db33: f64,
        var_cofdsmt_db34: f64,
        var_cofdsmt_db35: f64,
        var_cofdsmt_db4: f64,
        var_cofdsmt_db5: f64,
        var_cofdsmt_db6: f64,
        var_cofdsmt_db7: f64,
        var_cofdsmt_db8: f64,
        var_cofdsmt_db9: f64,
        var_cofdsmt_dn0: f64,
        var_cofdsmt_dn1: f64,
        var_cofdsmt_dn10: f64,
        var_cofdsmt_dn11: f64,
        var_cofdsmt_dn12: f64,
        var_cofdsmt_dn13: f64,
        var_cofdsmt_dn14: f64,
        var_cofdsmt_dn15: f64,
        var_cofdsmt_dn16: f64,
        var_cofdsmt_dn17: f64,
        var_cofdsmt_dn18: f64,
        var_cofdsmt_dn19: f64,
        var_cofdsmt_dn2: f64,
        var_cofdsmt_dn20: f64,
        var_cofdsmt_dn21: f64,
        var_cofdsmt_dn22: f64,
        var_cofdsmt_dn23: f64,
        var_cofdsmt_dn24: f64,
        var_cofdsmt_dn25: f64,
        var_cofdsmt_dn26: f64,
        var_cofdsmt_dn27: f64,
        var_cofdsmt_dn28: f64,
        var_cofdsmt_dn29: f64,
        var_cofdsmt_dn3: f64,
        var_cofdsmt_dn4: f64,
        var_cofdsmt_dn5: f64,
        var_cofdsmt_dn6: f64,
        var_cofdsmt_dn7: f64,
        var_cofdsmt_dn8: f64,
        var_cofdsmt_dn9: f64,
        var_cofsmt: f64,
        var_cofsmt0: f64,
        var_cofsmt0_db0: f64,
        var_cofsmt0_db1: f64,
        var_cofsmt0_db10: f64,
        var_cofsmt0_db11: f64,
        var_cofsmt0_db12: f64,
        var_cofsmt0_db13: f64,
        var_cofsmt0_db14: f64,
        var_cofsmt0_db15: f64,
        var_cofsmt0_db16: f64,
        var_cofsmt0_db17: f64,
        var_cofsmt0_db18: f64,
        var_cofsmt0_db19: f64,
        var_cofsmt0_db2: f64,
        var_cofsmt0_db20: f64,
        var_cofsmt0_db21: f64,
        var_cofsmt0_db22: f64,
        var_cofsmt0_db23: f64,
        var_cofsmt0_db24: f64,
        var_cofsmt0_db25: f64,
        var_cofsmt0_db26: f64,
        var_cofsmt0_db27: f64,
        var_cofsmt0_db28: f64,
        var_cofsmt0_db29: f64,
        var_cofsmt0_db3: f64,
        var_cofsmt0_db30: f64,
        var_cofsmt0_db31: f64,
        var_cofsmt0_db32: f64,
        var_cofsmt0_db33: f64,
        var_cofsmt0_db34: f64,
        var_cofsmt0_db35: f64,
        var_cofsmt0_db4: f64,
        var_cofsmt0_db5: f64,
        var_cofsmt0_db6: f64,
        var_cofsmt0_db7: f64,
        var_cofsmt0_db8: f64,
        var_cofsmt0_db9: f64,
        var_cofsmt0_dn0: f64,
        var_cofsmt0_dn1: f64,
        var_cofsmt0_dn10: f64,
        var_cofsmt0_dn11: f64,
        var_cofsmt0_dn12: f64,
        var_cofsmt0_dn13: f64,
        var_cofsmt0_dn14: f64,
        var_cofsmt0_dn15: f64,
        var_cofsmt0_dn16: f64,
        var_cofsmt0_dn17: f64,
        var_cofsmt0_dn18: f64,
        var_cofsmt0_dn19: f64,
        var_cofsmt0_dn2: f64,
        var_cofsmt0_dn20: f64,
        var_cofsmt0_dn21: f64,
        var_cofsmt0_dn22: f64,
        var_cofsmt0_dn23: f64,
        var_cofsmt0_dn24: f64,
        var_cofsmt0_dn25: f64,
        var_cofsmt0_dn26: f64,
        var_cofsmt0_dn27: f64,
        var_cofsmt0_dn28: f64,
        var_cofsmt0_dn29: f64,
        var_cofsmt0_dn3: f64,
        var_cofsmt0_dn4: f64,
        var_cofsmt0_dn5: f64,
        var_cofsmt0_dn6: f64,
        var_cofsmt0_dn7: f64,
        var_cofsmt0_dn8: f64,
        var_cofsmt0_dn9: f64,
        var_cofsmt_db0: f64,
        var_cofsmt_db1: f64,
        var_cofsmt_db10: f64,
        var_cofsmt_db11: f64,
        var_cofsmt_db12: f64,
        var_cofsmt_db13: f64,
        var_cofsmt_db14: f64,
        var_cofsmt_db15: f64,
        var_cofsmt_db16: f64,
        var_cofsmt_db17: f64,
        var_cofsmt_db18: f64,
        var_cofsmt_db19: f64,
        var_cofsmt_db2: f64,
        var_cofsmt_db20: f64,
        var_cofsmt_db21: f64,
        var_cofsmt_db22: f64,
        var_cofsmt_db23: f64,
        var_cofsmt_db24: f64,
        var_cofsmt_db25: f64,
        var_cofsmt_db26: f64,
        var_cofsmt_db27: f64,
        var_cofsmt_db28: f64,
        var_cofsmt_db29: f64,
        var_cofsmt_db3: f64,
        var_cofsmt_db30: f64,
        var_cofsmt_db31: f64,
        var_cofsmt_db32: f64,
        var_cofsmt_db33: f64,
        var_cofsmt_db34: f64,
        var_cofsmt_db35: f64,
        var_cofsmt_db4: f64,
        var_cofsmt_db5: f64,
        var_cofsmt_db6: f64,
        var_cofsmt_db7: f64,
        var_cofsmt_db8: f64,
        var_cofsmt_db9: f64,
        var_cofsmt_dn0: f64,
        var_cofsmt_dn1: f64,
        var_cofsmt_dn10: f64,
        var_cofsmt_dn11: f64,
        var_cofsmt_dn12: f64,
        var_cofsmt_dn13: f64,
        var_cofsmt_dn14: f64,
        var_cofsmt_dn15: f64,
        var_cofsmt_dn16: f64,
        var_cofsmt_dn17: f64,
        var_cofsmt_dn18: f64,
        var_cofsmt_dn19: f64,
        var_cofsmt_dn2: f64,
        var_cofsmt_dn20: f64,
        var_cofsmt_dn21: f64,
        var_cofsmt_dn22: f64,
        var_cofsmt_dn23: f64,
        var_cofsmt_dn24: f64,
        var_cofsmt_dn25: f64,
        var_cofsmt_dn26: f64,
        var_cofsmt_dn27: f64,
        var_cofsmt_dn28: f64,
        var_cofsmt_dn29: f64,
        var_cofsmt_dn3: f64,
        var_cofsmt_dn4: f64,
        var_cofsmt_dn5: f64,
        var_cofsmt_dn6: f64,
        var_cofsmt_dn7: f64,
        var_cofsmt_dn8: f64,
        var_cofsmt_dn9: f64,
        var_guard497: f64,
        var_guard498: f64,
        var_guard499_slot: &mut f64,
        var_guard500_slot: &mut f64,
        var_guard501_slot: &mut f64,
        var_guard502_slot: &mut f64,
        var_qofd_slot: &mut f64,
        var_qofd_db0_slot: &mut f64,
        var_qofd_db1_slot: &mut f64,
        var_qofd_db10_slot: &mut f64,
        var_qofd_db11_slot: &mut f64,
        var_qofd_db12_slot: &mut f64,
        var_qofd_db13_slot: &mut f64,
        var_qofd_db14_slot: &mut f64,
        var_qofd_db15_slot: &mut f64,
        var_qofd_db16_slot: &mut f64,
        var_qofd_db17_slot: &mut f64,
        var_qofd_db18_slot: &mut f64,
        var_qofd_db19_slot: &mut f64,
        var_qofd_db2_slot: &mut f64,
        var_qofd_db20_slot: &mut f64,
        var_qofd_db21_slot: &mut f64,
        var_qofd_db22_slot: &mut f64,
        var_qofd_db23_slot: &mut f64,
        var_qofd_db24_slot: &mut f64,
        var_qofd_db25_slot: &mut f64,
        var_qofd_db26_slot: &mut f64,
        var_qofd_db27_slot: &mut f64,
        var_qofd_db28_slot: &mut f64,
        var_qofd_db29_slot: &mut f64,
        var_qofd_db3_slot: &mut f64,
        var_qofd_db30_slot: &mut f64,
        var_qofd_db31_slot: &mut f64,
        var_qofd_db32_slot: &mut f64,
        var_qofd_db33_slot: &mut f64,
        var_qofd_db34_slot: &mut f64,
        var_qofd_db35_slot: &mut f64,
        var_qofd_db4_slot: &mut f64,
        var_qofd_db5_slot: &mut f64,
        var_qofd_db6_slot: &mut f64,
        var_qofd_db7_slot: &mut f64,
        var_qofd_db8_slot: &mut f64,
        var_qofd_db9_slot: &mut f64,
        var_qofd_dn0_slot: &mut f64,
        var_qofd_dn1_slot: &mut f64,
        var_qofd_dn10_slot: &mut f64,
        var_qofd_dn11_slot: &mut f64,
        var_qofd_dn12_slot: &mut f64,
        var_qofd_dn13_slot: &mut f64,
        var_qofd_dn14_slot: &mut f64,
        var_qofd_dn15_slot: &mut f64,
        var_qofd_dn16_slot: &mut f64,
        var_qofd_dn17_slot: &mut f64,
        var_qofd_dn18_slot: &mut f64,
        var_qofd_dn19_slot: &mut f64,
        var_qofd_dn2_slot: &mut f64,
        var_qofd_dn20_slot: &mut f64,
        var_qofd_dn21_slot: &mut f64,
        var_qofd_dn22_slot: &mut f64,
        var_qofd_dn23_slot: &mut f64,
        var_qofd_dn24_slot: &mut f64,
        var_qofd_dn25_slot: &mut f64,
        var_qofd_dn26_slot: &mut f64,
        var_qofd_dn27_slot: &mut f64,
        var_qofd_dn28_slot: &mut f64,
        var_qofd_dn29_slot: &mut f64,
        var_qofd_dn3_slot: &mut f64,
        var_qofd_dn4_slot: &mut f64,
        var_qofd_dn5_slot: &mut f64,
        var_qofd_dn6_slot: &mut f64,
        var_qofd_dn7_slot: &mut f64,
        var_qofd_dn8_slot: &mut f64,
        var_qofd_dn9_slot: &mut f64,
        var_qofds_slot: &mut f64,
        var_qofds_db0_slot: &mut f64,
        var_qofds_db1_slot: &mut f64,
        var_qofds_db10_slot: &mut f64,
        var_qofds_db11_slot: &mut f64,
        var_qofds_db12_slot: &mut f64,
        var_qofds_db13_slot: &mut f64,
        var_qofds_db14_slot: &mut f64,
        var_qofds_db15_slot: &mut f64,
        var_qofds_db16_slot: &mut f64,
        var_qofds_db17_slot: &mut f64,
        var_qofds_db18_slot: &mut f64,
        var_qofds_db19_slot: &mut f64,
        var_qofds_db2_slot: &mut f64,
        var_qofds_db20_slot: &mut f64,
        var_qofds_db21_slot: &mut f64,
        var_qofds_db22_slot: &mut f64,
        var_qofds_db23_slot: &mut f64,
        var_qofds_db24_slot: &mut f64,
        var_qofds_db25_slot: &mut f64,
        var_qofds_db26_slot: &mut f64,
        var_qofds_db27_slot: &mut f64,
        var_qofds_db28_slot: &mut f64,
        var_qofds_db29_slot: &mut f64,
        var_qofds_db3_slot: &mut f64,
        var_qofds_db30_slot: &mut f64,
        var_qofds_db31_slot: &mut f64,
        var_qofds_db32_slot: &mut f64,
        var_qofds_db33_slot: &mut f64,
        var_qofds_db34_slot: &mut f64,
        var_qofds_db35_slot: &mut f64,
        var_qofds_db4_slot: &mut f64,
        var_qofds_db5_slot: &mut f64,
        var_qofds_db6_slot: &mut f64,
        var_qofds_db7_slot: &mut f64,
        var_qofds_db8_slot: &mut f64,
        var_qofds_db9_slot: &mut f64,
        var_qofds_dn0_slot: &mut f64,
        var_qofds_dn1_slot: &mut f64,
        var_qofds_dn10_slot: &mut f64,
        var_qofds_dn11_slot: &mut f64,
        var_qofds_dn12_slot: &mut f64,
        var_qofds_dn13_slot: &mut f64,
        var_qofds_dn14_slot: &mut f64,
        var_qofds_dn15_slot: &mut f64,
        var_qofds_dn16_slot: &mut f64,
        var_qofds_dn17_slot: &mut f64,
        var_qofds_dn18_slot: &mut f64,
        var_qofds_dn19_slot: &mut f64,
        var_qofds_dn2_slot: &mut f64,
        var_qofds_dn20_slot: &mut f64,
        var_qofds_dn21_slot: &mut f64,
        var_qofds_dn22_slot: &mut f64,
        var_qofds_dn23_slot: &mut f64,
        var_qofds_dn24_slot: &mut f64,
        var_qofds_dn25_slot: &mut f64,
        var_qofds_dn26_slot: &mut f64,
        var_qofds_dn27_slot: &mut f64,
        var_qofds_dn28_slot: &mut f64,
        var_qofds_dn29_slot: &mut f64,
        var_qofds_dn3_slot: &mut f64,
        var_qofds_dn4_slot: &mut f64,
        var_qofds_dn5_slot: &mut f64,
        var_qofds_dn6_slot: &mut f64,
        var_qofds_dn7_slot: &mut f64,
        var_qofds_dn8_slot: &mut f64,
        var_qofds_dn9_slot: &mut f64,
        var_qofs_slot: &mut f64,
        var_qofs_db0_slot: &mut f64,
        var_qofs_db1_slot: &mut f64,
        var_qofs_db10_slot: &mut f64,
        var_qofs_db11_slot: &mut f64,
        var_qofs_db12_slot: &mut f64,
        var_qofs_db13_slot: &mut f64,
        var_qofs_db14_slot: &mut f64,
        var_qofs_db15_slot: &mut f64,
        var_qofs_db16_slot: &mut f64,
        var_qofs_db17_slot: &mut f64,
        var_qofs_db18_slot: &mut f64,
        var_qofs_db19_slot: &mut f64,
        var_qofs_db2_slot: &mut f64,
        var_qofs_db20_slot: &mut f64,
        var_qofs_db21_slot: &mut f64,
        var_qofs_db22_slot: &mut f64,
        var_qofs_db23_slot: &mut f64,
        var_qofs_db24_slot: &mut f64,
        var_qofs_db25_slot: &mut f64,
        var_qofs_db26_slot: &mut f64,
        var_qofs_db27_slot: &mut f64,
        var_qofs_db28_slot: &mut f64,
        var_qofs_db29_slot: &mut f64,
        var_qofs_db3_slot: &mut f64,
        var_qofs_db30_slot: &mut f64,
        var_qofs_db31_slot: &mut f64,
        var_qofs_db32_slot: &mut f64,
        var_qofs_db33_slot: &mut f64,
        var_qofs_db34_slot: &mut f64,
        var_qofs_db35_slot: &mut f64,
        var_qofs_db4_slot: &mut f64,
        var_qofs_db5_slot: &mut f64,
        var_qofs_db6_slot: &mut f64,
        var_qofs_db7_slot: &mut f64,
        var_qofs_db8_slot: &mut f64,
        var_qofs_db9_slot: &mut f64,
        var_qofs_dn0_slot: &mut f64,
        var_qofs_dn1_slot: &mut f64,
        var_qofs_dn10_slot: &mut f64,
        var_qofs_dn11_slot: &mut f64,
        var_qofs_dn12_slot: &mut f64,
        var_qofs_dn13_slot: &mut f64,
        var_qofs_dn14_slot: &mut f64,
        var_qofs_dn15_slot: &mut f64,
        var_qofs_dn16_slot: &mut f64,
        var_qofs_dn17_slot: &mut f64,
        var_qofs_dn18_slot: &mut f64,
        var_qofs_dn19_slot: &mut f64,
        var_qofs_dn2_slot: &mut f64,
        var_qofs_dn20_slot: &mut f64,
        var_qofs_dn21_slot: &mut f64,
        var_qofs_dn22_slot: &mut f64,
        var_qofs_dn23_slot: &mut f64,
        var_qofs_dn24_slot: &mut f64,
        var_qofs_dn25_slot: &mut f64,
        var_qofs_dn26_slot: &mut f64,
        var_qofs_dn27_slot: &mut f64,
        var_qofs_dn28_slot: &mut f64,
        var_qofs_dn29_slot: &mut f64,
        var_qofs_dn3_slot: &mut f64,
        var_qofs_dn4_slot: &mut f64,
        var_qofs_dn5_slot: &mut f64,
        var_qofs_dn6_slot: &mut f64,
        var_qofs_dn7_slot: &mut f64,
        var_qofs_dn8_slot: &mut f64,
        var_qofs_dn9_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let mut var_guard499: f64 = *var_guard499_slot;
        let mut var_guard500: f64 = *var_guard500_slot;
        let mut var_guard501: f64 = *var_guard501_slot;
        let mut var_guard502: f64 = *var_guard502_slot;
        let mut var_qofd: f64 = *var_qofd_slot;
        let mut var_qofd_db0: f64 = *var_qofd_db0_slot;
        let mut var_qofd_db1: f64 = *var_qofd_db1_slot;
        let mut var_qofd_db10: f64 = *var_qofd_db10_slot;
        let mut var_qofd_db11: f64 = *var_qofd_db11_slot;
        let mut var_qofd_db12: f64 = *var_qofd_db12_slot;
        let mut var_qofd_db13: f64 = *var_qofd_db13_slot;
        let mut var_qofd_db14: f64 = *var_qofd_db14_slot;
        let mut var_qofd_db15: f64 = *var_qofd_db15_slot;
        let mut var_qofd_db16: f64 = *var_qofd_db16_slot;
        let mut var_qofd_db17: f64 = *var_qofd_db17_slot;
        let mut var_qofd_db18: f64 = *var_qofd_db18_slot;
        let mut var_qofd_db19: f64 = *var_qofd_db19_slot;
        let mut var_qofd_db2: f64 = *var_qofd_db2_slot;
        let mut var_qofd_db20: f64 = *var_qofd_db20_slot;
        let mut var_qofd_db21: f64 = *var_qofd_db21_slot;
        let mut var_qofd_db22: f64 = *var_qofd_db22_slot;
        let mut var_qofd_db23: f64 = *var_qofd_db23_slot;
        let mut var_qofd_db24: f64 = *var_qofd_db24_slot;
        let mut var_qofd_db25: f64 = *var_qofd_db25_slot;
        let mut var_qofd_db26: f64 = *var_qofd_db26_slot;
        let mut var_qofd_db27: f64 = *var_qofd_db27_slot;
        let mut var_qofd_db28: f64 = *var_qofd_db28_slot;
        let mut var_qofd_db29: f64 = *var_qofd_db29_slot;
        let mut var_qofd_db3: f64 = *var_qofd_db3_slot;
        let mut var_qofd_db30: f64 = *var_qofd_db30_slot;
        let mut var_qofd_db31: f64 = *var_qofd_db31_slot;
        let mut var_qofd_db32: f64 = *var_qofd_db32_slot;
        let mut var_qofd_db33: f64 = *var_qofd_db33_slot;
        let mut var_qofd_db34: f64 = *var_qofd_db34_slot;
        let mut var_qofd_db35: f64 = *var_qofd_db35_slot;
        let mut var_qofd_db4: f64 = *var_qofd_db4_slot;
        let mut var_qofd_db5: f64 = *var_qofd_db5_slot;
        let mut var_qofd_db6: f64 = *var_qofd_db6_slot;
        let mut var_qofd_db7: f64 = *var_qofd_db7_slot;
        let mut var_qofd_db8: f64 = *var_qofd_db8_slot;
        let mut var_qofd_db9: f64 = *var_qofd_db9_slot;
        let mut var_qofd_dn0: f64 = *var_qofd_dn0_slot;
        let mut var_qofd_dn1: f64 = *var_qofd_dn1_slot;
        let mut var_qofd_dn10: f64 = *var_qofd_dn10_slot;
        let mut var_qofd_dn11: f64 = *var_qofd_dn11_slot;
        let mut var_qofd_dn12: f64 = *var_qofd_dn12_slot;
        let mut var_qofd_dn13: f64 = *var_qofd_dn13_slot;
        let mut var_qofd_dn14: f64 = *var_qofd_dn14_slot;
        let mut var_qofd_dn15: f64 = *var_qofd_dn15_slot;
        let mut var_qofd_dn16: f64 = *var_qofd_dn16_slot;
        let mut var_qofd_dn17: f64 = *var_qofd_dn17_slot;
        let mut var_qofd_dn18: f64 = *var_qofd_dn18_slot;
        let mut var_qofd_dn19: f64 = *var_qofd_dn19_slot;
        let mut var_qofd_dn2: f64 = *var_qofd_dn2_slot;
        let mut var_qofd_dn20: f64 = *var_qofd_dn20_slot;
        let mut var_qofd_dn21: f64 = *var_qofd_dn21_slot;
        let mut var_qofd_dn22: f64 = *var_qofd_dn22_slot;
        let mut var_qofd_dn23: f64 = *var_qofd_dn23_slot;
        let mut var_qofd_dn24: f64 = *var_qofd_dn24_slot;
        let mut var_qofd_dn25: f64 = *var_qofd_dn25_slot;
        let mut var_qofd_dn26: f64 = *var_qofd_dn26_slot;
        let mut var_qofd_dn27: f64 = *var_qofd_dn27_slot;
        let mut var_qofd_dn28: f64 = *var_qofd_dn28_slot;
        let mut var_qofd_dn29: f64 = *var_qofd_dn29_slot;
        let mut var_qofd_dn3: f64 = *var_qofd_dn3_slot;
        let mut var_qofd_dn4: f64 = *var_qofd_dn4_slot;
        let mut var_qofd_dn5: f64 = *var_qofd_dn5_slot;
        let mut var_qofd_dn6: f64 = *var_qofd_dn6_slot;
        let mut var_qofd_dn7: f64 = *var_qofd_dn7_slot;
        let mut var_qofd_dn8: f64 = *var_qofd_dn8_slot;
        let mut var_qofd_dn9: f64 = *var_qofd_dn9_slot;
        let mut var_qofds: f64 = *var_qofds_slot;
        let mut var_qofds_db0: f64 = *var_qofds_db0_slot;
        let mut var_qofds_db1: f64 = *var_qofds_db1_slot;
        let mut var_qofds_db10: f64 = *var_qofds_db10_slot;
        let mut var_qofds_db11: f64 = *var_qofds_db11_slot;
        let mut var_qofds_db12: f64 = *var_qofds_db12_slot;
        let mut var_qofds_db13: f64 = *var_qofds_db13_slot;
        let mut var_qofds_db14: f64 = *var_qofds_db14_slot;
        let mut var_qofds_db15: f64 = *var_qofds_db15_slot;
        let mut var_qofds_db16: f64 = *var_qofds_db16_slot;
        let mut var_qofds_db17: f64 = *var_qofds_db17_slot;
        let mut var_qofds_db18: f64 = *var_qofds_db18_slot;
        let mut var_qofds_db19: f64 = *var_qofds_db19_slot;
        let mut var_qofds_db2: f64 = *var_qofds_db2_slot;
        let mut var_qofds_db20: f64 = *var_qofds_db20_slot;
        let mut var_qofds_db21: f64 = *var_qofds_db21_slot;
        let mut var_qofds_db22: f64 = *var_qofds_db22_slot;
        let mut var_qofds_db23: f64 = *var_qofds_db23_slot;
        let mut var_qofds_db24: f64 = *var_qofds_db24_slot;
        let mut var_qofds_db25: f64 = *var_qofds_db25_slot;
        let mut var_qofds_db26: f64 = *var_qofds_db26_slot;
        let mut var_qofds_db27: f64 = *var_qofds_db27_slot;
        let mut var_qofds_db28: f64 = *var_qofds_db28_slot;
        let mut var_qofds_db29: f64 = *var_qofds_db29_slot;
        let mut var_qofds_db3: f64 = *var_qofds_db3_slot;
        let mut var_qofds_db30: f64 = *var_qofds_db30_slot;
        let mut var_qofds_db31: f64 = *var_qofds_db31_slot;
        let mut var_qofds_db32: f64 = *var_qofds_db32_slot;
        let mut var_qofds_db33: f64 = *var_qofds_db33_slot;
        let mut var_qofds_db34: f64 = *var_qofds_db34_slot;
        let mut var_qofds_db35: f64 = *var_qofds_db35_slot;
        let mut var_qofds_db4: f64 = *var_qofds_db4_slot;
        let mut var_qofds_db5: f64 = *var_qofds_db5_slot;
        let mut var_qofds_db6: f64 = *var_qofds_db6_slot;
        let mut var_qofds_db7: f64 = *var_qofds_db7_slot;
        let mut var_qofds_db8: f64 = *var_qofds_db8_slot;
        let mut var_qofds_db9: f64 = *var_qofds_db9_slot;
        let mut var_qofds_dn0: f64 = *var_qofds_dn0_slot;
        let mut var_qofds_dn1: f64 = *var_qofds_dn1_slot;
        let mut var_qofds_dn10: f64 = *var_qofds_dn10_slot;
        let mut var_qofds_dn11: f64 = *var_qofds_dn11_slot;
        let mut var_qofds_dn12: f64 = *var_qofds_dn12_slot;
        let mut var_qofds_dn13: f64 = *var_qofds_dn13_slot;
        let mut var_qofds_dn14: f64 = *var_qofds_dn14_slot;
        let mut var_qofds_dn15: f64 = *var_qofds_dn15_slot;
        let mut var_qofds_dn16: f64 = *var_qofds_dn16_slot;
        let mut var_qofds_dn17: f64 = *var_qofds_dn17_slot;
        let mut var_qofds_dn18: f64 = *var_qofds_dn18_slot;
        let mut var_qofds_dn19: f64 = *var_qofds_dn19_slot;
        let mut var_qofds_dn2: f64 = *var_qofds_dn2_slot;
        let mut var_qofds_dn20: f64 = *var_qofds_dn20_slot;
        let mut var_qofds_dn21: f64 = *var_qofds_dn21_slot;
        let mut var_qofds_dn22: f64 = *var_qofds_dn22_slot;
        let mut var_qofds_dn23: f64 = *var_qofds_dn23_slot;
        let mut var_qofds_dn24: f64 = *var_qofds_dn24_slot;
        let mut var_qofds_dn25: f64 = *var_qofds_dn25_slot;
        let mut var_qofds_dn26: f64 = *var_qofds_dn26_slot;
        let mut var_qofds_dn27: f64 = *var_qofds_dn27_slot;
        let mut var_qofds_dn28: f64 = *var_qofds_dn28_slot;
        let mut var_qofds_dn29: f64 = *var_qofds_dn29_slot;
        let mut var_qofds_dn3: f64 = *var_qofds_dn3_slot;
        let mut var_qofds_dn4: f64 = *var_qofds_dn4_slot;
        let mut var_qofds_dn5: f64 = *var_qofds_dn5_slot;
        let mut var_qofds_dn6: f64 = *var_qofds_dn6_slot;
        let mut var_qofds_dn7: f64 = *var_qofds_dn7_slot;
        let mut var_qofds_dn8: f64 = *var_qofds_dn8_slot;
        let mut var_qofds_dn9: f64 = *var_qofds_dn9_slot;
        let mut var_qofs: f64 = *var_qofs_slot;
        let mut var_qofs_db0: f64 = *var_qofs_db0_slot;
        let mut var_qofs_db1: f64 = *var_qofs_db1_slot;
        let mut var_qofs_db10: f64 = *var_qofs_db10_slot;
        let mut var_qofs_db11: f64 = *var_qofs_db11_slot;
        let mut var_qofs_db12: f64 = *var_qofs_db12_slot;
        let mut var_qofs_db13: f64 = *var_qofs_db13_slot;
        let mut var_qofs_db14: f64 = *var_qofs_db14_slot;
        let mut var_qofs_db15: f64 = *var_qofs_db15_slot;
        let mut var_qofs_db16: f64 = *var_qofs_db16_slot;
        let mut var_qofs_db17: f64 = *var_qofs_db17_slot;
        let mut var_qofs_db18: f64 = *var_qofs_db18_slot;
        let mut var_qofs_db19: f64 = *var_qofs_db19_slot;
        let mut var_qofs_db2: f64 = *var_qofs_db2_slot;
        let mut var_qofs_db20: f64 = *var_qofs_db20_slot;
        let mut var_qofs_db21: f64 = *var_qofs_db21_slot;
        let mut var_qofs_db22: f64 = *var_qofs_db22_slot;
        let mut var_qofs_db23: f64 = *var_qofs_db23_slot;
        let mut var_qofs_db24: f64 = *var_qofs_db24_slot;
        let mut var_qofs_db25: f64 = *var_qofs_db25_slot;
        let mut var_qofs_db26: f64 = *var_qofs_db26_slot;
        let mut var_qofs_db27: f64 = *var_qofs_db27_slot;
        let mut var_qofs_db28: f64 = *var_qofs_db28_slot;
        let mut var_qofs_db29: f64 = *var_qofs_db29_slot;
        let mut var_qofs_db3: f64 = *var_qofs_db3_slot;
        let mut var_qofs_db30: f64 = *var_qofs_db30_slot;
        let mut var_qofs_db31: f64 = *var_qofs_db31_slot;
        let mut var_qofs_db32: f64 = *var_qofs_db32_slot;
        let mut var_qofs_db33: f64 = *var_qofs_db33_slot;
        let mut var_qofs_db34: f64 = *var_qofs_db34_slot;
        let mut var_qofs_db35: f64 = *var_qofs_db35_slot;
        let mut var_qofs_db4: f64 = *var_qofs_db4_slot;
        let mut var_qofs_db5: f64 = *var_qofs_db5_slot;
        let mut var_qofs_db6: f64 = *var_qofs_db6_slot;
        let mut var_qofs_db7: f64 = *var_qofs_db7_slot;
        let mut var_qofs_db8: f64 = *var_qofs_db8_slot;
        let mut var_qofs_db9: f64 = *var_qofs_db9_slot;
        let mut var_qofs_dn0: f64 = *var_qofs_dn0_slot;
        let mut var_qofs_dn1: f64 = *var_qofs_dn1_slot;
        let mut var_qofs_dn10: f64 = *var_qofs_dn10_slot;
        let mut var_qofs_dn11: f64 = *var_qofs_dn11_slot;
        let mut var_qofs_dn12: f64 = *var_qofs_dn12_slot;
        let mut var_qofs_dn13: f64 = *var_qofs_dn13_slot;
        let mut var_qofs_dn14: f64 = *var_qofs_dn14_slot;
        let mut var_qofs_dn15: f64 = *var_qofs_dn15_slot;
        let mut var_qofs_dn16: f64 = *var_qofs_dn16_slot;
        let mut var_qofs_dn17: f64 = *var_qofs_dn17_slot;
        let mut var_qofs_dn18: f64 = *var_qofs_dn18_slot;
        let mut var_qofs_dn19: f64 = *var_qofs_dn19_slot;
        let mut var_qofs_dn2: f64 = *var_qofs_dn2_slot;
        let mut var_qofs_dn20: f64 = *var_qofs_dn20_slot;
        let mut var_qofs_dn21: f64 = *var_qofs_dn21_slot;
        let mut var_qofs_dn22: f64 = *var_qofs_dn22_slot;
        let mut var_qofs_dn23: f64 = *var_qofs_dn23_slot;
        let mut var_qofs_dn24: f64 = *var_qofs_dn24_slot;
        let mut var_qofs_dn25: f64 = *var_qofs_dn25_slot;
        let mut var_qofs_dn26: f64 = *var_qofs_dn26_slot;
        let mut var_qofs_dn27: f64 = *var_qofs_dn27_slot;
        let mut var_qofs_dn28: f64 = *var_qofs_dn28_slot;
        let mut var_qofs_dn29: f64 = *var_qofs_dn29_slot;
        let mut var_qofs_dn3: f64 = *var_qofs_dn3_slot;
        let mut var_qofs_dn4: f64 = *var_qofs_dn4_slot;
        let mut var_qofs_dn5: f64 = *var_qofs_dn5_slot;
        let mut var_qofs_dn6: f64 = *var_qofs_dn6_slot;
        let mut var_qofs_dn7: f64 = *var_qofs_dn7_slot;
        let mut var_qofs_dn8: f64 = *var_qofs_dn8_slot;
        let mut var_qofs_dn9: f64 = *var_qofs_dn9_slot;

        let (assign46200_e44890, assign46200_e44890_d_n0, assign46200_e44890_d_n1, assign46200_e44890_d_n2, assign46200_e44890_d_n3, assign46200_e44890_d_n4, assign46200_e44890_d_n5, assign46200_e44890_d_n6, assign46200_e44890_d_n7, assign46200_e44890_d_n8, assign46200_e44890_d_n9, assign46200_e44890_d_n10, assign46200_e44890_d_n11, assign46200_e44890_d_n12, assign46200_e44890_d_n13, assign46200_e44890_d_n14, assign46200_e44890_d_n15, assign46200_e44890_d_n16, assign46200_e44890_d_n17, assign46200_e44890_d_n18, assign46200_e44890_d_n19, assign46200_e44890_d_n20, assign46200_e44890_d_n21, assign46200_e44890_d_n22, assign46200_e44890_d_n23, assign46200_e44890_d_n24, assign46200_e44890_d_n25, assign46200_e44890_d_n26, assign46200_e44890_d_n27, assign46200_e44890_d_n28, assign46200_e44890_d_n29, assign46200_e44890_d_b0, assign46200_e44890_d_b1, assign46200_e44890_d_b2, assign46200_e44890_d_b3, assign46200_e44890_d_b4, assign46200_e44890_d_b5, assign46200_e44890_d_b6, assign46200_e44890_d_b7, assign46200_e44890_d_b8, assign46200_e44890_d_b9, assign46200_e44890_d_b10, assign46200_e44890_d_b11, assign46200_e44890_d_b12, assign46200_e44890_d_b13, assign46200_e44890_d_b14, assign46200_e44890_d_b15, assign46200_e44890_d_b16, assign46200_e44890_d_b17, assign46200_e44890_d_b18, assign46200_e44890_d_b19, assign46200_e44890_d_b20, assign46200_e44890_d_b21, assign46200_e44890_d_b22, assign46200_e44890_d_b23, assign46200_e44890_d_b24, assign46200_e44890_d_b25, assign46200_e44890_d_b26, assign46200_e44890_d_b27, assign46200_e44890_d_b28, assign46200_e44890_d_b29, assign46200_e44890_d_b30, assign46200_e44890_d_b31, assign46200_e44890_d_b32, assign46200_e44890_d_b33, assign46200_e44890_d_b34, assign46200_e44890_d_b35,) = {
    if ((var_guard497 == 0.0) && (var_guard498 == 0.0)) {
        let assign46200_e44870: f64 = (p.p0 * p.p2);
        let assign46200_e44873: f64 = (var_cofsmt0 * (nv6 - nv2));
        let assign46200_e44876: f64 = (var_cofsmt * p.p28);
        let assign46200_e44880: f64 = ((nv6 - nv2) - p.p27);
        let assign46200_e44882: f64 = (assign46200_e44880 / p.p28);
        let assign46200_e44883: f64 = (assign46200_e44882).exp();
        let assign46200_e44884: f64 = (1.0 + assign46200_e44883);
        let assign46200_e44885: f64 = (assign46200_e44884).ln();
        let assign46200_e44886: f64 = (assign46200_e44876 * assign46200_e44885);
        let assign46200_e44887: f64 = (assign46200_e44873 + assign46200_e44886);
        let assign46200_e44888: f64 = (assign46200_e44870 * assign46200_e44887);
        (assign46200_e44888, (assign46200_e44870 * ((var_cofsmt0_dn0 * (nv6 - nv2)) + ((var_cofsmt_dn0 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn1 * (nv6 - nv2)) + ((var_cofsmt_dn1 * p.p28) * assign46200_e44885))), (assign46200_e44870 * (((var_cofsmt0_dn2 * (nv6 - nv2)) + (-var_cofsmt0)) + (((var_cofsmt_dn2 * p.p28) * assign46200_e44885) + (assign46200_e44876 * ((assign46200_e44883 * (-1.0 / p.p28)) / assign46200_e44884))))), (assign46200_e44870 * ((var_cofsmt0_dn3 * (nv6 - nv2)) + ((var_cofsmt_dn3 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn4 * (nv6 - nv2)) + ((var_cofsmt_dn4 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn5 * (nv6 - nv2)) + ((var_cofsmt_dn5 * p.p28) * assign46200_e44885))), (assign46200_e44870 * (((var_cofsmt0_dn6 * (nv6 - nv2)) + var_cofsmt0) + (((var_cofsmt_dn6 * p.p28) * assign46200_e44885) + (assign46200_e44876 * ((assign46200_e44883 * (1.0 / p.p28)) / assign46200_e44884))))), (assign46200_e44870 * ((var_cofsmt0_dn7 * (nv6 - nv2)) + ((var_cofsmt_dn7 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn8 * (nv6 - nv2)) + ((var_cofsmt_dn8 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn9 * (nv6 - nv2)) + ((var_cofsmt_dn9 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn10 * (nv6 - nv2)) + ((var_cofsmt_dn10 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn11 * (nv6 - nv2)) + ((var_cofsmt_dn11 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn12 * (nv6 - nv2)) + ((var_cofsmt_dn12 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn13 * (nv6 - nv2)) + ((var_cofsmt_dn13 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn14 * (nv6 - nv2)) + ((var_cofsmt_dn14 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn15 * (nv6 - nv2)) + ((var_cofsmt_dn15 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn16 * (nv6 - nv2)) + ((var_cofsmt_dn16 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn17 * (nv6 - nv2)) + ((var_cofsmt_dn17 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn18 * (nv6 - nv2)) + ((var_cofsmt_dn18 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn19 * (nv6 - nv2)) + ((var_cofsmt_dn19 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn20 * (nv6 - nv2)) + ((var_cofsmt_dn20 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn21 * (nv6 - nv2)) + ((var_cofsmt_dn21 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn22 * (nv6 - nv2)) + ((var_cofsmt_dn22 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn23 * (nv6 - nv2)) + ((var_cofsmt_dn23 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn24 * (nv6 - nv2)) + ((var_cofsmt_dn24 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn25 * (nv6 - nv2)) + ((var_cofsmt_dn25 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn26 * (nv6 - nv2)) + ((var_cofsmt_dn26 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn27 * (nv6 - nv2)) + ((var_cofsmt_dn27 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn28 * (nv6 - nv2)) + ((var_cofsmt_dn28 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_dn29 * (nv6 - nv2)) + ((var_cofsmt_dn29 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db0 * (nv6 - nv2)) + ((var_cofsmt_db0 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db1 * (nv6 - nv2)) + ((var_cofsmt_db1 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db2 * (nv6 - nv2)) + ((var_cofsmt_db2 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db3 * (nv6 - nv2)) + ((var_cofsmt_db3 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db4 * (nv6 - nv2)) + ((var_cofsmt_db4 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db5 * (nv6 - nv2)) + ((var_cofsmt_db5 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db6 * (nv6 - nv2)) + ((var_cofsmt_db6 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db7 * (nv6 - nv2)) + ((var_cofsmt_db7 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db8 * (nv6 - nv2)) + ((var_cofsmt_db8 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db9 * (nv6 - nv2)) + ((var_cofsmt_db9 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db10 * (nv6 - nv2)) + ((var_cofsmt_db10 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db11 * (nv6 - nv2)) + ((var_cofsmt_db11 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db12 * (nv6 - nv2)) + ((var_cofsmt_db12 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db13 * (nv6 - nv2)) + ((var_cofsmt_db13 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db14 * (nv6 - nv2)) + ((var_cofsmt_db14 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db15 * (nv6 - nv2)) + ((var_cofsmt_db15 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db16 * (nv6 - nv2)) + ((var_cofsmt_db16 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db17 * (nv6 - nv2)) + ((var_cofsmt_db17 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db18 * (nv6 - nv2)) + ((var_cofsmt_db18 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db19 * (nv6 - nv2)) + ((var_cofsmt_db19 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db20 * (nv6 - nv2)) + ((var_cofsmt_db20 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db21 * (nv6 - nv2)) + ((var_cofsmt_db21 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db22 * (nv6 - nv2)) + ((var_cofsmt_db22 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db23 * (nv6 - nv2)) + ((var_cofsmt_db23 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db24 * (nv6 - nv2)) + ((var_cofsmt_db24 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db25 * (nv6 - nv2)) + ((var_cofsmt_db25 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db26 * (nv6 - nv2)) + ((var_cofsmt_db26 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db27 * (nv6 - nv2)) + ((var_cofsmt_db27 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db28 * (nv6 - nv2)) + ((var_cofsmt_db28 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db29 * (nv6 - nv2)) + ((var_cofsmt_db29 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db30 * (nv6 - nv2)) + ((var_cofsmt_db30 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db31 * (nv6 - nv2)) + ((var_cofsmt_db31 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db32 * (nv6 - nv2)) + ((var_cofsmt_db32 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db33 * (nv6 - nv2)) + ((var_cofsmt_db33 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db34 * (nv6 - nv2)) + ((var_cofsmt_db34 * p.p28) * assign46200_e44885))), (assign46200_e44870 * ((var_cofsmt0_db35 * (nv6 - nv2)) + ((var_cofsmt_db35 * p.p28) * assign46200_e44885))),)
    } else {
        (var_qofs, var_qofs_dn0, var_qofs_dn1, var_qofs_dn2, var_qofs_dn3, var_qofs_dn4, var_qofs_dn5, var_qofs_dn6, var_qofs_dn7, var_qofs_dn8, var_qofs_dn9, var_qofs_dn10, var_qofs_dn11, var_qofs_dn12, var_qofs_dn13, var_qofs_dn14, var_qofs_dn15, var_qofs_dn16, var_qofs_dn17, var_qofs_dn18, var_qofs_dn19, var_qofs_dn20, var_qofs_dn21, var_qofs_dn22, var_qofs_dn23, var_qofs_dn24, var_qofs_dn25, var_qofs_dn26, var_qofs_dn27, var_qofs_dn28, var_qofs_dn29, var_qofs_db0, var_qofs_db1, var_qofs_db2, var_qofs_db3, var_qofs_db4, var_qofs_db5, var_qofs_db6, var_qofs_db7, var_qofs_db8, var_qofs_db9, var_qofs_db10, var_qofs_db11, var_qofs_db12, var_qofs_db13, var_qofs_db14, var_qofs_db15, var_qofs_db16, var_qofs_db17, var_qofs_db18, var_qofs_db19, var_qofs_db20, var_qofs_db21, var_qofs_db22, var_qofs_db23, var_qofs_db24, var_qofs_db25, var_qofs_db26, var_qofs_db27, var_qofs_db28, var_qofs_db29, var_qofs_db30, var_qofs_db31, var_qofs_db32, var_qofs_db33, var_qofs_db34, var_qofs_db35,)
    }
};
        var_qofs = assign46200_e44890;
        var_qofs_dn0 = assign46200_e44890_d_n0;
        var_qofs_dn1 = assign46200_e44890_d_n1;
        var_qofs_dn2 = assign46200_e44890_d_n2;
        var_qofs_dn3 = assign46200_e44890_d_n3;
        var_qofs_dn4 = assign46200_e44890_d_n4;
        var_qofs_dn5 = assign46200_e44890_d_n5;
        var_qofs_dn6 = assign46200_e44890_d_n6;
        var_qofs_dn7 = assign46200_e44890_d_n7;
        var_qofs_dn8 = assign46200_e44890_d_n8;
        var_qofs_dn9 = assign46200_e44890_d_n9;
        var_qofs_dn10 = assign46200_e44890_d_n10;
        var_qofs_dn11 = assign46200_e44890_d_n11;
        var_qofs_dn12 = assign46200_e44890_d_n12;
        var_qofs_dn13 = assign46200_e44890_d_n13;
        var_qofs_dn14 = assign46200_e44890_d_n14;
        var_qofs_dn15 = assign46200_e44890_d_n15;
        var_qofs_dn16 = assign46200_e44890_d_n16;
        var_qofs_dn17 = assign46200_e44890_d_n17;
        var_qofs_dn18 = assign46200_e44890_d_n18;
        var_qofs_dn19 = assign46200_e44890_d_n19;
        var_qofs_dn20 = assign46200_e44890_d_n20;
        var_qofs_dn21 = assign46200_e44890_d_n21;
        var_qofs_dn22 = assign46200_e44890_d_n22;
        var_qofs_dn23 = assign46200_e44890_d_n23;
        var_qofs_dn24 = assign46200_e44890_d_n24;
        var_qofs_dn25 = assign46200_e44890_d_n25;
        var_qofs_dn26 = assign46200_e44890_d_n26;
        var_qofs_dn27 = assign46200_e44890_d_n27;
        var_qofs_dn28 = assign46200_e44890_d_n28;
        var_qofs_dn29 = assign46200_e44890_d_n29;
        var_qofs_db0 = assign46200_e44890_d_b0;
        var_qofs_db1 = assign46200_e44890_d_b1;
        var_qofs_db2 = assign46200_e44890_d_b2;
        var_qofs_db3 = assign46200_e44890_d_b3;
        var_qofs_db4 = assign46200_e44890_d_b4;
        var_qofs_db5 = assign46200_e44890_d_b5;
        var_qofs_db6 = assign46200_e44890_d_b6;
        var_qofs_db7 = assign46200_e44890_d_b7;
        var_qofs_db8 = assign46200_e44890_d_b8;
        var_qofs_db9 = assign46200_e44890_d_b9;
        var_qofs_db10 = assign46200_e44890_d_b10;
        var_qofs_db11 = assign46200_e44890_d_b11;
        var_qofs_db12 = assign46200_e44890_d_b12;
        var_qofs_db13 = assign46200_e44890_d_b13;
        var_qofs_db14 = assign46200_e44890_d_b14;
        var_qofs_db15 = assign46200_e44890_d_b15;
        var_qofs_db16 = assign46200_e44890_d_b16;
        var_qofs_db17 = assign46200_e44890_d_b17;
        var_qofs_db18 = assign46200_e44890_d_b18;
        var_qofs_db19 = assign46200_e44890_d_b19;
        var_qofs_db20 = assign46200_e44890_d_b20;
        var_qofs_db21 = assign46200_e44890_d_b21;
        var_qofs_db22 = assign46200_e44890_d_b22;
        var_qofs_db23 = assign46200_e44890_d_b23;
        var_qofs_db24 = assign46200_e44890_d_b24;
        var_qofs_db25 = assign46200_e44890_d_b25;
        var_qofs_db26 = assign46200_e44890_d_b26;
        var_qofs_db27 = assign46200_e44890_d_b27;
        var_qofs_db28 = assign46200_e44890_d_b28;
        var_qofs_db29 = assign46200_e44890_d_b29;
        var_qofs_db30 = assign46200_e44890_d_b30;
        var_qofs_db31 = assign46200_e44890_d_b31;
        var_qofs_db32 = assign46200_e44890_d_b32;
        var_qofs_db33 = assign46200_e44890_d_b33;
        var_qofs_db34 = assign46200_e44890_d_b34;
        var_qofs_db35 = assign46200_e44890_d_b35;

        let assign46210_e44893: f64 = ((nv6 - nv0) - p.p27);
        let assign46210_e44895: f64 = (assign46210_e44893 / p.p28);
        let assign46210_e44897: f64 = if assign46210_e44895 > 50.0 { 1.0 } else { 0.0 };
        var_guard499 = assign46210_e44897;

        let (assign46220_e44913, assign46220_e44913_d_n0, assign46220_e44913_d_n1, assign46220_e44913_d_n2, assign46220_e44913_d_n3, assign46220_e44913_d_n4, assign46220_e44913_d_n5, assign46220_e44913_d_n6, assign46220_e44913_d_n7, assign46220_e44913_d_n8, assign46220_e44913_d_n9, assign46220_e44913_d_n10, assign46220_e44913_d_n11, assign46220_e44913_d_n12, assign46220_e44913_d_n13, assign46220_e44913_d_n14, assign46220_e44913_d_n15, assign46220_e44913_d_n16, assign46220_e44913_d_n17, assign46220_e44913_d_n18, assign46220_e44913_d_n19, assign46220_e44913_d_n20, assign46220_e44913_d_n21, assign46220_e44913_d_n22, assign46220_e44913_d_n23, assign46220_e44913_d_n24, assign46220_e44913_d_n25, assign46220_e44913_d_n26, assign46220_e44913_d_n27, assign46220_e44913_d_n28, assign46220_e44913_d_n29, assign46220_e44913_d_b0, assign46220_e44913_d_b1, assign46220_e44913_d_b2, assign46220_e44913_d_b3, assign46220_e44913_d_b4, assign46220_e44913_d_b5, assign46220_e44913_d_b6, assign46220_e44913_d_b7, assign46220_e44913_d_b8, assign46220_e44913_d_b9, assign46220_e44913_d_b10, assign46220_e44913_d_b11, assign46220_e44913_d_b12, assign46220_e44913_d_b13, assign46220_e44913_d_b14, assign46220_e44913_d_b15, assign46220_e44913_d_b16, assign46220_e44913_d_b17, assign46220_e44913_d_b18, assign46220_e44913_d_b19, assign46220_e44913_d_b20, assign46220_e44913_d_b21, assign46220_e44913_d_b22, assign46220_e44913_d_b23, assign46220_e44913_d_b24, assign46220_e44913_d_b25, assign46220_e44913_d_b26, assign46220_e44913_d_b27, assign46220_e44913_d_b28, assign46220_e44913_d_b29, assign46220_e44913_d_b30, assign46220_e44913_d_b31, assign46220_e44913_d_b32, assign46220_e44913_d_b33, assign46220_e44913_d_b34, assign46220_e44913_d_b35,) = {
    if (var_guard499 != 0.0) {
        let assign46220_e44901: f64 = (p.p0 * p.p2);
        let assign46220_e44904: f64 = (var_cofdmt0 * (nv6 - nv0));
        let assign46220_e44908: f64 = ((nv6 - nv0) - p.p27);
        let assign46220_e44909: f64 = (var_cofdmt * assign46220_e44908);
        let assign46220_e44910: f64 = (assign46220_e44904 + assign46220_e44909);
        let assign46220_e44911: f64 = (assign46220_e44901 * assign46220_e44910);
        (assign46220_e44911, (assign46220_e44901 * (((var_cofdmt0_dn0 * (nv6 - nv0)) + (-var_cofdmt0)) + ((var_cofdmt_dn0 * assign46220_e44908) + (-var_cofdmt)))), (assign46220_e44901 * ((var_cofdmt0_dn1 * (nv6 - nv0)) + (var_cofdmt_dn1 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn2 * (nv6 - nv0)) + (var_cofdmt_dn2 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn3 * (nv6 - nv0)) + (var_cofdmt_dn3 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn4 * (nv6 - nv0)) + (var_cofdmt_dn4 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn5 * (nv6 - nv0)) + (var_cofdmt_dn5 * assign46220_e44908))), (assign46220_e44901 * (((var_cofdmt0_dn6 * (nv6 - nv0)) + var_cofdmt0) + ((var_cofdmt_dn6 * assign46220_e44908) + var_cofdmt))), (assign46220_e44901 * ((var_cofdmt0_dn7 * (nv6 - nv0)) + (var_cofdmt_dn7 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn8 * (nv6 - nv0)) + (var_cofdmt_dn8 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn9 * (nv6 - nv0)) + (var_cofdmt_dn9 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn10 * (nv6 - nv0)) + (var_cofdmt_dn10 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn11 * (nv6 - nv0)) + (var_cofdmt_dn11 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn12 * (nv6 - nv0)) + (var_cofdmt_dn12 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn13 * (nv6 - nv0)) + (var_cofdmt_dn13 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn14 * (nv6 - nv0)) + (var_cofdmt_dn14 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn15 * (nv6 - nv0)) + (var_cofdmt_dn15 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn16 * (nv6 - nv0)) + (var_cofdmt_dn16 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn17 * (nv6 - nv0)) + (var_cofdmt_dn17 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn18 * (nv6 - nv0)) + (var_cofdmt_dn18 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn19 * (nv6 - nv0)) + (var_cofdmt_dn19 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn20 * (nv6 - nv0)) + (var_cofdmt_dn20 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn21 * (nv6 - nv0)) + (var_cofdmt_dn21 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn22 * (nv6 - nv0)) + (var_cofdmt_dn22 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn23 * (nv6 - nv0)) + (var_cofdmt_dn23 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn24 * (nv6 - nv0)) + (var_cofdmt_dn24 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn25 * (nv6 - nv0)) + (var_cofdmt_dn25 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn26 * (nv6 - nv0)) + (var_cofdmt_dn26 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn27 * (nv6 - nv0)) + (var_cofdmt_dn27 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn28 * (nv6 - nv0)) + (var_cofdmt_dn28 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_dn29 * (nv6 - nv0)) + (var_cofdmt_dn29 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db0 * (nv6 - nv0)) + (var_cofdmt_db0 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db1 * (nv6 - nv0)) + (var_cofdmt_db1 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db2 * (nv6 - nv0)) + (var_cofdmt_db2 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db3 * (nv6 - nv0)) + (var_cofdmt_db3 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db4 * (nv6 - nv0)) + (var_cofdmt_db4 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db5 * (nv6 - nv0)) + (var_cofdmt_db5 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db6 * (nv6 - nv0)) + (var_cofdmt_db6 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db7 * (nv6 - nv0)) + (var_cofdmt_db7 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db8 * (nv6 - nv0)) + (var_cofdmt_db8 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db9 * (nv6 - nv0)) + (var_cofdmt_db9 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db10 * (nv6 - nv0)) + (var_cofdmt_db10 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db11 * (nv6 - nv0)) + (var_cofdmt_db11 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db12 * (nv6 - nv0)) + (var_cofdmt_db12 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db13 * (nv6 - nv0)) + (var_cofdmt_db13 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db14 * (nv6 - nv0)) + (var_cofdmt_db14 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db15 * (nv6 - nv0)) + (var_cofdmt_db15 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db16 * (nv6 - nv0)) + (var_cofdmt_db16 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db17 * (nv6 - nv0)) + (var_cofdmt_db17 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db18 * (nv6 - nv0)) + (var_cofdmt_db18 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db19 * (nv6 - nv0)) + (var_cofdmt_db19 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db20 * (nv6 - nv0)) + (var_cofdmt_db20 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db21 * (nv6 - nv0)) + (var_cofdmt_db21 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db22 * (nv6 - nv0)) + (var_cofdmt_db22 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db23 * (nv6 - nv0)) + (var_cofdmt_db23 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db24 * (nv6 - nv0)) + (var_cofdmt_db24 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db25 * (nv6 - nv0)) + (var_cofdmt_db25 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db26 * (nv6 - nv0)) + (var_cofdmt_db26 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db27 * (nv6 - nv0)) + (var_cofdmt_db27 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db28 * (nv6 - nv0)) + (var_cofdmt_db28 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db29 * (nv6 - nv0)) + (var_cofdmt_db29 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db30 * (nv6 - nv0)) + (var_cofdmt_db30 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db31 * (nv6 - nv0)) + (var_cofdmt_db31 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db32 * (nv6 - nv0)) + (var_cofdmt_db32 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db33 * (nv6 - nv0)) + (var_cofdmt_db33 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db34 * (nv6 - nv0)) + (var_cofdmt_db34 * assign46220_e44908))), (assign46220_e44901 * ((var_cofdmt0_db35 * (nv6 - nv0)) + (var_cofdmt_db35 * assign46220_e44908))),)
    } else {
        (var_qofd, var_qofd_dn0, var_qofd_dn1, var_qofd_dn2, var_qofd_dn3, var_qofd_dn4, var_qofd_dn5, var_qofd_dn6, var_qofd_dn7, var_qofd_dn8, var_qofd_dn9, var_qofd_dn10, var_qofd_dn11, var_qofd_dn12, var_qofd_dn13, var_qofd_dn14, var_qofd_dn15, var_qofd_dn16, var_qofd_dn17, var_qofd_dn18, var_qofd_dn19, var_qofd_dn20, var_qofd_dn21, var_qofd_dn22, var_qofd_dn23, var_qofd_dn24, var_qofd_dn25, var_qofd_dn26, var_qofd_dn27, var_qofd_dn28, var_qofd_dn29, var_qofd_db0, var_qofd_db1, var_qofd_db2, var_qofd_db3, var_qofd_db4, var_qofd_db5, var_qofd_db6, var_qofd_db7, var_qofd_db8, var_qofd_db9, var_qofd_db10, var_qofd_db11, var_qofd_db12, var_qofd_db13, var_qofd_db14, var_qofd_db15, var_qofd_db16, var_qofd_db17, var_qofd_db18, var_qofd_db19, var_qofd_db20, var_qofd_db21, var_qofd_db22, var_qofd_db23, var_qofd_db24, var_qofd_db25, var_qofd_db26, var_qofd_db27, var_qofd_db28, var_qofd_db29, var_qofd_db30, var_qofd_db31, var_qofd_db32, var_qofd_db33, var_qofd_db34, var_qofd_db35,)
    }
};
        var_qofd = assign46220_e44913;
        var_qofd_dn0 = assign46220_e44913_d_n0;
        var_qofd_dn1 = assign46220_e44913_d_n1;
        var_qofd_dn2 = assign46220_e44913_d_n2;
        var_qofd_dn3 = assign46220_e44913_d_n3;
        var_qofd_dn4 = assign46220_e44913_d_n4;
        var_qofd_dn5 = assign46220_e44913_d_n5;
        var_qofd_dn6 = assign46220_e44913_d_n6;
        var_qofd_dn7 = assign46220_e44913_d_n7;
        var_qofd_dn8 = assign46220_e44913_d_n8;
        var_qofd_dn9 = assign46220_e44913_d_n9;
        var_qofd_dn10 = assign46220_e44913_d_n10;
        var_qofd_dn11 = assign46220_e44913_d_n11;
        var_qofd_dn12 = assign46220_e44913_d_n12;
        var_qofd_dn13 = assign46220_e44913_d_n13;
        var_qofd_dn14 = assign46220_e44913_d_n14;
        var_qofd_dn15 = assign46220_e44913_d_n15;
        var_qofd_dn16 = assign46220_e44913_d_n16;
        var_qofd_dn17 = assign46220_e44913_d_n17;
        var_qofd_dn18 = assign46220_e44913_d_n18;
        var_qofd_dn19 = assign46220_e44913_d_n19;
        var_qofd_dn20 = assign46220_e44913_d_n20;
        var_qofd_dn21 = assign46220_e44913_d_n21;
        var_qofd_dn22 = assign46220_e44913_d_n22;
        var_qofd_dn23 = assign46220_e44913_d_n23;
        var_qofd_dn24 = assign46220_e44913_d_n24;
        var_qofd_dn25 = assign46220_e44913_d_n25;
        var_qofd_dn26 = assign46220_e44913_d_n26;
        var_qofd_dn27 = assign46220_e44913_d_n27;
        var_qofd_dn28 = assign46220_e44913_d_n28;
        var_qofd_dn29 = assign46220_e44913_d_n29;
        var_qofd_db0 = assign46220_e44913_d_b0;
        var_qofd_db1 = assign46220_e44913_d_b1;
        var_qofd_db2 = assign46220_e44913_d_b2;
        var_qofd_db3 = assign46220_e44913_d_b3;
        var_qofd_db4 = assign46220_e44913_d_b4;
        var_qofd_db5 = assign46220_e44913_d_b5;
        var_qofd_db6 = assign46220_e44913_d_b6;
        var_qofd_db7 = assign46220_e44913_d_b7;
        var_qofd_db8 = assign46220_e44913_d_b8;
        var_qofd_db9 = assign46220_e44913_d_b9;
        var_qofd_db10 = assign46220_e44913_d_b10;
        var_qofd_db11 = assign46220_e44913_d_b11;
        var_qofd_db12 = assign46220_e44913_d_b12;
        var_qofd_db13 = assign46220_e44913_d_b13;
        var_qofd_db14 = assign46220_e44913_d_b14;
        var_qofd_db15 = assign46220_e44913_d_b15;
        var_qofd_db16 = assign46220_e44913_d_b16;
        var_qofd_db17 = assign46220_e44913_d_b17;
        var_qofd_db18 = assign46220_e44913_d_b18;
        var_qofd_db19 = assign46220_e44913_d_b19;
        var_qofd_db20 = assign46220_e44913_d_b20;
        var_qofd_db21 = assign46220_e44913_d_b21;
        var_qofd_db22 = assign46220_e44913_d_b22;
        var_qofd_db23 = assign46220_e44913_d_b23;
        var_qofd_db24 = assign46220_e44913_d_b24;
        var_qofd_db25 = assign46220_e44913_d_b25;
        var_qofd_db26 = assign46220_e44913_d_b26;
        var_qofd_db27 = assign46220_e44913_d_b27;
        var_qofd_db28 = assign46220_e44913_d_b28;
        var_qofd_db29 = assign46220_e44913_d_b29;
        var_qofd_db30 = assign46220_e44913_d_b30;
        var_qofd_db31 = assign46220_e44913_d_b31;
        var_qofd_db32 = assign46220_e44913_d_b32;
        var_qofd_db33 = assign46220_e44913_d_b33;
        var_qofd_db34 = assign46220_e44913_d_b34;
        var_qofd_db35 = assign46220_e44913_d_b35;

        let assign46230_e44916: f64 = ((nv6 - nv0) - p.p27);
        let assign46230_e44918: f64 = (assign46230_e44916 / p.p28);
        let assign46230_e44920: f64 = (-50.0);
        let assign46230_e44921: f64 = if assign46230_e44918 < assign46230_e44920 { 1.0 } else { 0.0 };
        var_guard500 = assign46230_e44921;

        let (assign46240_e44945, assign46240_e44945_d_n0, assign46240_e44945_d_n1, assign46240_e44945_d_n2, assign46240_e44945_d_n3, assign46240_e44945_d_n4, assign46240_e44945_d_n5, assign46240_e44945_d_n6, assign46240_e44945_d_n7, assign46240_e44945_d_n8, assign46240_e44945_d_n9, assign46240_e44945_d_n10, assign46240_e44945_d_n11, assign46240_e44945_d_n12, assign46240_e44945_d_n13, assign46240_e44945_d_n14, assign46240_e44945_d_n15, assign46240_e44945_d_n16, assign46240_e44945_d_n17, assign46240_e44945_d_n18, assign46240_e44945_d_n19, assign46240_e44945_d_n20, assign46240_e44945_d_n21, assign46240_e44945_d_n22, assign46240_e44945_d_n23, assign46240_e44945_d_n24, assign46240_e44945_d_n25, assign46240_e44945_d_n26, assign46240_e44945_d_n27, assign46240_e44945_d_n28, assign46240_e44945_d_n29, assign46240_e44945_d_b0, assign46240_e44945_d_b1, assign46240_e44945_d_b2, assign46240_e44945_d_b3, assign46240_e44945_d_b4, assign46240_e44945_d_b5, assign46240_e44945_d_b6, assign46240_e44945_d_b7, assign46240_e44945_d_b8, assign46240_e44945_d_b9, assign46240_e44945_d_b10, assign46240_e44945_d_b11, assign46240_e44945_d_b12, assign46240_e44945_d_b13, assign46240_e44945_d_b14, assign46240_e44945_d_b15, assign46240_e44945_d_b16, assign46240_e44945_d_b17, assign46240_e44945_d_b18, assign46240_e44945_d_b19, assign46240_e44945_d_b20, assign46240_e44945_d_b21, assign46240_e44945_d_b22, assign46240_e44945_d_b23, assign46240_e44945_d_b24, assign46240_e44945_d_b25, assign46240_e44945_d_b26, assign46240_e44945_d_b27, assign46240_e44945_d_b28, assign46240_e44945_d_b29, assign46240_e44945_d_b30, assign46240_e44945_d_b31, assign46240_e44945_d_b32, assign46240_e44945_d_b33, assign46240_e44945_d_b34, assign46240_e44945_d_b35,) = {
    if ((var_guard499 == 0.0) && (var_guard500 != 0.0)) {
        let assign46240_e44928: f64 = (p.p0 * p.p2);
        let assign46240_e44931: f64 = (var_cofdmt0 * (nv6 - nv0));
        let assign46240_e44934: f64 = (var_cofdmt * p.p28);
        let assign46240_e44937: f64 = ((nv6 - nv0) - p.p27);
        let assign46240_e44939: f64 = (assign46240_e44937 / p.p28);
        let assign46240_e44940: f64 = (assign46240_e44939).exp();
        let assign46240_e44941: f64 = (assign46240_e44934 * assign46240_e44940);
        let assign46240_e44942: f64 = (assign46240_e44931 + assign46240_e44941);
        let assign46240_e44943: f64 = (assign46240_e44928 * assign46240_e44942);
        (assign46240_e44943, (assign46240_e44928 * (((var_cofdmt0_dn0 * (nv6 - nv0)) + (-var_cofdmt0)) + (((var_cofdmt_dn0 * p.p28) * assign46240_e44940) + (assign46240_e44934 * (assign46240_e44940 * (-1.0 / p.p28)))))), (assign46240_e44928 * ((var_cofdmt0_dn1 * (nv6 - nv0)) + ((var_cofdmt_dn1 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn2 * (nv6 - nv0)) + ((var_cofdmt_dn2 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn3 * (nv6 - nv0)) + ((var_cofdmt_dn3 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn4 * (nv6 - nv0)) + ((var_cofdmt_dn4 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn5 * (nv6 - nv0)) + ((var_cofdmt_dn5 * p.p28) * assign46240_e44940))), (assign46240_e44928 * (((var_cofdmt0_dn6 * (nv6 - nv0)) + var_cofdmt0) + (((var_cofdmt_dn6 * p.p28) * assign46240_e44940) + (assign46240_e44934 * (assign46240_e44940 * (1.0 / p.p28)))))), (assign46240_e44928 * ((var_cofdmt0_dn7 * (nv6 - nv0)) + ((var_cofdmt_dn7 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn8 * (nv6 - nv0)) + ((var_cofdmt_dn8 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn9 * (nv6 - nv0)) + ((var_cofdmt_dn9 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn10 * (nv6 - nv0)) + ((var_cofdmt_dn10 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn11 * (nv6 - nv0)) + ((var_cofdmt_dn11 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn12 * (nv6 - nv0)) + ((var_cofdmt_dn12 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn13 * (nv6 - nv0)) + ((var_cofdmt_dn13 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn14 * (nv6 - nv0)) + ((var_cofdmt_dn14 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn15 * (nv6 - nv0)) + ((var_cofdmt_dn15 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn16 * (nv6 - nv0)) + ((var_cofdmt_dn16 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn17 * (nv6 - nv0)) + ((var_cofdmt_dn17 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn18 * (nv6 - nv0)) + ((var_cofdmt_dn18 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn19 * (nv6 - nv0)) + ((var_cofdmt_dn19 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn20 * (nv6 - nv0)) + ((var_cofdmt_dn20 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn21 * (nv6 - nv0)) + ((var_cofdmt_dn21 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn22 * (nv6 - nv0)) + ((var_cofdmt_dn22 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn23 * (nv6 - nv0)) + ((var_cofdmt_dn23 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn24 * (nv6 - nv0)) + ((var_cofdmt_dn24 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn25 * (nv6 - nv0)) + ((var_cofdmt_dn25 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn26 * (nv6 - nv0)) + ((var_cofdmt_dn26 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn27 * (nv6 - nv0)) + ((var_cofdmt_dn27 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn28 * (nv6 - nv0)) + ((var_cofdmt_dn28 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_dn29 * (nv6 - nv0)) + ((var_cofdmt_dn29 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db0 * (nv6 - nv0)) + ((var_cofdmt_db0 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db1 * (nv6 - nv0)) + ((var_cofdmt_db1 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db2 * (nv6 - nv0)) + ((var_cofdmt_db2 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db3 * (nv6 - nv0)) + ((var_cofdmt_db3 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db4 * (nv6 - nv0)) + ((var_cofdmt_db4 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db5 * (nv6 - nv0)) + ((var_cofdmt_db5 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db6 * (nv6 - nv0)) + ((var_cofdmt_db6 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db7 * (nv6 - nv0)) + ((var_cofdmt_db7 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db8 * (nv6 - nv0)) + ((var_cofdmt_db8 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db9 * (nv6 - nv0)) + ((var_cofdmt_db9 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db10 * (nv6 - nv0)) + ((var_cofdmt_db10 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db11 * (nv6 - nv0)) + ((var_cofdmt_db11 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db12 * (nv6 - nv0)) + ((var_cofdmt_db12 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db13 * (nv6 - nv0)) + ((var_cofdmt_db13 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db14 * (nv6 - nv0)) + ((var_cofdmt_db14 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db15 * (nv6 - nv0)) + ((var_cofdmt_db15 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db16 * (nv6 - nv0)) + ((var_cofdmt_db16 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db17 * (nv6 - nv0)) + ((var_cofdmt_db17 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db18 * (nv6 - nv0)) + ((var_cofdmt_db18 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db19 * (nv6 - nv0)) + ((var_cofdmt_db19 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db20 * (nv6 - nv0)) + ((var_cofdmt_db20 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db21 * (nv6 - nv0)) + ((var_cofdmt_db21 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db22 * (nv6 - nv0)) + ((var_cofdmt_db22 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db23 * (nv6 - nv0)) + ((var_cofdmt_db23 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db24 * (nv6 - nv0)) + ((var_cofdmt_db24 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db25 * (nv6 - nv0)) + ((var_cofdmt_db25 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db26 * (nv6 - nv0)) + ((var_cofdmt_db26 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db27 * (nv6 - nv0)) + ((var_cofdmt_db27 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db28 * (nv6 - nv0)) + ((var_cofdmt_db28 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db29 * (nv6 - nv0)) + ((var_cofdmt_db29 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db30 * (nv6 - nv0)) + ((var_cofdmt_db30 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db31 * (nv6 - nv0)) + ((var_cofdmt_db31 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db32 * (nv6 - nv0)) + ((var_cofdmt_db32 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db33 * (nv6 - nv0)) + ((var_cofdmt_db33 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db34 * (nv6 - nv0)) + ((var_cofdmt_db34 * p.p28) * assign46240_e44940))), (assign46240_e44928 * ((var_cofdmt0_db35 * (nv6 - nv0)) + ((var_cofdmt_db35 * p.p28) * assign46240_e44940))),)
    } else {
        (var_qofd, var_qofd_dn0, var_qofd_dn1, var_qofd_dn2, var_qofd_dn3, var_qofd_dn4, var_qofd_dn5, var_qofd_dn6, var_qofd_dn7, var_qofd_dn8, var_qofd_dn9, var_qofd_dn10, var_qofd_dn11, var_qofd_dn12, var_qofd_dn13, var_qofd_dn14, var_qofd_dn15, var_qofd_dn16, var_qofd_dn17, var_qofd_dn18, var_qofd_dn19, var_qofd_dn20, var_qofd_dn21, var_qofd_dn22, var_qofd_dn23, var_qofd_dn24, var_qofd_dn25, var_qofd_dn26, var_qofd_dn27, var_qofd_dn28, var_qofd_dn29, var_qofd_db0, var_qofd_db1, var_qofd_db2, var_qofd_db3, var_qofd_db4, var_qofd_db5, var_qofd_db6, var_qofd_db7, var_qofd_db8, var_qofd_db9, var_qofd_db10, var_qofd_db11, var_qofd_db12, var_qofd_db13, var_qofd_db14, var_qofd_db15, var_qofd_db16, var_qofd_db17, var_qofd_db18, var_qofd_db19, var_qofd_db20, var_qofd_db21, var_qofd_db22, var_qofd_db23, var_qofd_db24, var_qofd_db25, var_qofd_db26, var_qofd_db27, var_qofd_db28, var_qofd_db29, var_qofd_db30, var_qofd_db31, var_qofd_db32, var_qofd_db33, var_qofd_db34, var_qofd_db35,)
    }
};
        var_qofd = assign46240_e44945;
        var_qofd_dn0 = assign46240_e44945_d_n0;
        var_qofd_dn1 = assign46240_e44945_d_n1;
        var_qofd_dn2 = assign46240_e44945_d_n2;
        var_qofd_dn3 = assign46240_e44945_d_n3;
        var_qofd_dn4 = assign46240_e44945_d_n4;
        var_qofd_dn5 = assign46240_e44945_d_n5;
        var_qofd_dn6 = assign46240_e44945_d_n6;
        var_qofd_dn7 = assign46240_e44945_d_n7;
        var_qofd_dn8 = assign46240_e44945_d_n8;
        var_qofd_dn9 = assign46240_e44945_d_n9;
        var_qofd_dn10 = assign46240_e44945_d_n10;
        var_qofd_dn11 = assign46240_e44945_d_n11;
        var_qofd_dn12 = assign46240_e44945_d_n12;
        var_qofd_dn13 = assign46240_e44945_d_n13;
        var_qofd_dn14 = assign46240_e44945_d_n14;
        var_qofd_dn15 = assign46240_e44945_d_n15;
        var_qofd_dn16 = assign46240_e44945_d_n16;
        var_qofd_dn17 = assign46240_e44945_d_n17;
        var_qofd_dn18 = assign46240_e44945_d_n18;
        var_qofd_dn19 = assign46240_e44945_d_n19;
        var_qofd_dn20 = assign46240_e44945_d_n20;
        var_qofd_dn21 = assign46240_e44945_d_n21;
        var_qofd_dn22 = assign46240_e44945_d_n22;
        var_qofd_dn23 = assign46240_e44945_d_n23;
        var_qofd_dn24 = assign46240_e44945_d_n24;
        var_qofd_dn25 = assign46240_e44945_d_n25;
        var_qofd_dn26 = assign46240_e44945_d_n26;
        var_qofd_dn27 = assign46240_e44945_d_n27;
        var_qofd_dn28 = assign46240_e44945_d_n28;
        var_qofd_dn29 = assign46240_e44945_d_n29;
        var_qofd_db0 = assign46240_e44945_d_b0;
        var_qofd_db1 = assign46240_e44945_d_b1;
        var_qofd_db2 = assign46240_e44945_d_b2;
        var_qofd_db3 = assign46240_e44945_d_b3;
        var_qofd_db4 = assign46240_e44945_d_b4;
        var_qofd_db5 = assign46240_e44945_d_b5;
        var_qofd_db6 = assign46240_e44945_d_b6;
        var_qofd_db7 = assign46240_e44945_d_b7;
        var_qofd_db8 = assign46240_e44945_d_b8;
        var_qofd_db9 = assign46240_e44945_d_b9;
        var_qofd_db10 = assign46240_e44945_d_b10;
        var_qofd_db11 = assign46240_e44945_d_b11;
        var_qofd_db12 = assign46240_e44945_d_b12;
        var_qofd_db13 = assign46240_e44945_d_b13;
        var_qofd_db14 = assign46240_e44945_d_b14;
        var_qofd_db15 = assign46240_e44945_d_b15;
        var_qofd_db16 = assign46240_e44945_d_b16;
        var_qofd_db17 = assign46240_e44945_d_b17;
        var_qofd_db18 = assign46240_e44945_d_b18;
        var_qofd_db19 = assign46240_e44945_d_b19;
        var_qofd_db20 = assign46240_e44945_d_b20;
        var_qofd_db21 = assign46240_e44945_d_b21;
        var_qofd_db22 = assign46240_e44945_d_b22;
        var_qofd_db23 = assign46240_e44945_d_b23;
        var_qofd_db24 = assign46240_e44945_d_b24;
        var_qofd_db25 = assign46240_e44945_d_b25;
        var_qofd_db26 = assign46240_e44945_d_b26;
        var_qofd_db27 = assign46240_e44945_d_b27;
        var_qofd_db28 = assign46240_e44945_d_b28;
        var_qofd_db29 = assign46240_e44945_d_b29;
        var_qofd_db30 = assign46240_e44945_d_b30;
        var_qofd_db31 = assign46240_e44945_d_b31;
        var_qofd_db32 = assign46240_e44945_d_b32;
        var_qofd_db33 = assign46240_e44945_d_b33;
        var_qofd_db34 = assign46240_e44945_d_b34;
        var_qofd_db35 = assign46240_e44945_d_b35;

        let (assign46250_e44973, assign46250_e44973_d_n0, assign46250_e44973_d_n1, assign46250_e44973_d_n2, assign46250_e44973_d_n3, assign46250_e44973_d_n4, assign46250_e44973_d_n5, assign46250_e44973_d_n6, assign46250_e44973_d_n7, assign46250_e44973_d_n8, assign46250_e44973_d_n9, assign46250_e44973_d_n10, assign46250_e44973_d_n11, assign46250_e44973_d_n12, assign46250_e44973_d_n13, assign46250_e44973_d_n14, assign46250_e44973_d_n15, assign46250_e44973_d_n16, assign46250_e44973_d_n17, assign46250_e44973_d_n18, assign46250_e44973_d_n19, assign46250_e44973_d_n20, assign46250_e44973_d_n21, assign46250_e44973_d_n22, assign46250_e44973_d_n23, assign46250_e44973_d_n24, assign46250_e44973_d_n25, assign46250_e44973_d_n26, assign46250_e44973_d_n27, assign46250_e44973_d_n28, assign46250_e44973_d_n29, assign46250_e44973_d_b0, assign46250_e44973_d_b1, assign46250_e44973_d_b2, assign46250_e44973_d_b3, assign46250_e44973_d_b4, assign46250_e44973_d_b5, assign46250_e44973_d_b6, assign46250_e44973_d_b7, assign46250_e44973_d_b8, assign46250_e44973_d_b9, assign46250_e44973_d_b10, assign46250_e44973_d_b11, assign46250_e44973_d_b12, assign46250_e44973_d_b13, assign46250_e44973_d_b14, assign46250_e44973_d_b15, assign46250_e44973_d_b16, assign46250_e44973_d_b17, assign46250_e44973_d_b18, assign46250_e44973_d_b19, assign46250_e44973_d_b20, assign46250_e44973_d_b21, assign46250_e44973_d_b22, assign46250_e44973_d_b23, assign46250_e44973_d_b24, assign46250_e44973_d_b25, assign46250_e44973_d_b26, assign46250_e44973_d_b27, assign46250_e44973_d_b28, assign46250_e44973_d_b29, assign46250_e44973_d_b30, assign46250_e44973_d_b31, assign46250_e44973_d_b32, assign46250_e44973_d_b33, assign46250_e44973_d_b34, assign46250_e44973_d_b35,) = {
    if ((var_guard499 == 0.0) && (var_guard500 == 0.0)) {
        let assign46250_e44953: f64 = (p.p0 * p.p2);
        let assign46250_e44956: f64 = (var_cofdmt0 * (nv6 - nv0));
        let assign46250_e44959: f64 = (var_cofdmt * p.p28);
        let assign46250_e44963: f64 = ((nv6 - nv0) - p.p27);
        let assign46250_e44965: f64 = (assign46250_e44963 / p.p28);
        let assign46250_e44966: f64 = (assign46250_e44965).exp();
        let assign46250_e44967: f64 = (1.0 + assign46250_e44966);
        let assign46250_e44968: f64 = (assign46250_e44967).ln();
        let assign46250_e44969: f64 = (assign46250_e44959 * assign46250_e44968);
        let assign46250_e44970: f64 = (assign46250_e44956 + assign46250_e44969);
        let assign46250_e44971: f64 = (assign46250_e44953 * assign46250_e44970);
        (assign46250_e44971, (assign46250_e44953 * (((var_cofdmt0_dn0 * (nv6 - nv0)) + (-var_cofdmt0)) + (((var_cofdmt_dn0 * p.p28) * assign46250_e44968) + (assign46250_e44959 * ((assign46250_e44966 * (-1.0 / p.p28)) / assign46250_e44967))))), (assign46250_e44953 * ((var_cofdmt0_dn1 * (nv6 - nv0)) + ((var_cofdmt_dn1 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn2 * (nv6 - nv0)) + ((var_cofdmt_dn2 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn3 * (nv6 - nv0)) + ((var_cofdmt_dn3 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn4 * (nv6 - nv0)) + ((var_cofdmt_dn4 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn5 * (nv6 - nv0)) + ((var_cofdmt_dn5 * p.p28) * assign46250_e44968))), (assign46250_e44953 * (((var_cofdmt0_dn6 * (nv6 - nv0)) + var_cofdmt0) + (((var_cofdmt_dn6 * p.p28) * assign46250_e44968) + (assign46250_e44959 * ((assign46250_e44966 * (1.0 / p.p28)) / assign46250_e44967))))), (assign46250_e44953 * ((var_cofdmt0_dn7 * (nv6 - nv0)) + ((var_cofdmt_dn7 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn8 * (nv6 - nv0)) + ((var_cofdmt_dn8 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn9 * (nv6 - nv0)) + ((var_cofdmt_dn9 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn10 * (nv6 - nv0)) + ((var_cofdmt_dn10 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn11 * (nv6 - nv0)) + ((var_cofdmt_dn11 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn12 * (nv6 - nv0)) + ((var_cofdmt_dn12 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn13 * (nv6 - nv0)) + ((var_cofdmt_dn13 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn14 * (nv6 - nv0)) + ((var_cofdmt_dn14 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn15 * (nv6 - nv0)) + ((var_cofdmt_dn15 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn16 * (nv6 - nv0)) + ((var_cofdmt_dn16 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn17 * (nv6 - nv0)) + ((var_cofdmt_dn17 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn18 * (nv6 - nv0)) + ((var_cofdmt_dn18 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn19 * (nv6 - nv0)) + ((var_cofdmt_dn19 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn20 * (nv6 - nv0)) + ((var_cofdmt_dn20 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn21 * (nv6 - nv0)) + ((var_cofdmt_dn21 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn22 * (nv6 - nv0)) + ((var_cofdmt_dn22 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn23 * (nv6 - nv0)) + ((var_cofdmt_dn23 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn24 * (nv6 - nv0)) + ((var_cofdmt_dn24 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn25 * (nv6 - nv0)) + ((var_cofdmt_dn25 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn26 * (nv6 - nv0)) + ((var_cofdmt_dn26 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn27 * (nv6 - nv0)) + ((var_cofdmt_dn27 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn28 * (nv6 - nv0)) + ((var_cofdmt_dn28 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_dn29 * (nv6 - nv0)) + ((var_cofdmt_dn29 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db0 * (nv6 - nv0)) + ((var_cofdmt_db0 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db1 * (nv6 - nv0)) + ((var_cofdmt_db1 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db2 * (nv6 - nv0)) + ((var_cofdmt_db2 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db3 * (nv6 - nv0)) + ((var_cofdmt_db3 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db4 * (nv6 - nv0)) + ((var_cofdmt_db4 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db5 * (nv6 - nv0)) + ((var_cofdmt_db5 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db6 * (nv6 - nv0)) + ((var_cofdmt_db6 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db7 * (nv6 - nv0)) + ((var_cofdmt_db7 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db8 * (nv6 - nv0)) + ((var_cofdmt_db8 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db9 * (nv6 - nv0)) + ((var_cofdmt_db9 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db10 * (nv6 - nv0)) + ((var_cofdmt_db10 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db11 * (nv6 - nv0)) + ((var_cofdmt_db11 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db12 * (nv6 - nv0)) + ((var_cofdmt_db12 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db13 * (nv6 - nv0)) + ((var_cofdmt_db13 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db14 * (nv6 - nv0)) + ((var_cofdmt_db14 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db15 * (nv6 - nv0)) + ((var_cofdmt_db15 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db16 * (nv6 - nv0)) + ((var_cofdmt_db16 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db17 * (nv6 - nv0)) + ((var_cofdmt_db17 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db18 * (nv6 - nv0)) + ((var_cofdmt_db18 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db19 * (nv6 - nv0)) + ((var_cofdmt_db19 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db20 * (nv6 - nv0)) + ((var_cofdmt_db20 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db21 * (nv6 - nv0)) + ((var_cofdmt_db21 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db22 * (nv6 - nv0)) + ((var_cofdmt_db22 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db23 * (nv6 - nv0)) + ((var_cofdmt_db23 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db24 * (nv6 - nv0)) + ((var_cofdmt_db24 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db25 * (nv6 - nv0)) + ((var_cofdmt_db25 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db26 * (nv6 - nv0)) + ((var_cofdmt_db26 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db27 * (nv6 - nv0)) + ((var_cofdmt_db27 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db28 * (nv6 - nv0)) + ((var_cofdmt_db28 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db29 * (nv6 - nv0)) + ((var_cofdmt_db29 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db30 * (nv6 - nv0)) + ((var_cofdmt_db30 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db31 * (nv6 - nv0)) + ((var_cofdmt_db31 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db32 * (nv6 - nv0)) + ((var_cofdmt_db32 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db33 * (nv6 - nv0)) + ((var_cofdmt_db33 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db34 * (nv6 - nv0)) + ((var_cofdmt_db34 * p.p28) * assign46250_e44968))), (assign46250_e44953 * ((var_cofdmt0_db35 * (nv6 - nv0)) + ((var_cofdmt_db35 * p.p28) * assign46250_e44968))),)
    } else {
        (var_qofd, var_qofd_dn0, var_qofd_dn1, var_qofd_dn2, var_qofd_dn3, var_qofd_dn4, var_qofd_dn5, var_qofd_dn6, var_qofd_dn7, var_qofd_dn8, var_qofd_dn9, var_qofd_dn10, var_qofd_dn11, var_qofd_dn12, var_qofd_dn13, var_qofd_dn14, var_qofd_dn15, var_qofd_dn16, var_qofd_dn17, var_qofd_dn18, var_qofd_dn19, var_qofd_dn20, var_qofd_dn21, var_qofd_dn22, var_qofd_dn23, var_qofd_dn24, var_qofd_dn25, var_qofd_dn26, var_qofd_dn27, var_qofd_dn28, var_qofd_dn29, var_qofd_db0, var_qofd_db1, var_qofd_db2, var_qofd_db3, var_qofd_db4, var_qofd_db5, var_qofd_db6, var_qofd_db7, var_qofd_db8, var_qofd_db9, var_qofd_db10, var_qofd_db11, var_qofd_db12, var_qofd_db13, var_qofd_db14, var_qofd_db15, var_qofd_db16, var_qofd_db17, var_qofd_db18, var_qofd_db19, var_qofd_db20, var_qofd_db21, var_qofd_db22, var_qofd_db23, var_qofd_db24, var_qofd_db25, var_qofd_db26, var_qofd_db27, var_qofd_db28, var_qofd_db29, var_qofd_db30, var_qofd_db31, var_qofd_db32, var_qofd_db33, var_qofd_db34, var_qofd_db35,)
    }
};
        var_qofd = assign46250_e44973;
        var_qofd_dn0 = assign46250_e44973_d_n0;
        var_qofd_dn1 = assign46250_e44973_d_n1;
        var_qofd_dn2 = assign46250_e44973_d_n2;
        var_qofd_dn3 = assign46250_e44973_d_n3;
        var_qofd_dn4 = assign46250_e44973_d_n4;
        var_qofd_dn5 = assign46250_e44973_d_n5;
        var_qofd_dn6 = assign46250_e44973_d_n6;
        var_qofd_dn7 = assign46250_e44973_d_n7;
        var_qofd_dn8 = assign46250_e44973_d_n8;
        var_qofd_dn9 = assign46250_e44973_d_n9;
        var_qofd_dn10 = assign46250_e44973_d_n10;
        var_qofd_dn11 = assign46250_e44973_d_n11;
        var_qofd_dn12 = assign46250_e44973_d_n12;
        var_qofd_dn13 = assign46250_e44973_d_n13;
        var_qofd_dn14 = assign46250_e44973_d_n14;
        var_qofd_dn15 = assign46250_e44973_d_n15;
        var_qofd_dn16 = assign46250_e44973_d_n16;
        var_qofd_dn17 = assign46250_e44973_d_n17;
        var_qofd_dn18 = assign46250_e44973_d_n18;
        var_qofd_dn19 = assign46250_e44973_d_n19;
        var_qofd_dn20 = assign46250_e44973_d_n20;
        var_qofd_dn21 = assign46250_e44973_d_n21;
        var_qofd_dn22 = assign46250_e44973_d_n22;
        var_qofd_dn23 = assign46250_e44973_d_n23;
        var_qofd_dn24 = assign46250_e44973_d_n24;
        var_qofd_dn25 = assign46250_e44973_d_n25;
        var_qofd_dn26 = assign46250_e44973_d_n26;
        var_qofd_dn27 = assign46250_e44973_d_n27;
        var_qofd_dn28 = assign46250_e44973_d_n28;
        var_qofd_dn29 = assign46250_e44973_d_n29;
        var_qofd_db0 = assign46250_e44973_d_b0;
        var_qofd_db1 = assign46250_e44973_d_b1;
        var_qofd_db2 = assign46250_e44973_d_b2;
        var_qofd_db3 = assign46250_e44973_d_b3;
        var_qofd_db4 = assign46250_e44973_d_b4;
        var_qofd_db5 = assign46250_e44973_d_b5;
        var_qofd_db6 = assign46250_e44973_d_b6;
        var_qofd_db7 = assign46250_e44973_d_b7;
        var_qofd_db8 = assign46250_e44973_d_b8;
        var_qofd_db9 = assign46250_e44973_d_b9;
        var_qofd_db10 = assign46250_e44973_d_b10;
        var_qofd_db11 = assign46250_e44973_d_b11;
        var_qofd_db12 = assign46250_e44973_d_b12;
        var_qofd_db13 = assign46250_e44973_d_b13;
        var_qofd_db14 = assign46250_e44973_d_b14;
        var_qofd_db15 = assign46250_e44973_d_b15;
        var_qofd_db16 = assign46250_e44973_d_b16;
        var_qofd_db17 = assign46250_e44973_d_b17;
        var_qofd_db18 = assign46250_e44973_d_b18;
        var_qofd_db19 = assign46250_e44973_d_b19;
        var_qofd_db20 = assign46250_e44973_d_b20;
        var_qofd_db21 = assign46250_e44973_d_b21;
        var_qofd_db22 = assign46250_e44973_d_b22;
        var_qofd_db23 = assign46250_e44973_d_b23;
        var_qofd_db24 = assign46250_e44973_d_b24;
        var_qofd_db25 = assign46250_e44973_d_b25;
        var_qofd_db26 = assign46250_e44973_d_b26;
        var_qofd_db27 = assign46250_e44973_d_b27;
        var_qofd_db28 = assign46250_e44973_d_b28;
        var_qofd_db29 = assign46250_e44973_d_b29;
        var_qofd_db30 = assign46250_e44973_d_b30;
        var_qofd_db31 = assign46250_e44973_d_b31;
        var_qofd_db32 = assign46250_e44973_d_b32;
        var_qofd_db33 = assign46250_e44973_d_b33;
        var_qofd_db34 = assign46250_e44973_d_b34;
        var_qofd_db35 = assign46250_e44973_d_b35;

        let assign46260_e44976: f64 = ((nv2 - nv0) - p.p27);
        let assign46260_e44978: f64 = (assign46260_e44976 / p.p28);
        let assign46260_e44980: f64 = if assign46260_e44978 > 50.0 { 1.0 } else { 0.0 };
        var_guard501 = assign46260_e44980;

        let (assign46270_e44996, assign46270_e44996_d_n0, assign46270_e44996_d_n1, assign46270_e44996_d_n2, assign46270_e44996_d_n3, assign46270_e44996_d_n4, assign46270_e44996_d_n5, assign46270_e44996_d_n6, assign46270_e44996_d_n7, assign46270_e44996_d_n8, assign46270_e44996_d_n9, assign46270_e44996_d_n10, assign46270_e44996_d_n11, assign46270_e44996_d_n12, assign46270_e44996_d_n13, assign46270_e44996_d_n14, assign46270_e44996_d_n15, assign46270_e44996_d_n16, assign46270_e44996_d_n17, assign46270_e44996_d_n18, assign46270_e44996_d_n19, assign46270_e44996_d_n20, assign46270_e44996_d_n21, assign46270_e44996_d_n22, assign46270_e44996_d_n23, assign46270_e44996_d_n24, assign46270_e44996_d_n25, assign46270_e44996_d_n26, assign46270_e44996_d_n27, assign46270_e44996_d_n28, assign46270_e44996_d_n29, assign46270_e44996_d_b0, assign46270_e44996_d_b1, assign46270_e44996_d_b2, assign46270_e44996_d_b3, assign46270_e44996_d_b4, assign46270_e44996_d_b5, assign46270_e44996_d_b6, assign46270_e44996_d_b7, assign46270_e44996_d_b8, assign46270_e44996_d_b9, assign46270_e44996_d_b10, assign46270_e44996_d_b11, assign46270_e44996_d_b12, assign46270_e44996_d_b13, assign46270_e44996_d_b14, assign46270_e44996_d_b15, assign46270_e44996_d_b16, assign46270_e44996_d_b17, assign46270_e44996_d_b18, assign46270_e44996_d_b19, assign46270_e44996_d_b20, assign46270_e44996_d_b21, assign46270_e44996_d_b22, assign46270_e44996_d_b23, assign46270_e44996_d_b24, assign46270_e44996_d_b25, assign46270_e44996_d_b26, assign46270_e44996_d_b27, assign46270_e44996_d_b28, assign46270_e44996_d_b29, assign46270_e44996_d_b30, assign46270_e44996_d_b31, assign46270_e44996_d_b32, assign46270_e44996_d_b33, assign46270_e44996_d_b34, assign46270_e44996_d_b35,) = {
    if (var_guard501 != 0.0) {
        let assign46270_e44984: f64 = (p.p0 * p.p2);
        let assign46270_e44987: f64 = (var_cofdsmt0 * (nv2 - nv0));
        let assign46270_e44991: f64 = ((nv2 - nv0) - p.p27);
        let assign46270_e44992: f64 = (var_cofdsmt * assign46270_e44991);
        let assign46270_e44993: f64 = (assign46270_e44987 + assign46270_e44992);
        let assign46270_e44994: f64 = (assign46270_e44984 * assign46270_e44993);
        (assign46270_e44994, (assign46270_e44984 * (((var_cofdsmt0_dn0 * (nv2 - nv0)) + (-var_cofdsmt0)) + ((var_cofdsmt_dn0 * assign46270_e44991) + (-var_cofdsmt)))), (assign46270_e44984 * ((var_cofdsmt0_dn1 * (nv2 - nv0)) + (var_cofdsmt_dn1 * assign46270_e44991))), (assign46270_e44984 * (((var_cofdsmt0_dn2 * (nv2 - nv0)) + var_cofdsmt0) + ((var_cofdsmt_dn2 * assign46270_e44991) + var_cofdsmt))), (assign46270_e44984 * ((var_cofdsmt0_dn3 * (nv2 - nv0)) + (var_cofdsmt_dn3 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn4 * (nv2 - nv0)) + (var_cofdsmt_dn4 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn5 * (nv2 - nv0)) + (var_cofdsmt_dn5 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn6 * (nv2 - nv0)) + (var_cofdsmt_dn6 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn7 * (nv2 - nv0)) + (var_cofdsmt_dn7 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn8 * (nv2 - nv0)) + (var_cofdsmt_dn8 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn9 * (nv2 - nv0)) + (var_cofdsmt_dn9 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn10 * (nv2 - nv0)) + (var_cofdsmt_dn10 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn11 * (nv2 - nv0)) + (var_cofdsmt_dn11 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn12 * (nv2 - nv0)) + (var_cofdsmt_dn12 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn13 * (nv2 - nv0)) + (var_cofdsmt_dn13 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn14 * (nv2 - nv0)) + (var_cofdsmt_dn14 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn15 * (nv2 - nv0)) + (var_cofdsmt_dn15 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn16 * (nv2 - nv0)) + (var_cofdsmt_dn16 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn17 * (nv2 - nv0)) + (var_cofdsmt_dn17 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn18 * (nv2 - nv0)) + (var_cofdsmt_dn18 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn19 * (nv2 - nv0)) + (var_cofdsmt_dn19 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn20 * (nv2 - nv0)) + (var_cofdsmt_dn20 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn21 * (nv2 - nv0)) + (var_cofdsmt_dn21 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn22 * (nv2 - nv0)) + (var_cofdsmt_dn22 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn23 * (nv2 - nv0)) + (var_cofdsmt_dn23 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn24 * (nv2 - nv0)) + (var_cofdsmt_dn24 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn25 * (nv2 - nv0)) + (var_cofdsmt_dn25 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn26 * (nv2 - nv0)) + (var_cofdsmt_dn26 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn27 * (nv2 - nv0)) + (var_cofdsmt_dn27 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn28 * (nv2 - nv0)) + (var_cofdsmt_dn28 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_dn29 * (nv2 - nv0)) + (var_cofdsmt_dn29 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db0 * (nv2 - nv0)) + (var_cofdsmt_db0 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db1 * (nv2 - nv0)) + (var_cofdsmt_db1 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db2 * (nv2 - nv0)) + (var_cofdsmt_db2 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db3 * (nv2 - nv0)) + (var_cofdsmt_db3 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db4 * (nv2 - nv0)) + (var_cofdsmt_db4 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db5 * (nv2 - nv0)) + (var_cofdsmt_db5 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db6 * (nv2 - nv0)) + (var_cofdsmt_db6 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db7 * (nv2 - nv0)) + (var_cofdsmt_db7 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db8 * (nv2 - nv0)) + (var_cofdsmt_db8 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db9 * (nv2 - nv0)) + (var_cofdsmt_db9 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db10 * (nv2 - nv0)) + (var_cofdsmt_db10 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db11 * (nv2 - nv0)) + (var_cofdsmt_db11 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db12 * (nv2 - nv0)) + (var_cofdsmt_db12 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db13 * (nv2 - nv0)) + (var_cofdsmt_db13 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db14 * (nv2 - nv0)) + (var_cofdsmt_db14 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db15 * (nv2 - nv0)) + (var_cofdsmt_db15 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db16 * (nv2 - nv0)) + (var_cofdsmt_db16 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db17 * (nv2 - nv0)) + (var_cofdsmt_db17 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db18 * (nv2 - nv0)) + (var_cofdsmt_db18 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db19 * (nv2 - nv0)) + (var_cofdsmt_db19 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db20 * (nv2 - nv0)) + (var_cofdsmt_db20 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db21 * (nv2 - nv0)) + (var_cofdsmt_db21 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db22 * (nv2 - nv0)) + (var_cofdsmt_db22 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db23 * (nv2 - nv0)) + (var_cofdsmt_db23 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db24 * (nv2 - nv0)) + (var_cofdsmt_db24 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db25 * (nv2 - nv0)) + (var_cofdsmt_db25 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db26 * (nv2 - nv0)) + (var_cofdsmt_db26 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db27 * (nv2 - nv0)) + (var_cofdsmt_db27 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db28 * (nv2 - nv0)) + (var_cofdsmt_db28 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db29 * (nv2 - nv0)) + (var_cofdsmt_db29 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db30 * (nv2 - nv0)) + (var_cofdsmt_db30 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db31 * (nv2 - nv0)) + (var_cofdsmt_db31 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db32 * (nv2 - nv0)) + (var_cofdsmt_db32 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db33 * (nv2 - nv0)) + (var_cofdsmt_db33 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db34 * (nv2 - nv0)) + (var_cofdsmt_db34 * assign46270_e44991))), (assign46270_e44984 * ((var_cofdsmt0_db35 * (nv2 - nv0)) + (var_cofdsmt_db35 * assign46270_e44991))),)
    } else {
        (var_qofds, var_qofds_dn0, var_qofds_dn1, var_qofds_dn2, var_qofds_dn3, var_qofds_dn4, var_qofds_dn5, var_qofds_dn6, var_qofds_dn7, var_qofds_dn8, var_qofds_dn9, var_qofds_dn10, var_qofds_dn11, var_qofds_dn12, var_qofds_dn13, var_qofds_dn14, var_qofds_dn15, var_qofds_dn16, var_qofds_dn17, var_qofds_dn18, var_qofds_dn19, var_qofds_dn20, var_qofds_dn21, var_qofds_dn22, var_qofds_dn23, var_qofds_dn24, var_qofds_dn25, var_qofds_dn26, var_qofds_dn27, var_qofds_dn28, var_qofds_dn29, var_qofds_db0, var_qofds_db1, var_qofds_db2, var_qofds_db3, var_qofds_db4, var_qofds_db5, var_qofds_db6, var_qofds_db7, var_qofds_db8, var_qofds_db9, var_qofds_db10, var_qofds_db11, var_qofds_db12, var_qofds_db13, var_qofds_db14, var_qofds_db15, var_qofds_db16, var_qofds_db17, var_qofds_db18, var_qofds_db19, var_qofds_db20, var_qofds_db21, var_qofds_db22, var_qofds_db23, var_qofds_db24, var_qofds_db25, var_qofds_db26, var_qofds_db27, var_qofds_db28, var_qofds_db29, var_qofds_db30, var_qofds_db31, var_qofds_db32, var_qofds_db33, var_qofds_db34, var_qofds_db35,)
    }
};
        var_qofds = assign46270_e44996;
        var_qofds_dn0 = assign46270_e44996_d_n0;
        var_qofds_dn1 = assign46270_e44996_d_n1;
        var_qofds_dn2 = assign46270_e44996_d_n2;
        var_qofds_dn3 = assign46270_e44996_d_n3;
        var_qofds_dn4 = assign46270_e44996_d_n4;
        var_qofds_dn5 = assign46270_e44996_d_n5;
        var_qofds_dn6 = assign46270_e44996_d_n6;
        var_qofds_dn7 = assign46270_e44996_d_n7;
        var_qofds_dn8 = assign46270_e44996_d_n8;
        var_qofds_dn9 = assign46270_e44996_d_n9;
        var_qofds_dn10 = assign46270_e44996_d_n10;
        var_qofds_dn11 = assign46270_e44996_d_n11;
        var_qofds_dn12 = assign46270_e44996_d_n12;
        var_qofds_dn13 = assign46270_e44996_d_n13;
        var_qofds_dn14 = assign46270_e44996_d_n14;
        var_qofds_dn15 = assign46270_e44996_d_n15;
        var_qofds_dn16 = assign46270_e44996_d_n16;
        var_qofds_dn17 = assign46270_e44996_d_n17;
        var_qofds_dn18 = assign46270_e44996_d_n18;
        var_qofds_dn19 = assign46270_e44996_d_n19;
        var_qofds_dn20 = assign46270_e44996_d_n20;
        var_qofds_dn21 = assign46270_e44996_d_n21;
        var_qofds_dn22 = assign46270_e44996_d_n22;
        var_qofds_dn23 = assign46270_e44996_d_n23;
        var_qofds_dn24 = assign46270_e44996_d_n24;
        var_qofds_dn25 = assign46270_e44996_d_n25;
        var_qofds_dn26 = assign46270_e44996_d_n26;
        var_qofds_dn27 = assign46270_e44996_d_n27;
        var_qofds_dn28 = assign46270_e44996_d_n28;
        var_qofds_dn29 = assign46270_e44996_d_n29;
        var_qofds_db0 = assign46270_e44996_d_b0;
        var_qofds_db1 = assign46270_e44996_d_b1;
        var_qofds_db2 = assign46270_e44996_d_b2;
        var_qofds_db3 = assign46270_e44996_d_b3;
        var_qofds_db4 = assign46270_e44996_d_b4;
        var_qofds_db5 = assign46270_e44996_d_b5;
        var_qofds_db6 = assign46270_e44996_d_b6;
        var_qofds_db7 = assign46270_e44996_d_b7;
        var_qofds_db8 = assign46270_e44996_d_b8;
        var_qofds_db9 = assign46270_e44996_d_b9;
        var_qofds_db10 = assign46270_e44996_d_b10;
        var_qofds_db11 = assign46270_e44996_d_b11;
        var_qofds_db12 = assign46270_e44996_d_b12;
        var_qofds_db13 = assign46270_e44996_d_b13;
        var_qofds_db14 = assign46270_e44996_d_b14;
        var_qofds_db15 = assign46270_e44996_d_b15;
        var_qofds_db16 = assign46270_e44996_d_b16;
        var_qofds_db17 = assign46270_e44996_d_b17;
        var_qofds_db18 = assign46270_e44996_d_b18;
        var_qofds_db19 = assign46270_e44996_d_b19;
        var_qofds_db20 = assign46270_e44996_d_b20;
        var_qofds_db21 = assign46270_e44996_d_b21;
        var_qofds_db22 = assign46270_e44996_d_b22;
        var_qofds_db23 = assign46270_e44996_d_b23;
        var_qofds_db24 = assign46270_e44996_d_b24;
        var_qofds_db25 = assign46270_e44996_d_b25;
        var_qofds_db26 = assign46270_e44996_d_b26;
        var_qofds_db27 = assign46270_e44996_d_b27;
        var_qofds_db28 = assign46270_e44996_d_b28;
        var_qofds_db29 = assign46270_e44996_d_b29;
        var_qofds_db30 = assign46270_e44996_d_b30;
        var_qofds_db31 = assign46270_e44996_d_b31;
        var_qofds_db32 = assign46270_e44996_d_b32;
        var_qofds_db33 = assign46270_e44996_d_b33;
        var_qofds_db34 = assign46270_e44996_d_b34;
        var_qofds_db35 = assign46270_e44996_d_b35;

        let assign46280_e44999: f64 = ((nv2 - nv0) - p.p27);
        let assign46280_e45001: f64 = (assign46280_e44999 / p.p28);
        let assign46280_e45003: f64 = (-50.0);
        let assign46280_e45004: f64 = if assign46280_e45001 < assign46280_e45003 { 1.0 } else { 0.0 };
        var_guard502 = assign46280_e45004;


        *var_guard499_slot = var_guard499;
        *var_guard500_slot = var_guard500;
        *var_guard501_slot = var_guard501;
        *var_guard502_slot = var_guard502;
        *var_qofd_slot = var_qofd;
        *var_qofd_db0_slot = var_qofd_db0;
        *var_qofd_db1_slot = var_qofd_db1;
        *var_qofd_db10_slot = var_qofd_db10;
        *var_qofd_db11_slot = var_qofd_db11;
        *var_qofd_db12_slot = var_qofd_db12;
        *var_qofd_db13_slot = var_qofd_db13;
        *var_qofd_db14_slot = var_qofd_db14;
        *var_qofd_db15_slot = var_qofd_db15;
        *var_qofd_db16_slot = var_qofd_db16;
        *var_qofd_db17_slot = var_qofd_db17;
        *var_qofd_db18_slot = var_qofd_db18;
        *var_qofd_db19_slot = var_qofd_db19;
        *var_qofd_db2_slot = var_qofd_db2;
        *var_qofd_db20_slot = var_qofd_db20;
        *var_qofd_db21_slot = var_qofd_db21;
        *var_qofd_db22_slot = var_qofd_db22;
        *var_qofd_db23_slot = var_qofd_db23;
        *var_qofd_db24_slot = var_qofd_db24;
        *var_qofd_db25_slot = var_qofd_db25;
        *var_qofd_db26_slot = var_qofd_db26;
        *var_qofd_db27_slot = var_qofd_db27;
        *var_qofd_db28_slot = var_qofd_db28;
        *var_qofd_db29_slot = var_qofd_db29;
        *var_qofd_db3_slot = var_qofd_db3;
        *var_qofd_db30_slot = var_qofd_db30;
        *var_qofd_db31_slot = var_qofd_db31;
        *var_qofd_db32_slot = var_qofd_db32;
        *var_qofd_db33_slot = var_qofd_db33;
        *var_qofd_db34_slot = var_qofd_db34;
        *var_qofd_db35_slot = var_qofd_db35;
        *var_qofd_db4_slot = var_qofd_db4;
        *var_qofd_db5_slot = var_qofd_db5;
        *var_qofd_db6_slot = var_qofd_db6;
        *var_qofd_db7_slot = var_qofd_db7;
        *var_qofd_db8_slot = var_qofd_db8;
        *var_qofd_db9_slot = var_qofd_db9;
        *var_qofd_dn0_slot = var_qofd_dn0;
        *var_qofd_dn1_slot = var_qofd_dn1;
        *var_qofd_dn10_slot = var_qofd_dn10;
        *var_qofd_dn11_slot = var_qofd_dn11;
        *var_qofd_dn12_slot = var_qofd_dn12;
        *var_qofd_dn13_slot = var_qofd_dn13;
        *var_qofd_dn14_slot = var_qofd_dn14;
        *var_qofd_dn15_slot = var_qofd_dn15;
        *var_qofd_dn16_slot = var_qofd_dn16;
        *var_qofd_dn17_slot = var_qofd_dn17;
        *var_qofd_dn18_slot = var_qofd_dn18;
        *var_qofd_dn19_slot = var_qofd_dn19;
        *var_qofd_dn2_slot = var_qofd_dn2;
        *var_qofd_dn20_slot = var_qofd_dn20;
        *var_qofd_dn21_slot = var_qofd_dn21;
        *var_qofd_dn22_slot = var_qofd_dn22;
        *var_qofd_dn23_slot = var_qofd_dn23;
        *var_qofd_dn24_slot = var_qofd_dn24;
        *var_qofd_dn25_slot = var_qofd_dn25;
        *var_qofd_dn26_slot = var_qofd_dn26;
        *var_qofd_dn27_slot = var_qofd_dn27;
        *var_qofd_dn28_slot = var_qofd_dn28;
        *var_qofd_dn29_slot = var_qofd_dn29;
        *var_qofd_dn3_slot = var_qofd_dn3;
        *var_qofd_dn4_slot = var_qofd_dn4;
        *var_qofd_dn5_slot = var_qofd_dn5;
        *var_qofd_dn6_slot = var_qofd_dn6;
        *var_qofd_dn7_slot = var_qofd_dn7;
        *var_qofd_dn8_slot = var_qofd_dn8;
        *var_qofd_dn9_slot = var_qofd_dn9;
        *var_qofds_slot = var_qofds;
        *var_qofds_db0_slot = var_qofds_db0;
        *var_qofds_db1_slot = var_qofds_db1;
        *var_qofds_db10_slot = var_qofds_db10;
        *var_qofds_db11_slot = var_qofds_db11;
        *var_qofds_db12_slot = var_qofds_db12;
        *var_qofds_db13_slot = var_qofds_db13;
        *var_qofds_db14_slot = var_qofds_db14;
        *var_qofds_db15_slot = var_qofds_db15;
        *var_qofds_db16_slot = var_qofds_db16;
        *var_qofds_db17_slot = var_qofds_db17;
        *var_qofds_db18_slot = var_qofds_db18;
        *var_qofds_db19_slot = var_qofds_db19;
        *var_qofds_db2_slot = var_qofds_db2;
        *var_qofds_db20_slot = var_qofds_db20;
        *var_qofds_db21_slot = var_qofds_db21;
        *var_qofds_db22_slot = var_qofds_db22;
        *var_qofds_db23_slot = var_qofds_db23;
        *var_qofds_db24_slot = var_qofds_db24;
        *var_qofds_db25_slot = var_qofds_db25;
        *var_qofds_db26_slot = var_qofds_db26;
        *var_qofds_db27_slot = var_qofds_db27;
        *var_qofds_db28_slot = var_qofds_db28;
        *var_qofds_db29_slot = var_qofds_db29;
        *var_qofds_db3_slot = var_qofds_db3;
        *var_qofds_db30_slot = var_qofds_db30;
        *var_qofds_db31_slot = var_qofds_db31;
        *var_qofds_db32_slot = var_qofds_db32;
        *var_qofds_db33_slot = var_qofds_db33;
        *var_qofds_db34_slot = var_qofds_db34;
        *var_qofds_db35_slot = var_qofds_db35;
        *var_qofds_db4_slot = var_qofds_db4;
        *var_qofds_db5_slot = var_qofds_db5;
        *var_qofds_db6_slot = var_qofds_db6;
        *var_qofds_db7_slot = var_qofds_db7;
        *var_qofds_db8_slot = var_qofds_db8;
        *var_qofds_db9_slot = var_qofds_db9;
        *var_qofds_dn0_slot = var_qofds_dn0;
        *var_qofds_dn1_slot = var_qofds_dn1;
        *var_qofds_dn10_slot = var_qofds_dn10;
        *var_qofds_dn11_slot = var_qofds_dn11;
        *var_qofds_dn12_slot = var_qofds_dn12;
        *var_qofds_dn13_slot = var_qofds_dn13;
        *var_qofds_dn14_slot = var_qofds_dn14;
        *var_qofds_dn15_slot = var_qofds_dn15;
        *var_qofds_dn16_slot = var_qofds_dn16;
        *var_qofds_dn17_slot = var_qofds_dn17;
        *var_qofds_dn18_slot = var_qofds_dn18;
        *var_qofds_dn19_slot = var_qofds_dn19;
        *var_qofds_dn2_slot = var_qofds_dn2;
        *var_qofds_dn20_slot = var_qofds_dn20;
        *var_qofds_dn21_slot = var_qofds_dn21;
        *var_qofds_dn22_slot = var_qofds_dn22;
        *var_qofds_dn23_slot = var_qofds_dn23;
        *var_qofds_dn24_slot = var_qofds_dn24;
        *var_qofds_dn25_slot = var_qofds_dn25;
        *var_qofds_dn26_slot = var_qofds_dn26;
        *var_qofds_dn27_slot = var_qofds_dn27;
        *var_qofds_dn28_slot = var_qofds_dn28;
        *var_qofds_dn29_slot = var_qofds_dn29;
        *var_qofds_dn3_slot = var_qofds_dn3;
        *var_qofds_dn4_slot = var_qofds_dn4;
        *var_qofds_dn5_slot = var_qofds_dn5;
        *var_qofds_dn6_slot = var_qofds_dn6;
        *var_qofds_dn7_slot = var_qofds_dn7;
        *var_qofds_dn8_slot = var_qofds_dn8;
        *var_qofds_dn9_slot = var_qofds_dn9;
        *var_qofs_slot = var_qofs;
        *var_qofs_db0_slot = var_qofs_db0;
        *var_qofs_db1_slot = var_qofs_db1;
        *var_qofs_db10_slot = var_qofs_db10;
        *var_qofs_db11_slot = var_qofs_db11;
        *var_qofs_db12_slot = var_qofs_db12;
        *var_qofs_db13_slot = var_qofs_db13;
        *var_qofs_db14_slot = var_qofs_db14;
        *var_qofs_db15_slot = var_qofs_db15;
        *var_qofs_db16_slot = var_qofs_db16;
        *var_qofs_db17_slot = var_qofs_db17;
        *var_qofs_db18_slot = var_qofs_db18;
        *var_qofs_db19_slot = var_qofs_db19;
        *var_qofs_db2_slot = var_qofs_db2;
        *var_qofs_db20_slot = var_qofs_db20;
        *var_qofs_db21_slot = var_qofs_db21;
        *var_qofs_db22_slot = var_qofs_db22;
        *var_qofs_db23_slot = var_qofs_db23;
        *var_qofs_db24_slot = var_qofs_db24;
        *var_qofs_db25_slot = var_qofs_db25;
        *var_qofs_db26_slot = var_qofs_db26;
        *var_qofs_db27_slot = var_qofs_db27;
        *var_qofs_db28_slot = var_qofs_db28;
        *var_qofs_db29_slot = var_qofs_db29;
        *var_qofs_db3_slot = var_qofs_db3;
        *var_qofs_db30_slot = var_qofs_db30;
        *var_qofs_db31_slot = var_qofs_db31;
        *var_qofs_db32_slot = var_qofs_db32;
        *var_qofs_db33_slot = var_qofs_db33;
        *var_qofs_db34_slot = var_qofs_db34;
        *var_qofs_db35_slot = var_qofs_db35;
        *var_qofs_db4_slot = var_qofs_db4;
        *var_qofs_db5_slot = var_qofs_db5;
        *var_qofs_db6_slot = var_qofs_db6;
        *var_qofs_db7_slot = var_qofs_db7;
        *var_qofs_db8_slot = var_qofs_db8;
        *var_qofs_db9_slot = var_qofs_db9;
        *var_qofs_dn0_slot = var_qofs_dn0;
        *var_qofs_dn1_slot = var_qofs_dn1;
        *var_qofs_dn10_slot = var_qofs_dn10;
        *var_qofs_dn11_slot = var_qofs_dn11;
        *var_qofs_dn12_slot = var_qofs_dn12;
        *var_qofs_dn13_slot = var_qofs_dn13;
        *var_qofs_dn14_slot = var_qofs_dn14;
        *var_qofs_dn15_slot = var_qofs_dn15;
        *var_qofs_dn16_slot = var_qofs_dn16;
        *var_qofs_dn17_slot = var_qofs_dn17;
        *var_qofs_dn18_slot = var_qofs_dn18;
        *var_qofs_dn19_slot = var_qofs_dn19;
        *var_qofs_dn2_slot = var_qofs_dn2;
        *var_qofs_dn20_slot = var_qofs_dn20;
        *var_qofs_dn21_slot = var_qofs_dn21;
        *var_qofs_dn22_slot = var_qofs_dn22;
        *var_qofs_dn23_slot = var_qofs_dn23;
        *var_qofs_dn24_slot = var_qofs_dn24;
        *var_qofs_dn25_slot = var_qofs_dn25;
        *var_qofs_dn26_slot = var_qofs_dn26;
        *var_qofs_dn27_slot = var_qofs_dn27;
        *var_qofs_dn28_slot = var_qofs_dn28;
        *var_qofs_dn29_slot = var_qofs_dn29;
        *var_qofs_dn3_slot = var_qofs_dn3;
        *var_qofs_dn4_slot = var_qofs_dn4;
        *var_qofs_dn5_slot = var_qofs_dn5;
        *var_qofs_dn6_slot = var_qofs_dn6;
        *var_qofs_dn7_slot = var_qofs_dn7;
        *var_qofs_dn8_slot = var_qofs_dn8;
        *var_qofs_dn9_slot = var_qofs_dn9;
    }

    pub(super) fn stamp_transient_block_121(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_cofdsmt: f64,
        var_cofdsmt0: f64,
        var_cofdsmt0_db0: f64,
        var_cofdsmt0_db1: f64,
        var_cofdsmt0_db10: f64,
        var_cofdsmt0_db11: f64,
        var_cofdsmt0_db12: f64,
        var_cofdsmt0_db13: f64,
        var_cofdsmt0_db14: f64,
        var_cofdsmt0_db15: f64,
        var_cofdsmt0_db16: f64,
        var_cofdsmt0_db17: f64,
        var_cofdsmt0_db18: f64,
        var_cofdsmt0_db19: f64,
        var_cofdsmt0_db2: f64,
        var_cofdsmt0_db20: f64,
        var_cofdsmt0_db21: f64,
        var_cofdsmt0_db22: f64,
        var_cofdsmt0_db23: f64,
        var_cofdsmt0_db24: f64,
        var_cofdsmt0_db25: f64,
        var_cofdsmt0_db26: f64,
        var_cofdsmt0_db27: f64,
        var_cofdsmt0_db28: f64,
        var_cofdsmt0_db29: f64,
        var_cofdsmt0_db3: f64,
        var_cofdsmt0_db30: f64,
        var_cofdsmt0_db31: f64,
        var_cofdsmt0_db32: f64,
        var_cofdsmt0_db33: f64,
        var_cofdsmt0_db34: f64,
        var_cofdsmt0_db35: f64,
        var_cofdsmt0_db4: f64,
        var_cofdsmt0_db5: f64,
        var_cofdsmt0_db6: f64,
        var_cofdsmt0_db7: f64,
        var_cofdsmt0_db8: f64,
        var_cofdsmt0_db9: f64,
        var_cofdsmt0_dn0: f64,
        var_cofdsmt0_dn1: f64,
        var_cofdsmt0_dn10: f64,
        var_cofdsmt0_dn11: f64,
        var_cofdsmt0_dn12: f64,
        var_cofdsmt0_dn13: f64,
        var_cofdsmt0_dn14: f64,
        var_cofdsmt0_dn15: f64,
        var_cofdsmt0_dn16: f64,
        var_cofdsmt0_dn17: f64,
        var_cofdsmt0_dn18: f64,
        var_cofdsmt0_dn19: f64,
        var_cofdsmt0_dn2: f64,
        var_cofdsmt0_dn20: f64,
        var_cofdsmt0_dn21: f64,
        var_cofdsmt0_dn22: f64,
        var_cofdsmt0_dn23: f64,
        var_cofdsmt0_dn24: f64,
        var_cofdsmt0_dn25: f64,
        var_cofdsmt0_dn26: f64,
        var_cofdsmt0_dn27: f64,
        var_cofdsmt0_dn28: f64,
        var_cofdsmt0_dn29: f64,
        var_cofdsmt0_dn3: f64,
        var_cofdsmt0_dn4: f64,
        var_cofdsmt0_dn5: f64,
        var_cofdsmt0_dn6: f64,
        var_cofdsmt0_dn7: f64,
        var_cofdsmt0_dn8: f64,
        var_cofdsmt0_dn9: f64,
        var_cofdsmt_db0: f64,
        var_cofdsmt_db1: f64,
        var_cofdsmt_db10: f64,
        var_cofdsmt_db11: f64,
        var_cofdsmt_db12: f64,
        var_cofdsmt_db13: f64,
        var_cofdsmt_db14: f64,
        var_cofdsmt_db15: f64,
        var_cofdsmt_db16: f64,
        var_cofdsmt_db17: f64,
        var_cofdsmt_db18: f64,
        var_cofdsmt_db19: f64,
        var_cofdsmt_db2: f64,
        var_cofdsmt_db20: f64,
        var_cofdsmt_db21: f64,
        var_cofdsmt_db22: f64,
        var_cofdsmt_db23: f64,
        var_cofdsmt_db24: f64,
        var_cofdsmt_db25: f64,
        var_cofdsmt_db26: f64,
        var_cofdsmt_db27: f64,
        var_cofdsmt_db28: f64,
        var_cofdsmt_db29: f64,
        var_cofdsmt_db3: f64,
        var_cofdsmt_db30: f64,
        var_cofdsmt_db31: f64,
        var_cofdsmt_db32: f64,
        var_cofdsmt_db33: f64,
        var_cofdsmt_db34: f64,
        var_cofdsmt_db35: f64,
        var_cofdsmt_db4: f64,
        var_cofdsmt_db5: f64,
        var_cofdsmt_db6: f64,
        var_cofdsmt_db7: f64,
        var_cofdsmt_db8: f64,
        var_cofdsmt_db9: f64,
        var_cofdsmt_dn0: f64,
        var_cofdsmt_dn1: f64,
        var_cofdsmt_dn10: f64,
        var_cofdsmt_dn11: f64,
        var_cofdsmt_dn12: f64,
        var_cofdsmt_dn13: f64,
        var_cofdsmt_dn14: f64,
        var_cofdsmt_dn15: f64,
        var_cofdsmt_dn16: f64,
        var_cofdsmt_dn17: f64,
        var_cofdsmt_dn18: f64,
        var_cofdsmt_dn19: f64,
        var_cofdsmt_dn2: f64,
        var_cofdsmt_dn20: f64,
        var_cofdsmt_dn21: f64,
        var_cofdsmt_dn22: f64,
        var_cofdsmt_dn23: f64,
        var_cofdsmt_dn24: f64,
        var_cofdsmt_dn25: f64,
        var_cofdsmt_dn26: f64,
        var_cofdsmt_dn27: f64,
        var_cofdsmt_dn28: f64,
        var_cofdsmt_dn29: f64,
        var_cofdsmt_dn3: f64,
        var_cofdsmt_dn4: f64,
        var_cofdsmt_dn5: f64,
        var_cofdsmt_dn6: f64,
        var_cofdsmt_dn7: f64,
        var_cofdsmt_dn8: f64,
        var_cofdsmt_dn9: f64,
        var_cofssubmt: f64,
        var_cofssubmt0: f64,
        var_cofssubmt0_db0: f64,
        var_cofssubmt0_db1: f64,
        var_cofssubmt0_db10: f64,
        var_cofssubmt0_db11: f64,
        var_cofssubmt0_db12: f64,
        var_cofssubmt0_db13: f64,
        var_cofssubmt0_db14: f64,
        var_cofssubmt0_db15: f64,
        var_cofssubmt0_db16: f64,
        var_cofssubmt0_db17: f64,
        var_cofssubmt0_db18: f64,
        var_cofssubmt0_db19: f64,
        var_cofssubmt0_db2: f64,
        var_cofssubmt0_db20: f64,
        var_cofssubmt0_db21: f64,
        var_cofssubmt0_db22: f64,
        var_cofssubmt0_db23: f64,
        var_cofssubmt0_db24: f64,
        var_cofssubmt0_db25: f64,
        var_cofssubmt0_db26: f64,
        var_cofssubmt0_db27: f64,
        var_cofssubmt0_db28: f64,
        var_cofssubmt0_db29: f64,
        var_cofssubmt0_db3: f64,
        var_cofssubmt0_db30: f64,
        var_cofssubmt0_db31: f64,
        var_cofssubmt0_db32: f64,
        var_cofssubmt0_db33: f64,
        var_cofssubmt0_db34: f64,
        var_cofssubmt0_db35: f64,
        var_cofssubmt0_db4: f64,
        var_cofssubmt0_db5: f64,
        var_cofssubmt0_db6: f64,
        var_cofssubmt0_db7: f64,
        var_cofssubmt0_db8: f64,
        var_cofssubmt0_db9: f64,
        var_cofssubmt0_dn0: f64,
        var_cofssubmt0_dn1: f64,
        var_cofssubmt0_dn10: f64,
        var_cofssubmt0_dn11: f64,
        var_cofssubmt0_dn12: f64,
        var_cofssubmt0_dn13: f64,
        var_cofssubmt0_dn14: f64,
        var_cofssubmt0_dn15: f64,
        var_cofssubmt0_dn16: f64,
        var_cofssubmt0_dn17: f64,
        var_cofssubmt0_dn18: f64,
        var_cofssubmt0_dn19: f64,
        var_cofssubmt0_dn2: f64,
        var_cofssubmt0_dn20: f64,
        var_cofssubmt0_dn21: f64,
        var_cofssubmt0_dn22: f64,
        var_cofssubmt0_dn23: f64,
        var_cofssubmt0_dn24: f64,
        var_cofssubmt0_dn25: f64,
        var_cofssubmt0_dn26: f64,
        var_cofssubmt0_dn27: f64,
        var_cofssubmt0_dn28: f64,
        var_cofssubmt0_dn29: f64,
        var_cofssubmt0_dn3: f64,
        var_cofssubmt0_dn4: f64,
        var_cofssubmt0_dn5: f64,
        var_cofssubmt0_dn6: f64,
        var_cofssubmt0_dn7: f64,
        var_cofssubmt0_dn8: f64,
        var_cofssubmt0_dn9: f64,
        var_cofssubmt_db0: f64,
        var_cofssubmt_db1: f64,
        var_cofssubmt_db10: f64,
        var_cofssubmt_db11: f64,
        var_cofssubmt_db12: f64,
        var_cofssubmt_db13: f64,
        var_cofssubmt_db14: f64,
        var_cofssubmt_db15: f64,
        var_cofssubmt_db16: f64,
        var_cofssubmt_db17: f64,
        var_cofssubmt_db18: f64,
        var_cofssubmt_db19: f64,
        var_cofssubmt_db2: f64,
        var_cofssubmt_db20: f64,
        var_cofssubmt_db21: f64,
        var_cofssubmt_db22: f64,
        var_cofssubmt_db23: f64,
        var_cofssubmt_db24: f64,
        var_cofssubmt_db25: f64,
        var_cofssubmt_db26: f64,
        var_cofssubmt_db27: f64,
        var_cofssubmt_db28: f64,
        var_cofssubmt_db29: f64,
        var_cofssubmt_db3: f64,
        var_cofssubmt_db30: f64,
        var_cofssubmt_db31: f64,
        var_cofssubmt_db32: f64,
        var_cofssubmt_db33: f64,
        var_cofssubmt_db34: f64,
        var_cofssubmt_db35: f64,
        var_cofssubmt_db4: f64,
        var_cofssubmt_db5: f64,
        var_cofssubmt_db6: f64,
        var_cofssubmt_db7: f64,
        var_cofssubmt_db8: f64,
        var_cofssubmt_db9: f64,
        var_cofssubmt_dn0: f64,
        var_cofssubmt_dn1: f64,
        var_cofssubmt_dn10: f64,
        var_cofssubmt_dn11: f64,
        var_cofssubmt_dn12: f64,
        var_cofssubmt_dn13: f64,
        var_cofssubmt_dn14: f64,
        var_cofssubmt_dn15: f64,
        var_cofssubmt_dn16: f64,
        var_cofssubmt_dn17: f64,
        var_cofssubmt_dn18: f64,
        var_cofssubmt_dn19: f64,
        var_cofssubmt_dn2: f64,
        var_cofssubmt_dn20: f64,
        var_cofssubmt_dn21: f64,
        var_cofssubmt_dn22: f64,
        var_cofssubmt_dn23: f64,
        var_cofssubmt_dn24: f64,
        var_cofssubmt_dn25: f64,
        var_cofssubmt_dn26: f64,
        var_cofssubmt_dn27: f64,
        var_cofssubmt_dn28: f64,
        var_cofssubmt_dn29: f64,
        var_cofssubmt_dn3: f64,
        var_cofssubmt_dn4: f64,
        var_cofssubmt_dn5: f64,
        var_cofssubmt_dn6: f64,
        var_cofssubmt_dn7: f64,
        var_cofssubmt_dn8: f64,
        var_cofssubmt_dn9: f64,
        var_guard501: f64,
        var_guard502: f64,
        var_guard503_slot: &mut f64,
        var_guard504_slot: &mut f64,
        var_guard505_slot: &mut f64,
        var_qofds_slot: &mut f64,
        var_qofds_db0_slot: &mut f64,
        var_qofds_db1_slot: &mut f64,
        var_qofds_db10_slot: &mut f64,
        var_qofds_db11_slot: &mut f64,
        var_qofds_db12_slot: &mut f64,
        var_qofds_db13_slot: &mut f64,
        var_qofds_db14_slot: &mut f64,
        var_qofds_db15_slot: &mut f64,
        var_qofds_db16_slot: &mut f64,
        var_qofds_db17_slot: &mut f64,
        var_qofds_db18_slot: &mut f64,
        var_qofds_db19_slot: &mut f64,
        var_qofds_db2_slot: &mut f64,
        var_qofds_db20_slot: &mut f64,
        var_qofds_db21_slot: &mut f64,
        var_qofds_db22_slot: &mut f64,
        var_qofds_db23_slot: &mut f64,
        var_qofds_db24_slot: &mut f64,
        var_qofds_db25_slot: &mut f64,
        var_qofds_db26_slot: &mut f64,
        var_qofds_db27_slot: &mut f64,
        var_qofds_db28_slot: &mut f64,
        var_qofds_db29_slot: &mut f64,
        var_qofds_db3_slot: &mut f64,
        var_qofds_db30_slot: &mut f64,
        var_qofds_db31_slot: &mut f64,
        var_qofds_db32_slot: &mut f64,
        var_qofds_db33_slot: &mut f64,
        var_qofds_db34_slot: &mut f64,
        var_qofds_db35_slot: &mut f64,
        var_qofds_db4_slot: &mut f64,
        var_qofds_db5_slot: &mut f64,
        var_qofds_db6_slot: &mut f64,
        var_qofds_db7_slot: &mut f64,
        var_qofds_db8_slot: &mut f64,
        var_qofds_db9_slot: &mut f64,
        var_qofds_dn0_slot: &mut f64,
        var_qofds_dn1_slot: &mut f64,
        var_qofds_dn10_slot: &mut f64,
        var_qofds_dn11_slot: &mut f64,
        var_qofds_dn12_slot: &mut f64,
        var_qofds_dn13_slot: &mut f64,
        var_qofds_dn14_slot: &mut f64,
        var_qofds_dn15_slot: &mut f64,
        var_qofds_dn16_slot: &mut f64,
        var_qofds_dn17_slot: &mut f64,
        var_qofds_dn18_slot: &mut f64,
        var_qofds_dn19_slot: &mut f64,
        var_qofds_dn2_slot: &mut f64,
        var_qofds_dn20_slot: &mut f64,
        var_qofds_dn21_slot: &mut f64,
        var_qofds_dn22_slot: &mut f64,
        var_qofds_dn23_slot: &mut f64,
        var_qofds_dn24_slot: &mut f64,
        var_qofds_dn25_slot: &mut f64,
        var_qofds_dn26_slot: &mut f64,
        var_qofds_dn27_slot: &mut f64,
        var_qofds_dn28_slot: &mut f64,
        var_qofds_dn29_slot: &mut f64,
        var_qofds_dn3_slot: &mut f64,
        var_qofds_dn4_slot: &mut f64,
        var_qofds_dn5_slot: &mut f64,
        var_qofds_dn6_slot: &mut f64,
        var_qofds_dn7_slot: &mut f64,
        var_qofds_dn8_slot: &mut f64,
        var_qofds_dn9_slot: &mut f64,
        var_qofssub_slot: &mut f64,
        var_qofssub_db0_slot: &mut f64,
        var_qofssub_db1_slot: &mut f64,
        var_qofssub_db10_slot: &mut f64,
        var_qofssub_db11_slot: &mut f64,
        var_qofssub_db12_slot: &mut f64,
        var_qofssub_db13_slot: &mut f64,
        var_qofssub_db14_slot: &mut f64,
        var_qofssub_db15_slot: &mut f64,
        var_qofssub_db16_slot: &mut f64,
        var_qofssub_db17_slot: &mut f64,
        var_qofssub_db18_slot: &mut f64,
        var_qofssub_db19_slot: &mut f64,
        var_qofssub_db2_slot: &mut f64,
        var_qofssub_db20_slot: &mut f64,
        var_qofssub_db21_slot: &mut f64,
        var_qofssub_db22_slot: &mut f64,
        var_qofssub_db23_slot: &mut f64,
        var_qofssub_db24_slot: &mut f64,
        var_qofssub_db25_slot: &mut f64,
        var_qofssub_db26_slot: &mut f64,
        var_qofssub_db27_slot: &mut f64,
        var_qofssub_db28_slot: &mut f64,
        var_qofssub_db29_slot: &mut f64,
        var_qofssub_db3_slot: &mut f64,
        var_qofssub_db30_slot: &mut f64,
        var_qofssub_db31_slot: &mut f64,
        var_qofssub_db32_slot: &mut f64,
        var_qofssub_db33_slot: &mut f64,
        var_qofssub_db34_slot: &mut f64,
        var_qofssub_db35_slot: &mut f64,
        var_qofssub_db4_slot: &mut f64,
        var_qofssub_db5_slot: &mut f64,
        var_qofssub_db6_slot: &mut f64,
        var_qofssub_db7_slot: &mut f64,
        var_qofssub_db8_slot: &mut f64,
        var_qofssub_db9_slot: &mut f64,
        var_qofssub_dn0_slot: &mut f64,
        var_qofssub_dn1_slot: &mut f64,
        var_qofssub_dn10_slot: &mut f64,
        var_qofssub_dn11_slot: &mut f64,
        var_qofssub_dn12_slot: &mut f64,
        var_qofssub_dn13_slot: &mut f64,
        var_qofssub_dn14_slot: &mut f64,
        var_qofssub_dn15_slot: &mut f64,
        var_qofssub_dn16_slot: &mut f64,
        var_qofssub_dn17_slot: &mut f64,
        var_qofssub_dn18_slot: &mut f64,
        var_qofssub_dn19_slot: &mut f64,
        var_qofssub_dn2_slot: &mut f64,
        var_qofssub_dn20_slot: &mut f64,
        var_qofssub_dn21_slot: &mut f64,
        var_qofssub_dn22_slot: &mut f64,
        var_qofssub_dn23_slot: &mut f64,
        var_qofssub_dn24_slot: &mut f64,
        var_qofssub_dn25_slot: &mut f64,
        var_qofssub_dn26_slot: &mut f64,
        var_qofssub_dn27_slot: &mut f64,
        var_qofssub_dn28_slot: &mut f64,
        var_qofssub_dn29_slot: &mut f64,
        var_qofssub_dn3_slot: &mut f64,
        var_qofssub_dn4_slot: &mut f64,
        var_qofssub_dn5_slot: &mut f64,
        var_qofssub_dn6_slot: &mut f64,
        var_qofssub_dn7_slot: &mut f64,
        var_qofssub_dn8_slot: &mut f64,
        var_qofssub_dn9_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let mut var_guard503: f64 = *var_guard503_slot;
        let mut var_guard504: f64 = *var_guard504_slot;
        let mut var_guard505: f64 = *var_guard505_slot;
        let mut var_qofds: f64 = *var_qofds_slot;
        let mut var_qofds_db0: f64 = *var_qofds_db0_slot;
        let mut var_qofds_db1: f64 = *var_qofds_db1_slot;
        let mut var_qofds_db10: f64 = *var_qofds_db10_slot;
        let mut var_qofds_db11: f64 = *var_qofds_db11_slot;
        let mut var_qofds_db12: f64 = *var_qofds_db12_slot;
        let mut var_qofds_db13: f64 = *var_qofds_db13_slot;
        let mut var_qofds_db14: f64 = *var_qofds_db14_slot;
        let mut var_qofds_db15: f64 = *var_qofds_db15_slot;
        let mut var_qofds_db16: f64 = *var_qofds_db16_slot;
        let mut var_qofds_db17: f64 = *var_qofds_db17_slot;
        let mut var_qofds_db18: f64 = *var_qofds_db18_slot;
        let mut var_qofds_db19: f64 = *var_qofds_db19_slot;
        let mut var_qofds_db2: f64 = *var_qofds_db2_slot;
        let mut var_qofds_db20: f64 = *var_qofds_db20_slot;
        let mut var_qofds_db21: f64 = *var_qofds_db21_slot;
        let mut var_qofds_db22: f64 = *var_qofds_db22_slot;
        let mut var_qofds_db23: f64 = *var_qofds_db23_slot;
        let mut var_qofds_db24: f64 = *var_qofds_db24_slot;
        let mut var_qofds_db25: f64 = *var_qofds_db25_slot;
        let mut var_qofds_db26: f64 = *var_qofds_db26_slot;
        let mut var_qofds_db27: f64 = *var_qofds_db27_slot;
        let mut var_qofds_db28: f64 = *var_qofds_db28_slot;
        let mut var_qofds_db29: f64 = *var_qofds_db29_slot;
        let mut var_qofds_db3: f64 = *var_qofds_db3_slot;
        let mut var_qofds_db30: f64 = *var_qofds_db30_slot;
        let mut var_qofds_db31: f64 = *var_qofds_db31_slot;
        let mut var_qofds_db32: f64 = *var_qofds_db32_slot;
        let mut var_qofds_db33: f64 = *var_qofds_db33_slot;
        let mut var_qofds_db34: f64 = *var_qofds_db34_slot;
        let mut var_qofds_db35: f64 = *var_qofds_db35_slot;
        let mut var_qofds_db4: f64 = *var_qofds_db4_slot;
        let mut var_qofds_db5: f64 = *var_qofds_db5_slot;
        let mut var_qofds_db6: f64 = *var_qofds_db6_slot;
        let mut var_qofds_db7: f64 = *var_qofds_db7_slot;
        let mut var_qofds_db8: f64 = *var_qofds_db8_slot;
        let mut var_qofds_db9: f64 = *var_qofds_db9_slot;
        let mut var_qofds_dn0: f64 = *var_qofds_dn0_slot;
        let mut var_qofds_dn1: f64 = *var_qofds_dn1_slot;
        let mut var_qofds_dn10: f64 = *var_qofds_dn10_slot;
        let mut var_qofds_dn11: f64 = *var_qofds_dn11_slot;
        let mut var_qofds_dn12: f64 = *var_qofds_dn12_slot;
        let mut var_qofds_dn13: f64 = *var_qofds_dn13_slot;
        let mut var_qofds_dn14: f64 = *var_qofds_dn14_slot;
        let mut var_qofds_dn15: f64 = *var_qofds_dn15_slot;
        let mut var_qofds_dn16: f64 = *var_qofds_dn16_slot;
        let mut var_qofds_dn17: f64 = *var_qofds_dn17_slot;
        let mut var_qofds_dn18: f64 = *var_qofds_dn18_slot;
        let mut var_qofds_dn19: f64 = *var_qofds_dn19_slot;
        let mut var_qofds_dn2: f64 = *var_qofds_dn2_slot;
        let mut var_qofds_dn20: f64 = *var_qofds_dn20_slot;
        let mut var_qofds_dn21: f64 = *var_qofds_dn21_slot;
        let mut var_qofds_dn22: f64 = *var_qofds_dn22_slot;
        let mut var_qofds_dn23: f64 = *var_qofds_dn23_slot;
        let mut var_qofds_dn24: f64 = *var_qofds_dn24_slot;
        let mut var_qofds_dn25: f64 = *var_qofds_dn25_slot;
        let mut var_qofds_dn26: f64 = *var_qofds_dn26_slot;
        let mut var_qofds_dn27: f64 = *var_qofds_dn27_slot;
        let mut var_qofds_dn28: f64 = *var_qofds_dn28_slot;
        let mut var_qofds_dn29: f64 = *var_qofds_dn29_slot;
        let mut var_qofds_dn3: f64 = *var_qofds_dn3_slot;
        let mut var_qofds_dn4: f64 = *var_qofds_dn4_slot;
        let mut var_qofds_dn5: f64 = *var_qofds_dn5_slot;
        let mut var_qofds_dn6: f64 = *var_qofds_dn6_slot;
        let mut var_qofds_dn7: f64 = *var_qofds_dn7_slot;
        let mut var_qofds_dn8: f64 = *var_qofds_dn8_slot;
        let mut var_qofds_dn9: f64 = *var_qofds_dn9_slot;
        let mut var_qofssub: f64 = *var_qofssub_slot;
        let mut var_qofssub_db0: f64 = *var_qofssub_db0_slot;
        let mut var_qofssub_db1: f64 = *var_qofssub_db1_slot;
        let mut var_qofssub_db10: f64 = *var_qofssub_db10_slot;
        let mut var_qofssub_db11: f64 = *var_qofssub_db11_slot;
        let mut var_qofssub_db12: f64 = *var_qofssub_db12_slot;
        let mut var_qofssub_db13: f64 = *var_qofssub_db13_slot;
        let mut var_qofssub_db14: f64 = *var_qofssub_db14_slot;
        let mut var_qofssub_db15: f64 = *var_qofssub_db15_slot;
        let mut var_qofssub_db16: f64 = *var_qofssub_db16_slot;
        let mut var_qofssub_db17: f64 = *var_qofssub_db17_slot;
        let mut var_qofssub_db18: f64 = *var_qofssub_db18_slot;
        let mut var_qofssub_db19: f64 = *var_qofssub_db19_slot;
        let mut var_qofssub_db2: f64 = *var_qofssub_db2_slot;
        let mut var_qofssub_db20: f64 = *var_qofssub_db20_slot;
        let mut var_qofssub_db21: f64 = *var_qofssub_db21_slot;
        let mut var_qofssub_db22: f64 = *var_qofssub_db22_slot;
        let mut var_qofssub_db23: f64 = *var_qofssub_db23_slot;
        let mut var_qofssub_db24: f64 = *var_qofssub_db24_slot;
        let mut var_qofssub_db25: f64 = *var_qofssub_db25_slot;
        let mut var_qofssub_db26: f64 = *var_qofssub_db26_slot;
        let mut var_qofssub_db27: f64 = *var_qofssub_db27_slot;
        let mut var_qofssub_db28: f64 = *var_qofssub_db28_slot;
        let mut var_qofssub_db29: f64 = *var_qofssub_db29_slot;
        let mut var_qofssub_db3: f64 = *var_qofssub_db3_slot;
        let mut var_qofssub_db30: f64 = *var_qofssub_db30_slot;
        let mut var_qofssub_db31: f64 = *var_qofssub_db31_slot;
        let mut var_qofssub_db32: f64 = *var_qofssub_db32_slot;
        let mut var_qofssub_db33: f64 = *var_qofssub_db33_slot;
        let mut var_qofssub_db34: f64 = *var_qofssub_db34_slot;
        let mut var_qofssub_db35: f64 = *var_qofssub_db35_slot;
        let mut var_qofssub_db4: f64 = *var_qofssub_db4_slot;
        let mut var_qofssub_db5: f64 = *var_qofssub_db5_slot;
        let mut var_qofssub_db6: f64 = *var_qofssub_db6_slot;
        let mut var_qofssub_db7: f64 = *var_qofssub_db7_slot;
        let mut var_qofssub_db8: f64 = *var_qofssub_db8_slot;
        let mut var_qofssub_db9: f64 = *var_qofssub_db9_slot;
        let mut var_qofssub_dn0: f64 = *var_qofssub_dn0_slot;
        let mut var_qofssub_dn1: f64 = *var_qofssub_dn1_slot;
        let mut var_qofssub_dn10: f64 = *var_qofssub_dn10_slot;
        let mut var_qofssub_dn11: f64 = *var_qofssub_dn11_slot;
        let mut var_qofssub_dn12: f64 = *var_qofssub_dn12_slot;
        let mut var_qofssub_dn13: f64 = *var_qofssub_dn13_slot;
        let mut var_qofssub_dn14: f64 = *var_qofssub_dn14_slot;
        let mut var_qofssub_dn15: f64 = *var_qofssub_dn15_slot;
        let mut var_qofssub_dn16: f64 = *var_qofssub_dn16_slot;
        let mut var_qofssub_dn17: f64 = *var_qofssub_dn17_slot;
        let mut var_qofssub_dn18: f64 = *var_qofssub_dn18_slot;
        let mut var_qofssub_dn19: f64 = *var_qofssub_dn19_slot;
        let mut var_qofssub_dn2: f64 = *var_qofssub_dn2_slot;
        let mut var_qofssub_dn20: f64 = *var_qofssub_dn20_slot;
        let mut var_qofssub_dn21: f64 = *var_qofssub_dn21_slot;
        let mut var_qofssub_dn22: f64 = *var_qofssub_dn22_slot;
        let mut var_qofssub_dn23: f64 = *var_qofssub_dn23_slot;
        let mut var_qofssub_dn24: f64 = *var_qofssub_dn24_slot;
        let mut var_qofssub_dn25: f64 = *var_qofssub_dn25_slot;
        let mut var_qofssub_dn26: f64 = *var_qofssub_dn26_slot;
        let mut var_qofssub_dn27: f64 = *var_qofssub_dn27_slot;
        let mut var_qofssub_dn28: f64 = *var_qofssub_dn28_slot;
        let mut var_qofssub_dn29: f64 = *var_qofssub_dn29_slot;
        let mut var_qofssub_dn3: f64 = *var_qofssub_dn3_slot;
        let mut var_qofssub_dn4: f64 = *var_qofssub_dn4_slot;
        let mut var_qofssub_dn5: f64 = *var_qofssub_dn5_slot;
        let mut var_qofssub_dn6: f64 = *var_qofssub_dn6_slot;
        let mut var_qofssub_dn7: f64 = *var_qofssub_dn7_slot;
        let mut var_qofssub_dn8: f64 = *var_qofssub_dn8_slot;
        let mut var_qofssub_dn9: f64 = *var_qofssub_dn9_slot;

        let (assign46290_e45028, assign46290_e45028_d_n0, assign46290_e45028_d_n1, assign46290_e45028_d_n2, assign46290_e45028_d_n3, assign46290_e45028_d_n4, assign46290_e45028_d_n5, assign46290_e45028_d_n6, assign46290_e45028_d_n7, assign46290_e45028_d_n8, assign46290_e45028_d_n9, assign46290_e45028_d_n10, assign46290_e45028_d_n11, assign46290_e45028_d_n12, assign46290_e45028_d_n13, assign46290_e45028_d_n14, assign46290_e45028_d_n15, assign46290_e45028_d_n16, assign46290_e45028_d_n17, assign46290_e45028_d_n18, assign46290_e45028_d_n19, assign46290_e45028_d_n20, assign46290_e45028_d_n21, assign46290_e45028_d_n22, assign46290_e45028_d_n23, assign46290_e45028_d_n24, assign46290_e45028_d_n25, assign46290_e45028_d_n26, assign46290_e45028_d_n27, assign46290_e45028_d_n28, assign46290_e45028_d_n29, assign46290_e45028_d_b0, assign46290_e45028_d_b1, assign46290_e45028_d_b2, assign46290_e45028_d_b3, assign46290_e45028_d_b4, assign46290_e45028_d_b5, assign46290_e45028_d_b6, assign46290_e45028_d_b7, assign46290_e45028_d_b8, assign46290_e45028_d_b9, assign46290_e45028_d_b10, assign46290_e45028_d_b11, assign46290_e45028_d_b12, assign46290_e45028_d_b13, assign46290_e45028_d_b14, assign46290_e45028_d_b15, assign46290_e45028_d_b16, assign46290_e45028_d_b17, assign46290_e45028_d_b18, assign46290_e45028_d_b19, assign46290_e45028_d_b20, assign46290_e45028_d_b21, assign46290_e45028_d_b22, assign46290_e45028_d_b23, assign46290_e45028_d_b24, assign46290_e45028_d_b25, assign46290_e45028_d_b26, assign46290_e45028_d_b27, assign46290_e45028_d_b28, assign46290_e45028_d_b29, assign46290_e45028_d_b30, assign46290_e45028_d_b31, assign46290_e45028_d_b32, assign46290_e45028_d_b33, assign46290_e45028_d_b34, assign46290_e45028_d_b35,) = {
    if ((var_guard501 == 0.0) && (var_guard502 != 0.0)) {
        let assign46290_e45011: f64 = (p.p0 * p.p2);
        let assign46290_e45014: f64 = (var_cofdsmt0 * (nv2 - nv0));
        let assign46290_e45017: f64 = (var_cofdsmt * p.p28);
        let assign46290_e45020: f64 = ((nv2 - nv0) - p.p27);
        let assign46290_e45022: f64 = (assign46290_e45020 / p.p28);
        let assign46290_e45023: f64 = (assign46290_e45022).exp();
        let assign46290_e45024: f64 = (assign46290_e45017 * assign46290_e45023);
        let assign46290_e45025: f64 = (assign46290_e45014 + assign46290_e45024);
        let assign46290_e45026: f64 = (assign46290_e45011 * assign46290_e45025);
        (assign46290_e45026, (assign46290_e45011 * (((var_cofdsmt0_dn0 * (nv2 - nv0)) + (-var_cofdsmt0)) + (((var_cofdsmt_dn0 * p.p28) * assign46290_e45023) + (assign46290_e45017 * (assign46290_e45023 * (-1.0 / p.p28)))))), (assign46290_e45011 * ((var_cofdsmt0_dn1 * (nv2 - nv0)) + ((var_cofdsmt_dn1 * p.p28) * assign46290_e45023))), (assign46290_e45011 * (((var_cofdsmt0_dn2 * (nv2 - nv0)) + var_cofdsmt0) + (((var_cofdsmt_dn2 * p.p28) * assign46290_e45023) + (assign46290_e45017 * (assign46290_e45023 * (1.0 / p.p28)))))), (assign46290_e45011 * ((var_cofdsmt0_dn3 * (nv2 - nv0)) + ((var_cofdsmt_dn3 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn4 * (nv2 - nv0)) + ((var_cofdsmt_dn4 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn5 * (nv2 - nv0)) + ((var_cofdsmt_dn5 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn6 * (nv2 - nv0)) + ((var_cofdsmt_dn6 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn7 * (nv2 - nv0)) + ((var_cofdsmt_dn7 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn8 * (nv2 - nv0)) + ((var_cofdsmt_dn8 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn9 * (nv2 - nv0)) + ((var_cofdsmt_dn9 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn10 * (nv2 - nv0)) + ((var_cofdsmt_dn10 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn11 * (nv2 - nv0)) + ((var_cofdsmt_dn11 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn12 * (nv2 - nv0)) + ((var_cofdsmt_dn12 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn13 * (nv2 - nv0)) + ((var_cofdsmt_dn13 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn14 * (nv2 - nv0)) + ((var_cofdsmt_dn14 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn15 * (nv2 - nv0)) + ((var_cofdsmt_dn15 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn16 * (nv2 - nv0)) + ((var_cofdsmt_dn16 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn17 * (nv2 - nv0)) + ((var_cofdsmt_dn17 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn18 * (nv2 - nv0)) + ((var_cofdsmt_dn18 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn19 * (nv2 - nv0)) + ((var_cofdsmt_dn19 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn20 * (nv2 - nv0)) + ((var_cofdsmt_dn20 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn21 * (nv2 - nv0)) + ((var_cofdsmt_dn21 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn22 * (nv2 - nv0)) + ((var_cofdsmt_dn22 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn23 * (nv2 - nv0)) + ((var_cofdsmt_dn23 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn24 * (nv2 - nv0)) + ((var_cofdsmt_dn24 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn25 * (nv2 - nv0)) + ((var_cofdsmt_dn25 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn26 * (nv2 - nv0)) + ((var_cofdsmt_dn26 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn27 * (nv2 - nv0)) + ((var_cofdsmt_dn27 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn28 * (nv2 - nv0)) + ((var_cofdsmt_dn28 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_dn29 * (nv2 - nv0)) + ((var_cofdsmt_dn29 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db0 * (nv2 - nv0)) + ((var_cofdsmt_db0 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db1 * (nv2 - nv0)) + ((var_cofdsmt_db1 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db2 * (nv2 - nv0)) + ((var_cofdsmt_db2 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db3 * (nv2 - nv0)) + ((var_cofdsmt_db3 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db4 * (nv2 - nv0)) + ((var_cofdsmt_db4 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db5 * (nv2 - nv0)) + ((var_cofdsmt_db5 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db6 * (nv2 - nv0)) + ((var_cofdsmt_db6 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db7 * (nv2 - nv0)) + ((var_cofdsmt_db7 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db8 * (nv2 - nv0)) + ((var_cofdsmt_db8 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db9 * (nv2 - nv0)) + ((var_cofdsmt_db9 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db10 * (nv2 - nv0)) + ((var_cofdsmt_db10 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db11 * (nv2 - nv0)) + ((var_cofdsmt_db11 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db12 * (nv2 - nv0)) + ((var_cofdsmt_db12 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db13 * (nv2 - nv0)) + ((var_cofdsmt_db13 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db14 * (nv2 - nv0)) + ((var_cofdsmt_db14 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db15 * (nv2 - nv0)) + ((var_cofdsmt_db15 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db16 * (nv2 - nv0)) + ((var_cofdsmt_db16 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db17 * (nv2 - nv0)) + ((var_cofdsmt_db17 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db18 * (nv2 - nv0)) + ((var_cofdsmt_db18 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db19 * (nv2 - nv0)) + ((var_cofdsmt_db19 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db20 * (nv2 - nv0)) + ((var_cofdsmt_db20 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db21 * (nv2 - nv0)) + ((var_cofdsmt_db21 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db22 * (nv2 - nv0)) + ((var_cofdsmt_db22 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db23 * (nv2 - nv0)) + ((var_cofdsmt_db23 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db24 * (nv2 - nv0)) + ((var_cofdsmt_db24 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db25 * (nv2 - nv0)) + ((var_cofdsmt_db25 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db26 * (nv2 - nv0)) + ((var_cofdsmt_db26 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db27 * (nv2 - nv0)) + ((var_cofdsmt_db27 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db28 * (nv2 - nv0)) + ((var_cofdsmt_db28 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db29 * (nv2 - nv0)) + ((var_cofdsmt_db29 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db30 * (nv2 - nv0)) + ((var_cofdsmt_db30 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db31 * (nv2 - nv0)) + ((var_cofdsmt_db31 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db32 * (nv2 - nv0)) + ((var_cofdsmt_db32 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db33 * (nv2 - nv0)) + ((var_cofdsmt_db33 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db34 * (nv2 - nv0)) + ((var_cofdsmt_db34 * p.p28) * assign46290_e45023))), (assign46290_e45011 * ((var_cofdsmt0_db35 * (nv2 - nv0)) + ((var_cofdsmt_db35 * p.p28) * assign46290_e45023))),)
    } else {
        (var_qofds, var_qofds_dn0, var_qofds_dn1, var_qofds_dn2, var_qofds_dn3, var_qofds_dn4, var_qofds_dn5, var_qofds_dn6, var_qofds_dn7, var_qofds_dn8, var_qofds_dn9, var_qofds_dn10, var_qofds_dn11, var_qofds_dn12, var_qofds_dn13, var_qofds_dn14, var_qofds_dn15, var_qofds_dn16, var_qofds_dn17, var_qofds_dn18, var_qofds_dn19, var_qofds_dn20, var_qofds_dn21, var_qofds_dn22, var_qofds_dn23, var_qofds_dn24, var_qofds_dn25, var_qofds_dn26, var_qofds_dn27, var_qofds_dn28, var_qofds_dn29, var_qofds_db0, var_qofds_db1, var_qofds_db2, var_qofds_db3, var_qofds_db4, var_qofds_db5, var_qofds_db6, var_qofds_db7, var_qofds_db8, var_qofds_db9, var_qofds_db10, var_qofds_db11, var_qofds_db12, var_qofds_db13, var_qofds_db14, var_qofds_db15, var_qofds_db16, var_qofds_db17, var_qofds_db18, var_qofds_db19, var_qofds_db20, var_qofds_db21, var_qofds_db22, var_qofds_db23, var_qofds_db24, var_qofds_db25, var_qofds_db26, var_qofds_db27, var_qofds_db28, var_qofds_db29, var_qofds_db30, var_qofds_db31, var_qofds_db32, var_qofds_db33, var_qofds_db34, var_qofds_db35,)
    }
};
        var_qofds = assign46290_e45028;
        var_qofds_dn0 = assign46290_e45028_d_n0;
        var_qofds_dn1 = assign46290_e45028_d_n1;
        var_qofds_dn2 = assign46290_e45028_d_n2;
        var_qofds_dn3 = assign46290_e45028_d_n3;
        var_qofds_dn4 = assign46290_e45028_d_n4;
        var_qofds_dn5 = assign46290_e45028_d_n5;
        var_qofds_dn6 = assign46290_e45028_d_n6;
        var_qofds_dn7 = assign46290_e45028_d_n7;
        var_qofds_dn8 = assign46290_e45028_d_n8;
        var_qofds_dn9 = assign46290_e45028_d_n9;
        var_qofds_dn10 = assign46290_e45028_d_n10;
        var_qofds_dn11 = assign46290_e45028_d_n11;
        var_qofds_dn12 = assign46290_e45028_d_n12;
        var_qofds_dn13 = assign46290_e45028_d_n13;
        var_qofds_dn14 = assign46290_e45028_d_n14;
        var_qofds_dn15 = assign46290_e45028_d_n15;
        var_qofds_dn16 = assign46290_e45028_d_n16;
        var_qofds_dn17 = assign46290_e45028_d_n17;
        var_qofds_dn18 = assign46290_e45028_d_n18;
        var_qofds_dn19 = assign46290_e45028_d_n19;
        var_qofds_dn20 = assign46290_e45028_d_n20;
        var_qofds_dn21 = assign46290_e45028_d_n21;
        var_qofds_dn22 = assign46290_e45028_d_n22;
        var_qofds_dn23 = assign46290_e45028_d_n23;
        var_qofds_dn24 = assign46290_e45028_d_n24;
        var_qofds_dn25 = assign46290_e45028_d_n25;
        var_qofds_dn26 = assign46290_e45028_d_n26;
        var_qofds_dn27 = assign46290_e45028_d_n27;
        var_qofds_dn28 = assign46290_e45028_d_n28;
        var_qofds_dn29 = assign46290_e45028_d_n29;
        var_qofds_db0 = assign46290_e45028_d_b0;
        var_qofds_db1 = assign46290_e45028_d_b1;
        var_qofds_db2 = assign46290_e45028_d_b2;
        var_qofds_db3 = assign46290_e45028_d_b3;
        var_qofds_db4 = assign46290_e45028_d_b4;
        var_qofds_db5 = assign46290_e45028_d_b5;
        var_qofds_db6 = assign46290_e45028_d_b6;
        var_qofds_db7 = assign46290_e45028_d_b7;
        var_qofds_db8 = assign46290_e45028_d_b8;
        var_qofds_db9 = assign46290_e45028_d_b9;
        var_qofds_db10 = assign46290_e45028_d_b10;
        var_qofds_db11 = assign46290_e45028_d_b11;
        var_qofds_db12 = assign46290_e45028_d_b12;
        var_qofds_db13 = assign46290_e45028_d_b13;
        var_qofds_db14 = assign46290_e45028_d_b14;
        var_qofds_db15 = assign46290_e45028_d_b15;
        var_qofds_db16 = assign46290_e45028_d_b16;
        var_qofds_db17 = assign46290_e45028_d_b17;
        var_qofds_db18 = assign46290_e45028_d_b18;
        var_qofds_db19 = assign46290_e45028_d_b19;
        var_qofds_db20 = assign46290_e45028_d_b20;
        var_qofds_db21 = assign46290_e45028_d_b21;
        var_qofds_db22 = assign46290_e45028_d_b22;
        var_qofds_db23 = assign46290_e45028_d_b23;
        var_qofds_db24 = assign46290_e45028_d_b24;
        var_qofds_db25 = assign46290_e45028_d_b25;
        var_qofds_db26 = assign46290_e45028_d_b26;
        var_qofds_db27 = assign46290_e45028_d_b27;
        var_qofds_db28 = assign46290_e45028_d_b28;
        var_qofds_db29 = assign46290_e45028_d_b29;
        var_qofds_db30 = assign46290_e45028_d_b30;
        var_qofds_db31 = assign46290_e45028_d_b31;
        var_qofds_db32 = assign46290_e45028_d_b32;
        var_qofds_db33 = assign46290_e45028_d_b33;
        var_qofds_db34 = assign46290_e45028_d_b34;
        var_qofds_db35 = assign46290_e45028_d_b35;

        let (assign46300_e45056, assign46300_e45056_d_n0, assign46300_e45056_d_n1, assign46300_e45056_d_n2, assign46300_e45056_d_n3, assign46300_e45056_d_n4, assign46300_e45056_d_n5, assign46300_e45056_d_n6, assign46300_e45056_d_n7, assign46300_e45056_d_n8, assign46300_e45056_d_n9, assign46300_e45056_d_n10, assign46300_e45056_d_n11, assign46300_e45056_d_n12, assign46300_e45056_d_n13, assign46300_e45056_d_n14, assign46300_e45056_d_n15, assign46300_e45056_d_n16, assign46300_e45056_d_n17, assign46300_e45056_d_n18, assign46300_e45056_d_n19, assign46300_e45056_d_n20, assign46300_e45056_d_n21, assign46300_e45056_d_n22, assign46300_e45056_d_n23, assign46300_e45056_d_n24, assign46300_e45056_d_n25, assign46300_e45056_d_n26, assign46300_e45056_d_n27, assign46300_e45056_d_n28, assign46300_e45056_d_n29, assign46300_e45056_d_b0, assign46300_e45056_d_b1, assign46300_e45056_d_b2, assign46300_e45056_d_b3, assign46300_e45056_d_b4, assign46300_e45056_d_b5, assign46300_e45056_d_b6, assign46300_e45056_d_b7, assign46300_e45056_d_b8, assign46300_e45056_d_b9, assign46300_e45056_d_b10, assign46300_e45056_d_b11, assign46300_e45056_d_b12, assign46300_e45056_d_b13, assign46300_e45056_d_b14, assign46300_e45056_d_b15, assign46300_e45056_d_b16, assign46300_e45056_d_b17, assign46300_e45056_d_b18, assign46300_e45056_d_b19, assign46300_e45056_d_b20, assign46300_e45056_d_b21, assign46300_e45056_d_b22, assign46300_e45056_d_b23, assign46300_e45056_d_b24, assign46300_e45056_d_b25, assign46300_e45056_d_b26, assign46300_e45056_d_b27, assign46300_e45056_d_b28, assign46300_e45056_d_b29, assign46300_e45056_d_b30, assign46300_e45056_d_b31, assign46300_e45056_d_b32, assign46300_e45056_d_b33, assign46300_e45056_d_b34, assign46300_e45056_d_b35,) = {
    if ((var_guard501 == 0.0) && (var_guard502 == 0.0)) {
        let assign46300_e45036: f64 = (p.p0 * p.p2);
        let assign46300_e45039: f64 = (var_cofdsmt0 * (nv2 - nv0));
        let assign46300_e45042: f64 = (var_cofdsmt * p.p28);
        let assign46300_e45046: f64 = ((nv2 - nv0) - p.p27);
        let assign46300_e45048: f64 = (assign46300_e45046 / p.p28);
        let assign46300_e45049: f64 = (assign46300_e45048).exp();
        let assign46300_e45050: f64 = (1.0 + assign46300_e45049);
        let assign46300_e45051: f64 = (assign46300_e45050).ln();
        let assign46300_e45052: f64 = (assign46300_e45042 * assign46300_e45051);
        let assign46300_e45053: f64 = (assign46300_e45039 + assign46300_e45052);
        let assign46300_e45054: f64 = (assign46300_e45036 * assign46300_e45053);
        (assign46300_e45054, (assign46300_e45036 * (((var_cofdsmt0_dn0 * (nv2 - nv0)) + (-var_cofdsmt0)) + (((var_cofdsmt_dn0 * p.p28) * assign46300_e45051) + (assign46300_e45042 * ((assign46300_e45049 * (-1.0 / p.p28)) / assign46300_e45050))))), (assign46300_e45036 * ((var_cofdsmt0_dn1 * (nv2 - nv0)) + ((var_cofdsmt_dn1 * p.p28) * assign46300_e45051))), (assign46300_e45036 * (((var_cofdsmt0_dn2 * (nv2 - nv0)) + var_cofdsmt0) + (((var_cofdsmt_dn2 * p.p28) * assign46300_e45051) + (assign46300_e45042 * ((assign46300_e45049 * (1.0 / p.p28)) / assign46300_e45050))))), (assign46300_e45036 * ((var_cofdsmt0_dn3 * (nv2 - nv0)) + ((var_cofdsmt_dn3 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn4 * (nv2 - nv0)) + ((var_cofdsmt_dn4 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn5 * (nv2 - nv0)) + ((var_cofdsmt_dn5 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn6 * (nv2 - nv0)) + ((var_cofdsmt_dn6 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn7 * (nv2 - nv0)) + ((var_cofdsmt_dn7 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn8 * (nv2 - nv0)) + ((var_cofdsmt_dn8 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn9 * (nv2 - nv0)) + ((var_cofdsmt_dn9 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn10 * (nv2 - nv0)) + ((var_cofdsmt_dn10 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn11 * (nv2 - nv0)) + ((var_cofdsmt_dn11 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn12 * (nv2 - nv0)) + ((var_cofdsmt_dn12 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn13 * (nv2 - nv0)) + ((var_cofdsmt_dn13 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn14 * (nv2 - nv0)) + ((var_cofdsmt_dn14 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn15 * (nv2 - nv0)) + ((var_cofdsmt_dn15 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn16 * (nv2 - nv0)) + ((var_cofdsmt_dn16 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn17 * (nv2 - nv0)) + ((var_cofdsmt_dn17 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn18 * (nv2 - nv0)) + ((var_cofdsmt_dn18 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn19 * (nv2 - nv0)) + ((var_cofdsmt_dn19 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn20 * (nv2 - nv0)) + ((var_cofdsmt_dn20 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn21 * (nv2 - nv0)) + ((var_cofdsmt_dn21 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn22 * (nv2 - nv0)) + ((var_cofdsmt_dn22 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn23 * (nv2 - nv0)) + ((var_cofdsmt_dn23 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn24 * (nv2 - nv0)) + ((var_cofdsmt_dn24 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn25 * (nv2 - nv0)) + ((var_cofdsmt_dn25 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn26 * (nv2 - nv0)) + ((var_cofdsmt_dn26 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn27 * (nv2 - nv0)) + ((var_cofdsmt_dn27 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn28 * (nv2 - nv0)) + ((var_cofdsmt_dn28 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_dn29 * (nv2 - nv0)) + ((var_cofdsmt_dn29 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db0 * (nv2 - nv0)) + ((var_cofdsmt_db0 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db1 * (nv2 - nv0)) + ((var_cofdsmt_db1 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db2 * (nv2 - nv0)) + ((var_cofdsmt_db2 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db3 * (nv2 - nv0)) + ((var_cofdsmt_db3 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db4 * (nv2 - nv0)) + ((var_cofdsmt_db4 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db5 * (nv2 - nv0)) + ((var_cofdsmt_db5 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db6 * (nv2 - nv0)) + ((var_cofdsmt_db6 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db7 * (nv2 - nv0)) + ((var_cofdsmt_db7 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db8 * (nv2 - nv0)) + ((var_cofdsmt_db8 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db9 * (nv2 - nv0)) + ((var_cofdsmt_db9 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db10 * (nv2 - nv0)) + ((var_cofdsmt_db10 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db11 * (nv2 - nv0)) + ((var_cofdsmt_db11 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db12 * (nv2 - nv0)) + ((var_cofdsmt_db12 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db13 * (nv2 - nv0)) + ((var_cofdsmt_db13 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db14 * (nv2 - nv0)) + ((var_cofdsmt_db14 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db15 * (nv2 - nv0)) + ((var_cofdsmt_db15 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db16 * (nv2 - nv0)) + ((var_cofdsmt_db16 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db17 * (nv2 - nv0)) + ((var_cofdsmt_db17 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db18 * (nv2 - nv0)) + ((var_cofdsmt_db18 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db19 * (nv2 - nv0)) + ((var_cofdsmt_db19 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db20 * (nv2 - nv0)) + ((var_cofdsmt_db20 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db21 * (nv2 - nv0)) + ((var_cofdsmt_db21 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db22 * (nv2 - nv0)) + ((var_cofdsmt_db22 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db23 * (nv2 - nv0)) + ((var_cofdsmt_db23 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db24 * (nv2 - nv0)) + ((var_cofdsmt_db24 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db25 * (nv2 - nv0)) + ((var_cofdsmt_db25 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db26 * (nv2 - nv0)) + ((var_cofdsmt_db26 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db27 * (nv2 - nv0)) + ((var_cofdsmt_db27 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db28 * (nv2 - nv0)) + ((var_cofdsmt_db28 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db29 * (nv2 - nv0)) + ((var_cofdsmt_db29 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db30 * (nv2 - nv0)) + ((var_cofdsmt_db30 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db31 * (nv2 - nv0)) + ((var_cofdsmt_db31 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db32 * (nv2 - nv0)) + ((var_cofdsmt_db32 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db33 * (nv2 - nv0)) + ((var_cofdsmt_db33 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db34 * (nv2 - nv0)) + ((var_cofdsmt_db34 * p.p28) * assign46300_e45051))), (assign46300_e45036 * ((var_cofdsmt0_db35 * (nv2 - nv0)) + ((var_cofdsmt_db35 * p.p28) * assign46300_e45051))),)
    } else {
        (var_qofds, var_qofds_dn0, var_qofds_dn1, var_qofds_dn2, var_qofds_dn3, var_qofds_dn4, var_qofds_dn5, var_qofds_dn6, var_qofds_dn7, var_qofds_dn8, var_qofds_dn9, var_qofds_dn10, var_qofds_dn11, var_qofds_dn12, var_qofds_dn13, var_qofds_dn14, var_qofds_dn15, var_qofds_dn16, var_qofds_dn17, var_qofds_dn18, var_qofds_dn19, var_qofds_dn20, var_qofds_dn21, var_qofds_dn22, var_qofds_dn23, var_qofds_dn24, var_qofds_dn25, var_qofds_dn26, var_qofds_dn27, var_qofds_dn28, var_qofds_dn29, var_qofds_db0, var_qofds_db1, var_qofds_db2, var_qofds_db3, var_qofds_db4, var_qofds_db5, var_qofds_db6, var_qofds_db7, var_qofds_db8, var_qofds_db9, var_qofds_db10, var_qofds_db11, var_qofds_db12, var_qofds_db13, var_qofds_db14, var_qofds_db15, var_qofds_db16, var_qofds_db17, var_qofds_db18, var_qofds_db19, var_qofds_db20, var_qofds_db21, var_qofds_db22, var_qofds_db23, var_qofds_db24, var_qofds_db25, var_qofds_db26, var_qofds_db27, var_qofds_db28, var_qofds_db29, var_qofds_db30, var_qofds_db31, var_qofds_db32, var_qofds_db33, var_qofds_db34, var_qofds_db35,)
    }
};
        var_qofds = assign46300_e45056;
        var_qofds_dn0 = assign46300_e45056_d_n0;
        var_qofds_dn1 = assign46300_e45056_d_n1;
        var_qofds_dn2 = assign46300_e45056_d_n2;
        var_qofds_dn3 = assign46300_e45056_d_n3;
        var_qofds_dn4 = assign46300_e45056_d_n4;
        var_qofds_dn5 = assign46300_e45056_d_n5;
        var_qofds_dn6 = assign46300_e45056_d_n6;
        var_qofds_dn7 = assign46300_e45056_d_n7;
        var_qofds_dn8 = assign46300_e45056_d_n8;
        var_qofds_dn9 = assign46300_e45056_d_n9;
        var_qofds_dn10 = assign46300_e45056_d_n10;
        var_qofds_dn11 = assign46300_e45056_d_n11;
        var_qofds_dn12 = assign46300_e45056_d_n12;
        var_qofds_dn13 = assign46300_e45056_d_n13;
        var_qofds_dn14 = assign46300_e45056_d_n14;
        var_qofds_dn15 = assign46300_e45056_d_n15;
        var_qofds_dn16 = assign46300_e45056_d_n16;
        var_qofds_dn17 = assign46300_e45056_d_n17;
        var_qofds_dn18 = assign46300_e45056_d_n18;
        var_qofds_dn19 = assign46300_e45056_d_n19;
        var_qofds_dn20 = assign46300_e45056_d_n20;
        var_qofds_dn21 = assign46300_e45056_d_n21;
        var_qofds_dn22 = assign46300_e45056_d_n22;
        var_qofds_dn23 = assign46300_e45056_d_n23;
        var_qofds_dn24 = assign46300_e45056_d_n24;
        var_qofds_dn25 = assign46300_e45056_d_n25;
        var_qofds_dn26 = assign46300_e45056_d_n26;
        var_qofds_dn27 = assign46300_e45056_d_n27;
        var_qofds_dn28 = assign46300_e45056_d_n28;
        var_qofds_dn29 = assign46300_e45056_d_n29;
        var_qofds_db0 = assign46300_e45056_d_b0;
        var_qofds_db1 = assign46300_e45056_d_b1;
        var_qofds_db2 = assign46300_e45056_d_b2;
        var_qofds_db3 = assign46300_e45056_d_b3;
        var_qofds_db4 = assign46300_e45056_d_b4;
        var_qofds_db5 = assign46300_e45056_d_b5;
        var_qofds_db6 = assign46300_e45056_d_b6;
        var_qofds_db7 = assign46300_e45056_d_b7;
        var_qofds_db8 = assign46300_e45056_d_b8;
        var_qofds_db9 = assign46300_e45056_d_b9;
        var_qofds_db10 = assign46300_e45056_d_b10;
        var_qofds_db11 = assign46300_e45056_d_b11;
        var_qofds_db12 = assign46300_e45056_d_b12;
        var_qofds_db13 = assign46300_e45056_d_b13;
        var_qofds_db14 = assign46300_e45056_d_b14;
        var_qofds_db15 = assign46300_e45056_d_b15;
        var_qofds_db16 = assign46300_e45056_d_b16;
        var_qofds_db17 = assign46300_e45056_d_b17;
        var_qofds_db18 = assign46300_e45056_d_b18;
        var_qofds_db19 = assign46300_e45056_d_b19;
        var_qofds_db20 = assign46300_e45056_d_b20;
        var_qofds_db21 = assign46300_e45056_d_b21;
        var_qofds_db22 = assign46300_e45056_d_b22;
        var_qofds_db23 = assign46300_e45056_d_b23;
        var_qofds_db24 = assign46300_e45056_d_b24;
        var_qofds_db25 = assign46300_e45056_d_b25;
        var_qofds_db26 = assign46300_e45056_d_b26;
        var_qofds_db27 = assign46300_e45056_d_b27;
        var_qofds_db28 = assign46300_e45056_d_b28;
        var_qofds_db29 = assign46300_e45056_d_b29;
        var_qofds_db30 = assign46300_e45056_d_b30;
        var_qofds_db31 = assign46300_e45056_d_b31;
        var_qofds_db32 = assign46300_e45056_d_b32;
        var_qofds_db33 = assign46300_e45056_d_b33;
        var_qofds_db34 = assign46300_e45056_d_b34;
        var_qofds_db35 = assign46300_e45056_d_b35;

        let assign46310_e45059: f64 = ((nv3 - nv2) - p.p27);
        let assign46310_e45061: f64 = (assign46310_e45059 / p.p28);
        let assign46310_e45063: f64 = if assign46310_e45061 > 50.0 { 1.0 } else { 0.0 };
        var_guard503 = assign46310_e45063;

        let (assign46320_e45079, assign46320_e45079_d_n0, assign46320_e45079_d_n1, assign46320_e45079_d_n2, assign46320_e45079_d_n3, assign46320_e45079_d_n4, assign46320_e45079_d_n5, assign46320_e45079_d_n6, assign46320_e45079_d_n7, assign46320_e45079_d_n8, assign46320_e45079_d_n9, assign46320_e45079_d_n10, assign46320_e45079_d_n11, assign46320_e45079_d_n12, assign46320_e45079_d_n13, assign46320_e45079_d_n14, assign46320_e45079_d_n15, assign46320_e45079_d_n16, assign46320_e45079_d_n17, assign46320_e45079_d_n18, assign46320_e45079_d_n19, assign46320_e45079_d_n20, assign46320_e45079_d_n21, assign46320_e45079_d_n22, assign46320_e45079_d_n23, assign46320_e45079_d_n24, assign46320_e45079_d_n25, assign46320_e45079_d_n26, assign46320_e45079_d_n27, assign46320_e45079_d_n28, assign46320_e45079_d_n29, assign46320_e45079_d_b0, assign46320_e45079_d_b1, assign46320_e45079_d_b2, assign46320_e45079_d_b3, assign46320_e45079_d_b4, assign46320_e45079_d_b5, assign46320_e45079_d_b6, assign46320_e45079_d_b7, assign46320_e45079_d_b8, assign46320_e45079_d_b9, assign46320_e45079_d_b10, assign46320_e45079_d_b11, assign46320_e45079_d_b12, assign46320_e45079_d_b13, assign46320_e45079_d_b14, assign46320_e45079_d_b15, assign46320_e45079_d_b16, assign46320_e45079_d_b17, assign46320_e45079_d_b18, assign46320_e45079_d_b19, assign46320_e45079_d_b20, assign46320_e45079_d_b21, assign46320_e45079_d_b22, assign46320_e45079_d_b23, assign46320_e45079_d_b24, assign46320_e45079_d_b25, assign46320_e45079_d_b26, assign46320_e45079_d_b27, assign46320_e45079_d_b28, assign46320_e45079_d_b29, assign46320_e45079_d_b30, assign46320_e45079_d_b31, assign46320_e45079_d_b32, assign46320_e45079_d_b33, assign46320_e45079_d_b34, assign46320_e45079_d_b35,) = {
    if (var_guard503 != 0.0) {
        let assign46320_e45067: f64 = (p.p0 * p.p2);
        let assign46320_e45070: f64 = (var_cofssubmt0 * (nv3 - nv2));
        let assign46320_e45074: f64 = ((nv3 - nv2) - p.p27);
        let assign46320_e45075: f64 = (var_cofssubmt * assign46320_e45074);
        let assign46320_e45076: f64 = (assign46320_e45070 + assign46320_e45075);
        let assign46320_e45077: f64 = (assign46320_e45067 * assign46320_e45076);
        (assign46320_e45077, (assign46320_e45067 * ((var_cofssubmt0_dn0 * (nv3 - nv2)) + (var_cofssubmt_dn0 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn1 * (nv3 - nv2)) + (var_cofssubmt_dn1 * assign46320_e45074))), (assign46320_e45067 * (((var_cofssubmt0_dn2 * (nv3 - nv2)) + (-var_cofssubmt0)) + ((var_cofssubmt_dn2 * assign46320_e45074) + (-var_cofssubmt)))), (assign46320_e45067 * (((var_cofssubmt0_dn3 * (nv3 - nv2)) + var_cofssubmt0) + ((var_cofssubmt_dn3 * assign46320_e45074) + var_cofssubmt))), (assign46320_e45067 * ((var_cofssubmt0_dn4 * (nv3 - nv2)) + (var_cofssubmt_dn4 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn5 * (nv3 - nv2)) + (var_cofssubmt_dn5 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn6 * (nv3 - nv2)) + (var_cofssubmt_dn6 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn7 * (nv3 - nv2)) + (var_cofssubmt_dn7 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn8 * (nv3 - nv2)) + (var_cofssubmt_dn8 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn9 * (nv3 - nv2)) + (var_cofssubmt_dn9 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn10 * (nv3 - nv2)) + (var_cofssubmt_dn10 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn11 * (nv3 - nv2)) + (var_cofssubmt_dn11 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn12 * (nv3 - nv2)) + (var_cofssubmt_dn12 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn13 * (nv3 - nv2)) + (var_cofssubmt_dn13 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn14 * (nv3 - nv2)) + (var_cofssubmt_dn14 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn15 * (nv3 - nv2)) + (var_cofssubmt_dn15 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn16 * (nv3 - nv2)) + (var_cofssubmt_dn16 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn17 * (nv3 - nv2)) + (var_cofssubmt_dn17 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn18 * (nv3 - nv2)) + (var_cofssubmt_dn18 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn19 * (nv3 - nv2)) + (var_cofssubmt_dn19 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn20 * (nv3 - nv2)) + (var_cofssubmt_dn20 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn21 * (nv3 - nv2)) + (var_cofssubmt_dn21 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn22 * (nv3 - nv2)) + (var_cofssubmt_dn22 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn23 * (nv3 - nv2)) + (var_cofssubmt_dn23 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn24 * (nv3 - nv2)) + (var_cofssubmt_dn24 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn25 * (nv3 - nv2)) + (var_cofssubmt_dn25 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn26 * (nv3 - nv2)) + (var_cofssubmt_dn26 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn27 * (nv3 - nv2)) + (var_cofssubmt_dn27 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn28 * (nv3 - nv2)) + (var_cofssubmt_dn28 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_dn29 * (nv3 - nv2)) + (var_cofssubmt_dn29 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db0 * (nv3 - nv2)) + (var_cofssubmt_db0 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db1 * (nv3 - nv2)) + (var_cofssubmt_db1 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db2 * (nv3 - nv2)) + (var_cofssubmt_db2 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db3 * (nv3 - nv2)) + (var_cofssubmt_db3 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db4 * (nv3 - nv2)) + (var_cofssubmt_db4 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db5 * (nv3 - nv2)) + (var_cofssubmt_db5 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db6 * (nv3 - nv2)) + (var_cofssubmt_db6 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db7 * (nv3 - nv2)) + (var_cofssubmt_db7 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db8 * (nv3 - nv2)) + (var_cofssubmt_db8 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db9 * (nv3 - nv2)) + (var_cofssubmt_db9 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db10 * (nv3 - nv2)) + (var_cofssubmt_db10 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db11 * (nv3 - nv2)) + (var_cofssubmt_db11 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db12 * (nv3 - nv2)) + (var_cofssubmt_db12 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db13 * (nv3 - nv2)) + (var_cofssubmt_db13 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db14 * (nv3 - nv2)) + (var_cofssubmt_db14 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db15 * (nv3 - nv2)) + (var_cofssubmt_db15 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db16 * (nv3 - nv2)) + (var_cofssubmt_db16 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db17 * (nv3 - nv2)) + (var_cofssubmt_db17 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db18 * (nv3 - nv2)) + (var_cofssubmt_db18 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db19 * (nv3 - nv2)) + (var_cofssubmt_db19 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db20 * (nv3 - nv2)) + (var_cofssubmt_db20 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db21 * (nv3 - nv2)) + (var_cofssubmt_db21 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db22 * (nv3 - nv2)) + (var_cofssubmt_db22 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db23 * (nv3 - nv2)) + (var_cofssubmt_db23 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db24 * (nv3 - nv2)) + (var_cofssubmt_db24 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db25 * (nv3 - nv2)) + (var_cofssubmt_db25 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db26 * (nv3 - nv2)) + (var_cofssubmt_db26 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db27 * (nv3 - nv2)) + (var_cofssubmt_db27 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db28 * (nv3 - nv2)) + (var_cofssubmt_db28 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db29 * (nv3 - nv2)) + (var_cofssubmt_db29 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db30 * (nv3 - nv2)) + (var_cofssubmt_db30 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db31 * (nv3 - nv2)) + (var_cofssubmt_db31 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db32 * (nv3 - nv2)) + (var_cofssubmt_db32 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db33 * (nv3 - nv2)) + (var_cofssubmt_db33 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db34 * (nv3 - nv2)) + (var_cofssubmt_db34 * assign46320_e45074))), (assign46320_e45067 * ((var_cofssubmt0_db35 * (nv3 - nv2)) + (var_cofssubmt_db35 * assign46320_e45074))),)
    } else {
        (var_qofssub, var_qofssub_dn0, var_qofssub_dn1, var_qofssub_dn2, var_qofssub_dn3, var_qofssub_dn4, var_qofssub_dn5, var_qofssub_dn6, var_qofssub_dn7, var_qofssub_dn8, var_qofssub_dn9, var_qofssub_dn10, var_qofssub_dn11, var_qofssub_dn12, var_qofssub_dn13, var_qofssub_dn14, var_qofssub_dn15, var_qofssub_dn16, var_qofssub_dn17, var_qofssub_dn18, var_qofssub_dn19, var_qofssub_dn20, var_qofssub_dn21, var_qofssub_dn22, var_qofssub_dn23, var_qofssub_dn24, var_qofssub_dn25, var_qofssub_dn26, var_qofssub_dn27, var_qofssub_dn28, var_qofssub_dn29, var_qofssub_db0, var_qofssub_db1, var_qofssub_db2, var_qofssub_db3, var_qofssub_db4, var_qofssub_db5, var_qofssub_db6, var_qofssub_db7, var_qofssub_db8, var_qofssub_db9, var_qofssub_db10, var_qofssub_db11, var_qofssub_db12, var_qofssub_db13, var_qofssub_db14, var_qofssub_db15, var_qofssub_db16, var_qofssub_db17, var_qofssub_db18, var_qofssub_db19, var_qofssub_db20, var_qofssub_db21, var_qofssub_db22, var_qofssub_db23, var_qofssub_db24, var_qofssub_db25, var_qofssub_db26, var_qofssub_db27, var_qofssub_db28, var_qofssub_db29, var_qofssub_db30, var_qofssub_db31, var_qofssub_db32, var_qofssub_db33, var_qofssub_db34, var_qofssub_db35,)
    }
};
        var_qofssub = assign46320_e45079;
        var_qofssub_dn0 = assign46320_e45079_d_n0;
        var_qofssub_dn1 = assign46320_e45079_d_n1;
        var_qofssub_dn2 = assign46320_e45079_d_n2;
        var_qofssub_dn3 = assign46320_e45079_d_n3;
        var_qofssub_dn4 = assign46320_e45079_d_n4;
        var_qofssub_dn5 = assign46320_e45079_d_n5;
        var_qofssub_dn6 = assign46320_e45079_d_n6;
        var_qofssub_dn7 = assign46320_e45079_d_n7;
        var_qofssub_dn8 = assign46320_e45079_d_n8;
        var_qofssub_dn9 = assign46320_e45079_d_n9;
        var_qofssub_dn10 = assign46320_e45079_d_n10;
        var_qofssub_dn11 = assign46320_e45079_d_n11;
        var_qofssub_dn12 = assign46320_e45079_d_n12;
        var_qofssub_dn13 = assign46320_e45079_d_n13;
        var_qofssub_dn14 = assign46320_e45079_d_n14;
        var_qofssub_dn15 = assign46320_e45079_d_n15;
        var_qofssub_dn16 = assign46320_e45079_d_n16;
        var_qofssub_dn17 = assign46320_e45079_d_n17;
        var_qofssub_dn18 = assign46320_e45079_d_n18;
        var_qofssub_dn19 = assign46320_e45079_d_n19;
        var_qofssub_dn20 = assign46320_e45079_d_n20;
        var_qofssub_dn21 = assign46320_e45079_d_n21;
        var_qofssub_dn22 = assign46320_e45079_d_n22;
        var_qofssub_dn23 = assign46320_e45079_d_n23;
        var_qofssub_dn24 = assign46320_e45079_d_n24;
        var_qofssub_dn25 = assign46320_e45079_d_n25;
        var_qofssub_dn26 = assign46320_e45079_d_n26;
        var_qofssub_dn27 = assign46320_e45079_d_n27;
        var_qofssub_dn28 = assign46320_e45079_d_n28;
        var_qofssub_dn29 = assign46320_e45079_d_n29;
        var_qofssub_db0 = assign46320_e45079_d_b0;
        var_qofssub_db1 = assign46320_e45079_d_b1;
        var_qofssub_db2 = assign46320_e45079_d_b2;
        var_qofssub_db3 = assign46320_e45079_d_b3;
        var_qofssub_db4 = assign46320_e45079_d_b4;
        var_qofssub_db5 = assign46320_e45079_d_b5;
        var_qofssub_db6 = assign46320_e45079_d_b6;
        var_qofssub_db7 = assign46320_e45079_d_b7;
        var_qofssub_db8 = assign46320_e45079_d_b8;
        var_qofssub_db9 = assign46320_e45079_d_b9;
        var_qofssub_db10 = assign46320_e45079_d_b10;
        var_qofssub_db11 = assign46320_e45079_d_b11;
        var_qofssub_db12 = assign46320_e45079_d_b12;
        var_qofssub_db13 = assign46320_e45079_d_b13;
        var_qofssub_db14 = assign46320_e45079_d_b14;
        var_qofssub_db15 = assign46320_e45079_d_b15;
        var_qofssub_db16 = assign46320_e45079_d_b16;
        var_qofssub_db17 = assign46320_e45079_d_b17;
        var_qofssub_db18 = assign46320_e45079_d_b18;
        var_qofssub_db19 = assign46320_e45079_d_b19;
        var_qofssub_db20 = assign46320_e45079_d_b20;
        var_qofssub_db21 = assign46320_e45079_d_b21;
        var_qofssub_db22 = assign46320_e45079_d_b22;
        var_qofssub_db23 = assign46320_e45079_d_b23;
        var_qofssub_db24 = assign46320_e45079_d_b24;
        var_qofssub_db25 = assign46320_e45079_d_b25;
        var_qofssub_db26 = assign46320_e45079_d_b26;
        var_qofssub_db27 = assign46320_e45079_d_b27;
        var_qofssub_db28 = assign46320_e45079_d_b28;
        var_qofssub_db29 = assign46320_e45079_d_b29;
        var_qofssub_db30 = assign46320_e45079_d_b30;
        var_qofssub_db31 = assign46320_e45079_d_b31;
        var_qofssub_db32 = assign46320_e45079_d_b32;
        var_qofssub_db33 = assign46320_e45079_d_b33;
        var_qofssub_db34 = assign46320_e45079_d_b34;
        var_qofssub_db35 = assign46320_e45079_d_b35;

        let assign46330_e45082: f64 = ((nv3 - nv2) - p.p27);
        let assign46330_e45084: f64 = (assign46330_e45082 / p.p28);
        let assign46330_e45086: f64 = (-50.0);
        let assign46330_e45087: f64 = if assign46330_e45084 < assign46330_e45086 { 1.0 } else { 0.0 };
        var_guard504 = assign46330_e45087;

        let (assign46340_e45111, assign46340_e45111_d_n0, assign46340_e45111_d_n1, assign46340_e45111_d_n2, assign46340_e45111_d_n3, assign46340_e45111_d_n4, assign46340_e45111_d_n5, assign46340_e45111_d_n6, assign46340_e45111_d_n7, assign46340_e45111_d_n8, assign46340_e45111_d_n9, assign46340_e45111_d_n10, assign46340_e45111_d_n11, assign46340_e45111_d_n12, assign46340_e45111_d_n13, assign46340_e45111_d_n14, assign46340_e45111_d_n15, assign46340_e45111_d_n16, assign46340_e45111_d_n17, assign46340_e45111_d_n18, assign46340_e45111_d_n19, assign46340_e45111_d_n20, assign46340_e45111_d_n21, assign46340_e45111_d_n22, assign46340_e45111_d_n23, assign46340_e45111_d_n24, assign46340_e45111_d_n25, assign46340_e45111_d_n26, assign46340_e45111_d_n27, assign46340_e45111_d_n28, assign46340_e45111_d_n29, assign46340_e45111_d_b0, assign46340_e45111_d_b1, assign46340_e45111_d_b2, assign46340_e45111_d_b3, assign46340_e45111_d_b4, assign46340_e45111_d_b5, assign46340_e45111_d_b6, assign46340_e45111_d_b7, assign46340_e45111_d_b8, assign46340_e45111_d_b9, assign46340_e45111_d_b10, assign46340_e45111_d_b11, assign46340_e45111_d_b12, assign46340_e45111_d_b13, assign46340_e45111_d_b14, assign46340_e45111_d_b15, assign46340_e45111_d_b16, assign46340_e45111_d_b17, assign46340_e45111_d_b18, assign46340_e45111_d_b19, assign46340_e45111_d_b20, assign46340_e45111_d_b21, assign46340_e45111_d_b22, assign46340_e45111_d_b23, assign46340_e45111_d_b24, assign46340_e45111_d_b25, assign46340_e45111_d_b26, assign46340_e45111_d_b27, assign46340_e45111_d_b28, assign46340_e45111_d_b29, assign46340_e45111_d_b30, assign46340_e45111_d_b31, assign46340_e45111_d_b32, assign46340_e45111_d_b33, assign46340_e45111_d_b34, assign46340_e45111_d_b35,) = {
    if ((var_guard503 == 0.0) && (var_guard504 != 0.0)) {
        let assign46340_e45094: f64 = (p.p0 * p.p2);
        let assign46340_e45097: f64 = (var_cofssubmt0 * (nv3 - nv2));
        let assign46340_e45100: f64 = (var_cofssubmt * p.p28);
        let assign46340_e45103: f64 = ((nv3 - nv2) - p.p27);
        let assign46340_e45105: f64 = (assign46340_e45103 / p.p28);
        let assign46340_e45106: f64 = (assign46340_e45105).exp();
        let assign46340_e45107: f64 = (assign46340_e45100 * assign46340_e45106);
        let assign46340_e45108: f64 = (assign46340_e45097 + assign46340_e45107);
        let assign46340_e45109: f64 = (assign46340_e45094 * assign46340_e45108);
        (assign46340_e45109, (assign46340_e45094 * ((var_cofssubmt0_dn0 * (nv3 - nv2)) + ((var_cofssubmt_dn0 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn1 * (nv3 - nv2)) + ((var_cofssubmt_dn1 * p.p28) * assign46340_e45106))), (assign46340_e45094 * (((var_cofssubmt0_dn2 * (nv3 - nv2)) + (-var_cofssubmt0)) + (((var_cofssubmt_dn2 * p.p28) * assign46340_e45106) + (assign46340_e45100 * (assign46340_e45106 * (-1.0 / p.p28)))))), (assign46340_e45094 * (((var_cofssubmt0_dn3 * (nv3 - nv2)) + var_cofssubmt0) + (((var_cofssubmt_dn3 * p.p28) * assign46340_e45106) + (assign46340_e45100 * (assign46340_e45106 * (1.0 / p.p28)))))), (assign46340_e45094 * ((var_cofssubmt0_dn4 * (nv3 - nv2)) + ((var_cofssubmt_dn4 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn5 * (nv3 - nv2)) + ((var_cofssubmt_dn5 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn6 * (nv3 - nv2)) + ((var_cofssubmt_dn6 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn7 * (nv3 - nv2)) + ((var_cofssubmt_dn7 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn8 * (nv3 - nv2)) + ((var_cofssubmt_dn8 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn9 * (nv3 - nv2)) + ((var_cofssubmt_dn9 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn10 * (nv3 - nv2)) + ((var_cofssubmt_dn10 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn11 * (nv3 - nv2)) + ((var_cofssubmt_dn11 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn12 * (nv3 - nv2)) + ((var_cofssubmt_dn12 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn13 * (nv3 - nv2)) + ((var_cofssubmt_dn13 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn14 * (nv3 - nv2)) + ((var_cofssubmt_dn14 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn15 * (nv3 - nv2)) + ((var_cofssubmt_dn15 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn16 * (nv3 - nv2)) + ((var_cofssubmt_dn16 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn17 * (nv3 - nv2)) + ((var_cofssubmt_dn17 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn18 * (nv3 - nv2)) + ((var_cofssubmt_dn18 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn19 * (nv3 - nv2)) + ((var_cofssubmt_dn19 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn20 * (nv3 - nv2)) + ((var_cofssubmt_dn20 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn21 * (nv3 - nv2)) + ((var_cofssubmt_dn21 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn22 * (nv3 - nv2)) + ((var_cofssubmt_dn22 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn23 * (nv3 - nv2)) + ((var_cofssubmt_dn23 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn24 * (nv3 - nv2)) + ((var_cofssubmt_dn24 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn25 * (nv3 - nv2)) + ((var_cofssubmt_dn25 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn26 * (nv3 - nv2)) + ((var_cofssubmt_dn26 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn27 * (nv3 - nv2)) + ((var_cofssubmt_dn27 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn28 * (nv3 - nv2)) + ((var_cofssubmt_dn28 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_dn29 * (nv3 - nv2)) + ((var_cofssubmt_dn29 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db0 * (nv3 - nv2)) + ((var_cofssubmt_db0 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db1 * (nv3 - nv2)) + ((var_cofssubmt_db1 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db2 * (nv3 - nv2)) + ((var_cofssubmt_db2 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db3 * (nv3 - nv2)) + ((var_cofssubmt_db3 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db4 * (nv3 - nv2)) + ((var_cofssubmt_db4 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db5 * (nv3 - nv2)) + ((var_cofssubmt_db5 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db6 * (nv3 - nv2)) + ((var_cofssubmt_db6 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db7 * (nv3 - nv2)) + ((var_cofssubmt_db7 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db8 * (nv3 - nv2)) + ((var_cofssubmt_db8 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db9 * (nv3 - nv2)) + ((var_cofssubmt_db9 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db10 * (nv3 - nv2)) + ((var_cofssubmt_db10 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db11 * (nv3 - nv2)) + ((var_cofssubmt_db11 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db12 * (nv3 - nv2)) + ((var_cofssubmt_db12 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db13 * (nv3 - nv2)) + ((var_cofssubmt_db13 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db14 * (nv3 - nv2)) + ((var_cofssubmt_db14 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db15 * (nv3 - nv2)) + ((var_cofssubmt_db15 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db16 * (nv3 - nv2)) + ((var_cofssubmt_db16 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db17 * (nv3 - nv2)) + ((var_cofssubmt_db17 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db18 * (nv3 - nv2)) + ((var_cofssubmt_db18 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db19 * (nv3 - nv2)) + ((var_cofssubmt_db19 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db20 * (nv3 - nv2)) + ((var_cofssubmt_db20 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db21 * (nv3 - nv2)) + ((var_cofssubmt_db21 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db22 * (nv3 - nv2)) + ((var_cofssubmt_db22 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db23 * (nv3 - nv2)) + ((var_cofssubmt_db23 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db24 * (nv3 - nv2)) + ((var_cofssubmt_db24 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db25 * (nv3 - nv2)) + ((var_cofssubmt_db25 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db26 * (nv3 - nv2)) + ((var_cofssubmt_db26 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db27 * (nv3 - nv2)) + ((var_cofssubmt_db27 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db28 * (nv3 - nv2)) + ((var_cofssubmt_db28 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db29 * (nv3 - nv2)) + ((var_cofssubmt_db29 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db30 * (nv3 - nv2)) + ((var_cofssubmt_db30 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db31 * (nv3 - nv2)) + ((var_cofssubmt_db31 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db32 * (nv3 - nv2)) + ((var_cofssubmt_db32 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db33 * (nv3 - nv2)) + ((var_cofssubmt_db33 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db34 * (nv3 - nv2)) + ((var_cofssubmt_db34 * p.p28) * assign46340_e45106))), (assign46340_e45094 * ((var_cofssubmt0_db35 * (nv3 - nv2)) + ((var_cofssubmt_db35 * p.p28) * assign46340_e45106))),)
    } else {
        (var_qofssub, var_qofssub_dn0, var_qofssub_dn1, var_qofssub_dn2, var_qofssub_dn3, var_qofssub_dn4, var_qofssub_dn5, var_qofssub_dn6, var_qofssub_dn7, var_qofssub_dn8, var_qofssub_dn9, var_qofssub_dn10, var_qofssub_dn11, var_qofssub_dn12, var_qofssub_dn13, var_qofssub_dn14, var_qofssub_dn15, var_qofssub_dn16, var_qofssub_dn17, var_qofssub_dn18, var_qofssub_dn19, var_qofssub_dn20, var_qofssub_dn21, var_qofssub_dn22, var_qofssub_dn23, var_qofssub_dn24, var_qofssub_dn25, var_qofssub_dn26, var_qofssub_dn27, var_qofssub_dn28, var_qofssub_dn29, var_qofssub_db0, var_qofssub_db1, var_qofssub_db2, var_qofssub_db3, var_qofssub_db4, var_qofssub_db5, var_qofssub_db6, var_qofssub_db7, var_qofssub_db8, var_qofssub_db9, var_qofssub_db10, var_qofssub_db11, var_qofssub_db12, var_qofssub_db13, var_qofssub_db14, var_qofssub_db15, var_qofssub_db16, var_qofssub_db17, var_qofssub_db18, var_qofssub_db19, var_qofssub_db20, var_qofssub_db21, var_qofssub_db22, var_qofssub_db23, var_qofssub_db24, var_qofssub_db25, var_qofssub_db26, var_qofssub_db27, var_qofssub_db28, var_qofssub_db29, var_qofssub_db30, var_qofssub_db31, var_qofssub_db32, var_qofssub_db33, var_qofssub_db34, var_qofssub_db35,)
    }
};
        var_qofssub = assign46340_e45111;
        var_qofssub_dn0 = assign46340_e45111_d_n0;
        var_qofssub_dn1 = assign46340_e45111_d_n1;
        var_qofssub_dn2 = assign46340_e45111_d_n2;
        var_qofssub_dn3 = assign46340_e45111_d_n3;
        var_qofssub_dn4 = assign46340_e45111_d_n4;
        var_qofssub_dn5 = assign46340_e45111_d_n5;
        var_qofssub_dn6 = assign46340_e45111_d_n6;
        var_qofssub_dn7 = assign46340_e45111_d_n7;
        var_qofssub_dn8 = assign46340_e45111_d_n8;
        var_qofssub_dn9 = assign46340_e45111_d_n9;
        var_qofssub_dn10 = assign46340_e45111_d_n10;
        var_qofssub_dn11 = assign46340_e45111_d_n11;
        var_qofssub_dn12 = assign46340_e45111_d_n12;
        var_qofssub_dn13 = assign46340_e45111_d_n13;
        var_qofssub_dn14 = assign46340_e45111_d_n14;
        var_qofssub_dn15 = assign46340_e45111_d_n15;
        var_qofssub_dn16 = assign46340_e45111_d_n16;
        var_qofssub_dn17 = assign46340_e45111_d_n17;
        var_qofssub_dn18 = assign46340_e45111_d_n18;
        var_qofssub_dn19 = assign46340_e45111_d_n19;
        var_qofssub_dn20 = assign46340_e45111_d_n20;
        var_qofssub_dn21 = assign46340_e45111_d_n21;
        var_qofssub_dn22 = assign46340_e45111_d_n22;
        var_qofssub_dn23 = assign46340_e45111_d_n23;
        var_qofssub_dn24 = assign46340_e45111_d_n24;
        var_qofssub_dn25 = assign46340_e45111_d_n25;
        var_qofssub_dn26 = assign46340_e45111_d_n26;
        var_qofssub_dn27 = assign46340_e45111_d_n27;
        var_qofssub_dn28 = assign46340_e45111_d_n28;
        var_qofssub_dn29 = assign46340_e45111_d_n29;
        var_qofssub_db0 = assign46340_e45111_d_b0;
        var_qofssub_db1 = assign46340_e45111_d_b1;
        var_qofssub_db2 = assign46340_e45111_d_b2;
        var_qofssub_db3 = assign46340_e45111_d_b3;
        var_qofssub_db4 = assign46340_e45111_d_b4;
        var_qofssub_db5 = assign46340_e45111_d_b5;
        var_qofssub_db6 = assign46340_e45111_d_b6;
        var_qofssub_db7 = assign46340_e45111_d_b7;
        var_qofssub_db8 = assign46340_e45111_d_b8;
        var_qofssub_db9 = assign46340_e45111_d_b9;
        var_qofssub_db10 = assign46340_e45111_d_b10;
        var_qofssub_db11 = assign46340_e45111_d_b11;
        var_qofssub_db12 = assign46340_e45111_d_b12;
        var_qofssub_db13 = assign46340_e45111_d_b13;
        var_qofssub_db14 = assign46340_e45111_d_b14;
        var_qofssub_db15 = assign46340_e45111_d_b15;
        var_qofssub_db16 = assign46340_e45111_d_b16;
        var_qofssub_db17 = assign46340_e45111_d_b17;
        var_qofssub_db18 = assign46340_e45111_d_b18;
        var_qofssub_db19 = assign46340_e45111_d_b19;
        var_qofssub_db20 = assign46340_e45111_d_b20;
        var_qofssub_db21 = assign46340_e45111_d_b21;
        var_qofssub_db22 = assign46340_e45111_d_b22;
        var_qofssub_db23 = assign46340_e45111_d_b23;
        var_qofssub_db24 = assign46340_e45111_d_b24;
        var_qofssub_db25 = assign46340_e45111_d_b25;
        var_qofssub_db26 = assign46340_e45111_d_b26;
        var_qofssub_db27 = assign46340_e45111_d_b27;
        var_qofssub_db28 = assign46340_e45111_d_b28;
        var_qofssub_db29 = assign46340_e45111_d_b29;
        var_qofssub_db30 = assign46340_e45111_d_b30;
        var_qofssub_db31 = assign46340_e45111_d_b31;
        var_qofssub_db32 = assign46340_e45111_d_b32;
        var_qofssub_db33 = assign46340_e45111_d_b33;
        var_qofssub_db34 = assign46340_e45111_d_b34;
        var_qofssub_db35 = assign46340_e45111_d_b35;

        let (assign46350_e45139, assign46350_e45139_d_n0, assign46350_e45139_d_n1, assign46350_e45139_d_n2, assign46350_e45139_d_n3, assign46350_e45139_d_n4, assign46350_e45139_d_n5, assign46350_e45139_d_n6, assign46350_e45139_d_n7, assign46350_e45139_d_n8, assign46350_e45139_d_n9, assign46350_e45139_d_n10, assign46350_e45139_d_n11, assign46350_e45139_d_n12, assign46350_e45139_d_n13, assign46350_e45139_d_n14, assign46350_e45139_d_n15, assign46350_e45139_d_n16, assign46350_e45139_d_n17, assign46350_e45139_d_n18, assign46350_e45139_d_n19, assign46350_e45139_d_n20, assign46350_e45139_d_n21, assign46350_e45139_d_n22, assign46350_e45139_d_n23, assign46350_e45139_d_n24, assign46350_e45139_d_n25, assign46350_e45139_d_n26, assign46350_e45139_d_n27, assign46350_e45139_d_n28, assign46350_e45139_d_n29, assign46350_e45139_d_b0, assign46350_e45139_d_b1, assign46350_e45139_d_b2, assign46350_e45139_d_b3, assign46350_e45139_d_b4, assign46350_e45139_d_b5, assign46350_e45139_d_b6, assign46350_e45139_d_b7, assign46350_e45139_d_b8, assign46350_e45139_d_b9, assign46350_e45139_d_b10, assign46350_e45139_d_b11, assign46350_e45139_d_b12, assign46350_e45139_d_b13, assign46350_e45139_d_b14, assign46350_e45139_d_b15, assign46350_e45139_d_b16, assign46350_e45139_d_b17, assign46350_e45139_d_b18, assign46350_e45139_d_b19, assign46350_e45139_d_b20, assign46350_e45139_d_b21, assign46350_e45139_d_b22, assign46350_e45139_d_b23, assign46350_e45139_d_b24, assign46350_e45139_d_b25, assign46350_e45139_d_b26, assign46350_e45139_d_b27, assign46350_e45139_d_b28, assign46350_e45139_d_b29, assign46350_e45139_d_b30, assign46350_e45139_d_b31, assign46350_e45139_d_b32, assign46350_e45139_d_b33, assign46350_e45139_d_b34, assign46350_e45139_d_b35,) = {
    if ((var_guard503 == 0.0) && (var_guard504 == 0.0)) {
        let assign46350_e45119: f64 = (p.p0 * p.p2);
        let assign46350_e45122: f64 = (var_cofssubmt0 * (nv3 - nv2));
        let assign46350_e45125: f64 = (var_cofssubmt * p.p28);
        let assign46350_e45129: f64 = ((nv3 - nv2) - p.p27);
        let assign46350_e45131: f64 = (assign46350_e45129 / p.p28);
        let assign46350_e45132: f64 = (assign46350_e45131).exp();
        let assign46350_e45133: f64 = (1.0 + assign46350_e45132);
        let assign46350_e45134: f64 = (assign46350_e45133).ln();
        let assign46350_e45135: f64 = (assign46350_e45125 * assign46350_e45134);
        let assign46350_e45136: f64 = (assign46350_e45122 + assign46350_e45135);
        let assign46350_e45137: f64 = (assign46350_e45119 * assign46350_e45136);
        (assign46350_e45137, (assign46350_e45119 * ((var_cofssubmt0_dn0 * (nv3 - nv2)) + ((var_cofssubmt_dn0 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn1 * (nv3 - nv2)) + ((var_cofssubmt_dn1 * p.p28) * assign46350_e45134))), (assign46350_e45119 * (((var_cofssubmt0_dn2 * (nv3 - nv2)) + (-var_cofssubmt0)) + (((var_cofssubmt_dn2 * p.p28) * assign46350_e45134) + (assign46350_e45125 * ((assign46350_e45132 * (-1.0 / p.p28)) / assign46350_e45133))))), (assign46350_e45119 * (((var_cofssubmt0_dn3 * (nv3 - nv2)) + var_cofssubmt0) + (((var_cofssubmt_dn3 * p.p28) * assign46350_e45134) + (assign46350_e45125 * ((assign46350_e45132 * (1.0 / p.p28)) / assign46350_e45133))))), (assign46350_e45119 * ((var_cofssubmt0_dn4 * (nv3 - nv2)) + ((var_cofssubmt_dn4 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn5 * (nv3 - nv2)) + ((var_cofssubmt_dn5 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn6 * (nv3 - nv2)) + ((var_cofssubmt_dn6 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn7 * (nv3 - nv2)) + ((var_cofssubmt_dn7 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn8 * (nv3 - nv2)) + ((var_cofssubmt_dn8 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn9 * (nv3 - nv2)) + ((var_cofssubmt_dn9 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn10 * (nv3 - nv2)) + ((var_cofssubmt_dn10 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn11 * (nv3 - nv2)) + ((var_cofssubmt_dn11 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn12 * (nv3 - nv2)) + ((var_cofssubmt_dn12 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn13 * (nv3 - nv2)) + ((var_cofssubmt_dn13 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn14 * (nv3 - nv2)) + ((var_cofssubmt_dn14 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn15 * (nv3 - nv2)) + ((var_cofssubmt_dn15 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn16 * (nv3 - nv2)) + ((var_cofssubmt_dn16 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn17 * (nv3 - nv2)) + ((var_cofssubmt_dn17 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn18 * (nv3 - nv2)) + ((var_cofssubmt_dn18 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn19 * (nv3 - nv2)) + ((var_cofssubmt_dn19 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn20 * (nv3 - nv2)) + ((var_cofssubmt_dn20 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn21 * (nv3 - nv2)) + ((var_cofssubmt_dn21 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn22 * (nv3 - nv2)) + ((var_cofssubmt_dn22 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn23 * (nv3 - nv2)) + ((var_cofssubmt_dn23 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn24 * (nv3 - nv2)) + ((var_cofssubmt_dn24 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn25 * (nv3 - nv2)) + ((var_cofssubmt_dn25 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn26 * (nv3 - nv2)) + ((var_cofssubmt_dn26 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn27 * (nv3 - nv2)) + ((var_cofssubmt_dn27 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn28 * (nv3 - nv2)) + ((var_cofssubmt_dn28 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_dn29 * (nv3 - nv2)) + ((var_cofssubmt_dn29 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db0 * (nv3 - nv2)) + ((var_cofssubmt_db0 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db1 * (nv3 - nv2)) + ((var_cofssubmt_db1 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db2 * (nv3 - nv2)) + ((var_cofssubmt_db2 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db3 * (nv3 - nv2)) + ((var_cofssubmt_db3 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db4 * (nv3 - nv2)) + ((var_cofssubmt_db4 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db5 * (nv3 - nv2)) + ((var_cofssubmt_db5 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db6 * (nv3 - nv2)) + ((var_cofssubmt_db6 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db7 * (nv3 - nv2)) + ((var_cofssubmt_db7 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db8 * (nv3 - nv2)) + ((var_cofssubmt_db8 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db9 * (nv3 - nv2)) + ((var_cofssubmt_db9 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db10 * (nv3 - nv2)) + ((var_cofssubmt_db10 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db11 * (nv3 - nv2)) + ((var_cofssubmt_db11 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db12 * (nv3 - nv2)) + ((var_cofssubmt_db12 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db13 * (nv3 - nv2)) + ((var_cofssubmt_db13 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db14 * (nv3 - nv2)) + ((var_cofssubmt_db14 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db15 * (nv3 - nv2)) + ((var_cofssubmt_db15 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db16 * (nv3 - nv2)) + ((var_cofssubmt_db16 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db17 * (nv3 - nv2)) + ((var_cofssubmt_db17 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db18 * (nv3 - nv2)) + ((var_cofssubmt_db18 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db19 * (nv3 - nv2)) + ((var_cofssubmt_db19 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db20 * (nv3 - nv2)) + ((var_cofssubmt_db20 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db21 * (nv3 - nv2)) + ((var_cofssubmt_db21 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db22 * (nv3 - nv2)) + ((var_cofssubmt_db22 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db23 * (nv3 - nv2)) + ((var_cofssubmt_db23 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db24 * (nv3 - nv2)) + ((var_cofssubmt_db24 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db25 * (nv3 - nv2)) + ((var_cofssubmt_db25 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db26 * (nv3 - nv2)) + ((var_cofssubmt_db26 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db27 * (nv3 - nv2)) + ((var_cofssubmt_db27 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db28 * (nv3 - nv2)) + ((var_cofssubmt_db28 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db29 * (nv3 - nv2)) + ((var_cofssubmt_db29 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db30 * (nv3 - nv2)) + ((var_cofssubmt_db30 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db31 * (nv3 - nv2)) + ((var_cofssubmt_db31 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db32 * (nv3 - nv2)) + ((var_cofssubmt_db32 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db33 * (nv3 - nv2)) + ((var_cofssubmt_db33 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db34 * (nv3 - nv2)) + ((var_cofssubmt_db34 * p.p28) * assign46350_e45134))), (assign46350_e45119 * ((var_cofssubmt0_db35 * (nv3 - nv2)) + ((var_cofssubmt_db35 * p.p28) * assign46350_e45134))),)
    } else {
        (var_qofssub, var_qofssub_dn0, var_qofssub_dn1, var_qofssub_dn2, var_qofssub_dn3, var_qofssub_dn4, var_qofssub_dn5, var_qofssub_dn6, var_qofssub_dn7, var_qofssub_dn8, var_qofssub_dn9, var_qofssub_dn10, var_qofssub_dn11, var_qofssub_dn12, var_qofssub_dn13, var_qofssub_dn14, var_qofssub_dn15, var_qofssub_dn16, var_qofssub_dn17, var_qofssub_dn18, var_qofssub_dn19, var_qofssub_dn20, var_qofssub_dn21, var_qofssub_dn22, var_qofssub_dn23, var_qofssub_dn24, var_qofssub_dn25, var_qofssub_dn26, var_qofssub_dn27, var_qofssub_dn28, var_qofssub_dn29, var_qofssub_db0, var_qofssub_db1, var_qofssub_db2, var_qofssub_db3, var_qofssub_db4, var_qofssub_db5, var_qofssub_db6, var_qofssub_db7, var_qofssub_db8, var_qofssub_db9, var_qofssub_db10, var_qofssub_db11, var_qofssub_db12, var_qofssub_db13, var_qofssub_db14, var_qofssub_db15, var_qofssub_db16, var_qofssub_db17, var_qofssub_db18, var_qofssub_db19, var_qofssub_db20, var_qofssub_db21, var_qofssub_db22, var_qofssub_db23, var_qofssub_db24, var_qofssub_db25, var_qofssub_db26, var_qofssub_db27, var_qofssub_db28, var_qofssub_db29, var_qofssub_db30, var_qofssub_db31, var_qofssub_db32, var_qofssub_db33, var_qofssub_db34, var_qofssub_db35,)
    }
};
        var_qofssub = assign46350_e45139;
        var_qofssub_dn0 = assign46350_e45139_d_n0;
        var_qofssub_dn1 = assign46350_e45139_d_n1;
        var_qofssub_dn2 = assign46350_e45139_d_n2;
        var_qofssub_dn3 = assign46350_e45139_d_n3;
        var_qofssub_dn4 = assign46350_e45139_d_n4;
        var_qofssub_dn5 = assign46350_e45139_d_n5;
        var_qofssub_dn6 = assign46350_e45139_d_n6;
        var_qofssub_dn7 = assign46350_e45139_d_n7;
        var_qofssub_dn8 = assign46350_e45139_d_n8;
        var_qofssub_dn9 = assign46350_e45139_d_n9;
        var_qofssub_dn10 = assign46350_e45139_d_n10;
        var_qofssub_dn11 = assign46350_e45139_d_n11;
        var_qofssub_dn12 = assign46350_e45139_d_n12;
        var_qofssub_dn13 = assign46350_e45139_d_n13;
        var_qofssub_dn14 = assign46350_e45139_d_n14;
        var_qofssub_dn15 = assign46350_e45139_d_n15;
        var_qofssub_dn16 = assign46350_e45139_d_n16;
        var_qofssub_dn17 = assign46350_e45139_d_n17;
        var_qofssub_dn18 = assign46350_e45139_d_n18;
        var_qofssub_dn19 = assign46350_e45139_d_n19;
        var_qofssub_dn20 = assign46350_e45139_d_n20;
        var_qofssub_dn21 = assign46350_e45139_d_n21;
        var_qofssub_dn22 = assign46350_e45139_d_n22;
        var_qofssub_dn23 = assign46350_e45139_d_n23;
        var_qofssub_dn24 = assign46350_e45139_d_n24;
        var_qofssub_dn25 = assign46350_e45139_d_n25;
        var_qofssub_dn26 = assign46350_e45139_d_n26;
        var_qofssub_dn27 = assign46350_e45139_d_n27;
        var_qofssub_dn28 = assign46350_e45139_d_n28;
        var_qofssub_dn29 = assign46350_e45139_d_n29;
        var_qofssub_db0 = assign46350_e45139_d_b0;
        var_qofssub_db1 = assign46350_e45139_d_b1;
        var_qofssub_db2 = assign46350_e45139_d_b2;
        var_qofssub_db3 = assign46350_e45139_d_b3;
        var_qofssub_db4 = assign46350_e45139_d_b4;
        var_qofssub_db5 = assign46350_e45139_d_b5;
        var_qofssub_db6 = assign46350_e45139_d_b6;
        var_qofssub_db7 = assign46350_e45139_d_b7;
        var_qofssub_db8 = assign46350_e45139_d_b8;
        var_qofssub_db9 = assign46350_e45139_d_b9;
        var_qofssub_db10 = assign46350_e45139_d_b10;
        var_qofssub_db11 = assign46350_e45139_d_b11;
        var_qofssub_db12 = assign46350_e45139_d_b12;
        var_qofssub_db13 = assign46350_e45139_d_b13;
        var_qofssub_db14 = assign46350_e45139_d_b14;
        var_qofssub_db15 = assign46350_e45139_d_b15;
        var_qofssub_db16 = assign46350_e45139_d_b16;
        var_qofssub_db17 = assign46350_e45139_d_b17;
        var_qofssub_db18 = assign46350_e45139_d_b18;
        var_qofssub_db19 = assign46350_e45139_d_b19;
        var_qofssub_db20 = assign46350_e45139_d_b20;
        var_qofssub_db21 = assign46350_e45139_d_b21;
        var_qofssub_db22 = assign46350_e45139_d_b22;
        var_qofssub_db23 = assign46350_e45139_d_b23;
        var_qofssub_db24 = assign46350_e45139_d_b24;
        var_qofssub_db25 = assign46350_e45139_d_b25;
        var_qofssub_db26 = assign46350_e45139_d_b26;
        var_qofssub_db27 = assign46350_e45139_d_b27;
        var_qofssub_db28 = assign46350_e45139_d_b28;
        var_qofssub_db29 = assign46350_e45139_d_b29;
        var_qofssub_db30 = assign46350_e45139_d_b30;
        var_qofssub_db31 = assign46350_e45139_d_b31;
        var_qofssub_db32 = assign46350_e45139_d_b32;
        var_qofssub_db33 = assign46350_e45139_d_b33;
        var_qofssub_db34 = assign46350_e45139_d_b34;
        var_qofssub_db35 = assign46350_e45139_d_b35;

        let assign46360_e45142: f64 = ((nv3 - nv0) - p.p27);
        let assign46360_e45144: f64 = (assign46360_e45142 / p.p28);
        let assign46360_e45146: f64 = if assign46360_e45144 > 50.0 { 1.0 } else { 0.0 };
        var_guard505 = assign46360_e45146;


        *var_guard503_slot = var_guard503;
        *var_guard504_slot = var_guard504;
        *var_guard505_slot = var_guard505;
        *var_qofds_slot = var_qofds;
        *var_qofds_db0_slot = var_qofds_db0;
        *var_qofds_db1_slot = var_qofds_db1;
        *var_qofds_db10_slot = var_qofds_db10;
        *var_qofds_db11_slot = var_qofds_db11;
        *var_qofds_db12_slot = var_qofds_db12;
        *var_qofds_db13_slot = var_qofds_db13;
        *var_qofds_db14_slot = var_qofds_db14;
        *var_qofds_db15_slot = var_qofds_db15;
        *var_qofds_db16_slot = var_qofds_db16;
        *var_qofds_db17_slot = var_qofds_db17;
        *var_qofds_db18_slot = var_qofds_db18;
        *var_qofds_db19_slot = var_qofds_db19;
        *var_qofds_db2_slot = var_qofds_db2;
        *var_qofds_db20_slot = var_qofds_db20;
        *var_qofds_db21_slot = var_qofds_db21;
        *var_qofds_db22_slot = var_qofds_db22;
        *var_qofds_db23_slot = var_qofds_db23;
        *var_qofds_db24_slot = var_qofds_db24;
        *var_qofds_db25_slot = var_qofds_db25;
        *var_qofds_db26_slot = var_qofds_db26;
        *var_qofds_db27_slot = var_qofds_db27;
        *var_qofds_db28_slot = var_qofds_db28;
        *var_qofds_db29_slot = var_qofds_db29;
        *var_qofds_db3_slot = var_qofds_db3;
        *var_qofds_db30_slot = var_qofds_db30;
        *var_qofds_db31_slot = var_qofds_db31;
        *var_qofds_db32_slot = var_qofds_db32;
        *var_qofds_db33_slot = var_qofds_db33;
        *var_qofds_db34_slot = var_qofds_db34;
        *var_qofds_db35_slot = var_qofds_db35;
        *var_qofds_db4_slot = var_qofds_db4;
        *var_qofds_db5_slot = var_qofds_db5;
        *var_qofds_db6_slot = var_qofds_db6;
        *var_qofds_db7_slot = var_qofds_db7;
        *var_qofds_db8_slot = var_qofds_db8;
        *var_qofds_db9_slot = var_qofds_db9;
        *var_qofds_dn0_slot = var_qofds_dn0;
        *var_qofds_dn1_slot = var_qofds_dn1;
        *var_qofds_dn10_slot = var_qofds_dn10;
        *var_qofds_dn11_slot = var_qofds_dn11;
        *var_qofds_dn12_slot = var_qofds_dn12;
        *var_qofds_dn13_slot = var_qofds_dn13;
        *var_qofds_dn14_slot = var_qofds_dn14;
        *var_qofds_dn15_slot = var_qofds_dn15;
        *var_qofds_dn16_slot = var_qofds_dn16;
        *var_qofds_dn17_slot = var_qofds_dn17;
        *var_qofds_dn18_slot = var_qofds_dn18;
        *var_qofds_dn19_slot = var_qofds_dn19;
        *var_qofds_dn2_slot = var_qofds_dn2;
        *var_qofds_dn20_slot = var_qofds_dn20;
        *var_qofds_dn21_slot = var_qofds_dn21;
        *var_qofds_dn22_slot = var_qofds_dn22;
        *var_qofds_dn23_slot = var_qofds_dn23;
        *var_qofds_dn24_slot = var_qofds_dn24;
        *var_qofds_dn25_slot = var_qofds_dn25;
        *var_qofds_dn26_slot = var_qofds_dn26;
        *var_qofds_dn27_slot = var_qofds_dn27;
        *var_qofds_dn28_slot = var_qofds_dn28;
        *var_qofds_dn29_slot = var_qofds_dn29;
        *var_qofds_dn3_slot = var_qofds_dn3;
        *var_qofds_dn4_slot = var_qofds_dn4;
        *var_qofds_dn5_slot = var_qofds_dn5;
        *var_qofds_dn6_slot = var_qofds_dn6;
        *var_qofds_dn7_slot = var_qofds_dn7;
        *var_qofds_dn8_slot = var_qofds_dn8;
        *var_qofds_dn9_slot = var_qofds_dn9;
        *var_qofssub_slot = var_qofssub;
        *var_qofssub_db0_slot = var_qofssub_db0;
        *var_qofssub_db1_slot = var_qofssub_db1;
        *var_qofssub_db10_slot = var_qofssub_db10;
        *var_qofssub_db11_slot = var_qofssub_db11;
        *var_qofssub_db12_slot = var_qofssub_db12;
        *var_qofssub_db13_slot = var_qofssub_db13;
        *var_qofssub_db14_slot = var_qofssub_db14;
        *var_qofssub_db15_slot = var_qofssub_db15;
        *var_qofssub_db16_slot = var_qofssub_db16;
        *var_qofssub_db17_slot = var_qofssub_db17;
        *var_qofssub_db18_slot = var_qofssub_db18;
        *var_qofssub_db19_slot = var_qofssub_db19;
        *var_qofssub_db2_slot = var_qofssub_db2;
        *var_qofssub_db20_slot = var_qofssub_db20;
        *var_qofssub_db21_slot = var_qofssub_db21;
        *var_qofssub_db22_slot = var_qofssub_db22;
        *var_qofssub_db23_slot = var_qofssub_db23;
        *var_qofssub_db24_slot = var_qofssub_db24;
        *var_qofssub_db25_slot = var_qofssub_db25;
        *var_qofssub_db26_slot = var_qofssub_db26;
        *var_qofssub_db27_slot = var_qofssub_db27;
        *var_qofssub_db28_slot = var_qofssub_db28;
        *var_qofssub_db29_slot = var_qofssub_db29;
        *var_qofssub_db3_slot = var_qofssub_db3;
        *var_qofssub_db30_slot = var_qofssub_db30;
        *var_qofssub_db31_slot = var_qofssub_db31;
        *var_qofssub_db32_slot = var_qofssub_db32;
        *var_qofssub_db33_slot = var_qofssub_db33;
        *var_qofssub_db34_slot = var_qofssub_db34;
        *var_qofssub_db35_slot = var_qofssub_db35;
        *var_qofssub_db4_slot = var_qofssub_db4;
        *var_qofssub_db5_slot = var_qofssub_db5;
        *var_qofssub_db6_slot = var_qofssub_db6;
        *var_qofssub_db7_slot = var_qofssub_db7;
        *var_qofssub_db8_slot = var_qofssub_db8;
        *var_qofssub_db9_slot = var_qofssub_db9;
        *var_qofssub_dn0_slot = var_qofssub_dn0;
        *var_qofssub_dn1_slot = var_qofssub_dn1;
        *var_qofssub_dn10_slot = var_qofssub_dn10;
        *var_qofssub_dn11_slot = var_qofssub_dn11;
        *var_qofssub_dn12_slot = var_qofssub_dn12;
        *var_qofssub_dn13_slot = var_qofssub_dn13;
        *var_qofssub_dn14_slot = var_qofssub_dn14;
        *var_qofssub_dn15_slot = var_qofssub_dn15;
        *var_qofssub_dn16_slot = var_qofssub_dn16;
        *var_qofssub_dn17_slot = var_qofssub_dn17;
        *var_qofssub_dn18_slot = var_qofssub_dn18;
        *var_qofssub_dn19_slot = var_qofssub_dn19;
        *var_qofssub_dn2_slot = var_qofssub_dn2;
        *var_qofssub_dn20_slot = var_qofssub_dn20;
        *var_qofssub_dn21_slot = var_qofssub_dn21;
        *var_qofssub_dn22_slot = var_qofssub_dn22;
        *var_qofssub_dn23_slot = var_qofssub_dn23;
        *var_qofssub_dn24_slot = var_qofssub_dn24;
        *var_qofssub_dn25_slot = var_qofssub_dn25;
        *var_qofssub_dn26_slot = var_qofssub_dn26;
        *var_qofssub_dn27_slot = var_qofssub_dn27;
        *var_qofssub_dn28_slot = var_qofssub_dn28;
        *var_qofssub_dn29_slot = var_qofssub_dn29;
        *var_qofssub_dn3_slot = var_qofssub_dn3;
        *var_qofssub_dn4_slot = var_qofssub_dn4;
        *var_qofssub_dn5_slot = var_qofssub_dn5;
        *var_qofssub_dn6_slot = var_qofssub_dn6;
        *var_qofssub_dn7_slot = var_qofssub_dn7;
        *var_qofssub_dn8_slot = var_qofssub_dn8;
        *var_qofssub_dn9_slot = var_qofssub_dn9;
    }

    pub(super) fn stamp_transient_block_122(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_cofdsubmt: f64,
        var_cofdsubmt0: f64,
        var_cofdsubmt0_db0: f64,
        var_cofdsubmt0_db1: f64,
        var_cofdsubmt0_db10: f64,
        var_cofdsubmt0_db11: f64,
        var_cofdsubmt0_db12: f64,
        var_cofdsubmt0_db13: f64,
        var_cofdsubmt0_db14: f64,
        var_cofdsubmt0_db15: f64,
        var_cofdsubmt0_db16: f64,
        var_cofdsubmt0_db17: f64,
        var_cofdsubmt0_db18: f64,
        var_cofdsubmt0_db19: f64,
        var_cofdsubmt0_db2: f64,
        var_cofdsubmt0_db20: f64,
        var_cofdsubmt0_db21: f64,
        var_cofdsubmt0_db22: f64,
        var_cofdsubmt0_db23: f64,
        var_cofdsubmt0_db24: f64,
        var_cofdsubmt0_db25: f64,
        var_cofdsubmt0_db26: f64,
        var_cofdsubmt0_db27: f64,
        var_cofdsubmt0_db28: f64,
        var_cofdsubmt0_db29: f64,
        var_cofdsubmt0_db3: f64,
        var_cofdsubmt0_db30: f64,
        var_cofdsubmt0_db31: f64,
        var_cofdsubmt0_db32: f64,
        var_cofdsubmt0_db33: f64,
        var_cofdsubmt0_db34: f64,
        var_cofdsubmt0_db35: f64,
        var_cofdsubmt0_db4: f64,
        var_cofdsubmt0_db5: f64,
        var_cofdsubmt0_db6: f64,
        var_cofdsubmt0_db7: f64,
        var_cofdsubmt0_db8: f64,
        var_cofdsubmt0_db9: f64,
        var_cofdsubmt0_dn0: f64,
        var_cofdsubmt0_dn1: f64,
        var_cofdsubmt0_dn10: f64,
        var_cofdsubmt0_dn11: f64,
        var_cofdsubmt0_dn12: f64,
        var_cofdsubmt0_dn13: f64,
        var_cofdsubmt0_dn14: f64,
        var_cofdsubmt0_dn15: f64,
        var_cofdsubmt0_dn16: f64,
        var_cofdsubmt0_dn17: f64,
        var_cofdsubmt0_dn18: f64,
        var_cofdsubmt0_dn19: f64,
        var_cofdsubmt0_dn2: f64,
        var_cofdsubmt0_dn20: f64,
        var_cofdsubmt0_dn21: f64,
        var_cofdsubmt0_dn22: f64,
        var_cofdsubmt0_dn23: f64,
        var_cofdsubmt0_dn24: f64,
        var_cofdsubmt0_dn25: f64,
        var_cofdsubmt0_dn26: f64,
        var_cofdsubmt0_dn27: f64,
        var_cofdsubmt0_dn28: f64,
        var_cofdsubmt0_dn29: f64,
        var_cofdsubmt0_dn3: f64,
        var_cofdsubmt0_dn4: f64,
        var_cofdsubmt0_dn5: f64,
        var_cofdsubmt0_dn6: f64,
        var_cofdsubmt0_dn7: f64,
        var_cofdsubmt0_dn8: f64,
        var_cofdsubmt0_dn9: f64,
        var_cofdsubmt_db0: f64,
        var_cofdsubmt_db1: f64,
        var_cofdsubmt_db10: f64,
        var_cofdsubmt_db11: f64,
        var_cofdsubmt_db12: f64,
        var_cofdsubmt_db13: f64,
        var_cofdsubmt_db14: f64,
        var_cofdsubmt_db15: f64,
        var_cofdsubmt_db16: f64,
        var_cofdsubmt_db17: f64,
        var_cofdsubmt_db18: f64,
        var_cofdsubmt_db19: f64,
        var_cofdsubmt_db2: f64,
        var_cofdsubmt_db20: f64,
        var_cofdsubmt_db21: f64,
        var_cofdsubmt_db22: f64,
        var_cofdsubmt_db23: f64,
        var_cofdsubmt_db24: f64,
        var_cofdsubmt_db25: f64,
        var_cofdsubmt_db26: f64,
        var_cofdsubmt_db27: f64,
        var_cofdsubmt_db28: f64,
        var_cofdsubmt_db29: f64,
        var_cofdsubmt_db3: f64,
        var_cofdsubmt_db30: f64,
        var_cofdsubmt_db31: f64,
        var_cofdsubmt_db32: f64,
        var_cofdsubmt_db33: f64,
        var_cofdsubmt_db34: f64,
        var_cofdsubmt_db35: f64,
        var_cofdsubmt_db4: f64,
        var_cofdsubmt_db5: f64,
        var_cofdsubmt_db6: f64,
        var_cofdsubmt_db7: f64,
        var_cofdsubmt_db8: f64,
        var_cofdsubmt_db9: f64,
        var_cofdsubmt_dn0: f64,
        var_cofdsubmt_dn1: f64,
        var_cofdsubmt_dn10: f64,
        var_cofdsubmt_dn11: f64,
        var_cofdsubmt_dn12: f64,
        var_cofdsubmt_dn13: f64,
        var_cofdsubmt_dn14: f64,
        var_cofdsubmt_dn15: f64,
        var_cofdsubmt_dn16: f64,
        var_cofdsubmt_dn17: f64,
        var_cofdsubmt_dn18: f64,
        var_cofdsubmt_dn19: f64,
        var_cofdsubmt_dn2: f64,
        var_cofdsubmt_dn20: f64,
        var_cofdsubmt_dn21: f64,
        var_cofdsubmt_dn22: f64,
        var_cofdsubmt_dn23: f64,
        var_cofdsubmt_dn24: f64,
        var_cofdsubmt_dn25: f64,
        var_cofdsubmt_dn26: f64,
        var_cofdsubmt_dn27: f64,
        var_cofdsubmt_dn28: f64,
        var_cofdsubmt_dn29: f64,
        var_cofdsubmt_dn3: f64,
        var_cofdsubmt_dn4: f64,
        var_cofdsubmt_dn5: f64,
        var_cofdsubmt_dn6: f64,
        var_cofdsubmt_dn7: f64,
        var_cofdsubmt_dn8: f64,
        var_cofdsubmt_dn9: f64,
        var_cofgsubmt: f64,
        var_cofgsubmt0: f64,
        var_cofgsubmt0_db0: f64,
        var_cofgsubmt0_db1: f64,
        var_cofgsubmt0_db10: f64,
        var_cofgsubmt0_db11: f64,
        var_cofgsubmt0_db12: f64,
        var_cofgsubmt0_db13: f64,
        var_cofgsubmt0_db14: f64,
        var_cofgsubmt0_db15: f64,
        var_cofgsubmt0_db16: f64,
        var_cofgsubmt0_db17: f64,
        var_cofgsubmt0_db18: f64,
        var_cofgsubmt0_db19: f64,
        var_cofgsubmt0_db2: f64,
        var_cofgsubmt0_db20: f64,
        var_cofgsubmt0_db21: f64,
        var_cofgsubmt0_db22: f64,
        var_cofgsubmt0_db23: f64,
        var_cofgsubmt0_db24: f64,
        var_cofgsubmt0_db25: f64,
        var_cofgsubmt0_db26: f64,
        var_cofgsubmt0_db27: f64,
        var_cofgsubmt0_db28: f64,
        var_cofgsubmt0_db29: f64,
        var_cofgsubmt0_db3: f64,
        var_cofgsubmt0_db30: f64,
        var_cofgsubmt0_db31: f64,
        var_cofgsubmt0_db32: f64,
        var_cofgsubmt0_db33: f64,
        var_cofgsubmt0_db34: f64,
        var_cofgsubmt0_db35: f64,
        var_cofgsubmt0_db4: f64,
        var_cofgsubmt0_db5: f64,
        var_cofgsubmt0_db6: f64,
        var_cofgsubmt0_db7: f64,
        var_cofgsubmt0_db8: f64,
        var_cofgsubmt0_db9: f64,
        var_cofgsubmt0_dn0: f64,
        var_cofgsubmt0_dn1: f64,
        var_cofgsubmt0_dn10: f64,
        var_cofgsubmt0_dn11: f64,
        var_cofgsubmt0_dn12: f64,
        var_cofgsubmt0_dn13: f64,
        var_cofgsubmt0_dn14: f64,
        var_cofgsubmt0_dn15: f64,
        var_cofgsubmt0_dn16: f64,
        var_cofgsubmt0_dn17: f64,
        var_cofgsubmt0_dn18: f64,
        var_cofgsubmt0_dn19: f64,
        var_cofgsubmt0_dn2: f64,
        var_cofgsubmt0_dn20: f64,
        var_cofgsubmt0_dn21: f64,
        var_cofgsubmt0_dn22: f64,
        var_cofgsubmt0_dn23: f64,
        var_cofgsubmt0_dn24: f64,
        var_cofgsubmt0_dn25: f64,
        var_cofgsubmt0_dn26: f64,
        var_cofgsubmt0_dn27: f64,
        var_cofgsubmt0_dn28: f64,
        var_cofgsubmt0_dn29: f64,
        var_cofgsubmt0_dn3: f64,
        var_cofgsubmt0_dn4: f64,
        var_cofgsubmt0_dn5: f64,
        var_cofgsubmt0_dn6: f64,
        var_cofgsubmt0_dn7: f64,
        var_cofgsubmt0_dn8: f64,
        var_cofgsubmt0_dn9: f64,
        var_cofgsubmt_db0: f64,
        var_cofgsubmt_db1: f64,
        var_cofgsubmt_db10: f64,
        var_cofgsubmt_db11: f64,
        var_cofgsubmt_db12: f64,
        var_cofgsubmt_db13: f64,
        var_cofgsubmt_db14: f64,
        var_cofgsubmt_db15: f64,
        var_cofgsubmt_db16: f64,
        var_cofgsubmt_db17: f64,
        var_cofgsubmt_db18: f64,
        var_cofgsubmt_db19: f64,
        var_cofgsubmt_db2: f64,
        var_cofgsubmt_db20: f64,
        var_cofgsubmt_db21: f64,
        var_cofgsubmt_db22: f64,
        var_cofgsubmt_db23: f64,
        var_cofgsubmt_db24: f64,
        var_cofgsubmt_db25: f64,
        var_cofgsubmt_db26: f64,
        var_cofgsubmt_db27: f64,
        var_cofgsubmt_db28: f64,
        var_cofgsubmt_db29: f64,
        var_cofgsubmt_db3: f64,
        var_cofgsubmt_db30: f64,
        var_cofgsubmt_db31: f64,
        var_cofgsubmt_db32: f64,
        var_cofgsubmt_db33: f64,
        var_cofgsubmt_db34: f64,
        var_cofgsubmt_db35: f64,
        var_cofgsubmt_db4: f64,
        var_cofgsubmt_db5: f64,
        var_cofgsubmt_db6: f64,
        var_cofgsubmt_db7: f64,
        var_cofgsubmt_db8: f64,
        var_cofgsubmt_db9: f64,
        var_cofgsubmt_dn0: f64,
        var_cofgsubmt_dn1: f64,
        var_cofgsubmt_dn10: f64,
        var_cofgsubmt_dn11: f64,
        var_cofgsubmt_dn12: f64,
        var_cofgsubmt_dn13: f64,
        var_cofgsubmt_dn14: f64,
        var_cofgsubmt_dn15: f64,
        var_cofgsubmt_dn16: f64,
        var_cofgsubmt_dn17: f64,
        var_cofgsubmt_dn18: f64,
        var_cofgsubmt_dn19: f64,
        var_cofgsubmt_dn2: f64,
        var_cofgsubmt_dn20: f64,
        var_cofgsubmt_dn21: f64,
        var_cofgsubmt_dn22: f64,
        var_cofgsubmt_dn23: f64,
        var_cofgsubmt_dn24: f64,
        var_cofgsubmt_dn25: f64,
        var_cofgsubmt_dn26: f64,
        var_cofgsubmt_dn27: f64,
        var_cofgsubmt_dn28: f64,
        var_cofgsubmt_dn29: f64,
        var_cofgsubmt_dn3: f64,
        var_cofgsubmt_dn4: f64,
        var_cofgsubmt_dn5: f64,
        var_cofgsubmt_dn6: f64,
        var_cofgsubmt_dn7: f64,
        var_cofgsubmt_dn8: f64,
        var_cofgsubmt_dn9: f64,
        var_guard505: f64,
        var_guard506_slot: &mut f64,
        var_guard507_slot: &mut f64,
        var_guard508_slot: &mut f64,
        var_qofdsub_slot: &mut f64,
        var_qofdsub_db0_slot: &mut f64,
        var_qofdsub_db1_slot: &mut f64,
        var_qofdsub_db10_slot: &mut f64,
        var_qofdsub_db11_slot: &mut f64,
        var_qofdsub_db12_slot: &mut f64,
        var_qofdsub_db13_slot: &mut f64,
        var_qofdsub_db14_slot: &mut f64,
        var_qofdsub_db15_slot: &mut f64,
        var_qofdsub_db16_slot: &mut f64,
        var_qofdsub_db17_slot: &mut f64,
        var_qofdsub_db18_slot: &mut f64,
        var_qofdsub_db19_slot: &mut f64,
        var_qofdsub_db2_slot: &mut f64,
        var_qofdsub_db20_slot: &mut f64,
        var_qofdsub_db21_slot: &mut f64,
        var_qofdsub_db22_slot: &mut f64,
        var_qofdsub_db23_slot: &mut f64,
        var_qofdsub_db24_slot: &mut f64,
        var_qofdsub_db25_slot: &mut f64,
        var_qofdsub_db26_slot: &mut f64,
        var_qofdsub_db27_slot: &mut f64,
        var_qofdsub_db28_slot: &mut f64,
        var_qofdsub_db29_slot: &mut f64,
        var_qofdsub_db3_slot: &mut f64,
        var_qofdsub_db30_slot: &mut f64,
        var_qofdsub_db31_slot: &mut f64,
        var_qofdsub_db32_slot: &mut f64,
        var_qofdsub_db33_slot: &mut f64,
        var_qofdsub_db34_slot: &mut f64,
        var_qofdsub_db35_slot: &mut f64,
        var_qofdsub_db4_slot: &mut f64,
        var_qofdsub_db5_slot: &mut f64,
        var_qofdsub_db6_slot: &mut f64,
        var_qofdsub_db7_slot: &mut f64,
        var_qofdsub_db8_slot: &mut f64,
        var_qofdsub_db9_slot: &mut f64,
        var_qofdsub_dn0_slot: &mut f64,
        var_qofdsub_dn1_slot: &mut f64,
        var_qofdsub_dn10_slot: &mut f64,
        var_qofdsub_dn11_slot: &mut f64,
        var_qofdsub_dn12_slot: &mut f64,
        var_qofdsub_dn13_slot: &mut f64,
        var_qofdsub_dn14_slot: &mut f64,
        var_qofdsub_dn15_slot: &mut f64,
        var_qofdsub_dn16_slot: &mut f64,
        var_qofdsub_dn17_slot: &mut f64,
        var_qofdsub_dn18_slot: &mut f64,
        var_qofdsub_dn19_slot: &mut f64,
        var_qofdsub_dn2_slot: &mut f64,
        var_qofdsub_dn20_slot: &mut f64,
        var_qofdsub_dn21_slot: &mut f64,
        var_qofdsub_dn22_slot: &mut f64,
        var_qofdsub_dn23_slot: &mut f64,
        var_qofdsub_dn24_slot: &mut f64,
        var_qofdsub_dn25_slot: &mut f64,
        var_qofdsub_dn26_slot: &mut f64,
        var_qofdsub_dn27_slot: &mut f64,
        var_qofdsub_dn28_slot: &mut f64,
        var_qofdsub_dn29_slot: &mut f64,
        var_qofdsub_dn3_slot: &mut f64,
        var_qofdsub_dn4_slot: &mut f64,
        var_qofdsub_dn5_slot: &mut f64,
        var_qofdsub_dn6_slot: &mut f64,
        var_qofdsub_dn7_slot: &mut f64,
        var_qofdsub_dn8_slot: &mut f64,
        var_qofdsub_dn9_slot: &mut f64,
        var_qofgsub_slot: &mut f64,
        var_qofgsub_db0_slot: &mut f64,
        var_qofgsub_db1_slot: &mut f64,
        var_qofgsub_db10_slot: &mut f64,
        var_qofgsub_db11_slot: &mut f64,
        var_qofgsub_db12_slot: &mut f64,
        var_qofgsub_db13_slot: &mut f64,
        var_qofgsub_db14_slot: &mut f64,
        var_qofgsub_db15_slot: &mut f64,
        var_qofgsub_db16_slot: &mut f64,
        var_qofgsub_db17_slot: &mut f64,
        var_qofgsub_db18_slot: &mut f64,
        var_qofgsub_db19_slot: &mut f64,
        var_qofgsub_db2_slot: &mut f64,
        var_qofgsub_db20_slot: &mut f64,
        var_qofgsub_db21_slot: &mut f64,
        var_qofgsub_db22_slot: &mut f64,
        var_qofgsub_db23_slot: &mut f64,
        var_qofgsub_db24_slot: &mut f64,
        var_qofgsub_db25_slot: &mut f64,
        var_qofgsub_db26_slot: &mut f64,
        var_qofgsub_db27_slot: &mut f64,
        var_qofgsub_db28_slot: &mut f64,
        var_qofgsub_db29_slot: &mut f64,
        var_qofgsub_db3_slot: &mut f64,
        var_qofgsub_db30_slot: &mut f64,
        var_qofgsub_db31_slot: &mut f64,
        var_qofgsub_db32_slot: &mut f64,
        var_qofgsub_db33_slot: &mut f64,
        var_qofgsub_db34_slot: &mut f64,
        var_qofgsub_db35_slot: &mut f64,
        var_qofgsub_db4_slot: &mut f64,
        var_qofgsub_db5_slot: &mut f64,
        var_qofgsub_db6_slot: &mut f64,
        var_qofgsub_db7_slot: &mut f64,
        var_qofgsub_db8_slot: &mut f64,
        var_qofgsub_db9_slot: &mut f64,
        var_qofgsub_dn0_slot: &mut f64,
        var_qofgsub_dn1_slot: &mut f64,
        var_qofgsub_dn10_slot: &mut f64,
        var_qofgsub_dn11_slot: &mut f64,
        var_qofgsub_dn12_slot: &mut f64,
        var_qofgsub_dn13_slot: &mut f64,
        var_qofgsub_dn14_slot: &mut f64,
        var_qofgsub_dn15_slot: &mut f64,
        var_qofgsub_dn16_slot: &mut f64,
        var_qofgsub_dn17_slot: &mut f64,
        var_qofgsub_dn18_slot: &mut f64,
        var_qofgsub_dn19_slot: &mut f64,
        var_qofgsub_dn2_slot: &mut f64,
        var_qofgsub_dn20_slot: &mut f64,
        var_qofgsub_dn21_slot: &mut f64,
        var_qofgsub_dn22_slot: &mut f64,
        var_qofgsub_dn23_slot: &mut f64,
        var_qofgsub_dn24_slot: &mut f64,
        var_qofgsub_dn25_slot: &mut f64,
        var_qofgsub_dn26_slot: &mut f64,
        var_qofgsub_dn27_slot: &mut f64,
        var_qofgsub_dn28_slot: &mut f64,
        var_qofgsub_dn29_slot: &mut f64,
        var_qofgsub_dn3_slot: &mut f64,
        var_qofgsub_dn4_slot: &mut f64,
        var_qofgsub_dn5_slot: &mut f64,
        var_qofgsub_dn6_slot: &mut f64,
        var_qofgsub_dn7_slot: &mut f64,
        var_qofgsub_dn8_slot: &mut f64,
        var_qofgsub_dn9_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let mut var_guard506: f64 = *var_guard506_slot;
        let mut var_guard507: f64 = *var_guard507_slot;
        let mut var_guard508: f64 = *var_guard508_slot;
        let mut var_qofdsub: f64 = *var_qofdsub_slot;
        let mut var_qofdsub_db0: f64 = *var_qofdsub_db0_slot;
        let mut var_qofdsub_db1: f64 = *var_qofdsub_db1_slot;
        let mut var_qofdsub_db10: f64 = *var_qofdsub_db10_slot;
        let mut var_qofdsub_db11: f64 = *var_qofdsub_db11_slot;
        let mut var_qofdsub_db12: f64 = *var_qofdsub_db12_slot;
        let mut var_qofdsub_db13: f64 = *var_qofdsub_db13_slot;
        let mut var_qofdsub_db14: f64 = *var_qofdsub_db14_slot;
        let mut var_qofdsub_db15: f64 = *var_qofdsub_db15_slot;
        let mut var_qofdsub_db16: f64 = *var_qofdsub_db16_slot;
        let mut var_qofdsub_db17: f64 = *var_qofdsub_db17_slot;
        let mut var_qofdsub_db18: f64 = *var_qofdsub_db18_slot;
        let mut var_qofdsub_db19: f64 = *var_qofdsub_db19_slot;
        let mut var_qofdsub_db2: f64 = *var_qofdsub_db2_slot;
        let mut var_qofdsub_db20: f64 = *var_qofdsub_db20_slot;
        let mut var_qofdsub_db21: f64 = *var_qofdsub_db21_slot;
        let mut var_qofdsub_db22: f64 = *var_qofdsub_db22_slot;
        let mut var_qofdsub_db23: f64 = *var_qofdsub_db23_slot;
        let mut var_qofdsub_db24: f64 = *var_qofdsub_db24_slot;
        let mut var_qofdsub_db25: f64 = *var_qofdsub_db25_slot;
        let mut var_qofdsub_db26: f64 = *var_qofdsub_db26_slot;
        let mut var_qofdsub_db27: f64 = *var_qofdsub_db27_slot;
        let mut var_qofdsub_db28: f64 = *var_qofdsub_db28_slot;
        let mut var_qofdsub_db29: f64 = *var_qofdsub_db29_slot;
        let mut var_qofdsub_db3: f64 = *var_qofdsub_db3_slot;
        let mut var_qofdsub_db30: f64 = *var_qofdsub_db30_slot;
        let mut var_qofdsub_db31: f64 = *var_qofdsub_db31_slot;
        let mut var_qofdsub_db32: f64 = *var_qofdsub_db32_slot;
        let mut var_qofdsub_db33: f64 = *var_qofdsub_db33_slot;
        let mut var_qofdsub_db34: f64 = *var_qofdsub_db34_slot;
        let mut var_qofdsub_db35: f64 = *var_qofdsub_db35_slot;
        let mut var_qofdsub_db4: f64 = *var_qofdsub_db4_slot;
        let mut var_qofdsub_db5: f64 = *var_qofdsub_db5_slot;
        let mut var_qofdsub_db6: f64 = *var_qofdsub_db6_slot;
        let mut var_qofdsub_db7: f64 = *var_qofdsub_db7_slot;
        let mut var_qofdsub_db8: f64 = *var_qofdsub_db8_slot;
        let mut var_qofdsub_db9: f64 = *var_qofdsub_db9_slot;
        let mut var_qofdsub_dn0: f64 = *var_qofdsub_dn0_slot;
        let mut var_qofdsub_dn1: f64 = *var_qofdsub_dn1_slot;
        let mut var_qofdsub_dn10: f64 = *var_qofdsub_dn10_slot;
        let mut var_qofdsub_dn11: f64 = *var_qofdsub_dn11_slot;
        let mut var_qofdsub_dn12: f64 = *var_qofdsub_dn12_slot;
        let mut var_qofdsub_dn13: f64 = *var_qofdsub_dn13_slot;
        let mut var_qofdsub_dn14: f64 = *var_qofdsub_dn14_slot;
        let mut var_qofdsub_dn15: f64 = *var_qofdsub_dn15_slot;
        let mut var_qofdsub_dn16: f64 = *var_qofdsub_dn16_slot;
        let mut var_qofdsub_dn17: f64 = *var_qofdsub_dn17_slot;
        let mut var_qofdsub_dn18: f64 = *var_qofdsub_dn18_slot;
        let mut var_qofdsub_dn19: f64 = *var_qofdsub_dn19_slot;
        let mut var_qofdsub_dn2: f64 = *var_qofdsub_dn2_slot;
        let mut var_qofdsub_dn20: f64 = *var_qofdsub_dn20_slot;
        let mut var_qofdsub_dn21: f64 = *var_qofdsub_dn21_slot;
        let mut var_qofdsub_dn22: f64 = *var_qofdsub_dn22_slot;
        let mut var_qofdsub_dn23: f64 = *var_qofdsub_dn23_slot;
        let mut var_qofdsub_dn24: f64 = *var_qofdsub_dn24_slot;
        let mut var_qofdsub_dn25: f64 = *var_qofdsub_dn25_slot;
        let mut var_qofdsub_dn26: f64 = *var_qofdsub_dn26_slot;
        let mut var_qofdsub_dn27: f64 = *var_qofdsub_dn27_slot;
        let mut var_qofdsub_dn28: f64 = *var_qofdsub_dn28_slot;
        let mut var_qofdsub_dn29: f64 = *var_qofdsub_dn29_slot;
        let mut var_qofdsub_dn3: f64 = *var_qofdsub_dn3_slot;
        let mut var_qofdsub_dn4: f64 = *var_qofdsub_dn4_slot;
        let mut var_qofdsub_dn5: f64 = *var_qofdsub_dn5_slot;
        let mut var_qofdsub_dn6: f64 = *var_qofdsub_dn6_slot;
        let mut var_qofdsub_dn7: f64 = *var_qofdsub_dn7_slot;
        let mut var_qofdsub_dn8: f64 = *var_qofdsub_dn8_slot;
        let mut var_qofdsub_dn9: f64 = *var_qofdsub_dn9_slot;
        let mut var_qofgsub: f64 = *var_qofgsub_slot;
        let mut var_qofgsub_db0: f64 = *var_qofgsub_db0_slot;
        let mut var_qofgsub_db1: f64 = *var_qofgsub_db1_slot;
        let mut var_qofgsub_db10: f64 = *var_qofgsub_db10_slot;
        let mut var_qofgsub_db11: f64 = *var_qofgsub_db11_slot;
        let mut var_qofgsub_db12: f64 = *var_qofgsub_db12_slot;
        let mut var_qofgsub_db13: f64 = *var_qofgsub_db13_slot;
        let mut var_qofgsub_db14: f64 = *var_qofgsub_db14_slot;
        let mut var_qofgsub_db15: f64 = *var_qofgsub_db15_slot;
        let mut var_qofgsub_db16: f64 = *var_qofgsub_db16_slot;
        let mut var_qofgsub_db17: f64 = *var_qofgsub_db17_slot;
        let mut var_qofgsub_db18: f64 = *var_qofgsub_db18_slot;
        let mut var_qofgsub_db19: f64 = *var_qofgsub_db19_slot;
        let mut var_qofgsub_db2: f64 = *var_qofgsub_db2_slot;
        let mut var_qofgsub_db20: f64 = *var_qofgsub_db20_slot;
        let mut var_qofgsub_db21: f64 = *var_qofgsub_db21_slot;
        let mut var_qofgsub_db22: f64 = *var_qofgsub_db22_slot;
        let mut var_qofgsub_db23: f64 = *var_qofgsub_db23_slot;
        let mut var_qofgsub_db24: f64 = *var_qofgsub_db24_slot;
        let mut var_qofgsub_db25: f64 = *var_qofgsub_db25_slot;
        let mut var_qofgsub_db26: f64 = *var_qofgsub_db26_slot;
        let mut var_qofgsub_db27: f64 = *var_qofgsub_db27_slot;
        let mut var_qofgsub_db28: f64 = *var_qofgsub_db28_slot;
        let mut var_qofgsub_db29: f64 = *var_qofgsub_db29_slot;
        let mut var_qofgsub_db3: f64 = *var_qofgsub_db3_slot;
        let mut var_qofgsub_db30: f64 = *var_qofgsub_db30_slot;
        let mut var_qofgsub_db31: f64 = *var_qofgsub_db31_slot;
        let mut var_qofgsub_db32: f64 = *var_qofgsub_db32_slot;
        let mut var_qofgsub_db33: f64 = *var_qofgsub_db33_slot;
        let mut var_qofgsub_db34: f64 = *var_qofgsub_db34_slot;
        let mut var_qofgsub_db35: f64 = *var_qofgsub_db35_slot;
        let mut var_qofgsub_db4: f64 = *var_qofgsub_db4_slot;
        let mut var_qofgsub_db5: f64 = *var_qofgsub_db5_slot;
        let mut var_qofgsub_db6: f64 = *var_qofgsub_db6_slot;
        let mut var_qofgsub_db7: f64 = *var_qofgsub_db7_slot;
        let mut var_qofgsub_db8: f64 = *var_qofgsub_db8_slot;
        let mut var_qofgsub_db9: f64 = *var_qofgsub_db9_slot;
        let mut var_qofgsub_dn0: f64 = *var_qofgsub_dn0_slot;
        let mut var_qofgsub_dn1: f64 = *var_qofgsub_dn1_slot;
        let mut var_qofgsub_dn10: f64 = *var_qofgsub_dn10_slot;
        let mut var_qofgsub_dn11: f64 = *var_qofgsub_dn11_slot;
        let mut var_qofgsub_dn12: f64 = *var_qofgsub_dn12_slot;
        let mut var_qofgsub_dn13: f64 = *var_qofgsub_dn13_slot;
        let mut var_qofgsub_dn14: f64 = *var_qofgsub_dn14_slot;
        let mut var_qofgsub_dn15: f64 = *var_qofgsub_dn15_slot;
        let mut var_qofgsub_dn16: f64 = *var_qofgsub_dn16_slot;
        let mut var_qofgsub_dn17: f64 = *var_qofgsub_dn17_slot;
        let mut var_qofgsub_dn18: f64 = *var_qofgsub_dn18_slot;
        let mut var_qofgsub_dn19: f64 = *var_qofgsub_dn19_slot;
        let mut var_qofgsub_dn2: f64 = *var_qofgsub_dn2_slot;
        let mut var_qofgsub_dn20: f64 = *var_qofgsub_dn20_slot;
        let mut var_qofgsub_dn21: f64 = *var_qofgsub_dn21_slot;
        let mut var_qofgsub_dn22: f64 = *var_qofgsub_dn22_slot;
        let mut var_qofgsub_dn23: f64 = *var_qofgsub_dn23_slot;
        let mut var_qofgsub_dn24: f64 = *var_qofgsub_dn24_slot;
        let mut var_qofgsub_dn25: f64 = *var_qofgsub_dn25_slot;
        let mut var_qofgsub_dn26: f64 = *var_qofgsub_dn26_slot;
        let mut var_qofgsub_dn27: f64 = *var_qofgsub_dn27_slot;
        let mut var_qofgsub_dn28: f64 = *var_qofgsub_dn28_slot;
        let mut var_qofgsub_dn29: f64 = *var_qofgsub_dn29_slot;
        let mut var_qofgsub_dn3: f64 = *var_qofgsub_dn3_slot;
        let mut var_qofgsub_dn4: f64 = *var_qofgsub_dn4_slot;
        let mut var_qofgsub_dn5: f64 = *var_qofgsub_dn5_slot;
        let mut var_qofgsub_dn6: f64 = *var_qofgsub_dn6_slot;
        let mut var_qofgsub_dn7: f64 = *var_qofgsub_dn7_slot;
        let mut var_qofgsub_dn8: f64 = *var_qofgsub_dn8_slot;
        let mut var_qofgsub_dn9: f64 = *var_qofgsub_dn9_slot;

        let (assign46370_e45162, assign46370_e45162_d_n0, assign46370_e45162_d_n1, assign46370_e45162_d_n2, assign46370_e45162_d_n3, assign46370_e45162_d_n4, assign46370_e45162_d_n5, assign46370_e45162_d_n6, assign46370_e45162_d_n7, assign46370_e45162_d_n8, assign46370_e45162_d_n9, assign46370_e45162_d_n10, assign46370_e45162_d_n11, assign46370_e45162_d_n12, assign46370_e45162_d_n13, assign46370_e45162_d_n14, assign46370_e45162_d_n15, assign46370_e45162_d_n16, assign46370_e45162_d_n17, assign46370_e45162_d_n18, assign46370_e45162_d_n19, assign46370_e45162_d_n20, assign46370_e45162_d_n21, assign46370_e45162_d_n22, assign46370_e45162_d_n23, assign46370_e45162_d_n24, assign46370_e45162_d_n25, assign46370_e45162_d_n26, assign46370_e45162_d_n27, assign46370_e45162_d_n28, assign46370_e45162_d_n29, assign46370_e45162_d_b0, assign46370_e45162_d_b1, assign46370_e45162_d_b2, assign46370_e45162_d_b3, assign46370_e45162_d_b4, assign46370_e45162_d_b5, assign46370_e45162_d_b6, assign46370_e45162_d_b7, assign46370_e45162_d_b8, assign46370_e45162_d_b9, assign46370_e45162_d_b10, assign46370_e45162_d_b11, assign46370_e45162_d_b12, assign46370_e45162_d_b13, assign46370_e45162_d_b14, assign46370_e45162_d_b15, assign46370_e45162_d_b16, assign46370_e45162_d_b17, assign46370_e45162_d_b18, assign46370_e45162_d_b19, assign46370_e45162_d_b20, assign46370_e45162_d_b21, assign46370_e45162_d_b22, assign46370_e45162_d_b23, assign46370_e45162_d_b24, assign46370_e45162_d_b25, assign46370_e45162_d_b26, assign46370_e45162_d_b27, assign46370_e45162_d_b28, assign46370_e45162_d_b29, assign46370_e45162_d_b30, assign46370_e45162_d_b31, assign46370_e45162_d_b32, assign46370_e45162_d_b33, assign46370_e45162_d_b34, assign46370_e45162_d_b35,) = {
    if (var_guard505 != 0.0) {
        let assign46370_e45150: f64 = (p.p0 * p.p2);
        let assign46370_e45153: f64 = (var_cofdsubmt0 * (nv3 - nv0));
        let assign46370_e45157: f64 = ((nv3 - nv0) - p.p27);
        let assign46370_e45158: f64 = (var_cofdsubmt * assign46370_e45157);
        let assign46370_e45159: f64 = (assign46370_e45153 + assign46370_e45158);
        let assign46370_e45160: f64 = (assign46370_e45150 * assign46370_e45159);
        (assign46370_e45160, (assign46370_e45150 * (((var_cofdsubmt0_dn0 * (nv3 - nv0)) + (-var_cofdsubmt0)) + ((var_cofdsubmt_dn0 * assign46370_e45157) + (-var_cofdsubmt)))), (assign46370_e45150 * ((var_cofdsubmt0_dn1 * (nv3 - nv0)) + (var_cofdsubmt_dn1 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn2 * (nv3 - nv0)) + (var_cofdsubmt_dn2 * assign46370_e45157))), (assign46370_e45150 * (((var_cofdsubmt0_dn3 * (nv3 - nv0)) + var_cofdsubmt0) + ((var_cofdsubmt_dn3 * assign46370_e45157) + var_cofdsubmt))), (assign46370_e45150 * ((var_cofdsubmt0_dn4 * (nv3 - nv0)) + (var_cofdsubmt_dn4 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn5 * (nv3 - nv0)) + (var_cofdsubmt_dn5 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn6 * (nv3 - nv0)) + (var_cofdsubmt_dn6 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn7 * (nv3 - nv0)) + (var_cofdsubmt_dn7 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn8 * (nv3 - nv0)) + (var_cofdsubmt_dn8 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn9 * (nv3 - nv0)) + (var_cofdsubmt_dn9 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn10 * (nv3 - nv0)) + (var_cofdsubmt_dn10 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn11 * (nv3 - nv0)) + (var_cofdsubmt_dn11 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn12 * (nv3 - nv0)) + (var_cofdsubmt_dn12 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn13 * (nv3 - nv0)) + (var_cofdsubmt_dn13 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn14 * (nv3 - nv0)) + (var_cofdsubmt_dn14 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn15 * (nv3 - nv0)) + (var_cofdsubmt_dn15 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn16 * (nv3 - nv0)) + (var_cofdsubmt_dn16 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn17 * (nv3 - nv0)) + (var_cofdsubmt_dn17 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn18 * (nv3 - nv0)) + (var_cofdsubmt_dn18 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn19 * (nv3 - nv0)) + (var_cofdsubmt_dn19 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn20 * (nv3 - nv0)) + (var_cofdsubmt_dn20 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn21 * (nv3 - nv0)) + (var_cofdsubmt_dn21 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn22 * (nv3 - nv0)) + (var_cofdsubmt_dn22 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn23 * (nv3 - nv0)) + (var_cofdsubmt_dn23 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn24 * (nv3 - nv0)) + (var_cofdsubmt_dn24 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn25 * (nv3 - nv0)) + (var_cofdsubmt_dn25 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn26 * (nv3 - nv0)) + (var_cofdsubmt_dn26 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn27 * (nv3 - nv0)) + (var_cofdsubmt_dn27 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn28 * (nv3 - nv0)) + (var_cofdsubmt_dn28 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_dn29 * (nv3 - nv0)) + (var_cofdsubmt_dn29 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db0 * (nv3 - nv0)) + (var_cofdsubmt_db0 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db1 * (nv3 - nv0)) + (var_cofdsubmt_db1 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db2 * (nv3 - nv0)) + (var_cofdsubmt_db2 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db3 * (nv3 - nv0)) + (var_cofdsubmt_db3 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db4 * (nv3 - nv0)) + (var_cofdsubmt_db4 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db5 * (nv3 - nv0)) + (var_cofdsubmt_db5 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db6 * (nv3 - nv0)) + (var_cofdsubmt_db6 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db7 * (nv3 - nv0)) + (var_cofdsubmt_db7 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db8 * (nv3 - nv0)) + (var_cofdsubmt_db8 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db9 * (nv3 - nv0)) + (var_cofdsubmt_db9 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db10 * (nv3 - nv0)) + (var_cofdsubmt_db10 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db11 * (nv3 - nv0)) + (var_cofdsubmt_db11 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db12 * (nv3 - nv0)) + (var_cofdsubmt_db12 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db13 * (nv3 - nv0)) + (var_cofdsubmt_db13 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db14 * (nv3 - nv0)) + (var_cofdsubmt_db14 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db15 * (nv3 - nv0)) + (var_cofdsubmt_db15 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db16 * (nv3 - nv0)) + (var_cofdsubmt_db16 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db17 * (nv3 - nv0)) + (var_cofdsubmt_db17 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db18 * (nv3 - nv0)) + (var_cofdsubmt_db18 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db19 * (nv3 - nv0)) + (var_cofdsubmt_db19 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db20 * (nv3 - nv0)) + (var_cofdsubmt_db20 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db21 * (nv3 - nv0)) + (var_cofdsubmt_db21 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db22 * (nv3 - nv0)) + (var_cofdsubmt_db22 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db23 * (nv3 - nv0)) + (var_cofdsubmt_db23 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db24 * (nv3 - nv0)) + (var_cofdsubmt_db24 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db25 * (nv3 - nv0)) + (var_cofdsubmt_db25 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db26 * (nv3 - nv0)) + (var_cofdsubmt_db26 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db27 * (nv3 - nv0)) + (var_cofdsubmt_db27 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db28 * (nv3 - nv0)) + (var_cofdsubmt_db28 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db29 * (nv3 - nv0)) + (var_cofdsubmt_db29 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db30 * (nv3 - nv0)) + (var_cofdsubmt_db30 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db31 * (nv3 - nv0)) + (var_cofdsubmt_db31 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db32 * (nv3 - nv0)) + (var_cofdsubmt_db32 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db33 * (nv3 - nv0)) + (var_cofdsubmt_db33 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db34 * (nv3 - nv0)) + (var_cofdsubmt_db34 * assign46370_e45157))), (assign46370_e45150 * ((var_cofdsubmt0_db35 * (nv3 - nv0)) + (var_cofdsubmt_db35 * assign46370_e45157))),)
    } else {
        (var_qofdsub, var_qofdsub_dn0, var_qofdsub_dn1, var_qofdsub_dn2, var_qofdsub_dn3, var_qofdsub_dn4, var_qofdsub_dn5, var_qofdsub_dn6, var_qofdsub_dn7, var_qofdsub_dn8, var_qofdsub_dn9, var_qofdsub_dn10, var_qofdsub_dn11, var_qofdsub_dn12, var_qofdsub_dn13, var_qofdsub_dn14, var_qofdsub_dn15, var_qofdsub_dn16, var_qofdsub_dn17, var_qofdsub_dn18, var_qofdsub_dn19, var_qofdsub_dn20, var_qofdsub_dn21, var_qofdsub_dn22, var_qofdsub_dn23, var_qofdsub_dn24, var_qofdsub_dn25, var_qofdsub_dn26, var_qofdsub_dn27, var_qofdsub_dn28, var_qofdsub_dn29, var_qofdsub_db0, var_qofdsub_db1, var_qofdsub_db2, var_qofdsub_db3, var_qofdsub_db4, var_qofdsub_db5, var_qofdsub_db6, var_qofdsub_db7, var_qofdsub_db8, var_qofdsub_db9, var_qofdsub_db10, var_qofdsub_db11, var_qofdsub_db12, var_qofdsub_db13, var_qofdsub_db14, var_qofdsub_db15, var_qofdsub_db16, var_qofdsub_db17, var_qofdsub_db18, var_qofdsub_db19, var_qofdsub_db20, var_qofdsub_db21, var_qofdsub_db22, var_qofdsub_db23, var_qofdsub_db24, var_qofdsub_db25, var_qofdsub_db26, var_qofdsub_db27, var_qofdsub_db28, var_qofdsub_db29, var_qofdsub_db30, var_qofdsub_db31, var_qofdsub_db32, var_qofdsub_db33, var_qofdsub_db34, var_qofdsub_db35,)
    }
};
        var_qofdsub = assign46370_e45162;
        var_qofdsub_dn0 = assign46370_e45162_d_n0;
        var_qofdsub_dn1 = assign46370_e45162_d_n1;
        var_qofdsub_dn2 = assign46370_e45162_d_n2;
        var_qofdsub_dn3 = assign46370_e45162_d_n3;
        var_qofdsub_dn4 = assign46370_e45162_d_n4;
        var_qofdsub_dn5 = assign46370_e45162_d_n5;
        var_qofdsub_dn6 = assign46370_e45162_d_n6;
        var_qofdsub_dn7 = assign46370_e45162_d_n7;
        var_qofdsub_dn8 = assign46370_e45162_d_n8;
        var_qofdsub_dn9 = assign46370_e45162_d_n9;
        var_qofdsub_dn10 = assign46370_e45162_d_n10;
        var_qofdsub_dn11 = assign46370_e45162_d_n11;
        var_qofdsub_dn12 = assign46370_e45162_d_n12;
        var_qofdsub_dn13 = assign46370_e45162_d_n13;
        var_qofdsub_dn14 = assign46370_e45162_d_n14;
        var_qofdsub_dn15 = assign46370_e45162_d_n15;
        var_qofdsub_dn16 = assign46370_e45162_d_n16;
        var_qofdsub_dn17 = assign46370_e45162_d_n17;
        var_qofdsub_dn18 = assign46370_e45162_d_n18;
        var_qofdsub_dn19 = assign46370_e45162_d_n19;
        var_qofdsub_dn20 = assign46370_e45162_d_n20;
        var_qofdsub_dn21 = assign46370_e45162_d_n21;
        var_qofdsub_dn22 = assign46370_e45162_d_n22;
        var_qofdsub_dn23 = assign46370_e45162_d_n23;
        var_qofdsub_dn24 = assign46370_e45162_d_n24;
        var_qofdsub_dn25 = assign46370_e45162_d_n25;
        var_qofdsub_dn26 = assign46370_e45162_d_n26;
        var_qofdsub_dn27 = assign46370_e45162_d_n27;
        var_qofdsub_dn28 = assign46370_e45162_d_n28;
        var_qofdsub_dn29 = assign46370_e45162_d_n29;
        var_qofdsub_db0 = assign46370_e45162_d_b0;
        var_qofdsub_db1 = assign46370_e45162_d_b1;
        var_qofdsub_db2 = assign46370_e45162_d_b2;
        var_qofdsub_db3 = assign46370_e45162_d_b3;
        var_qofdsub_db4 = assign46370_e45162_d_b4;
        var_qofdsub_db5 = assign46370_e45162_d_b5;
        var_qofdsub_db6 = assign46370_e45162_d_b6;
        var_qofdsub_db7 = assign46370_e45162_d_b7;
        var_qofdsub_db8 = assign46370_e45162_d_b8;
        var_qofdsub_db9 = assign46370_e45162_d_b9;
        var_qofdsub_db10 = assign46370_e45162_d_b10;
        var_qofdsub_db11 = assign46370_e45162_d_b11;
        var_qofdsub_db12 = assign46370_e45162_d_b12;
        var_qofdsub_db13 = assign46370_e45162_d_b13;
        var_qofdsub_db14 = assign46370_e45162_d_b14;
        var_qofdsub_db15 = assign46370_e45162_d_b15;
        var_qofdsub_db16 = assign46370_e45162_d_b16;
        var_qofdsub_db17 = assign46370_e45162_d_b17;
        var_qofdsub_db18 = assign46370_e45162_d_b18;
        var_qofdsub_db19 = assign46370_e45162_d_b19;
        var_qofdsub_db20 = assign46370_e45162_d_b20;
        var_qofdsub_db21 = assign46370_e45162_d_b21;
        var_qofdsub_db22 = assign46370_e45162_d_b22;
        var_qofdsub_db23 = assign46370_e45162_d_b23;
        var_qofdsub_db24 = assign46370_e45162_d_b24;
        var_qofdsub_db25 = assign46370_e45162_d_b25;
        var_qofdsub_db26 = assign46370_e45162_d_b26;
        var_qofdsub_db27 = assign46370_e45162_d_b27;
        var_qofdsub_db28 = assign46370_e45162_d_b28;
        var_qofdsub_db29 = assign46370_e45162_d_b29;
        var_qofdsub_db30 = assign46370_e45162_d_b30;
        var_qofdsub_db31 = assign46370_e45162_d_b31;
        var_qofdsub_db32 = assign46370_e45162_d_b32;
        var_qofdsub_db33 = assign46370_e45162_d_b33;
        var_qofdsub_db34 = assign46370_e45162_d_b34;
        var_qofdsub_db35 = assign46370_e45162_d_b35;

        let assign46380_e45165: f64 = ((nv3 - nv0) - p.p27);
        let assign46380_e45167: f64 = (assign46380_e45165 / p.p28);
        let assign46380_e45169: f64 = (-50.0);
        let assign46380_e45170: f64 = if assign46380_e45167 < assign46380_e45169 { 1.0 } else { 0.0 };
        var_guard506 = assign46380_e45170;

        let (assign46390_e45194, assign46390_e45194_d_n0, assign46390_e45194_d_n1, assign46390_e45194_d_n2, assign46390_e45194_d_n3, assign46390_e45194_d_n4, assign46390_e45194_d_n5, assign46390_e45194_d_n6, assign46390_e45194_d_n7, assign46390_e45194_d_n8, assign46390_e45194_d_n9, assign46390_e45194_d_n10, assign46390_e45194_d_n11, assign46390_e45194_d_n12, assign46390_e45194_d_n13, assign46390_e45194_d_n14, assign46390_e45194_d_n15, assign46390_e45194_d_n16, assign46390_e45194_d_n17, assign46390_e45194_d_n18, assign46390_e45194_d_n19, assign46390_e45194_d_n20, assign46390_e45194_d_n21, assign46390_e45194_d_n22, assign46390_e45194_d_n23, assign46390_e45194_d_n24, assign46390_e45194_d_n25, assign46390_e45194_d_n26, assign46390_e45194_d_n27, assign46390_e45194_d_n28, assign46390_e45194_d_n29, assign46390_e45194_d_b0, assign46390_e45194_d_b1, assign46390_e45194_d_b2, assign46390_e45194_d_b3, assign46390_e45194_d_b4, assign46390_e45194_d_b5, assign46390_e45194_d_b6, assign46390_e45194_d_b7, assign46390_e45194_d_b8, assign46390_e45194_d_b9, assign46390_e45194_d_b10, assign46390_e45194_d_b11, assign46390_e45194_d_b12, assign46390_e45194_d_b13, assign46390_e45194_d_b14, assign46390_e45194_d_b15, assign46390_e45194_d_b16, assign46390_e45194_d_b17, assign46390_e45194_d_b18, assign46390_e45194_d_b19, assign46390_e45194_d_b20, assign46390_e45194_d_b21, assign46390_e45194_d_b22, assign46390_e45194_d_b23, assign46390_e45194_d_b24, assign46390_e45194_d_b25, assign46390_e45194_d_b26, assign46390_e45194_d_b27, assign46390_e45194_d_b28, assign46390_e45194_d_b29, assign46390_e45194_d_b30, assign46390_e45194_d_b31, assign46390_e45194_d_b32, assign46390_e45194_d_b33, assign46390_e45194_d_b34, assign46390_e45194_d_b35,) = {
    if ((var_guard505 == 0.0) && (var_guard506 != 0.0)) {
        let assign46390_e45177: f64 = (p.p0 * p.p2);
        let assign46390_e45180: f64 = (var_cofdsubmt0 * (nv3 - nv0));
        let assign46390_e45183: f64 = (var_cofdsubmt * p.p28);
        let assign46390_e45186: f64 = ((nv3 - nv0) - p.p27);
        let assign46390_e45188: f64 = (assign46390_e45186 / p.p28);
        let assign46390_e45189: f64 = (assign46390_e45188).exp();
        let assign46390_e45190: f64 = (assign46390_e45183 * assign46390_e45189);
        let assign46390_e45191: f64 = (assign46390_e45180 + assign46390_e45190);
        let assign46390_e45192: f64 = (assign46390_e45177 * assign46390_e45191);
        (assign46390_e45192, (assign46390_e45177 * (((var_cofdsubmt0_dn0 * (nv3 - nv0)) + (-var_cofdsubmt0)) + (((var_cofdsubmt_dn0 * p.p28) * assign46390_e45189) + (assign46390_e45183 * (assign46390_e45189 * (-1.0 / p.p28)))))), (assign46390_e45177 * ((var_cofdsubmt0_dn1 * (nv3 - nv0)) + ((var_cofdsubmt_dn1 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn2 * (nv3 - nv0)) + ((var_cofdsubmt_dn2 * p.p28) * assign46390_e45189))), (assign46390_e45177 * (((var_cofdsubmt0_dn3 * (nv3 - nv0)) + var_cofdsubmt0) + (((var_cofdsubmt_dn3 * p.p28) * assign46390_e45189) + (assign46390_e45183 * (assign46390_e45189 * (1.0 / p.p28)))))), (assign46390_e45177 * ((var_cofdsubmt0_dn4 * (nv3 - nv0)) + ((var_cofdsubmt_dn4 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn5 * (nv3 - nv0)) + ((var_cofdsubmt_dn5 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn6 * (nv3 - nv0)) + ((var_cofdsubmt_dn6 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn7 * (nv3 - nv0)) + ((var_cofdsubmt_dn7 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn8 * (nv3 - nv0)) + ((var_cofdsubmt_dn8 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn9 * (nv3 - nv0)) + ((var_cofdsubmt_dn9 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn10 * (nv3 - nv0)) + ((var_cofdsubmt_dn10 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn11 * (nv3 - nv0)) + ((var_cofdsubmt_dn11 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn12 * (nv3 - nv0)) + ((var_cofdsubmt_dn12 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn13 * (nv3 - nv0)) + ((var_cofdsubmt_dn13 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn14 * (nv3 - nv0)) + ((var_cofdsubmt_dn14 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn15 * (nv3 - nv0)) + ((var_cofdsubmt_dn15 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn16 * (nv3 - nv0)) + ((var_cofdsubmt_dn16 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn17 * (nv3 - nv0)) + ((var_cofdsubmt_dn17 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn18 * (nv3 - nv0)) + ((var_cofdsubmt_dn18 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn19 * (nv3 - nv0)) + ((var_cofdsubmt_dn19 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn20 * (nv3 - nv0)) + ((var_cofdsubmt_dn20 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn21 * (nv3 - nv0)) + ((var_cofdsubmt_dn21 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn22 * (nv3 - nv0)) + ((var_cofdsubmt_dn22 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn23 * (nv3 - nv0)) + ((var_cofdsubmt_dn23 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn24 * (nv3 - nv0)) + ((var_cofdsubmt_dn24 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn25 * (nv3 - nv0)) + ((var_cofdsubmt_dn25 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn26 * (nv3 - nv0)) + ((var_cofdsubmt_dn26 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn27 * (nv3 - nv0)) + ((var_cofdsubmt_dn27 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn28 * (nv3 - nv0)) + ((var_cofdsubmt_dn28 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_dn29 * (nv3 - nv0)) + ((var_cofdsubmt_dn29 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db0 * (nv3 - nv0)) + ((var_cofdsubmt_db0 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db1 * (nv3 - nv0)) + ((var_cofdsubmt_db1 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db2 * (nv3 - nv0)) + ((var_cofdsubmt_db2 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db3 * (nv3 - nv0)) + ((var_cofdsubmt_db3 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db4 * (nv3 - nv0)) + ((var_cofdsubmt_db4 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db5 * (nv3 - nv0)) + ((var_cofdsubmt_db5 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db6 * (nv3 - nv0)) + ((var_cofdsubmt_db6 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db7 * (nv3 - nv0)) + ((var_cofdsubmt_db7 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db8 * (nv3 - nv0)) + ((var_cofdsubmt_db8 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db9 * (nv3 - nv0)) + ((var_cofdsubmt_db9 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db10 * (nv3 - nv0)) + ((var_cofdsubmt_db10 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db11 * (nv3 - nv0)) + ((var_cofdsubmt_db11 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db12 * (nv3 - nv0)) + ((var_cofdsubmt_db12 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db13 * (nv3 - nv0)) + ((var_cofdsubmt_db13 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db14 * (nv3 - nv0)) + ((var_cofdsubmt_db14 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db15 * (nv3 - nv0)) + ((var_cofdsubmt_db15 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db16 * (nv3 - nv0)) + ((var_cofdsubmt_db16 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db17 * (nv3 - nv0)) + ((var_cofdsubmt_db17 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db18 * (nv3 - nv0)) + ((var_cofdsubmt_db18 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db19 * (nv3 - nv0)) + ((var_cofdsubmt_db19 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db20 * (nv3 - nv0)) + ((var_cofdsubmt_db20 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db21 * (nv3 - nv0)) + ((var_cofdsubmt_db21 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db22 * (nv3 - nv0)) + ((var_cofdsubmt_db22 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db23 * (nv3 - nv0)) + ((var_cofdsubmt_db23 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db24 * (nv3 - nv0)) + ((var_cofdsubmt_db24 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db25 * (nv3 - nv0)) + ((var_cofdsubmt_db25 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db26 * (nv3 - nv0)) + ((var_cofdsubmt_db26 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db27 * (nv3 - nv0)) + ((var_cofdsubmt_db27 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db28 * (nv3 - nv0)) + ((var_cofdsubmt_db28 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db29 * (nv3 - nv0)) + ((var_cofdsubmt_db29 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db30 * (nv3 - nv0)) + ((var_cofdsubmt_db30 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db31 * (nv3 - nv0)) + ((var_cofdsubmt_db31 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db32 * (nv3 - nv0)) + ((var_cofdsubmt_db32 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db33 * (nv3 - nv0)) + ((var_cofdsubmt_db33 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db34 * (nv3 - nv0)) + ((var_cofdsubmt_db34 * p.p28) * assign46390_e45189))), (assign46390_e45177 * ((var_cofdsubmt0_db35 * (nv3 - nv0)) + ((var_cofdsubmt_db35 * p.p28) * assign46390_e45189))),)
    } else {
        (var_qofdsub, var_qofdsub_dn0, var_qofdsub_dn1, var_qofdsub_dn2, var_qofdsub_dn3, var_qofdsub_dn4, var_qofdsub_dn5, var_qofdsub_dn6, var_qofdsub_dn7, var_qofdsub_dn8, var_qofdsub_dn9, var_qofdsub_dn10, var_qofdsub_dn11, var_qofdsub_dn12, var_qofdsub_dn13, var_qofdsub_dn14, var_qofdsub_dn15, var_qofdsub_dn16, var_qofdsub_dn17, var_qofdsub_dn18, var_qofdsub_dn19, var_qofdsub_dn20, var_qofdsub_dn21, var_qofdsub_dn22, var_qofdsub_dn23, var_qofdsub_dn24, var_qofdsub_dn25, var_qofdsub_dn26, var_qofdsub_dn27, var_qofdsub_dn28, var_qofdsub_dn29, var_qofdsub_db0, var_qofdsub_db1, var_qofdsub_db2, var_qofdsub_db3, var_qofdsub_db4, var_qofdsub_db5, var_qofdsub_db6, var_qofdsub_db7, var_qofdsub_db8, var_qofdsub_db9, var_qofdsub_db10, var_qofdsub_db11, var_qofdsub_db12, var_qofdsub_db13, var_qofdsub_db14, var_qofdsub_db15, var_qofdsub_db16, var_qofdsub_db17, var_qofdsub_db18, var_qofdsub_db19, var_qofdsub_db20, var_qofdsub_db21, var_qofdsub_db22, var_qofdsub_db23, var_qofdsub_db24, var_qofdsub_db25, var_qofdsub_db26, var_qofdsub_db27, var_qofdsub_db28, var_qofdsub_db29, var_qofdsub_db30, var_qofdsub_db31, var_qofdsub_db32, var_qofdsub_db33, var_qofdsub_db34, var_qofdsub_db35,)
    }
};
        var_qofdsub = assign46390_e45194;
        var_qofdsub_dn0 = assign46390_e45194_d_n0;
        var_qofdsub_dn1 = assign46390_e45194_d_n1;
        var_qofdsub_dn2 = assign46390_e45194_d_n2;
        var_qofdsub_dn3 = assign46390_e45194_d_n3;
        var_qofdsub_dn4 = assign46390_e45194_d_n4;
        var_qofdsub_dn5 = assign46390_e45194_d_n5;
        var_qofdsub_dn6 = assign46390_e45194_d_n6;
        var_qofdsub_dn7 = assign46390_e45194_d_n7;
        var_qofdsub_dn8 = assign46390_e45194_d_n8;
        var_qofdsub_dn9 = assign46390_e45194_d_n9;
        var_qofdsub_dn10 = assign46390_e45194_d_n10;
        var_qofdsub_dn11 = assign46390_e45194_d_n11;
        var_qofdsub_dn12 = assign46390_e45194_d_n12;
        var_qofdsub_dn13 = assign46390_e45194_d_n13;
        var_qofdsub_dn14 = assign46390_e45194_d_n14;
        var_qofdsub_dn15 = assign46390_e45194_d_n15;
        var_qofdsub_dn16 = assign46390_e45194_d_n16;
        var_qofdsub_dn17 = assign46390_e45194_d_n17;
        var_qofdsub_dn18 = assign46390_e45194_d_n18;
        var_qofdsub_dn19 = assign46390_e45194_d_n19;
        var_qofdsub_dn20 = assign46390_e45194_d_n20;
        var_qofdsub_dn21 = assign46390_e45194_d_n21;
        var_qofdsub_dn22 = assign46390_e45194_d_n22;
        var_qofdsub_dn23 = assign46390_e45194_d_n23;
        var_qofdsub_dn24 = assign46390_e45194_d_n24;
        var_qofdsub_dn25 = assign46390_e45194_d_n25;
        var_qofdsub_dn26 = assign46390_e45194_d_n26;
        var_qofdsub_dn27 = assign46390_e45194_d_n27;
        var_qofdsub_dn28 = assign46390_e45194_d_n28;
        var_qofdsub_dn29 = assign46390_e45194_d_n29;
        var_qofdsub_db0 = assign46390_e45194_d_b0;
        var_qofdsub_db1 = assign46390_e45194_d_b1;
        var_qofdsub_db2 = assign46390_e45194_d_b2;
        var_qofdsub_db3 = assign46390_e45194_d_b3;
        var_qofdsub_db4 = assign46390_e45194_d_b4;
        var_qofdsub_db5 = assign46390_e45194_d_b5;
        var_qofdsub_db6 = assign46390_e45194_d_b6;
        var_qofdsub_db7 = assign46390_e45194_d_b7;
        var_qofdsub_db8 = assign46390_e45194_d_b8;
        var_qofdsub_db9 = assign46390_e45194_d_b9;
        var_qofdsub_db10 = assign46390_e45194_d_b10;
        var_qofdsub_db11 = assign46390_e45194_d_b11;
        var_qofdsub_db12 = assign46390_e45194_d_b12;
        var_qofdsub_db13 = assign46390_e45194_d_b13;
        var_qofdsub_db14 = assign46390_e45194_d_b14;
        var_qofdsub_db15 = assign46390_e45194_d_b15;
        var_qofdsub_db16 = assign46390_e45194_d_b16;
        var_qofdsub_db17 = assign46390_e45194_d_b17;
        var_qofdsub_db18 = assign46390_e45194_d_b18;
        var_qofdsub_db19 = assign46390_e45194_d_b19;
        var_qofdsub_db20 = assign46390_e45194_d_b20;
        var_qofdsub_db21 = assign46390_e45194_d_b21;
        var_qofdsub_db22 = assign46390_e45194_d_b22;
        var_qofdsub_db23 = assign46390_e45194_d_b23;
        var_qofdsub_db24 = assign46390_e45194_d_b24;
        var_qofdsub_db25 = assign46390_e45194_d_b25;
        var_qofdsub_db26 = assign46390_e45194_d_b26;
        var_qofdsub_db27 = assign46390_e45194_d_b27;
        var_qofdsub_db28 = assign46390_e45194_d_b28;
        var_qofdsub_db29 = assign46390_e45194_d_b29;
        var_qofdsub_db30 = assign46390_e45194_d_b30;
        var_qofdsub_db31 = assign46390_e45194_d_b31;
        var_qofdsub_db32 = assign46390_e45194_d_b32;
        var_qofdsub_db33 = assign46390_e45194_d_b33;
        var_qofdsub_db34 = assign46390_e45194_d_b34;
        var_qofdsub_db35 = assign46390_e45194_d_b35;

        let (assign46400_e45222, assign46400_e45222_d_n0, assign46400_e45222_d_n1, assign46400_e45222_d_n2, assign46400_e45222_d_n3, assign46400_e45222_d_n4, assign46400_e45222_d_n5, assign46400_e45222_d_n6, assign46400_e45222_d_n7, assign46400_e45222_d_n8, assign46400_e45222_d_n9, assign46400_e45222_d_n10, assign46400_e45222_d_n11, assign46400_e45222_d_n12, assign46400_e45222_d_n13, assign46400_e45222_d_n14, assign46400_e45222_d_n15, assign46400_e45222_d_n16, assign46400_e45222_d_n17, assign46400_e45222_d_n18, assign46400_e45222_d_n19, assign46400_e45222_d_n20, assign46400_e45222_d_n21, assign46400_e45222_d_n22, assign46400_e45222_d_n23, assign46400_e45222_d_n24, assign46400_e45222_d_n25, assign46400_e45222_d_n26, assign46400_e45222_d_n27, assign46400_e45222_d_n28, assign46400_e45222_d_n29, assign46400_e45222_d_b0, assign46400_e45222_d_b1, assign46400_e45222_d_b2, assign46400_e45222_d_b3, assign46400_e45222_d_b4, assign46400_e45222_d_b5, assign46400_e45222_d_b6, assign46400_e45222_d_b7, assign46400_e45222_d_b8, assign46400_e45222_d_b9, assign46400_e45222_d_b10, assign46400_e45222_d_b11, assign46400_e45222_d_b12, assign46400_e45222_d_b13, assign46400_e45222_d_b14, assign46400_e45222_d_b15, assign46400_e45222_d_b16, assign46400_e45222_d_b17, assign46400_e45222_d_b18, assign46400_e45222_d_b19, assign46400_e45222_d_b20, assign46400_e45222_d_b21, assign46400_e45222_d_b22, assign46400_e45222_d_b23, assign46400_e45222_d_b24, assign46400_e45222_d_b25, assign46400_e45222_d_b26, assign46400_e45222_d_b27, assign46400_e45222_d_b28, assign46400_e45222_d_b29, assign46400_e45222_d_b30, assign46400_e45222_d_b31, assign46400_e45222_d_b32, assign46400_e45222_d_b33, assign46400_e45222_d_b34, assign46400_e45222_d_b35,) = {
    if ((var_guard505 == 0.0) && (var_guard506 == 0.0)) {
        let assign46400_e45202: f64 = (p.p0 * p.p2);
        let assign46400_e45205: f64 = (var_cofdsubmt0 * (nv3 - nv0));
        let assign46400_e45208: f64 = (var_cofdsubmt * p.p28);
        let assign46400_e45212: f64 = ((nv3 - nv0) - p.p27);
        let assign46400_e45214: f64 = (assign46400_e45212 / p.p28);
        let assign46400_e45215: f64 = (assign46400_e45214).exp();
        let assign46400_e45216: f64 = (1.0 + assign46400_e45215);
        let assign46400_e45217: f64 = (assign46400_e45216).ln();
        let assign46400_e45218: f64 = (assign46400_e45208 * assign46400_e45217);
        let assign46400_e45219: f64 = (assign46400_e45205 + assign46400_e45218);
        let assign46400_e45220: f64 = (assign46400_e45202 * assign46400_e45219);
        (assign46400_e45220, (assign46400_e45202 * (((var_cofdsubmt0_dn0 * (nv3 - nv0)) + (-var_cofdsubmt0)) + (((var_cofdsubmt_dn0 * p.p28) * assign46400_e45217) + (assign46400_e45208 * ((assign46400_e45215 * (-1.0 / p.p28)) / assign46400_e45216))))), (assign46400_e45202 * ((var_cofdsubmt0_dn1 * (nv3 - nv0)) + ((var_cofdsubmt_dn1 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn2 * (nv3 - nv0)) + ((var_cofdsubmt_dn2 * p.p28) * assign46400_e45217))), (assign46400_e45202 * (((var_cofdsubmt0_dn3 * (nv3 - nv0)) + var_cofdsubmt0) + (((var_cofdsubmt_dn3 * p.p28) * assign46400_e45217) + (assign46400_e45208 * ((assign46400_e45215 * (1.0 / p.p28)) / assign46400_e45216))))), (assign46400_e45202 * ((var_cofdsubmt0_dn4 * (nv3 - nv0)) + ((var_cofdsubmt_dn4 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn5 * (nv3 - nv0)) + ((var_cofdsubmt_dn5 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn6 * (nv3 - nv0)) + ((var_cofdsubmt_dn6 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn7 * (nv3 - nv0)) + ((var_cofdsubmt_dn7 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn8 * (nv3 - nv0)) + ((var_cofdsubmt_dn8 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn9 * (nv3 - nv0)) + ((var_cofdsubmt_dn9 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn10 * (nv3 - nv0)) + ((var_cofdsubmt_dn10 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn11 * (nv3 - nv0)) + ((var_cofdsubmt_dn11 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn12 * (nv3 - nv0)) + ((var_cofdsubmt_dn12 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn13 * (nv3 - nv0)) + ((var_cofdsubmt_dn13 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn14 * (nv3 - nv0)) + ((var_cofdsubmt_dn14 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn15 * (nv3 - nv0)) + ((var_cofdsubmt_dn15 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn16 * (nv3 - nv0)) + ((var_cofdsubmt_dn16 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn17 * (nv3 - nv0)) + ((var_cofdsubmt_dn17 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn18 * (nv3 - nv0)) + ((var_cofdsubmt_dn18 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn19 * (nv3 - nv0)) + ((var_cofdsubmt_dn19 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn20 * (nv3 - nv0)) + ((var_cofdsubmt_dn20 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn21 * (nv3 - nv0)) + ((var_cofdsubmt_dn21 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn22 * (nv3 - nv0)) + ((var_cofdsubmt_dn22 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn23 * (nv3 - nv0)) + ((var_cofdsubmt_dn23 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn24 * (nv3 - nv0)) + ((var_cofdsubmt_dn24 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn25 * (nv3 - nv0)) + ((var_cofdsubmt_dn25 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn26 * (nv3 - nv0)) + ((var_cofdsubmt_dn26 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn27 * (nv3 - nv0)) + ((var_cofdsubmt_dn27 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn28 * (nv3 - nv0)) + ((var_cofdsubmt_dn28 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_dn29 * (nv3 - nv0)) + ((var_cofdsubmt_dn29 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db0 * (nv3 - nv0)) + ((var_cofdsubmt_db0 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db1 * (nv3 - nv0)) + ((var_cofdsubmt_db1 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db2 * (nv3 - nv0)) + ((var_cofdsubmt_db2 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db3 * (nv3 - nv0)) + ((var_cofdsubmt_db3 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db4 * (nv3 - nv0)) + ((var_cofdsubmt_db4 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db5 * (nv3 - nv0)) + ((var_cofdsubmt_db5 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db6 * (nv3 - nv0)) + ((var_cofdsubmt_db6 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db7 * (nv3 - nv0)) + ((var_cofdsubmt_db7 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db8 * (nv3 - nv0)) + ((var_cofdsubmt_db8 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db9 * (nv3 - nv0)) + ((var_cofdsubmt_db9 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db10 * (nv3 - nv0)) + ((var_cofdsubmt_db10 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db11 * (nv3 - nv0)) + ((var_cofdsubmt_db11 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db12 * (nv3 - nv0)) + ((var_cofdsubmt_db12 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db13 * (nv3 - nv0)) + ((var_cofdsubmt_db13 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db14 * (nv3 - nv0)) + ((var_cofdsubmt_db14 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db15 * (nv3 - nv0)) + ((var_cofdsubmt_db15 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db16 * (nv3 - nv0)) + ((var_cofdsubmt_db16 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db17 * (nv3 - nv0)) + ((var_cofdsubmt_db17 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db18 * (nv3 - nv0)) + ((var_cofdsubmt_db18 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db19 * (nv3 - nv0)) + ((var_cofdsubmt_db19 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db20 * (nv3 - nv0)) + ((var_cofdsubmt_db20 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db21 * (nv3 - nv0)) + ((var_cofdsubmt_db21 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db22 * (nv3 - nv0)) + ((var_cofdsubmt_db22 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db23 * (nv3 - nv0)) + ((var_cofdsubmt_db23 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db24 * (nv3 - nv0)) + ((var_cofdsubmt_db24 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db25 * (nv3 - nv0)) + ((var_cofdsubmt_db25 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db26 * (nv3 - nv0)) + ((var_cofdsubmt_db26 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db27 * (nv3 - nv0)) + ((var_cofdsubmt_db27 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db28 * (nv3 - nv0)) + ((var_cofdsubmt_db28 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db29 * (nv3 - nv0)) + ((var_cofdsubmt_db29 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db30 * (nv3 - nv0)) + ((var_cofdsubmt_db30 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db31 * (nv3 - nv0)) + ((var_cofdsubmt_db31 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db32 * (nv3 - nv0)) + ((var_cofdsubmt_db32 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db33 * (nv3 - nv0)) + ((var_cofdsubmt_db33 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db34 * (nv3 - nv0)) + ((var_cofdsubmt_db34 * p.p28) * assign46400_e45217))), (assign46400_e45202 * ((var_cofdsubmt0_db35 * (nv3 - nv0)) + ((var_cofdsubmt_db35 * p.p28) * assign46400_e45217))),)
    } else {
        (var_qofdsub, var_qofdsub_dn0, var_qofdsub_dn1, var_qofdsub_dn2, var_qofdsub_dn3, var_qofdsub_dn4, var_qofdsub_dn5, var_qofdsub_dn6, var_qofdsub_dn7, var_qofdsub_dn8, var_qofdsub_dn9, var_qofdsub_dn10, var_qofdsub_dn11, var_qofdsub_dn12, var_qofdsub_dn13, var_qofdsub_dn14, var_qofdsub_dn15, var_qofdsub_dn16, var_qofdsub_dn17, var_qofdsub_dn18, var_qofdsub_dn19, var_qofdsub_dn20, var_qofdsub_dn21, var_qofdsub_dn22, var_qofdsub_dn23, var_qofdsub_dn24, var_qofdsub_dn25, var_qofdsub_dn26, var_qofdsub_dn27, var_qofdsub_dn28, var_qofdsub_dn29, var_qofdsub_db0, var_qofdsub_db1, var_qofdsub_db2, var_qofdsub_db3, var_qofdsub_db4, var_qofdsub_db5, var_qofdsub_db6, var_qofdsub_db7, var_qofdsub_db8, var_qofdsub_db9, var_qofdsub_db10, var_qofdsub_db11, var_qofdsub_db12, var_qofdsub_db13, var_qofdsub_db14, var_qofdsub_db15, var_qofdsub_db16, var_qofdsub_db17, var_qofdsub_db18, var_qofdsub_db19, var_qofdsub_db20, var_qofdsub_db21, var_qofdsub_db22, var_qofdsub_db23, var_qofdsub_db24, var_qofdsub_db25, var_qofdsub_db26, var_qofdsub_db27, var_qofdsub_db28, var_qofdsub_db29, var_qofdsub_db30, var_qofdsub_db31, var_qofdsub_db32, var_qofdsub_db33, var_qofdsub_db34, var_qofdsub_db35,)
    }
};
        var_qofdsub = assign46400_e45222;
        var_qofdsub_dn0 = assign46400_e45222_d_n0;
        var_qofdsub_dn1 = assign46400_e45222_d_n1;
        var_qofdsub_dn2 = assign46400_e45222_d_n2;
        var_qofdsub_dn3 = assign46400_e45222_d_n3;
        var_qofdsub_dn4 = assign46400_e45222_d_n4;
        var_qofdsub_dn5 = assign46400_e45222_d_n5;
        var_qofdsub_dn6 = assign46400_e45222_d_n6;
        var_qofdsub_dn7 = assign46400_e45222_d_n7;
        var_qofdsub_dn8 = assign46400_e45222_d_n8;
        var_qofdsub_dn9 = assign46400_e45222_d_n9;
        var_qofdsub_dn10 = assign46400_e45222_d_n10;
        var_qofdsub_dn11 = assign46400_e45222_d_n11;
        var_qofdsub_dn12 = assign46400_e45222_d_n12;
        var_qofdsub_dn13 = assign46400_e45222_d_n13;
        var_qofdsub_dn14 = assign46400_e45222_d_n14;
        var_qofdsub_dn15 = assign46400_e45222_d_n15;
        var_qofdsub_dn16 = assign46400_e45222_d_n16;
        var_qofdsub_dn17 = assign46400_e45222_d_n17;
        var_qofdsub_dn18 = assign46400_e45222_d_n18;
        var_qofdsub_dn19 = assign46400_e45222_d_n19;
        var_qofdsub_dn20 = assign46400_e45222_d_n20;
        var_qofdsub_dn21 = assign46400_e45222_d_n21;
        var_qofdsub_dn22 = assign46400_e45222_d_n22;
        var_qofdsub_dn23 = assign46400_e45222_d_n23;
        var_qofdsub_dn24 = assign46400_e45222_d_n24;
        var_qofdsub_dn25 = assign46400_e45222_d_n25;
        var_qofdsub_dn26 = assign46400_e45222_d_n26;
        var_qofdsub_dn27 = assign46400_e45222_d_n27;
        var_qofdsub_dn28 = assign46400_e45222_d_n28;
        var_qofdsub_dn29 = assign46400_e45222_d_n29;
        var_qofdsub_db0 = assign46400_e45222_d_b0;
        var_qofdsub_db1 = assign46400_e45222_d_b1;
        var_qofdsub_db2 = assign46400_e45222_d_b2;
        var_qofdsub_db3 = assign46400_e45222_d_b3;
        var_qofdsub_db4 = assign46400_e45222_d_b4;
        var_qofdsub_db5 = assign46400_e45222_d_b5;
        var_qofdsub_db6 = assign46400_e45222_d_b6;
        var_qofdsub_db7 = assign46400_e45222_d_b7;
        var_qofdsub_db8 = assign46400_e45222_d_b8;
        var_qofdsub_db9 = assign46400_e45222_d_b9;
        var_qofdsub_db10 = assign46400_e45222_d_b10;
        var_qofdsub_db11 = assign46400_e45222_d_b11;
        var_qofdsub_db12 = assign46400_e45222_d_b12;
        var_qofdsub_db13 = assign46400_e45222_d_b13;
        var_qofdsub_db14 = assign46400_e45222_d_b14;
        var_qofdsub_db15 = assign46400_e45222_d_b15;
        var_qofdsub_db16 = assign46400_e45222_d_b16;
        var_qofdsub_db17 = assign46400_e45222_d_b17;
        var_qofdsub_db18 = assign46400_e45222_d_b18;
        var_qofdsub_db19 = assign46400_e45222_d_b19;
        var_qofdsub_db20 = assign46400_e45222_d_b20;
        var_qofdsub_db21 = assign46400_e45222_d_b21;
        var_qofdsub_db22 = assign46400_e45222_d_b22;
        var_qofdsub_db23 = assign46400_e45222_d_b23;
        var_qofdsub_db24 = assign46400_e45222_d_b24;
        var_qofdsub_db25 = assign46400_e45222_d_b25;
        var_qofdsub_db26 = assign46400_e45222_d_b26;
        var_qofdsub_db27 = assign46400_e45222_d_b27;
        var_qofdsub_db28 = assign46400_e45222_d_b28;
        var_qofdsub_db29 = assign46400_e45222_d_b29;
        var_qofdsub_db30 = assign46400_e45222_d_b30;
        var_qofdsub_db31 = assign46400_e45222_d_b31;
        var_qofdsub_db32 = assign46400_e45222_d_b32;
        var_qofdsub_db33 = assign46400_e45222_d_b33;
        var_qofdsub_db34 = assign46400_e45222_d_b34;
        var_qofdsub_db35 = assign46400_e45222_d_b35;

        let assign46410_e45225: f64 = ((nv6 - nv3) - p.p27);
        let assign46410_e45227: f64 = (assign46410_e45225 / p.p28);
        let assign46410_e45229: f64 = if assign46410_e45227 > 50.0 { 1.0 } else { 0.0 };
        var_guard507 = assign46410_e45229;

        let (assign46420_e45245, assign46420_e45245_d_n0, assign46420_e45245_d_n1, assign46420_e45245_d_n2, assign46420_e45245_d_n3, assign46420_e45245_d_n4, assign46420_e45245_d_n5, assign46420_e45245_d_n6, assign46420_e45245_d_n7, assign46420_e45245_d_n8, assign46420_e45245_d_n9, assign46420_e45245_d_n10, assign46420_e45245_d_n11, assign46420_e45245_d_n12, assign46420_e45245_d_n13, assign46420_e45245_d_n14, assign46420_e45245_d_n15, assign46420_e45245_d_n16, assign46420_e45245_d_n17, assign46420_e45245_d_n18, assign46420_e45245_d_n19, assign46420_e45245_d_n20, assign46420_e45245_d_n21, assign46420_e45245_d_n22, assign46420_e45245_d_n23, assign46420_e45245_d_n24, assign46420_e45245_d_n25, assign46420_e45245_d_n26, assign46420_e45245_d_n27, assign46420_e45245_d_n28, assign46420_e45245_d_n29, assign46420_e45245_d_b0, assign46420_e45245_d_b1, assign46420_e45245_d_b2, assign46420_e45245_d_b3, assign46420_e45245_d_b4, assign46420_e45245_d_b5, assign46420_e45245_d_b6, assign46420_e45245_d_b7, assign46420_e45245_d_b8, assign46420_e45245_d_b9, assign46420_e45245_d_b10, assign46420_e45245_d_b11, assign46420_e45245_d_b12, assign46420_e45245_d_b13, assign46420_e45245_d_b14, assign46420_e45245_d_b15, assign46420_e45245_d_b16, assign46420_e45245_d_b17, assign46420_e45245_d_b18, assign46420_e45245_d_b19, assign46420_e45245_d_b20, assign46420_e45245_d_b21, assign46420_e45245_d_b22, assign46420_e45245_d_b23, assign46420_e45245_d_b24, assign46420_e45245_d_b25, assign46420_e45245_d_b26, assign46420_e45245_d_b27, assign46420_e45245_d_b28, assign46420_e45245_d_b29, assign46420_e45245_d_b30, assign46420_e45245_d_b31, assign46420_e45245_d_b32, assign46420_e45245_d_b33, assign46420_e45245_d_b34, assign46420_e45245_d_b35,) = {
    if (var_guard507 != 0.0) {
        let assign46420_e45233: f64 = (p.p0 * p.p2);
        let assign46420_e45236: f64 = (var_cofgsubmt0 * (nv6 - nv3));
        let assign46420_e45240: f64 = ((nv6 - nv3) - p.p27);
        let assign46420_e45241: f64 = (var_cofgsubmt * assign46420_e45240);
        let assign46420_e45242: f64 = (assign46420_e45236 + assign46420_e45241);
        let assign46420_e45243: f64 = (assign46420_e45233 * assign46420_e45242);
        (assign46420_e45243, (assign46420_e45233 * ((var_cofgsubmt0_dn0 * (nv6 - nv3)) + (var_cofgsubmt_dn0 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn1 * (nv6 - nv3)) + (var_cofgsubmt_dn1 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn2 * (nv6 - nv3)) + (var_cofgsubmt_dn2 * assign46420_e45240))), (assign46420_e45233 * (((var_cofgsubmt0_dn3 * (nv6 - nv3)) + (-var_cofgsubmt0)) + ((var_cofgsubmt_dn3 * assign46420_e45240) + (-var_cofgsubmt)))), (assign46420_e45233 * ((var_cofgsubmt0_dn4 * (nv6 - nv3)) + (var_cofgsubmt_dn4 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn5 * (nv6 - nv3)) + (var_cofgsubmt_dn5 * assign46420_e45240))), (assign46420_e45233 * (((var_cofgsubmt0_dn6 * (nv6 - nv3)) + var_cofgsubmt0) + ((var_cofgsubmt_dn6 * assign46420_e45240) + var_cofgsubmt))), (assign46420_e45233 * ((var_cofgsubmt0_dn7 * (nv6 - nv3)) + (var_cofgsubmt_dn7 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn8 * (nv6 - nv3)) + (var_cofgsubmt_dn8 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn9 * (nv6 - nv3)) + (var_cofgsubmt_dn9 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn10 * (nv6 - nv3)) + (var_cofgsubmt_dn10 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn11 * (nv6 - nv3)) + (var_cofgsubmt_dn11 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn12 * (nv6 - nv3)) + (var_cofgsubmt_dn12 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn13 * (nv6 - nv3)) + (var_cofgsubmt_dn13 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn14 * (nv6 - nv3)) + (var_cofgsubmt_dn14 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn15 * (nv6 - nv3)) + (var_cofgsubmt_dn15 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn16 * (nv6 - nv3)) + (var_cofgsubmt_dn16 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn17 * (nv6 - nv3)) + (var_cofgsubmt_dn17 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn18 * (nv6 - nv3)) + (var_cofgsubmt_dn18 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn19 * (nv6 - nv3)) + (var_cofgsubmt_dn19 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn20 * (nv6 - nv3)) + (var_cofgsubmt_dn20 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn21 * (nv6 - nv3)) + (var_cofgsubmt_dn21 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn22 * (nv6 - nv3)) + (var_cofgsubmt_dn22 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn23 * (nv6 - nv3)) + (var_cofgsubmt_dn23 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn24 * (nv6 - nv3)) + (var_cofgsubmt_dn24 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn25 * (nv6 - nv3)) + (var_cofgsubmt_dn25 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn26 * (nv6 - nv3)) + (var_cofgsubmt_dn26 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn27 * (nv6 - nv3)) + (var_cofgsubmt_dn27 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn28 * (nv6 - nv3)) + (var_cofgsubmt_dn28 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_dn29 * (nv6 - nv3)) + (var_cofgsubmt_dn29 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db0 * (nv6 - nv3)) + (var_cofgsubmt_db0 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db1 * (nv6 - nv3)) + (var_cofgsubmt_db1 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db2 * (nv6 - nv3)) + (var_cofgsubmt_db2 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db3 * (nv6 - nv3)) + (var_cofgsubmt_db3 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db4 * (nv6 - nv3)) + (var_cofgsubmt_db4 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db5 * (nv6 - nv3)) + (var_cofgsubmt_db5 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db6 * (nv6 - nv3)) + (var_cofgsubmt_db6 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db7 * (nv6 - nv3)) + (var_cofgsubmt_db7 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db8 * (nv6 - nv3)) + (var_cofgsubmt_db8 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db9 * (nv6 - nv3)) + (var_cofgsubmt_db9 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db10 * (nv6 - nv3)) + (var_cofgsubmt_db10 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db11 * (nv6 - nv3)) + (var_cofgsubmt_db11 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db12 * (nv6 - nv3)) + (var_cofgsubmt_db12 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db13 * (nv6 - nv3)) + (var_cofgsubmt_db13 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db14 * (nv6 - nv3)) + (var_cofgsubmt_db14 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db15 * (nv6 - nv3)) + (var_cofgsubmt_db15 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db16 * (nv6 - nv3)) + (var_cofgsubmt_db16 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db17 * (nv6 - nv3)) + (var_cofgsubmt_db17 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db18 * (nv6 - nv3)) + (var_cofgsubmt_db18 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db19 * (nv6 - nv3)) + (var_cofgsubmt_db19 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db20 * (nv6 - nv3)) + (var_cofgsubmt_db20 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db21 * (nv6 - nv3)) + (var_cofgsubmt_db21 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db22 * (nv6 - nv3)) + (var_cofgsubmt_db22 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db23 * (nv6 - nv3)) + (var_cofgsubmt_db23 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db24 * (nv6 - nv3)) + (var_cofgsubmt_db24 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db25 * (nv6 - nv3)) + (var_cofgsubmt_db25 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db26 * (nv6 - nv3)) + (var_cofgsubmt_db26 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db27 * (nv6 - nv3)) + (var_cofgsubmt_db27 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db28 * (nv6 - nv3)) + (var_cofgsubmt_db28 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db29 * (nv6 - nv3)) + (var_cofgsubmt_db29 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db30 * (nv6 - nv3)) + (var_cofgsubmt_db30 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db31 * (nv6 - nv3)) + (var_cofgsubmt_db31 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db32 * (nv6 - nv3)) + (var_cofgsubmt_db32 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db33 * (nv6 - nv3)) + (var_cofgsubmt_db33 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db34 * (nv6 - nv3)) + (var_cofgsubmt_db34 * assign46420_e45240))), (assign46420_e45233 * ((var_cofgsubmt0_db35 * (nv6 - nv3)) + (var_cofgsubmt_db35 * assign46420_e45240))),)
    } else {
        (var_qofgsub, var_qofgsub_dn0, var_qofgsub_dn1, var_qofgsub_dn2, var_qofgsub_dn3, var_qofgsub_dn4, var_qofgsub_dn5, var_qofgsub_dn6, var_qofgsub_dn7, var_qofgsub_dn8, var_qofgsub_dn9, var_qofgsub_dn10, var_qofgsub_dn11, var_qofgsub_dn12, var_qofgsub_dn13, var_qofgsub_dn14, var_qofgsub_dn15, var_qofgsub_dn16, var_qofgsub_dn17, var_qofgsub_dn18, var_qofgsub_dn19, var_qofgsub_dn20, var_qofgsub_dn21, var_qofgsub_dn22, var_qofgsub_dn23, var_qofgsub_dn24, var_qofgsub_dn25, var_qofgsub_dn26, var_qofgsub_dn27, var_qofgsub_dn28, var_qofgsub_dn29, var_qofgsub_db0, var_qofgsub_db1, var_qofgsub_db2, var_qofgsub_db3, var_qofgsub_db4, var_qofgsub_db5, var_qofgsub_db6, var_qofgsub_db7, var_qofgsub_db8, var_qofgsub_db9, var_qofgsub_db10, var_qofgsub_db11, var_qofgsub_db12, var_qofgsub_db13, var_qofgsub_db14, var_qofgsub_db15, var_qofgsub_db16, var_qofgsub_db17, var_qofgsub_db18, var_qofgsub_db19, var_qofgsub_db20, var_qofgsub_db21, var_qofgsub_db22, var_qofgsub_db23, var_qofgsub_db24, var_qofgsub_db25, var_qofgsub_db26, var_qofgsub_db27, var_qofgsub_db28, var_qofgsub_db29, var_qofgsub_db30, var_qofgsub_db31, var_qofgsub_db32, var_qofgsub_db33, var_qofgsub_db34, var_qofgsub_db35,)
    }
};
        var_qofgsub = assign46420_e45245;
        var_qofgsub_dn0 = assign46420_e45245_d_n0;
        var_qofgsub_dn1 = assign46420_e45245_d_n1;
        var_qofgsub_dn2 = assign46420_e45245_d_n2;
        var_qofgsub_dn3 = assign46420_e45245_d_n3;
        var_qofgsub_dn4 = assign46420_e45245_d_n4;
        var_qofgsub_dn5 = assign46420_e45245_d_n5;
        var_qofgsub_dn6 = assign46420_e45245_d_n6;
        var_qofgsub_dn7 = assign46420_e45245_d_n7;
        var_qofgsub_dn8 = assign46420_e45245_d_n8;
        var_qofgsub_dn9 = assign46420_e45245_d_n9;
        var_qofgsub_dn10 = assign46420_e45245_d_n10;
        var_qofgsub_dn11 = assign46420_e45245_d_n11;
        var_qofgsub_dn12 = assign46420_e45245_d_n12;
        var_qofgsub_dn13 = assign46420_e45245_d_n13;
        var_qofgsub_dn14 = assign46420_e45245_d_n14;
        var_qofgsub_dn15 = assign46420_e45245_d_n15;
        var_qofgsub_dn16 = assign46420_e45245_d_n16;
        var_qofgsub_dn17 = assign46420_e45245_d_n17;
        var_qofgsub_dn18 = assign46420_e45245_d_n18;
        var_qofgsub_dn19 = assign46420_e45245_d_n19;
        var_qofgsub_dn20 = assign46420_e45245_d_n20;
        var_qofgsub_dn21 = assign46420_e45245_d_n21;
        var_qofgsub_dn22 = assign46420_e45245_d_n22;
        var_qofgsub_dn23 = assign46420_e45245_d_n23;
        var_qofgsub_dn24 = assign46420_e45245_d_n24;
        var_qofgsub_dn25 = assign46420_e45245_d_n25;
        var_qofgsub_dn26 = assign46420_e45245_d_n26;
        var_qofgsub_dn27 = assign46420_e45245_d_n27;
        var_qofgsub_dn28 = assign46420_e45245_d_n28;
        var_qofgsub_dn29 = assign46420_e45245_d_n29;
        var_qofgsub_db0 = assign46420_e45245_d_b0;
        var_qofgsub_db1 = assign46420_e45245_d_b1;
        var_qofgsub_db2 = assign46420_e45245_d_b2;
        var_qofgsub_db3 = assign46420_e45245_d_b3;
        var_qofgsub_db4 = assign46420_e45245_d_b4;
        var_qofgsub_db5 = assign46420_e45245_d_b5;
        var_qofgsub_db6 = assign46420_e45245_d_b6;
        var_qofgsub_db7 = assign46420_e45245_d_b7;
        var_qofgsub_db8 = assign46420_e45245_d_b8;
        var_qofgsub_db9 = assign46420_e45245_d_b9;
        var_qofgsub_db10 = assign46420_e45245_d_b10;
        var_qofgsub_db11 = assign46420_e45245_d_b11;
        var_qofgsub_db12 = assign46420_e45245_d_b12;
        var_qofgsub_db13 = assign46420_e45245_d_b13;
        var_qofgsub_db14 = assign46420_e45245_d_b14;
        var_qofgsub_db15 = assign46420_e45245_d_b15;
        var_qofgsub_db16 = assign46420_e45245_d_b16;
        var_qofgsub_db17 = assign46420_e45245_d_b17;
        var_qofgsub_db18 = assign46420_e45245_d_b18;
        var_qofgsub_db19 = assign46420_e45245_d_b19;
        var_qofgsub_db20 = assign46420_e45245_d_b20;
        var_qofgsub_db21 = assign46420_e45245_d_b21;
        var_qofgsub_db22 = assign46420_e45245_d_b22;
        var_qofgsub_db23 = assign46420_e45245_d_b23;
        var_qofgsub_db24 = assign46420_e45245_d_b24;
        var_qofgsub_db25 = assign46420_e45245_d_b25;
        var_qofgsub_db26 = assign46420_e45245_d_b26;
        var_qofgsub_db27 = assign46420_e45245_d_b27;
        var_qofgsub_db28 = assign46420_e45245_d_b28;
        var_qofgsub_db29 = assign46420_e45245_d_b29;
        var_qofgsub_db30 = assign46420_e45245_d_b30;
        var_qofgsub_db31 = assign46420_e45245_d_b31;
        var_qofgsub_db32 = assign46420_e45245_d_b32;
        var_qofgsub_db33 = assign46420_e45245_d_b33;
        var_qofgsub_db34 = assign46420_e45245_d_b34;
        var_qofgsub_db35 = assign46420_e45245_d_b35;

        let assign46430_e45248: f64 = ((nv6 - nv3) - p.p27);
        let assign46430_e45250: f64 = (assign46430_e45248 / p.p28);
        let assign46430_e45252: f64 = (-50.0);
        let assign46430_e45253: f64 = if assign46430_e45250 < assign46430_e45252 { 1.0 } else { 0.0 };
        var_guard508 = assign46430_e45253;

        let (assign46440_e45277, assign46440_e45277_d_n0, assign46440_e45277_d_n1, assign46440_e45277_d_n2, assign46440_e45277_d_n3, assign46440_e45277_d_n4, assign46440_e45277_d_n5, assign46440_e45277_d_n6, assign46440_e45277_d_n7, assign46440_e45277_d_n8, assign46440_e45277_d_n9, assign46440_e45277_d_n10, assign46440_e45277_d_n11, assign46440_e45277_d_n12, assign46440_e45277_d_n13, assign46440_e45277_d_n14, assign46440_e45277_d_n15, assign46440_e45277_d_n16, assign46440_e45277_d_n17, assign46440_e45277_d_n18, assign46440_e45277_d_n19, assign46440_e45277_d_n20, assign46440_e45277_d_n21, assign46440_e45277_d_n22, assign46440_e45277_d_n23, assign46440_e45277_d_n24, assign46440_e45277_d_n25, assign46440_e45277_d_n26, assign46440_e45277_d_n27, assign46440_e45277_d_n28, assign46440_e45277_d_n29, assign46440_e45277_d_b0, assign46440_e45277_d_b1, assign46440_e45277_d_b2, assign46440_e45277_d_b3, assign46440_e45277_d_b4, assign46440_e45277_d_b5, assign46440_e45277_d_b6, assign46440_e45277_d_b7, assign46440_e45277_d_b8, assign46440_e45277_d_b9, assign46440_e45277_d_b10, assign46440_e45277_d_b11, assign46440_e45277_d_b12, assign46440_e45277_d_b13, assign46440_e45277_d_b14, assign46440_e45277_d_b15, assign46440_e45277_d_b16, assign46440_e45277_d_b17, assign46440_e45277_d_b18, assign46440_e45277_d_b19, assign46440_e45277_d_b20, assign46440_e45277_d_b21, assign46440_e45277_d_b22, assign46440_e45277_d_b23, assign46440_e45277_d_b24, assign46440_e45277_d_b25, assign46440_e45277_d_b26, assign46440_e45277_d_b27, assign46440_e45277_d_b28, assign46440_e45277_d_b29, assign46440_e45277_d_b30, assign46440_e45277_d_b31, assign46440_e45277_d_b32, assign46440_e45277_d_b33, assign46440_e45277_d_b34, assign46440_e45277_d_b35,) = {
    if ((var_guard507 == 0.0) && (var_guard508 != 0.0)) {
        let assign46440_e45260: f64 = (p.p0 * p.p2);
        let assign46440_e45263: f64 = (var_cofgsubmt0 * (nv6 - nv3));
        let assign46440_e45266: f64 = (var_cofgsubmt * p.p28);
        let assign46440_e45269: f64 = ((nv6 - nv3) - p.p27);
        let assign46440_e45271: f64 = (assign46440_e45269 / p.p28);
        let assign46440_e45272: f64 = (assign46440_e45271).exp();
        let assign46440_e45273: f64 = (assign46440_e45266 * assign46440_e45272);
        let assign46440_e45274: f64 = (assign46440_e45263 + assign46440_e45273);
        let assign46440_e45275: f64 = (assign46440_e45260 * assign46440_e45274);
        (assign46440_e45275, (assign46440_e45260 * ((var_cofgsubmt0_dn0 * (nv6 - nv3)) + ((var_cofgsubmt_dn0 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn1 * (nv6 - nv3)) + ((var_cofgsubmt_dn1 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn2 * (nv6 - nv3)) + ((var_cofgsubmt_dn2 * p.p28) * assign46440_e45272))), (assign46440_e45260 * (((var_cofgsubmt0_dn3 * (nv6 - nv3)) + (-var_cofgsubmt0)) + (((var_cofgsubmt_dn3 * p.p28) * assign46440_e45272) + (assign46440_e45266 * (assign46440_e45272 * (-1.0 / p.p28)))))), (assign46440_e45260 * ((var_cofgsubmt0_dn4 * (nv6 - nv3)) + ((var_cofgsubmt_dn4 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn5 * (nv6 - nv3)) + ((var_cofgsubmt_dn5 * p.p28) * assign46440_e45272))), (assign46440_e45260 * (((var_cofgsubmt0_dn6 * (nv6 - nv3)) + var_cofgsubmt0) + (((var_cofgsubmt_dn6 * p.p28) * assign46440_e45272) + (assign46440_e45266 * (assign46440_e45272 * (1.0 / p.p28)))))), (assign46440_e45260 * ((var_cofgsubmt0_dn7 * (nv6 - nv3)) + ((var_cofgsubmt_dn7 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn8 * (nv6 - nv3)) + ((var_cofgsubmt_dn8 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn9 * (nv6 - nv3)) + ((var_cofgsubmt_dn9 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn10 * (nv6 - nv3)) + ((var_cofgsubmt_dn10 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn11 * (nv6 - nv3)) + ((var_cofgsubmt_dn11 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn12 * (nv6 - nv3)) + ((var_cofgsubmt_dn12 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn13 * (nv6 - nv3)) + ((var_cofgsubmt_dn13 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn14 * (nv6 - nv3)) + ((var_cofgsubmt_dn14 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn15 * (nv6 - nv3)) + ((var_cofgsubmt_dn15 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn16 * (nv6 - nv3)) + ((var_cofgsubmt_dn16 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn17 * (nv6 - nv3)) + ((var_cofgsubmt_dn17 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn18 * (nv6 - nv3)) + ((var_cofgsubmt_dn18 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn19 * (nv6 - nv3)) + ((var_cofgsubmt_dn19 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn20 * (nv6 - nv3)) + ((var_cofgsubmt_dn20 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn21 * (nv6 - nv3)) + ((var_cofgsubmt_dn21 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn22 * (nv6 - nv3)) + ((var_cofgsubmt_dn22 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn23 * (nv6 - nv3)) + ((var_cofgsubmt_dn23 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn24 * (nv6 - nv3)) + ((var_cofgsubmt_dn24 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn25 * (nv6 - nv3)) + ((var_cofgsubmt_dn25 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn26 * (nv6 - nv3)) + ((var_cofgsubmt_dn26 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn27 * (nv6 - nv3)) + ((var_cofgsubmt_dn27 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn28 * (nv6 - nv3)) + ((var_cofgsubmt_dn28 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_dn29 * (nv6 - nv3)) + ((var_cofgsubmt_dn29 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db0 * (nv6 - nv3)) + ((var_cofgsubmt_db0 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db1 * (nv6 - nv3)) + ((var_cofgsubmt_db1 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db2 * (nv6 - nv3)) + ((var_cofgsubmt_db2 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db3 * (nv6 - nv3)) + ((var_cofgsubmt_db3 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db4 * (nv6 - nv3)) + ((var_cofgsubmt_db4 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db5 * (nv6 - nv3)) + ((var_cofgsubmt_db5 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db6 * (nv6 - nv3)) + ((var_cofgsubmt_db6 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db7 * (nv6 - nv3)) + ((var_cofgsubmt_db7 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db8 * (nv6 - nv3)) + ((var_cofgsubmt_db8 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db9 * (nv6 - nv3)) + ((var_cofgsubmt_db9 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db10 * (nv6 - nv3)) + ((var_cofgsubmt_db10 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db11 * (nv6 - nv3)) + ((var_cofgsubmt_db11 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db12 * (nv6 - nv3)) + ((var_cofgsubmt_db12 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db13 * (nv6 - nv3)) + ((var_cofgsubmt_db13 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db14 * (nv6 - nv3)) + ((var_cofgsubmt_db14 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db15 * (nv6 - nv3)) + ((var_cofgsubmt_db15 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db16 * (nv6 - nv3)) + ((var_cofgsubmt_db16 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db17 * (nv6 - nv3)) + ((var_cofgsubmt_db17 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db18 * (nv6 - nv3)) + ((var_cofgsubmt_db18 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db19 * (nv6 - nv3)) + ((var_cofgsubmt_db19 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db20 * (nv6 - nv3)) + ((var_cofgsubmt_db20 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db21 * (nv6 - nv3)) + ((var_cofgsubmt_db21 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db22 * (nv6 - nv3)) + ((var_cofgsubmt_db22 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db23 * (nv6 - nv3)) + ((var_cofgsubmt_db23 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db24 * (nv6 - nv3)) + ((var_cofgsubmt_db24 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db25 * (nv6 - nv3)) + ((var_cofgsubmt_db25 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db26 * (nv6 - nv3)) + ((var_cofgsubmt_db26 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db27 * (nv6 - nv3)) + ((var_cofgsubmt_db27 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db28 * (nv6 - nv3)) + ((var_cofgsubmt_db28 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db29 * (nv6 - nv3)) + ((var_cofgsubmt_db29 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db30 * (nv6 - nv3)) + ((var_cofgsubmt_db30 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db31 * (nv6 - nv3)) + ((var_cofgsubmt_db31 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db32 * (nv6 - nv3)) + ((var_cofgsubmt_db32 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db33 * (nv6 - nv3)) + ((var_cofgsubmt_db33 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db34 * (nv6 - nv3)) + ((var_cofgsubmt_db34 * p.p28) * assign46440_e45272))), (assign46440_e45260 * ((var_cofgsubmt0_db35 * (nv6 - nv3)) + ((var_cofgsubmt_db35 * p.p28) * assign46440_e45272))),)
    } else {
        (var_qofgsub, var_qofgsub_dn0, var_qofgsub_dn1, var_qofgsub_dn2, var_qofgsub_dn3, var_qofgsub_dn4, var_qofgsub_dn5, var_qofgsub_dn6, var_qofgsub_dn7, var_qofgsub_dn8, var_qofgsub_dn9, var_qofgsub_dn10, var_qofgsub_dn11, var_qofgsub_dn12, var_qofgsub_dn13, var_qofgsub_dn14, var_qofgsub_dn15, var_qofgsub_dn16, var_qofgsub_dn17, var_qofgsub_dn18, var_qofgsub_dn19, var_qofgsub_dn20, var_qofgsub_dn21, var_qofgsub_dn22, var_qofgsub_dn23, var_qofgsub_dn24, var_qofgsub_dn25, var_qofgsub_dn26, var_qofgsub_dn27, var_qofgsub_dn28, var_qofgsub_dn29, var_qofgsub_db0, var_qofgsub_db1, var_qofgsub_db2, var_qofgsub_db3, var_qofgsub_db4, var_qofgsub_db5, var_qofgsub_db6, var_qofgsub_db7, var_qofgsub_db8, var_qofgsub_db9, var_qofgsub_db10, var_qofgsub_db11, var_qofgsub_db12, var_qofgsub_db13, var_qofgsub_db14, var_qofgsub_db15, var_qofgsub_db16, var_qofgsub_db17, var_qofgsub_db18, var_qofgsub_db19, var_qofgsub_db20, var_qofgsub_db21, var_qofgsub_db22, var_qofgsub_db23, var_qofgsub_db24, var_qofgsub_db25, var_qofgsub_db26, var_qofgsub_db27, var_qofgsub_db28, var_qofgsub_db29, var_qofgsub_db30, var_qofgsub_db31, var_qofgsub_db32, var_qofgsub_db33, var_qofgsub_db34, var_qofgsub_db35,)
    }
};
        var_qofgsub = assign46440_e45277;
        var_qofgsub_dn0 = assign46440_e45277_d_n0;
        var_qofgsub_dn1 = assign46440_e45277_d_n1;
        var_qofgsub_dn2 = assign46440_e45277_d_n2;
        var_qofgsub_dn3 = assign46440_e45277_d_n3;
        var_qofgsub_dn4 = assign46440_e45277_d_n4;
        var_qofgsub_dn5 = assign46440_e45277_d_n5;
        var_qofgsub_dn6 = assign46440_e45277_d_n6;
        var_qofgsub_dn7 = assign46440_e45277_d_n7;
        var_qofgsub_dn8 = assign46440_e45277_d_n8;
        var_qofgsub_dn9 = assign46440_e45277_d_n9;
        var_qofgsub_dn10 = assign46440_e45277_d_n10;
        var_qofgsub_dn11 = assign46440_e45277_d_n11;
        var_qofgsub_dn12 = assign46440_e45277_d_n12;
        var_qofgsub_dn13 = assign46440_e45277_d_n13;
        var_qofgsub_dn14 = assign46440_e45277_d_n14;
        var_qofgsub_dn15 = assign46440_e45277_d_n15;
        var_qofgsub_dn16 = assign46440_e45277_d_n16;
        var_qofgsub_dn17 = assign46440_e45277_d_n17;
        var_qofgsub_dn18 = assign46440_e45277_d_n18;
        var_qofgsub_dn19 = assign46440_e45277_d_n19;
        var_qofgsub_dn20 = assign46440_e45277_d_n20;
        var_qofgsub_dn21 = assign46440_e45277_d_n21;
        var_qofgsub_dn22 = assign46440_e45277_d_n22;
        var_qofgsub_dn23 = assign46440_e45277_d_n23;
        var_qofgsub_dn24 = assign46440_e45277_d_n24;
        var_qofgsub_dn25 = assign46440_e45277_d_n25;
        var_qofgsub_dn26 = assign46440_e45277_d_n26;
        var_qofgsub_dn27 = assign46440_e45277_d_n27;
        var_qofgsub_dn28 = assign46440_e45277_d_n28;
        var_qofgsub_dn29 = assign46440_e45277_d_n29;
        var_qofgsub_db0 = assign46440_e45277_d_b0;
        var_qofgsub_db1 = assign46440_e45277_d_b1;
        var_qofgsub_db2 = assign46440_e45277_d_b2;
        var_qofgsub_db3 = assign46440_e45277_d_b3;
        var_qofgsub_db4 = assign46440_e45277_d_b4;
        var_qofgsub_db5 = assign46440_e45277_d_b5;
        var_qofgsub_db6 = assign46440_e45277_d_b6;
        var_qofgsub_db7 = assign46440_e45277_d_b7;
        var_qofgsub_db8 = assign46440_e45277_d_b8;
        var_qofgsub_db9 = assign46440_e45277_d_b9;
        var_qofgsub_db10 = assign46440_e45277_d_b10;
        var_qofgsub_db11 = assign46440_e45277_d_b11;
        var_qofgsub_db12 = assign46440_e45277_d_b12;
        var_qofgsub_db13 = assign46440_e45277_d_b13;
        var_qofgsub_db14 = assign46440_e45277_d_b14;
        var_qofgsub_db15 = assign46440_e45277_d_b15;
        var_qofgsub_db16 = assign46440_e45277_d_b16;
        var_qofgsub_db17 = assign46440_e45277_d_b17;
        var_qofgsub_db18 = assign46440_e45277_d_b18;
        var_qofgsub_db19 = assign46440_e45277_d_b19;
        var_qofgsub_db20 = assign46440_e45277_d_b20;
        var_qofgsub_db21 = assign46440_e45277_d_b21;
        var_qofgsub_db22 = assign46440_e45277_d_b22;
        var_qofgsub_db23 = assign46440_e45277_d_b23;
        var_qofgsub_db24 = assign46440_e45277_d_b24;
        var_qofgsub_db25 = assign46440_e45277_d_b25;
        var_qofgsub_db26 = assign46440_e45277_d_b26;
        var_qofgsub_db27 = assign46440_e45277_d_b27;
        var_qofgsub_db28 = assign46440_e45277_d_b28;
        var_qofgsub_db29 = assign46440_e45277_d_b29;
        var_qofgsub_db30 = assign46440_e45277_d_b30;
        var_qofgsub_db31 = assign46440_e45277_d_b31;
        var_qofgsub_db32 = assign46440_e45277_d_b32;
        var_qofgsub_db33 = assign46440_e45277_d_b33;
        var_qofgsub_db34 = assign46440_e45277_d_b34;
        var_qofgsub_db35 = assign46440_e45277_d_b35;


        *var_guard506_slot = var_guard506;
        *var_guard507_slot = var_guard507;
        *var_guard508_slot = var_guard508;
        *var_qofdsub_slot = var_qofdsub;
        *var_qofdsub_db0_slot = var_qofdsub_db0;
        *var_qofdsub_db1_slot = var_qofdsub_db1;
        *var_qofdsub_db10_slot = var_qofdsub_db10;
        *var_qofdsub_db11_slot = var_qofdsub_db11;
        *var_qofdsub_db12_slot = var_qofdsub_db12;
        *var_qofdsub_db13_slot = var_qofdsub_db13;
        *var_qofdsub_db14_slot = var_qofdsub_db14;
        *var_qofdsub_db15_slot = var_qofdsub_db15;
        *var_qofdsub_db16_slot = var_qofdsub_db16;
        *var_qofdsub_db17_slot = var_qofdsub_db17;
        *var_qofdsub_db18_slot = var_qofdsub_db18;
        *var_qofdsub_db19_slot = var_qofdsub_db19;
        *var_qofdsub_db2_slot = var_qofdsub_db2;
        *var_qofdsub_db20_slot = var_qofdsub_db20;
        *var_qofdsub_db21_slot = var_qofdsub_db21;
        *var_qofdsub_db22_slot = var_qofdsub_db22;
        *var_qofdsub_db23_slot = var_qofdsub_db23;
        *var_qofdsub_db24_slot = var_qofdsub_db24;
        *var_qofdsub_db25_slot = var_qofdsub_db25;
        *var_qofdsub_db26_slot = var_qofdsub_db26;
        *var_qofdsub_db27_slot = var_qofdsub_db27;
        *var_qofdsub_db28_slot = var_qofdsub_db28;
        *var_qofdsub_db29_slot = var_qofdsub_db29;
        *var_qofdsub_db3_slot = var_qofdsub_db3;
        *var_qofdsub_db30_slot = var_qofdsub_db30;
        *var_qofdsub_db31_slot = var_qofdsub_db31;
        *var_qofdsub_db32_slot = var_qofdsub_db32;
        *var_qofdsub_db33_slot = var_qofdsub_db33;
        *var_qofdsub_db34_slot = var_qofdsub_db34;
        *var_qofdsub_db35_slot = var_qofdsub_db35;
        *var_qofdsub_db4_slot = var_qofdsub_db4;
        *var_qofdsub_db5_slot = var_qofdsub_db5;
        *var_qofdsub_db6_slot = var_qofdsub_db6;
        *var_qofdsub_db7_slot = var_qofdsub_db7;
        *var_qofdsub_db8_slot = var_qofdsub_db8;
        *var_qofdsub_db9_slot = var_qofdsub_db9;
        *var_qofdsub_dn0_slot = var_qofdsub_dn0;
        *var_qofdsub_dn1_slot = var_qofdsub_dn1;
        *var_qofdsub_dn10_slot = var_qofdsub_dn10;
        *var_qofdsub_dn11_slot = var_qofdsub_dn11;
        *var_qofdsub_dn12_slot = var_qofdsub_dn12;
        *var_qofdsub_dn13_slot = var_qofdsub_dn13;
        *var_qofdsub_dn14_slot = var_qofdsub_dn14;
        *var_qofdsub_dn15_slot = var_qofdsub_dn15;
        *var_qofdsub_dn16_slot = var_qofdsub_dn16;
        *var_qofdsub_dn17_slot = var_qofdsub_dn17;
        *var_qofdsub_dn18_slot = var_qofdsub_dn18;
        *var_qofdsub_dn19_slot = var_qofdsub_dn19;
        *var_qofdsub_dn2_slot = var_qofdsub_dn2;
        *var_qofdsub_dn20_slot = var_qofdsub_dn20;
        *var_qofdsub_dn21_slot = var_qofdsub_dn21;
        *var_qofdsub_dn22_slot = var_qofdsub_dn22;
        *var_qofdsub_dn23_slot = var_qofdsub_dn23;
        *var_qofdsub_dn24_slot = var_qofdsub_dn24;
        *var_qofdsub_dn25_slot = var_qofdsub_dn25;
        *var_qofdsub_dn26_slot = var_qofdsub_dn26;
        *var_qofdsub_dn27_slot = var_qofdsub_dn27;
        *var_qofdsub_dn28_slot = var_qofdsub_dn28;
        *var_qofdsub_dn29_slot = var_qofdsub_dn29;
        *var_qofdsub_dn3_slot = var_qofdsub_dn3;
        *var_qofdsub_dn4_slot = var_qofdsub_dn4;
        *var_qofdsub_dn5_slot = var_qofdsub_dn5;
        *var_qofdsub_dn6_slot = var_qofdsub_dn6;
        *var_qofdsub_dn7_slot = var_qofdsub_dn7;
        *var_qofdsub_dn8_slot = var_qofdsub_dn8;
        *var_qofdsub_dn9_slot = var_qofdsub_dn9;
        *var_qofgsub_slot = var_qofgsub;
        *var_qofgsub_db0_slot = var_qofgsub_db0;
        *var_qofgsub_db1_slot = var_qofgsub_db1;
        *var_qofgsub_db10_slot = var_qofgsub_db10;
        *var_qofgsub_db11_slot = var_qofgsub_db11;
        *var_qofgsub_db12_slot = var_qofgsub_db12;
        *var_qofgsub_db13_slot = var_qofgsub_db13;
        *var_qofgsub_db14_slot = var_qofgsub_db14;
        *var_qofgsub_db15_slot = var_qofgsub_db15;
        *var_qofgsub_db16_slot = var_qofgsub_db16;
        *var_qofgsub_db17_slot = var_qofgsub_db17;
        *var_qofgsub_db18_slot = var_qofgsub_db18;
        *var_qofgsub_db19_slot = var_qofgsub_db19;
        *var_qofgsub_db2_slot = var_qofgsub_db2;
        *var_qofgsub_db20_slot = var_qofgsub_db20;
        *var_qofgsub_db21_slot = var_qofgsub_db21;
        *var_qofgsub_db22_slot = var_qofgsub_db22;
        *var_qofgsub_db23_slot = var_qofgsub_db23;
        *var_qofgsub_db24_slot = var_qofgsub_db24;
        *var_qofgsub_db25_slot = var_qofgsub_db25;
        *var_qofgsub_db26_slot = var_qofgsub_db26;
        *var_qofgsub_db27_slot = var_qofgsub_db27;
        *var_qofgsub_db28_slot = var_qofgsub_db28;
        *var_qofgsub_db29_slot = var_qofgsub_db29;
        *var_qofgsub_db3_slot = var_qofgsub_db3;
        *var_qofgsub_db30_slot = var_qofgsub_db30;
        *var_qofgsub_db31_slot = var_qofgsub_db31;
        *var_qofgsub_db32_slot = var_qofgsub_db32;
        *var_qofgsub_db33_slot = var_qofgsub_db33;
        *var_qofgsub_db34_slot = var_qofgsub_db34;
        *var_qofgsub_db35_slot = var_qofgsub_db35;
        *var_qofgsub_db4_slot = var_qofgsub_db4;
        *var_qofgsub_db5_slot = var_qofgsub_db5;
        *var_qofgsub_db6_slot = var_qofgsub_db6;
        *var_qofgsub_db7_slot = var_qofgsub_db7;
        *var_qofgsub_db8_slot = var_qofgsub_db8;
        *var_qofgsub_db9_slot = var_qofgsub_db9;
        *var_qofgsub_dn0_slot = var_qofgsub_dn0;
        *var_qofgsub_dn1_slot = var_qofgsub_dn1;
        *var_qofgsub_dn10_slot = var_qofgsub_dn10;
        *var_qofgsub_dn11_slot = var_qofgsub_dn11;
        *var_qofgsub_dn12_slot = var_qofgsub_dn12;
        *var_qofgsub_dn13_slot = var_qofgsub_dn13;
        *var_qofgsub_dn14_slot = var_qofgsub_dn14;
        *var_qofgsub_dn15_slot = var_qofgsub_dn15;
        *var_qofgsub_dn16_slot = var_qofgsub_dn16;
        *var_qofgsub_dn17_slot = var_qofgsub_dn17;
        *var_qofgsub_dn18_slot = var_qofgsub_dn18;
        *var_qofgsub_dn19_slot = var_qofgsub_dn19;
        *var_qofgsub_dn2_slot = var_qofgsub_dn2;
        *var_qofgsub_dn20_slot = var_qofgsub_dn20;
        *var_qofgsub_dn21_slot = var_qofgsub_dn21;
        *var_qofgsub_dn22_slot = var_qofgsub_dn22;
        *var_qofgsub_dn23_slot = var_qofgsub_dn23;
        *var_qofgsub_dn24_slot = var_qofgsub_dn24;
        *var_qofgsub_dn25_slot = var_qofgsub_dn25;
        *var_qofgsub_dn26_slot = var_qofgsub_dn26;
        *var_qofgsub_dn27_slot = var_qofgsub_dn27;
        *var_qofgsub_dn28_slot = var_qofgsub_dn28;
        *var_qofgsub_dn29_slot = var_qofgsub_dn29;
        *var_qofgsub_dn3_slot = var_qofgsub_dn3;
        *var_qofgsub_dn4_slot = var_qofgsub_dn4;
        *var_qofgsub_dn5_slot = var_qofgsub_dn5;
        *var_qofgsub_dn6_slot = var_qofgsub_dn6;
        *var_qofgsub_dn7_slot = var_qofgsub_dn7;
        *var_qofgsub_dn8_slot = var_qofgsub_dn8;
        *var_qofgsub_dn9_slot = var_qofgsub_dn9;
    }

    pub(super) fn stamp_transient_block_123(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_cofgsubmt: f64,
        var_cofgsubmt0: f64,
        var_cofgsubmt0_db0: f64,
        var_cofgsubmt0_db1: f64,
        var_cofgsubmt0_db10: f64,
        var_cofgsubmt0_db11: f64,
        var_cofgsubmt0_db12: f64,
        var_cofgsubmt0_db13: f64,
        var_cofgsubmt0_db14: f64,
        var_cofgsubmt0_db15: f64,
        var_cofgsubmt0_db16: f64,
        var_cofgsubmt0_db17: f64,
        var_cofgsubmt0_db18: f64,
        var_cofgsubmt0_db19: f64,
        var_cofgsubmt0_db2: f64,
        var_cofgsubmt0_db20: f64,
        var_cofgsubmt0_db21: f64,
        var_cofgsubmt0_db22: f64,
        var_cofgsubmt0_db23: f64,
        var_cofgsubmt0_db24: f64,
        var_cofgsubmt0_db25: f64,
        var_cofgsubmt0_db26: f64,
        var_cofgsubmt0_db27: f64,
        var_cofgsubmt0_db28: f64,
        var_cofgsubmt0_db29: f64,
        var_cofgsubmt0_db3: f64,
        var_cofgsubmt0_db30: f64,
        var_cofgsubmt0_db31: f64,
        var_cofgsubmt0_db32: f64,
        var_cofgsubmt0_db33: f64,
        var_cofgsubmt0_db34: f64,
        var_cofgsubmt0_db35: f64,
        var_cofgsubmt0_db4: f64,
        var_cofgsubmt0_db5: f64,
        var_cofgsubmt0_db6: f64,
        var_cofgsubmt0_db7: f64,
        var_cofgsubmt0_db8: f64,
        var_cofgsubmt0_db9: f64,
        var_cofgsubmt0_dn0: f64,
        var_cofgsubmt0_dn1: f64,
        var_cofgsubmt0_dn10: f64,
        var_cofgsubmt0_dn11: f64,
        var_cofgsubmt0_dn12: f64,
        var_cofgsubmt0_dn13: f64,
        var_cofgsubmt0_dn14: f64,
        var_cofgsubmt0_dn15: f64,
        var_cofgsubmt0_dn16: f64,
        var_cofgsubmt0_dn17: f64,
        var_cofgsubmt0_dn18: f64,
        var_cofgsubmt0_dn19: f64,
        var_cofgsubmt0_dn2: f64,
        var_cofgsubmt0_dn20: f64,
        var_cofgsubmt0_dn21: f64,
        var_cofgsubmt0_dn22: f64,
        var_cofgsubmt0_dn23: f64,
        var_cofgsubmt0_dn24: f64,
        var_cofgsubmt0_dn25: f64,
        var_cofgsubmt0_dn26: f64,
        var_cofgsubmt0_dn27: f64,
        var_cofgsubmt0_dn28: f64,
        var_cofgsubmt0_dn29: f64,
        var_cofgsubmt0_dn3: f64,
        var_cofgsubmt0_dn4: f64,
        var_cofgsubmt0_dn5: f64,
        var_cofgsubmt0_dn6: f64,
        var_cofgsubmt0_dn7: f64,
        var_cofgsubmt0_dn8: f64,
        var_cofgsubmt0_dn9: f64,
        var_cofgsubmt_db0: f64,
        var_cofgsubmt_db1: f64,
        var_cofgsubmt_db10: f64,
        var_cofgsubmt_db11: f64,
        var_cofgsubmt_db12: f64,
        var_cofgsubmt_db13: f64,
        var_cofgsubmt_db14: f64,
        var_cofgsubmt_db15: f64,
        var_cofgsubmt_db16: f64,
        var_cofgsubmt_db17: f64,
        var_cofgsubmt_db18: f64,
        var_cofgsubmt_db19: f64,
        var_cofgsubmt_db2: f64,
        var_cofgsubmt_db20: f64,
        var_cofgsubmt_db21: f64,
        var_cofgsubmt_db22: f64,
        var_cofgsubmt_db23: f64,
        var_cofgsubmt_db24: f64,
        var_cofgsubmt_db25: f64,
        var_cofgsubmt_db26: f64,
        var_cofgsubmt_db27: f64,
        var_cofgsubmt_db28: f64,
        var_cofgsubmt_db29: f64,
        var_cofgsubmt_db3: f64,
        var_cofgsubmt_db30: f64,
        var_cofgsubmt_db31: f64,
        var_cofgsubmt_db32: f64,
        var_cofgsubmt_db33: f64,
        var_cofgsubmt_db34: f64,
        var_cofgsubmt_db35: f64,
        var_cofgsubmt_db4: f64,
        var_cofgsubmt_db5: f64,
        var_cofgsubmt_db6: f64,
        var_cofgsubmt_db7: f64,
        var_cofgsubmt_db8: f64,
        var_cofgsubmt_db9: f64,
        var_cofgsubmt_dn0: f64,
        var_cofgsubmt_dn1: f64,
        var_cofgsubmt_dn10: f64,
        var_cofgsubmt_dn11: f64,
        var_cofgsubmt_dn12: f64,
        var_cofgsubmt_dn13: f64,
        var_cofgsubmt_dn14: f64,
        var_cofgsubmt_dn15: f64,
        var_cofgsubmt_dn16: f64,
        var_cofgsubmt_dn17: f64,
        var_cofgsubmt_dn18: f64,
        var_cofgsubmt_dn19: f64,
        var_cofgsubmt_dn2: f64,
        var_cofgsubmt_dn20: f64,
        var_cofgsubmt_dn21: f64,
        var_cofgsubmt_dn22: f64,
        var_cofgsubmt_dn23: f64,
        var_cofgsubmt_dn24: f64,
        var_cofgsubmt_dn25: f64,
        var_cofgsubmt_dn26: f64,
        var_cofgsubmt_dn27: f64,
        var_cofgsubmt_dn28: f64,
        var_cofgsubmt_dn29: f64,
        var_cofgsubmt_dn3: f64,
        var_cofgsubmt_dn4: f64,
        var_cofgsubmt_dn5: f64,
        var_cofgsubmt_dn6: f64,
        var_cofgsubmt_dn7: f64,
        var_cofgsubmt_dn8: f64,
        var_cofgsubmt_dn9: f64,
        var_guard507: f64,
        var_guard508: f64,
        var_guard523_slot: &mut f64,
        var_qofgsub_slot: &mut f64,
        var_qofgsub_db0_slot: &mut f64,
        var_qofgsub_db1_slot: &mut f64,
        var_qofgsub_db10_slot: &mut f64,
        var_qofgsub_db11_slot: &mut f64,
        var_qofgsub_db12_slot: &mut f64,
        var_qofgsub_db13_slot: &mut f64,
        var_qofgsub_db14_slot: &mut f64,
        var_qofgsub_db15_slot: &mut f64,
        var_qofgsub_db16_slot: &mut f64,
        var_qofgsub_db17_slot: &mut f64,
        var_qofgsub_db18_slot: &mut f64,
        var_qofgsub_db19_slot: &mut f64,
        var_qofgsub_db2_slot: &mut f64,
        var_qofgsub_db20_slot: &mut f64,
        var_qofgsub_db21_slot: &mut f64,
        var_qofgsub_db22_slot: &mut f64,
        var_qofgsub_db23_slot: &mut f64,
        var_qofgsub_db24_slot: &mut f64,
        var_qofgsub_db25_slot: &mut f64,
        var_qofgsub_db26_slot: &mut f64,
        var_qofgsub_db27_slot: &mut f64,
        var_qofgsub_db28_slot: &mut f64,
        var_qofgsub_db29_slot: &mut f64,
        var_qofgsub_db3_slot: &mut f64,
        var_qofgsub_db30_slot: &mut f64,
        var_qofgsub_db31_slot: &mut f64,
        var_qofgsub_db32_slot: &mut f64,
        var_qofgsub_db33_slot: &mut f64,
        var_qofgsub_db34_slot: &mut f64,
        var_qofgsub_db35_slot: &mut f64,
        var_qofgsub_db4_slot: &mut f64,
        var_qofgsub_db5_slot: &mut f64,
        var_qofgsub_db6_slot: &mut f64,
        var_qofgsub_db7_slot: &mut f64,
        var_qofgsub_db8_slot: &mut f64,
        var_qofgsub_db9_slot: &mut f64,
        var_qofgsub_dn0_slot: &mut f64,
        var_qofgsub_dn1_slot: &mut f64,
        var_qofgsub_dn10_slot: &mut f64,
        var_qofgsub_dn11_slot: &mut f64,
        var_qofgsub_dn12_slot: &mut f64,
        var_qofgsub_dn13_slot: &mut f64,
        var_qofgsub_dn14_slot: &mut f64,
        var_qofgsub_dn15_slot: &mut f64,
        var_qofgsub_dn16_slot: &mut f64,
        var_qofgsub_dn17_slot: &mut f64,
        var_qofgsub_dn18_slot: &mut f64,
        var_qofgsub_dn19_slot: &mut f64,
        var_qofgsub_dn2_slot: &mut f64,
        var_qofgsub_dn20_slot: &mut f64,
        var_qofgsub_dn21_slot: &mut f64,
        var_qofgsub_dn22_slot: &mut f64,
        var_qofgsub_dn23_slot: &mut f64,
        var_qofgsub_dn24_slot: &mut f64,
        var_qofgsub_dn25_slot: &mut f64,
        var_qofgsub_dn26_slot: &mut f64,
        var_qofgsub_dn27_slot: &mut f64,
        var_qofgsub_dn28_slot: &mut f64,
        var_qofgsub_dn29_slot: &mut f64,
        var_qofgsub_dn3_slot: &mut f64,
        var_qofgsub_dn4_slot: &mut f64,
        var_qofgsub_dn5_slot: &mut f64,
        var_qofgsub_dn6_slot: &mut f64,
        var_qofgsub_dn7_slot: &mut f64,
        var_qofgsub_dn8_slot: &mut f64,
        var_qofgsub_dn9_slot: &mut f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let mut var_guard523: f64 = *var_guard523_slot;
        let mut var_qofgsub: f64 = *var_qofgsub_slot;
        let mut var_qofgsub_db0: f64 = *var_qofgsub_db0_slot;
        let mut var_qofgsub_db1: f64 = *var_qofgsub_db1_slot;
        let mut var_qofgsub_db10: f64 = *var_qofgsub_db10_slot;
        let mut var_qofgsub_db11: f64 = *var_qofgsub_db11_slot;
        let mut var_qofgsub_db12: f64 = *var_qofgsub_db12_slot;
        let mut var_qofgsub_db13: f64 = *var_qofgsub_db13_slot;
        let mut var_qofgsub_db14: f64 = *var_qofgsub_db14_slot;
        let mut var_qofgsub_db15: f64 = *var_qofgsub_db15_slot;
        let mut var_qofgsub_db16: f64 = *var_qofgsub_db16_slot;
        let mut var_qofgsub_db17: f64 = *var_qofgsub_db17_slot;
        let mut var_qofgsub_db18: f64 = *var_qofgsub_db18_slot;
        let mut var_qofgsub_db19: f64 = *var_qofgsub_db19_slot;
        let mut var_qofgsub_db2: f64 = *var_qofgsub_db2_slot;
        let mut var_qofgsub_db20: f64 = *var_qofgsub_db20_slot;
        let mut var_qofgsub_db21: f64 = *var_qofgsub_db21_slot;
        let mut var_qofgsub_db22: f64 = *var_qofgsub_db22_slot;
        let mut var_qofgsub_db23: f64 = *var_qofgsub_db23_slot;
        let mut var_qofgsub_db24: f64 = *var_qofgsub_db24_slot;
        let mut var_qofgsub_db25: f64 = *var_qofgsub_db25_slot;
        let mut var_qofgsub_db26: f64 = *var_qofgsub_db26_slot;
        let mut var_qofgsub_db27: f64 = *var_qofgsub_db27_slot;
        let mut var_qofgsub_db28: f64 = *var_qofgsub_db28_slot;
        let mut var_qofgsub_db29: f64 = *var_qofgsub_db29_slot;
        let mut var_qofgsub_db3: f64 = *var_qofgsub_db3_slot;
        let mut var_qofgsub_db30: f64 = *var_qofgsub_db30_slot;
        let mut var_qofgsub_db31: f64 = *var_qofgsub_db31_slot;
        let mut var_qofgsub_db32: f64 = *var_qofgsub_db32_slot;
        let mut var_qofgsub_db33: f64 = *var_qofgsub_db33_slot;
        let mut var_qofgsub_db34: f64 = *var_qofgsub_db34_slot;
        let mut var_qofgsub_db35: f64 = *var_qofgsub_db35_slot;
        let mut var_qofgsub_db4: f64 = *var_qofgsub_db4_slot;
        let mut var_qofgsub_db5: f64 = *var_qofgsub_db5_slot;
        let mut var_qofgsub_db6: f64 = *var_qofgsub_db6_slot;
        let mut var_qofgsub_db7: f64 = *var_qofgsub_db7_slot;
        let mut var_qofgsub_db8: f64 = *var_qofgsub_db8_slot;
        let mut var_qofgsub_db9: f64 = *var_qofgsub_db9_slot;
        let mut var_qofgsub_dn0: f64 = *var_qofgsub_dn0_slot;
        let mut var_qofgsub_dn1: f64 = *var_qofgsub_dn1_slot;
        let mut var_qofgsub_dn10: f64 = *var_qofgsub_dn10_slot;
        let mut var_qofgsub_dn11: f64 = *var_qofgsub_dn11_slot;
        let mut var_qofgsub_dn12: f64 = *var_qofgsub_dn12_slot;
        let mut var_qofgsub_dn13: f64 = *var_qofgsub_dn13_slot;
        let mut var_qofgsub_dn14: f64 = *var_qofgsub_dn14_slot;
        let mut var_qofgsub_dn15: f64 = *var_qofgsub_dn15_slot;
        let mut var_qofgsub_dn16: f64 = *var_qofgsub_dn16_slot;
        let mut var_qofgsub_dn17: f64 = *var_qofgsub_dn17_slot;
        let mut var_qofgsub_dn18: f64 = *var_qofgsub_dn18_slot;
        let mut var_qofgsub_dn19: f64 = *var_qofgsub_dn19_slot;
        let mut var_qofgsub_dn2: f64 = *var_qofgsub_dn2_slot;
        let mut var_qofgsub_dn20: f64 = *var_qofgsub_dn20_slot;
        let mut var_qofgsub_dn21: f64 = *var_qofgsub_dn21_slot;
        let mut var_qofgsub_dn22: f64 = *var_qofgsub_dn22_slot;
        let mut var_qofgsub_dn23: f64 = *var_qofgsub_dn23_slot;
        let mut var_qofgsub_dn24: f64 = *var_qofgsub_dn24_slot;
        let mut var_qofgsub_dn25: f64 = *var_qofgsub_dn25_slot;
        let mut var_qofgsub_dn26: f64 = *var_qofgsub_dn26_slot;
        let mut var_qofgsub_dn27: f64 = *var_qofgsub_dn27_slot;
        let mut var_qofgsub_dn28: f64 = *var_qofgsub_dn28_slot;
        let mut var_qofgsub_dn29: f64 = *var_qofgsub_dn29_slot;
        let mut var_qofgsub_dn3: f64 = *var_qofgsub_dn3_slot;
        let mut var_qofgsub_dn4: f64 = *var_qofgsub_dn4_slot;
        let mut var_qofgsub_dn5: f64 = *var_qofgsub_dn5_slot;
        let mut var_qofgsub_dn6: f64 = *var_qofgsub_dn6_slot;
        let mut var_qofgsub_dn7: f64 = *var_qofgsub_dn7_slot;
        let mut var_qofgsub_dn8: f64 = *var_qofgsub_dn8_slot;
        let mut var_qofgsub_dn9: f64 = *var_qofgsub_dn9_slot;

        let (assign46450_e45305, assign46450_e45305_d_n0, assign46450_e45305_d_n1, assign46450_e45305_d_n2, assign46450_e45305_d_n3, assign46450_e45305_d_n4, assign46450_e45305_d_n5, assign46450_e45305_d_n6, assign46450_e45305_d_n7, assign46450_e45305_d_n8, assign46450_e45305_d_n9, assign46450_e45305_d_n10, assign46450_e45305_d_n11, assign46450_e45305_d_n12, assign46450_e45305_d_n13, assign46450_e45305_d_n14, assign46450_e45305_d_n15, assign46450_e45305_d_n16, assign46450_e45305_d_n17, assign46450_e45305_d_n18, assign46450_e45305_d_n19, assign46450_e45305_d_n20, assign46450_e45305_d_n21, assign46450_e45305_d_n22, assign46450_e45305_d_n23, assign46450_e45305_d_n24, assign46450_e45305_d_n25, assign46450_e45305_d_n26, assign46450_e45305_d_n27, assign46450_e45305_d_n28, assign46450_e45305_d_n29, assign46450_e45305_d_b0, assign46450_e45305_d_b1, assign46450_e45305_d_b2, assign46450_e45305_d_b3, assign46450_e45305_d_b4, assign46450_e45305_d_b5, assign46450_e45305_d_b6, assign46450_e45305_d_b7, assign46450_e45305_d_b8, assign46450_e45305_d_b9, assign46450_e45305_d_b10, assign46450_e45305_d_b11, assign46450_e45305_d_b12, assign46450_e45305_d_b13, assign46450_e45305_d_b14, assign46450_e45305_d_b15, assign46450_e45305_d_b16, assign46450_e45305_d_b17, assign46450_e45305_d_b18, assign46450_e45305_d_b19, assign46450_e45305_d_b20, assign46450_e45305_d_b21, assign46450_e45305_d_b22, assign46450_e45305_d_b23, assign46450_e45305_d_b24, assign46450_e45305_d_b25, assign46450_e45305_d_b26, assign46450_e45305_d_b27, assign46450_e45305_d_b28, assign46450_e45305_d_b29, assign46450_e45305_d_b30, assign46450_e45305_d_b31, assign46450_e45305_d_b32, assign46450_e45305_d_b33, assign46450_e45305_d_b34, assign46450_e45305_d_b35,) = {
    if ((var_guard507 == 0.0) && (var_guard508 == 0.0)) {
        let assign46450_e45285: f64 = (p.p0 * p.p2);
        let assign46450_e45288: f64 = (var_cofgsubmt0 * (nv6 - nv3));
        let assign46450_e45291: f64 = (var_cofgsubmt * p.p28);
        let assign46450_e45295: f64 = ((nv6 - nv3) - p.p27);
        let assign46450_e45297: f64 = (assign46450_e45295 / p.p28);
        let assign46450_e45298: f64 = (assign46450_e45297).exp();
        let assign46450_e45299: f64 = (1.0 + assign46450_e45298);
        let assign46450_e45300: f64 = (assign46450_e45299).ln();
        let assign46450_e45301: f64 = (assign46450_e45291 * assign46450_e45300);
        let assign46450_e45302: f64 = (assign46450_e45288 + assign46450_e45301);
        let assign46450_e45303: f64 = (assign46450_e45285 * assign46450_e45302);
        (assign46450_e45303, (assign46450_e45285 * ((var_cofgsubmt0_dn0 * (nv6 - nv3)) + ((var_cofgsubmt_dn0 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn1 * (nv6 - nv3)) + ((var_cofgsubmt_dn1 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn2 * (nv6 - nv3)) + ((var_cofgsubmt_dn2 * p.p28) * assign46450_e45300))), (assign46450_e45285 * (((var_cofgsubmt0_dn3 * (nv6 - nv3)) + (-var_cofgsubmt0)) + (((var_cofgsubmt_dn3 * p.p28) * assign46450_e45300) + (assign46450_e45291 * ((assign46450_e45298 * (-1.0 / p.p28)) / assign46450_e45299))))), (assign46450_e45285 * ((var_cofgsubmt0_dn4 * (nv6 - nv3)) + ((var_cofgsubmt_dn4 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn5 * (nv6 - nv3)) + ((var_cofgsubmt_dn5 * p.p28) * assign46450_e45300))), (assign46450_e45285 * (((var_cofgsubmt0_dn6 * (nv6 - nv3)) + var_cofgsubmt0) + (((var_cofgsubmt_dn6 * p.p28) * assign46450_e45300) + (assign46450_e45291 * ((assign46450_e45298 * (1.0 / p.p28)) / assign46450_e45299))))), (assign46450_e45285 * ((var_cofgsubmt0_dn7 * (nv6 - nv3)) + ((var_cofgsubmt_dn7 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn8 * (nv6 - nv3)) + ((var_cofgsubmt_dn8 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn9 * (nv6 - nv3)) + ((var_cofgsubmt_dn9 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn10 * (nv6 - nv3)) + ((var_cofgsubmt_dn10 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn11 * (nv6 - nv3)) + ((var_cofgsubmt_dn11 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn12 * (nv6 - nv3)) + ((var_cofgsubmt_dn12 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn13 * (nv6 - nv3)) + ((var_cofgsubmt_dn13 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn14 * (nv6 - nv3)) + ((var_cofgsubmt_dn14 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn15 * (nv6 - nv3)) + ((var_cofgsubmt_dn15 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn16 * (nv6 - nv3)) + ((var_cofgsubmt_dn16 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn17 * (nv6 - nv3)) + ((var_cofgsubmt_dn17 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn18 * (nv6 - nv3)) + ((var_cofgsubmt_dn18 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn19 * (nv6 - nv3)) + ((var_cofgsubmt_dn19 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn20 * (nv6 - nv3)) + ((var_cofgsubmt_dn20 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn21 * (nv6 - nv3)) + ((var_cofgsubmt_dn21 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn22 * (nv6 - nv3)) + ((var_cofgsubmt_dn22 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn23 * (nv6 - nv3)) + ((var_cofgsubmt_dn23 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn24 * (nv6 - nv3)) + ((var_cofgsubmt_dn24 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn25 * (nv6 - nv3)) + ((var_cofgsubmt_dn25 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn26 * (nv6 - nv3)) + ((var_cofgsubmt_dn26 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn27 * (nv6 - nv3)) + ((var_cofgsubmt_dn27 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn28 * (nv6 - nv3)) + ((var_cofgsubmt_dn28 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_dn29 * (nv6 - nv3)) + ((var_cofgsubmt_dn29 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db0 * (nv6 - nv3)) + ((var_cofgsubmt_db0 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db1 * (nv6 - nv3)) + ((var_cofgsubmt_db1 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db2 * (nv6 - nv3)) + ((var_cofgsubmt_db2 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db3 * (nv6 - nv3)) + ((var_cofgsubmt_db3 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db4 * (nv6 - nv3)) + ((var_cofgsubmt_db4 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db5 * (nv6 - nv3)) + ((var_cofgsubmt_db5 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db6 * (nv6 - nv3)) + ((var_cofgsubmt_db6 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db7 * (nv6 - nv3)) + ((var_cofgsubmt_db7 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db8 * (nv6 - nv3)) + ((var_cofgsubmt_db8 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db9 * (nv6 - nv3)) + ((var_cofgsubmt_db9 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db10 * (nv6 - nv3)) + ((var_cofgsubmt_db10 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db11 * (nv6 - nv3)) + ((var_cofgsubmt_db11 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db12 * (nv6 - nv3)) + ((var_cofgsubmt_db12 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db13 * (nv6 - nv3)) + ((var_cofgsubmt_db13 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db14 * (nv6 - nv3)) + ((var_cofgsubmt_db14 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db15 * (nv6 - nv3)) + ((var_cofgsubmt_db15 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db16 * (nv6 - nv3)) + ((var_cofgsubmt_db16 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db17 * (nv6 - nv3)) + ((var_cofgsubmt_db17 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db18 * (nv6 - nv3)) + ((var_cofgsubmt_db18 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db19 * (nv6 - nv3)) + ((var_cofgsubmt_db19 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db20 * (nv6 - nv3)) + ((var_cofgsubmt_db20 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db21 * (nv6 - nv3)) + ((var_cofgsubmt_db21 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db22 * (nv6 - nv3)) + ((var_cofgsubmt_db22 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db23 * (nv6 - nv3)) + ((var_cofgsubmt_db23 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db24 * (nv6 - nv3)) + ((var_cofgsubmt_db24 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db25 * (nv6 - nv3)) + ((var_cofgsubmt_db25 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db26 * (nv6 - nv3)) + ((var_cofgsubmt_db26 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db27 * (nv6 - nv3)) + ((var_cofgsubmt_db27 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db28 * (nv6 - nv3)) + ((var_cofgsubmt_db28 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db29 * (nv6 - nv3)) + ((var_cofgsubmt_db29 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db30 * (nv6 - nv3)) + ((var_cofgsubmt_db30 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db31 * (nv6 - nv3)) + ((var_cofgsubmt_db31 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db32 * (nv6 - nv3)) + ((var_cofgsubmt_db32 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db33 * (nv6 - nv3)) + ((var_cofgsubmt_db33 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db34 * (nv6 - nv3)) + ((var_cofgsubmt_db34 * p.p28) * assign46450_e45300))), (assign46450_e45285 * ((var_cofgsubmt0_db35 * (nv6 - nv3)) + ((var_cofgsubmt_db35 * p.p28) * assign46450_e45300))),)
    } else {
        (var_qofgsub, var_qofgsub_dn0, var_qofgsub_dn1, var_qofgsub_dn2, var_qofgsub_dn3, var_qofgsub_dn4, var_qofgsub_dn5, var_qofgsub_dn6, var_qofgsub_dn7, var_qofgsub_dn8, var_qofgsub_dn9, var_qofgsub_dn10, var_qofgsub_dn11, var_qofgsub_dn12, var_qofgsub_dn13, var_qofgsub_dn14, var_qofgsub_dn15, var_qofgsub_dn16, var_qofgsub_dn17, var_qofgsub_dn18, var_qofgsub_dn19, var_qofgsub_dn20, var_qofgsub_dn21, var_qofgsub_dn22, var_qofgsub_dn23, var_qofgsub_dn24, var_qofgsub_dn25, var_qofgsub_dn26, var_qofgsub_dn27, var_qofgsub_dn28, var_qofgsub_dn29, var_qofgsub_db0, var_qofgsub_db1, var_qofgsub_db2, var_qofgsub_db3, var_qofgsub_db4, var_qofgsub_db5, var_qofgsub_db6, var_qofgsub_db7, var_qofgsub_db8, var_qofgsub_db9, var_qofgsub_db10, var_qofgsub_db11, var_qofgsub_db12, var_qofgsub_db13, var_qofgsub_db14, var_qofgsub_db15, var_qofgsub_db16, var_qofgsub_db17, var_qofgsub_db18, var_qofgsub_db19, var_qofgsub_db20, var_qofgsub_db21, var_qofgsub_db22, var_qofgsub_db23, var_qofgsub_db24, var_qofgsub_db25, var_qofgsub_db26, var_qofgsub_db27, var_qofgsub_db28, var_qofgsub_db29, var_qofgsub_db30, var_qofgsub_db31, var_qofgsub_db32, var_qofgsub_db33, var_qofgsub_db34, var_qofgsub_db35,)
    }
};
        var_qofgsub = assign46450_e45305;
        var_qofgsub_dn0 = assign46450_e45305_d_n0;
        var_qofgsub_dn1 = assign46450_e45305_d_n1;
        var_qofgsub_dn2 = assign46450_e45305_d_n2;
        var_qofgsub_dn3 = assign46450_e45305_d_n3;
        var_qofgsub_dn4 = assign46450_e45305_d_n4;
        var_qofgsub_dn5 = assign46450_e45305_d_n5;
        var_qofgsub_dn6 = assign46450_e45305_d_n6;
        var_qofgsub_dn7 = assign46450_e45305_d_n7;
        var_qofgsub_dn8 = assign46450_e45305_d_n8;
        var_qofgsub_dn9 = assign46450_e45305_d_n9;
        var_qofgsub_dn10 = assign46450_e45305_d_n10;
        var_qofgsub_dn11 = assign46450_e45305_d_n11;
        var_qofgsub_dn12 = assign46450_e45305_d_n12;
        var_qofgsub_dn13 = assign46450_e45305_d_n13;
        var_qofgsub_dn14 = assign46450_e45305_d_n14;
        var_qofgsub_dn15 = assign46450_e45305_d_n15;
        var_qofgsub_dn16 = assign46450_e45305_d_n16;
        var_qofgsub_dn17 = assign46450_e45305_d_n17;
        var_qofgsub_dn18 = assign46450_e45305_d_n18;
        var_qofgsub_dn19 = assign46450_e45305_d_n19;
        var_qofgsub_dn20 = assign46450_e45305_d_n20;
        var_qofgsub_dn21 = assign46450_e45305_d_n21;
        var_qofgsub_dn22 = assign46450_e45305_d_n22;
        var_qofgsub_dn23 = assign46450_e45305_d_n23;
        var_qofgsub_dn24 = assign46450_e45305_d_n24;
        var_qofgsub_dn25 = assign46450_e45305_d_n25;
        var_qofgsub_dn26 = assign46450_e45305_d_n26;
        var_qofgsub_dn27 = assign46450_e45305_d_n27;
        var_qofgsub_dn28 = assign46450_e45305_d_n28;
        var_qofgsub_dn29 = assign46450_e45305_d_n29;
        var_qofgsub_db0 = assign46450_e45305_d_b0;
        var_qofgsub_db1 = assign46450_e45305_d_b1;
        var_qofgsub_db2 = assign46450_e45305_d_b2;
        var_qofgsub_db3 = assign46450_e45305_d_b3;
        var_qofgsub_db4 = assign46450_e45305_d_b4;
        var_qofgsub_db5 = assign46450_e45305_d_b5;
        var_qofgsub_db6 = assign46450_e45305_d_b6;
        var_qofgsub_db7 = assign46450_e45305_d_b7;
        var_qofgsub_db8 = assign46450_e45305_d_b8;
        var_qofgsub_db9 = assign46450_e45305_d_b9;
        var_qofgsub_db10 = assign46450_e45305_d_b10;
        var_qofgsub_db11 = assign46450_e45305_d_b11;
        var_qofgsub_db12 = assign46450_e45305_d_b12;
        var_qofgsub_db13 = assign46450_e45305_d_b13;
        var_qofgsub_db14 = assign46450_e45305_d_b14;
        var_qofgsub_db15 = assign46450_e45305_d_b15;
        var_qofgsub_db16 = assign46450_e45305_d_b16;
        var_qofgsub_db17 = assign46450_e45305_d_b17;
        var_qofgsub_db18 = assign46450_e45305_d_b18;
        var_qofgsub_db19 = assign46450_e45305_d_b19;
        var_qofgsub_db20 = assign46450_e45305_d_b20;
        var_qofgsub_db21 = assign46450_e45305_d_b21;
        var_qofgsub_db22 = assign46450_e45305_d_b22;
        var_qofgsub_db23 = assign46450_e45305_d_b23;
        var_qofgsub_db24 = assign46450_e45305_d_b24;
        var_qofgsub_db25 = assign46450_e45305_d_b25;
        var_qofgsub_db26 = assign46450_e45305_d_b26;
        var_qofgsub_db27 = assign46450_e45305_d_b27;
        var_qofgsub_db28 = assign46450_e45305_d_b28;
        var_qofgsub_db29 = assign46450_e45305_d_b29;
        var_qofgsub_db30 = assign46450_e45305_d_b30;
        var_qofgsub_db31 = assign46450_e45305_d_b31;
        var_qofgsub_db32 = assign46450_e45305_d_b32;
        var_qofgsub_db33 = assign46450_e45305_d_b33;
        var_qofgsub_db34 = assign46450_e45305_d_b34;
        var_qofgsub_db35 = assign46450_e45305_d_b35;

        s.store_add_scaled_value_products(114, A::add_scaled_value_products3(A::add_scaled_value_products3(A::add_scaled_products3(s.ad_value(115), A::voltage(ctx, nodes, Some(5), Some(9)), 1.0, s.ad_value(160), A::voltage(ctx, nodes, Some(18), Some(17)), 1.0, s.ad_value(154), A::voltage(ctx, nodes, Some(13), Some(19)), 1.0), 1.0, s.ad_value(184), A::voltage(ctx, nodes, Some(12), Some(13)), 1.0, s.ad_value(178), A::voltage(ctx, nodes, Some(11), Some(12)), 1.0, s.ad_value(172), A::voltage(ctx, nodes, Some(10), Some(11)), 1.0), 1.0, s.ad_value(166), A::voltage(ctx, nodes, Some(9), Some(10)), 1.0, s.ad_value(190), A::voltage(ctx, nodes, Some(14), Some(5)), 1.0, s.ad_value(196), A::voltage(ctx, nodes, Some(15), Some(14)), 1.0), 1.0, s.ad_value(202), A::voltage(ctx, nodes, Some(16), Some(15)), 1.0, s.ad_value(208), A::voltage(ctx, nodes, Some(17), Some(16)), 1.0);

        s.b[2698] = ((s.v[4] >= p.p353) && (s.v[4] > 0.0));
        s.store_scalar(2698, if s.b[2698] { 1.0 } else { 0.0 });

        if s.b[2698] {
            s.store_add_ad_rhs(114, 114, A::div_scaled_product(A::voltage(ctx, nodes, Some(18), Some(0)), A::voltage(ctx, nodes, Some(18), Some(0)), 1.0, s.ad_value(1), 1.0));
        }

        s.b[2699] = ((s.v[3] >= p.p353) && (s.v[3] > 0.0));
        s.store_scalar(2699, if s.b[2699] { 1.0 } else { 0.0 });

        if s.b[2699] {
            s.store_add_ad_rhs(114, 114, A::div_scaled_product(A::voltage(ctx, nodes, Some(19), Some(2)), A::voltage(ctx, nodes, Some(19), Some(2)), 1.0, s.ad_value(2), 1.0));
        }

        let assign46690_e45519: f64 = if p.p320 > 0.0 { 1.0 } else { 0.0 };
        var_guard523 = assign46690_e45519;


        *var_guard523_slot = var_guard523;
        *var_qofgsub_slot = var_qofgsub;
        *var_qofgsub_db0_slot = var_qofgsub_db0;
        *var_qofgsub_db1_slot = var_qofgsub_db1;
        *var_qofgsub_db10_slot = var_qofgsub_db10;
        *var_qofgsub_db11_slot = var_qofgsub_db11;
        *var_qofgsub_db12_slot = var_qofgsub_db12;
        *var_qofgsub_db13_slot = var_qofgsub_db13;
        *var_qofgsub_db14_slot = var_qofgsub_db14;
        *var_qofgsub_db15_slot = var_qofgsub_db15;
        *var_qofgsub_db16_slot = var_qofgsub_db16;
        *var_qofgsub_db17_slot = var_qofgsub_db17;
        *var_qofgsub_db18_slot = var_qofgsub_db18;
        *var_qofgsub_db19_slot = var_qofgsub_db19;
        *var_qofgsub_db2_slot = var_qofgsub_db2;
        *var_qofgsub_db20_slot = var_qofgsub_db20;
        *var_qofgsub_db21_slot = var_qofgsub_db21;
        *var_qofgsub_db22_slot = var_qofgsub_db22;
        *var_qofgsub_db23_slot = var_qofgsub_db23;
        *var_qofgsub_db24_slot = var_qofgsub_db24;
        *var_qofgsub_db25_slot = var_qofgsub_db25;
        *var_qofgsub_db26_slot = var_qofgsub_db26;
        *var_qofgsub_db27_slot = var_qofgsub_db27;
        *var_qofgsub_db28_slot = var_qofgsub_db28;
        *var_qofgsub_db29_slot = var_qofgsub_db29;
        *var_qofgsub_db3_slot = var_qofgsub_db3;
        *var_qofgsub_db30_slot = var_qofgsub_db30;
        *var_qofgsub_db31_slot = var_qofgsub_db31;
        *var_qofgsub_db32_slot = var_qofgsub_db32;
        *var_qofgsub_db33_slot = var_qofgsub_db33;
        *var_qofgsub_db34_slot = var_qofgsub_db34;
        *var_qofgsub_db35_slot = var_qofgsub_db35;
        *var_qofgsub_db4_slot = var_qofgsub_db4;
        *var_qofgsub_db5_slot = var_qofgsub_db5;
        *var_qofgsub_db6_slot = var_qofgsub_db6;
        *var_qofgsub_db7_slot = var_qofgsub_db7;
        *var_qofgsub_db8_slot = var_qofgsub_db8;
        *var_qofgsub_db9_slot = var_qofgsub_db9;
        *var_qofgsub_dn0_slot = var_qofgsub_dn0;
        *var_qofgsub_dn1_slot = var_qofgsub_dn1;
        *var_qofgsub_dn10_slot = var_qofgsub_dn10;
        *var_qofgsub_dn11_slot = var_qofgsub_dn11;
        *var_qofgsub_dn12_slot = var_qofgsub_dn12;
        *var_qofgsub_dn13_slot = var_qofgsub_dn13;
        *var_qofgsub_dn14_slot = var_qofgsub_dn14;
        *var_qofgsub_dn15_slot = var_qofgsub_dn15;
        *var_qofgsub_dn16_slot = var_qofgsub_dn16;
        *var_qofgsub_dn17_slot = var_qofgsub_dn17;
        *var_qofgsub_dn18_slot = var_qofgsub_dn18;
        *var_qofgsub_dn19_slot = var_qofgsub_dn19;
        *var_qofgsub_dn2_slot = var_qofgsub_dn2;
        *var_qofgsub_dn20_slot = var_qofgsub_dn20;
        *var_qofgsub_dn21_slot = var_qofgsub_dn21;
        *var_qofgsub_dn22_slot = var_qofgsub_dn22;
        *var_qofgsub_dn23_slot = var_qofgsub_dn23;
        *var_qofgsub_dn24_slot = var_qofgsub_dn24;
        *var_qofgsub_dn25_slot = var_qofgsub_dn25;
        *var_qofgsub_dn26_slot = var_qofgsub_dn26;
        *var_qofgsub_dn27_slot = var_qofgsub_dn27;
        *var_qofgsub_dn28_slot = var_qofgsub_dn28;
        *var_qofgsub_dn29_slot = var_qofgsub_dn29;
        *var_qofgsub_dn3_slot = var_qofgsub_dn3;
        *var_qofgsub_dn4_slot = var_qofgsub_dn4;
        *var_qofgsub_dn5_slot = var_qofgsub_dn5;
        *var_qofgsub_dn6_slot = var_qofgsub_dn6;
        *var_qofgsub_dn7_slot = var_qofgsub_dn7;
        *var_qofgsub_dn8_slot = var_qofgsub_dn8;
        *var_qofgsub_dn9_slot = var_qofgsub_dn9;
    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.store_scalar(109, (p.p5 + 273.15));

        s.store_scalar(108, ctx_temp);

        s.store_voltage(110, ctx, nodes, Some(4), None);

        s.store_offset(111, 110, (s.v[108] + p.p3));

        s.b[298] = (s.v[111] < ((-270.0) + 273.15));
        s.store_scalar(298, if s.b[298] { 1.0 } else { 0.0 });

        if s.b[298] {
            s.store_scalar(111, ((-270.0) + 273.15));
        }

        s.b[299] = (s.v[111] > (1500.0 + 273.15));
        s.store_scalar(299, if s.b[299] { 1.0 } else { 0.0 });

        if ((!s.b[298]) && s.b[299]) {
            s.store_scalar(111, (1500.0 + 273.15));
        }

        s.store_scale(113, 111, (1.38062e-23 * 6.241457005723417e18));

        s.store_scale_ad(7, {
            if ((1.0 + (p.p21 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p21, (((((-s.v[109])) * (p.p21))) + (1.0)))
            }
        }, p.p9);

        s.store_scale_ad(8, {
            if ((1.0 + (p.p22 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p22, (((((-s.v[109])) * (p.p22))) + (1.0)))
            }
        }, p.p10);

        s.store_scale_ad(9, {
            if ((1.0 + (p.p23 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p23, (((((-s.v[109])) * (p.p23))) + (1.0)))
            }
        }, p.p11);

        s.store_scale_ad(10, {
            if ((1.0 + (p.p24 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p24, (((((-s.v[109])) * (p.p24))) + (1.0)))
            }
        }, p.p13);

        s.store_scale_ad(11, {
            if ((1.0 + (p.p25 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p25, (((((-s.v[109])) * (p.p25))) + (1.0)))
            }
        }, p.p12);

        s.store_scale_ad(12, {
            if ((1.0 + (p.p26 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p26, (((((-s.v[109])) * (p.p26))) + (1.0)))
            }
        }, p.p14);

        s.store_scale_ad(13, {
            if ((1.0 + (p.p21 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p21, (((((-s.v[109])) * (p.p21))) + (1.0)))
            }
        }, p.p15);

        s.store_scale_ad(14, {
            if ((1.0 + (p.p22 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p22, (((((-s.v[109])) * (p.p22))) + (1.0)))
            }
        }, p.p16);

        s.store_scale_ad(15, {
            if ((1.0 + (p.p23 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p23, (((((-s.v[109])) * (p.p23))) + (1.0)))
            }
        }, p.p17);

        s.store_scale_ad(16, {
            if ((1.0 + (p.p24 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p24, (((((-s.v[109])) * (p.p24))) + (1.0)))
            }
        }, p.p19);

        s.store_scale_ad(17, {
            if ((1.0 + (p.p25 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p25, (((((-s.v[109])) * (p.p25))) + (1.0)))
            }
        }, p.p18);

        s.store_scale_ad(18, {
            if ((1.0 + (p.p26 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p26, (((((-s.v[109])) * (p.p26))) + (1.0)))
            }
        }, p.p20);

        s.store_scale_ad(19, {
            if ((1.0 + (p.p8 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p8, (((((-s.v[109])) * (p.p8))) + (1.0)))
            }
        }, p.p7);

        s.store_scale_ad(20, {
            if ((1.0 + (p.p82 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p82, (((((-s.v[109])) * (p.p82))) + (1.0)))
            }
        }, p.p81);

        s.store_scale_ad(23, {
            if ((1.0 + (p.p104 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p104, (((((-s.v[109])) * (p.p104))) + (1.0)))
            }
        }, p.p103);

        s.store_scale_ad(26, {
            if ((1.0 + (p.p126 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p126, (((((-s.v[109])) * (p.p126))) + (1.0)))
            }
        }, p.p125);

        s.store_scale_ad(29, {
            if ((1.0 + (p.p148 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p148, (((((-s.v[109])) * (p.p148))) + (1.0)))
            }
        }, p.p147);

        s.store_scale_ad(21, {
            if ((1.0 + (p.p87 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p87, (((((-s.v[109])) * (p.p87))) + (1.0)))
            }
        }, p.p86);

        s.store_scale_ad(24, {
            if ((1.0 + (p.p109 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p109, (((((-s.v[109])) * (p.p109))) + (1.0)))
            }
        }, p.p108);

        s.store_scale_ad(27, {
            if ((1.0 + (p.p131 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p131, (((((-s.v[109])) * (p.p131))) + (1.0)))
            }
        }, p.p130);

        s.store_scale_ad(30, {
            if ((1.0 + (p.p153 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p153, (((((-s.v[109])) * (p.p153))) + (1.0)))
            }
        }, p.p152);

        s.store_scale_ad(22, {
            if ((1.0 + (p.p89 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p89, (((((-s.v[109])) * (p.p89))) + (1.0)))
            }
        }, p.p88);

        s.store_scale_ad(25, {
            if ((1.0 + (p.p111 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p111, (((((-s.v[109])) * (p.p111))) + (1.0)))
            }
        }, p.p110);

        s.store_scale_ad(28, {
            if ((1.0 + (p.p133 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p133, (((((-s.v[109])) * (p.p133))) + (1.0)))
            }
        }, p.p132);

        s.store_scale_ad(31, {
            if ((1.0 + (p.p155 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p155, (((((-s.v[109])) * (p.p155))) + (1.0)))
            }
        }, p.p154);

        s.store_scale_ad(32, {
            if ((1.0 + (p.p170 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p170, (((((-s.v[109])) * (p.p170))) + (1.0)))
            }
        }, p.p169);

        s.store_scale_ad(35, {
            if ((1.0 + (p.p192 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p192, (((((-s.v[109])) * (p.p192))) + (1.0)))
            }
        }, p.p191);

        s.store_scale_ad(38, {
            if ((1.0 + (p.p214 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p214, (((((-s.v[109])) * (p.p214))) + (1.0)))
            }
        }, p.p213);

        s.store_scale_ad(41, {
            if ((1.0 + (p.p236 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p236, (((((-s.v[109])) * (p.p236))) + (1.0)))
            }
        }, p.p235);

        s.store_scale_ad(33, {
            if ((1.0 + (p.p175 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p175, (((((-s.v[109])) * (p.p175))) + (1.0)))
            }
        }, p.p174);

        s.store_scale_ad(36, {
            if ((1.0 + (p.p197 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p197, (((((-s.v[109])) * (p.p197))) + (1.0)))
            }
        }, p.p196);

        s.store_scale_ad(39, {
            if ((1.0 + (p.p219 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p219, (((((-s.v[109])) * (p.p219))) + (1.0)))
            }
        }, p.p218);

        s.store_scale_ad(42, {
            if ((1.0 + (p.p241 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p241, (((((-s.v[109])) * (p.p241))) + (1.0)))
            }
        }, p.p240);

        s.store_scale_ad(34, {
            if ((1.0 + (p.p177 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p177, (((((-s.v[109])) * (p.p177))) + (1.0)))
            }
        }, p.p176);

        s.store_scale_ad(37, {
            if ((1.0 + (p.p199 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p199, (((((-s.v[109])) * (p.p199))) + (1.0)))
            }
        }, p.p198);

        s.store_scale_ad(40, {
            if ((1.0 + (p.p221 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p221, (((((-s.v[109])) * (p.p221))) + (1.0)))
            }
        }, p.p220);

        s.store_scale_ad(43, {
            if ((1.0 + (p.p243 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p243, (((((-s.v[109])) * (p.p243))) + (1.0)))
            }
        }, p.p242);

        s.store_scaled_voltage(44, ctx, nodes, Some(5), Some(9), p.p6);

        s.store_scaled_voltage(45, ctx, nodes, Some(8), Some(9), p.p6);

        s.store_scalar(224, 0.0);

        s.store_scalar(226, 0.0);

        s.store_scalar(225, 0.0);

        s.store_scalar(227, 0.0);

        s.store_scalar(228, 0.0);

        s.store_scalar(229, 0.0);

        s.store_scalar(230, 1.0);

        s.b[308] = (p.p328 == 1.0);
        s.store_scalar(308, if s.b[308] { 1.0 } else { 0.0 });

        s.b[309] = (p.p328 == 2.0);
        s.store_scalar(309, if s.b[309] { 1.0 } else { 0.0 });

        if ((!s.b[308]) && s.b[309]) {
            s.store_voltage(224, ctx, nodes, Some(22), None);
            s.store_voltage(225, ctx, nodes, Some(23), None);
            s.store_scaled_abs_ad(228, A::sub(s.ad_value(225), s.ad_value(224)), 1.0 / (p.p338));
            s.store_voltage(226, ctx, nodes, Some(25), None);
            s.store_voltage(227, ctx, nodes, Some(26), None);
            s.store_scaled_abs_ad(229, A::sub(s.ad_value(227), s.ad_value(226)), 1.0 / (p.p337));
            s.store_div_from_scalar_add_ad(230, 1.0, A::offset(s.ad_value(228), 1.0), s.ad_value(229));
        }

        s.b[312] = (p.p78 == 1.0);
        s.store_scalar(312, if s.b[312] { 1.0 } else { 0.0 });

        if s.b[312] {
            s.store_scaled_voltage(60, ctx, nodes, Some(7), Some(10), p.p6);
            s.store_scaled_voltage(62, ctx, nodes, Some(2), Some(10), p.p6);
        }

        if (!s.b[312]) {
            s.store_scaled_voltage(60, ctx, nodes, Some(2), Some(10), p.p6);
            s.store_scaled_voltage(62, ctx, nodes, Some(7), Some(10), p.p6);
        }

        s.store_scaled_voltage(61, ctx, nodes, Some(9), Some(10), p.p6);

        s.store_scaled_voltage(63, ctx, nodes, Some(3), Some(10), p.p6);

        s.b[313] = (p.p100 == 1.0);
        s.store_scalar(313, if s.b[313] { 1.0 } else { 0.0 });

        if s.b[313] {
            s.store_scaled_voltage(66, ctx, nodes, Some(7), Some(11), p.p6);
            s.store_scaled_voltage(68, ctx, nodes, Some(2), Some(11), p.p6);
        }

        if (!s.b[313]) {
            s.store_scaled_voltage(66, ctx, nodes, Some(2), Some(11), p.p6);
            s.store_scaled_voltage(68, ctx, nodes, Some(7), Some(11), p.p6);
        }

        s.store_scaled_voltage(67, ctx, nodes, Some(10), Some(11), p.p6);

        s.store_scaled_voltage(69, ctx, nodes, Some(3), Some(11), p.p6);

        s.b[314] = (p.p122 == 1.0);
        s.store_scalar(314, if s.b[314] { 1.0 } else { 0.0 });

        if s.b[314] {
            s.store_scaled_voltage(72, ctx, nodes, Some(7), Some(12), p.p6);
            s.store_scaled_voltage(74, ctx, nodes, Some(2), Some(12), p.p6);
        }

        if (!s.b[314]) {
            s.store_scaled_voltage(72, ctx, nodes, Some(2), Some(12), p.p6);
            s.store_scaled_voltage(74, ctx, nodes, Some(7), Some(12), p.p6);
        }

        s.store_scaled_voltage(73, ctx, nodes, Some(11), Some(12), p.p6);

        s.store_scaled_voltage(75, ctx, nodes, Some(3), Some(12), p.p6);

        s.b[315] = (p.p144 == 1.0);
        s.store_scalar(315, if s.b[315] { 1.0 } else { 0.0 });

        if s.b[315] {
            s.store_scaled_voltage(78, ctx, nodes, Some(7), Some(13), p.p6);
            s.store_scaled_voltage(80, ctx, nodes, Some(2), Some(13), p.p6);
        }

        if (!s.b[315]) {
            s.store_scaled_voltage(78, ctx, nodes, Some(2), Some(13), p.p6);
            s.store_scaled_voltage(80, ctx, nodes, Some(7), Some(13), p.p6);
        }

        s.store_scaled_voltage(79, ctx, nodes, Some(12), Some(13), p.p6);

        s.store_scaled_voltage(81, ctx, nodes, Some(3), Some(13), p.p6);

        s.b[316] = (p.p166 == 1.0);
        s.store_scalar(316, if s.b[316] { 1.0 } else { 0.0 });

        if s.b[316] {
            s.store_scaled_voltage(84, ctx, nodes, Some(7), Some(5), p.p6);
            s.store_scaled_voltage(86, ctx, nodes, Some(2), Some(5), p.p6);
        }

        if (!s.b[316]) {
            s.store_scaled_voltage(84, ctx, nodes, Some(2), Some(5), p.p6);
            s.store_scaled_voltage(86, ctx, nodes, Some(7), Some(5), p.p6);
        }

        s.store_scaled_voltage(85, ctx, nodes, Some(14), Some(5), p.p6);

        s.store_scaled_voltage(87, ctx, nodes, Some(3), Some(5), p.p6);

        s.b[317] = (p.p188 == 1.0);
        s.store_scalar(317, if s.b[317] { 1.0 } else { 0.0 });

        if s.b[317] {
            s.store_scaled_voltage(90, ctx, nodes, Some(7), Some(14), p.p6);
            s.store_scaled_voltage(92, ctx, nodes, Some(2), Some(14), p.p6);
        }

        if (!s.b[317]) {
            s.store_scaled_voltage(90, ctx, nodes, Some(2), Some(14), p.p6);
            s.store_scaled_voltage(92, ctx, nodes, Some(7), Some(14), p.p6);
        }

        s.store_scaled_voltage(91, ctx, nodes, Some(15), Some(14), p.p6);

        s.store_scaled_voltage(93, ctx, nodes, Some(3), Some(14), p.p6);

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[318] = (p.p210 == 1.0);
        s.store_scalar(318, if s.b[318] { 1.0 } else { 0.0 });

        if s.b[318] {
            s.store_scaled_voltage(96, ctx, nodes, Some(7), Some(15), p.p6);
            s.store_scaled_voltage(98, ctx, nodes, Some(2), Some(15), p.p6);
        }

        if (!s.b[318]) {
            s.store_scaled_voltage(96, ctx, nodes, Some(2), Some(15), p.p6);
            s.store_scaled_voltage(98, ctx, nodes, Some(7), Some(15), p.p6);
        }

        s.store_scaled_voltage(97, ctx, nodes, Some(16), Some(15), p.p6);

        s.store_scaled_voltage(99, ctx, nodes, Some(3), Some(15), p.p6);

        s.b[319] = (p.p232 == 1.0);
        s.store_scalar(319, if s.b[319] { 1.0 } else { 0.0 });

        if s.b[319] {
            s.store_scaled_voltage(102, ctx, nodes, Some(7), Some(16), p.p6);
            s.store_scaled_voltage(104, ctx, nodes, Some(2), Some(16), p.p6);
        }

        if (!s.b[319]) {
            s.store_scaled_voltage(102, ctx, nodes, Some(2), Some(16), p.p6);
            s.store_scaled_voltage(104, ctx, nodes, Some(7), Some(16), p.p6);
        }

        s.store_scaled_voltage(103, ctx, nodes, Some(17), Some(16), p.p6);

        s.store_scaled_voltage(105, ctx, nodes, Some(3), Some(16), p.p6);

        s.store_scalar(209, 0.0);

        s.store_scalar(210, 0.0);

        s.store_scalar(211, 0.0);

        s.store_scalar(212, 0.0);

        s.store_scalar(213, 0.0);

        s.b[320] = (p.p233 > p.p354);
        s.store_scalar(320, if s.b[320] { 1.0 } else { 0.0 });

        if s.b[320] {
            s.store_scalar(323, 0.0);
            s.store_scalar(324, 0.0);
            s.store_scalar(325, 0.0);
            s.store_scalar(326, 0.0);
            s.store_scalar(327, 0.0);
            s.store_scalar(328, 0.0);
            s.store_scalar(329, 0.0);
            s.copy_ad(330, 102);
            s.copy_ad(331, 103);
            s.store_scalar(332, p.p239);
            s.copy_ad(333, 104);
            s.copy_ad(334, 105);
            s.store_scalar(335, p.p237);
            s.copy_ad(336, 111);
            s.store_scalar(337, s.v[109]);
            s.copy_ad(338, 113);
            s.store_scalar(339, p.p0);
            s.store_scalar(340, p.p233);
            s.copy_ad(341, 41);
            s.store_scalar(342, p.p238);
            s.copy_ad(343, 42);
            s.copy_ad(344, 43);
            s.store_scalar(345, p.p234);
            s.store_scalar(346, p.p248);
            s.store_scalar(347, p.p247);
            s.store_scalar(348, 0.0);
            s.store_scalar(349, p.p249);
            s.store_scalar(350, p.p253);
            s.store_scalar(351, p.p244);
            s.store_scalar(352, p.p245);
            s.store_scalar(353, p.p246);
            s.store_scalar(354, p.p252);
            s.store_scalar(355, p.p251);
            s.store_scalar(356, p.p250);
            s.store_scalar(357, p.p39);
            s.store_scalar(358, p.p47);
            s.store_scalar(359, p.p45);
            s.store_scalar(360, p.p42);
            s.store_scalar(361, p.p2);
            s.store_scalar(362, p.p6);
            s.store_scalar(363, 1.0);
            s.store_scalar(364, 0.0);
            s.store_scalar(365, 0.0);
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
            s.store_scalar(377, 0.0);
            s.store_scalar(378, 0.0);
            s.store_scalar(379, 0.0);
            s.store_scalar(380, 0.0);
            s.store_scalar(381, 0.0);
            s.store_scalar(382, 0.0);
            s.store_scalar(383, 0.0);
            s.store_scalar(384, 0.0);
            s.store_scalar(385, 0.0);
            s.store_scalar(386, 0.0);
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
            s.store_scalar(401, 0.0);
            s.store_scalar(402, 0.0);
            s.store_scalar(405, 0.0);
            s.store_scalar(406, 0.0);
            s.store_scalar(407, 0.0);
            s.store_scalar(408, 0.0);
            s.store_scalar(409, 0.0);
            s.store_scalar(410, 0.0);
            s.store_scalar(411, 0.0);
            s.store_scalar(412, 0.0);
            s.store_scalar(413, 0.0);
            s.store_scalar(414, 0.0);
            s.store_scalar(415, 0.0);
            s.store_scalar(416, 0.0);
            s.store_scalar(417, 0.0);
            s.store_scalar(418, 0.0);
            s.store_scalar(419, 0.0);
            s.store_scalar(420, 0.0);
            s.store_scalar(421, 0.0);
            s.store_scalar(422, 0.0);
            s.store_scalar(423, 0.0);
            s.store_scalar(424, 0.0);
            s.store_scalar(425, 0.0);
            s.store_scalar(426, 0.0);
            s.store_scalar(427, 0.0);
            s.store_scalar(428, 0.0);
            s.store_scalar(429, 0.0);
            s.store_scalar(430, 0.0);
            s.store_scalar(431, 0.0);
            s.store_scalar(432, 0.0);
        }

        if s.b[320] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(429, 331, A::tanh_scaled_input(s.ad_value(331), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(429, 331, p.p53);
                } else {
                    s.store_scalar(429, 0.0);
                }
            }
        }

        if s.b[320] {
            s.store_sub(430, 330, 331);
            s.store_mul(364, 350, 338);
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[320] {
            s.store_add_scaled_product_value_ad(366, A::div_scaled_inputs(s.ad_value(346), 1.0, s.ad_value(338), 2.302585092994046), 1.0, 349, 429, 1.0);
            s.store_add_scaled_product_right_sub(367, 345, 1.0, 356, 336, 337, 1.0);
            s.store_pow_ad(385, A::div(s.ad_value(336), s.ad_value(337)), s.ad_value(358));
        }

        s.b[433] = (s.v[357] != 0.0);
        s.store_scalar(433, if s.b[433] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[433]) {
            s.store_div_ad_rhs(368, 429, A::pow(A::offset(A::pow(A::div(s.ad_value(429), s.ad_value(357)), s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353))));
        }

        if (s.b[320] && (!s.b[433])) {
            s.store_scalar(368, 0.0);
        }

        if s.b[320] {
            s.store_mul_add_scaled_product_rhs(365, 429, s.ad_value(347), 1.0, s.ad_value(368), s.ad_value(348), (-1.0));
            s.store_sub(328, 367, 365);
            s.store_scaled_mul(370, 366, 338, 2.0);
            s.store_mul(371, 341, 370);
            s.store_sub_scaled_inputs(428, 328, 1.0, 364, (p.p51 * 0.5));
        }

        if s.b[320] {
            s.store_div_scaled_inputs2_mixed_aii(427, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sub(s.ad_value(330), s.ad_value(430)), A::tanh_scaled_input(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(330), s.ad_value(430)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 428, (-1.0), 364, 1.0);
        }

        s.b[434] = (s.v[427] > 50.0);
        s.store_scalar(434, if s.b[434] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[434]) {
            s.store_scalar(386, 0.0);
        }

        s.b[435] = (s.v[427] < (-50.0));
        s.store_scalar(435, if s.b[435] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[434])) && s.b[435]) {
            s.store_scalar(386, 1.0);
        }

        if ((s.b[320] && (!s.b[434])) && (!s.b[435])) {
            s.store_div_from_scalar_offset_ad(386, 1.0, A::exp(s.ad_value(427)), 1.0);
        }

        if s.b[320] {
            s.store_div_scaled_inputs2_mixed_aai(387, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sub(s.ad_value(330), s.ad_value(430)), A::tanh_scaled_input(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(330), s.ad_value(430)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(364), s.ad_value(386), (-(p.p51 * 0.1))), (-1.0), 370, 1.0);
        }

        s.b[436] = (s.v[387] > 50.0);
        s.store_scalar(436, if s.b[436] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[436]) {
            s.store_mul(388, 371, 387);
        }

        s.b[437] = (s.v[387] < (-50.0));
        s.store_scalar(437, if s.b[437] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[436])) && s.b[437]) {
            s.store_mul_exp_rhs(388, 371, 387);
        }

        if ((s.b[320] && (!s.b[436])) && (!s.b[437])) {
            s.store_mul_ln_one_plus_exp_rhs(388, 371, 387);
        }

        if s.b[320] {
            s.store_div_ad_rhs(374, 352, A::mul_offset_rhs(s.ad_value(385), A::div_scaled_product(s.ad_value(354), s.ad_value(388), 1.0, s.ad_value(341), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(375, 351, A::div_scaled_offset_numerator(A::mul(s.ad_value(359), s.ad_value(337)), 1.0, 1.0, A::offset(A::mul(s.ad_value(359), s.ad_value(336)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(360), s.ad_value(429), 1.0, s.ad_value(340), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(355), s.ad_value(388), 1.0, s.ad_value(341), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(392, 375, 340, 1.0, 374, 1.0);
            s.store_add_scaled_product_right_ad(393, 392, (-1.0), 392, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(388), 2.0, s.ad_value(341), s.ad_value(392), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(394, A::mul_sub_from_scalar_rhs(s.ad_value(392), 1.0, s.ad_value(386)), 1.0, 370, 386, 1.0);
            s.store_add_scaled_product_value_ad(329, A::mul_sub_from_scalar_rhs(s.ad_value(393), 1.0, s.ad_value(386)), 1.0, 370, 386, 1.0);
        }

        if s.b[320] {
            s.store_div_from_scalar_pow_ad(395, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(331), s.ad_value(329)), 0.5, A::div(s.ad_value(331), s.ad_value(329)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(331), s.ad_value(329))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(331), s.ad_value(329)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(331), s.ad_value(329))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353)));
        }

        if s.b[320] {
            s.store_mul(396, 331, 395);
        }

        if s.b[320] {
            s.store_div_from_scalar_pow_ad(397, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353)));
        }

        if s.b[320] {
            s.store_mul_neg_lhs(398, 331, 397);
            s.store_div_scaled_inputs2_indices(427, 330, 1.0, 428, (-1.0), 364, 1.0);
        }

        s.b[438] = (s.v[427] > 50.0);
        s.store_scalar(438, if s.b[438] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[438]) {
            s.store_scalar(369, 0.0);
        }

        s.b[439] = (s.v[427] < (-50.0));
        s.store_scalar(439, if s.b[439] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[438])) && s.b[439]) {
            s.store_scalar(369, 1.0);
        }

        if ((s.b[320] && (!s.b[438])) && (!s.b[439])) {
            s.store_div_from_scalar_offset_ad(369, 1.0, A::exp(s.ad_value(427)), 1.0);
        }

        if s.b[320] {
            s.store_div_scaled_inputs3_mixed_iiai(372, 430, 1.0, 398, (-1.0), A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(364), s.ad_value(369), (-(p.p51 * 0.1))), -1.0, 370, 1.0);
        }

        s.b[440] = (s.v[372] > 50.0);
        s.store_scalar(440, if s.b[440] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[440]) {
            s.store_mul(373, 371, 372);
        }

        s.b[441] = (s.v[372] < (-50.0));
        s.store_scalar(441, if s.b[441] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[440])) && s.b[441]) {
            s.store_mul_exp_rhs(373, 371, 372);
        }

        if ((s.b[320] && (!s.b[440])) && (!s.b[441])) {
            s.store_mul_ln_one_plus_exp_rhs(373, 371, 372);
        }

        if s.b[320] {
            s.store_div_scaled_inputs2_indices(427, 430, 1.0, 428, (-1.0), 364, 1.0);
        }

        s.b[442] = (s.v[427] > 50.0);
        s.store_scalar(442, if s.b[442] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[442]) {
            s.store_scalar(399, 0.0);
        }

        s.b[443] = (s.v[427] < (-50.0));
        s.store_scalar(443, if s.b[443] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[442])) && s.b[443]) {
            s.store_scalar(399, 1.0);
        }

        if ((s.b[320] && (!s.b[442])) && (!s.b[443])) {
            s.store_div_from_scalar_offset_ad(399, 1.0, A::exp(s.ad_value(427)), 1.0);
        }

        if s.b[320] {
            s.store_div_scaled_inputs3_mixed_iiai(400, 330, 1.0, 396, (-1.0), A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(364), s.ad_value(399), (-(p.p51 * 0.1))), -1.0, 370, 1.0);
        }

        s.b[444] = (s.v[400] > 50.0);
        s.store_scalar(444, if s.b[444] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[444]) {
            s.store_mul(401, 371, 400);
        }

        s.b[445] = (s.v[400] < (-50.0));
        s.store_scalar(445, if s.b[445] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[444])) && s.b[445]) {
            s.store_mul_exp_rhs(401, 371, 400);
        }

        if ((s.b[320] && (!s.b[444])) && (!s.b[445])) {
            s.store_mul_ln_one_plus_exp_rhs(401, 371, 400);
        }

        if s.b[320] {
            s.store_div_scaled_inputs2_indices(402, 373, 1.0, 401, (-1.0), 341, 1.0);
            s.store_div(428, 402, 394);
            s.store_div_scaled_inputs_indices(377, 346, 1.0, 338, 2.302585092994046);
            s.store_scaled_mul(379, 377, 338, 2.0);
            s.store_mul(380, 341, 379);
            s.store_sub_scaled_inputs(432, 367, 1.0, 364, (p.p51 * 0.5));
        }

        if s.b[320] {
            s.store_div_scaled_inputs2_mixed_aii(431, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sub(s.ad_value(330), s.ad_value(430)), A::tanh_scaled_input(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(330), s.ad_value(430)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 432, (-1.0), 364, 1.0);
        }

        s.b[446] = (s.v[431] > 50.0);
        s.store_scalar(446, if s.b[446] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[446]) {
            s.store_scalar(389, 0.0);
        }

        s.b[447] = (s.v[431] < (-50.0));
        s.store_scalar(447, if s.b[447] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[446])) && s.b[447]) {
            s.store_scalar(389, 1.0);
        }

        if ((s.b[320] && (!s.b[446])) && (!s.b[447])) {
            s.store_div_from_scalar_offset_ad(389, 1.0, A::exp(s.ad_value(431)), 1.0);
        }

        if s.b[320] {
            s.store_div_scaled_inputs2_mixed_aai(390, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sub(s.ad_value(330), s.ad_value(430)), A::tanh_scaled_input(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(330), s.ad_value(430)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(367), 1.0, s.ad_value(364), s.ad_value(389), (-(p.p51 * 0.1))), (-1.0), 379, 1.0);
        }

        s.b[448] = (s.v[390] > 50.0);
        s.store_scalar(448, if s.b[448] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[448]) {
            s.store_mul(391, 380, 390);
        }

        s.b[449] = (s.v[390] < (-50.0));
        s.store_scalar(449, if s.b[449] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[448])) && s.b[449]) {
            s.store_mul_exp_rhs(391, 380, 390);
        }

        if ((s.b[320] && (!s.b[448])) && (!s.b[449])) {
            s.store_mul_ln_one_plus_exp_rhs(391, 380, 390);
        }

        if s.b[320] {
            s.store_div(383, 352, 385);
            s.store_mul_div_scaled_offset_numerator_rhs(384, 351, A::mul(s.ad_value(359), s.ad_value(337)), 1.0, 1.0, A::offset(A::mul(s.ad_value(359), s.ad_value(336)), 1.0), 1.0);
            s.store_div_scaled_product_indices(405, 384, 340, 1.0, 383, 1.0);
            s.store_add_scaled_product_right_ad(406, 405, (-1.0), 405, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(391), 2.0, s.ad_value(341), s.ad_value(405), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(407, A::mul_sub_from_scalar_rhs(s.ad_value(406), 1.0, s.ad_value(389)), 1.0, 379, 389, 1.0);
        }

        if s.b[320] {
            s.store_div_from_scalar_pow_ad(408, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(331), s.ad_value(407)), 0.5, A::div(s.ad_value(331), s.ad_value(407)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(331), s.ad_value(407))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(331), s.ad_value(407)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(331), s.ad_value(407))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353)));
        }

        if s.b[320] {
            s.store_mul(409, 331, 408);
        }

        if s.b[320] {
            s.store_div_from_scalar_pow_ad(410, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353)));
        }

        if s.b[320] {
            s.store_mul_neg_lhs(411, 331, 410);
            s.store_div_scaled_inputs2_indices(431, 330, 1.0, 432, (-1.0), 364, 1.0);
        }

        s.b[450] = (s.v[431] > 50.0);
        s.store_scalar(450, if s.b[450] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[450]) {
            s.store_scalar(378, 0.0);
        }

        s.b[451] = (s.v[431] < (-50.0));
        s.store_scalar(451, if s.b[451] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[450])) && s.b[451]) {
            s.store_scalar(378, 1.0);
        }

        if ((s.b[320] && (!s.b[450])) && (!s.b[451])) {
            s.store_div_from_scalar_offset_ad(378, 1.0, A::exp(s.ad_value(431)), 1.0);
        }

        if s.b[320] {
            s.store_div_scaled_inputs3_mixed_iiai(381, 430, 1.0, 411, (-1.0), A::add_scaled_product(s.ad_value(367), 1.0, s.ad_value(364), s.ad_value(378), (-(p.p51 * 0.1))), -1.0, 379, 1.0);
        }

        s.b[452] = (s.v[381] > 50.0);
        s.store_scalar(452, if s.b[452] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[452]) {
            s.store_mul(382, 380, 381);
        }

        s.b[453] = (s.v[381] < (-50.0));
        s.store_scalar(453, if s.b[453] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[452])) && s.b[453]) {
            s.store_mul_exp_rhs(382, 380, 381);
        }

        if ((s.b[320] && (!s.b[452])) && (!s.b[453])) {
            s.store_mul_ln_one_plus_exp_rhs(382, 380, 381);
        }

        if s.b[320] {
            s.store_div_scaled_inputs2_indices(431, 430, 1.0, 432, (-1.0), 364, 1.0);
        }

        s.b[454] = (s.v[431] > 50.0);
        s.store_scalar(454, if s.b[454] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[454]) {
            s.store_scalar(412, 0.0);
        }

        s.b[455] = (s.v[431] < (-50.0));
        s.store_scalar(455, if s.b[455] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[454])) && s.b[455]) {
            s.store_scalar(412, 1.0);
        }

        if ((s.b[320] && (!s.b[454])) && (!s.b[455])) {
            s.store_div_from_scalar_offset_ad(412, 1.0, A::exp(s.ad_value(431)), 1.0);
        }

        if s.b[320] {
            s.store_div_scaled_inputs3_mixed_iiai(413, 330, 1.0, 409, (-1.0), A::add_scaled_product(s.ad_value(367), 1.0, s.ad_value(364), s.ad_value(412), (-(p.p51 * 0.1))), -1.0, 379, 1.0);
        }

        s.b[456] = (s.v[413] > 50.0);
        s.store_scalar(456, if s.b[456] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[456]) {
            s.store_mul(414, 380, 413);
        }

        s.b[457] = (s.v[413] < (-50.0));
        s.store_scalar(457, if s.b[457] { 1.0 } else { 0.0 });

        if ((s.b[320] && (!s.b[456])) && s.b[457]) {
            s.store_mul_exp_rhs(414, 380, 413);
        }

        if ((s.b[320] && (!s.b[456])) && (!s.b[457])) {
            s.store_mul_ln_one_plus_exp_rhs(414, 380, 413);
        }

        if s.b[320] {
            s.store_offset_square(415, 382, 1e-38);
            s.store_offset_mul(416, 415, 382, 1e-57);
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[320] {
            s.store_offset_square(417, 414, 1e-38);
            s.store_offset_mul(418, 417, 414, 1e-57);
            s.store_offset_mul(419, 382, 414, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(420, 415, (2.0 / 3.0), 417, (2.0 / 3.0), 419, (2.0 / 3.0), A::offset(A::add(s.ad_value(382), s.ad_value(414)), 2e-19), 1.0);
            s.store_div_ad(421, A::add_scaled_inputs_products(s.ad_value(416), (2.0 * 2.0), s.ad_value(418), (3.0 * 2.0), s.ad_value(415), s.ad_value(414), (4.0 * 2.0), s.ad_value(417), s.ad_value(382), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(415), 15.0, s.ad_value(417), 15.0, s.ad_value(419), (2.0 * 15.0)));
            s.store_sub(422, 420, 421);
            s.copy_ad(423, 421);
            s.store_mul_product3_mixed_iaii(323, 363, A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(340)), 362, 422, 1.0);
            s.store_mul_product3_mixed_iaii(324, 363, A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(340)), 362, 423, 1.0);
        }

        s.b[458] = (s.v[332] == 1.0);
        s.store_scalar(458, if s.b[458] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[458]) {
            s.store_div_scaled_inputs3_indices(424, 333, 1.0, 367, -1.0, 364, (-(-(p.p51 * 0.5))), 379, 1.0);
        }

        s.b[459] = (s.v[424] > 50.0);
        s.store_scalar(459, if s.b[459] { 1.0 } else { 0.0 });

        if ((s.b[320] && s.b[458]) && s.b[459]) {
            s.copy_ad(427, 424);
        }

        s.b[460] = (s.v[424] < (-50.0));
        s.store_scalar(460, if s.b[460] { 1.0 } else { 0.0 });

        if (((s.b[320] && s.b[458]) && (!s.b[459])) && s.b[460]) {
            s.store_exp(427, 424);
        }

        if (((s.b[320] && s.b[458]) && (!s.b[459])) && (!s.b[460])) {
            s.store_ln_one_plus_exp(427, 424);
        }

        if (s.b[320] && s.b[458]) {
            s.store_mul_ad_product_lhs_mixed_ai(325, A::mul3(A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(362)), s.ad_value(343), s.ad_value(379)), 427, 363);
            s.store_div_scaled_inputs3_indices(425, 334, 1.0, 367, -1.0, 364, (-(-(p.p51 * 0.5))), 379, 1.0);
        }

        s.b[461] = (s.v[425] > 50.0);
        s.store_scalar(461, if s.b[461] { 1.0 } else { 0.0 });

        if ((s.b[320] && s.b[458]) && s.b[461]) {
            s.copy_ad(427, 425);
        }

        s.b[462] = (s.v[425] < (-50.0));
        s.store_scalar(462, if s.b[462] { 1.0 } else { 0.0 });

        if (((s.b[320] && s.b[458]) && (!s.b[461])) && s.b[462]) {
            s.store_exp(427, 425);
        }

        if (((s.b[320] && s.b[458]) && (!s.b[461])) && (!s.b[462])) {
            s.store_ln_one_plus_exp(427, 425);
        }

        if (s.b[320] && s.b[458]) {
            s.store_mul_ad_product_lhs_mixed_ai(326, A::mul3(A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(362)), s.ad_value(344), s.ad_value(379)), 427, 363);
        }

        if (s.b[320] && (!s.b[458])) {
            s.store_scalar(325, 0.0);
            s.store_scalar(326, 0.0);
        }

        s.b[463] = (s.v[335] == 1.0);
        s.store_scalar(463, if s.b[463] { 1.0 } else { 0.0 });

        if (s.b[320] && s.b[463]) {
            s.store_div_scaled_inputs3_indices(426, 330, 1.0, 367, -1.0, 364, (-(-(p.p51 * 0.5))), 379, 1.0);
        }

        s.b[464] = (s.v[426] > 50.0);
        s.store_scalar(464, if s.b[464] { 1.0 } else { 0.0 });

        if ((s.b[320] && s.b[463]) && s.b[464]) {
            s.copy_ad(427, 426);
        }

        s.b[465] = (s.v[426] < (-50.0));
        s.store_scalar(465, if s.b[465] { 1.0 } else { 0.0 });

        if (((s.b[320] && s.b[463]) && (!s.b[464])) && s.b[465]) {
            s.store_exp(427, 426);
        }

        if (((s.b[320] && s.b[463]) && (!s.b[464])) && (!s.b[465])) {
            s.store_ln_one_plus_exp(427, 426);
        }

        if (s.b[320] && s.b[463]) {
            s.store_mul_ad_product_lhs_mixed_ai(327, A::mul3(A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(362)), s.ad_value(342), s.ad_value(379)), 427, 363);
        }

        if (s.b[320] && (!s.b[463])) {
            s.store_scalar(327, 0.0);
        }

        if s.b[320] {
            s.copy_ad(209, 323);
            s.copy_ad(210, 324);
            s.copy_ad(211, 325);
            s.copy_ad(212, 326);
            s.copy_ad(213, 327);
        }

        s.b[466] = (p.p232 == 1.0);
        s.store_scalar(466, if s.b[466] { 1.0 } else { 0.0 });

        s.store_scalar(203, 0.0);

        s.store_scalar(204, 0.0);

        s.store_scalar(205, 0.0);

        s.store_scalar(206, 0.0);

        s.store_scalar(207, 0.0);

        s.b[467] = (p.p211 > p.p354);
        s.store_scalar(467, if s.b[467] { 1.0 } else { 0.0 });

        if s.b[467] {
            s.store_scalar(470, 0.0);
            s.store_scalar(471, 0.0);
            s.store_scalar(472, 0.0);
            s.store_scalar(473, 0.0);
            s.store_scalar(474, 0.0);
            s.store_scalar(475, 0.0);
            s.store_scalar(476, 0.0);
            s.copy_ad(477, 96);
            s.copy_ad(478, 97);
            s.store_scalar(479, p.p217);
            s.copy_ad(480, 98);
            s.copy_ad(481, 99);
            s.store_scalar(482, p.p215);
            s.copy_ad(483, 111);
            s.store_scalar(484, s.v[109]);
            s.copy_ad(485, 113);
            s.store_scalar(486, p.p0);
            s.store_scalar(487, p.p211);
            s.copy_ad(488, 38);
            s.store_scalar(489, p.p216);
            s.copy_ad(490, 39);
            s.copy_ad(491, 40);
            s.store_scalar(492, p.p212);
            s.store_scalar(493, p.p226);
            s.store_scalar(494, p.p225);
            s.store_scalar(495, 0.0);
            s.store_scalar(496, p.p227);
            s.store_scalar(497, p.p231);
            s.store_scalar(498, p.p222);
            s.store_scalar(499, p.p223);
            s.store_scalar(500, p.p224);
            s.store_scalar(501, p.p230);
            s.store_scalar(502, p.p229);
            s.store_scalar(503, p.p228);
            s.store_scalar(504, p.p39);
            s.store_scalar(505, p.p47);
            s.store_scalar(506, p.p45);
            s.store_scalar(507, p.p42);
            s.store_scalar(508, p.p2);
            s.store_scalar(509, p.p6);
            s.store_scalar(510, 1.0);
            s.store_scalar(511, 0.0);
            s.store_scalar(512, 0.0);
            s.store_scalar(513, 0.0);
            s.store_scalar(514, 0.0);
            s.store_scalar(515, 0.0);
            s.store_scalar(516, 0.0);
            s.store_scalar(517, 0.0);
            s.store_scalar(518, 0.0);
            s.store_scalar(519, 0.0);
            s.store_scalar(520, 0.0);
            s.store_scalar(521, 0.0);
            s.store_scalar(522, 0.0);
            s.store_scalar(524, 0.0);
            s.store_scalar(525, 0.0);
            s.store_scalar(526, 0.0);
            s.store_scalar(527, 0.0);
            s.store_scalar(528, 0.0);
            s.store_scalar(529, 0.0);
            s.store_scalar(530, 0.0);
            s.store_scalar(531, 0.0);
            s.store_scalar(532, 0.0);
            s.store_scalar(533, 0.0);
            s.store_scalar(534, 0.0);
            s.store_scalar(535, 0.0);
            s.store_scalar(536, 0.0);
            s.store_scalar(537, 0.0);
            s.store_scalar(538, 0.0);
            s.store_scalar(539, 0.0);
            s.store_scalar(540, 0.0);
            s.store_scalar(541, 0.0);
            s.store_scalar(542, 0.0);
            s.store_scalar(543, 0.0);
            s.store_scalar(544, 0.0);
            s.store_scalar(545, 0.0);
            s.store_scalar(546, 0.0);
            s.store_scalar(547, 0.0);
            s.store_scalar(548, 0.0);
            s.store_scalar(549, 0.0);
            s.store_scalar(552, 0.0);
            s.store_scalar(553, 0.0);
            s.store_scalar(554, 0.0);
            s.store_scalar(555, 0.0);
            s.store_scalar(556, 0.0);
            s.store_scalar(557, 0.0);
            s.store_scalar(558, 0.0);
        }

    }
}
